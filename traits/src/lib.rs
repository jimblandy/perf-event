#![doc = include_str!("../README.md")]

/// A marker trait implemented by all versions of the [`perf_event_attr`] type.
///
/// Each version of the [`perf-event-open-sys`] crate implements this trait for
/// its definition of the [`perf_event_attr`] struct, even as successive major
/// versions make semver-incompatible changes to the type.
///
/// The [`perf-event`] crate uses this trait to handle `perf_event_attr` values
/// from any version of the [`perf-event-open-sys`] raw bindings crate, without
/// ever naming such types directly in its API. This means that any given
/// version of `perf-event` (0.5.0 or later) can be used with any major version
/// of `perf-event-open-sys`.
///
/// See [the crate documentation][crate] for details.
///
/// # Safety
///
/// A type may implement this trait if it meets the following conditions:
///
/// - The type is `#[repr(C)]`.
///
/// - The type's layout matches that of the Linux kernel's [`perf_event_attr`]
///   type at some Linux kernel release.
///
/// - The type contains no padding bytes, and every union member fills the
///   entire union.
///
/// - Any series of bytes is a valid value of the type.
///
/// The third and fourth bullet points are true of all known Linux versions'
/// `perf_event_attr` types.
///
/// [`perf_event_attr`]: https://man7.org/linux/man-pages/man2/perf_event_open.2.html
/// [`perf-event-open-sys`]: https://crates.io/crates/perf-event-open-sys
/// [`perf-event`]: https://crates.io/crates/perf-event
/// [`Builder::attrs`]: https://docs.rs/perf-event/latest/perf_event/struct.Builder.html#method.attrs
/// [`Builder::set_attrs`]: https://docs.rs/perf-event/latest/perf_event/struct.Builder.html#method.set_attrs
pub unsafe trait PerfEventAttrType: Copy {
    /// Return the value of `self`'s `size` field.
    fn size(&self) -> usize;

    /// Set `self`'s `size` field to `size`.
    ///
    /// # Panics
    ///
    /// Panic if `size` does not fit in `self`'s `size` field.
    fn set_size(&mut self, size: usize);
}
