/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/tracepoint.h, linux/tsm-mr.h, and trace/define_trace.h.

// The trace-event declarations below intentionally retain the kernel trace
// framework's externally supplied `trace_event!` interface.

trace_event!(tsm_mr_read {
    proto: (mr: *const tsm_measurement_register),
    args: (mr),
    entry {
        string(mr, (*mr).mr_name),
        string(hash, if (*mr).mr_flags & TSM_MR_F_NOHASH != 0 {
            "data"
        } else {
            hash_algo_name[(*mr).mr_hash]
        }),
        dynamic_array(u8, d, (*mr).mr_size),
    },
    fast_assign: unsafe {
        assign_str!(mr);
        assign_str!(hash);
        memcpy(
            get_dynamic_array!(d),
            (*mr).mr_value,
            get_dynamic_array_len!(d),
        );
    },
    print: "[%s] %s:%s",
    print_args: (
        get_str!(mr),
        get_str!(hash),
        print_hex_str!(get_dynamic_array!(d), get_dynamic_array_len!(d)),
    ),
});

trace_event!(tsm_mr_refresh {
    proto: (mr: *const tsm_measurement_register, rc: core::ffi::c_int),
    args: (mr, rc),
    entry {
        string(mr, (*mr).mr_name),
        field(core::ffi::c_int, rc),
    },
    fast_assign: unsafe {
        assign_str!(mr);
        entry.rc = rc;
    },
    print: "[%s] %s:%d",
    print_args: (
        get_str!(mr),
        if entry.rc != 0 { "failed" } else { "succeeded" },
        entry.rc,
    ),
});

trace_event!(tsm_mr_write {
    proto: (mr: *const tsm_measurement_register, data: *const u8),
    args: (mr, data),
    entry {
        string(mr, (*mr).mr_name),
        string(hash, if (*mr).mr_flags & TSM_MR_F_NOHASH != 0 {
            "data"
        } else {
            hash_algo_name[(*mr).mr_hash]
        }),
        dynamic_array(u8, d, (*mr).mr_size),
    },
    fast_assign: unsafe {
        assign_str!(mr);
        assign_str!(hash);
        memcpy(
            get_dynamic_array!(d),
            data,
            get_dynamic_array_len!(d),
        );
    },
    print: "[%s] %s:%s",
    print_args: (
        get_str!(mr),
        get_str!(hash),
        print_hex_str!(get_dynamic_array!(d), get_dynamic_array_len!(d)),
    ),
});

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
