/*! Converting between different versions of `perf_event_attr`.

This module defines the [`checked_convert_attrs`] function.

*/

use perf_event_open_sys_traits::PerfEventAttrType;
use std::mem;

/// Convert a [`perf_event_attr`] from one version to another.
///
/// Convert a value whose type implements [`PerfEventAttrType`] to another such
/// type, using the Linux kernel's compatibility rules: fill any extra bytes in
/// the destination with zeros, and panic if there are any nonzero bytes in the
/// source that the destination has no room for.
///
/// This ensures that, if the conversion succeeds, the two structs have the same
/// meaning. For details, see the [`perf-event-open-sys-traits`] crate
/// documentation.
///
/// The size specified in `from`'s [`size`] field is ignored. The size
/// of the return value is always set to the proper size for its type.
///
/// [`perf_event_attr`]: perf_event_open_sys::perf_event_attr
/// [`PerfEventAttrType`]: perf_event_open_sys_traits::PerfEventAttrType
/// [`size`]: field@perf_event_open_sys::bindings::perf_event_attr::size
/// [`perf-event-open-sys-traits`]: perf_event_open_sys_traits
#[track_caller]
pub fn checked_convert_attrs<From, To>(from: From) -> To
where
    From: PerfEventAttrType,
    To: PerfEventAttrType,
{
    let from_size = mem::size_of::<From>();
    let to_size = mem::size_of::<To>();

    #[repr(C)]
    union Rendezvous<From: Copy, To: Copy> {
        from: From,
        to: To,
    }

    // Safety: Every field of `Rendezvous` implements `PerfEventAttrType`, which
    // promises that any byte pattern is a valid representation of `Self`.
    let mut rendezvous: Rendezvous<From, To> = unsafe { mem::zeroed() };

    rendezvous.from = from;

    // If `From` is larger than `To`, ensure the excess bytes are all zero,
    // implying that dropping them won't lose anything meaningful.
    if from_size > to_size {
        // Get a view of `rendezvous.from` as plain bytes.
        //
        // Safety: the slice covers only bytes in `rendezvous`, which has been
        // fully initialized.
        let bytes_of_from = unsafe { as_byte_slice(&rendezvous.from) };

        // Check for non-zero bytes that would be lost if the value were
        // truncated to `To`.
        if let Some(offset_within_tail) =
            bytes_of_from[to_size..].iter().position(|&byte| byte != 0)
        {
            let offset = to_size + offset_within_tail;
            let from_size_name = attr_size_name(from_size);
            let to_size_name = attr_size_name(to_size);

            panic!(
                "Truncating a `perf_event_attr` value with a size of {} bytes ({}) \
                 with a non-zero byte at offset {} \
                 to {} bytes ({}) would lose information",
                from_size, from_size_name, offset, to_size, to_size_name,
            );
        }
    }

    // Safety:
    // - We initialized `rendezvous` with zeros, so `to` is fully initialized.
    // - Since both types implement [`PerfEventAttrType`], we know
    //   that any block of bytes is a valid value.
    let mut to = unsafe { rendezvous.to };

    // At this point, `to`'s size was just copied from `from`. Adjust it to
    // reflect its new type.
    to.set_size(to_size);

    to
}

/// Return a byte slice covering all the bytes of `*value`.
///
/// # Safety
///
/// All bytes of `*value` must be initialized, including padding
/// bytes, as they will all be accessible via the returned slice.
unsafe fn as_byte_slice<T>(value: &T) -> &[u8] {
    std::slice::from_raw_parts(value as *const _ as *const u8, mem::size_of::<T>())
}

