//! A performance monitoring API for Linux.
//!
//! This crate provides access to processor and kernel counters for things like
//! instruction completions, cache references and misses, branch predictions,
//! context switches, page faults, and so on.
//!
//! For example, to compare the number of clock cycles elapsed with the number
//! of instructions completed during one call to `println!`:
//!
//!     use perf_event::{Builder, Group};
//!     use perf_event::events::Hardware;
//!
//!     fn main() -> std::io::Result<()> {
//!         // A `Group` lets us enable and disable several counters atomically.
//!         let mut group = Group::new()?;
//!         let cycles = Builder::new().group(&mut group).kind(Hardware::CPU_CYCLES).build()?;
//!         let insns = Builder::new().group(&mut group).kind(Hardware::INSTRUCTIONS).build()?;
//!
//!         let vec = (0..=51).collect::<Vec<_>>();
//!
//!         group.enable()?;
//!         println!("{:?}", vec);
//!         group.disable()?;
//!
//!         let counts = group.read()?;
//!         println!("cycles / instructions: {} / {} ({:.2} cpi)",
//!                  counts[&cycles],
//!                  counts[&insns],
//!                  (counts[&cycles] as f64 / counts[&insns] as f64));
//!
//!         Ok(())
//!     }
//!
//! This crate is built on top of the Linux [`perf_event_open`][man] system
//! call; that documentation has the authoritative explanations of exactly what
//! all the counters mean.
//!
//! There are two main types for measurement:
//!
//! -   A [`Counter`] is an individual counter. Use [`Builder`] to
//!     construct one.
//!
//! -   A [`Group`] is a collection of counters that can be enabled and
//!     disabled atomically, so that they cover exactly the same period of
//!     execution, allowing meaningful comparisons of the individual values.
//!
//! ### Call for PRs
//!
//! Linux's `perf_event_open` API can report all sorts of things this crate
//! doesn't yet understand: stack traces, logs of executable and shared library
//! activity, tracepoints, kprobes, uprobes, and so on. And beyond the counters
//! in the kernel header files, there are others that can only be found at
//! runtime by consulting `sysfs`, specific to particular processors and
//! devices. For example, modern Intel processors have counters that measure
//! power consumption in Joules.
//!
//! If you find yourself in need of something this crate doesn't support, please
//! consider submitting a pull request. (We intend to steadily raise our
//! standards for testing and documentation, to ensure that technical
//! contributions can have enough impact on users to justify the cost of
//! inclusion, so be forewarned.)
//!
//! [`Counter`]: struct.Counter.html
//! [`Builder`]: struct.Builder.html
//! [`Group`]: struct.Group.html
//! [man]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html

#![deny(missing_docs)]

use events::Event;
use libc::pid_t;
use perf_event_open_sys as sys;
use std::fs::File;
use std::io::{self, Read};
use std::os::raw::{c_int, c_uint, c_ulong};
use std::os::unix::io::{AsRawFd, FromRawFd};

pub mod events;

/// A counter for one kind of kernel or hardware event.
///
/// A `Counter` represents a single performance monitoring counter. You select
/// what sort of event you'd like to count when the `Counter` is created, then
/// you can enable and disable the counter, call its [`read`] method to
/// retrieve the current count, and reset it to zero.
///
/// A `Counter`'s value is always a `u64`.
///
/// For example, this counts the number of instructions retired (completed)
/// during a call to `println!`.
///
///     use perf_event::Builder;
///
///     fn main() -> std::io::Result<()> {
///         let mut counter = Builder::new().build()?;
///
///         let vec = (0..=51).collect::<Vec<_>>();
///
///         counter.enable()?;
///         println!("{:?}", vec);
///         counter.disable()?;
///
///         println!("{} instructions retired", counter.read()?);
///
///         Ok(())
///     }
///
/// It is often useful to count several different quantities over the same
/// period of time. For example, if you want to measure the average number of
/// clock cycles used per instruction, you must count both clock cycles and
/// instructions retired, for the same range of execution. The [`Group`] type
/// lets you enable, disable, read, and reset any number of counters
/// simultaneously.
///
/// When a counter is dropped, its kernel resources are freed along with it.
///
/// [`Group`]: struct.Group.html
/// [`read`]: #method.read
pub struct Counter {
    /// The file descriptor for this counter, returned by `perf_event_open`.
    ///
    /// When a `Counter` is dropped, this `File` is dropped, and the kernel
    /// removes the counter from any group it belongs to.
    file: File,

