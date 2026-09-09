/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2012 Sascha Hauer, Pengutronix
 */

// Forward declarations corresponding to the C declarations in the header.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_bridge {
    _private: [u8; 0],
}

extern "C" {
    pub fn devm_drm_of_display_mode_bridge(
        dev: *mut device,
        np: *mut device_node,
        type_: ::core::ffi::c_int,
    ) -> *mut drm_bridge;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
