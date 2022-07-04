# Release notes for `perf-event-open-sys`

## 3.0.0

-   Based on Linux kernel headers packaged by Fedora as `kernel-headers-5.18.4-200.fc36`.

-   Fix build for Android, x86_64-unknown-linux-musl.

-   Remove redundant prefixes from `bindings` constants derived from enums in
    the Linux kernel headers.
    
    For example, the kernel headers have the definition:
  
        /*
         * attr.type
         */
        enum perf_type_id {
            PERF_TYPE_HARDWARE			= 0,
            PERF_TYPE_SOFTWARE			= 1,
            PERF_TYPE_TRACEPOINT		= 2,
            ...
        };
  
    This crate used to render the above as constants like this:
  
        pub const perf_type_id_PERF_TYPE_HARDWARE: perf_type_id = 0;
        pub const perf_type_id_PERF_TYPE_SOFTWARE: perf_type_id = 1;
        pub const perf_type_id_PERF_TYPE_TRACEPOINT: perf_type_id = 2;
        ...

    The names incorporate the names of both the C enum and its constants. But
    since the constants' names are already prefixed (necessary because C places
    enumeration constants in the 'ordinary identifier' namespace), this is
    redundant.
    
    In v3.0.0, these constants are rendered in Rust like this:
    
        pub const PERF_TYPE_HARDWARE: perf_type_id = 0;
        pub const PERF_TYPE_SOFTWARE: perf_type_id = 1;
        pub const PERF_TYPE_TRACEPOINT: perf_type_id = 2;

    Here's the full list of prefixes that were stripped, in case you want to
    `sed` your way through a conversion:
    
        bp_type_idx_
        perf_bpf_event_type_
        perf_branch_sample_type_
        perf_branch_sample_type_shift_
        perf_callchain_context_
        perf_event_ioc_flags_
        perf_event_ioctls_
        perf_event_read_format_
        perf_event_sample_format_
        perf_event_type_
        perf_hw_cache_id_
        perf_hw_cache_op_result_id_
        perf_hw_id_
        perf_record_ksymbol_type_
        perf_sample_regs_abi_
        perf_sw_ids_
        perf_type_id_

