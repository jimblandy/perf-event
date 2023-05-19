//! A performance monitoring API for Linux.
//!
//! This crate provides access to processor and kernel counters for things like
//! instruction completions, cache references and misses, branch predictions,
//! context switches, page faults, and so on.
//!
//! For example, to compare the number of clock cycles elapsed with the number
//! of instructions completed during one call to `println!`:
//!
//! ```
//! use perf_event::events::Hardware;
//! use perf_event::{Builder, Group};
//!
//! # fn main() -> std::io::Result<()> {
//! // A `Group` lets us enable and disable several counters atomically.
//! let mut group = Group::new()?;
//! let cycles = group.add(&Builder::new(Hardware::CPU_CYCLES))?;
//! let insns = group.add(&Builder::new(Hardware::INSTRUCTIONS))?;
//!
//! let vec = (0..=51).collect::<Vec<_>>();
//!
//! group.enable()?;
//! println!("{:?}", vec);
//! group.disable()?;
//!
//! let counts = group.read()?;
//! println!(
//!     "cycles / instructions: {} / {} ({:.2} cpi)",
//!     counts[&cycles],
//!     counts[&insns],
//!     (counts[&cycles] as f64 / counts[&insns] as f64)
//! );
//!
//! Ok(())
//! # }
//! ```
//!
//! This crate is built on top of the Linux [`perf_event_open`][man] system
//! call; that documentation has the authoritative explanations of exactly what
//! all the counters mean.
//!
//! There are two main types for measurement:
//!
//! - A [`Counter`] is an individual counter. Use [`Builder`] to construct one.
//!
//! - A [`Group`] is a collection of counters that can be enabled and disabled
//!   atomically, so that they cover exactly the same period of execution,
//!   allowing meaningful comparisons of the individual values. You can
//!   construct one via [`Group::new`] or use [`Builder`] to construct one with
//!   custom settings.
//!
//! If you're familiar with the kernel API already:
//!
//! - A `Builder` holds the arguments to a `perf_event_open` call: a `struct
//!   perf_event_attr` and a few other fields.
//!
//! - `Counter` and `Group` objects are just event file descriptors, together
//!   with their kernel id numbers, and some other details you need to actually
//!   use them. They're different types because they yield different types of
//!   results, and because you can't retrieve a `Group`'s counts without knowing
//!   how many members it has.
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

#![cfg_attr(debug_assertions, warn(missing_docs))]
#![cfg_attr(not(debug_assertions), deny(missing_docs))]
// The bitflags macro is generating this lint internally.
#![allow(clippy::assign_op_pattern)]

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

pub mod events;

mod builder;
mod flags;
mod group;
mod group_data;
mod sampler;

// Make sure the examples in the readme are tested.
#[doc = include_str!("../README.md")]
mod readme {}

#[cfg(feature = "hooks")]
pub mod hooks;

// When the `"hooks"` feature is not enabled, call directly into
// `perf-event-open-sys`.
#[cfg(not(feature = "hooks"))]
use perf_event_open_sys as sys;

// When the `"hooks"` feature is enabled, `sys` functions allow for
// interposed functions that provide simulated results for testing.
#[cfg(feature = "hooks")]
use hooks::sys;

pub use crate::builder::{Builder, UnsupportedOptionsError};
#[doc(inline)]
pub use crate::data::{ReadFormat, SampleFlags as SampleFlag};
pub use crate::flags::{Clock, SampleSkid};
pub use crate::group::Group;
pub use crate::group_data::{GroupData, GroupEntry, GroupIter};
pub use crate::sampler::{Record, Sampler};

/// Support for parsing data contained within `Record`s.
///
/// Note that this module is actually just the [`perf-event-data`][ped] crate.
/// The documentation has been inlined here for convenience.
// TODO: Directly linking to the crate causes an ICE in rustdoc. It is fixed in
//       nightly but not in the latest stable.
///
/// [ped]: http://docs.rs/perf-event-data
///
/// # perf-event-data
#[doc(inline)]
pub use perf_event_data as data;

// ... separate public exports and non-public ones

use std::convert::TryInto;
use std::fmt;
use std::fs::File;
use std::io;
use std::os::fd::{AsRawFd, IntoRawFd, RawFd};
use std::time::Duration;

