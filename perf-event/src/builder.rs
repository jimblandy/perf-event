use std::ffi::c_int;
use std::ffi::c_ulong;
use std::fs::File;
use std::os::fd::AsRawFd;
use std::os::fd::FromRawFd;

use libc::pid_t;
use sys::bindings::perf_event_attr;

use crate::check_errno_syscall;
use crate::events;
use crate::events::Event;
use crate::flags::ReadFormat;
use crate::sys;
use crate::Clock;
use crate::Counter;
use crate::Group;
use crate::SampleBranchFlag;
use crate::SampleSkid;

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
///     # use perf_event::Builder;
///     # fn main() -> std::io::Result<()> {
///     let mut insns = Builder::new().build()?;
///     # Ok(()) }
///
/// The [`kind`] method lets you specify what sort of event you want to
/// count. So if you'd rather count branch instructions:
///
///     # use perf_event::Builder;
///     # use perf_event::events::Hardware;
///     # fn main() -> std::io::Result<()> {
///     let mut insns = Builder::new()
///         .kind(Hardware::BRANCH_INSTRUCTIONS)
///         .build()?;
///     # Ok(()) }
///
/// The [`group`] method lets you gather individual counters into `Group`
/// that can be enabled or disabled atomically:
///
///     # use perf_event::{Builder, Group};
///     # use perf_event::events::Hardware;
///     # fn main() -> std::io::Result<()> {
///     let mut group = Group::new()?;
///     let cycles = Builder::new().group(&mut group).kind(Hardware::CPU_CYCLES).build()?;
///     let insns = Builder::new().group(&mut group).kind(Hardware::INSTRUCTIONS).build()?;
///     # Ok(()) }
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
/// [`kind`]: Builder::kind
/// [`group`]: Builder::group
pub struct Builder<'a> {
    attrs: perf_event_attr,
    who: EventPid<'a>,
    cpu: Option<usize>,
    group: Option<&'a mut Group>,
}

impl<'a> Default for Builder<'a> {
    fn default() -> Builder<'a> {
        let mut attrs = perf_event_attr {
            // Setting `size` accurately will not prevent the code from working
            // on older kernels. The module comments for `perf_event_open_sys`
            // explain why in far too much detail.
            size: std::mem::size_of::<perf_event_attr>() as u32,
            ..perf_event_attr::default()
        };

        attrs.set_disabled(1);
        attrs.set_exclude_kernel(1); // don't count time in kernel
        attrs.set_exclude_hv(1); // don't count time in hypervisor

        // Request data for `time_enabled` and `time_running`.
        attrs.read_format |= sys::bindings::PERF_FORMAT_TOTAL_TIME_ENABLED as u64
            | sys::bindings::PERF_FORMAT_TOTAL_TIME_RUNNING as u64;

        let kind = Event::Hardware(events::Hardware::INSTRUCTIONS);
        kind.update_attrs(&mut attrs);

        Builder {
            attrs,
            who: EventPid::ThisProcess,
            cpu: None,
            group: None,
        }
    }
}

