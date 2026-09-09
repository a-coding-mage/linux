/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM riscv
//
// The C source uses the Linux tracepoint macro framework.  The framework
// declarations are external to this header and are therefore represented by
// the corresponding C-layout event entry types and event conditions here.

#[repr(C)]
pub struct SbiCallEntry {
    pub ext: ::core::ffi::c_int,
    pub fid: ::core::ffi::c_int,
}

#[repr(C)]
pub struct SbiReturnEntry {
    pub error: ::core::ffi::c_long,
    pub value: ::core::ffi::c_long,
}

/// Condition from `TRACE_EVENT_CONDITION(sbi_call, ...)`.
#[inline]
pub const unsafe fn sbi_call_condition(ext: ::core::ffi::c_int) -> bool {
    ext != SBI_EXT_HSM
}

/// Condition from `TRACE_EVENT_CONDITION(sbi_return, ...)`.
#[inline]
pub const unsafe fn sbi_return_condition(ext: ::core::ffi::c_int) -> bool {
    ext != SBI_EXT_HSM
}

/// Equivalent to the `TP_fast_assign` body for the `sbi_call` event.
#[inline]
pub unsafe fn sbi_call_fast_assign(
    entry: *mut SbiCallEntry,
    ext: ::core::ffi::c_int,
    fid: ::core::ffi::c_int,
) {
    (*entry).ext = ext;
    (*entry).fid = fid;
}

/// Equivalent to the `TP_fast_assign` body for the `sbi_return` event.
#[inline]
pub unsafe fn sbi_return_fast_assign(
    entry: *mut SbiReturnEntry,
    error: ::core::ffi::c_long,
    value: ::core::ffi::c_long,
) {
    (*entry).error = error;
    (*entry).value = value;
}

// TP_printk formats supplied by the source trace events:
//   sbi_call:   "ext=0x%x fid=%d"
//   sbi_return: "error=%ld value=0x%lx"
//
// `SBI_EXT_HSM` is provided by the external SBI dependency, as in the C
// header.  The Linux tracepoint include and trace-definition include are
// likewise external framework dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
