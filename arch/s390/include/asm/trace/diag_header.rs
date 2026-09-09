/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Tracepoint header for s390 diagnose calls
 *
 * Copyright IBM Corp. 2015
 * Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

/* TRACE_SYSTEM s390 */
/* TRACE_INCLUDE_PATH asm/trace */
/* TRACE_INCLUDE_FILE diag */

#[repr(C)]
pub struct S390DiagnoseTraceEntry {
    pub nr: u16,
}

/* TRACE_EVENT(s390_diagnose): TP_PROTO(unsigned short nr), TP_ARGS(nr),
 * TP_STRUCT__entry(__field(unsigned short, nr)),
 * TP_fast_assign(__entry->nr = nr), TP_printk("nr=0x%x", __entry->nr)
 */

#[cfg(feature = "CONFIG_TRACEPOINTS")]
unsafe extern "C" {
    pub fn trace_s390_diagnose_norecursion(diag_nr: i32);
}

#[cfg(not(feature = "CONFIG_TRACEPOINTS"))]
#[inline]
pub fn trace_s390_diagnose_norecursion(_diag_nr: i32) {}

/* trace/define_trace.h is intentionally not included in Rust. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
