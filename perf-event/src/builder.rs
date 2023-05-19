use std::fmt;
use std::fs::File;
use std::io::{self, ErrorKind};
use std::os::raw::{c_int, c_ulong};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::sync::Arc;

use libc::pid_t;
use perf_event_data::parse::ParseConfig;

use crate::events::{Event, EventData};
use crate::sys::bindings::perf_event_attr;
use crate::{check_errno_syscall, sys, Clock, Counter, Group, ReadFormat, SampleFlag, SampleSkid};

/// A builder for [`Counter`]s.
///
/// There are dozens of parameters that influence a `Counter`'s behavior.
/// `Builder` lets you construct a `Counter` by specifying only those parameters
/// for which you don't want the default value.
///
/// A freshly built `Counter` is disabled. To begin counting events, you must
/// call [`enable`] on the `Counter` or the `Group` to which it belongs.
///
/// For example, if you want a `Counter` for instructions retired by the current
/// process, those are `Builder`'s defaults, so you need only write:
///
/// ```
/// # use perf_event::Builder;
/// # use perf_event::events::Hardware;
/// #
/// let mut insns = Builder::new(Hardware::INSTRUCTIONS).build()?;
/// # std::io::Result::Ok(())
/// ```
///
/// If you would like to gather individual counters into a [`Group`] you can
/// use the [`Group::add`] method. A [`Group`] allows you to enable or disable
/// all the grouped counters atomically.
///
/// ```
/// # use perf_event::{Builder, Group};
/// # use perf_event::events::Hardware;
/// #
/// let mut group = Group::new()?;
/// let cycles = group.add(&Builder::new(Hardware::CPU_CYCLES))?;
/// let insns = group.add(&Builder::new(Hardware::INSTRUCTIONS))?;
/// #
/// # std::io::Result::Ok(())
/// ```
///
/// Other methods let you select:
///
/// - specific processes or cgroups to observe
/// - specific CPU cores to observe
///
/// `Builder` supports only a fraction of the many knobs and dials Linux offers,
/// but hopefully it will acquire methods to support more of them as time goes
/// on.
///
/// Internally, a `Builder` is just a wrapper around the kernel's `struct
/// perf_event_attr` type.
///
/// [`enable`]: Counter::enable
#[derive(Clone)]
pub struct Builder<'a> {
    attrs: perf_event_attr,
    who: EventPid<'a>,
    cpu: Option<usize>,

    // Some events need to hold onto data that is referenced in the builder.
    // The perf_event_attr struct obviously doesn't have lifetimes so the only
    // safe solution is to have the builder hold onto it.
    event_data: Option<Arc<dyn EventData>>,
}

// Needed for backwards compat
impl UnwindSafe for Builder<'_> {}
impl RefUnwindSafe for Builder<'_> {}

#[derive(Clone, Debug)]
enum EventPid<'a> {
    /// Monitor the calling process.
    ThisProcess,

    /// Monitor the given pid.
    Other(pid_t),

    /// Monitor members of the given cgroup.
    CGroup(&'a File),

    /// Monitor any process on some given CPU.
    Any,
}

impl<'a> EventPid<'a> {
    // Return the `pid` arg and the `flags` bits representing `self`.
    fn as_args(&self) -> (pid_t, u32) {
        match self {
            EventPid::Any => (-1, 0),
            EventPid::ThisProcess => (0, 0),
            EventPid::Other(pid) => (*pid, 0),
            EventPid::CGroup(file) => (file.as_raw_fd(), sys::bindings::PERF_FLAG_PID_CGROUP),
        }
    }
}

// Methods that actually do work on the builder and aren't just setting
// config values.
impl<'a> Builder<'a> {
    /// Return a new `Builder`, with all parameters set to their defaults.
    ///
    /// Return a new `Builder` for the specified event.
    pub fn new<E: Event + Sized>(event: E) -> Self {
        let mut attrs = perf_event_attr::default();

        // Do the update_attrs bit before we set any of the default state so
        // that user code can't break configuration we really care about.
        let data = event.update_attrs_with_data(&mut attrs);

        // Setting `size` accurately will not prevent the code from working
        // on older kernels. The module comments for `perf_event_open_sys`
        // explain why in far too much detail.
        attrs.size = std::mem::size_of::<perf_event_attr>() as u32;

        let mut builder = Self {
            attrs,
            who: EventPid::ThisProcess,
            cpu: None,
            event_data: data,
        };

        builder.enabled(false);
        builder.exclude_kernel(true);
        builder.exclude_hv(true);
        builder.read_format(ReadFormat::TOTAL_TIME_ENABLED | ReadFormat::TOTAL_TIME_RUNNING);
        builder
    }

