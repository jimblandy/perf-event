use crate::Builder;
use crate::ReadFormat;

used_in_docs!(Builder);

pub(crate) trait ReadFormatExt: Sized {
    const MAX_NON_GROUP_SIZE: usize;
    fn prefix_len(&self) -> usize;
    fn element_len(&self) -> usize;
}

impl ReadFormatExt for ReadFormat {
    const MAX_NON_GROUP_SIZE: usize = Self::all() //
        .difference(Self::GROUP)
        .bits()
        .count_ones() as usize
        + 1;

    // The format of a read from a group is like this
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

    /// The size of the common prefix when reading a group.
    fn prefix_len(&self) -> usize {
        1 + (*self & (Self::TOTAL_TIME_ENABLED | Self::TOTAL_TIME_RUNNING))
            .bits()
            .count_ones() as usize
    }

    /// The size of each element when reading a group
    fn element_len(&self) -> usize {
        1 + (*self & (Self::ID | Self::LOST)).bits().count_ones() as usize
    }
}

/// Configuration of how much skid is allowed when gathering samples.
///
/// Skid is the number of instructions that occur between an event occuring and
/// a sample being gathered by the kernel. Less skid is better but there are
/// hardware limitations around how small the skid can be.
///
/// Also see [`Builder::precise_ip`].
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
