//! Intercepting perf-event system calls, for testing and logging.
//!
//! Note: this module is only available when the `"hooks"` feature is enabled.
//!
//! Many performance counters' behavior is inherently
//! non-deterministic, making it difficult to write tests for code
//! that uses the `perf_event` crate. There may be no way to reliably
//! provoke the Linux kernel into exhibiting the behavior you want to
//! test against. Or you may want to test functionality like
//! whole-system profiling, which requires elevated privileges that
//! one would prefer to avoid granting to tests.
//!
//! This module lets you interpose your own implementation of all the
//! system calls and ioctls that `perf_event` uses, granting you
//! complete control over `perf_event`'s interactions with the outside
//! world. You can verify that the system calls receive the parameters
//! you expect, and provide whatever sorts of interesting responses
//! you need.
//!
//! There are three main pieces:
//!
//! - The [`Hooks`] trait has a method for every system call and ioctl
//!   that the `perf_event` crate uses.
//!
//! - The [`set_thread_hooks`] function lets you provide a `Box<dyn Hooks>`
//!   trait object whose methods the calling thread will use for all subsequent
//!   `perf_event` operations.
//!
//! - The [`clear_thread_hooks`] function restores the thread's
//!   original state, so that subsequent `perf_event` operations use
//!   the real Linux system calls.
//!
//! This functionality is too low-level for direct use in tests, but
//! it does provide the means with which one can build more ergonomic
//! test harnesses.
//!
//! ## Stability
//!
//! Using `set_thread_hooks`, you can observe the exact sequence of
//! system operations that the `perf_event` crate performs to carry
//! out requests from the user. Even if the interface remains the
//! same, the implementation of those requests can change without
//! notice, possibly causing a [`Hooks`] implementation to see a
//! different set of calls.
//!
//! The `perf_event` crate will not treat such implementation changes
//! as breaking changes for semver purposes, despite the fact that
//! they may break code using this module's functionality.
use libc::pid_t;
use perf_event_open_sys as real;
use perf_event_open_sys::bindings;
use std::cell::RefCell;
use std::os::raw::{c_char, c_int, c_uint, c_ulong};

std::thread_local! {
    static HOOKS: RefCell<Box<dyn Hooks + 'static>> = RefCell::new(Box::new(RealHooks));
}

/// Direct all perf-event system calls on this thread to `hooks`.
///
/// All subsequent uses by this crate of the underlying system calls
/// and ioctls from the `perf_event_open_sys` crate are redirected to
/// `hooks`' implementations of the correspoding methods from the
/// [`Hooks`] trait.
///
/// This affects only the calling thread. Any previously established
/// hooks on that thread are dropped.
///
/// # Safety
///
/// The specified `hooks` trait object intercepts calls provoked by
/// previously created [`Counter`] and [`Group`] objects, regardless
/// of which hooks were in effect when they were created. This could
/// make a hash of things.
///
/// [`Counter`]: crate::Counter
/// [`Group`]: crate::Group
pub unsafe fn set_thread_hooks(hooks: Box<dyn Hooks + 'static>) {
    HOOKS.with(|per_thread| {
        *per_thread.borrow_mut() = hooks;
    })
}

/// Direct all perf-event system calls on this thread to the real system calls.
///
/// All subsequent uses by this crate of the underlying system calls
/// and ioctls from the `perf_event_open_sys` crate are directed to
/// the underlying Linux operations, without interference.
///
/// This affects only the calling thread. Any previously established
/// hooks on that thread are dropped.
///
/// # Safety
///
/// The specified `hooks` trait object intercepts calls provoked by
/// previously created [`Counter`] and [`Group`] values, regardless of
/// which hooks were in effect when they were created. Letting values
/// created using hooked system calls suddenly see the real kernel
/// could make a hash of things.
///
/// [`Counter`]: crate::Counter
/// [`Group`]: crate::Group
pub unsafe fn clear_thread_hooks() {
    HOOKS.with(|per_thread| {
        *per_thread.borrow_mut() = Box::new(RealHooks);
    })
}

/// List of ioctls we need wrappers for.
///
/// We use this macro to generate the [`Hooks`] trait's definition,
/// the [`RealHooks`] implementation, and the functions in the `sys`
/// module that are actually used by callers.
macro_rules! define_ioctls {
    ( $expand:ident ) => {
        $expand ! { ENABLE, perf_event_ioctls_ENABLE, c_uint }
        $expand ! { DISABLE, perf_event_ioctls_DISABLE, c_uint }
        $expand ! { REFRESH, perf_event_ioctls_REFRESH, c_int }
        $expand ! { RESET, perf_event_ioctls_RESET, c_uint }
        $expand ! { PERIOD, perf_event_ioctls_PERIOD, u64 }
        $expand ! { SET_OUTPUT, perf_event_ioctls_SET_OUTPUT, c_int }
        $expand ! { SET_FILTER, perf_event_ioctls_SET_FILTER, *mut c_char }
        $expand ! { ID, perf_event_ioctls_ID, *mut u64 }
        $expand ! { SET_BPF, perf_event_ioctls_SET_BPF, u32 }
        $expand ! { PAUSE_OUTPUT, perf_event_ioctls_PAUSE_OUTPUT, u32 }
        $expand ! { QUERY_BPF, perf_event_ioctls_QUERY_BPF, *mut bindings::perf_event_query_bpf }
        $expand ! { MODIFY_ATTRIBUTES, perf_event_ioctls_MODIFY_ATTRIBUTES, *mut bindings::perf_event_attr }
    }
}

