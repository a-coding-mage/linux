/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Device core Trace Support
 * Copyright (C) 2021, Intel Corporation
 *
 * Author: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
 */

//! Rust translation of the C device trace declarations.
//!
//! The C header uses the Linux tracepoint-generation macros.  The declarations
//! below preserve the generated event payload and event interface; tracepoint
//! registration and formatting remain supplied by the surrounding tracepoint
//! implementation.

use core::ffi::{c_char, c_void};

// C dependency: `struct device` is declared by <linux/device.h>.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct devres_entry {
    /// `__string(devname, dev_name(dev))`
    pub devname: *const c_char,
    /// `__field(struct device *, dev)`
    pub dev: *mut device,
    /// `__field(const char *, op)`
    pub op: *const c_char,
    /// `__field(void *, node)`
    pub node: *mut c_void,
    /// `__string(name, name)`
    pub name: *const c_char,
    /// `__field(size_t, size)`
    pub size: usize,
}

extern "C" {
    /// Trace event corresponding to `DEFINE_EVENT(devres, devres_log, ...)`.
    pub fn devres_log(
        dev: *mut device,
        op: *const c_char,
        node: *mut c_void,
        name: *const c_char,
        size: usize,
    );
}

// The C `TP_printk` format is:
// "%s %3s %p %s (%zu bytes)", devname, op, node, name, size.
// Tracepoint declaration/registration is intentionally left to the external
// Linux tracepoint implementation, as in the original header.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