    /// The unique id assigned to this counter by the kernel.
    id: u64,
}

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
/// [`Counter`]: struct.Counter.html
/// [`enable`]: struct.Counter.html#method.enable
/// [`kind`]: #method.kind
/// [`group`]: #method.group
pub struct Builder<'a> {
    who: EventPid<'a>,
    cpu: Option<usize>,
    kind: Event,
    group: Option<&'a mut Group>,
}

#[derive(Debug)]
enum EventPid<'a> {
    /// Monitor the calling process.
    ThisProcess,

    /// Monitor the given pid.
    Other(pid_t),

    /// Monitor members of the given cgroup.
    CGroup(&'a File),
}

/// A group of counters that can be managed as a unit.
///
/// A `Group` represents a group of [`Counter`s] that can be enabled,
/// disabled, reset, or read as a single atomic operation. This is necessary if
/// you want to compare counter values, produce ratios, and so on, since those
/// operations are only meaningful on counters that cover exactly the same
/// period of execution.
///
/// A `Counter` is placed in a group when it is created, by calling the
/// `Builder`'s [`group`] method. A `Group`'s [`read`] method returns values
/// of all its member counters at once as a [`Counts`] value, which can be
/// indexed by `Counter` to retrieve a specific value.
///
/// For example, the following program computes the average number of cycles
/// used per instruction retired for a call to `println!`:
///
///     # fn main() -> std::io::Result<()> {
///     use perf_event::{Builder, Group};
///     use perf_event::events::Hardware;
///
///     let mut group = Group::new()?;
///     let cycles = Builder::new().group(&mut group).kind(Hardware::CPU_CYCLES).build()?;
///     let insns = Builder::new().group(&mut group).kind(Hardware::INSTRUCTIONS).build()?;
///
///     let vec = (0..=51).collect::<Vec<_>>();
///
///     group.enable()?;
///     println!("{:?}", vec);
///     group.disable()?;
///
///     let counts = group.read()?;
///     println!("cycles / instructions: {} / {} ({:.2} cpi)",
///              counts[&cycles],
///              counts[&insns],
///              (counts[&cycles] as f64 / counts[&insns] as f64));
///     # Ok(()) }
///
/// The lifetimes of `Counter`s and `Group`s are independent: placing a
/// `Counter` in a `Group` does not take ownership of the `Counter`, nor must
/// the `Counter`s in a group outlive the `Group`. If a `Counter` is dropped, it
/// is simply removed from its `Group`, and omitted from future results. If a
/// `Group` is dropped, its individual counters continue to count.
///
/// Enabling or disabling a `Group` affects each `Counter` that belongs to it.
/// Subsequent reads from the `Counter` will not reflect activity while the
/// `Group` was disabled, unless the `Counter` is re-enabled individually.
///
/// A `Group` and its members must all observe the same tasks and cpus; mixing
/// these makes building the `Counter` return an error. Unfortunately, there is
/// no way at present to specify a `Group`s task and cpu, so you can only use
/// `Group` on the calling task.
///
/// ## Limits on group size
///
/// Hardware counters are implemented using special-purpose registers on the
/// processor, of which there are only a fixed number. Without using groups, if
/// you request more hardware counters than the processor can actually support,
/// a complete count isn't possible, but the kernel will rotate the processor's
/// real registers amongst the measurements you've requested to at least produce
/// a sample.
///
/// But since the point of a counter group is that its members must cover
/// exactly the same period of time, this tactic can't be applied to support
/// large groups. If the kernel cannot schedule a group, its counters remain
/// zero. I think you can detect this situation by comparing the group's
/// 'enabled' and 'running' times, but this crate doesn't support those yet; see
/// [#5].
///
/// According to the `perf_list(1)` man page, you may be able to free up a
/// hardware counter by disabling the kernel's NMI watchdog, which reserves one
/// for detecting kernel hangs:
///
/// ```ignore
/// $ echo 0 > /proc/sys/kernel/nmi_watchdog
/// ```
///
/// You can reenable the watchdog when you're done like this:
///
/// ```ignore
/// $ echo 1 > /proc/sys/kernel/nmi_watchdog
/// ```
///
/// [`Counter`s]: struct.Counter.html
/// [`group`]: struct.Builder.html#method.group
/// [`read`]: #method.read
/// [`Counts`]: struct.Counts.html
/// [`#5`]: https://github.com/jimblandy/perf-event/issues/5
pub struct Group {
    /// The file descriptor for this counter, returned by `perf_event_open`.
    /// This counter itself is for the dummy software event, so it's not
    /// interesting.
    file: File,

