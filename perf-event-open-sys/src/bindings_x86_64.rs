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
#![allow(clippy::all)]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    fn extract_bit(byte: u8, index: usize) -> bool {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        Self::extract_bit(byte, index)
    }
    #[inline]
    pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = *(core::ptr::addr_of!((*this).storage) as *const u8).offset(byte_index as isize);
        Self::extract_bit(byte, index)
    }
    #[inline]
    fn change_bit(byte: u8, index: usize, val: bool) -> u8 {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            byte | mask
        } else {
            byte & !mask
        }
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte =
            (core::ptr::addr_of_mut!((*this).storage) as *mut u8).offset(byte_index as isize);
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if Self::raw_get_bit(this, i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            Self::raw_set_bit(this, index + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
pub const PERF_PMU_TYPE_SHIFT: u32 = 32;
pub const PERF_HW_EVENT_MASK: u32 = 4294967295;
pub const PERF_ATTR_SIZE_VER0: u32 = 64;
pub const PERF_ATTR_SIZE_VER1: u32 = 72;
pub const PERF_ATTR_SIZE_VER2: u32 = 80;
pub const PERF_ATTR_SIZE_VER3: u32 = 96;
pub const PERF_ATTR_SIZE_VER4: u32 = 104;
pub const PERF_ATTR_SIZE_VER5: u32 = 112;
pub const PERF_ATTR_SIZE_VER6: u32 = 120;
pub const PERF_ATTR_SIZE_VER7: u32 = 128;
pub const PERF_ATTR_SIZE_VER8: u32 = 136;
pub const PERF_RECORD_MISC_CPUMODE_MASK: u32 = 7;
pub const PERF_RECORD_MISC_CPUMODE_UNKNOWN: u32 = 0;
pub const PERF_RECORD_MISC_KERNEL: u32 = 1;
pub const PERF_RECORD_MISC_USER: u32 = 2;
pub const PERF_RECORD_MISC_HYPERVISOR: u32 = 3;
pub const PERF_RECORD_MISC_GUEST_KERNEL: u32 = 4;
pub const PERF_RECORD_MISC_GUEST_USER: u32 = 5;
pub const PERF_RECORD_MISC_PROC_MAP_PARSE_TIMEOUT: u32 = 4096;
pub const PERF_RECORD_MISC_MMAP_DATA: u32 = 8192;
pub const PERF_RECORD_MISC_COMM_EXEC: u32 = 8192;
pub const PERF_RECORD_MISC_FORK_EXEC: u32 = 8192;
pub const PERF_RECORD_MISC_SWITCH_OUT: u32 = 8192;
pub const PERF_RECORD_MISC_EXACT_IP: u32 = 16384;
pub const PERF_RECORD_MISC_SWITCH_OUT_PREEMPT: u32 = 16384;
pub const PERF_RECORD_MISC_MMAP_BUILD_ID: u32 = 16384;
pub const PERF_RECORD_MISC_EXT_RESERVED: u32 = 32768;
pub const PERF_RECORD_KSYMBOL_FLAGS_UNREGISTER: u32 = 1;
pub const PERF_MAX_STACK_DEPTH: u32 = 127;
pub const PERF_MAX_CONTEXTS_PER_STACK: u32 = 8;
pub const PERF_AUX_FLAG_TRUNCATED: u32 = 1;
pub const PERF_AUX_FLAG_OVERWRITE: u32 = 2;
pub const PERF_AUX_FLAG_PARTIAL: u32 = 4;
pub const PERF_AUX_FLAG_COLLISION: u32 = 8;
pub const PERF_AUX_FLAG_PMU_FORMAT_TYPE_MASK: u32 = 65280;
pub const PERF_AUX_FLAG_CORESIGHT_FORMAT_CORESIGHT: u32 = 0;
pub const PERF_AUX_FLAG_CORESIGHT_FORMAT_RAW: u32 = 256;
pub const PERF_FLAG_FD_NO_GROUP: u32 = 1;
pub const PERF_FLAG_FD_OUTPUT: u32 = 2;
pub const PERF_FLAG_PID_CGROUP: u32 = 4;
pub const PERF_FLAG_FD_CLOEXEC: u32 = 8;
pub const PERF_MEM_OP_NA: u32 = 1;
pub const PERF_MEM_OP_LOAD: u32 = 2;
pub const PERF_MEM_OP_STORE: u32 = 4;
pub const PERF_MEM_OP_PFETCH: u32 = 8;
pub const PERF_MEM_OP_EXEC: u32 = 16;
pub const PERF_MEM_OP_SHIFT: u32 = 0;
pub const PERF_MEM_LVL_NA: u32 = 1;
pub const PERF_MEM_LVL_HIT: u32 = 2;
pub const PERF_MEM_LVL_MISS: u32 = 4;
pub const PERF_MEM_LVL_L1: u32 = 8;
pub const PERF_MEM_LVL_LFB: u32 = 16;
pub const PERF_MEM_LVL_L2: u32 = 32;
pub const PERF_MEM_LVL_L3: u32 = 64;
pub const PERF_MEM_LVL_LOC_RAM: u32 = 128;
pub const PERF_MEM_LVL_REM_RAM1: u32 = 256;
pub const PERF_MEM_LVL_REM_RAM2: u32 = 512;
pub const PERF_MEM_LVL_REM_CCE1: u32 = 1024;
pub const PERF_MEM_LVL_REM_CCE2: u32 = 2048;
pub const PERF_MEM_LVL_IO: u32 = 4096;
pub const PERF_MEM_LVL_UNC: u32 = 8192;
pub const PERF_MEM_LVL_SHIFT: u32 = 5;
pub const PERF_MEM_REMOTE_REMOTE: u32 = 1;
pub const PERF_MEM_REMOTE_SHIFT: u32 = 37;
pub const PERF_MEM_LVLNUM_L1: u32 = 1;
pub const PERF_MEM_LVLNUM_L2: u32 = 2;
pub const PERF_MEM_LVLNUM_L3: u32 = 3;
pub const PERF_MEM_LVLNUM_L4: u32 = 4;
pub const PERF_MEM_LVLNUM_L2_MHB: u32 = 5;
pub const PERF_MEM_LVLNUM_MSC: u32 = 6;
pub const PERF_MEM_LVLNUM_UNC: u32 = 8;
pub const PERF_MEM_LVLNUM_CXL: u32 = 9;
pub const PERF_MEM_LVLNUM_IO: u32 = 10;
pub const PERF_MEM_LVLNUM_ANY_CACHE: u32 = 11;
pub const PERF_MEM_LVLNUM_LFB: u32 = 12;
pub const PERF_MEM_LVLNUM_RAM: u32 = 13;
pub const PERF_MEM_LVLNUM_PMEM: u32 = 14;
pub const PERF_MEM_LVLNUM_NA: u32 = 15;
pub const PERF_MEM_LVLNUM_SHIFT: u32 = 33;
pub const PERF_MEM_SNOOP_NA: u32 = 1;
pub const PERF_MEM_SNOOP_NONE: u32 = 2;
pub const PERF_MEM_SNOOP_HIT: u32 = 4;
pub const PERF_MEM_SNOOP_MISS: u32 = 8;
pub const PERF_MEM_SNOOP_HITM: u32 = 16;
pub const PERF_MEM_SNOOP_SHIFT: u32 = 19;
pub const PERF_MEM_SNOOPX_FWD: u32 = 1;
pub const PERF_MEM_SNOOPX_PEER: u32 = 2;
pub const PERF_MEM_SNOOPX_SHIFT: u32 = 38;
pub const PERF_MEM_LOCK_NA: u32 = 1;
pub const PERF_MEM_LOCK_LOCKED: u32 = 2;
pub const PERF_MEM_LOCK_SHIFT: u32 = 24;
pub const PERF_MEM_TLB_NA: u32 = 1;
pub const PERF_MEM_TLB_HIT: u32 = 2;
pub const PERF_MEM_TLB_MISS: u32 = 4;
pub const PERF_MEM_TLB_L1: u32 = 8;
pub const PERF_MEM_TLB_L2: u32 = 16;
pub const PERF_MEM_TLB_WK: u32 = 32;
pub const PERF_MEM_TLB_OS: u32 = 64;
pub const PERF_MEM_TLB_SHIFT: u32 = 26;
pub const PERF_MEM_BLK_NA: u32 = 1;
pub const PERF_MEM_BLK_DATA: u32 = 2;
pub const PERF_MEM_BLK_ADDR: u32 = 4;
pub const PERF_MEM_BLK_SHIFT: u32 = 40;
pub const PERF_MEM_HOPS_0: u32 = 1;
pub const PERF_MEM_HOPS_1: u32 = 2;
pub const PERF_MEM_HOPS_2: u32 = 3;
pub const PERF_MEM_HOPS_3: u32 = 4;
pub const PERF_MEM_HOPS_SHIFT: u32 = 43;
pub const PERF_BRANCH_ENTRY_INFO_BITS_MAX: u32 = 33;
pub const __NR_perf_event_open: u32 = 298;
pub type __u8 = ::std::os::raw::c_uchar;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __s32 = ::std::os::raw::c_int;
pub type __u32 = ::std::os::raw::c_uint;
pub type __s64 = ::std::os::raw::c_longlong;
pub type __u64 = ::std::os::raw::c_ulonglong;
pub const PERF_TYPE_HARDWARE: perf_type_id = 0;
pub const PERF_TYPE_SOFTWARE: perf_type_id = 1;
pub const PERF_TYPE_TRACEPOINT: perf_type_id = 2;
pub const PERF_TYPE_HW_CACHE: perf_type_id = 3;
pub const PERF_TYPE_RAW: perf_type_id = 4;
pub const PERF_TYPE_BREAKPOINT: perf_type_id = 5;
pub const PERF_TYPE_MAX: perf_type_id = 6;
pub type perf_type_id = ::std::os::raw::c_uint;
pub const PERF_COUNT_HW_CPU_CYCLES: perf_hw_id = 0;
pub const PERF_COUNT_HW_INSTRUCTIONS: perf_hw_id = 1;
pub const PERF_COUNT_HW_CACHE_REFERENCES: perf_hw_id = 2;
pub const PERF_COUNT_HW_CACHE_MISSES: perf_hw_id = 3;
pub const PERF_COUNT_HW_BRANCH_INSTRUCTIONS: perf_hw_id = 4;
pub const PERF_COUNT_HW_BRANCH_MISSES: perf_hw_id = 5;
pub const PERF_COUNT_HW_BUS_CYCLES: perf_hw_id = 6;
pub const PERF_COUNT_HW_STALLED_CYCLES_FRONTEND: perf_hw_id = 7;
pub const PERF_COUNT_HW_STALLED_CYCLES_BACKEND: perf_hw_id = 8;
pub const PERF_COUNT_HW_REF_CPU_CYCLES: perf_hw_id = 9;
pub const PERF_COUNT_HW_MAX: perf_hw_id = 10;
pub type perf_hw_id = ::std::os::raw::c_uint;
pub const PERF_COUNT_HW_CACHE_L1D: perf_hw_cache_id = 0;
pub const PERF_COUNT_HW_CACHE_L1I: perf_hw_cache_id = 1;
pub const PERF_COUNT_HW_CACHE_LL: perf_hw_cache_id = 2;
pub const PERF_COUNT_HW_CACHE_DTLB: perf_hw_cache_id = 3;
pub const PERF_COUNT_HW_CACHE_ITLB: perf_hw_cache_id = 4;
pub const PERF_COUNT_HW_CACHE_BPU: perf_hw_cache_id = 5;
pub const PERF_COUNT_HW_CACHE_NODE: perf_hw_cache_id = 6;
pub const PERF_COUNT_HW_CACHE_MAX: perf_hw_cache_id = 7;
pub type perf_hw_cache_id = ::std::os::raw::c_uint;
pub const PERF_COUNT_HW_CACHE_OP_READ: perf_hw_cache_op_id = 0;
pub const PERF_COUNT_HW_CACHE_OP_WRITE: perf_hw_cache_op_id = 1;
pub const PERF_COUNT_HW_CACHE_OP_PREFETCH: perf_hw_cache_op_id = 2;
pub const PERF_COUNT_HW_CACHE_OP_MAX: perf_hw_cache_op_id = 3;
pub type perf_hw_cache_op_id = ::std::os::raw::c_uint;
pub const PERF_COUNT_HW_CACHE_RESULT_ACCESS: perf_hw_cache_op_result_id = 0;
pub const PERF_COUNT_HW_CACHE_RESULT_MISS: perf_hw_cache_op_result_id = 1;
pub const PERF_COUNT_HW_CACHE_RESULT_MAX: perf_hw_cache_op_result_id = 2;
pub type perf_hw_cache_op_result_id = ::std::os::raw::c_uint;
pub const PERF_COUNT_SW_CPU_CLOCK: perf_sw_ids = 0;
pub const PERF_COUNT_SW_TASK_CLOCK: perf_sw_ids = 1;
pub const PERF_COUNT_SW_PAGE_FAULTS: perf_sw_ids = 2;
pub const PERF_COUNT_SW_CONTEXT_SWITCHES: perf_sw_ids = 3;
pub const PERF_COUNT_SW_CPU_MIGRATIONS: perf_sw_ids = 4;
pub const PERF_COUNT_SW_PAGE_FAULTS_MIN: perf_sw_ids = 5;
pub const PERF_COUNT_SW_PAGE_FAULTS_MAJ: perf_sw_ids = 6;
pub const PERF_COUNT_SW_ALIGNMENT_FAULTS: perf_sw_ids = 7;
pub const PERF_COUNT_SW_EMULATION_FAULTS: perf_sw_ids = 8;
pub const PERF_COUNT_SW_DUMMY: perf_sw_ids = 9;
pub const PERF_COUNT_SW_BPF_OUTPUT: perf_sw_ids = 10;
pub const PERF_COUNT_SW_CGROUP_SWITCHES: perf_sw_ids = 11;
pub const PERF_COUNT_SW_MAX: perf_sw_ids = 12;
pub type perf_sw_ids = ::std::os::raw::c_uint;
pub const PERF_SAMPLE_IP: perf_event_sample_format = 1;
pub const PERF_SAMPLE_TID: perf_event_sample_format = 2;
pub const PERF_SAMPLE_TIME: perf_event_sample_format = 4;
pub const PERF_SAMPLE_ADDR: perf_event_sample_format = 8;
pub const PERF_SAMPLE_READ: perf_event_sample_format = 16;
pub const PERF_SAMPLE_CALLCHAIN: perf_event_sample_format = 32;
pub const PERF_SAMPLE_ID: perf_event_sample_format = 64;
pub const PERF_SAMPLE_CPU: perf_event_sample_format = 128;
pub const PERF_SAMPLE_PERIOD: perf_event_sample_format = 256;
pub const PERF_SAMPLE_STREAM_ID: perf_event_sample_format = 512;
pub const PERF_SAMPLE_RAW: perf_event_sample_format = 1024;
pub const PERF_SAMPLE_BRANCH_STACK: perf_event_sample_format = 2048;
pub const PERF_SAMPLE_REGS_USER: perf_event_sample_format = 4096;
pub const PERF_SAMPLE_STACK_USER: perf_event_sample_format = 8192;
pub const PERF_SAMPLE_WEIGHT: perf_event_sample_format = 16384;
pub const PERF_SAMPLE_DATA_SRC: perf_event_sample_format = 32768;
pub const PERF_SAMPLE_IDENTIFIER: perf_event_sample_format = 65536;
pub const PERF_SAMPLE_TRANSACTION: perf_event_sample_format = 131072;
pub const PERF_SAMPLE_REGS_INTR: perf_event_sample_format = 262144;
pub const PERF_SAMPLE_PHYS_ADDR: perf_event_sample_format = 524288;
pub const PERF_SAMPLE_AUX: perf_event_sample_format = 1048576;
pub const PERF_SAMPLE_CGROUP: perf_event_sample_format = 2097152;
pub const PERF_SAMPLE_DATA_PAGE_SIZE: perf_event_sample_format = 4194304;
pub const PERF_SAMPLE_CODE_PAGE_SIZE: perf_event_sample_format = 8388608;
pub const PERF_SAMPLE_WEIGHT_STRUCT: perf_event_sample_format = 16777216;
pub const PERF_SAMPLE_MAX: perf_event_sample_format = 33554432;
pub type perf_event_sample_format = ::std::os::raw::c_uint;
pub const PERF_SAMPLE_BRANCH_USER_SHIFT: perf_branch_sample_type_shift = 0;
pub const PERF_SAMPLE_BRANCH_KERNEL_SHIFT: perf_branch_sample_type_shift = 1;
pub const PERF_SAMPLE_BRANCH_HV_SHIFT: perf_branch_sample_type_shift = 2;
pub const PERF_SAMPLE_BRANCH_ANY_SHIFT: perf_branch_sample_type_shift = 3;
pub const PERF_SAMPLE_BRANCH_ANY_CALL_SHIFT: perf_branch_sample_type_shift = 4;
pub const PERF_SAMPLE_BRANCH_ANY_RETURN_SHIFT: perf_branch_sample_type_shift = 5;
pub const PERF_SAMPLE_BRANCH_IND_CALL_SHIFT: perf_branch_sample_type_shift = 6;
pub const PERF_SAMPLE_BRANCH_ABORT_TX_SHIFT: perf_branch_sample_type_shift = 7;
pub const PERF_SAMPLE_BRANCH_IN_TX_SHIFT: perf_branch_sample_type_shift = 8;
pub const PERF_SAMPLE_BRANCH_NO_TX_SHIFT: perf_branch_sample_type_shift = 9;
pub const PERF_SAMPLE_BRANCH_COND_SHIFT: perf_branch_sample_type_shift = 10;
pub const PERF_SAMPLE_BRANCH_CALL_STACK_SHIFT: perf_branch_sample_type_shift = 11;
pub const PERF_SAMPLE_BRANCH_IND_JUMP_SHIFT: perf_branch_sample_type_shift = 12;
pub const PERF_SAMPLE_BRANCH_CALL_SHIFT: perf_branch_sample_type_shift = 13;
pub const PERF_SAMPLE_BRANCH_NO_FLAGS_SHIFT: perf_branch_sample_type_shift = 14;
pub const PERF_SAMPLE_BRANCH_NO_CYCLES_SHIFT: perf_branch_sample_type_shift = 15;
pub const PERF_SAMPLE_BRANCH_TYPE_SAVE_SHIFT: perf_branch_sample_type_shift = 16;
pub const PERF_SAMPLE_BRANCH_HW_INDEX_SHIFT: perf_branch_sample_type_shift = 17;
pub const PERF_SAMPLE_BRANCH_PRIV_SAVE_SHIFT: perf_branch_sample_type_shift = 18;
pub const PERF_SAMPLE_BRANCH_COUNTERS_SHIFT: perf_branch_sample_type_shift = 19;
pub const PERF_SAMPLE_BRANCH_MAX_SHIFT: perf_branch_sample_type_shift = 20;
pub type perf_branch_sample_type_shift = ::std::os::raw::c_uint;
pub const PERF_SAMPLE_BRANCH_USER: perf_branch_sample_type = 1;
pub const PERF_SAMPLE_BRANCH_KERNEL: perf_branch_sample_type = 2;
pub const PERF_SAMPLE_BRANCH_HV: perf_branch_sample_type = 4;
pub const PERF_SAMPLE_BRANCH_ANY: perf_branch_sample_type = 8;
pub const PERF_SAMPLE_BRANCH_ANY_CALL: perf_branch_sample_type = 16;
pub const PERF_SAMPLE_BRANCH_ANY_RETURN: perf_branch_sample_type = 32;
pub const PERF_SAMPLE_BRANCH_IND_CALL: perf_branch_sample_type = 64;
pub const PERF_SAMPLE_BRANCH_ABORT_TX: perf_branch_sample_type = 128;
pub const PERF_SAMPLE_BRANCH_IN_TX: perf_branch_sample_type = 256;
pub const PERF_SAMPLE_BRANCH_NO_TX: perf_branch_sample_type = 512;
pub const PERF_SAMPLE_BRANCH_COND: perf_branch_sample_type = 1024;
pub const PERF_SAMPLE_BRANCH_CALL_STACK: perf_branch_sample_type = 2048;
pub const PERF_SAMPLE_BRANCH_IND_JUMP: perf_branch_sample_type = 4096;
pub const PERF_SAMPLE_BRANCH_CALL: perf_branch_sample_type = 8192;
pub const PERF_SAMPLE_BRANCH_NO_FLAGS: perf_branch_sample_type = 16384;
pub const PERF_SAMPLE_BRANCH_NO_CYCLES: perf_branch_sample_type = 32768;
pub const PERF_SAMPLE_BRANCH_TYPE_SAVE: perf_branch_sample_type = 65536;
pub const PERF_SAMPLE_BRANCH_HW_INDEX: perf_branch_sample_type = 131072;
pub const PERF_SAMPLE_BRANCH_PRIV_SAVE: perf_branch_sample_type = 262144;
pub const PERF_SAMPLE_BRANCH_COUNTERS: perf_branch_sample_type = 524288;
pub const PERF_SAMPLE_BRANCH_MAX: perf_branch_sample_type = 1048576;
pub type perf_branch_sample_type = ::std::os::raw::c_uint;
pub const PERF_BR_UNKNOWN: _bindgen_ty_1 = 0;
pub const PERF_BR_COND: _bindgen_ty_1 = 1;
pub const PERF_BR_UNCOND: _bindgen_ty_1 = 2;
pub const PERF_BR_IND: _bindgen_ty_1 = 3;
pub const PERF_BR_CALL: _bindgen_ty_1 = 4;
pub const PERF_BR_IND_CALL: _bindgen_ty_1 = 5;
pub const PERF_BR_RET: _bindgen_ty_1 = 6;
pub const PERF_BR_SYSCALL: _bindgen_ty_1 = 7;
pub const PERF_BR_SYSRET: _bindgen_ty_1 = 8;
pub const PERF_BR_COND_CALL: _bindgen_ty_1 = 9;
pub const PERF_BR_COND_RET: _bindgen_ty_1 = 10;
pub const PERF_BR_ERET: _bindgen_ty_1 = 11;
pub const PERF_BR_IRQ: _bindgen_ty_1 = 12;
pub const PERF_BR_SERROR: _bindgen_ty_1 = 13;
pub const PERF_BR_NO_TX: _bindgen_ty_1 = 14;
pub const PERF_BR_EXTEND_ABI: _bindgen_ty_1 = 15;
pub const PERF_BR_MAX: _bindgen_ty_1 = 16;
pub type _bindgen_ty_1 = ::std::os::raw::c_uint;
pub const PERF_BR_SPEC_NA: _bindgen_ty_2 = 0;
pub const PERF_BR_SPEC_WRONG_PATH: _bindgen_ty_2 = 1;
pub const PERF_BR_NON_SPEC_CORRECT_PATH: _bindgen_ty_2 = 2;
pub const PERF_BR_SPEC_CORRECT_PATH: _bindgen_ty_2 = 3;
pub const PERF_BR_SPEC_MAX: _bindgen_ty_2 = 4;
pub type _bindgen_ty_2 = ::std::os::raw::c_uint;
pub const PERF_BR_NEW_FAULT_ALGN: _bindgen_ty_3 = 0;
pub const PERF_BR_NEW_FAULT_DATA: _bindgen_ty_3 = 1;
pub const PERF_BR_NEW_FAULT_INST: _bindgen_ty_3 = 2;
pub const PERF_BR_NEW_ARCH_1: _bindgen_ty_3 = 3;
pub const PERF_BR_NEW_ARCH_2: _bindgen_ty_3 = 4;
pub const PERF_BR_NEW_ARCH_3: _bindgen_ty_3 = 5;
pub const PERF_BR_NEW_ARCH_4: _bindgen_ty_3 = 6;
pub const PERF_BR_NEW_ARCH_5: _bindgen_ty_3 = 7;
pub const PERF_BR_NEW_MAX: _bindgen_ty_3 = 8;
pub type _bindgen_ty_3 = ::std::os::raw::c_uint;
pub const PERF_BR_PRIV_UNKNOWN: _bindgen_ty_4 = 0;
pub const PERF_BR_PRIV_USER: _bindgen_ty_4 = 1;
pub const PERF_BR_PRIV_KERNEL: _bindgen_ty_4 = 2;
pub const PERF_BR_PRIV_HV: _bindgen_ty_4 = 3;
pub type _bindgen_ty_4 = ::std::os::raw::c_uint;
pub const PERF_SAMPLE_REGS_ABI_NONE: perf_sample_regs_abi = 0;
pub const PERF_SAMPLE_REGS_ABI_32: perf_sample_regs_abi = 1;
pub const PERF_SAMPLE_REGS_ABI_64: perf_sample_regs_abi = 2;
pub type perf_sample_regs_abi = ::std::os::raw::c_uint;
pub const PERF_TXN_ELISION: _bindgen_ty_5 = 1;
pub const PERF_TXN_TRANSACTION: _bindgen_ty_5 = 2;
pub const PERF_TXN_SYNC: _bindgen_ty_5 = 4;
pub const PERF_TXN_ASYNC: _bindgen_ty_5 = 8;
pub const PERF_TXN_RETRY: _bindgen_ty_5 = 16;
pub const PERF_TXN_CONFLICT: _bindgen_ty_5 = 32;
pub const PERF_TXN_CAPACITY_WRITE: _bindgen_ty_5 = 64;
pub const PERF_TXN_CAPACITY_READ: _bindgen_ty_5 = 128;
pub const PERF_TXN_MAX: _bindgen_ty_5 = 256;
pub const PERF_TXN_ABORT_MASK: _bindgen_ty_5 = 18446744069414584320;
pub const PERF_TXN_ABORT_SHIFT: _bindgen_ty_5 = 32;
pub type _bindgen_ty_5 = ::std::os::raw::c_ulong;
pub const PERF_FORMAT_TOTAL_TIME_ENABLED: perf_event_read_format = 1;
pub const PERF_FORMAT_TOTAL_TIME_RUNNING: perf_event_read_format = 2;
pub const PERF_FORMAT_ID: perf_event_read_format = 4;
pub const PERF_FORMAT_GROUP: perf_event_read_format = 8;
pub const PERF_FORMAT_LOST: perf_event_read_format = 16;
pub const PERF_FORMAT_MAX: perf_event_read_format = 32;
pub type perf_event_read_format = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
#[non_exhaustive]
pub struct perf_event_attr {
    pub type_: __u32,
    pub size: __u32,
    pub config: __u64,
    pub __bindgen_anon_1: perf_event_attr__bindgen_ty_1,
    pub sample_type: __u64,
    pub read_format: __u64,
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
    pub __bindgen_anon_2: perf_event_attr__bindgen_ty_2,
    pub bp_type: __u32,
    pub __bindgen_anon_3: perf_event_attr__bindgen_ty_3,
    pub __bindgen_anon_4: perf_event_attr__bindgen_ty_4,
    pub branch_sample_type: __u64,
    pub sample_regs_user: __u64,
    pub sample_stack_user: __u32,
    pub clockid: __s32,
    pub sample_regs_intr: __u64,
    pub aux_watermark: __u32,
    pub sample_max_stack: __u16,
    pub __reserved_2: __u16,
    pub aux_sample_size: __u32,
    pub __bindgen_anon_5: perf_event_attr__bindgen_ty_5,
    pub sig_data: __u64,
    pub config3: __u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_event_attr__bindgen_ty_1 {
    pub sample_period: __u64,
    pub sample_freq: __u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of perf_event_attr__bindgen_ty_1"]
        [::std::mem::size_of::<perf_event_attr__bindgen_ty_1>() - 8usize];
    ["Alignment of perf_event_attr__bindgen_ty_1"]
        [::std::mem::align_of::<perf_event_attr__bindgen_ty_1>() - 8usize];
    ["Offset of field: perf_event_attr__bindgen_ty_1::sample_period"]
        [::std::mem::offset_of!(perf_event_attr__bindgen_ty_1, sample_period) - 0usize];
    ["Offset of field: perf_event_attr__bindgen_ty_1::sample_freq"]
        [::std::mem::offset_of!(perf_event_attr__bindgen_ty_1, sample_freq) - 0usize];
};
impl Default for perf_event_attr__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for perf_event_attr__bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "perf_event_attr__bindgen_ty_1 {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_event_attr__bindgen_ty_2 {
    pub wakeup_events: __u32,
    pub wakeup_watermark: __u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of perf_event_attr__bindgen_ty_2"]
        [::std::mem::size_of::<perf_event_attr__bindgen_ty_2>() - 4usize];
    ["Alignment of perf_event_attr__bindgen_ty_2"]
        [::std::mem::align_of::<perf_event_attr__bindgen_ty_2>() - 4usize];
    ["Offset of field: perf_event_attr__bindgen_ty_2::wakeup_events"]
        [::std::mem::offset_of!(perf_event_attr__bindgen_ty_2, wakeup_events) - 0usize];
    ["Offset of field: perf_event_attr__bindgen_ty_2::wakeup_watermark"]
        [::std::mem::offset_of!(perf_event_attr__bindgen_ty_2, wakeup_watermark) - 0usize];
};
impl Default for perf_event_attr__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for perf_event_attr__bindgen_ty_2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "perf_event_attr__bindgen_ty_2 {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_event_attr__bindgen_ty_3 {
    pub bp_addr: __u64,
    pub kprobe_func: __u64,
    pub uprobe_path: __u64,
    pub config1: __u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of perf_event_attr__bindgen_ty_3"]
        [::std::mem::size_of::<perf_event_attr__bindgen_ty_3>() - 8usize];
    ["Alignment of perf_event_attr__bindgen_ty_3"]
        [::std::mem::align_of::<perf_event_attr__bindgen_ty_3>() - 8usize];
    ["Offset of field: perf_event_attr__bindgen_ty_3::bp_addr"]
        [::std::mem::offset_of!(perf_event_attr__bindgen_ty_3, bp_addr) - 0usize];
    ["Offset of field: perf_event_attr__bindgen_ty_3::kprobe_func"]
        [::std::mem::offset_of!(perf_event_attr__bindgen_ty_3, kprobe_func) - 0usize];
    ["Offset of field: perf_event_attr__bindgen_ty_3::uprobe_path"]
        [::std::mem::offset_of!(perf_event_attr__bindgen_ty_3, uprobe_path) - 0usize];
    ["Offset of field: perf_event_attr__bindgen_ty_3::config1"]
        [::std::mem::offset_of!(perf_event_attr__bindgen_ty_3, config1) - 0usize];
};
impl Default for perf_event_attr__bindgen_ty_3 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for perf_event_attr__bindgen_ty_3 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "perf_event_attr__bindgen_ty_3 {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_event_attr__bindgen_ty_4 {
    pub bp_len: __u64,
    pub kprobe_addr: __u64,
    pub probe_offset: __u64,
    pub config2: __u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of perf_event_attr__bindgen_ty_4"]
        [::std::mem::size_of::<perf_event_attr__bindgen_ty_4>() - 8usize];
    ["Alignment of perf_event_attr__bindgen_ty_4"]
        [::std::mem::align_of::<perf_event_attr__bindgen_ty_4>() - 8usize];
    ["Offset of field: perf_event_attr__bindgen_ty_4::bp_len"]
        [::std::mem::offset_of!(perf_event_attr__bindgen_ty_4, bp_len) - 0usize];
    ["Offset of field: perf_event_attr__bindgen_ty_4::kprobe_addr"]
        [::std::mem::offset_of!(perf_event_attr__bindgen_ty_4, kprobe_addr) - 0usize];
    ["Offset of field: perf_event_attr__bindgen_ty_4::probe_offset"]
        [::std::mem::offset_of!(perf_event_attr__bindgen_ty_4, probe_offset) - 0usize];
    ["Offset of field: perf_event_attr__bindgen_ty_4::config2"]
        [::std::mem::offset_of!(perf_event_attr__bindgen_ty_4, config2) - 0usize];
};
impl Default for perf_event_attr__bindgen_ty_4 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for perf_event_attr__bindgen_ty_4 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "perf_event_attr__bindgen_ty_4 {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_event_attr__bindgen_ty_5 {
    pub aux_action: __u32,
    pub __bindgen_anon_1: perf_event_attr__bindgen_ty_5__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
