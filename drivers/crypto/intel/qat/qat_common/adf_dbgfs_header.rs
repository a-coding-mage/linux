/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2023 Intel Corporation */

/* CONFIG_DEBUG_FS controls whether the externally implemented debugfs hooks
 * are declared or replaced by empty inline functions. */

#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
extern "C" {
    pub fn adf_dbgfs_init(accel_dev: *mut adf_accel_dev);
    pub fn adf_dbgfs_add(accel_dev: *mut adf_accel_dev);
    pub fn adf_dbgfs_rm(accel_dev: *mut adf_accel_dev);
    pub fn adf_dbgfs_exit(accel_dev: *mut adf_accel_dev);
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub fn adf_dbgfs_init(_accel_dev: *mut adf_accel_dev) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub fn adf_dbgfs_add(_accel_dev: *mut adf_accel_dev) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub fn adf_dbgfs_rm(_accel_dev: *mut adf_accel_dev) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub fn adf_dbgfs_exit(_accel_dev: *mut adf_accel_dev) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
