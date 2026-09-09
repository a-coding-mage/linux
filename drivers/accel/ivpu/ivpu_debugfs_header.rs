/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020-2023 Intel Corporation
 */

// Opaque declaration corresponding to `struct ivpu_device`.
#[repr(C)]
pub struct ivpu_device {
    _private: [u8; 0],
}

// The CONFIG_DEBUG_FS build condition is preserved as a Rust configuration
// condition; the corresponding build configuration must define `debug_fs`.
#[cfg(debug_fs)]
extern "C" {
    pub fn ivpu_debugfs_init(vdev: *mut ivpu_device);
}

#[cfg(not(debug_fs))]
#[inline]
pub unsafe fn ivpu_debugfs_init(_vdev: *mut ivpu_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
