//! Events we can monitor or count.
//!
//! There are three general categories of event:
//!
//! -   [`Hardware`] events are counted by the processor itself. This
//!     includes things like clock cycles, instructions retired, and cache and
//!     branch prediction statistics.
//!
//! -   [`Software`] events are counted by the kernel. This includes things
//!     like context switches, page faults, and so on.
//!
//! -   [`Cache`] events offer a more detailed view of the processor's cache
//!     counters. You can select which level of the cache hierarchy to observe,
//!     discriminate between data and instruction caches, and so on.
//!
//! The `Event` type is just an enum with a variant for each of the above types,
//! which all implement `Into<Event>`.
//!
//! Linux supports many more kinds of events than this module covers, including
//! events specific to particular make and model of processor, and events that
//! are dynamically registered by drivers and kernel modules. If something you
//! want is missing, think about the best API to expose it, and submit a pull
//! request!
//!
//! [`Hardware`]: enum.Hardware.html
//! [`Software`]: enum.Software.html
//! [`Cache`]: struct.Cache.html

#![allow(non_camel_case_types)]
use perf_event_open_sys::bindings as bindings;

/// Any sort of event. This is a sum of the [`Hardware`],
/// [`Software`], and [`Cache`] types, which all implement
/// `Into<Event>`.
///
/// [`Hardware`]: enum.Hardware.html
/// [`Software`]: enum.Software.html
/// [`Cache`]: struct.Cache.html
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    #[allow(missing_docs)]
    Hardware(Hardware),

    #[allow(missing_docs)]
    Software(Software),

    #[allow(missing_docs)]
    Cache(Cache),
}

impl Event {
    pub(crate) fn as_type(&self) -> bindings::perf_type_id {
        match self {
            Event::Hardware(_) => bindings::perf_type_id_PERF_TYPE_HARDWARE,
            Event::Software(_) => bindings::perf_type_id_PERF_TYPE_SOFTWARE,
            Event::Cache(_) => bindings::perf_type_id_PERF_TYPE_HW_CACHE,
        }
    }

    pub(crate) fn as_config(self) -> u64 {
        match self {
            Event::Hardware(hw) => hw as _,
            Event::Software(sw) => sw as _,
            Event::Cache(cache) => cache.as_config(),
        }
    }
}

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
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Hardware {
    /// Total cycles.  Be wary of what happens during CPU frequency scaling.
    CPU_CYCLES = bindings::perf_hw_id_PERF_COUNT_HW_CPU_CYCLES,

    /// Retired instructions. Be careful, these can be affected by various
    /// issues, most notably hardware interrupt counts.
    INSTRUCTIONS = bindings::perf_hw_id_PERF_COUNT_HW_INSTRUCTIONS,

    /// Cache accesses. Usually this indicates Last Level Cache accesses but
    /// this may vary depending on your CPU. This may include prefetches and
    /// coherency messages; again this depends on the design of your CPU.
    CACHE_REFERENCES = bindings::perf_hw_id_PERF_COUNT_HW_CACHE_REFERENCES,

    /// Cache misses. Usually this indicates Last Level Cache misses; this is
    /// intended to be used in conjunction with the
    /// PERF_COUNT_HW_CACHE_REFERENCES event to calculate cache miss rates.
    CACHE_MISSES = bindings::perf_hw_id_PERF_COUNT_HW_CACHE_MISSES,

    /// Retired branch instructions. Prior to Linux 2.6.35, this used the wrong
    /// event on AMD processors.
    BRANCH_INSTRUCTIONS = bindings::perf_hw_id_PERF_COUNT_HW_BRANCH_INSTRUCTIONS,

    /// Mispredicted branch instructions.
    BRANCH_MISSES = bindings::perf_hw_id_PERF_COUNT_HW_BRANCH_MISSES,

    /// Bus cycles, which can be different from total cycles.
    BUS_CYCLES = bindings::perf_hw_id_PERF_COUNT_HW_BUS_CYCLES,

    /// Stalled cycles during issue. (since Linux 3.0)
    STALLED_CYCLES_FRONTEND = bindings::perf_hw_id_PERF_COUNT_HW_STALLED_CYCLES_FRONTEND,

    /// Stalled cycles during retirement. (since Linux 3.0)
    STALLED_CYCLES_BACKEND = bindings::perf_hw_id_PERF_COUNT_HW_STALLED_CYCLES_BACKEND,

    /// Total cycles; not affected by CPU frequency scaling. (since Linux 3.3)
    REF_CPU_CYCLES = bindings::perf_hw_id_PERF_COUNT_HW_REF_CPU_CYCLES,
}

