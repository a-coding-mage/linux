// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2014, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation unit.

use core::ffi::c_void;

#[repr(C)]
pub struct device {
    pub parent: *mut device,
}

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_regmap {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub enable_reg: u32,
    pub enable_mask: u32,
    pub enable_is_inverted: bool,
}

extern "C" {
    fn to_clk_regmap(hw: *mut clk_hw) -> *mut clk_regmap;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn dev_get_regmap(dev: *mut device, name: *const c_void) -> *mut regmap;
    fn devm_clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> i32;
}

/**
 * clk_is_enabled_regmap - standard is_enabled() for regmap users
 *
 * @hw: clk to operate on
 *
 * Clocks that use regmap for their register I/O can set the
 * enable_reg and enable_mask fields in their struct clk_regmap and then use
 * this as their is_enabled operation, saving some code.
 */
#[no_mangle]
pub unsafe extern "C" fn clk_is_enabled_regmap(hw: *mut clk_hw) -> i32 {
    let rclk = &mut *to_clk_regmap(hw);
    let mut val: u32 = 0;

    let ret = regmap_read(rclk.regmap, rclk.enable_reg, &mut val);
    if ret != 0 {
        return ret;
    }

    if rclk.enable_is_inverted {
        ((val & rclk.enable_mask) == 0) as i32
    } else {
        ((val & rclk.enable_mask) != 0) as i32
    }
}

/**
 * clk_enable_regmap - standard enable() for regmap users
 *
 * @hw: clk to operate on
 *
 * Clocks that use regmap for their register I/O can set the
 * enable_reg and enable_mask fields in their struct clk_regmap and then use
 * this as their enable() operation, saving some code.
 */
#[no_mangle]
pub unsafe extern "C" fn clk_enable_regmap(hw: *mut clk_hw) -> i32 {
    let rclk = &mut *to_clk_regmap(hw);
    let val: u32;

    if rclk.enable_is_inverted {
        val = 0;
    } else {
        val = rclk.enable_mask;
    }

    regmap_update_bits(rclk.regmap, rclk.enable_reg, rclk.enable_mask, val)
}

/**
 * clk_disable_regmap - standard disable() for regmap users
 *
 * @hw: clk to operate on
 *
 * Clocks that use regmap for their register I/O can set the
 * enable_reg and enable_mask fields in their struct clk_regmap and then use
 * this as their disable() operation, saving some code.
 */
#[no_mangle]
pub unsafe extern "C" fn clk_disable_regmap(hw: *mut clk_hw) {
    let rclk = &mut *to_clk_regmap(hw);
    let val: u32;

    if rclk.enable_is_inverted {
        val = rclk.enable_mask;
    } else {
        val = 0;
    }

    regmap_update_bits(rclk.regmap, rclk.enable_reg, rclk.enable_mask, val);
}

/**
 * devm_clk_register_regmap - register a clk_regmap clock
 *
 * @dev: reference to the caller's device
 * @rclk: clk to operate on
 *
 * Clocks that use regmap for their register I/O should register their
 * clk_regmap struct via this function so that the regmap is initialized
 * and so that the clock is registered with the common clock framework.
 */
#[no_mangle]
pub unsafe extern "C" fn devm_clk_register_regmap(
    dev: *mut device,
    rclk: *mut clk_regmap,
) -> i32 {
    if !dev.is_null() && !dev_get_regmap(dev, core::ptr::null()).is_null() {
        (*rclk).regmap = dev_get_regmap(dev, core::ptr::null());
    } else if !dev.is_null() && !(*dev).parent.is_null() {
        (*rclk).regmap = dev_get_regmap((*dev).parent, core::ptr::null());
    }

    devm_clk_hw_register(dev, &mut (*rclk).hw)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