// Methods that actually do work on the builder and aren't just setting
// config values.
impl<'a> Builder<'a> {
    /// Return a new `Builder`, with all parameters set to their defaults.
    pub fn new() -> Builder<'a> {
        Builder::default()
    }

    /// Count events of the given kind. This accepts an [`Event`] value,
    /// or any type that can be converted to one, so you can pass [`Hardware`],
    /// [`Software`] and [`Cache`] values directly.
    ///
    /// The default is to count retired instructions, or
    /// `Hardware::INSTRUCTIONS` events.
    ///
    /// For example, to count level 1 data cache references and misses, pass the
    /// appropriate `events::Cache` values:
    ///
    ///     # fn main() -> std::io::Result<()> {
    ///     use perf_event::{Builder, Group};
    ///     use perf_event::events::{Cache, CacheOp, CacheResult, WhichCache};
    ///
    ///     const ACCESS: Cache = Cache {
    ///         which: WhichCache::L1D,
    ///         operation: CacheOp::READ,
    ///         result: CacheResult::ACCESS,
    ///     };
    ///     const MISS: Cache = Cache { result: CacheResult::MISS, ..ACCESS };
    ///
    ///     let mut group = Group::new()?;
    ///     let access_counter = Builder::new().group(&mut group).kind(ACCESS).build()?;
    ///     let miss_counter = Builder::new().group(&mut group).kind(MISS).build()?;
    ///     # Ok(()) }
    ///
    /// [`Hardware`]: events::Hardware
    /// [`Software`]: events::Software
    /// [`Cache`]: events::Cache
    pub fn kind<K: Into<Event>>(mut self, kind: K) -> Builder<'a> {
        let kind = kind.into();
        kind.update_attrs(&mut self.attrs);
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
    pub fn build(mut self) -> std::io::Result<Counter> {
        let cpu = match self.cpu {
            Some(cpu) => cpu as c_int,
            None => -1,
        };
        let (pid, flags) = self.who.as_args();
        let group_fd = match self.group {
            Some(ref mut g) => {
                g.max_members += 1;
                g.file.as_raw_fd() as c_int
            }
            None => -1,
        };

        let file = unsafe {
            File::from_raw_fd(check_errno_syscall(|| {
                sys::perf_event_open(&mut self.attrs, pid, cpu, group_fd, flags as c_ulong)
            })?)
        };

        // If we're going to be part of a Group, retrieve the ID the kernel
        // assigned us, so we can find our results in a Counts structure. Even
        // if we're not part of a group, we'll use it in `Debug` output.
        let mut id = 0_u64;
        check_errno_syscall(|| unsafe { sys::ioctls::ID(file.as_raw_fd(), &mut id) })?;

        Ok(Counter::new(file, id))
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
    pub fn observe_self(mut self) -> Builder<'a> {
        self.who = EventPid::ThisProcess;
        self
    }

    /// Observe the process with the given process id. This requires
    /// [`CAP_SYS_PTRACE`][man-capabilities] capabilities.
    ///
    /// [man-capabilities]: http://man7.org/linux/man-pages/man7/capabilities.7.html
    pub fn observe_pid(mut self, pid: pid_t) -> Builder<'a> {
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
    pub fn any_pid(mut self) -> Builder<'a> {
        self.who = EventPid::Any;
        self
    }

    /// Observe code running in the given [cgroup][man-cgroups] (container). The
    /// `cgroup` argument should be a `File` referring to the cgroup's directory
    /// in the cgroupfs filesystem.
    ///
    /// [man-cgroups]: http://man7.org/linux/man-pages/man7/cgroups.7.html
    pub fn observe_cgroup(mut self, cgroup: &'a File) -> Builder<'a> {
        self.who = EventPid::CGroup(cgroup);
        self
    }

    /// Observe only code running on the given CPU core.
    pub fn one_cpu(mut self, cpu: usize) -> Builder<'a> {
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
    pub fn any_cpu(mut self) -> Builder<'a> {
        self.cpu = None;
        self
    }

    /// Place the counter in the given [`Group`]. Groups allow a set of counters
    /// to be enabled, disabled, or read as a single atomic operation, so that
    /// the counts can be usefully compared.
    ///
    /// [`Group`]: struct.Group.html
    pub fn group(mut self, group: &'a mut Group) -> Builder<'a> {
        self.group = Some(group);

        // man page: "Members of a group are usually initialized with disabled
        // set to zero."
        self.attrs.set_disabled(0);

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
        self.attrs.sample_period = period;
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
        self.attrs.sample_freq = frequency;
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
        self.attrs.wakeup_watermark = watermark as _;
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
    /// [manpage]: https://www.mankier.com/2/perf_event_open
    /// [`wakeup_watermark`]: Builder::wakeup_watermark
    /// [`Sampler::next_blocking`]: crate::Sampler::next_blocking
    pub fn wakeup_events(&mut self, events: usize) -> &mut Self {
        self.attrs.set_watermark(0);
        self.attrs.wakeup_events = events as _;
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
    /// [manpage]: https://www.mankier.com/2/perf_event_open
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
    /// [manpage]: https://www.mankier.com/2/perf_event_open
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
    /// [manpage]: https://www.mankier.com/2/perf_event_open
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
    /// [0]: https://www.mankier.com/2/clock_gettime
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

    /// Copy data to the user's signal handler (via `si_perf` in `siginfo_t`).
    ///
    /// This can be used to figure out which event caused the signal to be sent.
    /// It does nothing unless [`sigtrap`](Self::sigtrap) is also set to `true`.
    pub fn sig_data(&mut self, sig_data: u64) -> &mut Self {
        self.attrs.sig_data = sig_data;
        self
    }

    /// Specify which branches to include in the branch record.
    ///
    /// This does nothing unless [`SampleFlag::BRANCH_STACK`] is specified in
    /// the sample flags.
    pub fn branch_sample_type(&mut self, flags: SampleBranchFlag) -> &mut Self {
        self.attrs.branch_sample_type = flags.bits();
        self
    }

    /// Specify which CPU registers to dump in a sample.
    ///
    /// This does nothing unless [`SampleFlag::REGS_USER`] is part of the
    /// specified [`sample`](Builder::sample) flags.
    ///
    /// The actual layout of the register mask is architecture specific.
    /// You will generally want the `PERF_REG_<arch>` constants in
    /// [`perf_event_open_sys`]. (e.g. `PERF_REG_X86_SP`).
    pub fn sample_regs_user(&mut self, regs: u64) -> &mut Self {
        self.attrs.sample_regs_user = regs;
        self
    }

    /// Specify which CPU registers to dump in a sample.
    ///
    /// This does nothing unless [`SampleFlag::REGS_INTR`] is part of the
    /// specified [`sample`](Builder::sample) flags.
    ///
    /// The actual layout of the register mask is architecture specific.
    /// You will generally want the `PERF_REG_<arch>` constants in
    /// [`perf_event_open_sys`]. (e.g. `PERF_REG_X86_SP`).
    pub fn sample_regs_intr(&mut self, regs: u64) -> &mut Self {
        self.attrs.sample_regs_user = regs;
        self
    }

    /// Specify the maximum size of the user stack to dump.
    ///
    /// This option does nothing unless [`SampleFlag::STACK_USER`] is set in the
    /// sample flags.
    ///
    /// Note that the size of the array allocated within the sample record will
    /// always be exactly this size, even if the actual collected stack data is
    /// much smaller. The allocated sample buffer (when constructing a
    /// [`Sampler`]) will need to be large enough to accommodate the chosen
    /// stack size or else samples will be lost.
    ///
    /// [`Sampler`]: crate::Sampler
    pub fn sample_stack_user(&mut self, stack: u32) -> &mut Self {
        self.attrs.sample_stack_user = stack;
        self
    }

    /// Specify the maximum number of stack frames to include when unwinding the
    /// user stack.
    ///
    /// This does nothing unless [`SampleFlag::CALLCHAIN`] is set in the sample
    /// flags.
    ///
    /// Note that the kernel has a user configurable limit specified at
    /// `/proc/sys/kernel/perf_event_max_stack`. Setting `sample_max_stack` to
    /// larger than that limit will result in an `EOVERFLOW` error when building
    /// the counter.
    pub fn sample_max_stack(&mut self, max_stack: u16) -> &mut Self {
        self.attrs.sample_max_stack = max_stack;
        self
    }

    /// Specify how much data is required before the kernel emits an AUX record.
    pub fn aux_watermark(&mut self, watermark: u32) -> &mut Self {
        self.attrs.aux_watermark = watermark;
        self
    }

    /// Specify the desired size of AUX data.
    ///
    /// This does nothing unless [`SampleFlag::AUX`] is set in the sample flags.
    /// Note that the emitted aux data can be smaller than the requested size.
    pub fn aux_sample_size(&mut self, sample_size: u32) -> &mut Self {
        self.attrs.aux_sample_size = sample_size;
        self
    }
}

#[derive(Debug)]
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
