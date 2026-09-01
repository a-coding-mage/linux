// SPDX-License-Identifier: GPL-2.0-only
//
// aw88395.h --  ALSA SoC AW88395 codec support
//
// Copyright (c) 2022-2023 AWINIC Technology CO., LTD
//
// Author: Bruce zhao <zhaolei@awinic.com>
//

pub const AW88395_CHIP_ID_REG: u32 = 0x00;
pub const AW88395_START_RETRIES: u32 = 5;
pub const AW88395_START_WORK_DELAY_MS: u32 = 0;

pub const AW88395_DSP_16_DATA_MASK: u32 = 0x0000ffff;

pub const AW88395_I2C_NAME: &str = "aw88395";

pub const AW88395_RATES: u32 = SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_96000;
pub const AW88395_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

pub const FADE_TIME_MAX: u32 = 100000;
pub const FADE_TIME_MIN: u32 = 0;

macro_rules! AW88395_PROFILE_EXT {
    ($xname:expr, $profile_info:expr, $profile_get:expr, $profile_set:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            info: $profile_info,
            get: $profile_get,
            put: $profile_set,
        }
    };
}

pub(crate) use AW88395_PROFILE_EXT;

pub const AW88395_SYNC_START: u32 = 0;
pub const AW88395_ASYNC_START: u32 = 1;

pub const AW88395_STREAM_CLOSE: u32 = 0;
pub const AW88395_STREAM_OPEN: u32 = 1;

#[repr(C)]
pub struct aw88395 {
    pub aw_pa: *mut aw_device,
    pub lock: mutex,
    pub reset_gpio: *mut gpio_desc,
    pub start_work: delayed_work,
    pub regmap: *mut regmap,
    pub aw_cfg: *mut aw_container,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
