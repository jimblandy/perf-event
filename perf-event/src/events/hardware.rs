use c_enum::c_enum;
use perf_event_open_sys::bindings;

use crate::events::Event;

c_enum! {
    /// Hardware counters.
    ///
    /// These are counters implemented by the processor itself. Such counters vary
    /// from one architecture to the next, and even different models within a
    /// particular architecture will often change the way they expose this data.
    /// This is a selection of portable names for values that can be obtained on a
    /// wide variety of systems.
    ///
    /// Each variant of this enum corresponds to a particular `PERF_COUNT_HW_`...
    /// value supported by the [`perf_event_open`][man] system call.
    ///
    /// [man]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
    #[repr(transparent)]
    #[derive(Clone, Copy, Eq, PartialEq, Hash)]
    pub enum Hardware : u64 {
        /// Total cycles.
        CPU_CYCLES = bindings::PERF_COUNT_HW_CPU_CYCLES as _,

        /// Retired instructions.
        INSTRUCTIONS = bindings::PERF_COUNT_HW_INSTRUCTIONS as _,

        /// Cache accesses.
        CACHE_REFERENCES = bindings::PERF_COUNT_HW_CACHE_REFERENCES as _,

        /// Cache misses.
        CACHE_MISSES = bindings::PERF_COUNT_HW_CACHE_MISSES as _,

        /// Retired branch instructions.
        BRANCH_INSTRUCTIONS = bindings::PERF_COUNT_HW_BRANCH_INSTRUCTIONS as _,

        /// Mispredicted branch instructions.
        BRANCH_MISSES = bindings::PERF_COUNT_HW_BRANCH_MISSES as _,

        /// Bus cycles.
        BUS_CYCLES = bindings::PERF_COUNT_HW_BUS_CYCLES as _,

        /// Stalled cycles during issue.
        STALLED_CYCLES_FRONTEND = bindings::PERF_COUNT_HW_STALLED_CYCLES_FRONTEND as _,

        /// Stalled cycles during retirement.
        STALLED_CYCLES_BACKEND = bindings::PERF_COUNT_HW_STALLED_CYCLES_BACKEND as _,

        /// Total cycles, independent of frequency scaling.
        REF_CPU_CYCLES = bindings::PERF_COUNT_HW_REF_CPU_CYCLES as _,
    }
}

impl Event for Hardware {
    fn update_attrs(self, attr: &mut bindings::perf_event_attr) {
        attr.type_ = bindings::PERF_TYPE_HARDWARE;
        attr.config = self.into();
    }
}