impl From<Hardware> for Event {
    fn from(hw: Hardware) -> Event {
        Event::Hardware(hw)
    }
}

/// Software counters, implemented by the kernel.
///
/// Each variant of this enum corresponds to a particular `PERF_COUNT_SW_`...
/// value supported by the [`perf_event_open`][man] system call.
///
/// [man]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Software {
    /// This reports the CPU clock, a high-resolution per-CPU timer.
    CPU_CLOCK = bindings::perf_sw_ids_PERF_COUNT_SW_CPU_CLOCK,

    /// This reports a clock count specific to the task that is running.
    TASK_CLOCK = bindings::perf_sw_ids_PERF_COUNT_SW_TASK_CLOCK,

    /// This reports the number of page faults.
    PAGE_FAULTS = bindings::perf_sw_ids_PERF_COUNT_SW_PAGE_FAULTS,

    /// This counts context switches. Until Linux 2.6.34, these were all
    /// reported as user-space events, after that they are reported as happening
    /// in the kernel.
    CONTEXT_SWITCHES = bindings::perf_sw_ids_PERF_COUNT_SW_CONTEXT_SWITCHES,

    /// This reports the number of times the process has migrated to a new CPU.
    CPU_MIGRATIONS = bindings::perf_sw_ids_PERF_COUNT_SW_CPU_MIGRATIONS,

    /// This counts the number of minor page faults. These did not require disk
    /// I/O to handle.
    PAGE_FAULTS_MIN = bindings::perf_sw_ids_PERF_COUNT_SW_PAGE_FAULTS_MIN,

    /// This counts the number of major page faults. These required disk I/O to
    /// handle.
    PAGE_FAULTS_MAJ = bindings::perf_sw_ids_PERF_COUNT_SW_PAGE_FAULTS_MAJ,

    /// (since Linux 2.6.33) This counts the number of alignment faults. These
    /// happen when unaligned memory accesses happen; the kernel can handle
    /// these but it reduces performance. This happens only on some
    /// architectures (never on x86).
    ALIGNMENT_FAULTS = bindings::perf_sw_ids_PERF_COUNT_SW_ALIGNMENT_FAULTS,

    /// (since Linux 2.6.33) This counts the number of emulation faults. The
    /// kernel sometimes traps on unimplemented instructions and emulates them
    /// for user space. This can negatively impact performance.
    EMULATION_FAULTS = bindings::perf_sw_ids_PERF_COUNT_SW_EMULATION_FAULTS,

    /// (since Linux 3.12) This is a placeholder event that counts nothing.
    /// Informational sample record types such as mmap or comm must be
    /// associated with an active event. This dummy event allows gathering such
    /// records without requiring a counting event.
    DUMMY = bindings::perf_sw_ids_PERF_COUNT_SW_DUMMY,
}

impl From<Software> for Event {
    fn from(hw: Software) -> Event {
        Event::Software(hw)
    }
}

/// A cache event.
///
/// A cache event has three identifying characteristics:
///
/// - which cache to observe ([`which`])
///
/// - what sort of request it's handling ([`operation`])
///
/// - whether we want to count all cache accesses, or just misses
///   ([`result`]).
///
/// For example, to measure the L1 data cache's miss rate:
///
///     # use perf_event::{Builder, Group};
///     # use perf_event::events::{Cache, CacheOp, CacheResult, Hardware, WhichCache};
///     # fn main() -> std::io::Result<()> {
///     // A `Cache` value representing L1 data cache read accesses.
///     const ACCESS: Cache = Cache {
///         which: WhichCache::L1D,
///         operation: CacheOp::READ,
///         result: CacheResult::ACCESS,
///     };
///
///     // A `Cache` value representing L1 data cache read misses.
///     const MISS: Cache = Cache { result: CacheResult::MISS, ..ACCESS };
///
///     // Construct a `Group` containing the two new counters, from which we
///     // can get counts over matching periods of time.
///     let mut group = Group::new()?;
///     let access_counter = Builder::new().group(&mut group).kind(ACCESS).build()?;
///     let miss_counter = Builder::new().group(&mut group).kind(MISS).build()?;
///     # Ok(()) }
///
/// [`which`]: enum.WhichCache.html
/// [`operation`]: enum.CacheOp.html
/// [`result`]: enum.CacheResult.html
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cache {
    /// Which cache is being monitored? (data, instruction, ...)
    pub which: WhichCache,

    /// What operation is being monitored? (read, write, etc.)
    pub operation: CacheOp,

    /// All accesses, or just misses?
    pub result: CacheResult,
}

