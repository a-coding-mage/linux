// SPDX-License-Identifier: GPL-2.0-only
//
// aw88399.h --  ALSA SoC AW88399 codec support
//
// Copyright (c) 2023 AWINIC Technology CO., LTD
//
// Author: Weidong Wang <wangweidong.a@awinic.com>
//

// C dependency intent: #include <sound/aw88399.h>

pub const AW88399_I2C_NAME: &str = "aw88399";

pub const AW88399_RATES: u32 = SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_96000;
pub const AW88399_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

pub const FADE_TIME_MAX: u32 = 100000;
pub const FADE_TIME_MIN: u32 = 0;

pub const AW_CALI_READ_CNT_MAX: u32 = 8;
pub const AW88399_DSP_REG_CALRE: u32 = 0x8141;
pub const AW88399_DSP_REG_CALRE_SHIFT: u32 = 10;
pub const AW_CALI_DATA_SUM_RM: u32 = 2;

pub const AW88399_DSP_REG_CFG_MBMEC_ACTAMPTH: u32 = 0x9B4C;
pub const AW88399_DSP_REG_CFG_MBMEC_NOISEAMPTH: u32 = 0x9B4E;
pub const AW88399_DSP_REG_CFG_ADPZ_USTEPN: u32 = 0x9B6E;
pub const AW88399_DSP_REG_CFG_RE_ALPHA: u32 = 0x9BD4;
pub const AW_GET_IV_CNT_MAX: u32 = 6;

pub const AW88399_DSP_VOL_MUTE: u32 = 0xFF00;

pub const AW88399_DSP_LOW_POWER_SWITCH_CFG_ADDR: u32 = 0x9BEC;
pub const AW88399_DSP_LOW_POWER_SWITCH_DISABLE: u32 = 0x110b;

macro_rules! AW88399_PROFILE_EXT {
    ($xname:expr, $profile_info:expr, $profile_get:expr, $profile_set:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            info: $profile_info,
            get: $profile_get,
            put: $profile_set,
            ..Default::default()
        }
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
