/* SPDX-License-Identifier: GPL-2.0 */

// C preprocessor state:
// #undef TRACE_SYSTEM
// #define TRACE_SYSTEM vsyscall
//
// Header guard:
// #if !defined(__VSYSCALL_TRACE_H) || defined(TRACE_HEADER_MULTI_READ)
// #define __VSYSCALL_TRACE_H

use core::ffi::c_int;

// The C source includes <linux/tracepoint.h>.  Tracepoint registration and
// formatting are supplied by that external dependency.

/// Data carried by the `emulate_vsyscall` trace event.
#[repr(C)]
pub struct EmulateVsyscallEntry {
    pub nr: c_int,
}

// C declaration translated from:
// TRACE_EVENT(emulate_vsyscall,
//     TP_PROTO(int nr),
//     TP_ARGS(nr),
//     TP_STRUCT__entry(__field(int, nr)),
//     TP_fast_assign(__entry->nr = nr;),
//     TP_printk("nr = %d", __entry->nr)
// );
//
// The generated tracepoint interface is provided by the external tracepoint
// dependency; its payload layout is represented above.

// #endif

// #undef TRACE_INCLUDE_PATH
// #define TRACE_INCLUDE_PATH ../../arch/x86/entry/vsyscall/
// #define TRACE_INCLUDE_FILE vsyscall_trace
// #include <trace/define_trace.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
