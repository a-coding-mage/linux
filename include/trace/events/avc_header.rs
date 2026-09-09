/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Authors: Thiébaud Weksteen <tweek@google.com>
 *          Peter Enderborg <Peter.Enderborg@sony.com>
 */

// TRACE_SYSTEM avc
// The declarations below require the kernel tracepoint definitions supplied
// by the surrounding translation unit.

trace_event!(
    selinux_audited,
    proto(
        sad: *mut selinux_audit_data,
        scontext: *mut core::ffi::c_char,
        tcontext: *mut core::ffi::c_char,
        tclass: *const core::ffi::c_char,
    ),
    args(sad, scontext, tcontext, tclass),
    entry {
        requested: u32,
        denied: u32,
        audited: u32,
        result: core::ffi::c_int,
        scontext: *mut core::ffi::c_char,
        tcontext: *mut core::ffi::c_char,
        tclass: *const core::ffi::c_char,
    },
    fast_assign |entry: &mut SelinuxAuditedEntry, sad, scontext, tcontext, tclass| unsafe {
        entry.requested = (*sad).requested;
        entry.denied = (*sad).denied;
        entry.audited = (*sad).audited;
        entry.result = (*sad).result;
        entry.tcontext = tcontext;
        entry.scontext = scontext;
        entry.tclass = tclass;
    },
    printk(
        "requested=0x%x denied=0x%x audited=0x%x result=%d scontext=%s tcontext=%s tclass=%s",
        requested,
        denied,
        audited,
        result,
        scontext,
        tcontext,
        tclass,
    ),
);

// The following types are supplied by the translated kernel dependencies.
#[allow(non_camel_case_types)]
pub enum selinux_audit_data {}

#[repr(C)]
pub struct SelinuxAuditedEntry {
    pub requested: u32,
    pub denied: u32,
    pub audited: u32,
    pub result: core::ffi::c_int,
    pub scontext: *mut core::ffi::c_char,
    pub tcontext: *mut core::ffi::c_char,
    pub tclass: *const core::ffi::c_char,
}

// <trace/define_trace.h> is intentionally represented by the trace_event!
// declaration above and is provided by the surrounding tracepoint system.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
