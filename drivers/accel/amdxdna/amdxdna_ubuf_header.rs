/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2025, Advanced Micro Devices, Inc.
 */

// Translated from amdxdna_ubuf.h.
// C dependencies:
//   #include <drm/drm_device.h>
//   #include <linux/dma-buf.h>

use core::ffi::c_void;

#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_buf {
    _private: [u8; 0],
}

extern "C" {
    pub fn amdxdna_get_ubuf(
        dev: *mut drm_device,
        num_entries: u32,
        va_entries: *mut c_void,
    ) -> *mut dma_buf;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
