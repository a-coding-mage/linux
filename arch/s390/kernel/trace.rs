// SPDX-License-Identifier: GPL-2.0
/*
 * Tracepoint definitions for s390
 *
 * Copyright IBM Corp. 2015
 * Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

// Dependency intent from <linux/percpu.h> and <asm/trace/diag.h>.

// CREATE_TRACE_POINTS
// EXPORT_TRACEPOINT_SYMBOL(s390_diagnose);

extern "C" {
    fn trace_s390_diagnose(diag_nr: core::ffi::c_int);
    fn local_irq_save(flags: *mut core::ffi::c_ulong);
    fn local_irq_restore(flags: core::ffi::c_ulong);
}

// DEFINE_PER_CPU(unsigned int, diagnose_trace_depth);
static mut DIAGNOSE_TRACE_DEPTH: core::ffi::c_uint = 0;

// The CONFIG_LOCKDEP build-time condition is supplied by the surrounding build.
#[inline(never)]
pub unsafe fn trace_s390_diagnose_norecursion(diag_nr: core::ffi::c_int) {
    let mut flags: core::ffi::c_ulong = 0;
    let depth: *mut core::ffi::c_uint = &raw mut DIAGNOSE_TRACE_DEPTH;

    /* Avoid lockdep recursion. */
    if cfg!(feature = "CONFIG_LOCKDEP") {
        return;
    }
    local_irq_save(&mut flags as *mut core::ffi::c_ulong);
    if *depth == 0 {
        *depth = (*depth).wrapping_add(1);
        trace_s390_diagnose(diag_nr);
        *depth = (*depth).wrapping_sub(1);
    }
    local_irq_restore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
