// SPDX-License-Identifier: GPL-2.0
// Definitions for PCM1789 audio driver
// Copyright (C) 2018 Bootlin
// Mylene Josserand <mylene.josserand@bootlin.com>

// C dependencies from the original header:
// SNDRV_PCM_FMTBIT_S32_LE, SNDRV_PCM_FMTBIT_S24_LE, SNDRV_PCM_FMTBIT_S16_LE,
// struct regmap_config, struct device, and struct regmap are supplied elsewhere.

pub const PCM1789_FORMATS: u32 =
    SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S16_LE;

unsafe extern "C" {
    pub static pcm1789_regmap_config: regmap_config;

    pub fn pcm1789_common_init(dev: *mut device, regmap: *mut regmap) -> ::core::ffi::c_int;
    pub fn pcm1789_common_exit(dev: *mut device);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
