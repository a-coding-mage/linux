/* SPDX-License-Identifier: GPL-2.0 */
/*
 * tda1997x - NXP HDMI receiver
 *
 * Copyright 2017 Tim Harvey <tharvey@gateworks.com>
 *
 */

/* Platform Data */
#[repr(C)]
pub struct tda1997x_platform_data {
    pub vidout_bus_type: v4l2_mbus_type,
    pub vidout_bus_width: u32,
    pub vidout_port_cfg: [u8; 9],
    /* pin polarity (1=invert) */
    pub vidout_inv_de: bool,
    pub vidout_inv_hs: bool,
    pub vidout_inv_vs: bool,
    pub vidout_inv_pclk: bool,
    /* clock delays (0=-8, 1=-7 ... 15=+7 pixels) */
    pub vidout_delay_hs: u8,
    pub vidout_delay_vs: u8,
    pub vidout_delay_de: u8,
    pub vidout_delay_pclk: u8,
    /* sync selections (controls how sync pins are derived) */
    pub vidout_sel_hs: u8,
    pub vidout_sel_vs: u8,
    pub vidout_sel_de: u8,

    /* Audio Port Output */
    pub audout_format: i32,
    pub audout_mclk_fs: u32, /* clock multiplier */
    pub audout_width: u32, /* 13 or 32 bit */
    pub audout_layout: u32, /* layout0=AP0 layout1=AP0,AP1,AP2,AP3 */
    pub audout_layoutauto: bool, /* audio layout dictated by pkt header */
    pub audout_invert_clk: bool, /* data valid on rising edge of BCLK */
    pub audio_auto_mute: bool, /* enable hardware audio auto-mute */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
