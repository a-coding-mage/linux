/* SPDX-License-Identifier: GPL-2.0 */

// Translated from trace/events/csd.h.  The C tracepoint declaration machinery
// is supplied by the tracing subsystem; the following payloads preserve the
// fields and their C layout.

use core::ffi::c_void;

#[repr(C)]
pub struct CsdQueueCpuEntry {
    pub cpu: u32,
    pub callsite: *mut c_void,
    pub func: *mut c_void,
    pub csd: *mut c_void,
}

/// Tracepoint: csd_queue_cpu.
///
/// C prototype: (const unsigned int cpu, unsigned long callsite,
/// smp_call_func_t func, call_single_data_t *csd)
#[inline(always)]
pub unsafe fn csd_queue_cpu_entry(
    cpu: u32,
    callsite: usize,
    func: *mut c_void,
    csd: *mut c_void,
) -> CsdQueueCpuEntry {
    CsdQueueCpuEntry {
        cpu,
        callsite: callsite as *mut c_void,
        func,
        csd,
    }
}

/*
 * Tracepoints for a function which is called as an effect of
 * smp_call_function.*
 */
#[repr(C)]
pub struct CsdFunctionEntry {
    pub func: *mut c_void,
    pub csd: *mut c_void,
}

/// Tracepoint event class: csd_function.
#[inline(always)]
pub unsafe fn csd_function_entry(func: *mut c_void, csd: *mut c_void) -> CsdFunctionEntry {
    CsdFunctionEntry { func, csd }
}

/// Tracepoint event: csd_function_entry.
#[inline(always)]
pub unsafe fn csd_function_entry_event(func: *mut c_void, csd: *mut c_void) -> CsdFunctionEntry {
    CsdFunctionEntry { func, csd }
}

/// Tracepoint event: csd_function_exit.
#[inline(always)]
pub unsafe fn csd_function_exit(func: *mut c_void, csd: *mut c_void) -> CsdFunctionEntry {
    CsdFunctionEntry { func, csd }
}

// The C header includes <trace/define_trace.h> outside the include guard so
// that the tracing subsystem can emit the tracepoint definitions.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
