use bitflags::bitflags;

use crate::sys::bindings;
use crate::Builder;

used_in_docs!(Builder);

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
/// [0]: https://www.mankier.com/2/clock_gettime
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

bitflags! {
    /// Specify what branches to include in a branch record.
    ///
    /// This is used by the builder in combination with setting
    /// [`SampleFlag::BRANCH_STACK`].
    ///
    /// The first part of the value is the privilege level, which is a
    /// combination of `USER`, `BRANCH`, or `HV`. `PLM_ALL` is a convenience
    /// value with all 3 ORed together. If none of the privilege levels are set
    /// then the kernel will use the privilege level of the event.
    ///
    /// The second part specifies which branch types are to be included in the
    /// branch stack. At least one of these bits must be set.
    pub struct SampleBranchFlag: u64 {
        /// The branch target is in user space.
        const USER = bindings::PERF_SAMPLE_BRANCH_USER as _;

        /// The branch target is in kernel space.
        const KERNEL = bindings::PERF_SAMPLE_BRANCH_KERNEL as _;

        /// The branch target is in the hypervisor.
        const HV = bindings::PERF_SAMPLE_BRANCH_HV as _;

        /// Include any branch type.
        const ANY = bindings::PERF_SAMPLE_BRANCH_ANY as _;

        /// Include any call branch.
        ///
        /// This includes direct calls, indirect calls, and far jumps.
        const ANY_CALL = bindings::PERF_SAMPLE_BRANCH_ANY_CALL as _;

        /// Include indirect calls.
        const IND_CALL = bindings::PERF_SAMPLE_BRANCH_IND_CALL as _;

        /// Include direct calls.
        const CALL = bindings::PERF_SAMPLE_BRANCH_CALL as _;

        /// Include any return branch.
        const ANY_RETURN = bindings::PERF_SAMPLE_BRANCH_ANY_RETURN as _;

        /// Include indirect jumps.
        const IND_JUMP = bindings::PERF_SAMPLE_BRANCH_IND_JUMP as _;

        /// Include conditional branches.
        const COND = bindings::PERF_SAMPLE_BRANCH_COND as _;

        /// Include transactional memory aborts.
        const ABORT_TX = bindings::PERF_SAMPLE_BRANCH_ABORT_TX as _;

        /// Include branches in a transactional memory transaction.
        const IN_TX = bindings::PERF_SAMPLE_BRANCH_IN_TX as _;

        /// Include branches not in a transactional memory transaction.
        const NO_TX = bindings::PERF_SAMPLE_BRANCH_NO_TX as _;

        /// Include branches that are part of a hardware-generated call stack.
        ///
        /// Note that this requires hardware support. See the [manpage][0] for
        /// platforms which support this.
        ///
        /// [0]: https://www.mankier.com/2/perf_event_open
        const CALL_STACK = bindings::PERF_SAMPLE_BRANCH_CALL_STACK as _;
    }
}

impl SampleBranchFlag {
    /// All privilege levels (`USER`, `KERNEL`, and `HV`) ORed together.
    pub const PLM_ALL: Self = Self::USER.union(Self::KERNEL).union(Self::HV);
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
