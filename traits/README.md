# Compatibility traits for Linux kernel perf structs

This crate defines marker traits that allow the [`perf-event`] crate to preserve
semver compatibility even as new versions of the [`perf-event-open-sys`] crate
are published. Usually, you should not need to use this crate directly.

Each version of `perf-event-open-sys` has its version of the [`perf_event_attr`]
type implement this crate's [`PerfEventAttrType`] trait. Then, in the
`perf_event` crate, [`Builder::attrs`] and [`Builder::set_attrs`] accept any
type that implements `PerfEventAttrType`. This way, `perf_event`'s public API
does not directly mention any types from `perf-event-open-sys`, only the trait
which they all implement.

There will (hopefully) only ever be one major version of this crate: version 1.

## Background

The [`perf-event`] crate offers a high-level interface to the Linux
[`perf_event_open`] API, for gathering performance and debugging information
from the kernel and processor. The `perf-event` crate is built on unsafe Rust
bindings for the underlying system calls provided by the [`perf-event-open-sys`]
crate, which generates them from the Linux kernel headers using [`bindgen`].

From time to time, new versions of Linux add functionality to the
`perf_event_open` API, requiring a new version of [`perf-event-open-sys`]. If
new fields or bitfields are added to the the API's struct types
([`perf_event_attr`], mainly), then the new version of the crate cannot be
semver-compatible with the old one, so it gets a new major version number.

Although [`perf-event`] is meant to be safe and high-level, its [`Builder`] type
does offer [`attrs`] and [`set_attrs`] functions, which get and set the
`Builder`'s internal state as a raw `perf_event_attr` struct. If these functions
were to use the `sys` crate's type directly in their argument and return value
types, that would force us to release a new major version of `perf-event` for
every new major version of `perf-event-open-sys`. But a high-level API would
ideally avoid that kind of coupling with the low-level bindings.

Instead, [`Builder::attrs`] and [`Builder::set_attrs`] simply accept any type
that implements the [`PerfEventAttrType`] trait, allowing them to work with any
major version of `perf-event-open-sys`.

To do this, `perf-event` takes advantage of the Linux kernel's backward
compatibility rules:

-   System calls that pass a struct always include some indication of how large
    the struct is.

-   Struct fields are never moved or deleted.

-   New fields are only added at the end of structs.

-   New fields' semantics are chosen such that filling them with zeros
    preserves the old behavior.

These are the rules followed by the Linux kernel's `copy_struct_from_user`
function. Taken together, they imply that turning an old struct into a new
struct by extending it with zero bytes should always preserve its meaning. And
if a new struct's extra bytes are zero, then it can be truncated into an old
struct, again preserving its meaning.

So when the `perf-event` crate is presented with a `perf_event_attr` struct that
is smaller than the one it was built against, it zero-extends it; and when given
a struct that is larger, it checks that the excess bytes are all zero, panicking
if they are not. This ensures that `perf-event` and its caller always agree on
what the structs mean. For details, see the documentation for [`Builder::attrs`]
and [`Builder::set_attrs`].

[`perf-event`]: https://crates.io/crates/perf-event
[`perf_event_open`]: https://man7.org/linux/man-pages/man2/perf_event_open.2.html
[`perf-event-open-sys`]: https://crates.io/crates/perf-event-open-sys
[`bindgen`]: https://crates.io/crates/bindgen
[`perf_event_attr`]: https://docs.rs/perf-event-open-sys/latest/perf_event_open_sys/bindings/struct.perf_event_attr.html
[`Builder`]: https://docs.rs/perf-event/latest/perf_event/struct.Builder.html
[`attrs`]: https://docs.rs/perf-event/latest/perf_event/struct.Builder.html#method.attrs
[`set_attrs`]: https://docs.rs/perf-event/latest/perf_event/struct.Builder.html#method.set_attrs
[`Builder::attrs`]: https://docs.rs/perf-event/latest/perf_event/struct.Builder.html#method.attrs
[`Builder::set_attrs`]: https://docs.rs/perf-event/latest/perf_event/struct.Builder.html#method.set_attrs