macro_rules! expand_trait_method {
    ( $name:ident, $ioctl:ident, $arg_type:ty ) => {
        /// Wrapper for perf_event ioctl
        #[doc = stringify!($ioctl)]
        /// .
        #[allow(non_snake_case)]
        unsafe fn $name(&mut self, _fd: c_int, _arg: $arg_type) -> c_int {
            panic!(
                "unimplemented `perf_event::hooks::Hooks` method: {}",
                stringify!($name)
            );
        }
    };
}

/// A trait with a method for every system call and ioctl used by this crate.
///
/// The methods of this trait correspond to the public functions of
/// the [`perf_event_open_sys`][peos] crate used to implement this
/// crate's functionality. For testing purposes, you can redirect this
/// crate to a value of your own design that implements this trait by
/// calling [`set_thread_hooks`].
///
/// Each method has a default definition that panics. This means that
/// you only need to provide definitions for the operations your tests
/// actually use; if they touch anything else, you'll get a failure.
///
/// The [`RealHooks`] type implements this trait in terms of the real
/// Linux system calls and ioctls.
///
/// [peos]: https://docs.rs/perf-event-open-sys/latest/perf_event_open_sys/
#[allow(dead_code)]
pub trait Hooks {
    /// See [`perf_event_open_sys::perf_event_open`][peo].
    ///
    /// [peo]: https://docs.rs/perf-event-open-sys/latest/perf_event_open_sys/fn.perf_event_open.html
    #[allow(clippy::missing_safety_doc)]
    unsafe fn perf_event_open(
        &mut self,
        attrs: *mut bindings::perf_event_attr,
        pid: pid_t,
        cpu: c_int,
        group_fd: c_int,
        flags: c_ulong,
    ) -> c_int;
    define_ioctls!(expand_trait_method);
}

macro_rules! expand_realhooks_impl {
    ( $name:ident, $ioctl_:ident, $arg_type:ty ) => {
        #[allow(clippy::missing_safety_doc)]
        unsafe fn $name(&mut self, fd: c_int, arg: $arg_type) -> c_int {
            real::ioctls::$name(fd, arg)
        }
    };
}

/// An implementation of the [`Hooks`] trait in terms of the real Linux system calls.
///
/// This type implements each methods of the [`Hooks`] trait by
/// calling the underlying system call or ioctl. The following call
/// is equivalent to calling [`clear_thread_hooks`]:
///
///     # use perf_event::hooks;
///     # use perf_event::hooks::*;
///     unsafe {
///         set_thread_hooks(Box::new(RealHooks));
///     }
///
/// If what you want is non-intercepted access to the underlying
/// system calls, it's probably better to just access the
/// [`perf_event_open_sys`][peos] crate directly, rather than using this type.
///
/// [peos]: https://docs.rs/perf-event-open-sys/latest/perf_event_open_sys/
pub struct RealHooks;
impl Hooks for RealHooks {
    unsafe fn perf_event_open(
        &mut self,
        attrs: *mut bindings::perf_event_attr,
        pid: pid_t,
        cpu: c_int,
        group_fd: c_int,
        flags: c_ulong,
    ) -> c_int {
        real::perf_event_open(attrs, pid, cpu, group_fd, flags)
    }

    define_ioctls!(expand_realhooks_impl);
}

/// Wrapper around the `perf_event_open_sys` crate that supports
/// intercepting system calls and returning simulated results, for
/// testing.
pub mod sys {
    use super::HOOKS;
    use libc::pid_t;
    use std::os::raw::{c_int, c_ulong};

    pub use perf_event_open_sys::bindings;

    /// See [`perf_event_open_sys::perf_event_open`][peo].
    ///
    /// [peo]: https://docs.rs/perf-event-open-sys/latest/perf_event_open_sys/fn.perf_event_open.html
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn perf_event_open(
        attrs: *mut bindings::perf_event_attr,
        pid: pid_t,
        cpu: c_int,
        group_fd: c_int,
        flags: c_ulong,
    ) -> c_int {
        HOOKS.with(|hooks| {
            hooks
                .borrow_mut()
                .perf_event_open(attrs, pid, cpu, group_fd, flags)
        })
    }

    #[allow(dead_code, non_snake_case)]
    /// See the [`perf_event_open_sys::ioctl` module][peosi].
    ///
    /// [peosi]: https://docs.rs/perf-event-open-sys/latest/perf_event_open_sys/ioctls/index.html
    pub mod ioctls {
        use super::HOOKS;
        use perf_event_open_sys::bindings;
        use std::os::raw::{c_char, c_int, c_uint};

        macro_rules! expand_hooked_ioctl {
            ( $name:ident, $ioctl_:ident, $arg_type:ty ) => {
                /// See the [`perf_event_open_sys::ioctl` module][peosi].
                ///
                /// [peosi]: https://docs.rs/perf-event-open-sys/latest/perf_event_open_sys/ioctls/index.html
                #[allow(clippy::missing_safety_doc)]
                pub unsafe fn $name(fd: c_int, arg: $arg_type) -> c_int {
                    HOOKS.with(|hooks| hooks.borrow_mut().$name(fd, arg))
                }
            };
        }

        define_ioctls!(expand_hooked_ioctl);
    }
}
