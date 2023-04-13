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
//! use perf_event::{Builder, Group};
//! use perf_event::events::Hardware;
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
//! println!("cycles / instructions: {} / {} ({:.2} cpi)",
//!          counts[&cycles],
//!          counts[&insns],
//!          (counts[&cycles] as f64 / counts[&insns] as f64));
//!
//! Ok(())
//! # }
//!```
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

use std::io;

#[macro_use]
mod counter;

mod builder;
pub mod events;
mod flags;
mod group;
mod sampler;

#[cfg(feature = "hooks")]
pub mod hooks;

pub use crate::builder::Builder;
pub use crate::counter::Counter;
pub use crate::flags::SampleFlag;
pub use crate::group::{Counts, Group};
pub use crate::sampler::{Record, Sampler};

// When the `"hooks"` feature is not enabled, call directly into
// `perf-event-open-sys`.
#[cfg(not(feature = "hooks"))]
use perf_event_open_sys as sys;

// When the `"hooks"` feature is enabled, `sys` functions allow for
// interposed functions that provide simulated results for testing.
#[cfg(feature = "hooks")]
use hooks::sys;

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

/// View a slice of u64s as a byte slice.
fn as_byte_slice_mut(slice: &mut [u64]) -> &mut [u8] {
    unsafe {
        let (head, slice, tail) = slice.align_to_mut();
        assert!(head.is_empty());
        assert!(tail.is_empty());

        slice
    }
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
