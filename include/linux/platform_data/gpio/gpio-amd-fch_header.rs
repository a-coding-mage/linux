/* SPDX-License-Identifier: GPL-2.0+ */

/*
 * AMD FCH gpio driver platform-data
 *
 * Copyright (C) 2018 metux IT consult
 * Author: Enrico Weigelt <info@metux.net>
 *
 */

pub const AMD_FCH_GPIO_DRIVER_NAME: &str = "gpio_amd_fch";

/*
 * gpio register index definitions
 */
pub const AMD_FCH_GPIO_REG_GPIO49: i32 = 0x40;
pub const AMD_FCH_GPIO_REG_GPIO50: i32 = 0x41;
pub const AMD_FCH_GPIO_REG_GPIO51: i32 = 0x42;
pub const AMD_FCH_GPIO_REG_GPIO55_DEVSLP0: i32 = 0x43;
pub const AMD_FCH_GPIO_REG_GPIO57: i32 = 0x44;
pub const AMD_FCH_GPIO_REG_GPIO58: i32 = 0x45;
pub const AMD_FCH_GPIO_REG_GPIO59_DEVSLP1: i32 = 0x46;
pub const AMD_FCH_GPIO_REG_GPIO64: i32 = 0x47;
pub const AMD_FCH_GPIO_REG_GPIO68: i32 = 0x48;
pub const AMD_FCH_GPIO_REG_GPIO66_SPKR: i32 = 0x5B;
pub const AMD_FCH_GPIO_REG_GPIO71: i32 = 0x4D;
pub const AMD_FCH_GPIO_REG_GPIO32_GE1: i32 = 0x59;
pub const AMD_FCH_GPIO_REG_GPIO33_GE2: i32 = 0x5A;
pub const AMT_FCH_GPIO_REG_GEVT22: i32 = 0x09;

/*
 * struct amd_fch_gpio_pdata - GPIO chip platform data
 * @gpio_num: number of entries
 * @gpio_reg: array of gpio registers
 * @gpio_names: array of gpio names
 */
#[repr(C)]
pub struct amd_fch_gpio_pdata {
    pub gpio_num: i32,
    pub gpio_reg: *mut i32,
    pub gpio_names: *const *const core::ffi::c_char,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
