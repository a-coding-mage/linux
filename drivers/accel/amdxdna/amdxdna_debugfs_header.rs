/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2026, Advanced Micro Devices, Inc.
 */

// Dependency supplied by amdxdna_pci_drv.h in the C source.
#[repr(C)]
pub struct amdxdna_dev {
    _private: [u8; 0],
}

// Equivalent of CONFIG_DEBUG_FS. The feature name preserves the source
// build-time condition and is expected to be supplied by the build system.
#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe extern "C" {
    pub fn amdxdna_debugfs_init(xdna: *mut amdxdna_dev);
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn amdxdna_debugfs_init(_xdna: *mut amdxdna_dev) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
