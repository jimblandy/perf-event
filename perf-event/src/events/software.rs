use c_enum::c_enum;
use perf_event_open_sys::bindings;

use crate::events::Event;

c_enum! {
    /// Software counters, implemented by the kernel.
    ///
    /// Each variant of this enum corresponds to a particular `PERF_COUNT_SW_`...
    /// value supported by the [`perf_event_open`][man] system call.
    ///
    /// [man]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
    #[repr(transparent)]
    #[derive(Clone, Copy, Eq, PartialEq, Hash)]
    pub enum Software : u64 {
        /// High-resolution per-CPU timer.
        CPU_CLOCK = bindings::PERF_COUNT_SW_CPU_CLOCK as _,

        /// Per-task clock count.
        TASK_CLOCK = bindings::PERF_COUNT_SW_TASK_CLOCK as _,

        /// Page faults.
        PAGE_FAULTS = bindings::PERF_COUNT_SW_PAGE_FAULTS as _,

        /// Context switches.
        CONTEXT_SWITCHES = bindings::PERF_COUNT_SW_CONTEXT_SWITCHES as _,

        /// Process migration to another CPU.
        CPU_MIGRATIONS = bindings::PERF_COUNT_SW_CPU_MIGRATIONS as _,

        /// Minor page faults: resolved without needing I/O.
        PAGE_FAULTS_MIN = bindings::PERF_COUNT_SW_PAGE_FAULTS_MIN as _,

        /// Major page faults: I/O was required to resolve these.
        PAGE_FAULTS_MAJ = bindings::PERF_COUNT_SW_PAGE_FAULTS_MAJ as _,

        /// Alignment faults that required kernel intervention.
        ///
        /// This is only generated on some CPUs, and never on x86_64 or
        /// ARM.
        ALIGNMENT_FAULTS = bindings::PERF_COUNT_SW_ALIGNMENT_FAULTS as _,

        /// Instruction emulation faults.
        EMULATION_FAULTS = bindings::PERF_COUNT_SW_EMULATION_FAULTS as _,

        /// Placeholder, for collecting informational sample records.
        DUMMY = bindings::PERF_COUNT_SW_DUMMY as _,

        /// Special event type for streaming data from a eBPF program.
        ///
        /// See the documentation of the `bpf_perf_event_output` method in the
        /// [`bpf-helpers(7)`] manpage for details on how to use this event type.
        ///
        /// [`bpf-helpers(7)`]: https://man7.org/linux/man-pages/man7/bpf-helpers.7.html
        BPF_OUTPUT = bindings::PERF_COUNT_SW_BPF_OUTPUT as _,

        /// Context switches to a task in a different cgroup.
        CGROUP_SWITCHES = bindings::PERF_COUNT_SW_CGROUP_SWITCHES as _,
    }
}

impl Event for Software {
    fn update_attrs(self, attr: &mut bindings::perf_event_attr) {
        attr.type_ = bindings::PERF_TYPE_SOFTWARE;
        attr.config = self.into();
    }
}
