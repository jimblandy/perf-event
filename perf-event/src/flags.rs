#![allow(missing_docs)]

use bitflags::bitflags;

use crate::sys::bindings;

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
