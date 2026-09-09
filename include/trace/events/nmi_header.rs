/* SPDX-License-Identifier: GPL-2.0 */

//! Rust translation of the `nmi_handler` trace event declaration.
//!
//! The original header depends on the Linux tracepoint and ktime headers;
//! those facilities are supplied by the surrounding translation unit.

use core::ffi::c_void;

/// Payload recorded by the `nmi_handler` trace event.
#[repr(C)]
pub struct NmiHandlerEntry {
    pub handler: *mut c_void,
    pub delta_ns: i64,
    pub handled: i32,
}

impl NmiHandlerEntry {
    /// Equivalent to the original `TP_fast_assign` block.
    #[inline]
    pub unsafe fn assign(&mut self, handler: *mut c_void, delta_ns: i64, handled: i32) {
        self.handler = handler;
        self.delta_ns = delta_ns;
        self.handled = handled;
    }
}

/// Declaration corresponding to the generated tracepoint for `nmi_handler`.
///
/// The implementation is provided by the tracepoint subsystem, analogous to
/// the `TRACE_EVENT` expansion in the C header.
extern "C" {
    pub fn trace_nmi_handler(handler: *mut c_void, delta_ns: i64, handled: i32);
}

// Original TP_printk format:
// "%ps() delta_ns: %lld handled: %d"

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
