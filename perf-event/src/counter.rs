use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::os::unix::io::{AsRawFd, IntoRawFd, RawFd};

use crate::{check_errno_syscall, sys, ReadFormat, Sampler};

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
/// ```
/// use perf_event::Builder;
/// use perf_event::events::Hardware;
///
/// let mut counter = Builder::new(Hardware::INSTRUCTIONS).build()?;
///
/// let vec = (0..=51).collect::<Vec<_>>();
///
/// counter.enable()?;
/// println!("{:?}", vec);
/// counter.disable()?;
///
/// println!("{} instructions retired", counter.read()?);
/// # std::io::Result::Ok(())
/// ```
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
/// [`read`]: Self::read
/// [`Group`]: crate::Group
pub struct Counter {
    /// The file descriptor for this counter, returned by `perf_event_open`.
    ///
    /// When a `Counter` is dropped, this `File` is dropped, and the kernel
    /// removes the counter from any group it belongs to.
    file: File,

    /// The unique id assigned to this counter by the kernel.
    id: u64,

    /// The [`ReadFormat`] flags that were used to configure this `Counter`.
    read_format: ReadFormat,
}

impl Counter {
    pub(crate) fn new(file: File, read_format: ReadFormat) -> std::io::Result<Self> {
        // If we are part of a group then the id is used to find results in the
        // Counts structure. Otherwise, it's just used for debug output.
        let mut id = 0;
        check_errno_syscall(|| unsafe { sys::ioctls::ID(file.as_raw_fd(), &mut id) })?;

        Ok(Self {
            file,
            id,
            read_format,
        })
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
        check_errno_syscall(|| unsafe { sys::ioctls::ENABLE(self.as_raw_fd(), 0) }).map(|_| ())
    }

    /// Make this `Counter` stop counting its designated event. Its count is
    /// unaffected.
    ///
    /// Note that `Group` also has a [`disable`] method, which disables all
    /// its member `Counter`s as a single atomic operation.
    ///
    /// [`disable`]: struct.Group.html#method.disable
    pub fn disable(&mut self) -> io::Result<()> {
        check_errno_syscall(|| unsafe { sys::ioctls::DISABLE(self.as_raw_fd(), 0) }).map(|_| ())
    }

    /// Reset the value of this `Counter` to zero.
    ///
    /// Note that `Group` also has a [`reset`] method, which resets all
    /// its member `Counter`s as a single atomic operation.
    ///
    /// [`reset`]: struct.Group.html#method.reset
    pub fn reset(&mut self) -> io::Result<()> {
        check_errno_syscall(|| unsafe { sys::ioctls::RESET(self.as_raw_fd(), 0) }).map(|_| ())
    }

    /// Return this `Counter`'s current value as a `u64`.
    ///
    /// Consider using [`read_full`] or (if read_format has the required flags)
    /// [`read_count_and_time`] instead. There are limitations around how
    /// many hardware counters can be on a single CPU at a time. If more
    /// counters are requested than the hardware can support then the kernel
    /// will timeshare them on the hardware. Looking at just the counter value
    /// gives you no indication that this has happened.
    ///
    /// Note that `Group` also has a [`read`] method, which reads all
    /// its member `Counter`s' values at once.
    ///
    /// [`read`]: crate::Group::read
    /// [`read_full`]: Self::read_full
    /// [`read_count_and_time`]: Self::read_count_and_time
    pub fn read(&mut self) -> io::Result<u64> {
        let mut data = [0u64; ReadFormat::MAX_NON_GROUP_SIZE];
        let bytes = self.file.read(crate::as_byte_slice_mut(&mut data))?;

        debug_assert!(bytes >= std::mem::size_of::<u64>());

        Ok(data[0])
    }

