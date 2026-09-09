/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2023, Linaro Ltd.
 */

use core::ffi::c_char;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct icc_provider {
    _private: [u8; 0],
}

#[repr(C)]
pub struct icc_clk_data {
    pub clk: *mut clk,
    pub name: *const c_char,
    pub master_id: u32,
    pub slave_id: u32,
}

extern "C" {
    pub fn icc_clk_register(
        dev: *mut device,
        first_id: u32,
        num_clocks: u32,
        data: *const icc_clk_data,
    ) -> *mut icc_provider;

    pub fn devm_icc_clk_register(
        dev: *mut device,
        first_id: u32,
        num_clocks: u32,
        data: *const icc_clk_data,
    ) -> core::ffi::c_int;

    pub fn icc_clk_unregister(provider: *mut icc_provider);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
