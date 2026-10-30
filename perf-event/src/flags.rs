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
