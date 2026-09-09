/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * rcar-fcp.h  --  R-Car Frame Compression Processor Driver
 *
 * Copyright (C) 2016 Renesas Electronics Corporation
 *
 * Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
 */

// C forward declarations.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rcar_fcp_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

// Equivalent to IS_ENABLED(CONFIG_VIDEO_RENESAS_FCP).
#[cfg(feature = "CONFIG_VIDEO_RENESAS_FCP")]
extern "C" {
    pub fn rcar_fcp_get(np: *const device_node) -> *mut rcar_fcp_device;
    pub fn rcar_fcp_put(fcp: *mut rcar_fcp_device);
    pub fn rcar_fcp_get_device(fcp: *mut rcar_fcp_device) -> *mut device;
    pub fn rcar_fcp_enable(fcp: *mut rcar_fcp_device) -> i32;
    pub fn rcar_fcp_disable(fcp: *mut rcar_fcp_device);
    pub fn rcar_fcp_soft_reset(fcp: *mut rcar_fcp_device) -> i32;
}

#[cfg(not(feature = "CONFIG_VIDEO_RENESAS_FCP"))]
#[inline]
pub unsafe fn rcar_fcp_get(_np: *const device_node) -> *mut rcar_fcp_device {
    // ERR_PTR(-ENOENT), with Linux's ENOENT value of 2.
    (-2isize) as *mut rcar_fcp_device
}

#[cfg(not(feature = "CONFIG_VIDEO_RENESAS_FCP"))]
#[inline]
pub unsafe fn rcar_fcp_put(_fcp: *mut rcar_fcp_device) {}

#[cfg(not(feature = "CONFIG_VIDEO_RENESAS_FCP"))]
#[inline]
pub unsafe fn rcar_fcp_get_device(_fcp: *mut rcar_fcp_device) -> *mut device {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_VIDEO_RENESAS_FCP"))]
#[inline]
pub unsafe fn rcar_fcp_enable(_fcp: *mut rcar_fcp_device) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_VIDEO_RENESAS_FCP"))]
#[inline]
pub unsafe fn rcar_fcp_disable(_fcp: *mut rcar_fcp_device) {}

#[cfg(not(feature = "CONFIG_VIDEO_RENESAS_FCP"))]
#[inline]
pub unsafe fn rcar_fcp_soft_reset(_fcp: *mut rcar_fcp_device) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
