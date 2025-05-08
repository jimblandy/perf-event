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
use crate::sys;
use crate::Counter;
use crate::Group;

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

impl<'a> Builder<'a> {
    /// Return a new `Builder`, with all parameters set to their defaults.
    pub fn new() -> Builder<'a> {
        Builder::default()
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

    /// Include kernel code.
    pub fn include_kernel(mut self) -> Builder<'a> {
        self.attrs.set_exclude_kernel(0);
        self
    }

    /// Include hypervisor code.
    pub fn include_hv(mut self) -> Builder<'a> {
        self.attrs.set_exclude_hv(0);
        self
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
    pub fn inherit(mut self, inherit: bool) -> Builder<'a> {
        let flag = if inherit { 1 } else { 0 };
        self.attrs.set_inherit(flag);
        self
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