use crate::data::endian::Native;
use crate::data::parse::ParseConfig;
use crate::sys::bindings::PERF_IOC_FLAG_GROUP;
use crate::sys::ioctls;

/// A counter for a single kernel or hardware event.
///
/// A counter represents a single performance monitoring counter. When building
/// the counter you select the event you would like it to count. Once the
/// counter is created, then you can enable or disable it, call its [`read`]
/// method to retrieve its current value, and reset it to zero.
///
/// # Groups
/// The kernel allows for counters to be grouped together. A group of counters
/// will be scheduled onto the CPU as a unit. This allows you to directly
/// compare the values collected by multiple counters.
///
/// There are two ways to go about working with groups:
/// - Use the [`Group`] type. It is not configurable but it makes groups easy to
///   setup and use.
/// - Pick one `Counter` to be a group leader, create the other counters with
///   [`Builder::build_with_group`] and use [`enable_group`], [`disable_group`],
///   and [`reset_group`] on any of its members to control the group. To read
///   all counters in the group at once you'll need to create at least one
///   counter with [`ReadFormat::GROUP`] so that [`read_group`] will read the
///   entire group.
///
/// A counter represents a single performance monitoring counter. While
/// creating the counter - via [`Builder`] - you select the event you would
/// like to count. Once the counter is created, then you can enable or disable
/// it, call its [`read`] method to retrieve the current count (or counts if
/// it is a [`Group`]), and reset it to zero.
///
/// [`read`]: crate::Counter::read
/// [`read_group`]: Self::read_group
/// [`reset_group`]: Self::reset_group
/// [`enable_group`]: Self::enable_group
/// [`disable_group`]: Self::disable_group
pub struct Counter {
    /// The file descriptor for this counter, returned by `perf_event_open`.
    ///
    /// When a `Counter` is dropped, this `File` is dropped, and the kernel
    /// removes the counter from any group it belongs to.
    file: File,

    /// The unique id assigned to this counter by the kernel.
    id: u64,

    /// The parse config used by this counter.
    config: ParseConfig<Native>,

    /// If we are a `Group`, then this is the count of how many members we have.
    member_count: u32,
}

impl Counter {
    /// Common initialization code shared between counters and groups.
    pub(crate) fn new_internal(file: File, config: ParseConfig<Native>) -> std::io::Result<Self> {
        let mut counter = Self {
            file,
            id: 0,
            config,
            member_count: 1,
        };

        // If we are part of a group then the id is used to find results in the
        // Counts structure. Otherwise, it's just used for debug output.
        let mut id = 0;
        counter.ioctl(|fd| unsafe { ioctls::ID(fd, &mut id) })?;
        counter.id = id;

        Ok(counter)
    }

    /// Return this counter's kernel-assigned unique id.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// The [`ParseConfig`] for this `Counter`.
    pub fn config(&self) -> &ParseConfig<Native> {
        &self.config
    }

    /// Allow this `Counter` to begin counting its designated event.
    ///
    /// This does not affect whatever value the `Counter` had previously; new
    /// events add to the current count. To clear a `Counter`, use [`reset`].
    ///
    /// Note that, depending on how it was configured, a counter may start off
    /// enabled or be automatically enabled by the kernel when an event occurs.
    /// For example, setting [`enable_on_exec`] will cause this counter to be
    /// automatically enabled when the current process calls `execve(2)`.
    ///
    /// If you want to enable all counters in the same group as this one then
    /// use [`enable_group`] instead.
    ///
    /// # Examples
    /// Enable an individual counter:
    /// ```
    /// use perf_event::events::Hardware;
    /// use perf_event::Builder;
    ///
    /// let mut counter = Builder::new(Hardware::INSTRUCTIONS).build()?;
    /// counter.enable()?;
    /// // ...
    /// assert_ne!(counter.read()?, 0);
    /// # std::io::Result::Ok(())
    /// ```
    ///
    /// [`Group`]: crate::Group
    /// [`reset`]: Self::reset
    /// [`enable_group`]: Self::enable_group
    /// [`enable_on_exec`]: crate::Builder::enable_on_exec
    pub fn enable(&mut self) -> io::Result<()> {
        self.ioctl(|fd| unsafe { ioctls::ENABLE(fd, 0) })
    }

