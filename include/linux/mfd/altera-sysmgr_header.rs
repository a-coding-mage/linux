/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018-2019 Intel Corporation
 * Copyright (C) 2012 Freescale Semiconductor, Inc.
 * Copyright (C) 2012 Linaro Ltd.
 */

// C header guard: __LINUX_MFD_ALTERA_SYSMGR_H__

// Dependencies supplied by the surrounding Linux/Rust environment:
// linux/err.h, linux/errno.h, and linux/firmware/intel/stratix10-smc.h.

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_MFD_ALTERA_SYSMGR")]
extern "C" {
    pub fn altr_sysmgr_regmap_lookup_by_phandle(
        np: *mut device_node,
        property: *const ::core::ffi::c_char,
    ) -> *mut regmap;
}

#[cfg(not(feature = "CONFIG_MFD_ALTERA_SYSMGR"))]
#[inline]
pub unsafe fn altr_sysmgr_regmap_lookup_by_phandle(
    _np: *mut device_node,
    _property: *const ::core::ffi::c_char,
) -> *mut regmap {
    // ERR_PTR(-ENOTSUPP); ENOTSUPP is Linux errno 524.
    (-524isize) as *mut regmap
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
