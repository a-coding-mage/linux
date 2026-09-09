/* SPDX-License-Identifier: GPL-2.0 */
// TI LMU Common Core
// Copyright (C) 2018 Texas Instruments Incorporated - https://www.ti.com/

// C dependencies supplied by the surrounding kernel translation unit:
// linux/delay.h, linux/device.h, linux/init.h, linux/leds.h,
// linux/module.h, linux/regmap.h, linux/slab.h, and uapi/linux/uleds.h.

pub const LMU_11BIT_LSB_MASK: u32 = (1u32 << 0) | (1u32 << 1) | (1u32 << 2);
pub const LMU_11BIT_MSB_SHIFT: u32 = 3;

pub const MAX_BRIGHTNESS_8BIT: i32 = 255;
pub const MAX_BRIGHTNESS_11BIT: i32 = 2047;

#[repr(C)]
pub struct ti_lmu_bank {
    pub regmap: *mut regmap,

    pub max_brightness: i32,

    pub lsb_brightness_reg: u8,
    pub msb_brightness_reg: u8,

    pub runtime_ramp_reg: u8,
    pub ramp_up_usec: u32,
    pub ramp_down_usec: u32,
}

// Opaque types provided by external kernel dependencies.
pub enum regmap {}
pub enum device {}
pub enum fwnode_handle {}

extern "C" {
    pub fn ti_lmu_common_set_brightness(
        lmu_bank: *mut ti_lmu_bank,
        brightness: i32,
    ) -> i32;

    pub fn ti_lmu_common_set_ramp(lmu_bank: *mut ti_lmu_bank) -> i32;

    pub fn ti_lmu_common_get_ramp_params(
        dev: *mut device,
        child: *mut fwnode_handle,
        lmu_data: *mut ti_lmu_bank,
    ) -> i32;

    pub fn ti_lmu_common_get_brt_res(
        dev: *mut device,
        child: *mut fwnode_handle,
        lmu_data: *mut ti_lmu_bank,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
