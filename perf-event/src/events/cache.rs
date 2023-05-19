use c_enum::c_enum;
use perf_event_open_sys::bindings;

use crate::events::Event;

/// A cache event.
///
/// A cache event has three identifying characteristics:
///
/// - which cache to observe ([`which`])
///
/// - what sort of request it's handling ([`operation`])
///
/// - whether we want to count all cache accesses, or just misses ([`result`]).
///
/// For example, to measure the L1 data cache's miss rate:
///
/// ```
/// # use perf_event::{Builder, Group};
/// # use perf_event::events::{Cache, CacheOp, CacheResult, Hardware, WhichCache};
/// # fn main() -> std::io::Result<()> {
/// // A `Cache` value representing L1 data cache read accesses.
/// const ACCESS: Cache = Cache {
///     which: WhichCache::L1D,
///     operation: CacheOp::READ,
///     result: CacheResult::ACCESS,
/// };
///
/// // A `Cache` value representing L1 data cache read misses.
/// const MISS: Cache = Cache {
///     result: CacheResult::MISS,
///     ..ACCESS
/// };
///
/// // Construct a `Group` containing the two new counters, from which we
/// // can get counts over matching periods of time.
/// let mut group = Group::new()?;
/// let access_counter = group.add(&Builder::new(ACCESS))?;
/// let miss_counter = group.add(&Builder::new(MISS))?;
/// # Ok(()) }
/// ```
///
/// [`which`]: enum.WhichCache.html
/// [`operation`]: enum.CacheOp.html
/// [`result`]: enum.CacheResult.html
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cache {
    /// Which cache is being monitored? (data, instruction, ...)
    pub which: CacheId,

    /// What operation is being monitored? (read, write, etc.)
    pub operation: CacheOp,

    /// All accesses, or just misses?
    pub result: CacheResult,
}

impl Cache {
    fn as_config(&self) -> u64 {
        self.which.0 as u64 | ((self.operation.0 as u64) << 8) | ((self.result.0 as u64) << 16)
    }
}

impl Event for Cache {
    fn update_attrs(self, attr: &mut bindings::perf_event_attr) {
        attr.type_ = bindings::PERF_TYPE_HW_CACHE;
        attr.config = self.as_config()
    }
}

#[doc(hidden)]
#[deprecated = "WhichCache has been renamed to CacheId"]
pub type WhichCache = CacheId;

c_enum! {
    /// A cache whose events we would like to count.
    ///
    /// This is used in the `Cache` type as part of the identification of a cache
    /// event. Each variant here corresponds to a particular
    /// `PERF_COUNT_HW_CACHE_...` constant supported by the [`perf_event_open`][man]
    /// system call.
    ///
    /// [man]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
    #[repr(transparent)]
    #[derive(Clone, Copy, Eq, PartialEq, Hash)]
    pub enum CacheId : u8 {
        /// Level 1 data cache.
        L1D = bindings::PERF_COUNT_HW_CACHE_L1D as _,

        /// Level 1 instruction cache.
        L1I = bindings::PERF_COUNT_HW_CACHE_L1I as _,

        /// Last-level cache.
        LL = bindings::PERF_COUNT_HW_CACHE_LL as _,

        /// Data translation lookaside buffer (virtual address translation).
        DTLB = bindings::PERF_COUNT_HW_CACHE_DTLB as _,

        /// Instruction translation lookaside buffer (virtual address translation).
        ITLB = bindings::PERF_COUNT_HW_CACHE_ITLB as _,

        /// Branch prediction.
        BPU = bindings::PERF_COUNT_HW_CACHE_BPU as _,

        /// Memory accesses that stay local to the originating NUMA node.
        NODE = bindings::PERF_COUNT_HW_CACHE_NODE as _,
    }

    /// What sort of cache operation we would like to observe.
    ///
    /// This is used in the `Cache` type as part of the identification of a cache
    /// event. Each variant here corresponds to a particular
    /// `PERF_COUNT_HW_CACHE_OP_...` constant supported by the
    /// [`perf_event_open`][man] system call.
    ///
    /// [man]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
    #[repr(transparent)]
    #[derive(Clone, Copy, Eq, PartialEq, Hash)]
    pub enum CacheOp : u8 {
        /// Read accesses.
        READ = bindings::PERF_COUNT_HW_CACHE_OP_READ as _,

        /// Write accesses.
        WRITE = bindings::PERF_COUNT_HW_CACHE_OP_WRITE as _,

        /// Prefetch accesses.
        PREFETCH = bindings::PERF_COUNT_HW_CACHE_OP_PREFETCH as _,
    }

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
    #[repr(transparent)]
    #[derive(Clone, Copy, Eq, PartialEq, Hash)]
    pub enum CacheResult : u8 {
        /// Cache was accessed.
        ACCESS = bindings::PERF_COUNT_HW_CACHE_RESULT_ACCESS as _,

        /// Cache access was a miss.
        MISS = bindings::PERF_COUNT_HW_CACHE_RESULT_MISS as _,
    }
}
