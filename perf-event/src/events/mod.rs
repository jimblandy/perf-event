//! Events we can monitor or count.
//!
//! There are a few general categories of event:
//!
//! - [`Hardware`] events are counted by the processor itself. This includes
//!   things like clock cycles, instructions retired, and cache and branch
//!   prediction statistics.
//!
//! - [`Cache`] events, also counted by the processor, offer a more detailed
//!   view of the processor's cache counters. You can select which level of the
//!   cache hierarchy to observe, discriminate between data and instruction
//!   caches, and so on.
//!
//! - [`Software`] events are counted by the kernel. This includes things like
//!   context switches, page faults, and so on.
//!
//! - [`Breakpoint`] events correspond to hardware breakpoints. They can count
//!   read/write accesses to an address as well as execution of an instruction
//!   address.
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

use std::sync::Arc;

use perf_event_open_sys::bindings::perf_event_attr;

use crate::{Builder, Counter};

used_in_docs!(Counter);
used_in_docs!(Builder);

mod breakpoint;
mod cache;
mod hardware;
mod software;

pub use self::breakpoint::{Breakpoint, BreakpointAccess};
pub use self::cache::{Cache, CacheId, CacheOp, CacheResult};
pub use self::hardware::Hardware;
pub use self::software::Software;

#[allow(deprecated)]
pub use self::cache::WhichCache;

/// An event that we can monitor or count.
pub trait Event: Sized {
    /// Update the [`perf_event_attr`] struct so that it will record the
    /// requested event.
    ///
    /// The field that need to be set in order to configure the kernel to
    /// collect various events can vary by quite a bit so this crate avoids
    /// putting any restrictions here by just passing the whole
    /// [`perf_event_attr`] struct.
    fn update_attrs(self, attr: &mut perf_event_attr);

    /// Update the [`perf_event_attr`] struct so that it will record the
    /// requested event.
    ///
    /// This is exactly the same as `update_attrs` except it optionally allows
    /// the Event implementor to return data that needs to live until the
    /// actual [`Counter`] is constructed.
    ///
    /// [`Builder`] will always call this method instead of `update_attrs`.
    fn update_attrs_with_data(self, attr: &mut perf_event_attr) -> Option<Arc<dyn EventData>> {
        self.update_attrs(attr);
        None
    }
}

/// Trait for owned event data.
///
/// This is automatically implemented for any type which is both `Send` and
/// `Sync`.
pub trait EventData: Send + Sync {}

impl<T: Send + Sync> EventData for T {}
