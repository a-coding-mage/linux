/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2013 Samsung Electronics Co., Ltd.
 * Sylwester Nawrocki <s.nawrocki@samsung.com>
 */

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_phandle_args {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[cfg(all(feature = "CONFIG_OF", feature = "CONFIG_COMMON_CLK"))]
extern "C" {
    pub fn of_clk_get_hw(
        np: *mut device_node,
        index: c_int,
        con_id: *const c_char,
    ) -> *mut clk_hw;
}

/* !CONFIG_COMMON_CLK || !CONFIG_OF */
#[cfg(not(all(feature = "CONFIG_OF", feature = "CONFIG_COMMON_CLK")))]
#[inline]
pub unsafe fn of_clk_get_hw(
    _np: *mut device_node,
    _index: c_int,
    _con_id: *const c_char,
) -> *mut clk_hw {
    ERR_PTR(-ENOENT)
}

extern "C" {
    pub fn clk_find_hw(dev_id: *const c_char, con_id: *const c_char) -> *mut clk_hw;
}

#[cfg(feature = "CONFIG_COMMON_CLK")]
extern "C" {
    pub fn clk_hw_create_clk(
        dev: *mut device,
        hw: *mut clk_hw,
        dev_id: *const c_char,
        con_id: *const c_char,
    ) -> *mut clk;
    pub fn __clk_put(clk: *mut clk);
}

/* All these casts to avoid ifdefs in clkdev... */
#[cfg(not(feature = "CONFIG_COMMON_CLK"))]
#[inline]
pub unsafe fn clk_hw_create_clk(
    _dev: *mut device,
    hw: *mut clk_hw,
    _dev_id: *const c_char,
    _con_id: *const c_char,
) -> *mut clk {
    hw as *mut clk
}

#[cfg(not(feature = "CONFIG_COMMON_CLK"))]
#[inline]
pub unsafe fn __clk_put(_clk: *mut clk) {}

/* ERR_PTR and -ENOENT are supplied by the surrounding kernel environment. */
extern "C" {
    fn ERR_PTR(error: isize) -> *mut clk_hw;
}

const ENOENT: isize = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
