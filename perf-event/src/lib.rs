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
//! If you're familiar with the kernel API already:
//!
//! -   A `Builder` holds the arguments to a `perf_event_open` call:
//!     a `struct perf_event_attr` and a few other fields.
//!
//! -   `Counter` and `Group` objects are just event file descriptors, together
//!     with their kernel id numbers, and some other details you need to
//!     actually use them. They're different types because they yield different
//!     types of results, and because you can't retrieve a `Group`'s counts
//!     without knowing how many members it has.
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
//! consider submitting a pull request.
//!
//! [man]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html

#![deny(missing_docs)]

/// A helper macro for silencing warnings when a type is only implemented so
/// that it can be linked in the docs.
macro_rules! used_in_docs {
    ($t:ident) => {
        const _: () = {
            // Using a module here means that this macro can accept any identifier that
            // would normally be used in an import statement.
            mod use_item {
                #[allow(unused_imports)]
                use super::$t;
            }
        };
    };
}

use perf_event_open_sys::bindings::perf_event_attr;
use std::fs::File;
use std::io::{self, Read};
use std::os::raw::{c_int, c_uint};
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

pub mod events;

#[cfg(feature = "hooks")]
pub mod hooks;

mod builder;
mod counter;
mod flags;

// When the `"hooks"` feature is not enabled, call directly into
// `perf-event-open-sys`.
#[cfg(not(feature = "hooks"))]
use perf_event_open_sys as sys;

// When the `"hooks"` feature is enabled, `sys` functions allow for
// interposed functions that provide simulated results for testing.
#[cfg(feature = "hooks")]
use hooks::sys;

pub use crate::builder::Builder;
pub use crate::counter::Counter;
pub use crate::flags::{Clock, ReadFormat, SampleBranchFlag, SampleSkid};

/// A group of counters that can be managed as a unit.
///
/// A `Group` represents a group of [`Counter`]s that can be enabled,
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
/// no way at present to specify a `Group`'s task and cpu, so you can only use
/// `Group` on the calling task. If this is a problem, please file an issue.
///
/// Internally, a `Group` is just a wrapper around an event file descriptor.
///
/// ## Limits on group size
///
/// Hardware counters are implemented using special-purpose registers on the
/// processor, of which there are only a fixed number. (For example, an Intel
/// high-end laptop processor from 2015 has four such registers per virtual
/// processor.) Without using groups, if you request more hardware counters than
/// the processor can actually support, a complete count isn't possible, but the
/// kernel will rotate the processor's real registers amongst the measurements
/// you've requested to at least produce a sample.
///
/// But since the point of a counter group is that its members all cover exactly
/// the same period of time, this tactic can't be applied to support large
/// groups. If the kernel cannot schedule a group, its counters remain zero. I
/// think you can detect this situation by comparing the group's [`time_enabled`]
/// and [`time_running`] values. It might also be useful to set the `pinned` bit,
/// which puts the counter in an error state if it's not able to be put on the
/// CPU; see [#10].
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
/// [`group`]: Builder::group
/// [`read`]: Group::read
/// [`#5`]: https://github.com/jimblandy/perf-event/issues/5
/// [`#10`]: https://github.com/jimblandy/perf-event/issues/10
/// [`time_enabled`]: Counts::time_enabled
/// [`time_running`]: Counts::time_running
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
    max_members: usize,
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
/// For some kinds of events, the kernel may use timesharing to give all
/// counters access to scarce hardware registers. You can see how long a group
/// was actually running versus the entire time it was enabled using the
/// `time_enabled` and `time_running` methods:
///
///     # fn main() -> std::io::Result<()> {
///     # use perf_event::{Builder, Group};
///     # let mut group = Group::new()?;
///     # let insns = Builder::new().group(&mut group).build()?;
///     # let counts = group.read()?;
///     let scale = counts.time_enabled() as f64 /
///                 counts.time_running() as f64;
///     for (id, value) in &counts {
///         print!("Counter id {} has value {}",
///                id, (*value as f64 * scale) as u64);
///         if scale > 1.0 {
///             print!(" (estimated)");
///         }
///         println!();
///     }
///
///     # Ok(()) }
///
/// [`read`]: Group::read
pub struct Counts {
    // Raw results from the `read`.
    data: Vec<u64>,
}

/// The value of a counter, along with timesharing data.
///
/// Some counters are implemented in hardware, and the processor can run
/// only a fixed number of them at a time. If more counters are requested
/// than the hardware can support, the kernel timeshares them on the
/// hardware.
///
/// This struct holds the value of a counter, together with the time it was
/// enabled, and the proportion of that for which it was actually running.
#[repr(C)]
pub struct CountAndTime {
    /// The counter value.
    ///
    /// The meaning of this field depends on how the counter was configured when
    /// it was built; see ['Builder'].
    pub count: u64,

    /// How long this counter was enabled by the program, in nanoseconds.
    pub time_enabled: u64,

    /// How long the kernel actually ran this counter, in nanoseconds.
    ///
    /// If `time_enabled == time_running`, then the counter ran for the entire
    /// period it was enabled, without interruption. Otherwise, the counter
    /// shared the underlying hardware with others, and you should prorate its
    /// value accordingly.
    pub time_running: u64,
}

