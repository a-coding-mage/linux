/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2014, The Linux Foundation. All rights reserved. */

// Dependency supplied by the Linux clock-provider interface:
// use crate::{clk_hw, container_of, device};

#[repr(C)]
pub struct clk_regmap {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub enable_reg: ::core::ffi::c_uint,
    pub enable_mask: ::core::ffi::c_uint,
    pub enable_is_inverted: bool,
}

// Opaque dependency supplied by the regmap subsystem.
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

pub unsafe fn to_clk_regmap(hw: *mut clk_hw) -> *mut clk_regmap {
    container_of!(hw, clk_regmap, hw)
}

pub unsafe extern "C" fn clk_is_enabled_regmap(hw: *mut clk_hw) -> ::core::ffi::c_int;
pub unsafe extern "C" fn clk_enable_regmap(hw: *mut clk_hw) -> ::core::ffi::c_int;
pub unsafe extern "C" fn clk_disable_regmap(hw: *mut clk_hw);
pub unsafe extern "C" fn devm_clk_register_regmap(
    dev: *mut device,
    rclk: *mut clk_regmap,
) -> ::core::ffi::c_int;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
