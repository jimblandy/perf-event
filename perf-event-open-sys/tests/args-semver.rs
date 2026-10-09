/*! Check for semver-incompatible changes to function arguments.

If you make a change to `perf-event-open-sys` that causes compilation
errors in this file, that suggests that the next release that includes
your change may be semver-incompatible with its predecessor, so it may
need a new major version number.

As explained in [cargo-semver-checks#637], the `cargo semver-checks`
command cannot detect semver incompatibility caused by the types of
function arguments changing.

This is a serious gap in coverage. For example, fixing [perf-event#79]
will require assigning a new major version, but `cargo semver-checks`
will not warn us about this.

To work around the problem, for every public function in
`perf-event-open-sys`, this test file defines a function that takes
the same argument types, and calls the public function. Since they're
passing parameters, the values' types are fixed, so type inference
can't paper over changes by simply inferring different types for
actual parameters.

[cargo-semver-checks#637]: https://github.com/obi1kenobi/cargo-semver-checks/issues/637

*/
#![allow(dead_code, non_snake_case)]

use perf_event_open_sys::bindings::{self, perf_event_attr, perf_event_query_bpf};
use std::os::raw::{c_char, c_int, c_uint, c_ulong};

use libc::pid_t;

fn call_perf_event_open(
    attrs: *mut bindings::perf_event_attr,
    pid: pid_t,
    cpu: c_int,
    group_fd: c_int,
    flags: c_ulong,
) {
    unsafe {
        let _ = perf_event_open_sys::perf_event_open(attrs, pid, cpu, group_fd, flags);
    };
}

fn call_ioctls_ENABLE(fd: c_int, arg: c_uint) {
    unsafe {
        let _ = perf_event_open_sys::ioctls::ENABLE(fd, arg);
    };
}

fn call_ioctls_DISABLE(fd: c_int, arg: c_uint) {
    unsafe {
        let _ = perf_event_open_sys::ioctls::DISABLE(fd, arg);
    };
}

fn call_ioctls_REFRESH(fd: c_int, arg: c_int) {
    unsafe {
        let _ = perf_event_open_sys::ioctls::REFRESH(fd, arg);
    };
}

fn call_ioctls_RESET(fd: c_int, arg: c_uint) {
    unsafe {
        let _ = perf_event_open_sys::ioctls::RESET(fd, arg);
    };
}

fn call_ioctls_PERIOD(fd: c_int, arg: u64) {
    unsafe {
        let _ = perf_event_open_sys::ioctls::PERIOD(fd, arg);
    };
}

fn call_ioctls_SET_OUTPUT(fd: c_int, arg: c_int) {
    unsafe {
        let _ = perf_event_open_sys::ioctls::SET_OUTPUT(fd, arg);
    };
}

fn call_ioctls_SET_FILTER(fd: c_int, arg: *mut c_char) {
    unsafe {
        let _ = perf_event_open_sys::ioctls::SET_FILTER(fd, arg);
    };
}

fn call_ioctls_ID(fd: c_int, arg: *mut u64) {
    unsafe {
        let _ = perf_event_open_sys::ioctls::ID(fd, arg);
    };
}

fn call_ioctls_SET_BPF(fd: c_int, arg: u32) {
    unsafe {
        let _ = perf_event_open_sys::ioctls::SET_BPF(fd, arg);
    };
}

fn call_ioctls_PAUSE_OUTPUT(fd: c_int, arg: u32) {
    unsafe {
        let _ = perf_event_open_sys::ioctls::PAUSE_OUTPUT(fd, arg);
    };
}

fn call_ioctls_QUERY_BPF(fd: c_int, arg: *mut perf_event_query_bpf) {
    unsafe {
        let _ = perf_event_open_sys::ioctls::QUERY_BPF(fd, arg);
    };
}

fn call_ioctls_MODIFY_ATTRIBUTES(fd: c_int, arg: *mut perf_event_attr) {
    unsafe {
        let _ = perf_event_open_sys::ioctls::MODIFY_ATTRIBUTES(fd, arg);
    };
}
