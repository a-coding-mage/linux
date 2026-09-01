// SPDX-License-Identifier: GPL-2.0-only
/*
 * Apple Onboard Audio driver for Onyx codec (header)
 *
 * Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
 */

// C header dependencies removed from executable Rust:
// #include <linux/i2c.h>
// #include <asm/pmac_low_i2c.h>

/* PCM3052 register definitions */

/* the attenuation registers take values from
 * -1 (0dB) to -127 (-63.0 dB) or others (muted) */
pub const ONYX_REG_DAC_ATTEN_LEFT: u32 = 65;
pub const FIRSTREGISTER: u32 = ONYX_REG_DAC_ATTEN_LEFT;
pub const ONYX_REG_DAC_ATTEN_RIGHT: u32 = 66;

pub const ONYX_REG_CONTROL: u32 = 67;
pub const ONYX_MRST: u32 = 1 << 7;
pub const ONYX_SRST: u32 = 1 << 6;
pub const ONYX_ADPSV: u32 = 1 << 5;
pub const ONYX_DAPSV: u32 = 1 << 4;
pub const ONYX_SILICONVERSION: u32 = 1 << 0;
/* all others reserved */

pub const ONYX_REG_DAC_CONTROL: u32 = 68;
pub const ONYX_OVR1: u32 = 1 << 6;
pub const ONYX_MUTE_RIGHT: u32 = 1 << 1;
pub const ONYX_MUTE_LEFT: u32 = 1 << 0;

pub const ONYX_REG_DAC_DEEMPH: u32 = 69;
pub const ONYX_DIGDEEMPH_SHIFT: u32 = 5;
pub const ONYX_DIGDEEMPH_MASK: u32 = 3 << ONYX_DIGDEEMPH_SHIFT;
pub const ONYX_DIGDEEMPH_CTRL: u32 = 1 << 4;

pub const ONYX_REG_DAC_FILTER: u32 = 70;
pub const ONYX_ROLLOFF_FAST: u32 = 1 << 5;
pub const ONYX_DAC_FILTER_ALWAYS: u32 = 1 << 2;

pub const ONYX_REG_DAC_OUTPHASE: u32 = 71;
pub const ONYX_OUTPHASE_INVERTED: u32 = 1 << 0;

pub const ONYX_REG_ADC_CONTROL: u32 = 72;
pub const ONYX_ADC_INPUT_MIC: u32 = 1 << 5;
/* 8 + input gain in dB, valid range for input gain is -4 .. 20 dB */
pub const ONYX_ADC_PGA_GAIN_MASK: u32 = 0x1f;

pub const ONYX_REG_ADC_HPF_BYPASS: u32 = 75;
pub const ONYX_HPF_DISABLE: u32 = 1 << 3;
pub const ONYX_ADC_HPF_ALWAYS: u32 = 1 << 2;

pub const ONYX_REG_DIG_INFO1: u32 = 77;
pub const ONYX_MASK_DIN_TO_BPZ: u32 = 1 << 7;
/* bits 1-5 control channel bits 1-5 */
pub const ONYX_DIGOUT_DISABLE: u32 = 1 << 0;

pub const ONYX_REG_DIG_INFO2: u32 = 78;
/* controls channel bits 8-15 */

pub const ONYX_REG_DIG_INFO3: u32 = 79;
/* control channel bits 24-29, high 2 bits reserved */

pub const ONYX_REG_DIG_INFO4: u32 = 80;
pub const ONYX_VALIDL: u32 = 1 << 7;
pub const ONYX_VALIDR: u32 = 1 << 6;
pub const ONYX_SPDIF_ENABLE: u32 = 1 << 5;
/* lower 4 bits control bits 32-35 of channel control and word length */
pub const ONYX_WORDLEN_MASK: u32 = 0xF;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