impl From<Cache> for Event {
    fn from(hw: Cache) -> Event {
        Event::Cache(hw)
    }
}

impl Cache {
    fn as_config(&self) -> u64 {
        self.which as u64 |
        ((self.operation as u64) << 8) |
        ((self.result as u64) << 16)
    }
}

/// A cache whose events we would like to count.
///
/// This is used in the `Cache` type as part of the identification of a cache
/// event. Each variant here corresponds to a particular
/// `PERF_COUNT_HW_CACHE_...` constant supported by the [`perf_event_open`][man]
/// system call.
///
/// [man]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
#[repr(u32)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum WhichCache {
    /// for measuring Level 1 Data Cache
    L1D = bindings::perf_hw_cache_id_PERF_COUNT_HW_CACHE_L1D,

    /// for measuring Level 1 Instruction Cache
    L1I = bindings::perf_hw_cache_id_PERF_COUNT_HW_CACHE_L1I,

    /// for measuring Last-Level Cache
    LL = bindings::perf_hw_cache_id_PERF_COUNT_HW_CACHE_LL,

    /// for measuring the Data TLB
    DTLB = bindings::perf_hw_cache_id_PERF_COUNT_HW_CACHE_DTLB,

    /// for measuring the Instruction TLB
    ITLB = bindings::perf_hw_cache_id_PERF_COUNT_HW_CACHE_ITLB,

    /// for measuring the branch prediction unit
    BPU = bindings::perf_hw_cache_id_PERF_COUNT_HW_CACHE_BPU,

    /// (since Linux 3.1) for measuring local memory accesses
    NODE = bindings::perf_hw_cache_id_PERF_COUNT_HW_CACHE_NODE,
}

/// What sort of cache operation we would like to observe.
///
/// This is used in the `Cache` type as part of the identification of a cache
/// event. Each variant here corresponds to a particular
/// `PERF_COUNT_HW_CACHE_OP_...` constant supported by the
/// [`perf_event_open`][man] system call.
///
/// [man]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
#[repr(u32)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CacheOp {
    /// Read accesses.
    READ = bindings::perf_hw_cache_op_id_PERF_COUNT_HW_CACHE_OP_READ,

    /// Write accesses.
    WRITE = bindings::perf_hw_cache_op_id_PERF_COUNT_HW_CACHE_OP_WRITE,

    /// Prefetch accesses.
    PREFETCH = bindings::perf_hw_cache_op_id_PERF_COUNT_HW_CACHE_OP_PREFETCH,
}

#[repr(u32)]
/// What sort of cache result we're interested in observing.
///
/// `ACCESS` counts the total number of operations performed on the cache,
/// whereas `MISS` counts only those requests that the cache could not satisfy.
/// Treating `MISS` as a fraction of `ACCESS` gives you the cache's miss rate.
///
/// This is used used in the `Cache` type as part of the identification of a
/// cache event. Each variant here corresponds to a particular
/// `PERF_COUNT_HW_CACHE_RESULT_...` constant supported by the
/// [`perf_event_open`][man] system call.
///
/// [man]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CacheResult {
    /// to measure accesses
    ACCESS = bindings::perf_hw_cache_op_result_id_PERF_COUNT_HW_CACHE_RESULT_ACCESS,

    /// to measure misses
    MISS = bindings::perf_hw_cache_op_result_id_PERF_COUNT_HW_CACHE_RESULT_MISS,
}
