/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * LP55XX Platform Data Header
 *
 * Copyright (C) 2012 Texas Instruments
 *
 * Author: Milo(Woogyom) Kim <milo.kim@ti.com>
 *
 * Derived from leds-lp5521.h, leds-lp5523.h
 */

// C header guard and Linux includes omitted; referenced external types and
// constants are supplied by the surrounding translation unit.

/* Clock configuration */
pub const LP55XX_CLOCK_AUTO: u32 = 0;
pub const LP55XX_CLOCK_INT: u32 = 1;
pub const LP55XX_CLOCK_EXT: u32 = 2;

pub const LP55XX_MAX_GROUPED_CHAN: u32 = 4;

#[repr(C)]
pub struct lp55xx_led_config {
    pub name: *const core::ffi::c_char,
    pub default_trigger: *const core::ffi::c_char,
    pub chan_nr: u8,
    pub led_current: u8, /* mA x10, 0 if led is not connected */
    pub max_current: u8,
    pub num_colors: core::ffi::c_int,
    pub max_channel: u32,
    pub color_id: [core::ffi::c_int; LED_COLOR_ID_MAX],
    pub output_num: [core::ffi::c_int; LED_COLOR_ID_MAX],
}

#[repr(C)]
pub struct lp55xx_predef_pattern {
    pub r: *const u8,
    pub g: *const u8,
    pub b: *const u8,
    pub size_r: u8,
    pub size_g: u8,
    pub size_b: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum lp8501_pwr_sel {
    LP8501_ALL_VDD, /* D1~9 are connected to VDD */
    LP8501_6VDD_3VOUT, /* D1~6 with VDD, D7~9 with VOUT */
    LP8501_3VDD_6VOUT, /* D1~6 with VOUT, D7~9 with VDD */
    LP8501_ALL_VOUT, /* D1~9 are connected to VOUT */
}

/*
 * struct lp55xx_platform_data
 * @led_config        : Configurable led class device
 * @num_channels      : Number of LED channels
 * @label             : Used for naming LEDs
 * @clock_mode        : Input clock mode. LP55XX_CLOCK_AUTO or _INT or _EXT
 * @setup_resources   : Platform specific function before enabling the chip
 * @release_resources : Platform specific function after  disabling the chip
 * @enable_gpiod      : enable GPIO descriptor
 * @patterns          : Predefined pattern data for RGB channels
 * @num_patterns      : Number of patterns
 * @update_config     : Value of CONFIG register
 */
#[repr(C)]
pub struct lp55xx_platform_data {
    /* LED channel configuration */
    pub led_config: *mut lp55xx_led_config,
    pub num_channels: u8,
    pub label: *const core::ffi::c_char,

    /* Clock configuration */
    pub clock_mode: u8,

    /* Charge pump mode */
    pub charge_pump_mode: u32,

    /* optional enable GPIO */
    pub enable_gpiod: *mut gpio_desc,

    /* Predefined pattern data */
    pub patterns: *mut lp55xx_predef_pattern,
    pub num_patterns: u32,

    /* LP8501 specific */
    pub pwr_sel: lp8501_pwr_sel,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
