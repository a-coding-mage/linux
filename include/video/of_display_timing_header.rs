/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2012 Steffen Trumtrar <s.trumtrar@pengutronix.de>
 *
 * display timings of helpers
 */

use core::ffi::{c_char, c_int};

// Forward declarations from the C header.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct display_timing {
    _private: [u8; 0],
}

#[repr(C)]
pub struct display_timings {
    _private: [u8; 0],
}

pub const OF_USE_NATIVE_MODE: c_int = -1;

// The CONFIG_OF build-time condition is preserved with Rust cfg attributes.
#[cfg(CONFIG_OF)]
unsafe extern "C" {
    pub fn of_get_display_timing(
        np: *const device_node,
        name: *const c_char,
        dt: *mut display_timing,
    ) -> c_int;

    pub fn of_get_display_timings(np: *const device_node) -> *mut display_timings;
}

#[cfg(not(CONFIG_OF))]
#[inline]
pub unsafe fn of_get_display_timing(
    _np: *const device_node,
    _name: *const c_char,
    _dt: *mut display_timing,
) -> c_int {
    // Linux errno.h: ENOSYS (Function not implemented).
    -38
}

#[cfg(not(CONFIG_OF))]
#[inline]
pub unsafe fn of_get_display_timings(_np: *const device_node) -> *mut display_timings {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
