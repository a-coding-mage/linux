/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Platform data structure for g762 fan controller driver
 *
 * Copyright (C) 2013, Arnaud EBALARD <arno@natisbad.org>
 */

/*
 * Following structure can be used to set g762 driver platform specific data
 * during board init. Note that passing a sparse structure is possible but
 * will result in non-specified attributes to be set to default value, hence
 * overloading those installed during boot (e.g. by u-boot).
 */
#[repr(C)]
pub struct g762_platform_data {
    pub fan_startv: u32,
    pub fan_gear_mode: u32,
    pub pwm_polarity: u32,
    pub clk_freq: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
