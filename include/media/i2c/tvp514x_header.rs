/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * drivers/media/video/tvp514x.h
 *
 * Copyright (C) 2008 Texas Instruments Inc
 * Author: Vaibhav Hiremath <hvaibhav@ti.com>
 *
 * Contributors:
 *     Sivaraj R <sivaraj@ti.com>
 *     Brijesh R Jadav <brijesh.j@ti.com>
 *     Hardik Shah <hardik.shah@ti.com>
 *     Manjunath Hadli <mrh@ti.com>
 *     Karicheri Muralidharan <m-karicheri2@ti.com>
 */

/* Other macros */
pub const TVP514X_MODULE_NAME: &str = "tvp514x";

pub const TVP514X_XCLK_BT656: i32 = 27000000;

/* Number of pixels and number of lines per frame for different standards */
pub const NTSC_NUM_ACTIVE_PIXELS: i32 = 720;
pub const NTSC_NUM_ACTIVE_LINES: i32 = 480;
pub const PAL_NUM_ACTIVE_PIXELS: i32 = 720;
pub const PAL_NUM_ACTIVE_LINES: i32 = 576;

/* enum for different decoder input pin configuration */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tvp514x_input {
    /*
     * CVBS input selection
     */
    INPUT_CVBS_VI1A = 0x0,
    INPUT_CVBS_VI1B,
    INPUT_CVBS_VI1C,
    INPUT_CVBS_VI2A = 0x04,
    INPUT_CVBS_VI2B,
    INPUT_CVBS_VI2C,
    INPUT_CVBS_VI3A = 0x08,
    INPUT_CVBS_VI3B,
    INPUT_CVBS_VI3C,
    INPUT_CVBS_VI4A = 0x0C,
    /*
     * S-Video input selection
     */
    INPUT_SVIDEO_VI2A_VI1A = 0x44,
    INPUT_SVIDEO_VI2B_VI1B,
    INPUT_SVIDEO_VI2C_VI1C,
    INPUT_SVIDEO_VI2A_VI3A = 0x54,
    INPUT_SVIDEO_VI2B_VI3B,
    INPUT_SVIDEO_VI2C_VI3C,
    INPUT_SVIDEO_VI4A_VI1A = 0x4C,
    INPUT_SVIDEO_VI4A_VI1B,
    INPUT_SVIDEO_VI4A_VI1C,
    INPUT_SVIDEO_VI4A_VI3A = 0x5C,
    INPUT_SVIDEO_VI4A_VI3B,
    INPUT_SVIDEO_VI4A_VI3C,

    /* Need to add entries for
     * RGB, YPbPr and SCART.
     */
    INPUT_INVALID,
}

/* enum for output format supported. */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tvp514x_output {
    OUTPUT_10BIT_422_EMBEDDED_SYNC = 0,
    OUTPUT_20BIT_422_SEPERATE_SYNC,
    OUTPUT_10BIT_422_SEPERATE_SYNC = 3,
    OUTPUT_INVALID,
}

/**
 * struct tvp514x_platform_data - Platform data values and access functions.
 * @clk_polarity: Clock polarity of the current interface.
 * @hs_polarity: HSYNC Polarity configuration for current interface.
 * @vs_polarity: VSYNC Polarity configuration for current interface.
 */
#[repr(C)]
pub struct tvp514x_platform_data {
    /* Interface control params */
    pub clk_polarity: bool,
    pub hs_polarity: bool,
    pub vs_polarity: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
