/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * timb_video.h Platform struct for the Timberdale video driver
 * Copyright (c) 2009-2010 Intel Corporation
 */

// Dependency supplied by the surrounding I2C bindings:
// struct i2c_board_info;

#[repr(C)]
pub struct timb_video_platform_data_encoder {
    pub module_name: *const core::ffi::c_char,
    pub info: *mut i2c_board_info,
}

#[repr(C)]
pub struct timb_video_platform_data {
    pub dma_channel: core::ffi::c_int,
    /* The I2C adapter where the encoder is attached */
    pub i2c_adapter: core::ffi::c_int,
    pub encoder: timb_video_platform_data_encoder,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
