## perf-event: a Rust interface to Linux performance monitoring

![example workflow](https://github.com/jimblandy/perf-event/actions/workflows/master.yml/badge.svg)

*This is a nascent project. Tests are lacking. The design may change.*

This repository holds the source code for the [`perf_event`][pe] and
[`perf_event_open_sys`][peos] crates, which provide access to
performance monitoring hardware and software on Linux.

Even though Windows and Mac don't have the `perf_event_open` system
call, the `perf_event_open_sys` crate still builds on those platforms:
the type definitions in the `bindings` module can be useful to code
that needs to parse perf-related data produced on Linux or Android
systems. The syscall and ioctl wrapper functions are not available.

See the crates' subdirectories for details.

[pe]: https://crates.io/crates/perf-event
[peos]: https://crates.io/crates/perf-event-open-sys
