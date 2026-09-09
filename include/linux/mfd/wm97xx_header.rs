/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * wm97xx client interface
 *
 * Copyright (C) 2017 Robert Jarzmik
 */

// Forward declarations corresponding to the C header's incomplete types.
#[repr(C)]
pub struct regmap;

#[repr(C)]
pub struct wm97xx_batt_pdata;

#[repr(C)]
pub struct snd_ac97;

#[repr(C)]
pub struct wm97xx_platform_data {
    pub ac97: *mut snd_ac97,
    pub regmap: *mut regmap,
    pub batt_pdata: *mut wm97xx_batt_pdata,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