    /// Construct a [`Counter`] according to the specifications made on this
    /// `Builder`.
    ///
    /// If you want to add this counter to a group use [`build_with_group`]
    /// instead.
    ///
    /// By default, a newly built [`Counter`] is disabled. To begin counting
    /// events, you must call [`enable`] on the [`Counter`] or the [`Group`]
    /// to which it belongs. Alternatively, certain options (e.g.
    /// [`enable_on_exec`]) may be used to automatically enable the [`Counter`]
    /// once certain events occur.
    ///
    /// [`build_with_group`]: Self::build_with_group
    ///
    /// # Errors
    /// - The `perf_event_open` syscall has a large number of different errors
    ///   it can return. See the [man page][0] for details. Unfortunately, the
    ///   errors returned by the kernel are not always helpful.
    /// - This method translates `E2BIG` errors (which means the kernel did not
    ///   support some options) into a custom [`std::io::Error`] with kind
    ///   [`ErrorKind::Unsupported`] and an internal error of
    ///   [`UnsupportedOptionsError`]. This allows you to access the size of the
    ///   [`perf_event_attr`] struct that the kernel was expecting.
    ///
    /// # Panics
    /// This method panics if `attrs.size` has been set to a value larger than
    /// the size of the [`perf_event_attr`] struct.
    ///
    /// [`Group`]: crate::Group
    /// [`Group::add`]: crate::Group::add
    /// [`enable`]: crate::Counter::enable
    /// [`enable_on_exec`]: Builder::enable_on_exec
    /// [0]: https://man7.org/linux/man-pages/man2/perf_event_open.2.html
    pub fn build(&self) -> std::io::Result<Counter> {
        Counter::new_internal(self.build_impl(None)?, ParseConfig::from(self.attrs))
    }

    /// Construct a [`Counter`] as part of a group.
    ///
    /// The `group` passed in must be the leader of the group you to add the
    /// resulting [`Counter`] to.
    ///
    /// ## Notes
    /// - The group leader does not have to be a [`Group`] (although it can be),
    ///   any [`Counter`] will work just fine as a group leader provided it is
    ///   not already within a group itself.
    /// - Similarly with enabling, disabling, or resetting all counters in the
    ///   group. Any counter in the group can do those via [`enable_group`],
    ///   [`disable_group`], and [`reset_group`].
    /// - The same applies for reading group values. Any counter that has
    ///   [`ReadFormat::GROUP`] set in [`read_format`](Self::read_format)can
    ///   read the counter values for the entire group using [`read_group`].
    ///
    /// Note, however, that [`Group`] is likely to be more convenient if you
    /// don't want to set [`ReadFormat::GROUP`] on any of the counters
    /// within the group.
    ///
    /// [`enable_group`]: crate::Counter::enable_group
    /// [`disable_group`]: crate::Counter::disable_group
    /// [`reset_group`]: crate::Counter::reset_group
    /// [`read_group`]: crate::Counter::read_group
    /// [`ReadFormat::GROUP`]: crate::ReadFormat::GROUP
    ///
    /// # Errors
    /// - The `perf_event_open` syscall has a large number of different errors
    ///   it can return. See the [man page][0] for details. Unfortunately, the
    ///   errors returned by the kernel are not always helpful.
    /// - This method translates `E2BIG` errors (which means the kernel did not
    ///   support some options) into a custom [`std::io::Error`] with kind
    ///   [`ErrorKind::Unsupported`] and an internal error of
    ///   [`UnsupportedOptionsError`]. This allows you to access the size of the
    ///   [`perf_event_attr`] struct that the kernel was expecting.
    ///
    /// [0]: https://man7.org/linux/man-pages/man2/perf_event_open.2.html
    ///
    /// # Panics
    /// This method panics if `attrs.size` has been set to a value larger than
    /// the size of the [`perf_event_attr`] struct.
    pub fn build_with_group(&self, mut group: impl AsMut<Counter>) -> io::Result<Counter> {
        let group: &mut Counter = group.as_mut();
        let file = self.build_impl(Some(group.as_raw_fd()))?;

        group.member_count = group
            .member_count
            .checked_add(1)
            .expect("cannot add more than u32::MAX elements to a group");

        Counter::new_internal(file, ParseConfig::from(self.attrs))
    }

