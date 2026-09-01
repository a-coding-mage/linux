// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * definitions for PCM179X
 *
 * Copyright 2013 Amarula Solutions
 */

// C header dependencies: `struct device`, `struct regmap`,
// `struct regmap_config`, `snd_pcm_format_t`, and SNDRV_PCM_FMTBIT_* are
// supplied by other translated headers.
use crate::{
    device, regmap, regmap_config, snd_pcm_format_t, SNDRV_PCM_FMTBIT_S16_LE,
    SNDRV_PCM_FMTBIT_S24_LE, SNDRV_PCM_FMTBIT_S32_LE,
};

pub const PCM1792A_FORMATS: snd_pcm_format_t =
    SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S16_LE;

unsafe extern "C" {
    pub static pcm179x_regmap_config: regmap_config;

    pub fn pcm179x_common_init(
        dev: *mut device,
        regmap: *mut regmap,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