impl Group {
    /// Construct a new, empty `Group`.
    #[allow(unused_parens)]
    pub fn new() -> io::Result<Group> {
        // Open a placeholder perf counter that we can add other events to.
        let mut attrs = perf_event_attr {
            size: std::mem::size_of::<perf_event_attr>() as u32,
            type_: sys::bindings::PERF_TYPE_SOFTWARE,
            config: sys::bindings::PERF_COUNT_SW_DUMMY as u64,
            ..perf_event_attr::default()
        };

        attrs.set_disabled(1);
        attrs.set_exclude_kernel(1);
        attrs.set_exclude_hv(1);

        // Arrange to be able to identify the counters we read back.
        attrs.read_format = (sys::bindings::PERF_FORMAT_TOTAL_TIME_ENABLED
            | sys::bindings::PERF_FORMAT_TOTAL_TIME_RUNNING
            | sys::bindings::PERF_FORMAT_ID
            | sys::bindings::PERF_FORMAT_GROUP) as u64;

        let file = unsafe {
            File::from_raw_fd(check_errno_syscall(|| {
                sys::perf_event_open(&mut attrs, 0, -1, -1, 0)
            })?)
        };

        // Retrieve the ID the kernel assigned us.
        let mut id = 0_u64;
        check_errno_syscall(|| unsafe { sys::ioctls::ID(file.as_raw_fd(), &mut id) })?;

        Ok(Group {
            file,
            id,
            max_members: 1,
        })
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
            f(self.file.as_raw_fd(), sys::bindings::PERF_IOC_FLAG_GROUP)
        })
        .map(|_| ())
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
        // Since we passed `PERF_FORMAT_{ID,GROUP,TOTAL_TIME_{ENABLED,RUNNING}}`,
        // the data we'll read has the form:
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
        let mut data = vec![0_u64; 3 + 2 * self.max_members];
        assert_eq!(
            self.file.read(u64::slice_as_bytes_mut(&mut data))?,
            std::mem::size_of_val(&data[..])
        );

        let counts = Counts { data };

        // CountsIter assumes that the group's dummy count appears first.
        assert_eq!(counts.nth_ref(0).0, self.id);

        // Does the kernel ever return nonsense?
        assert!(counts.time_running() <= counts.time_enabled());

        // Update `max_members` for the next read.
        self.max_members = counts.len();

        Ok(counts)
    }
}

impl std::fmt::Debug for Group {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "Group {{ fd: {}, id: {} }}",
            self.file.as_raw_fd(),
            self.id
        )
    }
}

impl AsRawFd for Group {
    fn as_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

impl IntoRawFd for Group {
    fn into_raw_fd(self) -> RawFd {
        self.file.into_raw_fd()
    }
}

impl Counts {
    /// Return the number of counters this `Counts` holds results for.
    #[allow(clippy::len_without_is_empty)] // Groups are never empty.
    pub fn len(&self) -> usize {
        self.data[0] as usize
    }

    /// Return the number of nanoseconds the `Group` was enabled that
    /// contributed to this `Counts`' contents.
    pub fn time_enabled(&self) -> u64 {
        self.data[1]
    }

    /// Return the number of nanoseconds the `Group` was actually collecting
    /// counts that contributed to this `Counts`' contents.
    pub fn time_running(&self) -> u64 {
        self.data[2]
    }

    /// Return a range of indexes covering the count and id of the `n`'th counter.
    fn nth_index(n: usize) -> std::ops::Range<usize> {
        let base = 3 + 2 * n;
        base..base + 2
    }

    /// Return the id and count of the `n`'th counter. This returns a reference
    /// to the count, for use by the `Index` implementation.
    fn nth_ref(&self, n: usize) -> (u64, &u64) {
        let id_val = &self.data[Counts::nth_index(n)];

        // (id, &value)
        (id_val[1], &id_val[0])
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
    next: usize,
}

impl<'c> Iterator for CountsIter<'c> {
    type Item = (u64, &'c u64);
    fn next(&mut self) -> Option<(u64, &'c u64)> {
        if self.next >= self.counts.len() {
            return None;
        }
        let result = self.counts.nth_ref(self.next);
        self.next += 1;
        Some(result)
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
            .find(|&(id, _)| id == member.id())
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

/// A type whose values can be safely accessed as a slice of bytes.
///
/// # Safety
///
/// `Self` must be a type such that storing a value in memory
/// initializes all the bytes of that memory, so that
/// `slice_as_bytes_mut` can never expose uninitialized bytes to the
/// caller.
unsafe trait SliceAsBytesMut: Sized {
    fn slice_as_bytes_mut(slice: &mut [Self]) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(
                slice.as_mut_ptr() as *mut u8,
                std::mem::size_of_val(slice),
            )
        }
    }
}

unsafe impl SliceAsBytesMut for u64 {}

/// Produce an `io::Result` from an errno-style system call.
///
/// An 'errno-style' system call is one that reports failure by returning -1 and
/// setting the C `errno` value when an error occurs.
fn check_errno_syscall<F, R>(f: F) -> io::Result<R>
where
    F: FnOnce() -> R,
    R: PartialOrd + Default,
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
    Builder::new()
        .build()
        .expect("Couldn't build default Counter");
}

#[test]
#[cfg(target_os = "linux")]
fn test_error_code_is_correct() {
    // This configuration should always result in EINVAL
    let builder = Builder::new()
        // CPU_CLOCK is literally always supported so we don't have to worry
        // about test failures when in VMs.
        .kind(events::Software::CPU_CLOCK)
        // There should _hopefully_ never be a system with this many CPUs.
        .one_cpu(i32::MAX as usize);

    match builder.build() {
        Ok(_) => panic!("counter construction was not supposed to succeed"),
        Err(e) => assert_eq!(e.raw_os_error(), Some(libc::EINVAL)),
    }
}
