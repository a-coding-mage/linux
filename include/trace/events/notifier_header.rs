/* SPDX-License-Identifier: GPL-2.0 */

//! Rust translation of the Linux notifier tracepoint header.
//!
//! The C tracepoint declaration macros and `trace/define_trace.h` include are
//! represented by the corresponding C ABI data and event declarations below.

use core::ffi::c_void;

/// Payload stored by the `notifier_info` trace event class.
#[repr(C)]
pub struct NotifierInfoEntry {
    pub cb: *mut c_void,
}

impl NotifierInfoEntry {
    #[inline]
    pub const unsafe fn new(cb: *mut c_void) -> Self {
        Self { cb }
    }
}

/// `notifier_register` — called upon notifier callback registration.
///
/// `cb`: callback pointer.
extern "C" {
    pub fn notifier_register(cb: *mut c_void);
}

/// `notifier_unregister` — called upon notifier callback unregistration.
///
/// `cb`: callback pointer.
extern "C" {
    pub fn notifier_unregister(cb: *mut c_void);
}

/// `notifier_run` — called upon notifier callback execution.
///
/// `cb`: callback pointer.
extern "C" {
    pub fn notifier_run(cb: *mut c_void);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
