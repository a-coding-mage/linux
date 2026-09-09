/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the Linux tracepoint header `intel_ifs.h`.
// The C includes and TRACE_EVENT expansion are supplied by the surrounding
// kernel tracepoint infrastructure.

/// Entry data emitted by the `ifs_status` trace event.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifs_status_entry {
    pub batch: ::core::ffi::c_int,
    pub status: u64,
    pub start: u16,
    pub stop: u16,
}

/// Entry data emitted by the `ifs_sbaf` trace event.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifs_sbaf_entry {
    pub status: u64,
    pub batch: ::core::ffi::c_int,
    pub bundle: u16,
    pub pgm: u16,
}

/// Trace event equivalent of:
/// `TRACE_EVENT(ifs_status, TP_PROTO(int batch, int start, int stop, u64 status), ...)`
///
/// The kernel tracepoint implementation performs the assignment and formatted
/// output below when the event is enabled.
#[inline]
pub unsafe fn ifs_status_assign(
    entry: *mut ifs_status_entry,
    batch: ::core::ffi::c_int,
    start: ::core::ffi::c_int,
    stop: ::core::ffi::c_int,
    status: u64,
) {
    (*entry).batch = batch;
    (*entry).start = start as u16;
    (*entry).stop = stop as u16;
    (*entry).status = status;
}

/// Trace event equivalent of:
/// `TRACE_EVENT(ifs_sbaf, TP_PROTO(int batch, union ifs_sbaf activate,
/// union ifs_sbaf_status status), ...)`
///
/// `ifs_sbaf` and `ifs_sbaf_status` are external C union types supplied by the
/// surrounding implementation. Their fields are accessed by the original
/// tracepoint as `activate.bundle_idx`, `activate.pgm_idx`, and `status.data`.
/// The C tracepoint declaration is retained here as the authoritative ABI
/// description because Rust cannot name those external union definitions from
/// this header alone.
// TP_ARGS: batch, activate, status
// TP_STRUCT__entry: u64 status; int batch; u16 bundle; u16 pgm
// TP_fast_assign:
//   entry.status = status.data;
//   entry.batch = batch;
//   entry.bundle = activate.bundle_idx;
//   entry.pgm = activate.pgm_idx;
// TP_printk("batch: 0x%.2x, bundle_idx: 0x%.4x, pgm_idx: 0x%.4x, status: 0x%.16llx")

// TP_printk for ifs_status:
// "batch: 0x%.2x, start: 0x%.4x, stop: 0x%.4x, status: 0x%.16llx"

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
