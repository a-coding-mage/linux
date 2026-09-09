/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2026 Breno Leitao <leitao@debian.org>
 */

// TRACE_SYSTEM coredump
// The Linux scheduler and tracepoint definitions are supplied by other files.

/// coredump - called when a coredump starts
/// @sig: signal number that triggered the coredump
///
/// This tracepoint fires at the beginning of a coredump attempt,
/// providing a stable interface for monitoring coredump events.
#[repr(C)]
pub struct CoredumpEntry {
    pub sig: ::core::ffi::c_int,
    pub comm: [::core::ffi::c_char; TASK_COMM_LEN],
}

// TP_PROTO(int sig)
// TP_ARGS(sig)
// TP_STRUCT__entry(__field(int, sig) __array(char, comm, TASK_COMM_LEN))

/// Assign the fields of a coredump tracepoint entry.
///
/// This is the Rust equivalent of the TP_fast_assign block. `current_comm`
/// must point to at least `TASK_COMM_LEN` bytes, as does `entry.comm`.
#[inline]
pub unsafe fn coredump_fast_assign(
    entry: *mut CoredumpEntry,
    sig: ::core::ffi::c_int,
    current_comm: *const ::core::ffi::c_char,
) {
    (*entry).sig = sig;
    ::core::ptr::copy_nonoverlapping(
        current_comm,
        (*entry).comm.as_mut_ptr(),
        TASK_COMM_LEN,
    );
}

// TP_printk("sig=%d comm=%s", __entry->sig, __entry->comm)

// The generated tracepoint registration and definition are provided by
// trace/define_trace.h in the C build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