    /// Build a [`Group`] according to the specifications made on this
    /// `Builder`.
    ///
    /// Note that you will need to have set [`ReadFormat::GROUP`] within
    /// [`read_format`] to or this method will error.
    ///
    /// [`read_format`]: Self::read_format
    ///
    /// # Notes
    /// - A [`Group`] is just a wrapper around a [`Counter`] whose methods use
    ///   the corresponding `*_group` methods on [`Counter`].
    /// - The [`GroupData`] returned from [`Group::read`] doesn't include the
    ///   group itself when being iterated over. You will likely want to use the
    ///   [`Software::DUMMY`] event when constructing a group.
    ///
    /// # Errors
    /// - All errors that can be returned from [`build`](Self::build).
    /// - An error will be returned if [`ReadFormat::GROUP`] is not set within
    ///   `read_format`. It will have a kind of [`ErrorKind::Other`].
    ///
    /// # Panics
    /// This method panics if `attrs.size` has been set to a value larger than
    /// the size of the [`perf_event_attr`] struct.
    ///
    /// [`GroupData`]: crate::GroupData
    /// [`Software::DUMMY`]: crate::events::Software::DUMMY
    pub fn build_group(&self) -> io::Result<Group> {
        let read_format = ReadFormat::from_bits_retain(self.attrs.read_format);
        if !read_format.contains(ReadFormat::GROUP) {
            return Err(io::Error::new(
                ErrorKind::Other,
                "groups must be created with the GROUP flag enabled",
            ));
        }

        Ok(Group(self.build()?))
    }

    pub(crate) fn build_impl(&self, group_fd: Option<RawFd>) -> io::Result<File> {
        // Users of this crate can modify attrs.size (e.g. to use it for feature
        // detection) but in order for the perf_event_open call to be safe it
        // must not exceed the size of perf_event_attr.
        assert!(self.attrs.size <= std::mem::size_of::<perf_event_attr>() as u32);

        let cpu = match self.cpu {
            Some(cpu) => cpu as c_int,
            None => -1,
        };

        let (pid, flags) = self.who.as_args();
        let group_fd = group_fd.unwrap_or(-1);

        // Enable CLOEXEC by default. This the behaviour that the rust stdlib
        // uses for all its file descriptors.
        //
        // If you need to get a perf_event_open fd which does not have CLOEXEC
        // set then you can modify the flags after the fact with fcntl(2).
        let flags = flags | sys::bindings::PERF_FLAG_FD_CLOEXEC;

        let mut attrs = self.attrs;

        let result = check_errno_syscall(|| unsafe {
            sys::perf_event_open(&mut attrs, pid, cpu, group_fd, flags as c_ulong)
        });

        match result {
            Ok(fd) => unsafe { Ok(File::from_raw_fd(fd)) },
            // In case of an E2BIG error we return a custom error so that users
            // can get at the size expected by the kernel if they want to.
            Err(e) if e.raw_os_error() == Some(libc::E2BIG) => Err(std::io::Error::new(
                ErrorKind::Unsupported,
                UnsupportedOptionsError::new(attrs.size),
            )),
            Err(e) => Err(e),
        }
    }
}

impl<'a> Builder<'a> {
    /// Directly access the [`perf_event_attr`] within this builder.
    pub fn attrs(&self) -> &perf_event_attr {
        &self.attrs
    }

    /// Directly access the [`perf_event_attr`] within this builder.
    pub fn attrs_mut(&mut self) -> &mut perf_event_attr {
        &mut self.attrs
    }

    /// Observe the calling process. (This is the default.)
    pub fn observe_self(&mut self) -> &mut Self {
        self.who = EventPid::ThisProcess;
        self
    }

    /// Observe the process with the given process id. This requires
    /// [`CAP_SYS_PTRACE`][man-capabilities] capabilities.
    ///
    /// [man-capabilities]: http://man7.org/linux/man-pages/man7/capabilities.7.html
    pub fn observe_pid(&mut self, pid: pid_t) -> &mut Self {
        self.who = EventPid::Other(pid);
        self
    }