    /// Enable all counters in the same group as this one.
    ///
    /// This does not affect whatever value the `Counter` had previously; new
    /// events add to the current count. To clear a counter group, use
    /// [`reset_group`].
    ///
    /// See [`enable`] for the version that only applies to the current
    /// counter.
    ///
    /// # Examples
    /// Enable all counters in a group:
    /// ```
    /// use perf_event::events::Hardware;
    /// use perf_event::{Builder, Group};
    ///
    /// let mut group = Group::new()?;
    /// let mut cycles = Builder::new(Hardware::CPU_CYCLES).build_with_group(&mut group)?;
    /// group.enable()?;
    /// // ...
    /// assert_ne!(cycles.read()?, 0);
    /// # std::io::Result::Ok(())
    /// ```
    ///
    /// [`enable`]: Self::enable
    /// [`reset_group`]: Self::reset_group
    pub fn enable_group(&mut self) -> io::Result<()> {
        self.ioctl(|fd| unsafe { ioctls::ENABLE(fd, PERF_IOC_FLAG_GROUP) })
    }

    /// Make this `Counter` stop counting its designated event.
    ///
    /// This does not affect the value of this `Counter`.
    ///
    /// To disable all counters in the group use
    /// [`disable_group`](Self::disable_group).
    ///
    /// # Examples
    /// Disable a single counter:
    /// ```
    /// use perf_event::events::Hardware;
    /// use perf_event::Builder;
    ///
    /// let mut counter = Builder::new(Hardware::INSTRUCTIONS).build()?;
    /// counter.enable()?;
    ///
    /// // Counter is continuously updating
    /// let val1 = counter.read()?;
    /// let val2 = counter.read()?;
    /// counter.disable()?;
    ///
    /// // Counter is no longer updating
    /// let val3 = counter.read()?;
    /// let val4 = counter.read()?;
    ///
    /// assert_ne!(val1, val2);
    /// assert_eq!(val3, val4);
    /// # std::io::Result::Ok(())
    /// ```
    ///
    /// [`Group`]: crate::Group
    /// [`disable`]: struct.Group.html#method.disable
    pub fn disable(&mut self) -> io::Result<()> {
        self.ioctl(|fd| unsafe { ioctls::DISABLE(fd, 0) })
    }

    /// Disable all counters in the same group as this one.
    ///
    /// This does not affect the counter values.
    ///
    /// To disable only this counter use [`disable`].
    ///
    /// [`disable`]: Self::disable
    pub fn disable_group(&mut self) -> io::Result<()> {
        self.ioctl(|fd| unsafe { ioctls::DISABLE(fd, PERF_IOC_FLAG_GROUP) })
    }

    /// Reset the value of this `Counter` to zero.
    ///
    /// To reset the value of all counters in the current group use
    /// [`reset_group`](Self::reset_group).
    ///
    /// # Examples
    /// Reset a single counter
    /// ```
    /// use perf_event::events::Hardware;
    /// use perf_event::Builder;
    ///
    /// let mut counter = Builder::new(Hardware::INSTRUCTIONS).build()?;
    /// counter.enable()?;
    /// // ...
    /// counter.disable()?;
    ///
    /// assert_ne!(counter.read()?, 0);
    /// counter.reset()?;
    /// assert_eq!(counter.read()?, 0);
    /// # std::io::Result::Ok(())
    /// ```
    pub fn reset(&mut self) -> io::Result<()> {
        self.ioctl(|fd| unsafe { ioctls::RESET(fd, 0) })
    }

    /// Reset the value of all counters in the same group as this one to zero.
    ///
    /// To only reset the value of this counter use [`reset`](Self::reset).
    pub fn reset_group(&mut self) -> io::Result<()> {
        self.ioctl(|fd| unsafe { ioctls::RESET(fd, PERF_IOC_FLAG_GROUP) })
    }