    /// The unique id assigned to this group by the kernel. We only use this for
    /// assertions.
    id: u64,

    /// An upper bound on the number of Counters in this group. This lets us
    /// allocate buffers of sufficient size for for PERF_FORMAT_GROUP reads.
    ///
    /// There's no way to ask the kernel how many members a group has. And if we
    /// pass a group read a buffer that's too small, the kernel won't just
    /// return a truncated result; it returns ENOSPC and leaves the buffer
    /// untouched. So the buffer just has to be large enough.
    ///
    /// Since we're borrowed while building group members, adding members can
    /// increment this counter. But it's harder to decrement it when a member
    /// gets dropped: we don't require that a Group outlive its members, so they
    /// can't necessarily update their `Group`'s count from a `Drop` impl. So we
    /// just increment, giving us an overestimate, and then correct the count
    /// when we actually do a read.
    ///
    /// This includes the dummy counter for the group itself.
    max_members: usize
}

/// A collection of counts from a [`Group`] of counters.
///
/// This is the type returned by calling [`read`] on a [`Group`].
/// You can index it with a reference to a specific `Counter`:
///
///     # fn main() -> std::io::Result<()> {
///     # use perf_event::{Builder, Group};
///     # let mut group = Group::new()?;
///     # let cycles = Builder::new().group(&mut group).build()?;
///     # let insns = Builder::new().group(&mut group).build()?;
///     let counts = group.read()?;
///     println!("cycles / instructions: {} / {} ({:.2} cpi)",
///              counts[&cycles],
///              counts[&insns],
///              (counts[&cycles] as f64 / counts[&insns] as f64));
///     # Ok(()) }
///
/// Or you can iterate over the results it contains:
///
///     # fn main() -> std::io::Result<()> {
///     # use perf_event::Group;
///     # let counts = Group::new()?.read()?;
///     for (id, value) in &counts {
///         println!("Counter id {} has value {}", id, value);
///     }
///     # Ok(()) }
///
/// The `id` values produced by this iteration are internal identifiers assigned
/// by the kernel. You can use the [`Counter::id`] method to find a
/// specific counter's id.
///
/// [`Group`]: struct.Group.html
/// [`read`]: struct.Group.html#method.read
/// [`Counter::id`]: struct.Counter.html#method.id
pub struct Counts {
    // Raw results from the `read`.
    data: Vec<u64>
}

impl<'a> EventPid<'a> {
    // Return the `pid` arg and the `flags` bits representing `self`.
    fn as_args(&self) -> (pid_t, u32) {
        match self {
            EventPid::ThisProcess => (0, 0),
            EventPid::Other(pid) => (*pid, 0),
            EventPid::CGroup(file) =>
                (file.as_raw_fd(), sys::bindings::PERF_FLAG_PID_CGROUP),
        }
    }
}

impl<'a> Default for Builder<'a> {
    fn default() -> Builder<'a> {
        Builder {
            who: EventPid::ThisProcess,
            cpu: None,
            kind: Event::Hardware(events::Hardware::INSTRUCTIONS),
            group: None,
        }
    }
}

