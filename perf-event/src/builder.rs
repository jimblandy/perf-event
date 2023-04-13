use std::fs::File;
use std::os::raw::{c_int, c_ulong};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};

use libc::pid_t;

use crate::events::Event;
use crate::sys::bindings::perf_event_attr;
use crate::{check_errno_syscall, sys, Counter, SampleFlag};

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
/// -   specific processes or cgroups to observe
/// -   specific CPU cores to observe
///
/// `Builder` supports only a fraction of the many knobs and dials Linux offers,
/// but hopefully it will acquire methods to support more of them as time goes
/// on.
///
/// Internally, a `Builder` is just a wrapper around the kernel's `struct
/// perf_event_attr` type.
///
/// [`enable`]: Counter::enable
/// [`Group`]: crate::Group
/// [`Group::add`]: crate::Group::add
#[derive(Clone, Debug)]
pub struct Builder<'a> {
    attrs: perf_event_attr,
    who: EventPid<'a>,
    cpu: Option<usize>,
}

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

impl<'a> Builder<'a> {
    /// Return a new `Builder`, with all parameters set to their defaults.
    ///
    /// Return a new `Builder` for the specified event.
    pub fn new(event: impl Event) -> Self {
        let mut attrs = perf_event_attr::default();

        // Do the update_attrs bit before we set any of the default state so
        // that user code can't break configuration we really care about.
        event.update_attrs(&mut attrs);

        // Setting `size` accurately will not prevent the code from working
        // on older kernels. The module comments for `perf_event_open_sys`
        // explain why in far too much detail.
        attrs.size = std::mem::size_of::<perf_event_attr>() as u32;
        attrs.set_disabled(1);

        attrs.read_format = sys::bindings::PERF_FORMAT_TOTAL_TIME_ENABLED as u64
            | sys::bindings::PERF_FORMAT_TOTAL_TIME_RUNNING as u64;

        Self {
            attrs,
            who: EventPid::ThisProcess,
            cpu: None,
        }
    }

    /// Include kernel code.
    pub fn include_kernel(&mut self) -> &mut Self {
        self.attrs.set_exclude_kernel(0);
        self
    }

    /// Include hypervisor code.
    pub fn include_hv(&mut self) -> &mut Self {
        self.attrs.set_exclude_hv(0);
        self
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
        let flag = if inherit { 1 } else { 0 };
        self.attrs.set_inherit(flag);
        self
    }

    /// Construct a [`Counter`] according to the specifications made on this
    /// `Builder`.
    ///
    /// A freshly built `Counter` is disabled. To begin counting events, you
    /// must call [`enable`] on the `Counter` or the `Group` to which it belongs.
    ///
    /// If the `Builder` requests features that the running kernel does not
    /// support, it returns `Err(e)` where `e.kind() == ErrorKind::Other` and
    /// `e.raw_os_error() == Some(libc::E2BIG)`.
    ///
    /// Unfortunately, problems in counter configuration are detected at this
    /// point, by the kernel, not earlier when the offending request is made on
    /// the `Builder`. The kernel's returned errors are not always helpful.
    ///
    /// [`Counter`]: struct.Counter.html
    /// [`enable`]: struct.Counter.html#method.enable
    pub fn build(&self) -> std::io::Result<Counter> {
        self.build_with_group(None)
    }

    /// Alternative to `build` but with the group explicitly provided.
    ///
    /// Used within [`Group::add`].
    pub(crate) fn build_with_group(&self, group_fd: Option<RawFd>) -> std::io::Result<Counter> {
        let cpu = match self.cpu {
            Some(cpu) => cpu as c_int,
            None => -1,
        };

        let (pid, flags) = self.who.as_args();
        let group_fd = group_fd.unwrap_or(-1);

        let mut attrs = self.attrs;

        let file = unsafe {
            File::from_raw_fd(check_errno_syscall(|| {
                sys::perf_event_open(&mut attrs, pid, cpu, group_fd, flags as c_ulong)
            })?)
        };

        // If we're going to be part of a Group, retrieve the ID the kernel
        // assigned us, so we can find our results in a Counts structure. Even
        // if we're not part of a group, we'll use it in `Debug` output.
        let mut id = 0_u64;
        check_errno_syscall(|| unsafe { sys::ioctls::ID(file.as_raw_fd(), &mut id) })?;

        Ok(Counter { file, id })
    }
}

impl<'a> Builder<'a> {
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

    /// Enable the generation of MMAP records.
    ///
    /// MMAP records are emitted when the process/thread that is being
    /// observed creates a new executable memory mapping.
    pub fn mmap(&mut self, mmap: bool) -> &mut Self {
        self.attrs.set_mmap(mmap.into());
        self
    }

    /// Set how many bytes will be written before the kernel sends an overflow
    /// notification.
    ///
    /// Note only one of `wakeup_watermark` and [`wakeup_events`] can be
    /// configured.
    ///
    /// [`wakeup_events`]: Self::wakeup_events
    pub fn wakeup_watermark(&mut self, watermark: usize) -> &mut Self {
        self.attrs.set_watermark(1);
        self.attrs.__bindgen_anon_2.wakeup_watermark = watermark as _;
        self
    }

    /// Set how many samples will be written before the kernel sends an
    /// overflow notification.
    ///
    /// Note only one of [`wakeup_watermark`] and `wakeup_events` can be
    /// configured.
    ///
    /// Some caveats apply, see the [manpage] for the full documentation.
    ///
    /// [manpage]: https://man7.org/linux/man-pages/man2/perf_event_open.2.html
    /// [`wakeup_watermark`]: Self::wakeup_watermark
    pub fn wakeup_events(&mut self, events: usize) -> &mut Self {
        self.attrs.set_watermark(0);
        self.attrs.__bindgen_anon_2.wakeup_events = events as _;
        self
    }
}