    /// Observe all processes.
    ///
    /// Linux does not support observing all processes on all CPUs without
    /// restriction, so combining `any_pid` with [`any_cpu`] will cause the
    /// final [`build`] to return an error. This must be used together with
    /// [`one_cpu`], to select a specific CPU to observe.
    ///
    /// This requires [`CAP_PERFMON`][cap] or [`CAP_SYS_ADMIN`][cap]
    /// capabilities, or a `/proc/sys/kernel/perf_event_paranoid` value of less
    /// than 1.
    ///
    /// [`any_cpu`]: Builder::any_cpu
    /// [`build`]: Builder::build
    /// [`one_cpu`]: Builder::one_cpu
    /// [cap]: http://man7.org/linux/man-pages/man7/capabilities.7.html
    pub fn any_pid(&mut self) -> &mut Self {
        self.who = EventPid::Any;
        self
    }

    /// Observe code running in the given [cgroup][man-cgroups] (container). The
    /// `cgroup` argument should be a `File` referring to the cgroup's directory
    /// in the cgroupfs filesystem.
    ///
    /// [man-cgroups]: http://man7.org/linux/man-pages/man7/cgroups.7.html
    pub fn observe_cgroup(&mut self, cgroup: &'a File) -> &mut Self {
        self.who = EventPid::CGroup(cgroup);
        self
    }

    /// Observe only code running on the given CPU core.
    pub fn one_cpu(&mut self, cpu: usize) -> &mut Self {
        self.cpu = Some(cpu);
        self
    }

    /// Observe code running on any CPU core. (This is the default.)
    ///
    /// Linux does not support observing all processes on all CPUs without
    /// restriction, so combining `any_cpu` with [`any_pid`] will cause
    /// [`build`] to return an error. This must be used with [`observe_self`]
    /// (the default), [`observe_pid`], or [`observe_cgroup`].
    ///
    /// [`any_pid`]: Builder::any_pid
    /// [`build`]: Builder::build
    /// [`observe_self`]: Builder::observe_self
    /// [`observe_pid`]: Builder::observe_pid
    /// [`observe_cgroup`]: Builder::observe_cgroup
    pub fn any_cpu(&mut self) -> &mut Self {
        self.cpu = None;
        self
    }

    /// Indicate additional values to include in the generated sample events.
    ///
    /// Note that this method is additive and does not remove previously added
    /// sample types. See the documentation of [`SampleFlag`] or the [manpage]
    /// for what's available to be collected.
    ///
    /// # Example
    /// Here we build a sampler that grabs the instruction pointer, process ID,
    /// thread ID, and timestamp whenever the underlying event triggers a
    /// sampling.
    /// ```
    /// # use perf_event::{Builder, SampleFlag};
    /// # use perf_event::events::Hardware;
    /// let mut sampler = Builder::new(Hardware::INSTRUCTIONS)
    ///     .sample(SampleFlag::IP)
    ///     .sample(SampleFlag::TID)
    ///     .sample(SampleFlag::TIME)
    ///     .build()?
    ///     .sampled(8192)?;
    /// # Ok::<_, std::io::Error>(())
    /// ```
    ///
    /// [`SampleFlag`]: crate::SampleFlag
    /// [manpage]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
    pub fn sample(&mut self, sample: SampleFlag) -> &mut Self {
        self.attrs.sample_type |= sample.bits();
        self
    }

    /// Set the fields to include when reading from the counter.
    ///
    /// Note that this method is _not_ additive, unlike [`sample`].
    ///
    /// The implementation of this library will silently mask out certain flags
    /// if they would be invalid. For example, we will not allow you to set
    /// [`ReadFormat::GROUP`] when building a single counter.
    ///
    /// [`sample`]: Builder::sample
    pub fn read_format(&mut self, mut read_format: ReadFormat) -> &mut Self {
        if read_format.contains(ReadFormat::GROUP) {
            read_format |= ReadFormat::ID;
        }

        self.attrs.read_format = read_format.bits();
        self
    }
}

// Section for methods which directly modify attrs. These should correspond
// roughly 1-to-1 with the entries as documented in the manpage.
impl<'a> Builder<'a> {
    /// Whether this counter should start off enabled.
    ///
    /// When this is set, the counter will immediately start being recorded as
    /// soon as it is created.
    ///
    /// By default, this is false.
    pub fn enabled(&mut self, enabled: bool) -> &mut Self {
        self.attrs.set_disabled((!enabled).into());
        self
    }

