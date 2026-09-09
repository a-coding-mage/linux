/* SPDX-License-Identifier: GPL-2.0-only */

/* Copyright (c) 2020, The Linux Foundation. All rights reserved. */
/* Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries. */

use core::ffi::c_int;

#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct qaic_device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn qaic_ssr_register() -> c_int;
    pub fn qaic_ssr_unregister();
    pub fn qaic_clean_up_ssr(qdev: *mut qaic_device);
    pub fn qaic_ssr_init(qdev: *mut qaic_device, drm: *mut drm_device) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