#[non_exhaustive]
pub struct perf_event_attr__bindgen_ty_5__bindgen_ty_1 {
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of perf_event_attr__bindgen_ty_5__bindgen_ty_1"]
        [::std::mem::size_of::<perf_event_attr__bindgen_ty_5__bindgen_ty_1>() - 4usize];
    ["Alignment of perf_event_attr__bindgen_ty_5__bindgen_ty_1"]
        [::std::mem::align_of::<perf_event_attr__bindgen_ty_5__bindgen_ty_1>() - 4usize];
};
impl perf_event_attr__bindgen_ty_5__bindgen_ty_1 {
    #[inline]
    pub fn aux_start_paused(&self) -> __u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_aux_start_paused(&mut self, val: __u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aux_start_paused_raw(this: *const Self) -> __u32 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_aux_start_paused_raw(this: *mut Self, val: __u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aux_pause(&self) -> __u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_aux_pause(&mut self, val: __u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aux_pause_raw(this: *const Self) -> __u32 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_aux_pause_raw(this: *mut Self, val: __u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aux_resume(&self) -> __u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_aux_resume(&mut self, val: __u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aux_resume_raw(this: *const Self) -> __u32 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                2usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_aux_resume_raw(this: *mut Self, val: __u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                2usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn __reserved_3(&self) -> __u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 29u8) as u32) }
    }
    #[inline]
    pub fn set___reserved_3(&mut self, val: __u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 29u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn __reserved_3_raw(this: *const Self) -> __u32 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                3usize,
                29u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set___reserved_3_raw(this: *mut Self, val: __u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                3usize,
                29u8,
                val as u64,
            )
        }
    }
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn new_bitfield_1(
        aux_start_paused: __u32,
        aux_pause: __u32,
        aux_resume: __u32,
        __reserved_3: __u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let aux_start_paused: u32 = unsafe { ::std::mem::transmute(aux_start_paused) };
            aux_start_paused as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let aux_pause: u32 = unsafe { ::std::mem::transmute(aux_pause) };
            aux_pause as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let aux_resume: u32 = unsafe { ::std::mem::transmute(aux_resume) };
            aux_resume as u64
        });
        __bindgen_bitfield_unit.set(3usize, 29u8, {
            let __reserved_3: u32 = unsafe { ::std::mem::transmute(__reserved_3) };
            __reserved_3 as u64
        });
        __bindgen_bitfield_unit
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of perf_event_attr__bindgen_ty_5"]
        [::std::mem::size_of::<perf_event_attr__bindgen_ty_5>() - 4usize];
    ["Alignment of perf_event_attr__bindgen_ty_5"]
        [::std::mem::align_of::<perf_event_attr__bindgen_ty_5>() - 4usize];
    ["Offset of field: perf_event_attr__bindgen_ty_5::aux_action"]
        [::std::mem::offset_of!(perf_event_attr__bindgen_ty_5, aux_action) - 0usize];
};
impl Default for perf_event_attr__bindgen_ty_5 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for perf_event_attr__bindgen_ty_5 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "perf_event_attr__bindgen_ty_5 {{ union }}")
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of perf_event_attr"][::std::mem::size_of::<perf_event_attr>() - 136usize];
    ["Alignment of perf_event_attr"][::std::mem::align_of::<perf_event_attr>() - 8usize];
    ["Offset of field: perf_event_attr::type_"]
        [::std::mem::offset_of!(perf_event_attr, type_) - 0usize];
    ["Offset of field: perf_event_attr::size"]
        [::std::mem::offset_of!(perf_event_attr, size) - 4usize];
    ["Offset of field: perf_event_attr::config"]
        [::std::mem::offset_of!(perf_event_attr, config) - 8usize];
    ["Offset of field: perf_event_attr::sample_type"]
        [::std::mem::offset_of!(perf_event_attr, sample_type) - 24usize];
    ["Offset of field: perf_event_attr::read_format"]
        [::std::mem::offset_of!(perf_event_attr, read_format) - 32usize];
    ["Offset of field: perf_event_attr::bp_type"]
        [::std::mem::offset_of!(perf_event_attr, bp_type) - 52usize];
    ["Offset of field: perf_event_attr::branch_sample_type"]
        [::std::mem::offset_of!(perf_event_attr, branch_sample_type) - 72usize];
    ["Offset of field: perf_event_attr::sample_regs_user"]
        [::std::mem::offset_of!(perf_event_attr, sample_regs_user) - 80usize];
    ["Offset of field: perf_event_attr::sample_stack_user"]
        [::std::mem::offset_of!(perf_event_attr, sample_stack_user) - 88usize];
    ["Offset of field: perf_event_attr::clockid"]
        [::std::mem::offset_of!(perf_event_attr, clockid) - 92usize];
    ["Offset of field: perf_event_attr::sample_regs_intr"]
        [::std::mem::offset_of!(perf_event_attr, sample_regs_intr) - 96usize];
    ["Offset of field: perf_event_attr::aux_watermark"]
        [::std::mem::offset_of!(perf_event_attr, aux_watermark) - 104usize];
    ["Offset of field: perf_event_attr::sample_max_stack"]
        [::std::mem::offset_of!(perf_event_attr, sample_max_stack) - 108usize];
    ["Offset of field: perf_event_attr::__reserved_2"]
        [::std::mem::offset_of!(perf_event_attr, __reserved_2) - 110usize];
    ["Offset of field: perf_event_attr::aux_sample_size"]
        [::std::mem::offset_of!(perf_event_attr, aux_sample_size) - 112usize];
    ["Offset of field: perf_event_attr::sig_data"]
        [::std::mem::offset_of!(perf_event_attr, sig_data) - 120usize];
    ["Offset of field: perf_event_attr::config3"]
        [::std::mem::offset_of!(perf_event_attr, config3) - 128usize];
};
impl Default for perf_event_attr {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for perf_event_attr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! (f , "perf_event_attr {{ type: {:?}, size: {:?}, config: {:?}, __bindgen_anon_1: {:?}, sample_type: {:?}, read_format: {:?}, disabled : {:?}, inherit : {:?}, pinned : {:?}, exclusive : {:?}, exclude_user : {:?}, exclude_kernel : {:?}, exclude_hv : {:?}, exclude_idle : {:?}, mmap : {:?}, comm : {:?}, freq : {:?}, inherit_stat : {:?}, enable_on_exec : {:?}, task : {:?}, watermark : {:?}, precise_ip : {:?}, mmap_data : {:?}, sample_id_all : {:?}, exclude_host : {:?}, exclude_guest : {:?}, exclude_callchain_kernel : {:?}, exclude_callchain_user : {:?}, mmap2 : {:?}, comm_exec : {:?}, use_clockid : {:?}, context_switch : {:?}, write_backward : {:?}, namespaces : {:?}, ksymbol : {:?}, bpf_event : {:?}, aux_output : {:?}, cgroup : {:?}, text_poke : {:?}, build_id : {:?}, inherit_thread : {:?}, remove_on_exec : {:?}, sigtrap : {:?}, __reserved_1 : {:?}, __bindgen_anon_2: {:?}, bp_type: {:?}, __bindgen_anon_3: {:?}, __bindgen_anon_4: {:?}, branch_sample_type: {:?}, sample_regs_user: {:?}, sample_stack_user: {:?}, clockid: {:?}, sample_regs_intr: {:?}, aux_watermark: {:?}, sample_max_stack: {:?}, __reserved_2: {:?}, aux_sample_size: {:?}, __bindgen_anon_5: {:?}, sig_data: {:?}, config3: {:?} }}" , self . type_ , self . size , self . config , self . __bindgen_anon_1 , self . sample_type , self . read_format , self . disabled () , self . inherit () , self . pinned () , self . exclusive () , self . exclude_user () , self . exclude_kernel () , self . exclude_hv () , self . exclude_idle () , self . mmap () , self . comm () , self . freq () , self . inherit_stat () , self . enable_on_exec () , self . task () , self . watermark () , self . precise_ip () , self . mmap_data () , self . sample_id_all () , self . exclude_host () , self . exclude_guest () , self . exclude_callchain_kernel () , self . exclude_callchain_user () , self . mmap2 () , self . comm_exec () , self . use_clockid () , self . context_switch () , self . write_backward () , self . namespaces () , self . ksymbol () , self . bpf_event () , self . aux_output () , self . cgroup () , self . text_poke () , self . build_id () , self . inherit_thread () , self . remove_on_exec () , self . sigtrap () , self . __reserved_1 () , self . __bindgen_anon_2 , self . bp_type , self . __bindgen_anon_3 , self . __bindgen_anon_4 , self . branch_sample_type , self . sample_regs_user , self . sample_stack_user , self . clockid , self . sample_regs_intr , self . aux_watermark , self . sample_max_stack , self . __reserved_2 , self . aux_sample_size , self . __bindgen_anon_5 , self . sig_data , self . config3)
    }
}
impl perf_event_attr {
    #[inline]
    pub fn disabled(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_disabled(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn disabled_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_disabled_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn inherit(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_inherit(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn inherit_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_inherit_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn pinned(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_pinned(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn pinned_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                2usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_pinned_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                2usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn exclusive(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_exclusive(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn exclusive_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                3usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_exclusive_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                3usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn exclude_user(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_exclude_user(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn exclude_user_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                4usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_exclude_user_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                4usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn exclude_kernel(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_exclude_kernel(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn exclude_kernel_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                5usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_exclude_kernel_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                5usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn exclude_hv(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_exclude_hv(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn exclude_hv_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                6usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_exclude_hv_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                6usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn exclude_idle(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_exclude_idle(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn exclude_idle_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_exclude_idle_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mmap(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mmap(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mmap_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                8usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mmap_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                8usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn comm(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_comm(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn comm_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                9usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_comm_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                9usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn freq(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_freq(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(10usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn freq_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                10usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_freq_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                10usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn inherit_stat(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_inherit_stat(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(11usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn inherit_stat_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                11usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_inherit_stat_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                11usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn enable_on_exec(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(12usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_enable_on_exec(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(12usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn enable_on_exec_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                12usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_enable_on_exec_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                12usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn task(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_task(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn task_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_task_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn watermark(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(14usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_watermark(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(14usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn watermark_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                14usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_watermark_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                14usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn precise_ip(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(15usize, 2u8) as u64) }
    }
    #[inline]
    pub fn set_precise_ip(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(15usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn precise_ip_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                15usize,
                2u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_precise_ip_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                15usize,
                2u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mmap_data(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(17usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mmap_data(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(17usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mmap_data_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                17usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mmap_data_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                17usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn sample_id_all(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(18usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_sample_id_all(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(18usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn sample_id_all_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                18usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_sample_id_all_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                18usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn exclude_host(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(19usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_exclude_host(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(19usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn exclude_host_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                19usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_exclude_host_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                19usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn exclude_guest(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(20usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_exclude_guest(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(20usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn exclude_guest_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                20usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_exclude_guest_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                20usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn exclude_callchain_kernel(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(21usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_exclude_callchain_kernel(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(21usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn exclude_callchain_kernel_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                21usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_exclude_callchain_kernel_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                21usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn exclude_callchain_user(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(22usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_exclude_callchain_user(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(22usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn exclude_callchain_user_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                22usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_exclude_callchain_user_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                22usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mmap2(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(23usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mmap2(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(23usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mmap2_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                23usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mmap2_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                23usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn comm_exec(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(24usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_comm_exec(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn comm_exec_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                24usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_comm_exec_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                24usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn use_clockid(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(25usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_use_clockid(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(25usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn use_clockid_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                25usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_use_clockid_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                25usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn context_switch(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(26usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_context_switch(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(26usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn context_switch_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                26usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_context_switch_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                26usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn write_backward(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(27usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_write_backward(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(27usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn write_backward_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                27usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_write_backward_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                27usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn namespaces(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(28usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_namespaces(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(28usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn namespaces_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                28usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_namespaces_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                28usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn ksymbol(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(29usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_ksymbol(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(29usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn ksymbol_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                29usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_ksymbol_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                29usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn bpf_event(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(30usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_bpf_event(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(30usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn bpf_event_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                30usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_bpf_event_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                30usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aux_output(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(31usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_aux_output(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(31usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aux_output_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                31usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aux_output_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                31usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn cgroup(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(32usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_cgroup(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(32usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn cgroup_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                32usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_cgroup_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                32usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn text_poke(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(33usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_text_poke(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(33usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn text_poke_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                33usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_text_poke_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                33usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn build_id(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(34usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_build_id(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(34usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn build_id_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                34usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_build_id_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                34usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn inherit_thread(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(35usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_inherit_thread(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(35usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn inherit_thread_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                35usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_inherit_thread_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                35usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn remove_on_exec(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(36usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_remove_on_exec(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(36usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn remove_on_exec_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                36usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_remove_on_exec_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                36usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn sigtrap(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(37usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_sigtrap(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(37usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn sigtrap_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                37usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_sigtrap_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                37usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn __reserved_1(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(38usize, 26u8) as u64) }
    }
    #[inline]
    pub fn set___reserved_1(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(38usize, 26u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn __reserved_1_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                38usize,
                26u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set___reserved_1_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                38usize,
                26u8,
                val as u64,
            )
        }
    }
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn new_bitfield_1(
        disabled: __u64,
        inherit: __u64,
        pinned: __u64,
        exclusive: __u64,
        exclude_user: __u64,
        exclude_kernel: __u64,
        exclude_hv: __u64,
        exclude_idle: __u64,
        mmap: __u64,
        comm: __u64,
        freq: __u64,
        inherit_stat: __u64,
        enable_on_exec: __u64,
        task: __u64,
        watermark: __u64,
        precise_ip: __u64,
        mmap_data: __u64,
        sample_id_all: __u64,
        exclude_host: __u64,
        exclude_guest: __u64,
        exclude_callchain_kernel: __u64,
        exclude_callchain_user: __u64,
        mmap2: __u64,
        comm_exec: __u64,
        use_clockid: __u64,
        context_switch: __u64,
        write_backward: __u64,
        namespaces: __u64,
        ksymbol: __u64,
        bpf_event: __u64,
        aux_output: __u64,
        cgroup: __u64,
        text_poke: __u64,
        build_id: __u64,
        inherit_thread: __u64,
        remove_on_exec: __u64,
        sigtrap: __u64,
        __reserved_1: __u64,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let disabled: u64 = unsafe { ::std::mem::transmute(disabled) };
            disabled as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let inherit: u64 = unsafe { ::std::mem::transmute(inherit) };
            inherit as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let pinned: u64 = unsafe { ::std::mem::transmute(pinned) };
            pinned as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let exclusive: u64 = unsafe { ::std::mem::transmute(exclusive) };
            exclusive as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let exclude_user: u64 = unsafe { ::std::mem::transmute(exclude_user) };
            exclude_user as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let exclude_kernel: u64 = unsafe { ::std::mem::transmute(exclude_kernel) };
            exclude_kernel as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let exclude_hv: u64 = unsafe { ::std::mem::transmute(exclude_hv) };
            exclude_hv as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let exclude_idle: u64 = unsafe { ::std::mem::transmute(exclude_idle) };
            exclude_idle as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let mmap: u64 = unsafe { ::std::mem::transmute(mmap) };
            mmap as u64
        });
        __bindgen_bitfield_unit.set(9usize, 1u8, {
            let comm: u64 = unsafe { ::std::mem::transmute(comm) };
            comm as u64
        });
        __bindgen_bitfield_unit.set(10usize, 1u8, {
            let freq: u64 = unsafe { ::std::mem::transmute(freq) };
            freq as u64
        });
        __bindgen_bitfield_unit.set(11usize, 1u8, {
            let inherit_stat: u64 = unsafe { ::std::mem::transmute(inherit_stat) };
            inherit_stat as u64
        });
        __bindgen_bitfield_unit.set(12usize, 1u8, {
            let enable_on_exec: u64 = unsafe { ::std::mem::transmute(enable_on_exec) };
            enable_on_exec as u64
        });
        __bindgen_bitfield_unit.set(13usize, 1u8, {
            let task: u64 = unsafe { ::std::mem::transmute(task) };
            task as u64
        });
        __bindgen_bitfield_unit.set(14usize, 1u8, {
            let watermark: u64 = unsafe { ::std::mem::transmute(watermark) };
            watermark as u64
        });
        __bindgen_bitfield_unit.set(15usize, 2u8, {
            let precise_ip: u64 = unsafe { ::std::mem::transmute(precise_ip) };
            precise_ip as u64
        });
        __bindgen_bitfield_unit.set(17usize, 1u8, {
            let mmap_data: u64 = unsafe { ::std::mem::transmute(mmap_data) };
            mmap_data as u64
        });
        __bindgen_bitfield_unit.set(18usize, 1u8, {
            let sample_id_all: u64 = unsafe { ::std::mem::transmute(sample_id_all) };
            sample_id_all as u64
        });
        __bindgen_bitfield_unit.set(19usize, 1u8, {
            let exclude_host: u64 = unsafe { ::std::mem::transmute(exclude_host) };
            exclude_host as u64
        });
        __bindgen_bitfield_unit.set(20usize, 1u8, {
            let exclude_guest: u64 = unsafe { ::std::mem::transmute(exclude_guest) };
            exclude_guest as u64
        });
        __bindgen_bitfield_unit.set(21usize, 1u8, {
            let exclude_callchain_kernel: u64 =
                unsafe { ::std::mem::transmute(exclude_callchain_kernel) };
            exclude_callchain_kernel as u64
        });
        __bindgen_bitfield_unit.set(22usize, 1u8, {
            let exclude_callchain_user: u64 =
                unsafe { ::std::mem::transmute(exclude_callchain_user) };
            exclude_callchain_user as u64
        });
        __bindgen_bitfield_unit.set(23usize, 1u8, {
            let mmap2: u64 = unsafe { ::std::mem::transmute(mmap2) };
            mmap2 as u64
        });
        __bindgen_bitfield_unit.set(24usize, 1u8, {
            let comm_exec: u64 = unsafe { ::std::mem::transmute(comm_exec) };
            comm_exec as u64
        });
        __bindgen_bitfield_unit.set(25usize, 1u8, {
            let use_clockid: u64 = unsafe { ::std::mem::transmute(use_clockid) };
            use_clockid as u64
        });
        __bindgen_bitfield_unit.set(26usize, 1u8, {
            let context_switch: u64 = unsafe { ::std::mem::transmute(context_switch) };
            context_switch as u64
        });
        __bindgen_bitfield_unit.set(27usize, 1u8, {
            let write_backward: u64 = unsafe { ::std::mem::transmute(write_backward) };
            write_backward as u64
        });
        __bindgen_bitfield_unit.set(28usize, 1u8, {
            let namespaces: u64 = unsafe { ::std::mem::transmute(namespaces) };
            namespaces as u64
        });
        __bindgen_bitfield_unit.set(29usize, 1u8, {
            let ksymbol: u64 = unsafe { ::std::mem::transmute(ksymbol) };
            ksymbol as u64
        });
        __bindgen_bitfield_unit.set(30usize, 1u8, {
            let bpf_event: u64 = unsafe { ::std::mem::transmute(bpf_event) };
            bpf_event as u64
        });
        __bindgen_bitfield_unit.set(31usize, 1u8, {
            let aux_output: u64 = unsafe { ::std::mem::transmute(aux_output) };
            aux_output as u64
        });
        __bindgen_bitfield_unit.set(32usize, 1u8, {
            let cgroup: u64 = unsafe { ::std::mem::transmute(cgroup) };
            cgroup as u64
        });
        __bindgen_bitfield_unit.set(33usize, 1u8, {
            let text_poke: u64 = unsafe { ::std::mem::transmute(text_poke) };
            text_poke as u64
        });
        __bindgen_bitfield_unit.set(34usize, 1u8, {
            let build_id: u64 = unsafe { ::std::mem::transmute(build_id) };
            build_id as u64
        });
        __bindgen_bitfield_unit.set(35usize, 1u8, {
            let inherit_thread: u64 = unsafe { ::std::mem::transmute(inherit_thread) };
            inherit_thread as u64
        });
        __bindgen_bitfield_unit.set(36usize, 1u8, {
            let remove_on_exec: u64 = unsafe { ::std::mem::transmute(remove_on_exec) };
            remove_on_exec as u64
        });
        __bindgen_bitfield_unit.set(37usize, 1u8, {
            let sigtrap: u64 = unsafe { ::std::mem::transmute(sigtrap) };
            sigtrap as u64
        });
        __bindgen_bitfield_unit.set(38usize, 26u8, {
            let __reserved_1: u64 = unsafe { ::std::mem::transmute(__reserved_1) };
            __reserved_1 as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct perf_event_query_bpf {
    pub ids_len: __u32,
    pub prog_cnt: __u32,
    pub ids: __IncompleteArrayField<__u32>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of perf_event_query_bpf"][::std::mem::size_of::<perf_event_query_bpf>() - 8usize];
    ["Alignment of perf_event_query_bpf"][::std::mem::align_of::<perf_event_query_bpf>() - 4usize];
    ["Offset of field: perf_event_query_bpf::ids_len"]
        [::std::mem::offset_of!(perf_event_query_bpf, ids_len) - 0usize];
    ["Offset of field: perf_event_query_bpf::prog_cnt"]
        [::std::mem::offset_of!(perf_event_query_bpf, prog_cnt) - 4usize];
    ["Offset of field: perf_event_query_bpf::ids"]
        [::std::mem::offset_of!(perf_event_query_bpf, ids) - 8usize];
};
pub const PERF_IOC_FLAG_GROUP: perf_event_ioc_flags = 1;
pub type perf_event_ioc_flags = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
#[non_exhaustive]
pub struct perf_event_mmap_page {
    pub version: __u32,
    pub compat_version: __u32,
    pub lock: __u32,
    pub index: __u32,
    pub offset: __s64,
    pub time_enabled: __u64,
    pub time_running: __u64,
    pub __bindgen_anon_1: perf_event_mmap_page__bindgen_ty_1,
    pub pmc_width: __u16,
    pub time_shift: __u16,
    pub time_mult: __u32,
    pub time_offset: __u64,
    pub time_zero: __u64,
    pub size: __u32,
    pub __reserved_1: __u32,
    pub time_cycles: __u64,
    pub time_mask: __u64,
    pub __reserved: [__u8; 928usize],
    pub data_head: __u64,
    pub data_tail: __u64,
    pub data_offset: __u64,
    pub data_size: __u64,
    pub aux_head: __u64,
    pub aux_tail: __u64,
    pub aux_offset: __u64,
    pub aux_size: __u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_event_mmap_page__bindgen_ty_1 {
    pub capabilities: __u64,
    pub __bindgen_anon_1: perf_event_mmap_page__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
#[non_exhaustive]
pub struct perf_event_mmap_page__bindgen_ty_1__bindgen_ty_1 {
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of perf_event_mmap_page__bindgen_ty_1__bindgen_ty_1"]
        [::std::mem::size_of::<perf_event_mmap_page__bindgen_ty_1__bindgen_ty_1>() - 8usize];
    ["Alignment of perf_event_mmap_page__bindgen_ty_1__bindgen_ty_1"]
        [::std::mem::align_of::<perf_event_mmap_page__bindgen_ty_1__bindgen_ty_1>() - 8usize];
};
impl perf_event_mmap_page__bindgen_ty_1__bindgen_ty_1 {
    #[inline]
    pub fn cap_bit0(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_cap_bit0(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn cap_bit0_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_cap_bit0_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn cap_bit0_is_deprecated(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_cap_bit0_is_deprecated(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn cap_bit0_is_deprecated_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_cap_bit0_is_deprecated_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn cap_user_rdpmc(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_cap_user_rdpmc(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn cap_user_rdpmc_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                2usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_cap_user_rdpmc_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                2usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn cap_user_time(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_cap_user_time(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn cap_user_time_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                3usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_cap_user_time_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                3usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn cap_user_time_zero(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_cap_user_time_zero(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn cap_user_time_zero_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                4usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_cap_user_time_zero_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                4usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn cap_user_time_short(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_cap_user_time_short(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn cap_user_time_short_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                5usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_cap_user_time_short_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                5usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn cap_____res(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 58u8) as u64) }
    }
    #[inline]
    pub fn set_cap_____res(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 58u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn cap_____res_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                6usize,
                58u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_cap_____res_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                6usize,
                58u8,
                val as u64,
            )
        }
    }
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn new_bitfield_1(
        cap_bit0: __u64,
        cap_bit0_is_deprecated: __u64,
        cap_user_rdpmc: __u64,
        cap_user_time: __u64,
        cap_user_time_zero: __u64,
        cap_user_time_short: __u64,
        cap_____res: __u64,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let cap_bit0: u64 = unsafe { ::std::mem::transmute(cap_bit0) };
            cap_bit0 as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let cap_bit0_is_deprecated: u64 =
                unsafe { ::std::mem::transmute(cap_bit0_is_deprecated) };
            cap_bit0_is_deprecated as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let cap_user_rdpmc: u64 = unsafe { ::std::mem::transmute(cap_user_rdpmc) };
            cap_user_rdpmc as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let cap_user_time: u64 = unsafe { ::std::mem::transmute(cap_user_time) };
            cap_user_time as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let cap_user_time_zero: u64 = unsafe { ::std::mem::transmute(cap_user_time_zero) };
            cap_user_time_zero as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let cap_user_time_short: u64 = unsafe { ::std::mem::transmute(cap_user_time_short) };
            cap_user_time_short as u64
        });
        __bindgen_bitfield_unit.set(6usize, 58u8, {
            let cap_____res: u64 = unsafe { ::std::mem::transmute(cap_____res) };
            cap_____res as u64
        });
        __bindgen_bitfield_unit
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of perf_event_mmap_page__bindgen_ty_1"]
        [::std::mem::size_of::<perf_event_mmap_page__bindgen_ty_1>() - 8usize];
    ["Alignment of perf_event_mmap_page__bindgen_ty_1"]
        [::std::mem::align_of::<perf_event_mmap_page__bindgen_ty_1>() - 8usize];
    ["Offset of field: perf_event_mmap_page__bindgen_ty_1::capabilities"]
        [::std::mem::offset_of!(perf_event_mmap_page__bindgen_ty_1, capabilities) - 0usize];
};
impl Default for perf_event_mmap_page__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for perf_event_mmap_page__bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "perf_event_mmap_page__bindgen_ty_1 {{ union }}")
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of perf_event_mmap_page"][::std::mem::size_of::<perf_event_mmap_page>() - 1088usize];
    ["Alignment of perf_event_mmap_page"][::std::mem::align_of::<perf_event_mmap_page>() - 8usize];
    ["Offset of field: perf_event_mmap_page::version"]
        [::std::mem::offset_of!(perf_event_mmap_page, version) - 0usize];
    ["Offset of field: perf_event_mmap_page::compat_version"]
        [::std::mem::offset_of!(perf_event_mmap_page, compat_version) - 4usize];
    ["Offset of field: perf_event_mmap_page::lock"]
        [::std::mem::offset_of!(perf_event_mmap_page, lock) - 8usize];
    ["Offset of field: perf_event_mmap_page::index"]
        [::std::mem::offset_of!(perf_event_mmap_page, index) - 12usize];
    ["Offset of field: perf_event_mmap_page::offset"]
        [::std::mem::offset_of!(perf_event_mmap_page, offset) - 16usize];
    ["Offset of field: perf_event_mmap_page::time_enabled"]
        [::std::mem::offset_of!(perf_event_mmap_page, time_enabled) - 24usize];
    ["Offset of field: perf_event_mmap_page::time_running"]
        [::std::mem::offset_of!(perf_event_mmap_page, time_running) - 32usize];
    ["Offset of field: perf_event_mmap_page::pmc_width"]
        [::std::mem::offset_of!(perf_event_mmap_page, pmc_width) - 48usize];
    ["Offset of field: perf_event_mmap_page::time_shift"]
        [::std::mem::offset_of!(perf_event_mmap_page, time_shift) - 50usize];
    ["Offset of field: perf_event_mmap_page::time_mult"]
        [::std::mem::offset_of!(perf_event_mmap_page, time_mult) - 52usize];
    ["Offset of field: perf_event_mmap_page::time_offset"]
        [::std::mem::offset_of!(perf_event_mmap_page, time_offset) - 56usize];
    ["Offset of field: perf_event_mmap_page::time_zero"]
        [::std::mem::offset_of!(perf_event_mmap_page, time_zero) - 64usize];
    ["Offset of field: perf_event_mmap_page::size"]
        [::std::mem::offset_of!(perf_event_mmap_page, size) - 72usize];
    ["Offset of field: perf_event_mmap_page::__reserved_1"]
        [::std::mem::offset_of!(perf_event_mmap_page, __reserved_1) - 76usize];
    ["Offset of field: perf_event_mmap_page::time_cycles"]
        [::std::mem::offset_of!(perf_event_mmap_page, time_cycles) - 80usize];
    ["Offset of field: perf_event_mmap_page::time_mask"]
        [::std::mem::offset_of!(perf_event_mmap_page, time_mask) - 88usize];
    ["Offset of field: perf_event_mmap_page::__reserved"]
        [::std::mem::offset_of!(perf_event_mmap_page, __reserved) - 96usize];
    ["Offset of field: perf_event_mmap_page::data_head"]
        [::std::mem::offset_of!(perf_event_mmap_page, data_head) - 1024usize];
    ["Offset of field: perf_event_mmap_page::data_tail"]
        [::std::mem::offset_of!(perf_event_mmap_page, data_tail) - 1032usize];
    ["Offset of field: perf_event_mmap_page::data_offset"]
        [::std::mem::offset_of!(perf_event_mmap_page, data_offset) - 1040usize];
    ["Offset of field: perf_event_mmap_page::data_size"]
        [::std::mem::offset_of!(perf_event_mmap_page, data_size) - 1048usize];
    ["Offset of field: perf_event_mmap_page::aux_head"]
        [::std::mem::offset_of!(perf_event_mmap_page, aux_head) - 1056usize];
    ["Offset of field: perf_event_mmap_page::aux_tail"]
        [::std::mem::offset_of!(perf_event_mmap_page, aux_tail) - 1064usize];
    ["Offset of field: perf_event_mmap_page::aux_offset"]
        [::std::mem::offset_of!(perf_event_mmap_page, aux_offset) - 1072usize];
    ["Offset of field: perf_event_mmap_page::aux_size"]
        [::std::mem::offset_of!(perf_event_mmap_page, aux_size) - 1080usize];
};
impl Default for perf_event_mmap_page {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for perf_event_mmap_page {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! (f , "perf_event_mmap_page {{ version: {:?}, compat_version: {:?}, lock: {:?}, index: {:?}, offset: {:?}, time_enabled: {:?}, time_running: {:?}, __bindgen_anon_1: {:?}, pmc_width: {:?}, time_shift: {:?}, time_mult: {:?}, time_offset: {:?}, time_zero: {:?}, size: {:?}, __reserved_1: {:?}, time_cycles: {:?}, time_mask: {:?}, __reserved: {:?}, data_head: {:?}, data_tail: {:?}, data_offset: {:?}, data_size: {:?}, aux_head: {:?}, aux_tail: {:?}, aux_offset: {:?}, aux_size: {:?} }}" , self . version , self . compat_version , self . lock , self . index , self . offset , self . time_enabled , self . time_running , self . __bindgen_anon_1 , self . pmc_width , self . time_shift , self . time_mult , self . time_offset , self . time_zero , self . size , self . __reserved_1 , self . time_cycles , self . time_mask , self . __reserved , self . data_head , self . data_tail , self . data_offset , self . data_size , self . aux_head , self . aux_tail , self . aux_offset , self . aux_size)
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
#[non_exhaustive]
pub struct perf_event_header {
    pub type_: __u32,
    pub misc: __u16,
    pub size: __u16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of perf_event_header"][::std::mem::size_of::<perf_event_header>() - 8usize];
    ["Alignment of perf_event_header"][::std::mem::align_of::<perf_event_header>() - 4usize];
    ["Offset of field: perf_event_header::type_"]
        [::std::mem::offset_of!(perf_event_header, type_) - 0usize];
    ["Offset of field: perf_event_header::misc"]
        [::std::mem::offset_of!(perf_event_header, misc) - 4usize];
    ["Offset of field: perf_event_header::size"]
        [::std::mem::offset_of!(perf_event_header, size) - 6usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
#[non_exhaustive]
pub struct perf_ns_link_info {
    pub dev: __u64,
    pub ino: __u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of perf_ns_link_info"][::std::mem::size_of::<perf_ns_link_info>() - 16usize];
    ["Alignment of perf_ns_link_info"][::std::mem::align_of::<perf_ns_link_info>() - 8usize];
    ["Offset of field: perf_ns_link_info::dev"]
        [::std::mem::offset_of!(perf_ns_link_info, dev) - 0usize];
    ["Offset of field: perf_ns_link_info::ino"]
        [::std::mem::offset_of!(perf_ns_link_info, ino) - 8usize];
};
pub const NET_NS_INDEX: _bindgen_ty_6 = 0;
pub const UTS_NS_INDEX: _bindgen_ty_6 = 1;
pub const IPC_NS_INDEX: _bindgen_ty_6 = 2;
pub const PID_NS_INDEX: _bindgen_ty_6 = 3;
pub const USER_NS_INDEX: _bindgen_ty_6 = 4;
pub const MNT_NS_INDEX: _bindgen_ty_6 = 5;
pub const CGROUP_NS_INDEX: _bindgen_ty_6 = 6;
pub const NR_NAMESPACES: _bindgen_ty_6 = 7;
pub type _bindgen_ty_6 = ::std::os::raw::c_uint;
pub const PERF_RECORD_MMAP: perf_event_type = 1;
pub const PERF_RECORD_LOST: perf_event_type = 2;
pub const PERF_RECORD_COMM: perf_event_type = 3;
pub const PERF_RECORD_EXIT: perf_event_type = 4;
pub const PERF_RECORD_THROTTLE: perf_event_type = 5;
pub const PERF_RECORD_UNTHROTTLE: perf_event_type = 6;
pub const PERF_RECORD_FORK: perf_event_type = 7;
pub const PERF_RECORD_READ: perf_event_type = 8;
pub const PERF_RECORD_SAMPLE: perf_event_type = 9;
pub const PERF_RECORD_MMAP2: perf_event_type = 10;
pub const PERF_RECORD_AUX: perf_event_type = 11;
pub const PERF_RECORD_ITRACE_START: perf_event_type = 12;
pub const PERF_RECORD_LOST_SAMPLES: perf_event_type = 13;
pub const PERF_RECORD_SWITCH: perf_event_type = 14;
pub const PERF_RECORD_SWITCH_CPU_WIDE: perf_event_type = 15;
pub const PERF_RECORD_NAMESPACES: perf_event_type = 16;
pub const PERF_RECORD_KSYMBOL: perf_event_type = 17;
pub const PERF_RECORD_BPF_EVENT: perf_event_type = 18;
pub const PERF_RECORD_CGROUP: perf_event_type = 19;
pub const PERF_RECORD_TEXT_POKE: perf_event_type = 20;
pub const PERF_RECORD_AUX_OUTPUT_HW_ID: perf_event_type = 21;
pub const PERF_RECORD_MAX: perf_event_type = 22;
pub type perf_event_type = ::std::os::raw::c_uint;
pub const PERF_RECORD_KSYMBOL_TYPE_UNKNOWN: perf_record_ksymbol_type = 0;
pub const PERF_RECORD_KSYMBOL_TYPE_BPF: perf_record_ksymbol_type = 1;
pub const PERF_RECORD_KSYMBOL_TYPE_OOL: perf_record_ksymbol_type = 2;
pub const PERF_RECORD_KSYMBOL_TYPE_MAX: perf_record_ksymbol_type = 3;
pub type perf_record_ksymbol_type = ::std::os::raw::c_uint;
pub const PERF_BPF_EVENT_UNKNOWN: perf_bpf_event_type = 0;
pub const PERF_BPF_EVENT_PROG_LOAD: perf_bpf_event_type = 1;
pub const PERF_BPF_EVENT_PROG_UNLOAD: perf_bpf_event_type = 2;
pub const PERF_BPF_EVENT_MAX: perf_bpf_event_type = 3;
pub type perf_bpf_event_type = ::std::os::raw::c_uint;
pub const PERF_CONTEXT_HV: perf_callchain_context = 18446744073709551584;
pub const PERF_CONTEXT_KERNEL: perf_callchain_context = 18446744073709551488;
pub const PERF_CONTEXT_USER: perf_callchain_context = 18446744073709551104;
pub const PERF_CONTEXT_GUEST: perf_callchain_context = 18446744073709549568;
pub const PERF_CONTEXT_GUEST_KERNEL: perf_callchain_context = 18446744073709549440;
pub const PERF_CONTEXT_GUEST_USER: perf_callchain_context = 18446744073709549056;
pub const PERF_CONTEXT_MAX: perf_callchain_context = 18446744073709547521;
pub type perf_callchain_context = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_mem_data_src {
    pub val: __u64,
    pub __bindgen_anon_1: perf_mem_data_src__bindgen_ty_1,
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
#[non_exhaustive]
pub struct perf_mem_data_src__bindgen_ty_1 {
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of perf_mem_data_src__bindgen_ty_1"]
        [::std::mem::size_of::<perf_mem_data_src__bindgen_ty_1>() - 8usize];
    ["Alignment of perf_mem_data_src__bindgen_ty_1"]
        [::std::mem::align_of::<perf_mem_data_src__bindgen_ty_1>() - 8usize];
};
impl perf_mem_data_src__bindgen_ty_1 {
    #[inline]
    pub fn mem_op(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 5u8) as u64) }
    }
    #[inline]
    pub fn set_mem_op(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_op_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                5u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_op_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                5u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mem_lvl(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 14u8) as u64) }
    }
    #[inline]
    pub fn set_mem_lvl(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 14u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_lvl_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                5usize,
                14u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_lvl_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                5usize,
                14u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mem_snoop(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(19usize, 5u8) as u64) }
    }
    #[inline]
    pub fn set_mem_snoop(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(19usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_snoop_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                19usize,
                5u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_snoop_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                19usize,
                5u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mem_lock(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(24usize, 2u8) as u64) }
    }
    #[inline]
    pub fn set_mem_lock(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_lock_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                24usize,
                2u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_lock_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                24usize,
                2u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mem_dtlb(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(26usize, 7u8) as u64) }
    }
    #[inline]
    pub fn set_mem_dtlb(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(26usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_dtlb_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                26usize,
                7u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_dtlb_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                26usize,
                7u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mem_lvl_num(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(33usize, 4u8) as u64) }
    }
    #[inline]
    pub fn set_mem_lvl_num(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(33usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_lvl_num_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                33usize,
                4u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_lvl_num_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                33usize,
                4u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mem_remote(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(37usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_remote(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(37usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_remote_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                37usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_remote_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                37usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mem_snoopx(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(38usize, 2u8) as u64) }
    }
    #[inline]
    pub fn set_mem_snoopx(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(38usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_snoopx_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                38usize,
                2u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_snoopx_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                38usize,
                2u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mem_blk(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(40usize, 3u8) as u64) }
    }
    #[inline]
    pub fn set_mem_blk(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(40usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_blk_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                40usize,
                3u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_blk_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                40usize,
                3u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mem_hops(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(43usize, 3u8) as u64) }
    }
    #[inline]
    pub fn set_mem_hops(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(43usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_hops_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                43usize,
                3u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_hops_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                43usize,
                3u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mem_rsvd(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(46usize, 18u8) as u64) }
    }
    #[inline]
    pub fn set_mem_rsvd(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(46usize, 18u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_rsvd_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                46usize,
                18u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_rsvd_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                46usize,
                18u8,
                val as u64,
            )
        }
    }
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn new_bitfield_1(
        mem_op: __u64,
        mem_lvl: __u64,
        mem_snoop: __u64,
        mem_lock: __u64,
        mem_dtlb: __u64,
        mem_lvl_num: __u64,
        mem_remote: __u64,
        mem_snoopx: __u64,
        mem_blk: __u64,
        mem_hops: __u64,
        mem_rsvd: __u64,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 5u8, {
            let mem_op: u64 = unsafe { ::std::mem::transmute(mem_op) };
            mem_op as u64
        });
        __bindgen_bitfield_unit.set(5usize, 14u8, {
            let mem_lvl: u64 = unsafe { ::std::mem::transmute(mem_lvl) };
            mem_lvl as u64
        });
        __bindgen_bitfield_unit.set(19usize, 5u8, {
            let mem_snoop: u64 = unsafe { ::std::mem::transmute(mem_snoop) };
            mem_snoop as u64
        });
        __bindgen_bitfield_unit.set(24usize, 2u8, {
            let mem_lock: u64 = unsafe { ::std::mem::transmute(mem_lock) };
            mem_lock as u64
        });
        __bindgen_bitfield_unit.set(26usize, 7u8, {
            let mem_dtlb: u64 = unsafe { ::std::mem::transmute(mem_dtlb) };
            mem_dtlb as u64
        });
        __bindgen_bitfield_unit.set(33usize, 4u8, {
            let mem_lvl_num: u64 = unsafe { ::std::mem::transmute(mem_lvl_num) };
            mem_lvl_num as u64
        });
        __bindgen_bitfield_unit.set(37usize, 1u8, {
            let mem_remote: u64 = unsafe { ::std::mem::transmute(mem_remote) };
            mem_remote as u64
        });
        __bindgen_bitfield_unit.set(38usize, 2u8, {
            let mem_snoopx: u64 = unsafe { ::std::mem::transmute(mem_snoopx) };
            mem_snoopx as u64
        });
        __bindgen_bitfield_unit.set(40usize, 3u8, {
            let mem_blk: u64 = unsafe { ::std::mem::transmute(mem_blk) };
            mem_blk as u64
        });
        __bindgen_bitfield_unit.set(43usize, 3u8, {
            let mem_hops: u64 = unsafe { ::std::mem::transmute(mem_hops) };
            mem_hops as u64
        });
        __bindgen_bitfield_unit.set(46usize, 18u8, {
            let mem_rsvd: u64 = unsafe { ::std::mem::transmute(mem_rsvd) };
            mem_rsvd as u64
        });
        __bindgen_bitfield_unit
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of perf_mem_data_src"][::std::mem::size_of::<perf_mem_data_src>() - 8usize];
    ["Alignment of perf_mem_data_src"][::std::mem::align_of::<perf_mem_data_src>() - 8usize];
    ["Offset of field: perf_mem_data_src::val"]
        [::std::mem::offset_of!(perf_mem_data_src, val) - 0usize];
};
impl Default for perf_mem_data_src {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for perf_mem_data_src {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "perf_mem_data_src {{ union }}")
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
#[non_exhaustive]
pub struct perf_branch_entry {
    pub from: __u64,
    pub to: __u64,
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of perf_branch_entry"][::std::mem::size_of::<perf_branch_entry>() - 24usize];
    ["Alignment of perf_branch_entry"][::std::mem::align_of::<perf_branch_entry>() - 8usize];
    ["Offset of field: perf_branch_entry::from"]
        [::std::mem::offset_of!(perf_branch_entry, from) - 0usize];
    ["Offset of field: perf_branch_entry::to"]
        [::std::mem::offset_of!(perf_branch_entry, to) - 8usize];
};
impl perf_branch_entry {
    #[inline]
    pub fn mispred(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mispred(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mispred_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mispred_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn predicted(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_predicted(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn predicted_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_predicted_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn in_tx(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_in_tx(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn in_tx_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                2usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_in_tx_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                2usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn abort(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_abort(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn abort_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                3usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_abort_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                3usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn cycles(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 16u8) as u64) }
    }
    #[inline]
    pub fn set_cycles(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn cycles_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                4usize,
                16u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_cycles_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                4usize,
                16u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn type_(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(20usize, 4u8) as u64) }
    }
    #[inline]
    pub fn set_type(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(20usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn type__raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                20usize,
                4u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_type_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                20usize,
                4u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn spec(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(24usize, 2u8) as u64) }
    }
    #[inline]
    pub fn set_spec(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn spec_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                24usize,
                2u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_spec_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                24usize,
                2u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_type(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(26usize, 4u8) as u64) }
    }
    #[inline]
    pub fn set_new_type(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(26usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn new_type_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                26usize,
                4u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_new_type_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                26usize,
                4u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn priv_(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(30usize, 3u8) as u64) }
    }
    #[inline]
    pub fn set_priv(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(30usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn priv__raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                30usize,
                3u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_priv_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                30usize,
                3u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn reserved(&self) -> __u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(33usize, 31u8) as u64) }
    }
    #[inline]
    pub fn set_reserved(&mut self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(33usize, 31u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn reserved_raw(this: *const Self) -> __u64 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                33usize,
                31u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_reserved_raw(this: *mut Self, val: __u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                33usize,
                31u8,
                val as u64,
            )
        }
    }
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn new_bitfield_1(
        mispred: __u64,
        predicted: __u64,
        in_tx: __u64,
        abort: __u64,
        cycles: __u64,
        type_: __u64,
        spec: __u64,
        new_type: __u64,
        priv_: __u64,
        reserved: __u64,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mispred: u64 = unsafe { ::std::mem::transmute(mispred) };
            mispred as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let predicted: u64 = unsafe { ::std::mem::transmute(predicted) };
            predicted as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let in_tx: u64 = unsafe { ::std::mem::transmute(in_tx) };
            in_tx as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let abort: u64 = unsafe { ::std::mem::transmute(abort) };
            abort as u64
        });
        __bindgen_bitfield_unit.set(4usize, 16u8, {
            let cycles: u64 = unsafe { ::std::mem::transmute(cycles) };
            cycles as u64
        });
        __bindgen_bitfield_unit.set(20usize, 4u8, {
            let type_: u64 = unsafe { ::std::mem::transmute(type_) };
            type_ as u64
        });
        __bindgen_bitfield_unit.set(24usize, 2u8, {
            let spec: u64 = unsafe { ::std::mem::transmute(spec) };
            spec as u64
        });
        __bindgen_bitfield_unit.set(26usize, 4u8, {
            let new_type: u64 = unsafe { ::std::mem::transmute(new_type) };
            new_type as u64
        });
        __bindgen_bitfield_unit.set(30usize, 3u8, {
            let priv_: u64 = unsafe { ::std::mem::transmute(priv_) };
            priv_ as u64
        });
        __bindgen_bitfield_unit.set(33usize, 31u8, {
            let reserved: u64 = unsafe { ::std::mem::transmute(reserved) };
            reserved as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_sample_weight {
    pub full: __u64,
    pub __bindgen_anon_1: perf_sample_weight__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
#[non_exhaustive]
pub struct perf_sample_weight__bindgen_ty_1 {
    pub var1_dw: __u32,
    pub var2_w: __u16,
    pub var3_w: __u16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of perf_sample_weight__bindgen_ty_1"]
        [::std::mem::size_of::<perf_sample_weight__bindgen_ty_1>() - 8usize];
    ["Alignment of perf_sample_weight__bindgen_ty_1"]
        [::std::mem::align_of::<perf_sample_weight__bindgen_ty_1>() - 4usize];
    ["Offset of field: perf_sample_weight__bindgen_ty_1::var1_dw"]
        [::std::mem::offset_of!(perf_sample_weight__bindgen_ty_1, var1_dw) - 0usize];
    ["Offset of field: perf_sample_weight__bindgen_ty_1::var2_w"]
        [::std::mem::offset_of!(perf_sample_weight__bindgen_ty_1, var2_w) - 4usize];
    ["Offset of field: perf_sample_weight__bindgen_ty_1::var3_w"]
        [::std::mem::offset_of!(perf_sample_weight__bindgen_ty_1, var3_w) - 6usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of perf_sample_weight"][::std::mem::size_of::<perf_sample_weight>() - 8usize];
    ["Alignment of perf_sample_weight"][::std::mem::align_of::<perf_sample_weight>() - 8usize];
    ["Offset of field: perf_sample_weight::full"]
        [::std::mem::offset_of!(perf_sample_weight, full) - 0usize];
};
impl Default for perf_sample_weight {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for perf_sample_weight {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "perf_sample_weight {{ union }}")
    }
}
pub const HW_BREAKPOINT_LEN_1: _bindgen_ty_7 = 1;
pub const HW_BREAKPOINT_LEN_2: _bindgen_ty_7 = 2;
pub const HW_BREAKPOINT_LEN_3: _bindgen_ty_7 = 3;
pub const HW_BREAKPOINT_LEN_4: _bindgen_ty_7 = 4;
pub const HW_BREAKPOINT_LEN_5: _bindgen_ty_7 = 5;
pub const HW_BREAKPOINT_LEN_6: _bindgen_ty_7 = 6;
pub const HW_BREAKPOINT_LEN_7: _bindgen_ty_7 = 7;
pub const HW_BREAKPOINT_LEN_8: _bindgen_ty_7 = 8;
pub type _bindgen_ty_7 = ::std::os::raw::c_uint;
pub const HW_BREAKPOINT_EMPTY: _bindgen_ty_8 = 0;
pub const HW_BREAKPOINT_R: _bindgen_ty_8 = 1;
pub const HW_BREAKPOINT_W: _bindgen_ty_8 = 2;
pub const HW_BREAKPOINT_RW: _bindgen_ty_8 = 3;
pub const HW_BREAKPOINT_X: _bindgen_ty_8 = 4;
pub const HW_BREAKPOINT_INVALID: _bindgen_ty_8 = 7;
pub type _bindgen_ty_8 = ::std::os::raw::c_uint;
pub const PERF_REG_X86_AX: perf_event_x86_regs = 0;
pub const PERF_REG_X86_BX: perf_event_x86_regs = 1;
pub const PERF_REG_X86_CX: perf_event_x86_regs = 2;
pub const PERF_REG_X86_DX: perf_event_x86_regs = 3;
pub const PERF_REG_X86_SI: perf_event_x86_regs = 4;
pub const PERF_REG_X86_DI: perf_event_x86_regs = 5;
pub const PERF_REG_X86_BP: perf_event_x86_regs = 6;
pub const PERF_REG_X86_SP: perf_event_x86_regs = 7;
pub const PERF_REG_X86_IP: perf_event_x86_regs = 8;
pub const PERF_REG_X86_FLAGS: perf_event_x86_regs = 9;
pub const PERF_REG_X86_CS: perf_event_x86_regs = 10;
pub const PERF_REG_X86_SS: perf_event_x86_regs = 11;
pub const PERF_REG_X86_DS: perf_event_x86_regs = 12;
pub const PERF_REG_X86_ES: perf_event_x86_regs = 13;
pub const PERF_REG_X86_FS: perf_event_x86_regs = 14;
pub const PERF_REG_X86_GS: perf_event_x86_regs = 15;
pub const PERF_REG_X86_R8: perf_event_x86_regs = 16;
pub const PERF_REG_X86_R9: perf_event_x86_regs = 17;
pub const PERF_REG_X86_R10: perf_event_x86_regs = 18;
pub const PERF_REG_X86_R11: perf_event_x86_regs = 19;
pub const PERF_REG_X86_R12: perf_event_x86_regs = 20;
pub const PERF_REG_X86_R13: perf_event_x86_regs = 21;
pub const PERF_REG_X86_R14: perf_event_x86_regs = 22;
pub const PERF_REG_X86_R15: perf_event_x86_regs = 23;
pub const PERF_REG_X86_32_MAX: perf_event_x86_regs = 16;
pub const PERF_REG_X86_64_MAX: perf_event_x86_regs = 24;
pub const PERF_REG_X86_XMM0: perf_event_x86_regs = 32;
pub const PERF_REG_X86_XMM1: perf_event_x86_regs = 34;
pub const PERF_REG_X86_XMM2: perf_event_x86_regs = 36;
pub const PERF_REG_X86_XMM3: perf_event_x86_regs = 38;
pub const PERF_REG_X86_XMM4: perf_event_x86_regs = 40;
pub const PERF_REG_X86_XMM5: perf_event_x86_regs = 42;
pub const PERF_REG_X86_XMM6: perf_event_x86_regs = 44;
pub const PERF_REG_X86_XMM7: perf_event_x86_regs = 46;
pub const PERF_REG_X86_XMM8: perf_event_x86_regs = 48;
pub const PERF_REG_X86_XMM9: perf_event_x86_regs = 50;
pub const PERF_REG_X86_XMM10: perf_event_x86_regs = 52;
pub const PERF_REG_X86_XMM11: perf_event_x86_regs = 54;
pub const PERF_REG_X86_XMM12: perf_event_x86_regs = 56;
pub const PERF_REG_X86_XMM13: perf_event_x86_regs = 58;
pub const PERF_REG_X86_XMM14: perf_event_x86_regs = 60;
pub const PERF_REG_X86_XMM15: perf_event_x86_regs = 62;
pub const PERF_REG_X86_XMM_MAX: perf_event_x86_regs = 64;
pub type perf_event_x86_regs = ::std::os::raw::c_uint;
pub const ENABLE: perf_event_ioctls = 9216;
pub const DISABLE: perf_event_ioctls = 9217;
pub const REFRESH: perf_event_ioctls = 9218;
pub const RESET: perf_event_ioctls = 9219;
pub const PERIOD: perf_event_ioctls = 1074275332;
pub const SET_OUTPUT: perf_event_ioctls = 9221;
pub const SET_FILTER: perf_event_ioctls = 1074275334;
pub const ID: perf_event_ioctls = 2148017159;
pub const SET_BPF: perf_event_ioctls = 1074013192;
pub const PAUSE_OUTPUT: perf_event_ioctls = 1074013193;
pub const QUERY_BPF: perf_event_ioctls = 3221758986;
pub const MODIFY_ATTRIBUTES: perf_event_ioctls = 1074275339;
pub type perf_event_ioctls = ::std::os::raw::c_uint;
