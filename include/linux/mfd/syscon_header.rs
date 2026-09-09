/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * System Control Driver
 *
 * Copyright (C) 2012 Freescale Semiconductor, Inc.
 * Copyright (C) 2012 Linaro Ltd.
 *
 * Author: Dong Aisheng <dong.aisheng@linaro.org>
 */

// C dependencies: <linux/err.h>, <linux/errno.h>

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

// Linux errno values used by this header.
const ENOTSUPP: isize = 524;
const EOPNOTSUPP: i32 = 95;

#[inline]
unsafe fn err_ptr<T>(error: isize) -> *mut T {
    error as *mut T
}

#[cfg(CONFIG_MFD_SYSCON)]
extern "C" {
    pub fn device_node_to_regmap(np: *mut device_node) -> *mut regmap;
    pub fn syscon_node_to_regmap(np: *mut device_node) -> *mut regmap;
    pub fn syscon_regmap_lookup_by_compatible(s: *const core::ffi::c_char) -> *mut regmap;
    pub fn syscon_regmap_lookup_by_phandle(
        np: *mut device_node,
        property: *const core::ffi::c_char,
    ) -> *mut regmap;
    pub fn syscon_regmap_lookup_by_phandle_args(
        np: *mut device_node,
        property: *const core::ffi::c_char,
        arg_count: i32,
        out_args: *mut u32,
    ) -> *mut regmap;
    pub fn syscon_regmap_lookup_by_phandle_optional(
        np: *mut device_node,
        property: *const core::ffi::c_char,
    ) -> *mut regmap;
    pub fn of_syscon_register_regmap(np: *mut device_node, regmap: *mut regmap) -> i32;
}

#[cfg(not(CONFIG_MFD_SYSCON))]
#[inline]
pub unsafe fn device_node_to_regmap(_np: *mut device_node) -> *mut regmap {
    err_ptr(-ENOTSUPP)
}

#[cfg(not(CONFIG_MFD_SYSCON))]
#[inline]
pub unsafe fn syscon_node_to_regmap(_np: *mut device_node) -> *mut regmap {
    err_ptr(-ENOTSUPP)
}

#[cfg(not(CONFIG_MFD_SYSCON))]
#[inline]
pub unsafe fn syscon_regmap_lookup_by_compatible(
    _s: *const core::ffi::c_char,
) -> *mut regmap {
    err_ptr(-ENOTSUPP)
}

#[cfg(not(CONFIG_MFD_SYSCON))]
#[inline]
pub unsafe fn syscon_regmap_lookup_by_phandle(
    _np: *mut device_node,
    _property: *const core::ffi::c_char,
) -> *mut regmap {
    err_ptr(-ENOTSUPP)
}

#[cfg(not(CONFIG_MFD_SYSCON))]
#[inline]
pub unsafe fn syscon_regmap_lookup_by_phandle_args(
    _np: *mut device_node,
    _property: *const core::ffi::c_char,
    _arg_count: i32,
    _out_args: *mut u32,
) -> *mut regmap {
    err_ptr(-ENOTSUPP)
}

#[cfg(not(CONFIG_MFD_SYSCON))]
#[inline]
pub unsafe fn syscon_regmap_lookup_by_phandle_optional(
    _np: *mut device_node,
    _property: *const core::ffi::c_char,
) -> *mut regmap {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_MFD_SYSCON))]
#[inline]
pub unsafe fn of_syscon_register_regmap(
    _np: *mut device_node,
    _regmap: *mut regmap,
) -> i32 {
    -EOPNOTSUPP
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
