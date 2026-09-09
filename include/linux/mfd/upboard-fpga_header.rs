/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * UP Board CPLD/FPGA driver
 *
 * Copyright (c) AAEON. All rights reserved.
 * Copyright (C) 2024 Bootlin
 *
 * Author: Gary Wang <garywang@aaeon.com.tw>
 * Author: Thomas Richard <thomas.richard@bootlin.com>
 *
 */

// Original header guard: __LINUX_MFD_UPBOARD_FPGA_H

pub const UPBOARD_REGISTER_SIZE: u32 = 16;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum upboard_fpgareg {
    UPBOARD_REG_PLATFORM_ID = 0x10,
    UPBOARD_REG_FIRMWARE_ID = 0x11,
    UPBOARD_REG_FUNC_EN0 = 0x20,
    UPBOARD_REG_FUNC_EN1 = 0x21,
    UPBOARD_REG_GPIO_EN0 = 0x30,
    UPBOARD_REG_GPIO_EN1 = 0x31,
    UPBOARD_REG_GPIO_EN2 = 0x32,
    UPBOARD_REG_GPIO_DIR0 = 0x40,
    UPBOARD_REG_GPIO_DIR1 = 0x41,
    UPBOARD_REG_GPIO_DIR2 = 0x42,
    UPBOARD_REG_MAX,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum upboard_fpga_type {
    UPBOARD_UP_FPGA,
    UPBOARD_UP2_FPGA,
}

#[repr(C)]
pub struct upboard_fpga_data {
    pub type_: upboard_fpga_type,
    pub regmap_config: *const regmap_config,
}

#[repr(C)]
pub struct upboard_fpga {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub enable_gpio: *mut gpio_desc,
    pub reset_gpio: *mut gpio_desc,
    pub clear_gpio: *mut gpio_desc,
    pub strobe_gpio: *mut gpio_desc,
    pub datain_gpio: *mut gpio_desc,
    pub dataout_gpio: *mut gpio_desc,
    pub firmware_version: u32,
    pub fpga_data: *const upboard_fpga_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
