//! Types and constants used with `perf_event_open`.
//!
//! This module contains types and constants for use with the
//! [`perf_event_open`][man] system call. These are automatically generated from
//! the header files `<linux/perf_event.h>` and `<linux/hw_breakpoint.h>` by the
//! Rust [`bindgen`][bindgen] tool.
//!
//! It's not always obvious how `bindgen` will choose to reflect a given C
//! construct into Rust. The best approach I've found is simply to search
//! [the source code][src] for the C identifier name and see what `bindgen` did
//! with it.
//!
//! [man]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
//! [bindgen]: https://github.com/rust-lang/rust-bindgen
//! [src]: ../../src/perf_event_open_sys/bindings.rs.html

#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)] // `bindgen_test_layout` tests use bogus code
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::useless_transmute)]

