// SPDX-License-Identifier: GPL-2.0-only
/*
 * Apple Onboard Audio driver for tas codec (header)
 *
 * Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
 */

pub const TAS_REG_MCS: u32 = 0x01; /* main control */
pub const TAS_MCS_FASTLOAD: u32 = 1 << 7;
pub const TAS_MCS_SCLK64: u32 = 1 << 6;
pub const TAS_MCS_SPORT_MODE_MASK: u32 = 3 << 4;
pub const TAS_MCS_SPORT_MODE_I2S: u32 = 2 << 4;
pub const TAS_MCS_SPORT_MODE_RJ: u32 = 1 << 4;
pub const TAS_MCS_SPORT_MODE_LJ: u32 = 0 << 4;
pub const TAS_MCS_SPORT_WL_MASK: u32 = 3 << 0;
pub const TAS_MCS_SPORT_WL_16BIT: u32 = 0 << 0;
pub const TAS_MCS_SPORT_WL_18BIT: u32 = 1 << 0;
pub const TAS_MCS_SPORT_WL_20BIT: u32 = 2 << 0;
pub const TAS_MCS_SPORT_WL_24BIT: u32 = 3 << 0;

pub const TAS_REG_DRC: u32 = 0x02;
pub const TAS_REG_VOL: u32 = 0x04;
pub const TAS_REG_TREBLE: u32 = 0x05;
pub const TAS_REG_BASS: u32 = 0x06;
pub const TAS_REG_LMIX: u32 = 0x07;
pub const TAS_REG_RMIX: u32 = 0x08;

pub const TAS_REG_ACR: u32 = 0x40; /* analog control */
pub const TAS_ACR_B_MONAUREAL: u32 = 1 << 7;
pub const TAS_ACR_B_MON_SEL_RIGHT: u32 = 1 << 6;
pub const TAS_ACR_DEEMPH_MASK: u32 = 3 << 2;
pub const TAS_ACR_DEEMPH_OFF: u32 = 0 << 2;
pub const TAS_ACR_DEEMPH_48KHz: u32 = 1 << 2;
pub const TAS_ACR_DEEMPH_44KHz: u32 = 2 << 2;
pub const TAS_ACR_INPUT_B: u32 = 1 << 1;
pub const TAS_ACR_ANALOG_PDOWN: u32 = 1 << 0;

pub const TAS_REG_MCS2: u32 = 0x43; /* main control 2 */
pub const TAS_MCS2_ALLPASS: u32 = 1 << 1;

pub const TAS_REG_LEFT_BIQUAD6: u32 = 0x10;
pub const TAS_REG_RIGHT_BIQUAD6: u32 = 0x19;

pub const TAS_REG_LEFT_LOUDNESS: u32 = 0x21;
pub const TAS_REG_RIGHT_LOUDNESS: u32 = 0x22;
pub const TAS_REG_LEFT_LOUDNESS_GAIN: u32 = 0x23;
pub const TAS_REG_RIGHT_LOUDNESS_GAIN: u32 = 0x24;

pub const TAS3001_DRC_MAX: u32 = 0x5f;
pub const TAS3004_DRC_MAX: u32 = 0xef;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
