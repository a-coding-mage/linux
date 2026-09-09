/* SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause) */
/* Copyright 2019, 2023 NXP */

// Forward declarations from the C header.
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct caam_drv_private {
    _private: [u8; 0],
}

#[repr(C)]
pub struct caam_perfmon {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe extern "C" {
    pub fn caam_debugfs_init(
        ctrlpriv: *mut caam_drv_private,
        perfmon: *mut caam_perfmon,
        root: *mut dentry,
    );
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn caam_debugfs_init(
    _ctrlpriv: *mut caam_drv_private,
    _perfmon: *mut caam_perfmon,
    _root: *mut dentry,
) {
}

#[cfg(all(
    feature = "CONFIG_DEBUG_FS",
    feature = "CONFIG_CRYPTO_DEV_FSL_CAAM_CRYPTO_API_QI"
))]
unsafe extern "C" {
    pub fn caam_debugfs_qi_congested();
    pub fn caam_debugfs_qi_init(ctrlpriv: *mut caam_drv_private);
}

#[cfg(not(all(
    feature = "CONFIG_DEBUG_FS",
    feature = "CONFIG_CRYPTO_DEV_FSL_CAAM_CRYPTO_API_QI"
)))]
#[inline]
pub unsafe fn caam_debugfs_qi_congested() {
}

#[cfg(not(all(
    feature = "CONFIG_DEBUG_FS",
    feature = "CONFIG_CRYPTO_DEV_FSL_CAAM_CRYPTO_API_QI"
)))]
#[inline]
pub unsafe fn caam_debugfs_qi_init(_ctrlpriv: *mut caam_drv_private) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