impl<'a> Builder<'a> {
    /// Return a new `Builder`, with all parameters set to their defaults.
    pub fn new() -> Builder<'a> {
        Builder::default()
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
    pub fn any_cpu(mut self) -> Builder<'a> {
        self.cpu = None;
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
    /// [`Event`]: events/enum.Event.html
    /// [`Hardware`]: events/enum.Hardware.html
    /// [`Software`]: events/enum.Software.html
    /// [`Cache`]: events/struct.Cache.html
    pub fn kind<K: Into<Event>>(mut self, kind: K) -> Builder<'a> {
        self.kind = kind.into();
        self
    }

    /// Place the counter in the given [`Group`]. Groups allow a set of counters
    /// to be enabled, disabled, or read as a single atomic operation, so that
    /// the counts can be usefully compared.
    ///
    /// [`Group`]: struct.Group.html
    pub fn group(mut self, group: &'a mut Group) -> Builder<'a> {
        self.group = Some(group);
        self
    }

    /// Construct a [`Counter`] according to the specifications made on this
    /// `Builder`.
    ///
    /// A freshly built `Counter` is disabled. To begin counting events, you
    /// must call [`enable`] on the `Counter` or the `Group` to which it belongs.
    ///
    /// Unfortunately, problems in counter configuration are detected at this
    /// point, by the kernel, not earlier when the offending request is made on
    /// the `Builder`. The kernel's returned errors are not always helpful.
    ///
    /// [`Counter`]: struct.Counter.html
    /// [`enable`]: struct.Counter.html#method.enable
    pub fn build(self) -> std::io::Result<Counter> {
        let cpu = match self.cpu {
            Some(cpu) => cpu as c_int,
            None => -1,
        };
        let (pid, flags) = self.who.as_args();
        let group_fd = match self.group {
            Some(g) => {
                g.max_members += 1;
                g.file.as_raw_fd() as c_int
            }
            None => -1,
        };

        let mut attrs = sys::bindings::perf_event_attr::default();

        // Setting `size` accurately will not prevent the code from working on
        // older kernels. The module comments for `perf_event_open_sys` explain
        // why in detail.
        attrs.size = std::mem::size_of::<sys::bindings::perf_event_attr>() as u32;

        attrs.type_ = self.kind.as_type();
        attrs.config = self.kind.as_config();
        attrs.set_disabled(if group_fd != -1 {
            // man page: "Members of a group are usually initialized with
            // disabled set to zero."
            0
        } else {
            1
        });
        attrs.set_disabled(1);
        attrs.set_exclude_kernel(1);
        attrs.set_exclude_hv(1);

        let file = unsafe {
            File::from_raw_fd(check_raw_syscall(|| {
                sys::perf_event_open(&mut attrs, pid, cpu, group_fd, flags as c_ulong)
            })?)
        };

        // If we're going to be part of a Group, retrieve the ID the kernel
        // assigned us, so we can find our results in a Counts structure. Even
        // if we're not part of a group, we'll use it in `Debug` output.
        let mut id = 0_64;
        check_errno_syscall(|| unsafe {
            sys::ioctls::ID(file.as_raw_fd(), &mut id)
        })?;

        Ok(Counter { file, id, })
    }
}

impl Counter {
    /// Return this counter's kernel-assigned unique id.
    ///
    /// This can be useful when iterating over [`Counts`].
    ///
    /// [`Counts`]: struct.Counts.html
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Allow this `Counter` to begin counting its designated event.
    ///
    /// This does not affect whatever value the `Counter` had previously; new
    /// events add to the current count. To clear a `Counter`, use the
    /// [`reset`] method.
    ///
    /// Note that `Group` also has an [`enable`] method, which enables all
    /// its member `Counter`s as a single atomic operation.
    ///
    /// [`reset`]: #method.reset
    /// [`enable`]: struct.Group.html#method.enable
    pub fn enable(&mut self) -> io::Result<()> {
        check_errno_syscall(|| unsafe {
            sys::ioctls::ENABLE(self.file.as_raw_fd(), 0)
        }).map(|_| ())
    }

    /// Make this `Counter` stop counting its designated event. Its count is
    /// unaffected.
    ///
    /// Note that `Group` also has a [`disable`] method, which disables all
    /// its member `Counter`s as a single atomic operation.
    ///
    /// [`disable`]: struct.Group.html#method.disable
    pub fn disable(&mut self) -> io::Result<()> {
        check_errno_syscall(|| unsafe {
            sys::ioctls::DISABLE(self.file.as_raw_fd(), 0)
        }).map(|_| ())
    }

    /// Reset the value of this `Counter` to zero.
    ///
    /// Note that `Group` also has a [`reset`] method, which resets all
    /// its member `Counter`s as a single atomic operation.
    ///
    /// [`reset`]: struct.Group.html#method.reset
    pub fn reset(&mut self) -> io::Result<()> {
        check_errno_syscall(|| unsafe {
            sys::ioctls::RESET(self.file.as_raw_fd(), 0)
        }).map(|_| ())
    }

    /// Return this `Counter`'s current value as a `u64`.
    ///
    /// Note that `Group` also has a [`read`] method, which reads all
    /// its member `Counter`s' values at once.
    ///
    /// [`read`]: struct.Group.html#method.read
    pub fn read(&mut self) -> io::Result<u64> {
        let mut buf = [0_u8; 8];
        self.file.read_exact(&mut buf)?;
        Ok(u64::from_ne_bytes(buf))
    }
}

impl std::fmt::Debug for Counter {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "Counter {{ fd: {}, id: {} }}",
               self.file.as_raw_fd(), self.id)
    }
}

