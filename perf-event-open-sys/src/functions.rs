//! Wrappers for the `perf_event_open` system call and related ioctls.
//!
//! This module provides Rust bindings for the `perf_event_open`
//! system call, as well as its associated ioctl calls, based on the
//! types in `bindings`.
//!
//! Normally, one would just get such things from the `libc` crate,
//! but the GNU C Library does not provide a C binding for the
//! `perf_event_open` system call, and the Rust `libc` crate follows
//! its lead. So we need to fill in the gap here.
//!
//! This module is only compiled on Linux. The `bindings` declarations
//! are useful on other platforms for parsing perf files.

use crate::bindings;

use libc::pid_t;
use std::os::raw::{c_int, c_ulong};

/// The `perf_event_open` system call.
///
/// See the [`perf_event_open(2) man page`][man] for details.
///
/// On error, this returns -1, and the C `errno` value (accessible via
/// `std::io::Error::last_os_error`) is set to indicate the error.
///
/// Note: The `attrs` argument needs to be a `*mut` because if the `size` field
/// is too small or too large, the kernel writes the size it was expecing back
/// into that field. It might do other things as well.
///
/// # Safety
///
/// The `attrs` argument must point to a properly initialized
/// `perf_event_attr` struct. The measurements and other behaviors its
/// contents request must be safe.
///
/// [man]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
pub unsafe fn perf_event_open(
    attrs: *mut bindings::perf_event_attr,
    pid: pid_t,
    cpu: c_int,
    group_fd: c_int,
    flags: c_ulong,
) -> c_int {
    libc::syscall(
        bindings::__NR_perf_event_open as libc::c_long,
        attrs as *const bindings::perf_event_attr,
        pid,
        cpu,
        group_fd,
        flags,
    ) as c_int
}

#[allow(dead_code, non_snake_case)]
pub mod ioctls {
    //! Ioctls for use with `perf_event_open` file descriptors.
    //!
    //! See the [`perf_event_open(2)`][man] man page for details.
    //!
    //! On error, these return `-1` and set the C `errno` value.
    //!
    //! [man]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
    use crate::bindings::{self, perf_event_attr, perf_event_query_bpf};
    use std::os::raw::{c_char, c_int, c_uint, c_ulong};

    macro_rules! define_ioctls {
        ( $( $args:tt )* ) => {
            $(
                define_ioctl!($args);
            )*
        }
    }

    macro_rules! define_ioctl {
        ({ $name:ident, $ioctl:ident, $arg_type:ty }) => {
            #[allow(clippy::missing_safety_doc)]
            pub unsafe fn $name(fd: c_int, arg: $arg_type) -> c_int {
                untyped_ioctl(fd, bindings::$ioctl, arg)
            }
        };
    }

    define_ioctls! {
        { ENABLE, ENABLE, c_uint }
        { DISABLE, DISABLE, c_uint }
        { REFRESH, REFRESH, c_int }
        { RESET, RESET, c_uint }
        { PERIOD, PERIOD, u64 }
        { SET_OUTPUT, SET_OUTPUT, c_int }
        { SET_FILTER, SET_FILTER, *mut c_char }
        { ID, ID, *mut u64 }
        { SET_BPF, SET_BPF, u32 }
        { PAUSE_OUTPUT, PAUSE_OUTPUT, u32 }
        { QUERY_BPF, QUERY_BPF, *mut perf_event_query_bpf }
        { MODIFY_ATTRIBUTES, MODIFY_ATTRIBUTES, *mut perf_event_attr }
    }

    unsafe fn untyped_ioctl<A>(fd: c_int, ioctl: bindings::perf_event_ioctls, arg: A) -> c_int {
        #[cfg(any(target_env = "musl", target_os = "android"))]
        return libc::ioctl(fd, ioctl as c_int, arg);

        #[cfg(not(any(target_env = "musl", target_os = "android")))]
        libc::ioctl(fd, ioctl as c_ulong, arg)
    }
}
