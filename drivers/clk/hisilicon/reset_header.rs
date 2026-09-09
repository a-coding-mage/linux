/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2015 HiSilicon Technologies Co., Ltd.
 */

// Forward declarations supplied by other translation units.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hisi_reset_controller {
    _private: [u8; 0],
}

// `platform_device` is supplied by the platform-device dependency.
#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_RESET_CONTROLLER")]
extern "C" {
    pub fn hisi_reset_init(pdev: *mut platform_device) -> *mut hisi_reset_controller;
    pub fn hisi_reset_exit(rstc: *mut hisi_reset_controller);
}

#[cfg(not(feature = "CONFIG_RESET_CONTROLLER"))]
#[inline]
pub unsafe fn hisi_reset_init(_pdev: *mut platform_device) -> *mut hisi_reset_controller {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_RESET_CONTROLLER"))]
#[inline]
pub unsafe fn hisi_reset_exit(_rstc: *mut hisi_reset_controller) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
