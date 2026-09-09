/* SPDX-License-Identifier: GPL-2.0 */
//
// Rust translation of trace/events/pwc.h.
// The original file is a Linux tracepoint header.  Its TRACE_EVENT
// invocations are represented below as the corresponding C-layout event
// payloads and trace-format metadata; the kernel tracepoint registration and
// field accessors remain provided by the surrounding tracing infrastructure.

// Original include dependencies:
//   <linux/usb.h>
//   <linux/tracepoint.h>

/// Opaque kernel USB request type supplied by the Linux USB subsystem.
#[repr(C)]
pub struct urb {
    _private: [u8; 0],
}

/// Opaque PWC device type supplied by the PWC driver.
#[repr(C)]
pub struct pwc_device {
    _private: [u8; 0],
}

/// Opaque PWC frame-buffer type supplied by the PWC driver.
#[repr(C)]
pub struct pwc_frame_buf {
    _private: [u8; 0],
}

/// Payload corresponding to `TRACE_EVENT(pwc_handler_enter, ...)`.
#[repr(C)]
pub struct pwc_handler_enter_event {
    pub urb: *mut urb,
    pub fbuf: *mut pwc_frame_buf,
    pub urb__status: core::ffi::c_int,
    pub urb__actual_length: u32,
    pub fbuf__filled: core::ffi::c_int,
    /// `__string(name, pdev->v4l2_dev.name)`; storage is owned by the
    /// generated trace event infrastructure.
    pub name: *const core::ffi::c_char,
}

/// Trace print format for `pwc_handler_enter`.
pub const PWC_HANDLER_ENTER_PRINTK: &str =
    "dev=%s (fbuf=%p filled=%d) urb=%p (status=%d actual_length=%u)";

/// Payload corresponding to `TRACE_EVENT(pwc_handler_exit, ...)`.
#[repr(C)]
pub struct pwc_handler_exit_event {
    pub urb: *mut urb,
    pub fbuf: *mut pwc_frame_buf,
    pub fbuf__filled: core::ffi::c_int,
    /// `__string(name, pdev->v4l2_dev.name)`; storage is owned by the
    /// generated trace event infrastructure.
    pub name: *const core::ffi::c_char,
}

/// Trace print format for `pwc_handler_exit`.
pub const PWC_HANDLER_EXIT_PRINTK: &str =
    " dev=%s (fbuf=%p filled=%d) urb=%p";

// The following assignment descriptions preserve the original TRACE_EVENT
// TP_fast_assign operations.  The referenced members belong to external
// Linux/PWC definitions and are intentionally not redefined here.
//
// pwc_handler_enter:
//   event.urb = urb;
//   event.fbuf = pdev->fill_buf;
//   event.urb__status = urb->status;
//   event.urb__actual_length = urb->actual_length;
//   event.fbuf__filled = (pdev->fill_buf ? pdev->fill_buf->filled : 0);
//   __assign_str(name);
//
// pwc_handler_exit:
//   event.urb = urb;
//   event.fbuf = pdev->fill_buf;
//   event.fbuf__filled = pdev->fill_buf->filled;
//   __assign_str(name);

// `#include <trace/define_trace.h>` is a kernel build-time tracepoint
// expansion and has no standalone Rust item.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