    /// Set whether this counter is inherited by new threads.
    ///
    /// When this flag is set, this counter observes activity in new threads
    /// created by any thread already being observed.
    ///
    /// By default, the flag is unset: counters are not inherited, and observe
    /// only the threads specified when they are created.
    ///
    /// This flag cannot be set if the counter belongs to a `Group`. Doing so
    /// will result in an error when the counter is built. This is a kernel
    /// limitation.
    pub fn inherit(&mut self, inherit: bool) -> &mut Self {
        self.attrs.set_inherit(inherit.into());
        self
    }

    /// Set whether the counter is pinned to the PMU.
    ///
    /// If this flag is set, the kernel will attempt to keep the counter on
    /// always on the CPU if at all possible. If it fails to do so, the counter
    /// will enter an error state where reading it will always return EOF. For
    /// this crate, that would result in [`Counter::read`] returning an error
    /// with kind [`ErrorKind::UnexpectedEof`].
    ///
    /// This option only applies to hardware counters and group leaders. At
    /// this time this crate provides no way to configure group leaders so this
    /// option will only work when the resulting counter is not in a group.
    ///
    /// This is false by default.
    ///
    /// [`ErrorKind::UnexpectedEof`]: std::io::ErrorKind::UnexpectedEof
    pub fn pinned(&mut self, pinned: bool) -> &mut Self {
        self.attrs.set_pinned(pinned.into());
        self
    }

    /// Controls whether the counter or group can be scheduled onto a CPU
    /// alongside other counters or groups.
    ///
    /// This is false by default.
    pub fn exclusive(&mut self, exclusive: bool) -> &mut Self {
        self.attrs.set_exclusive(exclusive.into());
        self
    }

    /// Whether we should exclude events that occur in user space.
    ///
    /// This is false by default.
    pub fn exclude_user(&mut self, exclude_user: bool) -> &mut Self {
        self.attrs.set_exclude_user(exclude_user.into());
        self
    }

    /// Whether we should exclude events that occur in kernel space.
    ///
    /// Note that setting this to false may result in permission errors if
    /// the current `perf_event_paranoid` value is greater than 1.
    ///
    /// This is true by default.
    pub fn exclude_kernel(&mut self, exclude_kernel: bool) -> &mut Self {
        self.attrs.set_exclude_kernel(exclude_kernel.into());
        self
    }

    /// Include kernel code.
    ///
    /// See [`exclude_kernel`](Builder::exclude_kernel).
    pub fn include_kernel(&mut self) -> &mut Self {
        self.exclude_kernel(false)
    }

    /// Whether we should exclude events that happen in the hypervisor.
    ///
    /// This is not supported on all architectures as it required built-in
    /// support within the CPU itself.
    ///
    /// Note that setting this to false may result in permission errors if
    /// the current `perf_event_paranoid` value is greater than 1.
    ///
    /// This is true by default
    pub fn exclude_hv(&mut self, exclude_hv: bool) -> &mut Self {
        self.attrs.set_exclude_hv(exclude_hv.into());
        self
    }

    /// Include hypervisor code.
    ///
    /// See [`exclude_hv`](Builder::exclude_hv).
    pub fn include_hv(&mut self) -> &mut Self {
        self.exclude_hv(false)
    }

    /// Whether to exclude events that occur when running the idle task.
    ///
    /// Note that this only has an effect for software events.
    pub fn exclude_idle(&mut self, exclude_idle: bool) -> &mut Self {
        self.attrs.set_exclude_idle(exclude_idle.into());
        self
    }

    /// Enable the generation of MMAP records for executable memory maps.
    ///
    /// MMAP records are emitted when the process/thread that is being
    /// observed creates a new executable memory mapping.
    pub fn mmap(&mut self, mmap: bool) -> &mut Self {
        self.attrs.set_mmap(mmap.into());
        self
    }

    /// Enable the tracking of process command name changes.
    ///
    /// This can happen when a process calls `execve(2)`, `prctl(PR_SET_NAME)`,
    /// or writes to `/proc/self/comm`.
    ///
    /// If you also set the [`comm_exec`](Builder::comm_exec) flag, then the
    /// kernel will indicate which of these process name changes were due to
    /// calls to `execve(2)`.
    pub fn comm(&mut self, comm: bool) -> &mut Self {
        self.attrs.set_comm(comm.into());
        self
    }

