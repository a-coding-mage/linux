/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * File: include/linux/omapfb.h
 *
 * Framebuffer driver for TI OMAP boards
 *
 * Copyright (C) 2004 Nokia Corporation
 * Author: Imre Deak <imre.deak@nokia.com>
 */

// Dependency intent: declarations from <uapi/linux/omapfb.h> are supplied by
// other translated files.

#[repr(C)]
pub struct omap_lcd_config {
    pub panel_name: [core::ffi::c_char; 16],
    pub ctrl_name: [core::ffi::c_char; 16],
    pub nreset_gpio: i16,
    pub data_lines: u8,
}

#[repr(C)]
pub struct omapfb_platform_data {
    pub lcd: omap_lcd_config,
}

// The C __init annotation is a build/link-time attribute with no direct
// file-local Rust equivalent.
unsafe extern "C" {
    pub fn omapfb_set_lcd_config(config: *const omap_lcd_config);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
