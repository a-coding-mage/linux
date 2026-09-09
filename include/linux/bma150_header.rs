/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2011 Bosch Sensortec GmbH
 * Copyright (c) 2011 Unixphere
 */

// C header guard: _BMA150_H_

pub const BMA150_DRIVER: &str = "bma150";

pub const BMA150_RANGE_2G: i32 = 0;
pub const BMA150_RANGE_4G: i32 = 1;
pub const BMA150_RANGE_8G: i32 = 2;

pub const BMA150_BW_25HZ: i32 = 0;
pub const BMA150_BW_50HZ: i32 = 1;
pub const BMA150_BW_100HZ: i32 = 2;
pub const BMA150_BW_190HZ: i32 = 3;
pub const BMA150_BW_375HZ: i32 = 4;
pub const BMA150_BW_750HZ: i32 = 5;
pub const BMA150_BW_1500HZ: i32 = 6;

#[repr(C)]
pub struct bma150_cfg {
    pub any_motion_int: bool, // Set to enable any-motion interrupt
    pub hg_int: bool, // Set to enable high-G interrupt
    pub lg_int: bool, // Set to enable low-G interrupt
    pub any_motion_dur: u8, // Any-motion duration
    pub any_motion_thres: u8, // Any-motion threshold
    pub hg_hyst: u8, // High-G hysterisis
    pub hg_dur: u8, // High-G duration
    pub hg_thres: u8, // High-G threshold
    pub lg_hyst: u8, // Low-G hysterisis
    pub lg_dur: u8, // Low-G duration
    pub lg_thres: u8, // Low-G threshold
    pub range: u8, // one of BMA150_RANGE_xxx
    pub bandwidth: u8, // one of BMA150_BW_xxx
}

#[repr(C)]
pub struct bma150_platform_data {
    pub cfg: bma150_cfg,
    pub irq_gpio_cfg: Option<unsafe extern "C" fn() -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
