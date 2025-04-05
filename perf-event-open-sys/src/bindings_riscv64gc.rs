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
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
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
pub const __NR_perf_event_open: u32 = 241;
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
#[test]
fn bindgen_test_layout_perf_event_attr__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<perf_event_attr__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<perf_event_attr__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(perf_event_attr__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<perf_event_attr__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(perf_event_attr__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sample_period) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr__bindgen_ty_1),
            "::",
            stringify!(sample_period)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sample_freq) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr__bindgen_ty_1),
            "::",
            stringify!(sample_freq)
        )
    );
}
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
#[test]
fn bindgen_test_layout_perf_event_attr__bindgen_ty_2() {
    const UNINIT: ::std::mem::MaybeUninit<perf_event_attr__bindgen_ty_2> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<perf_event_attr__bindgen_ty_2>(),
        4usize,
        concat!("Size of: ", stringify!(perf_event_attr__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<perf_event_attr__bindgen_ty_2>(),
        4usize,
        concat!("Alignment of ", stringify!(perf_event_attr__bindgen_ty_2))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).wakeup_events) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr__bindgen_ty_2),
            "::",
            stringify!(wakeup_events)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).wakeup_watermark) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr__bindgen_ty_2),
            "::",
            stringify!(wakeup_watermark)
        )
    );
}
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
#[test]
fn bindgen_test_layout_perf_event_attr__bindgen_ty_3() {
    const UNINIT: ::std::mem::MaybeUninit<perf_event_attr__bindgen_ty_3> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<perf_event_attr__bindgen_ty_3>(),
        8usize,
        concat!("Size of: ", stringify!(perf_event_attr__bindgen_ty_3))
    );
    assert_eq!(
        ::std::mem::align_of::<perf_event_attr__bindgen_ty_3>(),
        8usize,
        concat!("Alignment of ", stringify!(perf_event_attr__bindgen_ty_3))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bp_addr) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr__bindgen_ty_3),
            "::",
            stringify!(bp_addr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).kprobe_func) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr__bindgen_ty_3),
            "::",
            stringify!(kprobe_func)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).uprobe_path) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr__bindgen_ty_3),
            "::",
            stringify!(uprobe_path)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).config1) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr__bindgen_ty_3),
            "::",
            stringify!(config1)
        )
    );
}
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
#[test]
fn bindgen_test_layout_perf_event_attr__bindgen_ty_4() {
    const UNINIT: ::std::mem::MaybeUninit<perf_event_attr__bindgen_ty_4> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<perf_event_attr__bindgen_ty_4>(),
        8usize,
        concat!("Size of: ", stringify!(perf_event_attr__bindgen_ty_4))
    );
    assert_eq!(
        ::std::mem::align_of::<perf_event_attr__bindgen_ty_4>(),
        8usize,
        concat!("Alignment of ", stringify!(perf_event_attr__bindgen_ty_4))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bp_len) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr__bindgen_ty_4),
            "::",
            stringify!(bp_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).kprobe_addr) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr__bindgen_ty_4),
            "::",
            stringify!(kprobe_addr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).probe_offset) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr__bindgen_ty_4),
            "::",
            stringify!(probe_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).config2) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr__bindgen_ty_4),
            "::",
            stringify!(config2)
        )
    );
}
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
pub struct perf_event_attr__bindgen_ty_5__bindgen_ty_1 {
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[test]
fn bindgen_test_layout_perf_event_attr__bindgen_ty_5__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<perf_event_attr__bindgen_ty_5__bindgen_ty_1>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(perf_event_attr__bindgen_ty_5__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<perf_event_attr__bindgen_ty_5__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(perf_event_attr__bindgen_ty_5__bindgen_ty_1)
        )
    );
}
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
    pub fn new_bitfield_1(
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
#[test]
fn bindgen_test_layout_perf_event_attr__bindgen_ty_5() {
    const UNINIT: ::std::mem::MaybeUninit<perf_event_attr__bindgen_ty_5> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<perf_event_attr__bindgen_ty_5>(),
        4usize,
        concat!("Size of: ", stringify!(perf_event_attr__bindgen_ty_5))
    );
    assert_eq!(
        ::std::mem::align_of::<perf_event_attr__bindgen_ty_5>(),
        4usize,
        concat!("Alignment of ", stringify!(perf_event_attr__bindgen_ty_5))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).aux_action) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr__bindgen_ty_5),
            "::",
            stringify!(aux_action)
        )
    );
}
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
#[test]
fn bindgen_test_layout_perf_event_attr() {
    const UNINIT: ::std::mem::MaybeUninit<perf_event_attr> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<perf_event_attr>(),
        136usize,
        concat!("Size of: ", stringify!(perf_event_attr))
    );
    assert_eq!(
        ::std::mem::align_of::<perf_event_attr>(),
        8usize,
        concat!("Alignment of ", stringify!(perf_event_attr))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).size) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).config) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr),
            "::",
            stringify!(config)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sample_type) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr),
            "::",
            stringify!(sample_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).read_format) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr),
            "::",
            stringify!(read_format)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bp_type) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr),
            "::",
            stringify!(bp_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).branch_sample_type) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr),
            "::",
            stringify!(branch_sample_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sample_regs_user) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr),
            "::",
            stringify!(sample_regs_user)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sample_stack_user) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr),
            "::",
            stringify!(sample_stack_user)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).clockid) as usize - ptr as usize },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr),
            "::",
            stringify!(clockid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sample_regs_intr) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr),
            "::",
            stringify!(sample_regs_intr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).aux_watermark) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr),
            "::",
            stringify!(aux_watermark)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sample_max_stack) as usize - ptr as usize },
        108usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr),
            "::",
            stringify!(sample_max_stack)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__reserved_2) as usize - ptr as usize },
        110usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr),
            "::",
            stringify!(__reserved_2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).aux_sample_size) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr),
            "::",
            stringify!(aux_sample_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sig_data) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr),
            "::",
            stringify!(sig_data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).config3) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr),
            "::",
            stringify!(config3)
        )
    );
}
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
    pub fn new_bitfield_1(
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
pub struct perf_event_query_bpf {
    pub ids_len: __u32,
    pub prog_cnt: __u32,
    pub ids: __IncompleteArrayField<__u32>,
}
#[test]
fn bindgen_test_layout_perf_event_query_bpf() {
    const UNINIT: ::std::mem::MaybeUninit<perf_event_query_bpf> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<perf_event_query_bpf>(),
        8usize,
        concat!("Size of: ", stringify!(perf_event_query_bpf))
    );
    assert_eq!(
        ::std::mem::align_of::<perf_event_query_bpf>(),
        4usize,
        concat!("Alignment of ", stringify!(perf_event_query_bpf))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ids_len) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_query_bpf),
            "::",
            stringify!(ids_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prog_cnt) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_query_bpf),
            "::",
            stringify!(prog_cnt)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ids) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_query_bpf),
            "::",
            stringify!(ids)
        )
    );
}
pub const PERF_IOC_FLAG_GROUP: perf_event_ioc_flags = 1;
pub type perf_event_ioc_flags = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
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
pub struct perf_event_mmap_page__bindgen_ty_1__bindgen_ty_1 {
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[test]
fn bindgen_test_layout_perf_event_mmap_page__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<perf_event_mmap_page__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(perf_event_mmap_page__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<perf_event_mmap_page__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(perf_event_mmap_page__bindgen_ty_1__bindgen_ty_1)
        )
    );
}
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
    pub fn new_bitfield_1(
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
#[test]
fn bindgen_test_layout_perf_event_mmap_page__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<perf_event_mmap_page__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<perf_event_mmap_page__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(perf_event_mmap_page__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<perf_event_mmap_page__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(perf_event_mmap_page__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).capabilities) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page__bindgen_ty_1),
            "::",
            stringify!(capabilities)
        )
    );
}
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
#[test]
fn bindgen_test_layout_perf_event_mmap_page() {
    const UNINIT: ::std::mem::MaybeUninit<perf_event_mmap_page> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<perf_event_mmap_page>(),
        1088usize,
        concat!("Size of: ", stringify!(perf_event_mmap_page))
    );
    assert_eq!(
        ::std::mem::align_of::<perf_event_mmap_page>(),
        8usize,
        concat!("Alignment of ", stringify!(perf_event_mmap_page))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).version) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).compat_version) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(compat_version)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lock) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(lock)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(index)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).offset) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time_enabled) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(time_enabled)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time_running) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(time_running)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pmc_width) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(pmc_width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time_shift) as usize - ptr as usize },
        50usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(time_shift)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time_mult) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(time_mult)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time_offset) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(time_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time_zero) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(time_zero)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).size) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__reserved_1) as usize - ptr as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(__reserved_1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time_cycles) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(time_cycles)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time_mask) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(time_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__reserved) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(__reserved)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data_head) as usize - ptr as usize },
        1024usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(data_head)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data_tail) as usize - ptr as usize },
        1032usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(data_tail)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data_offset) as usize - ptr as usize },
        1040usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(data_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data_size) as usize - ptr as usize },
        1048usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(data_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).aux_head) as usize - ptr as usize },
        1056usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(aux_head)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).aux_tail) as usize - ptr as usize },
        1064usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(aux_tail)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).aux_offset) as usize - ptr as usize },
        1072usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(aux_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).aux_size) as usize - ptr as usize },
        1080usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_mmap_page),
            "::",
            stringify!(aux_size)
        )
    );
}
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
pub struct perf_event_header {
    pub type_: __u32,
    pub misc: __u16,
    pub size: __u16,
}
#[test]
fn bindgen_test_layout_perf_event_header() {
    const UNINIT: ::std::mem::MaybeUninit<perf_event_header> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<perf_event_header>(),
        8usize,
        concat!("Size of: ", stringify!(perf_event_header))
    );
    assert_eq!(
        ::std::mem::align_of::<perf_event_header>(),
        4usize,
        concat!("Alignment of ", stringify!(perf_event_header))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_header),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).misc) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_header),
            "::",
            stringify!(misc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).size) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_header),
            "::",
            stringify!(size)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct perf_ns_link_info {
    pub dev: __u64,
    pub ino: __u64,
}
#[test]
fn bindgen_test_layout_perf_ns_link_info() {
    const UNINIT: ::std::mem::MaybeUninit<perf_ns_link_info> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<perf_ns_link_info>(),
        16usize,
        concat!("Size of: ", stringify!(perf_ns_link_info))
    );
    assert_eq!(
        ::std::mem::align_of::<perf_ns_link_info>(),
        8usize,
        concat!("Alignment of ", stringify!(perf_ns_link_info))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dev) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_ns_link_info),
            "::",
            stringify!(dev)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ino) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_ns_link_info),
            "::",
            stringify!(ino)
        )
    );
}
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
pub struct perf_mem_data_src__bindgen_ty_1 {
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[test]
fn bindgen_test_layout_perf_mem_data_src__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<perf_mem_data_src__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(perf_mem_data_src__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<perf_mem_data_src__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(perf_mem_data_src__bindgen_ty_1))
    );
}
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
    pub fn new_bitfield_1(
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
#[test]
fn bindgen_test_layout_perf_mem_data_src() {
    const UNINIT: ::std::mem::MaybeUninit<perf_mem_data_src> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<perf_mem_data_src>(),
        8usize,
        concat!("Size of: ", stringify!(perf_mem_data_src))
    );
    assert_eq!(
        ::std::mem::align_of::<perf_mem_data_src>(),
        8usize,
        concat!("Alignment of ", stringify!(perf_mem_data_src))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).val) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_mem_data_src),
            "::",
            stringify!(val)
        )
    );
}
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
pub struct perf_branch_entry {
    pub from: __u64,
    pub to: __u64,
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[test]
fn bindgen_test_layout_perf_branch_entry() {
    const UNINIT: ::std::mem::MaybeUninit<perf_branch_entry> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<perf_branch_entry>(),
        24usize,
        concat!("Size of: ", stringify!(perf_branch_entry))
    );
    assert_eq!(
        ::std::mem::align_of::<perf_branch_entry>(),
        8usize,
        concat!("Alignment of ", stringify!(perf_branch_entry))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).from) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_branch_entry),
            "::",
            stringify!(from)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).to) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_branch_entry),
            "::",
            stringify!(to)
        )
    );
}
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
    pub fn new_bitfield_1(
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
pub struct perf_sample_weight__bindgen_ty_1 {
    pub var1_dw: __u32,
    pub var2_w: __u16,
    pub var3_w: __u16,
}
#[test]
fn bindgen_test_layout_perf_sample_weight__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<perf_sample_weight__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<perf_sample_weight__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(perf_sample_weight__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<perf_sample_weight__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(perf_sample_weight__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).var1_dw) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_sample_weight__bindgen_ty_1),
            "::",
            stringify!(var1_dw)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).var2_w) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_sample_weight__bindgen_ty_1),
            "::",
            stringify!(var2_w)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).var3_w) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_sample_weight__bindgen_ty_1),
            "::",
            stringify!(var3_w)
        )
    );
}
#[test]
fn bindgen_test_layout_perf_sample_weight() {
    const UNINIT: ::std::mem::MaybeUninit<perf_sample_weight> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<perf_sample_weight>(),
        8usize,
        concat!("Size of: ", stringify!(perf_sample_weight))
    );
    assert_eq!(
        ::std::mem::align_of::<perf_sample_weight>(),
        8usize,
        concat!("Alignment of ", stringify!(perf_sample_weight))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).full) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_sample_weight),
            "::",
            stringify!(full)
        )
    );
}
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
pub const PERF_REG_RISCV_PC: perf_event_riscv_regs = 0;
pub const PERF_REG_RISCV_RA: perf_event_riscv_regs = 1;
pub const PERF_REG_RISCV_SP: perf_event_riscv_regs = 2;
pub const PERF_REG_RISCV_GP: perf_event_riscv_regs = 3;
pub const PERF_REG_RISCV_TP: perf_event_riscv_regs = 4;
pub const PERF_REG_RISCV_T0: perf_event_riscv_regs = 5;
pub const PERF_REG_RISCV_T1: perf_event_riscv_regs = 6;
pub const PERF_REG_RISCV_T2: perf_event_riscv_regs = 7;
pub const PERF_REG_RISCV_S0: perf_event_riscv_regs = 8;
pub const PERF_REG_RISCV_S1: perf_event_riscv_regs = 9;
pub const PERF_REG_RISCV_A0: perf_event_riscv_regs = 10;
pub const PERF_REG_RISCV_A1: perf_event_riscv_regs = 11;
pub const PERF_REG_RISCV_A2: perf_event_riscv_regs = 12;
pub const PERF_REG_RISCV_A3: perf_event_riscv_regs = 13;
pub const PERF_REG_RISCV_A4: perf_event_riscv_regs = 14;
pub const PERF_REG_RISCV_A5: perf_event_riscv_regs = 15;
pub const PERF_REG_RISCV_A6: perf_event_riscv_regs = 16;
pub const PERF_REG_RISCV_A7: perf_event_riscv_regs = 17;
pub const PERF_REG_RISCV_S2: perf_event_riscv_regs = 18;
pub const PERF_REG_RISCV_S3: perf_event_riscv_regs = 19;
pub const PERF_REG_RISCV_S4: perf_event_riscv_regs = 20;
pub const PERF_REG_RISCV_S5: perf_event_riscv_regs = 21;
pub const PERF_REG_RISCV_S6: perf_event_riscv_regs = 22;
pub const PERF_REG_RISCV_S7: perf_event_riscv_regs = 23;
pub const PERF_REG_RISCV_S8: perf_event_riscv_regs = 24;
pub const PERF_REG_RISCV_S9: perf_event_riscv_regs = 25;
pub const PERF_REG_RISCV_S10: perf_event_riscv_regs = 26;
pub const PERF_REG_RISCV_S11: perf_event_riscv_regs = 27;
pub const PERF_REG_RISCV_T3: perf_event_riscv_regs = 28;
pub const PERF_REG_RISCV_T4: perf_event_riscv_regs = 29;
pub const PERF_REG_RISCV_T5: perf_event_riscv_regs = 30;
pub const PERF_REG_RISCV_T6: perf_event_riscv_regs = 31;
pub const PERF_REG_RISCV_MAX: perf_event_riscv_regs = 32;
pub type perf_event_riscv_regs = ::std::os::raw::c_uint;
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