    /// Set the period at which the kernel will generate sample events.
    ///
    /// As an example, if the event is `Hardware::INSTRUCTIONS` and `period`
    /// is 100_000 then every 100_000 instructions the kernel will generate an
    /// event.
    ///
    /// Note that the actual precision at which the sample corresponds to the
    /// instant and location at which Nth event occurred is controlled by the
    /// [`precise_ip`] option.
    ///
    /// This setting is mutually exclusive with [`sample_frequency`].
    ///
    /// [`precise_ip`]: Builder::precise_ip
    /// [`sample_frequency`]: Builder::sample_frequency
    pub fn sample_period(&mut self, period: u64) -> &mut Self {
        self.attrs.set_freq(0);
        self.attrs.__bindgen_anon_1.sample_period = period;
        self
    }

    /// Set the frequency at which the kernel will generate sample events
    /// (in Hz).
    ///
    /// Note that this is not guaranteed to be exact. The kernel will adjust
    /// the period to attempt to keep the desired frequency but the rate at
    /// which events occur varies drastically then samples may not occur at
    /// the specified frequency.
    ///
    /// The amount to which samples correspond to the instant and location at
    /// which an event occurred is controlled by the [`precise_ip`] option.
    ///
    /// This setting is mutually exclusive with [`sample_period`].
    ///
    /// [`precise_ip`]: Builder::precise_ip
    /// [`sample_period`]: Builder::sample_period
    pub fn sample_frequency(&mut self, frequency: u64) -> &mut Self {
        self.attrs.set_freq(1);
        self.attrs.__bindgen_anon_1.sample_freq = frequency;
        self
    }

    /// Save event counts on context switch for inherited tasks.
    ///
    /// This option is only meaningful if [`inherit`] is also enabled.
    ///
    /// [`inherit`]: Builder::inherit
    pub fn inherit_stat(&mut self, inherit_stat: bool) -> &mut Self {
        self.attrs.set_inherit_stat(inherit_stat.into());
        self
    }

    /// Enable the counter automatically after a call to `execve(2)`.
    pub fn enable_on_exec(&mut self, enable_on_exec: bool) -> &mut Self {
        self.attrs.set_enable_on_exec(enable_on_exec.into());
        self
    }

    /// If set, then the kernel will generate fork and exit records.
    pub fn task(&mut self, task: bool) -> &mut Self {
        self.attrs.set_task(task.into());
        self
    }

    /// Set how many bytes will be written before the kernel sends an overflow
    /// notification.
    ///
    /// This controls how much data will be emitted before
    /// [`Sampler::next_blocking`] will wake up once blocked.
    ///
    /// This setting is mutually exclusive with [`wakeup_events`].
    ///
    /// [`wakeup_events`]: Self::wakeup_events
    /// [`Sampler::next_blocking`]: crate::Sampler::next_blocking
    pub fn wakeup_watermark(&mut self, watermark: usize) -> &mut Self {
        self.attrs.set_watermark(1);
        self.attrs.__bindgen_anon_2.wakeup_watermark = watermark as _;
        self
    }

    /// Set how many samples will be written before the kernel sends an
    /// overflow notification.
    ///
    /// This controls how much data will be emitted before
    /// [`Sampler::next_blocking`] will wake up once blocked. Note that only
    /// sample records (`PERF_RECORD_SAMPLE`) count towards the event count.
    ///
    /// Some caveats apply, see the [manpage] for the full documentation.
    ///
    /// This method is mutually exclusive with [`wakeup_watermark`].
    ///
    /// [manpage]: https://man7.org/linux/man-pages/man2/perf_event_open.2.html
    /// [`wakeup_watermark`]: Builder::wakeup_watermark
    /// [`Sampler::next_blocking`]: crate::Sampler::next_blocking
    pub fn wakeup_events(&mut self, events: usize) -> &mut Self {
        self.attrs.set_watermark(0);
        self.attrs.__bindgen_anon_2.wakeup_events = events as _;
        self
    }

    /// Control how much skid is permitted when recording events.
    ///
    /// Skid is the number of instructions that occur between an event occuring
    /// and a sample being gathered by the kernel. Less skid is better but
    /// there are hardware limitations around how small the skid can be.
    ///
    /// Also see [`SampleSkid`].
    pub fn precise_ip(&mut self, skid: SampleSkid) -> &mut Self {
        self.attrs.set_precise_ip(skid as _);
        self
    }