    /// Attach an eBPF program to this counter.
    ///
    /// This will only work if this counter was created as a kprobe
    /// tracepoint event.
    ///
    /// This method corresponds to the `IOC_SET_BPF` ioctl.
    pub fn set_bpf(&mut self, bpf: RawFd) -> io::Result<()> {
        self.ioctl(|fd| unsafe { ioctls::SET_BPF(fd, bpf as _) })
            .map(drop)
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
    /// use perf_event::events::Software;
    /// use perf_event::Builder;
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

    /// Helper function for doing ioctls on a counter.
    pub(crate) fn ioctl<F>(&self, ioctl: F) -> io::Result<()>
    where
        F: FnOnce(RawFd) -> libc::c_int,
    {
        check_errno_syscall(|| ioctl(self.as_raw_fd())).map(drop)
    }
}

impl Counter {
    /// Return this `Counter`'s current value as a `u64`.
    ///
    /// Consider using [`read_full`] or (if read_format has the required flags)
    /// [`read_count_and_time`] instead. There are limitations around how
    /// many hardware counters can be on a single CPU at a time. If more
    /// counters are requested than the hardware can support then the kernel
    /// will timeshare them on the hardware. Looking at just the counter value
    /// gives you no indication that this has happened.
    ///
    /// If you would like to read the values for an entire group then you will
    /// need to use [`read_group`] (and set [`ReadFormat::GROUP`]) instead.
    ///
    /// [`read_full`]: Self::read_full
    /// [`read_group`]: Self::read_group
    /// [`read_count_and_time`]: Self::read_count_and_time
    /// [`ReadFormat::GROUP`]: ReadFormat::GROUP
    ///
    /// # Errors
    /// This function may return errors in the following notable cases:
    /// - `ENOSPC` is returned if the `read_format` that this `Counter` was
    ///   built with does not match the format of the data. This can also occur
    ///   if `read_format` contained options not supported by this crate.
    /// - If the counter is part of a group and was unable to be pinned to the
    ///   CPU then reading will return an error with kind [`UnexpectedEof`].
    ///
    /// Other errors are also possible under unexpected conditions (e.g. `EBADF`
    /// if the file descriptor is closed).
    ///
    /// [`UnexpectedEof`]: io::ErrorKind::UnexpectedEof
    ///
    /// # Example
    /// ```
    /// use perf_event::events::Hardware;
    /// use perf_event::Builder;
    ///
    /// let mut counter = Builder::new(Hardware::INSTRUCTIONS).enabled(true).build()?;
    ///
    /// let instrs = counter.read()?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn read(&mut self) -> io::Result<u64> {
        Ok(self.read_full()?.count())
    }

    /// Return all data that this `Counter` is configured to provide.
    ///
    /// The exact fields that are returned within the [`CounterData`] struct
    /// depend on what was specified for `read_format` when constructing this
    /// counter. This method is the only one that gives access to all values
    /// returned by the kernel.
    ///
    /// If this `Counter` was created with [`ReadFormat::GROUP`] then this will
    /// read the entire group but only return the data for this specific
    /// counter.
    ///
    /// # Errors
    /// This function may return errors in the following notable cases:
    /// - `ENOSPC` is returned if the `read_format` that this `Counter` was
    ///   built with does not match the format of the data. This can also occur
    ///   if `read_format` contained options not supported by this crate.
    /// - If the counter is part of a group and was unable to be pinned to the
    ///   CPU then reading will return an error with kind [`UnexpectedEof`].
    ///
    /// Other errors are also possible under unexpected conditions (e.g. `EBADF`
    /// if the file descriptor is closed).
    ///
    /// [`UnexpectedEof`]: io::ErrorKind::UnexpectedEof
    ///
    /// # Example
    /// ```
    /// use perf_event::events::Hardware;
    /// use perf_event::{Builder, ReadFormat};
    /// use std::time::Duration;
    ///
    /// let mut counter = Builder::new(Hardware::INSTRUCTIONS)
    ///     .read_format(ReadFormat::TOTAL_TIME_RUNNING)
    ///     .enabled(true)
    ///     .build()?;
    /// // ...
    /// let data = counter.read_full()?;
    /// let instructions = data.count();
    /// let time_running = data.time_running().unwrap();
    /// let ips = instructions as f64 / time_running.as_secs_f64();
    ///
    /// println!("instructions/s: {ips}");
    /// # std::io::Result::Ok(())
    /// ```
    pub fn read_full(&mut self) -> io::Result<CounterData> {
        if !self.is_group() {
            return self.do_read_single();
        }

        let group = self.do_read_group()?;
        let entry = group.get(self).unwrap();
        let data = crate::data::ReadValue::from_group_and_entry(&group.data, &entry.0);

        Ok(CounterData(data))
    }

