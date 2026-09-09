/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM ipi
// The C tracepoint definitions below are represented as Rust declarations.
// Their registration and emission are provided by the tracepoint subsystem.

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct IpiSendCpuEntry {
    pub cpu: u32,
    pub callsite: *mut c_void,
    pub callback: *mut c_void,
}

#[repr(C)]
pub struct IpiSendCpumaskEntry {
    // __cpumask(cpumask)
    pub cpumask: *mut c_void,
    pub callsite: *mut c_void,
    pub callback: *mut c_void,
}

/// ipi_send_cpu(cpu, callsite, callback)
///
/// C tracepoint:
/// `cpu=%u callsite=%pS callback=%pS`
#[inline]
pub unsafe fn trace_ipi_send_cpu(
    cpu: u32,
    callsite: usize,
    callback: *mut c_void,
) {
    let _entry = IpiSendCpuEntry {
        cpu,
        callsite: callsite as *mut c_void,
        callback,
    };
    // The external tracepoint implementation consumes `_entry`.
}

/// ipi_send_cpumask(cpumask, callsite, callback)
///
/// C tracepoint:
/// `cpumask=%s callsite=%pS callback=%pS`
#[inline]
pub unsafe fn trace_ipi_send_cpumask(
    cpumask: *const c_void,
    callsite: usize,
    callback: *mut c_void,
) {
    let _entry = IpiSendCpumaskEntry {
        cpumask: cpumask as *mut c_void,
        callsite: callsite as *mut c_void,
        callback,
    };
    // The external tracepoint implementation consumes `_entry`.
}

// CONFIG_HAVE_EXTRA_IPI_TRACEPOINTS

#[repr(C)]
pub struct IpiRaiseEntry {
    // __cpumask(target_cpus)
    pub target_cpus: *mut c_void,
    pub reason: *const c_char,
}

#[repr(C)]
pub struct IpiHandlerEntry {
    pub reason: *const c_char,
}

/**
 * ipi_raise - called when a smp cross call is made
 *
 * @mask: mask of recipient CPUs for the IPI
 * @reason: string identifying the IPI purpose
 *
 * It is necessary for @reason to be a static string declared with
 * __tracepoint_string.
 */
#[inline]
pub unsafe fn trace_ipi_raise(mask: *const c_void, reason: *const c_char) {
    let _entry = IpiRaiseEntry {
        target_cpus: mask as *mut c_void,
        reason,
    };
    // The external tracepoint implementation consumes `_entry`.
}

#[inline]
pub unsafe fn trace_ipi_handler(reason: *const c_char) {
    let _entry = IpiHandlerEntry { reason };
    // The external tracepoint implementation consumes `_entry`.
}

/**
 * ipi_entry - called immediately before the IPI handler
 *
 * @reason: string identifying the IPI purpose
 *
 * It is necessary for @reason to be a static string declared with
 * __tracepoint_string, ideally the same as used with trace_ipi_raise
 * for that IPI.
 */
#[inline]
pub unsafe fn trace_ipi_entry(reason: *const c_char) {
    trace_ipi_handler(reason);
}

/**
 * ipi_exit - called immediately after the IPI handler returns
 *
 * @reason: string identifying the IPI purpose
 *
 * It is necessary for @reason to be a static string declared with
 * __tracepoint_string, ideally the same as used with trace_ipi_raise for
 * that IPI.
 */
#[inline]
pub unsafe fn trace_ipi_exit(reason: *const c_char) {
    trace_ipi_handler(reason);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
