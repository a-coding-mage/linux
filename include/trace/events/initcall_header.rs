/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM is initcall.
// This header is protected by _TRACE_INITCALL_H, except for multi-read use.
// The original includes linux/tracepoint.h; its trace-event declarations are
// supplied by the surrounding translation unit.

// TRACE_EVENT(initcall_level,
//
//     TP_PROTO(const char *level),
//     TP_ARGS(level),
//
//     TP_STRUCT__entry(
//         __string(level, level)
//     ),
//
//     TP_fast_assign(
//         __assign_str(level);
//     ),
//
//     TP_printk("level=%s", __get_str(level))
// );
trace_event!(initcall_level {
    proto: unsafe extern "C" fn(level: *const core::ffi::c_char),
    args: (level),
    entry: {
        string level;
    },
    assign: {
        assign_str!(level);
    },
    print: ("level=%s", get_str!(level)),
});

// TRACE_EVENT(initcall_start,
//
//     TP_PROTO(initcall_t func),
//     TP_ARGS(func),
//
//     TP_STRUCT__entry(
//         /*
//          * Use field_struct to avoid is_signed_type()
//          * comparison of a function pointer
//          */
//         __field_struct(initcall_t, func)
//     ),
//
//     TP_fast_assign(
//         __entry->func = func;
//     ),
//
//     TP_printk("func=%pS", __entry->func)
// );
trace_event!(initcall_start {
    proto: unsafe extern "C" fn(func: initcall_t),
    args: (func),
    entry: {
        field_struct initcall_t func;
    },
    assign: {
        entry.func = func;
    },
    print: ("func=%pS", entry.func),
});

// TRACE_EVENT(initcall_finish,
//
//     TP_PROTO(initcall_t func, int ret),
//     TP_ARGS(func, ret),
//
//     TP_STRUCT__entry(
//         /*
//          * Use field_struct to avoid is_signed_type()
//          * comparison of a function pointer
//          */
//         __field_struct(initcall_t, func)
//         __field(int, ret)
//     ),
//
//     TP_fast_assign(
//         __entry->func = func;
//         __entry->ret = ret;
//     ),
//
//     TP_printk("func=%pS ret=%d", __entry->func, __entry->ret)
// );
trace_event!(initcall_finish {
    proto: unsafe extern "C" fn(func: initcall_t, ret: core::ffi::c_int),
    args: (func, ret),
    entry: {
        field_struct initcall_t func;
        field core::ffi::c_int ret;
    },
    assign: {
        entry.func = func;
        entry.ret = ret;
    },
    print: ("func=%pS ret=%d", entry.func, entry.ret),
});

// The original trace/define_trace.h include is intentionally outside the
// header guard and is provided by the surrounding translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