fn attr_size_name(size: usize) -> &'static str {
    use std::convert::TryFrom;
    let Ok(size) = u32::try_from(size) else {
        return "<unreasonably large size>";
    };
    match size {
        perf_event_open_sys::bindings::PERF_ATTR_SIZE_VER0 => "`PERF_ATTR_SIZE_VER0`",
        perf_event_open_sys::bindings::PERF_ATTR_SIZE_VER1 => "`PERF_ATTR_SIZE_VER1`",
        perf_event_open_sys::bindings::PERF_ATTR_SIZE_VER2 => "`PERF_ATTR_SIZE_VER2`",
        perf_event_open_sys::bindings::PERF_ATTR_SIZE_VER3 => "`PERF_ATTR_SIZE_VER3`",
        perf_event_open_sys::bindings::PERF_ATTR_SIZE_VER4 => "`PERF_ATTR_SIZE_VER4`",
        perf_event_open_sys::bindings::PERF_ATTR_SIZE_VER5 => "`PERF_ATTR_SIZE_VER5`",
        perf_event_open_sys::bindings::PERF_ATTR_SIZE_VER6 => "`PERF_ATTR_SIZE_VER6`",
        perf_event_open_sys::bindings::PERF_ATTR_SIZE_VER7 => "`PERF_ATTR_SIZE_VER7`",
        perf_event_open_sys::bindings::PERF_ATTR_SIZE_VER8 => "`PERF_ATTR_SIZE_VER8`",
        perf_event_open_sys::bindings::PERF_ATTR_SIZE_VER9 => "`PERF_ATTR_SIZE_VER9`",
        _ => "<unrecognized size for `perf_event_attr`>",
    }
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use super::checked_convert_attrs;
    use perf_event_open_sys::bindings::{perf_event_attr, PERF_ATTR_SIZE_VER0};
    use perf_event_open_sys_traits::PerfEventAttrType;
    use std::mem;

    /// A `perf_event_attr` from a hypothetical older kernel, with only the
    /// `PERF_ATTR_SIZE_VER0` fields we need, and the rest as raw bytes.
    #[repr(C)]
    #[derive(Copy, Clone)]
    struct Older {
        type_: u32,
        size: u32,
        config: u64,
        rest: [u8; 48],
    }

    unsafe impl PerfEventAttrType for Older {
        fn size(&self) -> usize {
            self.size as usize
        }

        fn set_size(&mut self, size: usize) {
            self.size = size as u32;
        }
    }

    /// A `perf_event_attr` from a hypothetical newer kernel, with one extra
    /// field added at the end.
    #[repr(C)]
    #[derive(Copy, Clone)]
    struct Newer {
        attr: perf_event_attr,
        new_field: u64,
    }

    unsafe impl PerfEventAttrType for Newer {
        fn size(&self) -> usize {
            self.attr.size()
        }

        fn set_size(&mut self, size: usize) {
            self.attr.set_size(size);
        }
    }

    fn older() -> Older {
        Older {
            type_: 0,
            size: 0,
            config: 0,
            rest: [0; 48],
        }
    }

    fn newer() -> Newer {
        Newer {
            attr: perf_event_attr::default(),
            new_field: 0,
        }
    }

    #[test]
    fn older_is_ver0() {
        assert_eq!(mem::size_of::<Older>(), PERF_ATTR_SIZE_VER0 as usize);
    }

    #[test]
    fn same_type() {
        let mut from = perf_event_attr::default();
        from.type_ = 4;
        from.config = 0x1234;
        from.sample_regs_user = 0xff;
        from.set_disabled(1);

        let to: perf_event_attr = checked_convert_attrs(from);
        assert_eq!(to.type_, 4);
        assert_eq!(to.config, 0x1234);
        assert_eq!(to.sample_regs_user, 0xff);
        assert_eq!(to.disabled(), 1);
        assert_eq!(to.size as usize, mem::size_of::<perf_event_attr>());
    }

    #[test]
    fn zero_extend() {
        let mut from = older();
        from.type_ = 4;
        from.config = 0x1234;
        from.rest = [0xff; 48];

        let to: Newer = checked_convert_attrs(from);
        assert_eq!(to.attr.type_, 4);
        assert_eq!(to.attr.config, 0x1234);
        assert_eq!(to.attr.size as usize, mem::size_of::<Newer>());
        // `rest` covers offsets 16..64, ending with `bp_type` and `config1`.
        assert_eq!(to.attr.bp_type, 0xffff_ffff);
        assert_eq!(to.attr.config1, u64::MAX);
        // Everything past `PERF_ATTR_SIZE_VER0` must be zero.
        assert_eq!(to.attr.branch_sample_type, 0);
        assert_eq!(to.attr.sample_regs_user, 0);
        assert_eq!(to.attr.sig_data, 0);
        assert_eq!(to.new_field, 0);
    }

    #[test]
    fn truncate_zero_tail() {
        let mut from = newer();
        from.attr.type_ = 4;
        from.attr.config = 0x1234;
        from.attr.set_exclude_kernel(1);

        let to: perf_event_attr = checked_convert_attrs(from);
        assert_eq!(to.type_, 4);
        assert_eq!(to.config, 0x1234);
        assert_eq!(to.exclude_kernel(), 1);
        assert_eq!(to.size as usize, mem::size_of::<perf_event_attr>());

        let to: Older = checked_convert_attrs(from);
        assert_eq!(to.type_, 4);
        assert_eq!(to.config, 0x1234);
        assert_eq!(to.size, PERF_ATTR_SIZE_VER0);
    }

    #[test]
    #[should_panic(expected = "would lose information")]
    fn truncate_nonzero_tail() {
        let mut from = newer();
        from.attr.size = mem::size_of::<Newer>() as u32;
        from.new_field = 1;
        let _: perf_event_attr = checked_convert_attrs(from);
    }

    #[test]
    #[should_panic(expected = "with a non-zero byte at offset 80 to 64 bytes")]
    fn truncate_nonzero_tail_to_ver0() {
        let mut from = perf_event_attr::default();
        from.sample_regs_user = 1; // at offset 80
        let _: Older = checked_convert_attrs(from);
    }

    /// The `size` field doesn't affect the truncation check. In particular, a
    /// `size` of zero, as left by `Default`, doesn't disable it.
    #[test]
    #[should_panic(expected = "would lose information")]
    fn truncate_nonzero_tail_ignores_size() {
        let mut from = newer();
        assert_eq!(from.attr.size, 0);
        from.new_field = 1;
        let _: perf_event_attr = checked_convert_attrs(from);
    }

    /// Bytes beyond the `size` field are still copied.
    #[test]
    fn copy_ignores_size() {
        let mut from = perf_event_attr::default();
        from.size = PERF_ATTR_SIZE_VER0;
        from.sample_regs_user = 0xff; // past `PERF_ATTR_SIZE_VER0`

        let to: Newer = checked_convert_attrs(from);
        assert_eq!(to.attr.sample_regs_user, 0xff);
        assert_eq!(to.attr.size as usize, mem::size_of::<Newer>());
    }
}
