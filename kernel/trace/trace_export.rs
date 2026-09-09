// SPDX-License-Identifier: GPL-2.0
/*
 * trace_export.c - export basic ftrace utilities to user space
 *
 * Copyright (C) 2009 Steven Rostedt <srostedt@redhat.com>
 */

// The C source includes Linux headers and trace_entries.h.  Their declarations
// and macro-generated items are supplied by the surrounding translation unit.

/* Stub function for events with triggers */
#[allow(non_camel_case_types, unused_variables)]
unsafe fn ftrace_event_register(
    _call: *mut trace_event_call,
    _type: trace_reg,
    _data: *mut core::ffi::c_void,
) -> i32 {
    0
}

// TRACE_SYSTEM is ftrace in the original source.

/*
 * FTRACE_ENTRY_REG allows ftrace entries to define a register function and
 * thus become accessible via perf.  The following declarations are generated
 * by trace_entries.h in the C source and remain external to this file.
 */

#[allow(non_camel_case_types)]
type trace_reg = i32;

#[repr(C)]
pub struct trace_event_call {
    _private: [u8; 0],
}

extern "C" {
    static mut event_function: trace_event_call;
}

/// Corresponds to `ftrace_event_is_function` in the C source.
pub unsafe fn ftrace_event_is_function(call: *mut trace_event_call) -> bool {
    core::ptr::eq(call, core::ptr::addr_of_mut!(event_function))
}

/*
 * The C file invokes trace_entries.h several times with different macro
 * definitions.  Those invocations generate the ftrace event structures,
 * field arrays, event classes, event calls, and registration pointers.  Rust
 * cannot reproduce a C preprocessor include without the supplied entry
 * definitions; the generated declarations are therefore intentionally left
 * to the corresponding external translation unit.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
