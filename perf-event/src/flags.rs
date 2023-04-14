use bitflags::bitflags;

use crate::sys::bindings;

pub use self::bitflag_defs::*;

// Temporary, until all the bitflag fields are documented.
#[allow(missing_docs)]
mod bitflag_defs {
    use super::*;

    bitflags! {
        /// Specifies which fields to include in the sample.
        ///
        /// These values correspond to `PERF_SAMPLE_x` values. See the
        /// [manpage] for documentation on what they mean.
        ///
        /// [manpage]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
        pub struct SampleFlag : u64 {
            const IP = bindings::PERF_SAMPLE_IP as _;
            const TID = bindings::PERF_SAMPLE_TID as _;
            const TIME = bindings::PERF_SAMPLE_TIME as _;
            const ADDR = bindings::PERF_SAMPLE_ADDR as _;
            const READ = bindings::PERF_SAMPLE_READ as _;
            const CALLCHAIN = bindings::PERF_SAMPLE_CALLCHAIN as _;
            const ID = bindings::PERF_SAMPLE_ID as _;
            const CPU = bindings::PERF_SAMPLE_CPU as _;
            const PERIOD = bindings::PERF_SAMPLE_PERIOD as _;
            const STREAM_ID = bindings::PERF_SAMPLE_STREAM_ID as _;
            const RAW = bindings::PERF_SAMPLE_RAW as _;
            const BRANCH_STACK = bindings::PERF_SAMPLE_BRANCH_STACK as _;
            const REGS_USER = bindings::PERF_SAMPLE_REGS_USER as _;
            const STACK_USER = bindings::PERF_SAMPLE_STACK_USER as _;
            const WEIGHT = bindings::PERF_SAMPLE_WEIGHT as _;
            const DATA_SRC = bindings::PERF_SAMPLE_DATA_SRC as _;
            const IDENTIFIER = bindings::PERF_SAMPLE_IDENTIFIER as _;
            const TRANSACTION = bindings::PERF_SAMPLE_TRANSACTION as _;
            const REGS_INTR = bindings::PERF_SAMPLE_REGS_INTR as _;
            const PHYS_ADDR = bindings::PERF_SAMPLE_PHYS_ADDR as _;
            const AUX = bindings::PERF_SAMPLE_AUX as _;
            const CGROUP = bindings::PERF_SAMPLE_CGROUP as _;

            // The following are present in perf_event.h but not yet documented
            // in the manpage.
            const DATA_PAGE_SIZE = bindings::PERF_SAMPLE_DATA_PAGE_SIZE as _;
            const CODE_PAGE_SIZE = bindings::PERF_SAMPLE_CODE_PAGE_SIZE as _;
            const WEIGHT_STRUCT = bindings::PERF_SAMPLE_WEIGHT_STRUCT as _;
        }
    }

    bitflags! {
        /// Flags that control what data is returned when reading from a
        /// perf_event file descriptor.
        ///
        /// See the [man page][0] for the authoritative documentation on what
        /// these flags do.
        ///
        /// [0]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
        pub struct ReadFormat : u64 {
            /// Emit the total amount of time the counter has spent enabled.
            const TOTAL_TIME_ENABLED = bindings::PERF_FORMAT_TOTAL_TIME_ENABLED as _;

            /// Emit the total amount of time the counter was actually on the
            /// CPU.
            const TOTAL_TIME_RUNNING = bindings::PERF_FORMAT_TOTAL_TIME_RUNNING as _;

            /// Emit the counter ID.
            const ID = bindings::PERF_FORMAT_ID as _;

            /// If in a group, read all the counters in the group at once.
            const GROUP = bindings::PERF_FORMAT_GROUP as _;

            /// Emit the number of lost samples for this event.
            const LOST = bindings::PERF_FORMAT_LOST as _;
        }
    }

    impl ReadFormat {
        pub(crate) const MAX_NON_GROUP_SIZE: usize = Self::all() //
            .difference(Self::GROUP)
            .bits()
            .count_ones() as usize
            + 1;
    }
}

/// Configuration of how much skid is allowed when gathering samples.
///
/// Skid is the number of instructions that occur between an event occuring and
/// a sample being gathered by the kernel. Less skid is better but there are
/// hardware limitations around how small the skid can be.
///
/// Also see [`Builder::precise_ip`](crate::Builder::precise_ip).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SampleSkid {
    /// There may be an arbitrary number of instructions between the event and
    /// the recorded instruction pointer.
    Arbitrary = 0,

    /// There may be a constant number of instructions between the event and
    /// and the recorded instruction pointer.
    Constant = 1,

    /// We've requested that there be 0 skid. This does not guarantee that
    /// samples will actually have 0 skid.
    RequestZero = 2,

    /// Skid must be 0. If skid is 0 then the generated sample records will
    /// have the `PERF_RECORD_MISC_EXACT_IP` bit set.
    RequireZero = 3,
}

/// Supported linux clocks that can be used within a perf_event instance.
///
/// See the [`clock_gettime(2)`][0] manpage for the full documentation on what
/// each clock value actually means.
///
/// [0]: https://man7.org/linux/man-pages/man2/clock_gettime.2.html
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Clock(libc::clockid_t);

impl Clock {
    /// A clock following International Atomic Time.
    pub const TAI: Self = Self::new(libc::CLOCK_TAI);

    /// A clock that measures wall-clock time.
    pub const REALTIME: Self = Self::new(libc::CLOCK_REALTIME);

    /// A clock that is identical to `MONOTONIC` except it also includes any
    /// time during which the systems was suspended.
    pub const BOOTTIME: Self = Self::new(libc::CLOCK_BOOTTIME);

    /// A clock that (roughly) corresponds to the time that the system has been
    /// running since it was booted. (On Linux, at least).
    pub const MONOTONIC: Self = Self::new(libc::CLOCK_MONOTONIC);

    /// Similar to `MONOTONIC` but does not include NTP adjustments.
    pub const MONOTONIC_RAW: Self = Self::new(libc::CLOCK_MONOTONIC_RAW);
}

impl Clock {
    /// Construct a new `Clock` from the libc clockid value.
    pub const fn new(clockid: libc::clockid_t) -> Self {
        Self(clockid)
    }

    /// Extract the libc clockid value.
    pub const fn into_raw(self) -> libc::clockid_t {
        self.0
    }
}
