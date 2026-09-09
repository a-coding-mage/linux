/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Copyright (c) 2023 Meta Platforms, Inc. and affiliates
 *  Copyright (c) 2023 Intel and affiliates
 */

use core::ffi::c_int;

// External opaque types supplied by other translation units.
#[repr(C)]
pub struct dpll_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dpll_pin {
    _private: [u8; 0],
}

extern "C" {
    pub fn dpll_device_create_ntf(dpll: *mut dpll_device) -> c_int;

    pub fn dpll_device_delete_ntf(dpll: *mut dpll_device) -> c_int;

    pub fn dpll_pin_create_ntf(pin: *mut dpll_pin, src_clock_id: u64) -> c_int;

    pub fn dpll_pin_delete_ntf(pin: *mut dpll_pin, src_clock_id: u64) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