    /// Read the values of all the counters in the current group.
    ///
    /// Note that unless [`ReadFormat::GROUP`] was specified when building this
    /// `Counter` this will only read the data for the current `Counter`.
    ///
    /// # Errors
    /// This function may return errors in the following notable cases:
    /// - `ENOSPC` is returned if the `read_format` that this `Counter` was
    ///   built with does not match the format of the data. This can also occur
    ///   if `read_format` contained options not supported by this crate.
    /// - If the counter is part of a group and was unable to be pinned to the
    ///   CPU then reading will return an error with kind [`UnexpectedEof`].
    ///
    /// Other errors are also possible under unexpected conditions (e.g. `EBADF`
    /// if the file descriptor is closed).
    ///
    /// [`UnexpectedEof`]: io::ErrorKind::UnexpectedEof
    ///
    /// # Example
    /// Compute the CPI for a region of code:
    /// ```
    /// use perf_event::events::Hardware;
    /// use perf_event::{Builder, ReadFormat};
    ///
    /// let mut instrs = Builder::new(Hardware::INSTRUCTIONS)
    ///     .read_format(ReadFormat::GROUP)
    ///     .build()?;
    /// let mut cycles = Builder::new(Hardware::CPU_CYCLES).build_with_group(&mut instrs)?;
    ///
    /// instrs.enable_group()?;
    /// // ...
    /// instrs.disable_group()?;
    ///
    /// let data = instrs.read_group()?;
    /// let instrs = data[&instrs];
    /// let cycles = data[&cycles];
    ///
    /// println!("CPI: {}", cycles as f64 / instrs as f64);
    /// # std::io::Result::Ok(())
    /// ```
    pub fn read_group(&mut self) -> io::Result<GroupData> {
        if self.is_group() {
            self.do_read_group()
        } else {
            Ok(GroupData::new(self.do_read_single()?.0.into()))
        }
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
    ///     println!(
    ///         "{} instructions (estimated)",
    ///         (cat.count as u128 * cat.time_enabled as u128 / cat.time_running as u128) as u64
    ///     );
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
            time_enabled: data
                .time_enabled()
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        "time_enabled was not enabled within read_format",
                    )
                })?
                .as_nanos() as _,
            time_running: data
                .time_running()
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        "time_running was not enabled within read_format",
                    )
                })?
                .as_nanos() as _,
        })
    }

    fn is_group(&self) -> bool {
        self.config.read_format().contains(ReadFormat::GROUP)
    }

    /// Actual read implementation for when `ReadFormat::GROUP` is not set.
    fn do_read_single(&mut self) -> io::Result<CounterData> {
        use crate::flags::ReadFormatExt;
        use std::io::Read;
        use std::mem::size_of;

        debug_assert!(!self.is_group());

        let mut data = [0u8; ReadFormat::MAX_NON_GROUP_SIZE * size_of::<u64>()];
        let len = self.file.read(&mut data)?;

        if len == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "the kernel was unable to schedule the counter or group",
            ));
        }

        let mut parser = crate::data::parse::Parser::new(&data[..len], self.config.clone());
        let value: crate::data::ReadValue = parser
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(CounterData(value))
    }

    /// Actual read implementation for when `ReadFormat::GROUP` is set.
    fn do_read_group(&mut self) -> io::Result<GroupData> {
        use crate::data::ReadGroup;
        use crate::flags::ReadFormatExt;
        use std::io::Read;
        use std::mem::size_of;

        // The general structure format looks like this, depending on what
        // read_format flags were enabled.
        //
        // struct read_format {
        //     u64 nr;            /* The number of events */
        //     u64 time_enabled;  /* if PERF_FORMAT_TOTAL_TIME_ENABLED */
        //     u64 time_running;  /* if PERF_FORMAT_TOTAL_TIME_RUNNING */
        //     struct {
        //         u64 value;     /* The value of the event */
        //         u64 id;        /* if PERF_FORMAT_ID */
        //         u64 lost;      /* if PERF_FORMAT_LOST */
        //     } values[nr];
        // };
        let read_format = self.config.read_format();
        let prefix_len = read_format.prefix_len();
        let element_len = read_format.element_len();

        let mut elements = (self.member_count as usize).max(1);
        let mut data = vec![0u8; (prefix_len + elements * element_len) * size_of::<u64>()];

        // Backoff loop to try and get the correct size.
        //
        // There's no way to know when new counters are added to the current
        // group, so to make sure reads succeed we expand the buffer whenever
        // we get ENOSPC until the read completes.
        //
        // The next time around self.member_count will be set to the correct
        // count and we won't need to go through this loop multiple times.
        let len = loop {
            match self.file.read(&mut data) {
                Ok(len) => break len,
                Err(e) if e.raw_os_error() == Some(libc::ENOSPC) => {
                    elements *= 2;
                    data.resize((prefix_len + elements * element_len) * size_of::<u64>(), 0);
                }
                Err(e) => return Err(e),
            }
        };

        if len == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "the kernel was unable to schedule the counter or group",
            ));
        }

        data.truncate(len);
        let mut parser = crate::data::parse::Parser::new(data.as_slice(), self.config.clone());
        let data: ReadGroup = parser
            .parse::<ReadGroup>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            .into_owned();

        let data = GroupData::new(data);

        self.member_count = data
            .len()
            .try_into()
            .expect("group had more than u32::MAX elements");

        Ok(data)
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

