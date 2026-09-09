/* SPDX-License-Identifier: GPL-2.0 */

// C preprocessor state:
// #undef TRACE_SYSTEM
// #define TRACE_SYSTEM bpf_trace
// #if !defined(_TRACE_BPF_TRACE_H) || defined(TRACE_HEADER_MULTI_READ)
// #define _TRACE_BPF_TRACE_H
//
// The C header includes <linux/tracepoint.h>.  Its TRACE_EVENT machinery is
// supplied by that dependency and is intentionally not reimplemented here.

use core::ffi::c_char;

/// Payload of the `bpf_trace_printk` trace event.
///
/// In C this is declared by `TP_STRUCT__entry(__string(bpf_string,
/// bpf_string))`; the tracepoint implementation owns the storage for the
/// copied string.  The source argument is a pointer to a NUL-terminated C
/// string and is retained here with the same pointer semantics.
#[repr(C)]
pub struct BpfTracePrintk {
    pub bpf_string: *const c_char,
}

// TRACE_EVENT(bpf_trace_printk,
//
//     TP_PROTO(const char *bpf_string),
//
//     TP_ARGS(bpf_string),
//
//     TP_STRUCT__entry(
//         __string(bpf_string, bpf_string)
//     ),
//
//     TP_fast_assign(
//         __assign_str(bpf_string);
//     ),
//
//     TP_printk("%s", __get_str(bpf_string))
// );

// #endif /* _TRACE_BPF_TRACE_H */

// #undef TRACE_INCLUDE_PATH
// #define TRACE_INCLUDE_PATH .
// #define TRACE_INCLUDE_FILE bpf_trace
// #include <trace/define_trace.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
