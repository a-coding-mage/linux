/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * timb_radio.h Platform struct for the Timberdale radio driver
 * Copyright (c) 2009 Intel Corporation
 */

// Dependency supplied by <linux/i2c.h>.
pub struct i2c_board_info;

#[repr(C)]
pub struct timb_radio_platform_data {
    /* I2C adapter where the tuner and dsp are attached */
    pub i2c_adapter: i32,
    pub tuner: *mut i2c_board_info,
    pub dsp: *mut i2c_board_info,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
