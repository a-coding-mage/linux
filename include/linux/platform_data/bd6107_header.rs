/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * bd6107.h - Rohm BD6107 LEDs Driver
 */

// Opaque declaration corresponding to C's `struct device`.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bd6107_platform_data {
    pub dev: *mut device,
    pub def_value: ::core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
