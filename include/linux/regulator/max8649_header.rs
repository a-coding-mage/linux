/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Interface of Maxim max8649
 *
 * Copyright (C) 2009-2010 Marvell International Ltd.
 *      Haojian Zhuang <haojian.zhuang@marvell.com>
 */

// Dependency supplied by the surrounding kernel translation:
// #include <linux/regulator/machine.h>

#[repr(i32)]
pub enum Max8649Extclk {
    MAX8649_EXTCLK_26MHZ = 0,
    MAX8649_EXTCLK_13MHZ,
    MAX8649_EXTCLK_19MHZ, // 19.2MHz
}

#[repr(i32)]
pub enum Max8649Ramp {
    MAX8649_RAMP_32MV = 0,
    MAX8649_RAMP_16MV,
    MAX8649_RAMP_8MV,
    MAX8649_RAMP_4MV,
    MAX8649_RAMP_2MV,
    MAX8649_RAMP_1MV,
    MAX8649_RAMP_0_5MV,
    MAX8649_RAMP_0_25MV,
}

#[repr(C)]
pub struct max8649_platform_data {
    pub regulator: *mut regulator_init_data,

    // C bit-fields: mode:2, extclk_freq:2, extclk:1, ramp_timing:3,
    // and ramp_down:1. Rust has no native bit-field syntax; each is retained
    // as its declared unsigned storage unit for source-level access.
    pub mode: u32,        // bit[1:0] = VID1,VID0
    pub extclk_freq: u32,
    pub extclk: u32,
    pub ramp_timing: u32,
    pub ramp_down: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