    /// Enable the generation of MMAP records for non-executable memory maps.
    ///
    /// This is the data counterpart of [`mmap`](Builder::mmap).
    pub fn mmap_data(&mut self, mmap_data: bool) -> &mut Self {
        self.attrs.set_mmap_data(mmap_data.into());
        self
    }

    /// If enabled, then a subset of the sample fields will additionally be
    /// included in most non-`PERF_RECORD_SAMPLE` samples.
    ///
    /// See the [manpage] for the exact fields that are included and which
    /// records include the trailer.
    ///
    /// [manpage]: https://man7.org/linux/man-pages/man2/perf_event_open.2.html
    pub fn sample_id_all(&mut self, sample_id_all: bool) -> &mut Self {
        self.attrs.set_sample_id_all(sample_id_all.into());
        self
    }

    /// Only collect measurements for events occurring inside a VM instance.
    ///
    /// This is only meaningful when profiling from outside the VM instance.
    ///
    /// See the [manpage] for more documentation.
    ///
    /// [manpage]: https://man7.org/linux/man-pages/man2/perf_event_open.2.html
    pub fn exclude_host(&mut self, exclude_host: bool) -> &mut Self {
        self.attrs.set_exclude_host(exclude_host.into());
        self
    }

    /// Don't collect measurements for events occurring inside a VM instance.
    ///
    /// This is only meaningful when profiling from outside the VM instance.
    ///
    /// See the [manpage] for more documentation.
    ///
    /// [manpage]: https://man7.org/linux/man-pages/man2/perf_event_open.2.html
    pub fn exclude_guest(&mut self, exclude_guest: bool) -> &mut Self {
        self.attrs.set_exclude_guest(exclude_guest.into());
        self
    }

    /// Do not include stack frames in the kernel when gathering callchains as
    /// a part of recording a sample.
    pub fn exclude_callchain_kernel(&mut self, exclude_kernel: bool) -> &mut Self {
        self.attrs
            .set_exclude_callchain_kernel(exclude_kernel.into());
        self
    }

    /// Do not include stack frames from userspace when gathering a callchain
    /// as a part of recording a sample.
    pub fn exclude_callchain_user(&mut self, exclude_user: bool) -> &mut Self {
        self.attrs.set_exclude_callchain_user(exclude_user.into());
        self
    }

    /// Generate an extended executable mmap record.
    ///
    /// This record has enough info to uniquely identify which instance of a
    /// shared map it corresponds to. Note that you also need to set the `mmap`
    /// option for this to work.
    pub fn mmap2(&mut self, mmap2: bool) -> &mut Self {
        self.attrs.set_mmap2(mmap2.into());
        self
    }

    /// Check whether the kernel will annotate COMM records with the COMM_EXEC
    /// bit when they occur due to an `execve(2)` call.
    ///
    /// This option doesn't actually change the behaviour of the kernel.
    /// Instead, it is useful for feature detection.
    pub fn comm_exec(&mut self, comm_exec: bool) -> &mut Self {
        self.attrs.set_comm_exec(comm_exec.into());
        self
    }

    /// Select which linux clock to use for timestamps.
    ///
    /// If `clockid` is `None` then the kernel will use an internal timer. This
    /// timer may not be any of the options for clockid.
    ///
    /// See [`Clock`] and the [`clock_getttime(2)`][0] manpage for
    /// documentation on what the different clock values mean.
    ///
    /// [0]: https://man7.org/linux/man-pages/man2/clock_gettime.2.html
    pub fn clockid(&mut self, clockid: impl Into<Option<Clock>>) -> &mut Self {
        let clockid = clockid.into();
        self.attrs.set_use_clockid(clockid.is_some().into());
        self.attrs.clockid = clockid.map(Clock::into_raw).unwrap_or(0);
        self
    }

    /// Generate `SWITCH` records when a context switch occurs.
    ///
    /// Also enables the generation of `SWITCH_CPU_WIDE` records if profiling
    /// in cpu-wide mode.
    pub fn context_switch(&mut self, context_switch: bool) -> &mut Self {
        self.attrs.set_context_switch(context_switch.into());
        self
    }

    /// Generate `NAMESPACES` records when a task enters a new namespace.
    pub fn namespaces(&mut self, namespaces: bool) -> &mut Self {
        self.attrs.set_namespaces(namespaces.into());
        self
    }

