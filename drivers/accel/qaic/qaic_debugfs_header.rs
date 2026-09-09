/* SPDX-License-Identifier: GPL-2.0-only */

/* Copyright (c) 2020, The Linux Foundation. All rights reserved. */
/* Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries. */

/* C dependency: <drm/drm_file.h> */

/* CONFIG_DEBUG_FS */
#[cfg(CONFIG_DEBUG_FS)]
extern "C" {
    pub fn qaic_bootlog_register() -> ::core::ffi::c_int;
    pub fn qaic_bootlog_unregister();
    pub fn qaic_debugfs_init(qddev: *mut qaic_drm_device);
}

/* The definition of qaic_drm_device is supplied by another translation unit. */
#[cfg(not(CONFIG_DEBUG_FS))]
#[inline]
pub fn qaic_bootlog_register() -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_DEBUG_FS))]
#[inline]
pub fn qaic_bootlog_unregister() {}

#[cfg(not(CONFIG_DEBUG_FS))]
#[inline]
pub fn qaic_debugfs_init(_qddev: *mut qaic_drm_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
