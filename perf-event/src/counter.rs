use std::fs::File;
use std::io::{self, Read};
use std::os::fd::{AsRawFd, IntoRawFd, RawFd};

use crate::sys;
use crate::SliceAsBytesMut;
use crate::{check_errno_syscall, CountAndTime};

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
/// Internally, a `Counter` is just a wrapper around an event file descriptor.
///
/// [`read`]: Counter::read
pub struct Counter {
    /// The file descriptor for this counter, returned by `perf_event_open`.
    ///
    /// When a `Counter` is dropped, this `File` is dropped, and the kernel
    /// removes the counter from any group it belongs to.
    file: File,

    /// The unique id assigned to this counter by the kernel.
    id: u64,
}

impl Counter {
    pub(crate) fn new(file: File, id: u64) -> Self {
        Self { file, id }
    }

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
        check_errno_syscall(|| unsafe { sys::ioctls::ENABLE(self.file.as_raw_fd(), 0) }).map(|_| ())
    }

    /// Make this `Counter` stop counting its designated event. Its count is
    /// unaffected.
    ///
    /// Note that `Group` also has a [`disable`] method, which disables all
    /// its member `Counter`s as a single atomic operation.
    ///
    /// [`disable`]: struct.Group.html#method.disable
    pub fn disable(&mut self) -> io::Result<()> {
        check_errno_syscall(|| unsafe { sys::ioctls::DISABLE(self.file.as_raw_fd(), 0) })
            .map(|_| ())
    }

    /// Reset the value of this `Counter` to zero.
    ///
    /// Note that `Group` also has a [`reset`] method, which resets all
    /// its member `Counter`s as a single atomic operation.
    ///
    /// [`reset`]: struct.Group.html#method.reset
    pub fn reset(&mut self) -> io::Result<()> {
        check_errno_syscall(|| unsafe { sys::ioctls::RESET(self.file.as_raw_fd(), 0) }).map(|_| ())
    }

    /// Return this `Counter`'s current value as a `u64`.
    ///
    /// Consider using the [`read_count_and_time`] method instead of this one. Some
    /// counters are implemented in hardware, and the processor can support only
    /// a certain number running at a time. If more counters are requested than
    /// the hardware can support, the kernel timeshares them on the hardware.
    /// This method gives you no indication whether this has happened;
    /// `read_count_and_time` does.
    ///
    /// Note that `Group` also has a [`read`] method, which reads all
    /// its member `Counter`s' values at once.
    ///
    /// [`read`]: Group::read
    /// [`read_count_and_time`]: Counter::read_count_and_time
    pub fn read(&mut self) -> io::Result<u64> {
        Ok(self.read_count_and_time()?.count)
    }

    /// Return this `Counter`'s current value and timesharing data.
    ///
    /// Some counters are implemented in hardware, and the processor can run
    /// only a fixed number of them at a time. If more counters are requested
    /// than the hardware can support, the kernel timeshares them on the
    /// hardware.
    ///
    /// This method returns a [`CountAndTime`] struct, whose `count` field holds
    /// the counter's value, and whose `time_enabled` and `time_running` fields
    /// indicate how long you had enabled the counter, and how long the counter
    /// was actually scheduled on the processor. This lets you detect whether
    /// the counter was timeshared, and adjust your use accordingly. Times
    /// are reported in nanoseconds.
    ///
    ///     # use perf_event::Builder;
    ///     # fn main() -> std::io::Result<()> {
    ///     # let mut counter = Builder::new().build()?;
    ///     let cat = counter.read_count_and_time()?;
    ///     if cat.time_running == 0 {
    ///         println!("No data collected.");
    ///     } else if cat.time_running < cat.time_enabled {
    ///         // Note: this way of scaling is accurate, but `u128` division
    ///         // is usually implemented in software, which may be slow.
    ///         println!("{} instructions (estimated)",
    ///                  (cat.count as u128 *
    ///                   cat.time_enabled as u128 / cat.time_running as u128) as u64);
    ///     } else {
    ///         println!("{} instructions", cat.count);
    ///     }
    ///     # Ok(()) }
    ///
    /// Note that `Group` also has a [`read`] method, which reads all
    /// its member `Counter`s' values at once.
    ///
    /// [`read`]: Group::read
    pub fn read_count_and_time(&mut self) -> io::Result<CountAndTime> {
        let mut buf = [0_u64; 3];
        self.file.read_exact(u64::slice_as_bytes_mut(&mut buf))?;

        let cat = CountAndTime {
            count: buf[0],
            time_enabled: buf[1],
            time_running: buf[2],
        };

        // Does the kernel ever return nonsense?
        assert!(cat.time_running <= cat.time_enabled);

        Ok(cat)
    }
}

impl std::fmt::Debug for Counter {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "Counter {{ fd: {}, id: {} }}",
            self.file.as_raw_fd(),
            self.id
        )
    }
}

impl AsRawFd for Counter {
    fn as_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

impl IntoRawFd for Counter {
    fn into_raw_fd(self) -> RawFd {
        self.file.into_raw_fd()
    }
}
