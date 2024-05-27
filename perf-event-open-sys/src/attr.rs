//! This module contains a chain of deref structs which contain fields from
//! the unnamed unions within the [`perf_event_attr`] struct. This deref chain
//! allows users to access the relevant inline union fields directly, the same
//! as they would in C. By extension, that allows certain changes which would
//! be non-breaking in C but breaking changes in bindgen-generated Rust to
//! instead be non-breaking.
//!
//! The way it works is that we have several structs which manually place
//! fields at the appropriate offset. [`perf_event_attr`] derefs to one and
//! the chain continues on from there. All derefs are done by pointer casts
//! on `self` so the whole sequence should compile down to a no-op.
//!
//! Note that there is a limitation on how many deref impls rust-analyzer is
//! willing to traverse for autocompletion. Empirically, that limit seems to
//! be 9. Multiple fields are batched together here to ensure that the whole
//! chain of deref impls remains visible to autocomplete.

#![allow(non_camel_case_types)]

use std::mem::{self, MaybeUninit};
use std::ops::{Deref, DerefMut};

use memoffset::offset_of;

use crate::bindings::{self, perf_event_attr};

#[repr(C)]
pub struct AttrDeref1 {
    _pad1: [MaybeUninit<u8>; Layout1::PAD1],
    pub sample_period: u64,
    _pad2: [MaybeUninit<u8>; Layout1::PAD2],
    pub wakeup_events: u32,
    _pad3: [MaybeUninit<u8>; Layout1::PAD3],
    pub bp_addr: u64,
    _pad4: [MaybeUninit<u8>; Layout1::PAD4],
    pub bp_len: u64,
    _pad5: [MaybeUninit<u8>; Layout1::PAD5],
}

#[repr(C)]
pub struct AttrDeref2 {
    _pad1: [MaybeUninit<u8>; Layout1::PAD1],
    pub sample_freq: u64,
    _pad2: [MaybeUninit<u8>; Layout1::PAD2],
    pub wakeup_watermark: u32,
    _pad3: [MaybeUninit<u8>; Layout1::PAD3],
    pub kprobe_func: u64,
    _pad4: [MaybeUninit<u8>; Layout1::PAD4],
    pub kprobe_addr: u64,
    _pad5: [MaybeUninit<u8>; Layout1::PAD5],
}

#[repr(C)]
pub struct AttrDeref3 {
    _pad1: [MaybeUninit<u8>; Layout2::PAD1],
    pub uprobe_path: u64,
    _pad2: [MaybeUninit<u8>; Layout2::PAD2],
    pub probe_offset: u64,
    _pad3: [MaybeUninit<u8>; Layout2::PAD3],
}

#[repr(C)]
pub struct AttrDeref4 {
    _pad1: [MaybeUninit<u8>; Layout2::PAD1],
    pub config1: u64,
    _pad2: [MaybeUninit<u8>; Layout2::PAD2],
    pub config2: u64,
    _pad3: [MaybeUninit<u8>; Layout2::PAD3],
}

macro_rules! deref_cast {
    ($source:ident => $target:ident) => {
        impl Deref for $source {
            type Target = $target;

            #[inline]
            fn deref(&self) -> &Self::Target {
                const _: () = {
                    assert!(mem::size_of::<$source>() == mem::size_of::<$target>());
                };

                unsafe { &*(self as *const Self as *const Self::Target) }
            }
        }

        impl DerefMut for $source {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *(self as *mut Self as *mut Self::Target) }
            }
        }
    };
}

deref_cast!(perf_event_attr => AttrDeref1);
deref_cast!(AttrDeref1 => AttrDeref2);
deref_cast!(AttrDeref2 => AttrDeref3);
deref_cast!(AttrDeref3 => AttrDeref4);

enum Offsets {}

impl Offsets {
    const OFF1: usize = offset_of!(perf_event_attr, __bindgen_anon_1);
    const OFF2: usize = offset_of!(perf_event_attr, __bindgen_anon_2);
    const OFF3: usize = offset_of!(perf_event_attr, __bindgen_anon_3);
    const OFF4: usize = offset_of!(perf_event_attr, __bindgen_anon_4);

    const END1: usize = Self::OFF1 + mem::size_of::<bindings::perf_event_attr__bindgen_ty_1>();
    const END2: usize = Self::OFF2 + mem::size_of::<bindings::perf_event_attr__bindgen_ty_2>();
    const END3: usize = Self::OFF3 + mem::size_of::<bindings::perf_event_attr__bindgen_ty_3>();
    const END4: usize = Self::OFF4 + mem::size_of::<bindings::perf_event_attr__bindgen_ty_4>();
}

/// Layout with fields from all 4 unions
enum Layout1 {}

impl Layout1 {
    const PAD1: usize = Offsets::OFF1;
    const PAD2: usize = Offsets::OFF2 - Offsets::END1;
    const PAD3: usize = Offsets::OFF3 - Offsets::END2;
    const PAD4: usize = Offsets::OFF4 - Offsets::END3;
    const PAD5: usize = mem::size_of::<perf_event_attr>() - Offsets::END4;
}

/// Layout with fields from only the last 2 unions
enum Layout2 {}

impl Layout2 {
    const PAD1: usize = Offsets::OFF3;
    const PAD2: usize = Offsets::OFF4 - Offsets::END3;
    const PAD3: usize = mem::size_of::<perf_event_attr>() - Offsets::END4;
}