impl AsRef<Counter> for &'_ Counter {
    fn as_ref(&self) -> &Counter {
        self
    }
}

impl AsMut<Counter> for &'_ mut Counter {
    fn as_mut(&mut self) -> &mut Counter {
        self
    }
}

impl fmt::Debug for Counter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Counter")
            .field("fd", &self.as_raw_fd())
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}

/// The data retrieved by reading from a [`Counter`].
#[derive(Clone, Debug)]
pub struct CounterData(crate::data::ReadValue);

impl CounterData {
    /// The counter value.
    ///
    /// The meaning of this field depends on how the counter was configured when
    /// it was built; see ['Builder'].
    pub fn count(&self) -> u64 {
        self.0.value()
    }

    /// How long this counter was enabled by the program.
    ///
    /// This will be present if [`ReadFormat::TOTAL_TIME_ENABLED`] was
    /// specified in `read_format` when the counter was built.
    pub fn time_enabled(&self) -> Option<Duration> {
        self.0.time_enabled().map(Duration::from_nanos)
    }

    /// How long the kernel actually ran this counter.
    ///
    /// If `time_enabled == time_running` then the counter ran for the entire
    /// period it was enabled, without interruption. Otherwise, the counter
    /// shared the underlying hardware with others and you should adjust its
    /// value accordingly.
    ///
    /// This will be present if [`ReadFormat::TOTAL_TIME_RUNNING`] was
    /// specified in `read_format` when the counter was built.
    pub fn time_running(&self) -> Option<Duration> {
        self.0.time_running().map(Duration::from_nanos)
    }

    /// The number of lost samples of this event.
    ///
    /// This will be present if [`ReadFormat::LOST`] was specified in
    /// `read_format` when the counter was built.
    pub fn lost(&self) -> Option<u64> {
        self.0.lost()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_build() {
        Builder::new(crate::events::Software::DUMMY)
            .build()
            .expect("Couldn't build default Counter");
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_error_code_is_correct() {
        // This configuration should always result in EINVAL

        // CPU_CLOCK is literally always supported so we don't have to worry
        // about test failures when in VMs.
        let builder = Builder::new(events::Software::CPU_CLOCK)
            // There should _hopefully_ never be a system with this many CPUs.
            .one_cpu(i32::MAX as usize)
            .clone();

        match builder.build() {
            Ok(_) => panic!("counter construction was not supposed to succeed"),
            Err(e) => assert_eq!(e.raw_os_error(), Some(libc::EINVAL)),
        }
    }
}
