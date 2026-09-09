/* SPDX-License-Identifier: GPL-2.0 */

//! Rust translation of the `qla` trace event header.
//!
//! The Linux tracepoint declarations in the source are represented here as
//! C-compatible declarations.  Their tracepoint implementation is supplied
//! by the external tracing infrastructure.

use core::ffi::{c_char, c_void};

pub const QLA_MSG_MAX: usize = 256;

/// Opaque representation of `struct va_format` supplied by the kernel.
#[repr(C)]
pub struct va_format {
    _private: [u8; 0],
}

/// Arguments captured by the `qla_log_event` trace event class.
#[repr(C)]
pub struct qla_log_event_entry {
    /// String copied from the `buf` argument.
    pub buf: *const c_char,
    /// Formatted message produced from `va_format::fmt` and `va_format::va`.
    pub msg: *const c_char,
}

/// Trace event class corresponding to `DECLARE_EVENT_CLASS(qla_log_event, ...)`.
///
/// The source event stores `buf` and a variable-length formatted `msg`, with
/// the latter assigned from `vaf->fmt` and `vaf->va`.
#[repr(C)]
pub struct qla_log_event {
    pub buf: *const c_char,
    pub vaf: *mut va_format,
}

extern "C" {
    /// Trace event corresponding to `DEFINE_EVENT(qla_log_event, ql_dbg_log, ...)`.
    pub fn ql_dbg_log(buf: *const c_char, vaf: *mut va_format);
}

// The original header includes <linux/tracepoint.h> and
// <trace/define_trace.h>; those facilities provide the tracepoint machinery
// and are external dependencies of this translation.
//
// The original print format is: "%s %s", __get_str(buf), __get_str(msg).
// The original event assignment is equivalent to copying `buf` and
// formatting `msg` from `vaf->fmt` and `vaf->va`.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