    /// Generate `KSYMBOL` records when kernel symbols are registered or
    /// unregistered.
    pub fn ksymbol(&mut self, ksymbol: bool) -> &mut Self {
        self.attrs.set_ksymbol(ksymbol.into());
        self
    }

    /// Generate `BPF_EVENT` records when eBPF programs are loaded or unloaded.
    pub fn bpf_event(&mut self, bpf_event: bool) -> &mut Self {
        self.attrs.set_bpf_event(bpf_event.into());
        self
    }

    /// Output data for non-aux events to the aux buffer, if supported by the
    /// hardware.
    pub fn aux_output(&mut self, aux_output: bool) -> &mut Self {
        self.attrs.set_aux_output(aux_output.into());
        self
    }

    /// Generate `CGROUP` records when a new cgroup is created.
    pub fn cgroup(&mut self, cgroup: bool) -> &mut Self {
        self.attrs.set_cgroup(cgroup.into());
        self
    }

    /// Generate `TEXT_POKE` records when the kernel text (i.e. code) is
    /// modified.
    pub fn text_poke(&mut self, text_poke: bool) -> &mut Self {
        self.attrs.set_text_poke(text_poke.into());
        self
    }

    /// Whether to include the build id in `MMAP2` events.
    pub fn build_id(&mut self, build_id: bool) -> &mut Self {
        self.attrs.set_build_id(build_id.into());
        self
    }

    /// Only inherit the counter to new threads in the same process, not to
    /// other processes.
    pub fn inherit_thread(&mut self, inherit_thread: bool) -> &mut Self {
        self.attrs.set_inherit_thread(inherit_thread.into());
        self
    }

    /// Disable this counter when it successfully calls `execve(2)`.
    pub fn remove_on_exec(&mut self, remove_on_exec: bool) -> &mut Self {
        self.attrs.set_remove_on_exec(remove_on_exec.into());
        self
    }

    /// Synchronously send `SIGTRAP` to the process that created the counter
    /// when the sampled events overflow.
    pub fn sigtrap(&mut self, sigtrap: bool) -> &mut Self {
        self.attrs.set_sigtrap(sigtrap.into());
        self
    }
}

impl fmt::Debug for Builder<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Builder")
            .field("attrs", &self.attrs)
            .field("who", &self.who)
            .field("cpu", &self.cpu)
            .field(
                "event_data",
                &self.event_data.as_ref().map(|_| "<dyn EventData>"),
            )
            .finish()
    }
}

/// Attempted to build a counter using options that the current kernel does not
/// support.
///
/// This error is returned as the inner error from [`Builder::build`] or
/// [`Group::add`] if the kernel indicates that the [`perf_event_attr`]
/// arguments contained options that the current kernel does not support.
///
/// This can be used to implement feature detection and fall back to a config
/// which uses fewer options.
///
/// [`Group::add`]: crate::Group::add
///
/// # Example
/// ```
/// use perf_event::events::Software;
/// use perf_event::{Builder, UnsupportedOptionsError};
///
/// let mut builder = Builder::new(Software::DUMMY);
///
/// // The linux kernel will always return E2BIG when the size is less than
/// // PERF_ATTR_SIZE_VER0 (64) except if it is 0. This allows us to easily
/// // make an invalid call do figure out what size the kernel is expecting.
/// builder.attrs_mut().size = 1;
///
/// let error = builder.build().unwrap_err();
///
/// assert_eq!(error.kind(), std::io::ErrorKind::Unsupported);
/// assert_eq!(error.raw_os_error(), None);
///
/// let inner: &UnsupportedOptionsError = error.get_ref().unwrap().downcast_ref().unwrap();
///
/// println!("The expected size was {}", inner.expected_size());
/// ```
#[derive(Debug)]
pub struct UnsupportedOptionsError {
    expected_size: u32,
}

impl UnsupportedOptionsError {
    pub(crate) fn new(expected_size: u32) -> Self {
        Self { expected_size }
    }

    /// The size that the kernel expected the [`perf_event_attr`] struct to be.
    pub fn expected_size(&self) -> usize {
        self.expected_size as _
    }
}

impl fmt::Display for UnsupportedOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("perf_event_attr contained options not valid for the current kernel")
    }
}

impl std::error::Error for UnsupportedOptionsError {}
