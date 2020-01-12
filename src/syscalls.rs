use crate::bindings;
use std::fs::File;
use std::io;
use std::os::raw::{c_int, c_ulong};
use std::os::unix::io::{FromRawFd, RawFd};

pub fn perf_event_open(
    attrs: &bindings::perf_event_attr,
    pid: bindings::__kernel_pid_t,
    cpu: c_int,
    group_fd: c_int,
    flags: c_ulong,
) -> io::Result<File> {
    let result = unsafe {
        libc::syscall(
            bindings::__NR_perf_event_open as libc::c_long,
            attrs as *const bindings::perf_event_attr,
            pid,
            cpu,
            group_fd,
            flags,
        )
    };

    if result < 0 {
        return Err(io::Error::last_os_error());
    }

    let file = unsafe { File::from_raw_fd(result as RawFd) };

    Ok(file)
}

#[allow(dead_code, non_snake_case)]
pub mod ioctls {
    use crate::bindings::{self, __u32, __u64, perf_event_attr, perf_event_query_bpf};
    use std::fs::File;
    use std::io;
    use std::os::raw::{c_char, c_int, c_ulong};
    use std::os::unix::io::AsRawFd;

    macro_rules! define_ioctls {
        ( $( $args:tt )* ) => {
            $(
                define_ioctl!($args);
            )*
        }
    }

    macro_rules! define_ioctl {
        ({ $name:ident, $ioctl:ident, $arg_type:ty }) => {
            pub unsafe fn $name(file: &File, arg: $arg_type) -> io::Result<c_int> {
                untyped_ioctl(file, bindings::$ioctl, arg)
            }
        };
    }

    define_ioctls! {
        { ENABLE, perf_event_ioctls_ENABLE, c_int }
        { DISABLE, perf_event_ioctls_DISABLE, c_int }
        { REFRESH, perf_event_ioctls_REFRESH, c_int }
        { RESET, perf_event_ioctls_RESET, c_int }
        { PERIOD, perf_event_ioctls_PERIOD, __u64 }
        { SET_OUTPUT, perf_event_ioctls_SET_OUTPUT, c_int }
        { SET_FILTER, perf_event_ioctls_SET_FILTER, *mut c_char }
        { ID, perf_event_ioctls_ID, *mut __u64 }
        { SET_BPF, perf_event_ioctls_SET_BPF, __u32 }
        { PAUSE_OUTPUT, perf_event_ioctls_PAUSE_OUTPUT, __u32 }
        { QUERY_BPF, perf_event_ioctls_QUERY_BPF, *mut perf_event_query_bpf }
        { MODIFY_ATTRIBUTES, perf_event_ioctls_MODIFY_ATTRIBUTES, *mut perf_event_attr }
    }

    unsafe fn untyped_ioctl<A>(
        file: &File,
        ioctl: bindings::perf_event_ioctls,
        arg: A,
    ) -> io::Result<c_int> {
        let result = libc::ioctl(file.as_raw_fd() as c_int, ioctl as c_ulong, arg);

        if result < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(result)
    }
}