impl Group {
    /// Construct a new, empty `Group`.
    #[allow(unused_parens)]
    pub fn new() -> io::Result<Group> {
        // Open a placeholder perf counter that we can add other events to.
        let mut attrs = sys::bindings::perf_event_attr::default();
        attrs.type_ = sys::bindings::perf_type_id_PERF_TYPE_SOFTWARE;
        attrs.size = std::mem::size_of::<sys::bindings::perf_event_attr>() as u32;
        attrs.config = sys::bindings::perf_sw_ids_PERF_COUNT_SW_DUMMY as u64;
        attrs.set_disabled(1);
        attrs.set_exclude_kernel(1);
        attrs.set_exclude_hv(1);

        // Arrange to be able to identify the counters we read back.
        attrs.read_format = (sys::bindings::perf_event_read_format_PERF_FORMAT_ID |
                             sys::bindings::perf_event_read_format_PERF_FORMAT_GROUP) as u64;

        let file = unsafe {
            File::from_raw_fd(check_raw_syscall(|| {
                sys::perf_event_open(&mut attrs, 0, -1, -1, 0)
            })?)
        };

        // Retrieve the ID the kernel assigned us.
        let mut id = 0_64;
        check_errno_syscall(|| unsafe {
            sys::ioctls::ID(file.as_raw_fd(), &mut id)
        })?;

        Ok(Group { file, id, max_members: 1 })
    }

    /// Allow all `Counter`s in this `Group` to begin counting their designated
    /// events, as a single atomic operation.
    ///
    /// This does not affect whatever values the `Counter`s had previously; new
    /// events add to the current counts. To clear the `Counter`s, use the
    /// [`reset`] method.
    ///
    /// [`reset`]: #method.reset
    pub fn enable(&mut self) -> io::Result<()> {
        self.generic_ioctl(sys::ioctls::ENABLE)
    }

    /// Make all `Counter`s in this `Group` stop counting their designated
    /// events, as a single atomic operation. Their counts are unaffected.
    pub fn disable(&mut self) -> io::Result<()> {
        self.generic_ioctl(sys::ioctls::DISABLE)
    }

    /// Reset all `Counter`s in this `Group` to zero, as a single atomic operation.
    pub fn reset(&mut self) -> io::Result<()> {
        self.generic_ioctl(sys::ioctls::RESET)
    }

    /// Perform some group ioctl.
    ///
    /// `f` must be a syscall that sets `errno` and returns `-1` on failure.
    fn generic_ioctl(&mut self, f: unsafe fn(c_int, c_uint) -> c_int) -> io::Result<()> {
        check_errno_syscall(|| unsafe {
            f(self.file.as_raw_fd(),
              sys::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP)
        }).map(|_| ())
    }

    /// Return the values of all the `Counter`s in this `Group` as a [`Counts`]
    /// value.
    ///
    /// A `Counts` value is a map from specific `Counter`s to their values. You
    /// can find a specific `Counter`'s value by indexing:
    ///
    /// ```ignore
    /// let mut group = Group::new()?;
    /// let counter1 = Builder::new().group(&mut group).kind(...).build()?;
    /// let counter2 = Builder::new().group(&mut group).kind(...).build()?;
    /// ...
    /// let counts = group.read()?;
    /// println!("Rhombus inclinations per taxi medallion: {} / {} ({:.0}%)",
    ///          counts[&counter1],
    ///          counts[&counter2],
    ///          (counts[&counter1] as f64 / counts[&counter2] as f64) * 100.0);
    /// ```
    ///
    /// [`Counts`]: struct.Counts.html
    pub fn read(&mut self) -> io::Result<Counts> {
        // Since we passed PERF_FORMAT_ID | PERF_FORMAT_GROUP, the data we'll
        // read has the form:
        //
        //     struct read_format {
        //         u64 nr;            /* The number of events */
        //         u64 time_enabled;  /* if PERF_FORMAT_TOTAL_TIME_ENABLED */
        //         u64 time_running;  /* if PERF_FORMAT_TOTAL_TIME_RUNNING */
        //         struct {
        //             u64 value;     /* The value of the event */
        //             u64 id;        /* if PERF_FORMAT_ID */
        //         } values[nr];
        //     };
        //
        // We do not request the enabled and running times, so all we have are
        // the number of events and the value/id pairs.
        let mut data = vec![0_u64; 1 + 2 * self.max_members];
        self.file.read(u64::slice_as_bytes_mut(&mut data))?;

        let counts = Counts { data };

        // CountsIter assumes that the group's dummy count appears first.
        assert_eq!(counts.nth_ref(0).0, self.id);

        // Update `max_members` for the next read.
        self.max_members = counts.len();

        Ok(counts)
    }
}

