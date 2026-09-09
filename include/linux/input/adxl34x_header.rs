/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * include/linux/input/adxl34x.h
 *
 * Digital Accelerometer characteristics are highly application specific
 * and may vary between boards and models. The platform_data for the
 * device's "struct device" holds this information.
 *
 * Copyright 2009 Analog Devices Inc.
 */

// Dependency: declarations from <linux/input.h> are supplied externally.

pub const ADXL_SUPPRESS: u32 = 1 << 3;
pub const ADXL_TAP_X_EN: u32 = 1 << 2;
pub const ADXL_TAP_Y_EN: u32 = 1 << 1;
pub const ADXL_TAP_Z_EN: u32 = 1 << 0;

pub const ADXL_ACT_ACDC: u32 = 1 << 7;
pub const ADXL_ACT_X_EN: u32 = 1 << 6;
pub const ADXL_ACT_Y_EN: u32 = 1 << 5;
pub const ADXL_ACT_Z_EN: u32 = 1 << 4;
pub const ADXL_INACT_ACDC: u32 = 1 << 3;
pub const ADXL_INACT_X_EN: u32 = 1 << 2;
pub const ADXL_INACT_Y_EN: u32 = 1 << 1;
pub const ADXL_INACT_Z_EN: u32 = 1 << 0;

pub const ADXL_FULL_RES: u32 = 1 << 3;
pub const ADXL_RANGE_PM_2g: u32 = 0;
pub const ADXL_RANGE_PM_4g: u32 = 1;
pub const ADXL_RANGE_PM_8g: u32 = 2;
pub const ADXL_RANGE_PM_16g: u32 = 3;

pub const ADXL_LINK: u32 = 1 << 5;
pub const ADXL_AUTO_SLEEP: u32 = 1 << 4;

pub const ADXL_FIFO_BYPASS: u32 = 0;
pub const ADXL_FIFO_FIFO: u32 = 1;
pub const ADXL_FIFO_STREAM: u32 = 2;

pub const ADXL_EN_ORIENTATION_2D: u32 = 1;
pub const ADXL_EN_ORIENTATION_3D: u32 = 2;
pub const ADXL_EN_ORIENTATION_2D_3D: u32 = 3;

pub const ADXL_DEADZONE_ANGLE_0p0: u32 = 0; // !!!0.0 [deg]
pub const ADXL_DEADZONE_ANGLE_3p6: u32 = 1; // 3.6 [deg]
pub const ADXL_DEADZONE_ANGLE_7p2: u32 = 2; // 7.2 [deg]
pub const ADXL_DEADZONE_ANGLE_10p8: u32 = 3; // 10.8 [deg]
pub const ADXL_DEADZONE_ANGLE_14p4: u32 = 4; // 14.4 [deg]
pub const ADXL_DEADZONE_ANGLE_18p0: u32 = 5; // 18.0 [deg]
pub const ADXL_DEADZONE_ANGLE_21p6: u32 = 6; // 21.6 [deg]
pub const ADXL_DEADZONE_ANGLE_25p2: u32 = 7; // 25.2 [deg]

pub const ADXL_LP_FILTER_DIVISOR_2: u32 = 0;
pub const ADXL_LP_FILTER_DIVISOR_4: u32 = 1;
pub const ADXL_LP_FILTER_DIVISOR_8: u32 = 2;
pub const ADXL_LP_FILTER_DIVISOR_16: u32 = 3;
pub const ADXL_LP_FILTER_DIVISOR_32: u32 = 4;
pub const ADXL_LP_FILTER_DIVISOR_64: u32 = 5;
pub const ADXL_LP_FILTER_DIVISOR_128: u32 = 6;
pub const ADXL_LP_FILTER_DIVISOR_256: u32 = 7;

#[repr(C)]
pub struct adxl34x_platform_data {
    pub x_axis_offset: s8,
    pub y_axis_offset: s8,
    pub z_axis_offset: s8,
    pub tap_axis_control: u8,
    pub tap_threshold: u8,
    pub tap_duration: u8,
    pub tap_latency: u8,
    pub tap_window: u8,
    pub act_axis_control: u8,
    pub activity_threshold: u8,
    pub inactivity_threshold: u8,
    pub inactivity_time: u8,
    pub free_fall_threshold: u8,
    pub free_fall_time: u8,
    pub data_rate: u8,
    pub data_range: u8,
    pub low_power_mode: u8,
    pub power_mode: u8,
    pub fifo_mode: u8,
    pub watermark: u8,
    pub ev_type: u32,
    pub ev_code_x: u32,
    pub ev_code_y: u32,
    pub ev_code_z: u32,
    pub ev_code_tap: [u32; 3],
    pub ev_code_ff: u32,
    pub ev_code_act_inactivity: u32,
    pub use_int2: u8,
    pub orientation_enable: u8,
    pub deadzone_angle: u8,
    pub divisor_length: u8,
    pub ev_codes_orient_2d: [u32; 4],
    pub ev_codes_orient_3d: [u32; 6],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
