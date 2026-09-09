/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * A V4L2 driver for OmniVision OV7670 cameras.
 *
 * Copyright 2010 One Laptop Per Child
 */

#[repr(C)]
pub struct ov7670_config {
    pub min_width: ::core::ffi::c_int,    /* Filter out smaller sizes */
    pub min_height: ::core::ffi::c_int,   /* Filter out smaller sizes */
    pub clock_speed: ::core::ffi::c_int, /* External clock speed (MHz) */
    pub use_smbus: bool,                 /* Use smbus I/O instead of I2C */
    pub pll_bypass: bool,                /* Choose whether to bypass the PLL */
    pub pclk_hb_disable: bool, /* Disable toggling pixclk during horizontal blanking */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
