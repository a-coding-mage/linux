/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2022 Analog Devices Inc. */

pub const MAX98363_R2000_SW_RESET: u32 = 0x2000;
pub const MAX98363_R2001_INTR_RAW: u32 = 0x2001;
pub const MAX98363_R2003_INTR_STATE: u32 = 0x2003;
pub const MAX98363_R2005_INTR_FALG: u32 = 0x2005;
pub const MAX98363_R2007_INTR_EN: u32 = 0x2007;
pub const MAX98363_R2009_INTR_CLR: u32 = 0x2009;
pub const MAX98363_R2021_ERR_MON_CTRL: u32 = 0x2021;
pub const MAX98363_R2022_SPK_MON_THRESH: u32 = 0x2022;
pub const MAX98363_R2023_SPK_MON_DURATION: u32 = 0x2023;
pub const MAX98363_R2030_TONE_GEN_CFG: u32 = 0x2030;
pub const MAX98363_R203F_TONE_GEN_EN: u32 = 0x203F;
pub const MAX98363_R2040_AMP_VOL: u32 = 0x2040;
pub const MAX98363_R2041_AMP_GAIN: u32 = 0x2041;
pub const MAX98363_R2042_DSP_CFG: u32 = 0x2042;
pub const MAX98363_R21FF_REV_ID: u32 = 0x21FF;

/* MAX98363_R2021_ERR_MON_CTRL */
pub const MAX98363_SPKMON_SHIFT: u32 = 3;
pub const MAX98363_CLOCK_MON_SHIFT: u32 = 0;

/* MAX98363_R2042_DSP_CFG */
pub const MAX98363_AMP_DSP_CFG_RMP_SHIFT: u32 = 3;

#[repr(C)]
pub struct max98363_priv {
    pub regmap: *mut regmap,
    pub slave: *mut sdw_slave,
    pub hw_init: bool,
    pub first_hw_init: bool,
}

// External dependency types from included kernel headers in the original repository.
pub enum regmap {}
pub enum sdw_slave {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