    /// Return all data that this `Counter` is configured to provide.
    ///
    /// The exact fields that are returned within the [`CounterData`] struct
    /// depend on what was specified for `read_format` when constructing this
    /// counter. This method is the only one that gives access to all values
    /// returned by the kernel.
    ///
    /// # Errors
    /// See the [man page][man] for possible errors when reading from the
    /// counter.
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    /// use perf_event::{Builder, ReadFormat};
    /// use perf_event::events::Hardware;
    ///
    /// let mut counter = Builder::new(Hardware::INSTRUCTIONS)
    ///     .read_format(ReadFormat::TOTAL_TIME_RUNNING)
    ///     .enabled(true)
    ///     .build()?;
    /// // ...
    /// let data = counter.read_full()?;
    /// let instructions = data.count();
    /// let time_running = Duration::from_nanos(data.time_running().unwrap());
    /// let ips = instructions as f64 / time_running.as_secs_f64();
    ///
    /// println!("instructions/s: {ips}");
    /// # std::io::Result::Ok(())
    /// ```
    ///
    /// [man]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
    pub fn read_full(&mut self) -> io::Result<CounterData> {
        use std::mem::size_of;

        debug_assert!(!self.read_format.contains(ReadFormat::GROUP));

        let mut data = [0u64; ReadFormat::MAX_NON_GROUP_SIZE];
        let bytes = self.file.read(crate::as_byte_slice_mut(&mut data))?;

        // Should never happen but worth checking in debug mode.
        debug_assert_eq!(bytes % size_of::<u64>(), 0);

        let mut iter = data.iter().take(bytes / size_of::<u64>()).skip(1).copied();
        let mut read = |flag: ReadFormat| {
            self.read_format
                .contains(flag)
                .then(|| iter.next())
                .unwrap_or(Some(0))
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        "read_format did not match the data returned by the kernel",
                    )
                })
        };

        Ok(CounterData {
            read_format: self.read_format,
            value: data[0],
            time_enabled: read(ReadFormat::TOTAL_TIME_ENABLED)?,
            time_running: read(ReadFormat::TOTAL_TIME_RUNNING)?,
            lost: read(ReadFormat::LOST)?,
        })
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
    /// # Errors
    /// See the [man page][man] for possible errors when reading from the
    /// counter. This method will also return an error if `read_format` does
    /// not include both [`TOTAL_TIME_ENABLED`] and [`TOTAL_TIME_RUNNING`].
    ///
    /// # Example
    /// ```
    /// # use perf_event::Builder;
    /// # use perf_event::events::Software;
    /// #
    /// # let mut counter = Builder::new(Software::DUMMY).build()?;
    /// let cat = counter.read_count_and_time()?;
    /// if cat.time_running == 0 {
    ///     println!("No data collected.");
    /// } else if cat.time_running < cat.time_enabled {
    ///     // Note: this way of scaling is accurate, but `u128` division
    ///     // is usually implemented in software, which may be slow.
    ///     println!("{} instructions (estimated)",
    ///              (cat.count as u128 *
    ///               cat.time_enabled as u128 / cat.time_running as u128) as u64);
    /// } else {
    ///     println!("{} instructions", cat.count);
    /// }
    /// # std::io::Result::Ok(())
    /// ```
    ///
    /// Note that `Group` also has a [`read`] method, which reads all
    /// its member `Counter`s' values at once.
    ///
    /// [`read`]: crate::Group::read
    /// [`TOTAL_TIME_ENABLED`]: ReadFormat::TOTAL_TIME_ENABLED
    /// [`TOTAL_TIME_RUNNING`]: ReadFormat::TOTAL_TIME_RUNNING
    /// [man]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
    pub fn read_count_and_time(&mut self) -> io::Result<CountAndTime> {
        let data = self.read_full()?;

        Ok(CountAndTime {
            count: data.count(),
            time_enabled: data.time_enabled().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Other,
                    "time_enabled was not enabled within read_format",
                )
            })?,
            time_running: data.time_running().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Other,
                    "time_running was not enabled within read_format",
                )
            })?,
        })
    }

    /// Map a buffer for samples from this counter, returning a [`Sampler`]
    /// that can be used to access them.
    ///
    /// There are some restrictions on the size of the mapped buffer. To
    /// accomodate this `map_len` will always be rounded up to the next
    /// power-of-two multiple of the system page size. There will always
    /// be at least two pages allocated for the ring buffer: one for the
    /// control data structures, and one for actual data.
    ///
    /// # Example
    /// This example shows creating a sample to record mmap events within the
    /// current process. If you do this early enough, you can then track what
    /// libraries your process is loading.
    /// ```
    /// use perf_event::Builder;
    /// use perf_event::events::Software;
    ///
    /// let mut sampler = Builder::new(Software::DUMMY)
    ///     .mmap(true)
    ///     .build()?
    ///     .sampled(128)?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn sampled(self, map_len: usize) -> io::Result<Sampler> {
        let pagesize =
            check_errno_syscall(|| unsafe { libc::sysconf(libc::_SC_PAGESIZE) })? as usize;

        let len = pagesize
            + map_len
                .checked_next_power_of_two()
                .unwrap_or((usize::MAX >> 1) + 1)
                .max(pagesize);

        let mmap = memmap2::MmapOptions::new().len(len).map_raw(&self.file)?;

        Ok(Sampler::new(self, mmap))
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
#[derive(Copy, Clone, Debug)]
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

/// The data retrieved by reading from a [`Counter`].
#[derive(Clone)]
pub struct CounterData {
    // If you update this struct remember to update the Debug impl as well.
    //
    read_format: ReadFormat,
    value: u64,
    time_enabled: u64,
    time_running: u64,
    lost: u64,
}

impl CounterData {
    /// The counter value.
    ///
    /// The meaning of this field depends on how the counter was configured when
    /// it was built; see ['Builder'].
    pub fn count(&self) -> u64 {
        self.value
    }

    /// How long this counter was enabled by the program, in nanoseconds.
    ///
    /// This will be present if [`ReadFormat::TOTAL_TIME_ENABLED`] was
    /// specified in `read_format` when the counter was built.
    pub fn time_enabled(&self) -> Option<u64> {
        self.read_format
            .contains(ReadFormat::TOTAL_TIME_ENABLED)
            .then_some(self.time_enabled)
    }

    /// How long the kernel actually ran this counter, in nanoseconds.
    ///
    /// If `time_enabled == time_running` then the counter ran for the entire
    /// period it was enabled, without interruption. Otherwise, the counter
    /// shared the underlying hardware with others and you should adjust its
    /// value accordingly.
    ///
    /// This will be present if [`ReadFormat::TOTAL_TIME_RUNNING`] was
    /// specified in `read_format` when the counter was built.
    pub fn time_running(&self) -> Option<u64> {
        self.read_format
            .contains(ReadFormat::TOTAL_TIME_RUNNING)
            .then_some(self.time_running)
    }

    /// The number of lost samples of this event.
    ///
    /// This will be present if [`ReadFormat::LOST`] was specified in
    /// `read_format` when the counter was built.
    pub fn lost(&self) -> Option<u64> {
        self.read_format
            .contains(ReadFormat::LOST)
            .then_some(self.lost)
    }
}

impl fmt::Debug for CounterData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct("CounterData");

        dbg.field("count", &self.count());

        if let Some(time_enabled) = self.time_enabled() {
            dbg.field("time_enabled", &time_enabled);
        }

        if let Some(time_running) = self.time_running() {
            dbg.field("time_running", &time_running);
        }

        if let Some(lost) = self.lost() {
            dbg.field("lost", &lost);
        }

        dbg.finish_non_exhaustive()
    }
}
