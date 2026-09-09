/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Tracepoint header for hiperdispatch
 *
 * Copyright IBM Corp. 2024
 */

//! Rust translation of the s390 hiperdispatch tracepoint header.
//!
//! The C tracepoint framework declarations are represented here as the
//! corresponding event entry layouts and tracepoint interfaces.  The
//! definitions supplied by Linux's tracepoint headers remain external.

// C: TRACE_SYSTEM s390
// C include dependency: <linux/tracepoint.h>
// C trace include path/file: asm/trace/hiperdispatch

#[repr(C)]
#[derive(Copy, Clone)]
pub struct S390HdWorkFnEntry {
    pub steal_time_percentage: core::ffi::c_int,
    pub entitled_core_count: core::ffi::c_int,
    pub highcap_core_count: core::ffi::c_int,
}

/// Tracepoint `s390_hd_work_fn`.
///
/// C format string: "steal: %d entitled_core_count: %d highcap_core_count: %d"
pub unsafe fn s390_hd_work_fn(
    steal_time_percentage: core::ffi::c_int,
    entitled_core_count: core::ffi::c_int,
    highcap_core_count: core::ffi::c_int,
) {
    let _entry = S390HdWorkFnEntry {
        steal_time_percentage,
        entitled_core_count,
        highcap_core_count,
    };
    // The Linux tracepoint backend consumes the entry and emits the event.
    // Its implementation is an external dependency of this header.
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct S390HdRebuildDomainsEntry {
    pub current_highcap_core_count: core::ffi::c_int,
    pub new_highcap_core_count: core::ffi::c_int,
}

/// Tracepoint `s390_hd_rebuild_domains`.
///
/// C format string: "change highcap_core_count: %u -> %u"
pub unsafe fn s390_hd_rebuild_domains(
    current_highcap_core_count: core::ffi::c_int,
    new_highcap_core_count: core::ffi::c_int,
) {
    let _entry = S390HdRebuildDomainsEntry {
        current_highcap_core_count,
        new_highcap_core_count,
    };
    // The Linux tracepoint backend consumes the entry and emits the event.
    // Its implementation is an external dependency of this header.
}

// C: <trace/define_trace.h> is intentionally left as an external tracepoint
// definition dependency; it has no standalone Rust item in this translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