impl std::fmt::Debug for Group {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "Group {{ fd: {}, id: {} }}",
               self.file.as_raw_fd(), self.id)
    }
}

impl Counts {
    fn len(&self) -> usize {
        self.data[0] as usize
    }

    fn nth_ref(&self, n: usize) -> (u64, &u64) {
        assert!(n < self.len());
        // (id, &value)
        (self.data[1 + 2 * n + 1],
         &self.data[1 + 2 * n])
    }
}

/// An iterator over the counter values in a [`Counts`], returned by
/// [`Group::read`].
///
/// Each item is a pair `(id, &value)`, where `id` is the number assigned to the
/// counter by the kernel (see `Counter::id`), and `value` is that counter's
/// value.
///
/// [`Counts`]: struct.Counts.html
/// [`Counter::id`]: struct.Counter.html#method.id
/// [`Group::read`]: struct.Group.html#method.read
pub struct CountsIter<'c> {
    counts: &'c Counts,
    next: usize
}

impl<'c> Iterator for CountsIter<'c> {
    type Item = (u64, &'c u64);
    fn next(&mut self) -> Option<(u64, &'c u64)> {
        if self.next >= self.counts.len() {
            return None;
        }
        let result = self.counts.nth_ref(self.next);
        self.next += 1;
        return Some(result);
    }
}

impl<'c> IntoIterator for &'c Counts {
    type Item = (u64, &'c u64);
    type IntoIter = CountsIter<'c>;
    fn into_iter(self) -> CountsIter<'c> {
        CountsIter {
            counts: self,
            next: 1, // skip the `Group` itself, it's just a dummy.
        }
    }
}

impl Counts {
    /// Return the value recorded for `member` in `self`, or `None` if `member`
    /// is not present.
    ///
    /// If you know that `member` is in the group, you can simply index:
    ///
    ///     # fn main() -> std::io::Result<()> {
    ///     # use perf_event::{Builder, Group};
    ///     # let mut group = Group::new()?;
    ///     # let cycle_counter = Builder::new().group(&mut group).build()?;
    ///     # let counts = group.read()?;
    ///     let cycles = counts[&cycle_counter];
    ///     # Ok(()) }
    pub fn get(&self, member: &Counter) -> Option<&u64> {
        self.into_iter()
            .find(|&(id, _)| id == member.id)
            .map(|(_, value)| value)
    }

    /// Return an iterator over the counts in `self`.
    ///
    ///     # fn main() -> std::io::Result<()> {
    ///     # use perf_event::Group;
    ///     # let counts = Group::new()?.read()?;
    ///     for (id, value) in &counts {
    ///         println!("Counter id {} has value {}", id, value);
    ///     }
    ///     # Ok(()) }
    ///
    /// Each item is a pair `(id, &value)`, where `id` is the number assigned to
    /// the counter by the kernel (see `Counter::id`), and `value` is that
    /// counter's value.
    pub fn iter(&self) -> CountsIter {
        <&Counts as IntoIterator>::into_iter(self)
    }
}

impl std::ops::Index<&Counter> for Counts {
    type Output = u64;
    fn index(&self, index: &Counter) -> &u64 {
        self.get(index).unwrap()
    }
}

impl std::fmt::Debug for Counts {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_map().entries(self.into_iter()).finish()
    }
}

unsafe trait SliceAsBytesMut: Sized {
    fn slice_as_bytes_mut(slice: &mut [Self]) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut u8,
                                           std::mem::size_of_val(slice))
        }
    }
}

unsafe impl SliceAsBytesMut for u64 { }

/// Produce an `io::Result` from a raw system call.
///
/// A 'raw' system call is one that reports failure by returning negated raw OS
/// error value.
fn check_raw_syscall<F>(f: F) -> io::Result<c_int>
where F: FnOnce() -> c_int
{
    let result = f();
    if result < 0 {
        Err(io::Error::from_raw_os_error(-result))
    } else {
        Ok(result)
    }
}

/// Produce an `io::Result` from an errno-style system call.
///
/// An 'errno-style' system call is one that reports failure by returning -1 and
/// setting the C `errno` value when an error occurs.
fn check_errno_syscall<F, R>(f: F) -> io::Result<R>
where F: FnOnce() -> R,
      R: PartialOrd + Default
{
    let result = f();
    if result < R::default() {
        Err(io::Error::last_os_error())
    } else {
        Ok(result)
    }
}

#[test]
fn simple_build() {
    Builder::new().build().expect("Couldn't build default Counter");
}
