/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * da7219.h - DA7219 ALSA SoC Codec Driver
 *
 * Copyright (c) 2015 Dialog Semiconductor
 *
 * Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
 */

use core::ffi::{c_int, c_uint};

/*
 * Includes from the C header describe external kernel/ALSA dependencies:
 * linux/clk.h, linux/clkdev.h, linux/clk-provider.h, linux/regmap.h,
 * linux/regulator/consumer.h, sound/da7219.h
 */

/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * da7219.h - DA7219 ALSA SoC Codec Driver
 *
 * Copyright (c) 2015 Dialog Semiconductor
 *
 * Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
 */
/*
 * Registers
 */
pub const DA7219_MIC_1_GAIN_STATUS: u32 = 0x6;
pub const DA7219_MIXIN_L_GAIN_STATUS: u32 = 0x8;
pub const DA7219_ADC_L_GAIN_STATUS: u32 = 0xA;
pub const DA7219_DAC_L_GAIN_STATUS: u32 = 0xC;
pub const DA7219_DAC_R_GAIN_STATUS: u32 = 0xD;
pub const DA7219_HP_L_GAIN_STATUS: u32 = 0xE;
pub const DA7219_HP_R_GAIN_STATUS: u32 = 0xF;
pub const DA7219_MIC_1_SELECT: u32 = 0x10;
pub const DA7219_CIF_TIMEOUT_CTRL: u32 = 0x12;
pub const DA7219_CIF_CTRL: u32 = 0x13;
pub const DA7219_SR_24_48: u32 = 0x16;
pub const DA7219_SR: u32 = 0x17;
pub const DA7219_CIF_I2C_ADDR_CFG: u32 = 0x1B;
pub const DA7219_PLL_CTRL: u32 = 0x20;
pub const DA7219_PLL_FRAC_TOP: u32 = 0x22;
pub const DA7219_PLL_FRAC_BOT: u32 = 0x23;
pub const DA7219_PLL_INTEGER: u32 = 0x24;
pub const DA7219_PLL_SRM_STS: u32 = 0x25;
pub const DA7219_DIG_ROUTING_DAI: u32 = 0x2A;
pub const DA7219_DAI_CLK_MODE: u32 = 0x2B;
pub const DA7219_DAI_CTRL: u32 = 0x2C;
pub const DA7219_DAI_TDM_CTRL: u32 = 0x2D;
pub const DA7219_DIG_ROUTING_DAC: u32 = 0x2E;
pub const DA7219_ALC_CTRL1: u32 = 0x2F;
pub const DA7219_DAI_OFFSET_LOWER: u32 = 0x30;
pub const DA7219_DAI_OFFSET_UPPER: u32 = 0x31;
pub const DA7219_REFERENCES: u32 = 0x32;
pub const DA7219_MIXIN_L_SELECT: u32 = 0x33;
pub const DA7219_MIXIN_L_GAIN: u32 = 0x34;
pub const DA7219_ADC_L_GAIN: u32 = 0x36;
pub const DA7219_ADC_FILTERS1: u32 = 0x38;
pub const DA7219_MIC_1_GAIN: u32 = 0x39;
pub const DA7219_SIDETONE_CTRL: u32 = 0x3A;
pub const DA7219_SIDETONE_GAIN: u32 = 0x3B;
pub const DA7219_DROUTING_ST_OUTFILT_1L: u32 = 0x3C;
pub const DA7219_DROUTING_ST_OUTFILT_1R: u32 = 0x3D;
pub const DA7219_DAC_FILTERS5: u32 = 0x40;
pub const DA7219_DAC_FILTERS2: u32 = 0x41;
pub const DA7219_DAC_FILTERS3: u32 = 0x42;
pub const DA7219_DAC_FILTERS4: u32 = 0x43;
pub const DA7219_DAC_FILTERS1: u32 = 0x44;
pub const DA7219_DAC_L_GAIN: u32 = 0x45;
pub const DA7219_DAC_R_GAIN: u32 = 0x46;
pub const DA7219_CP_CTRL: u32 = 0x47;
pub const DA7219_HP_L_GAIN: u32 = 0x48;
pub const DA7219_HP_R_GAIN: u32 = 0x49;
pub const DA7219_MIXOUT_L_SELECT: u32 = 0x4B;
pub const DA7219_MIXOUT_R_SELECT: u32 = 0x4C;
pub const DA7219_SYSTEM_MODES_INPUT: u32 = 0x50;
pub const DA7219_SYSTEM_MODES_OUTPUT: u32 = 0x51;
pub const DA7219_MICBIAS_CTRL: u32 = 0x62;
pub const DA7219_MIC_1_CTRL: u32 = 0x63;
pub const DA7219_MIXIN_L_CTRL: u32 = 0x65;
pub const DA7219_ADC_L_CTRL: u32 = 0x67;
pub const DA7219_DAC_L_CTRL: u32 = 0x69;
pub const DA7219_DAC_R_CTRL: u32 = 0x6A;
pub const DA7219_HP_L_CTRL: u32 = 0x6B;
pub const DA7219_HP_R_CTRL: u32 = 0x6C;
pub const DA7219_MIXOUT_L_CTRL: u32 = 0x6E;
pub const DA7219_MIXOUT_R_CTRL: u32 = 0x6F;
pub const DA7219_CHIP_ID1: u32 = 0x81;
pub const DA7219_CHIP_ID2: u32 = 0x82;
pub const DA7219_CHIP_REVISION: u32 = 0x83;
pub const DA7219_IO_CTRL: u32 = 0x91;
pub const DA7219_GAIN_RAMP_CTRL: u32 = 0x92;
pub const DA7219_PC_COUNT: u32 = 0x94;
pub const DA7219_CP_VOL_THRESHOLD1: u32 = 0x95;
pub const DA7219_CP_DELAY: u32 = 0x96;
pub const DA7219_DIG_CTRL: u32 = 0x99;
pub const DA7219_ALC_CTRL2: u32 = 0x9A;
pub const DA7219_ALC_CTRL3: u32 = 0x9B;
pub const DA7219_ALC_NOISE: u32 = 0x9C;
pub const DA7219_ALC_TARGET_MIN: u32 = 0x9D;
pub const DA7219_ALC_TARGET_MAX: u32 = 0x9E;
pub const DA7219_ALC_GAIN_LIMITS: u32 = 0x9F;
pub const DA7219_ALC_ANA_GAIN_LIMITS: u32 = 0xA0;
pub const DA7219_ALC_ANTICLIP_CTRL: u32 = 0xA1;
pub const DA7219_ALC_ANTICLIP_LEVEL: u32 = 0xA2;
pub const DA7219_ALC_OFFSET_AUTO_M_L: u32 = 0xA3;
pub const DA7219_ALC_OFFSET_AUTO_U_L: u32 = 0xA4;
pub const DA7219_DAC_NG_SETUP_TIME: u32 = 0xAF;
pub const DA7219_DAC_NG_OFF_THRESH: u32 = 0xB0;
pub const DA7219_DAC_NG_ON_THRESH: u32 = 0xB1;
pub const DA7219_DAC_NG_CTRL: u32 = 0xB2;
pub const DA7219_TONE_GEN_CFG1: u32 = 0xB4;
pub const DA7219_TONE_GEN_CFG2: u32 = 0xB5;
pub const DA7219_TONE_GEN_CYCLES: u32 = 0xB6;
pub const DA7219_TONE_GEN_FREQ1_L: u32 = 0xB7;
pub const DA7219_TONE_GEN_FREQ1_U: u32 = 0xB8;
pub const DA7219_TONE_GEN_FREQ2_L: u32 = 0xB9;
pub const DA7219_TONE_GEN_FREQ2_U: u32 = 0xBA;
pub const DA7219_TONE_GEN_ON_PER: u32 = 0xBB;
pub const DA7219_TONE_GEN_OFF_PER: u32 = 0xBC;
pub const DA7219_SYSTEM_STATUS: u32 = 0xE0;
pub const DA7219_SYSTEM_ACTIVE: u32 = 0xFD;
/*
 * Bit Fields
 */
pub const DA7219_SWITCH_EN_MAX: u32 = 0x1;
/* DA7219_MIC_1_GAIN_STATUS = 0x6 */
pub const DA7219_MIC_1_AMP_GAIN_STATUS_SHIFT: u32 = 0;
pub const DA7219_MIC_1_AMP_GAIN_STATUS_MASK: u32 = 0x7 << 0;
pub const DA7219_MIC_1_AMP_GAIN_MAX: u32 = 0x7;
/* DA7219_MIXIN_L_GAIN_STATUS = 0x8 */
pub const DA7219_MIXIN_L_AMP_GAIN_STATUS_SHIFT: u32 = 0;
pub const DA7219_MIXIN_L_AMP_GAIN_STATUS_MASK: u32 = 0xF << 0;
/* DA7219_ADC_L_GAIN_STATUS = 0xA */
pub const DA7219_ADC_L_DIGITAL_GAIN_STATUS_SHIFT: u32 = 0;
pub const DA7219_ADC_L_DIGITAL_GAIN_STATUS_MASK: u32 = 0x7F << 0;
/* DA7219_DAC_L_GAIN_STATUS = 0xC */
pub const DA7219_DAC_L_DIGITAL_GAIN_STATUS_SHIFT: u32 = 0;
pub const DA7219_DAC_L_DIGITAL_GAIN_STATUS_MASK: u32 = 0x7F << 0;
/* DA7219_DAC_R_GAIN_STATUS = 0xD */
pub const DA7219_DAC_R_DIGITAL_GAIN_STATUS_SHIFT: u32 = 0;
pub const DA7219_DAC_R_DIGITAL_GAIN_STATUS_MASK: u32 = 0x7F << 0;
/* DA7219_HP_L_GAIN_STATUS = 0xE */
pub const DA7219_HP_L_AMP_GAIN_STATUS_SHIFT: u32 = 0;
pub const DA7219_HP_L_AMP_GAIN_STATUS_MASK: u32 = 0x3F << 0;
/* DA7219_HP_R_GAIN_STATUS = 0xF */
pub const DA7219_HP_R_AMP_GAIN_STATUS_SHIFT: u32 = 0;
pub const DA7219_HP_R_AMP_GAIN_STATUS_MASK: u32 = 0x3F << 0;
/* DA7219_MIC_1_SELECT = 0x10 */
pub const DA7219_MIC_1_AMP_IN_SEL_SHIFT: u32 = 0;
pub const DA7219_MIC_1_AMP_IN_SEL_MASK: u32 = 0x3 << 0;
/* DA7219_CIF_TIMEOUT_CTRL = 0x12 */
pub const DA7219_I2C_TIMEOUT_EN_SHIFT: u32 = 0;
pub const DA7219_I2C_TIMEOUT_EN_MASK: u32 = 0x1 << 0;
/* DA7219_CIF_CTRL = 0x13 */
pub const DA7219_CIF_I2C_WRITE_MODE_SHIFT: u32 = 0;
pub const DA7219_CIF_I2C_WRITE_MODE_MASK: u32 = 0x1 << 0;
pub const DA7219_CIF_REG_SOFT_RESET_SHIFT: u32 = 7;
pub const DA7219_CIF_REG_SOFT_RESET_MASK: u32 = 0x1 << 7;
/* DA7219_SR_24_48 = 0x16 */
pub const DA7219_SR_24_48_SHIFT: u32 = 0;
pub const DA7219_SR_24_48_MASK: u32 = 0x1 << 0;
/* DA7219_SR = 0x17 */
pub const DA7219_SR_SHIFT: u32 = 0;
pub const DA7219_SR_MASK: u32 = 0xF << 0;
pub const DA7219_SR_8000: u32 = 0x01 << 0;
pub const DA7219_SR_11025: u32 = 0x02 << 0;
pub const DA7219_SR_12000: u32 = 0x03 << 0;
pub const DA7219_SR_16000: u32 = 0x05 << 0;
pub const DA7219_SR_22050: u32 = 0x06 << 0;
pub const DA7219_SR_24000: u32 = 0x07 << 0;
pub const DA7219_SR_32000: u32 = 0x09 << 0;
pub const DA7219_SR_44100: u32 = 0x0A << 0;
pub const DA7219_SR_48000: u32 = 0x0B << 0;
pub const DA7219_SR_88200: u32 = 0x0E << 0;
pub const DA7219_SR_96000: u32 = 0x0F << 0;
/* DA7219_CIF_I2C_ADDR_CFG = 0x1B */
pub const DA7219_CIF_I2C_ADDR_CFG_SHIFT: u32 = 0;
pub const DA7219_CIF_I2C_ADDR_CFG_MASK: u32 = 0x3 << 0;
/* DA7219_PLL_CTRL = 0x20 */
pub const DA7219_PLL_INDIV_SHIFT: u32 = 2;
pub const DA7219_PLL_INDIV_MASK: u32 = 0x7 << 2;
pub const DA7219_PLL_INDIV_2_TO_4_5_MHZ: u32 = 0x0 << 2;
pub const DA7219_PLL_INDIV_4_5_TO_9_MHZ: u32 = 0x1 << 2;
pub const DA7219_PLL_INDIV_9_TO_18_MHZ: u32 = 0x2 << 2;
pub const DA7219_PLL_INDIV_18_TO_36_MHZ: u32 = 0x3 << 2;
pub const DA7219_PLL_INDIV_36_TO_54_MHZ: u32 = 0x4 << 2;
pub const DA7219_PLL_MCLK_SQR_EN_SHIFT: u32 = 5;
pub const DA7219_PLL_MCLK_SQR_EN_MASK: u32 = 0x1 << 5;
pub const DA7219_PLL_MODE_SHIFT: u32 = 6;
pub const DA7219_PLL_MODE_MASK: u32 = 0x3 << 6;
pub const DA7219_PLL_MODE_BYPASS: u32 = 0x0 << 6;
pub const DA7219_PLL_MODE_NORMAL: u32 = 0x1 << 6;
pub const DA7219_PLL_MODE_SRM: u32 = 0x2 << 6;
/* DA7219_PLL_FRAC_TOP = 0x22 */
pub const DA7219_PLL_FBDIV_FRAC_TOP_SHIFT: u32 = 0;
pub const DA7219_PLL_FBDIV_FRAC_TOP_MASK: u32 = 0x1F << 0;
/* DA7219_PLL_FRAC_BOT = 0x23 */
pub const DA7219_PLL_FBDIV_FRAC_BOT_SHIFT: u32 = 0;
pub const DA7219_PLL_FBDIV_FRAC_BOT_MASK: u32 = 0xFF << 0;
/* DA7219_PLL_INTEGER = 0x24 */
pub const DA7219_PLL_FBDIV_INTEGER_SHIFT: u32 = 0;
pub const DA7219_PLL_FBDIV_INTEGER_MASK: u32 = 0x7F << 0;
/* DA7219_PLL_SRM_STS = 0x25 */
pub const DA7219_PLL_SRM_STATE_SHIFT: u32 = 0;
pub const DA7219_PLL_SRM_STATE_MASK: u32 = 0xF << 0;
pub const DA7219_PLL_SRM_STATUS_SHIFT: u32 = 4;
pub const DA7219_PLL_SRM_STATUS_MASK: u32 = 0xF << 4;
pub const DA7219_PLL_SRM_STS_MCLK: u32 = 0x1 << 4;
pub const DA7219_PLL_SRM_STS_SRM_LOCK: u32 = 0x1 << 7;
/* DA7219_DIG_ROUTING_DAI = 0x2A */
pub const DA7219_DAI_L_SRC_SHIFT: u32 = 0;
pub const DA7219_DAI_L_SRC_MASK: u32 = 0x3 << 0;
pub const DA7219_DAI_R_SRC_SHIFT: u32 = 4;
pub const DA7219_DAI_R_SRC_MASK: u32 = 0x3 << 4;
pub const DA7219_OUT_SRC_MAX: u32 = 4;
/* DA7219_DAI_CLK_MODE = 0x2B */
pub const DA7219_DAI_BCLKS_PER_WCLK_SHIFT: u32 = 0;
pub const DA7219_DAI_BCLKS_PER_WCLK_MASK: u32 = 0x3 << 0;
pub const DA7219_DAI_BCLKS_PER_WCLK_32: u32 = 0x0 << 0;
pub const DA7219_DAI_BCLKS_PER_WCLK_64: u32 = 0x1 << 0;
pub const DA7219_DAI_BCLKS_PER_WCLK_128: u32 = 0x2 << 0;
pub const DA7219_DAI_BCLKS_PER_WCLK_256: u32 = 0x3 << 0;
pub const DA7219_DAI_CLK_POL_SHIFT: u32 = 2;
pub const DA7219_DAI_CLK_POL_MASK: u32 = 0x1 << 2;
pub const DA7219_DAI_CLK_POL_INV: u32 = 0x1 << 2;
pub const DA7219_DAI_WCLK_POL_SHIFT: u32 = 3;
pub const DA7219_DAI_WCLK_POL_MASK: u32 = 0x1 << 3;
pub const DA7219_DAI_WCLK_POL_INV: u32 = 0x1 << 3;
pub const DA7219_DAI_WCLK_TRI_STATE_SHIFT: u32 = 4;
pub const DA7219_DAI_WCLK_TRI_STATE_MASK: u32 = 0x1 << 4;
pub const DA7219_DAI_CLK_EN_SHIFT: u32 = 7;
pub const DA7219_DAI_CLK_EN_MASK: u32 = 0x1 << 7;
/* DA7219_DAI_CTRL = 0x2C */
pub const DA7219_DAI_FORMAT_SHIFT: u32 = 0;
pub const DA7219_DAI_FORMAT_MASK: u32 = 0x3 << 0;
pub const DA7219_DAI_FORMAT_I2S: u32 = 0x0 << 0;
pub const DA7219_DAI_FORMAT_LEFT_J: u32 = 0x1 << 0;
pub const DA7219_DAI_FORMAT_RIGHT_J: u32 = 0x2 << 0;
pub const DA7219_DAI_FORMAT_DSP: u32 = 0x3 << 0;
pub const DA7219_DAI_WORD_LENGTH_SHIFT: u32 = 2;
pub const DA7219_DAI_WORD_LENGTH_MASK: u32 = 0x3 << 2;
pub const DA7219_DAI_WORD_LENGTH_S16_LE: u32 = 0x0 << 2;
pub const DA7219_DAI_WORD_LENGTH_S20_LE: u32 = 0x1 << 2;
pub const DA7219_DAI_WORD_LENGTH_S24_LE: u32 = 0x2 << 2;
pub const DA7219_DAI_WORD_LENGTH_S32_LE: u32 = 0x3 << 2;
pub const DA7219_DAI_CH_NUM_SHIFT: u32 = 4;
pub const DA7219_DAI_CH_NUM_MASK: u32 = 0x3 << 4;
pub const DA7219_DAI_CH_NUM_MAX: u32 = 2;
pub const DA7219_DAI_EN_SHIFT: u32 = 7;
pub const DA7219_DAI_EN_MASK: u32 = 0x1 << 7;
/* DA7219_DAI_TDM_CTRL = 0x2D */
pub const DA7219_DAI_TDM_CH_EN_SHIFT: u32 = 0;
pub const DA7219_DAI_TDM_CH_EN_MASK: u32 = 0x3 << 0;
pub const DA7219_DAI_OE_SHIFT: u32 = 6;
pub const DA7219_DAI_OE_MASK: u32 = 0x1 << 6;
pub const DA7219_DAI_TDM_MODE_EN_SHIFT: u32 = 7;
pub const DA7219_DAI_TDM_MODE_EN_MASK: u32 = 0x1 << 7;
pub const DA7219_DAI_TDM_MAX_SLOTS: u32 = 2;
/* DA7219_DIG_ROUTING_DAC = 0x2E */
pub const DA7219_DAC_L_SRC_SHIFT: u32 = 0;
pub const DA7219_DAC_L_SRC_MASK: u32 = 0x3 << 0;
pub const DA7219_DAC_L_SRC_TONEGEN: u32 = 0x1 << 0;
pub const DA7219_DAC_L_MONO_SHIFT: u32 = 3;
pub const DA7219_DAC_L_MONO_MASK: u32 = 0x1 << 3;
pub const DA7219_DAC_R_SRC_SHIFT: u32 = 4;
pub const DA7219_DAC_R_SRC_MASK: u32 = 0x3 << 4;
pub const DA7219_DAC_R_SRC_TONEGEN: u32 = 0x1 << 4;
pub const DA7219_DAC_R_MONO_SHIFT: u32 = 7;
pub const DA7219_DAC_R_MONO_MASK: u32 = 0x1 << 7;
/* DA7219_ALC_CTRL1 = 0x2F */
pub const DA7219_ALC_OFFSET_EN_SHIFT: u32 = 0;
pub const DA7219_ALC_OFFSET_EN_MASK: u32 = 0x1 << 0;
pub const DA7219_ALC_SYNC_MODE_SHIFT: u32 = 1;
pub const DA7219_ALC_SYNC_MODE_MASK: u32 = 0x1 << 1;
pub const DA7219_ALC_EN_SHIFT: u32 = 3;
pub const DA7219_ALC_EN_MASK: u32 = 0x1 << 3;
pub const DA7219_ALC_AUTO_CALIB_EN_SHIFT: u32 = 4;
pub const DA7219_ALC_AUTO_CALIB_EN_MASK: u32 = 0x1 << 4;
pub const DA7219_ALC_CALIB_OVERFLOW_SHIFT: u32 = 5;
pub const DA7219_ALC_CALIB_OVERFLOW_MASK: u32 = 0x1 << 5;
/* DA7219_DAI_OFFSET_LOWER = 0x30 */
pub const DA7219_DAI_OFFSET_LOWER_SHIFT: u32 = 0;
pub const DA7219_DAI_OFFSET_LOWER_MASK: u32 = 0xFF << 0;
/* DA7219_DAI_OFFSET_UPPER = 0x31 */
pub const DA7219_DAI_OFFSET_UPPER_SHIFT: u32 = 0;
pub const DA7219_DAI_OFFSET_UPPER_MASK: u32 = 0x7 << 0;
pub const DA7219_DAI_OFFSET_MAX: u32 = 0x2FF;
/* DA7219_REFERENCES = 0x32 */
pub const DA7219_BIAS_EN_SHIFT: u32 = 3;
pub const DA7219_BIAS_EN_MASK: u32 = 0x1 << 3;
pub const DA7219_VMID_FAST_CHARGE_SHIFT: u32 = 4;
pub const DA7219_VMID_FAST_CHARGE_MASK: u32 = 0x1 << 4;
/* DA7219_MIXIN_L_SELECT = 0x33 */
pub const DA7219_MIXIN_L_MIX_SELECT_SHIFT: u32 = 0;
pub const DA7219_MIXIN_L_MIX_SELECT_MASK: u32 = 0x1 << 0;
/* DA7219_MIXIN_L_GAIN = 0x34 */
pub const DA7219_MIXIN_L_AMP_GAIN_SHIFT: u32 = 0;
pub const DA7219_MIXIN_L_AMP_GAIN_MASK: u32 = 0xF << 0;
pub const DA7219_MIXIN_L_AMP_GAIN_MAX: u32 = 0xF;
/* DA7219_ADC_L_GAIN = 0x36 */
pub const DA7219_ADC_L_DIGITAL_GAIN_SHIFT: u32 = 0;
pub const DA7219_ADC_L_DIGITAL_GAIN_MASK: u32 = 0x7F << 0;
pub const DA7219_ADC_L_DIGITAL_GAIN_MAX: u32 = 0x7F;
/* DA7219_ADC_FILTERS1 = 0x38 */
pub const DA7219_ADC_VOICE_HPF_CORNER_SHIFT: u32 = 0;
pub const DA7219_ADC_VOICE_HPF_CORNER_MASK: u32 = 0x7 << 0;
pub const DA7219_VOICE_HPF_CORNER_MAX: u32 = 8;
pub const DA7219_ADC_VOICE_EN_SHIFT: u32 = 3;
pub const DA7219_ADC_VOICE_EN_MASK: u32 = 0x1 << 3;
pub const DA7219_ADC_AUDIO_HPF_CORNER_SHIFT: u32 = 4;
pub const DA7219_ADC_AUDIO_HPF_CORNER_MASK: u32 = 0x3 << 4;
pub const DA7219_AUDIO_HPF_CORNER_MAX: u32 = 4;
pub const DA7219_ADC_HPF_EN_SHIFT: u32 = 7;
pub const DA7219_ADC_HPF_EN_MASK: u32 = 0x1 << 7;
pub const DA7219_HPF_MODE_SHIFT: u32 = 0;
pub const DA7219_HPF_DISABLED: u32 = 0x0 << 3 | 0x0 << 7;
pub const DA7219_HPF_AUDIO_EN: u32 = 0x0 << 3 | 0x1 << 7;
pub const DA7219_HPF_VOICE_EN: u32 = 0x1 << 3 | 0x1 << 7;
pub const DA7219_HPF_MODE_MASK: u32 = 0x1 << 3 | 0x1 << 7;
pub const DA7219_HPF_MODE_MAX: u32 = 3;
/* DA7219_MIC_1_GAIN = 0x39 */
pub const DA7219_MIC_1_AMP_GAIN_SHIFT: u32 = 0;
pub const DA7219_MIC_1_AMP_GAIN_MASK: u32 = 0x7 << 0;
/* DA7219_SIDETONE_CTRL = 0x3A */
pub const DA7219_SIDETONE_MUTE_EN_SHIFT: u32 = 6;
pub const DA7219_SIDETONE_MUTE_EN_MASK: u32 = 0x1 << 6;
pub const DA7219_SIDETONE_EN_SHIFT: u32 = 7;
pub const DA7219_SIDETONE_EN_MASK: u32 = 0x1 << 7;
/* DA7219_SIDETONE_GAIN = 0x3B */
pub const DA7219_SIDETONE_GAIN_SHIFT: u32 = 0;
pub const DA7219_SIDETONE_GAIN_MASK: u32 = 0xF << 0;
pub const DA7219_SIDETONE_GAIN_MAX: u32 = 0xE;
/* DA7219_DROUTING_ST_OUTFILT_1L = 0x3C */
pub const DA7219_OUTFILT_ST_1L_SRC_SHIFT: u32 = 0;
pub const DA7219_OUTFILT_ST_1L_SRC_MASK: u32 = 0x7 << 0;
pub const DA7219_DMIX_ST_SRC_OUTFILT1L_SHIFT: u32 = 0;
pub const DA7219_DMIX_ST_SRC_OUTFILT1R_SHIFT: u32 = 1;
pub const DA7219_DMIX_ST_SRC_SIDETONE_SHIFT: u32 = 2;
pub const DA7219_DMIX_ST_SRC_OUTFILT1L: u32 = 0x1 << 0;
pub const DA7219_DMIX_ST_SRC_OUTFILT1R: u32 = 0x1 << 1;
/* DA7219_DROUTING_ST_OUTFILT_1R = 0x3D */
pub const DA7219_OUTFILT_ST_1R_SRC_SHIFT: u32 = 0;
pub const DA7219_OUTFILT_ST_1R_SRC_MASK: u32 = 0x7 << 0;
/* DA7219_DAC_FILTERS5 = 0x40 */
pub const DA7219_DAC_SOFTMUTE_RATE_SHIFT: u32 = 4;
pub const DA7219_DAC_SOFTMUTE_RATE_MASK: u32 = 0x7 << 4;
pub const DA7219_DAC_SOFTMUTE_RATE_MAX: u32 = 7;
pub const DA7219_DAC_SOFTMUTE_EN_SHIFT: u32 = 7;
pub const DA7219_DAC_SOFTMUTE_EN_MASK: u32 = 0x1 << 7;
/* DA7219_DAC_FILTERS2 = 0x41 */
pub const DA7219_DAC_EQ_BAND1_SHIFT: u32 = 0;
pub const DA7219_DAC_EQ_BAND1_MASK: u32 = 0xF << 0;
pub const DA7219_DAC_EQ_BAND2_SHIFT: u32 = 4;
pub const DA7219_DAC_EQ_BAND2_MASK: u32 = 0xF << 4;
pub const DA7219_DAC_EQ_BAND_MAX: u32 = 0xF;
/* DA7219_DAC_FILTERS3 = 0x42 */
pub const DA7219_DAC_EQ_BAND3_SHIFT: u32 = 0;
pub const DA7219_DAC_EQ_BAND3_MASK: u32 = 0xF << 0;
pub const DA7219_DAC_EQ_BAND4_SHIFT: u32 = 4;
pub const DA7219_DAC_EQ_BAND4_MASK: u32 = 0xF << 4;
/* DA7219_DAC_FILTERS4 = 0x43 */
pub const DA7219_DAC_EQ_BAND5_SHIFT: u32 = 0;
pub const DA7219_DAC_EQ_BAND5_MASK: u32 = 0xF << 0;
pub const DA7219_DAC_EQ_EN_SHIFT: u32 = 7;
pub const DA7219_DAC_EQ_EN_MASK: u32 = 0x1 << 7;
/* DA7219_DAC_FILTERS1 = 0x44 */
pub const DA7219_DAC_VOICE_HPF_CORNER_SHIFT: u32 = 0;
pub const DA7219_DAC_VOICE_HPF_CORNER_MASK: u32 = 0x7 << 0;
pub const DA7219_DAC_VOICE_EN_SHIFT: u32 = 3;
pub const DA7219_DAC_VOICE_EN_MASK: u32 = 0x1 << 3;
pub const DA7219_DAC_AUDIO_HPF_CORNER_SHIFT: u32 = 4;
pub const DA7219_DAC_AUDIO_HPF_CORNER_MASK: u32 = 0x3 << 4;
pub const DA7219_DAC_HPF_EN_SHIFT: u32 = 7;
pub const DA7219_DAC_HPF_EN_MASK: u32 = 0x1 << 7;
/* DA7219_DAC_L_GAIN = 0x45 */
pub const DA7219_DAC_L_DIGITAL_GAIN_SHIFT: u32 = 0;
pub const DA7219_DAC_L_DIGITAL_GAIN_MASK: u32 = 0x7F << 0;
pub const DA7219_DAC_DIGITAL_GAIN_MAX: u32 = 0x7F;
pub const DA7219_DAC_DIGITAL_GAIN_0DB: u32 = 0x6F << 0;
/* DA7219_DAC_R_GAIN = 0x46 */
pub const DA7219_DAC_R_DIGITAL_GAIN_SHIFT: u32 = 0;
pub const DA7219_DAC_R_DIGITAL_GAIN_MASK: u32 = 0x7F << 0;
/* DA7219_CP_CTRL = 0x47 */
pub const DA7219_CP_MCHANGE_SHIFT: u32 = 4;
pub const DA7219_CP_MCHANGE_MASK: u32 = 0x3 << 4;
pub const DA7219_CP_MCHANGE_REL_MASK: u32 = 0x3;
pub const DA7219_CP_MCHANGE_MAX: u32 = 3;
pub const DA7219_CP_MCHANGE_LARGEST_VOL: u32 = 0x1;
pub const DA7219_CP_MCHANGE_DAC_VOL: u32 = 0x2;
pub const DA7219_CP_MCHANGE_SIG_MAG: u32 = 0x3;
pub const DA7219_CP_EN_SHIFT: u32 = 7;
pub const DA7219_CP_EN_MASK: u32 = 0x1 << 7;
/* DA7219_HP_L_GAIN = 0x48 */
pub const DA7219_HP_L_AMP_GAIN_SHIFT: u32 = 0;
pub const DA7219_HP_L_AMP_GAIN_MASK: u32 = 0x3F << 0;
pub const DA7219_HP_AMP_GAIN_MAX: u32 = 0x3F;
pub const DA7219_HP_AMP_GAIN_0DB: u32 = 0x39 << 0;
/* DA7219_HP_R_GAIN = 0x49 */
pub const DA7219_HP_R_AMP_GAIN_SHIFT: u32 = 0;
pub const DA7219_HP_R_AMP_GAIN_MASK: u32 = 0x3F << 0;
/* DA7219_MIXOUT_L_SELECT = 0x4B */
pub const DA7219_MIXOUT_L_MIX_SELECT_SHIFT: u32 = 0;
pub const DA7219_MIXOUT_L_MIX_SELECT_MASK: u32 = 0x1 << 0;
/* DA7219_MIXOUT_R_SELECT = 0x4C */
pub const DA7219_MIXOUT_R_MIX_SELECT_SHIFT: u32 = 0;
pub const DA7219_MIXOUT_R_MIX_SELECT_MASK: u32 = 0x1 << 0;
/* DA7219_SYSTEM_MODES_INPUT = 0x50 */
pub const DA7219_MODE_SUBMIT_SHIFT: u32 = 0;
pub const DA7219_MODE_SUBMIT_MASK: u32 = 0x1 << 0;
pub const DA7219_ADC_MODE_SHIFT: u32 = 1;
pub const DA7219_ADC_MODE_MASK: u32 = 0x7F << 1;
/* DA7219_SYSTEM_MODES_OUTPUT = 0x51 */
/* Duplicate C macro definition preserved: #define DA7219_MODE_SUBMIT_SHIFT 0 */
/* Duplicate C macro definition preserved: #define DA7219_MODE_SUBMIT_MASK 0x1 << 0 */
pub const DA7219_DAC_MODE_SHIFT: u32 = 1;
pub const DA7219_DAC_MODE_MASK: u32 = 0x7F << 1;
/* DA7219_MICBIAS_CTRL = 0x62 */
pub const DA7219_MICBIAS1_LEVEL_SHIFT: u32 = 0;
pub const DA7219_MICBIAS1_LEVEL_MASK: u32 = 0x7 << 0;
pub const DA7219_MICBIAS1_EN_SHIFT: u32 = 3;
pub const DA7219_MICBIAS1_EN_MASK: u32 = 0x1 << 3;
/* DA7219_MIC_1_CTRL = 0x63 */
pub const DA7219_MIC_1_AMP_RAMP_EN_SHIFT: u32 = 5;
pub const DA7219_MIC_1_AMP_RAMP_EN_MASK: u32 = 0x1 << 5;
pub const DA7219_MIC_1_AMP_MUTE_EN_SHIFT: u32 = 6;
pub const DA7219_MIC_1_AMP_MUTE_EN_MASK: u32 = 0x1 << 6;
pub const DA7219_MIC_1_AMP_EN_SHIFT: u32 = 7;
pub const DA7219_MIC_1_AMP_EN_MASK: u32 = 0x1 << 7;
/* DA7219_MIXIN_L_CTRL = 0x65 */
pub const DA7219_MIXIN_L_MIX_EN_SHIFT: u32 = 3;
pub const DA7219_MIXIN_L_MIX_EN_MASK: u32 = 0x1 << 3;
pub const DA7219_MIXIN_L_AMP_ZC_EN_SHIFT: u32 = 4;
pub const DA7219_MIXIN_L_AMP_ZC_EN_MASK: u32 = 0x1 << 4;
pub const DA7219_MIXIN_L_AMP_RAMP_EN_SHIFT: u32 = 5;
pub const DA7219_MIXIN_L_AMP_RAMP_EN_MASK: u32 = 0x1 << 5;
pub const DA7219_MIXIN_L_AMP_MUTE_EN_SHIFT: u32 = 6;
pub const DA7219_MIXIN_L_AMP_MUTE_EN_MASK: u32 = 0x1 << 6;
pub const DA7219_MIXIN_L_AMP_EN_SHIFT: u32 = 7;
pub const DA7219_MIXIN_L_AMP_EN_MASK: u32 = 0x1 << 7;
/* DA7219_ADC_L_CTRL = 0x67 */
pub const DA7219_ADC_L_BIAS_SHIFT: u32 = 0;
pub const DA7219_ADC_L_BIAS_MASK: u32 = 0x3 << 0;
pub const DA7219_ADC_L_RAMP_EN_SHIFT: u32 = 5;
pub const DA7219_ADC_L_RAMP_EN_MASK: u32 = 0x1 << 5;
pub const DA7219_ADC_L_MUTE_EN_SHIFT: u32 = 6;
pub const DA7219_ADC_L_MUTE_EN_MASK: u32 = 0x1 << 6;
pub const DA7219_ADC_L_EN_SHIFT: u32 = 7;
pub const DA7219_ADC_L_EN_MASK: u32 = 0x1 << 7;
/* DA7219_DAC_L_CTRL = 0x69 */
pub const DA7219_DAC_L_RAMP_EN_SHIFT: u32 = 5;
pub const DA7219_DAC_L_RAMP_EN_MASK: u32 = 0x1 << 5;
pub const DA7219_DAC_L_MUTE_EN_SHIFT: u32 = 6;
pub const DA7219_DAC_L_MUTE_EN_MASK: u32 = 0x1 << 6;
pub const DA7219_DAC_L_EN_SHIFT: u32 = 7;
pub const DA7219_DAC_L_EN_MASK: u32 = 0x1 << 7;
/* DA7219_DAC_R_CTRL = 0x6A */
pub const DA7219_DAC_R_RAMP_EN_SHIFT: u32 = 5;
pub const DA7219_DAC_R_RAMP_EN_MASK: u32 = 0x1 << 5;
pub const DA7219_DAC_R_MUTE_EN_SHIFT: u32 = 6;
pub const DA7219_DAC_R_MUTE_EN_MASK: u32 = 0x1 << 6;
pub const DA7219_DAC_R_EN_SHIFT: u32 = 7;
pub const DA7219_DAC_R_EN_MASK: u32 = 0x1 << 7;
/* DA7219_HP_L_CTRL = 0x6B */
pub const DA7219_HP_L_AMP_MIN_GAIN_EN_SHIFT: u32 = 2;
pub const DA7219_HP_L_AMP_MIN_GAIN_EN_MASK: u32 = 0x1 << 2;
pub const DA7219_HP_L_AMP_OE_SHIFT: u32 = 3;
pub const DA7219_HP_L_AMP_OE_MASK: u32 = 0x1 << 3;
pub const DA7219_HP_L_AMP_ZC_EN_SHIFT: u32 = 4;
pub const DA7219_HP_L_AMP_ZC_EN_MASK: u32 = 0x1 << 4;
pub const DA7219_HP_L_AMP_RAMP_EN_SHIFT: u32 = 5;
pub const DA7219_HP_L_AMP_RAMP_EN_MASK: u32 = 0x1 << 5;
pub const DA7219_HP_L_AMP_MUTE_EN_SHIFT: u32 = 6;
pub const DA7219_HP_L_AMP_MUTE_EN_MASK: u32 = 0x1 << 6;
pub const DA7219_HP_L_AMP_EN_SHIFT: u32 = 7;
pub const DA7219_HP_L_AMP_EN_MASK: u32 = 0x1 << 7;
/* DA7219_HP_R_CTRL = 0x6C */
pub const DA7219_HP_R_AMP_MIN_GAIN_EN_SHIFT: u32 = 2;
pub const DA7219_HP_R_AMP_MIN_GAIN_EN_MASK: u32 = 0x1 << 2;
pub const DA7219_HP_R_AMP_OE_SHIFT: u32 = 3;
pub const DA7219_HP_R_AMP_OE_MASK: u32 = 0x1 << 3;
pub const DA7219_HP_R_AMP_ZC_EN_SHIFT: u32 = 4;
pub const DA7219_HP_R_AMP_ZC_EN_MASK: u32 = 0x1 << 4;
pub const DA7219_HP_R_AMP_RAMP_EN_SHIFT: u32 = 5;
pub const DA7219_HP_R_AMP_RAMP_EN_MASK: u32 = 0x1 << 5;
pub const DA7219_HP_R_AMP_MUTE_EN_SHIFT: u32 = 6;
pub const DA7219_HP_R_AMP_MUTE_EN_MASK: u32 = 0x1 << 6;
pub const DA7219_HP_R_AMP_EN_SHIFT: u32 = 7;
pub const DA7219_HP_R_AMP_EN_MASK: u32 = 0x1 << 7;
/* DA7219_MIXOUT_L_CTRL = 0x6E */
pub const DA7219_MIXOUT_L_AMP_EN_SHIFT: u32 = 7;
pub const DA7219_MIXOUT_L_AMP_EN_MASK: u32 = 0x1 << 7;
/* DA7219_MIXOUT_R_CTRL = 0x6F */
pub const DA7219_MIXOUT_R_AMP_EN_SHIFT: u32 = 7;
pub const DA7219_MIXOUT_R_AMP_EN_MASK: u32 = 0x1 << 7;
/* DA7219_CHIP_ID1 = 0x81 */
pub const DA7219_CHIP_ID1_SHIFT: u32 = 0;
pub const DA7219_CHIP_ID1_MASK: u32 = 0xFF << 0;
/* DA7219_CHIP_ID2 = 0x82 */
pub const DA7219_CHIP_ID2_SHIFT: u32 = 0;
pub const DA7219_CHIP_ID2_MASK: u32 = 0xFF << 0;
/* DA7219_CHIP_REVISION = 0x83 */
pub const DA7219_CHIP_MINOR_SHIFT: u32 = 0;
pub const DA7219_CHIP_MINOR_MASK: u32 = 0xF << 0;
pub const DA7219_CHIP_MAJOR_SHIFT: u32 = 4;
pub const DA7219_CHIP_MAJOR_MASK: u32 = 0xF << 4;
/* DA7219_IO_CTRL = 0x91 */
pub const DA7219_IO_VOLTAGE_LEVEL_SHIFT: u32 = 0;
pub const DA7219_IO_VOLTAGE_LEVEL_MASK: u32 = 0x1 << 0;
pub const DA7219_IO_VOLTAGE_LEVEL_2_5V_3_6V: u32 = 0;
pub const DA7219_IO_VOLTAGE_LEVEL_1_2V_2_8V: u32 = 1;
/* DA7219_GAIN_RAMP_CTRL = 0x92 */
pub const DA7219_GAIN_RAMP_RATE_SHIFT: u32 = 0;
pub const DA7219_GAIN_RAMP_RATE_MASK: u32 = 0x3 << 0;
pub const DA7219_GAIN_RAMP_RATE_X8: u32 = 0x0 << 0;
pub const DA7219_GAIN_RAMP_RATE_NOMINAL: u32 = 0x1 << 0;
pub const DA7219_GAIN_RAMP_RATE_MAX: u32 = 4;
/* DA7219_PC_COUNT = 0x94 */
pub const DA7219_PC_FREERUN_SHIFT: u32 = 0;
pub const DA7219_PC_FREERUN_MASK: u32 = 0x1 << 0;
pub const DA7219_PC_RESYNC_AUTO_SHIFT: u32 = 1;
pub const DA7219_PC_RESYNC_AUTO_MASK: u32 = 0x1 << 1;
/* DA7219_CP_VOL_THRESHOLD1 = 0x95 */
pub const DA7219_CP_THRESH_VDD2_SHIFT: u32 = 0;
pub const DA7219_CP_THRESH_VDD2_MASK: u32 = 0x3F << 0;
pub const DA7219_CP_THRESH_VDD2_MAX: u32 = 0x3F;
/* DA7219_DIG_CTRL = 0x99 */
pub const DA7219_DAC_L_INV_SHIFT: u32 = 3;
pub const DA7219_DAC_L_INV_MASK: u32 = 0x1 << 3;
pub const DA7219_DAC_R_INV_SHIFT: u32 = 7;
pub const DA7219_DAC_R_INV_MASK: u32 = 0x1 << 7;
/* DA7219_ALC_CTRL2 = 0x9A */
pub const DA7219_ALC_ATTACK_SHIFT: u32 = 0;
pub const DA7219_ALC_ATTACK_MASK: u32 = 0xF << 0;
pub const DA7219_ALC_ATTACK_MAX: u32 = 13;
pub const DA7219_ALC_RELEASE_SHIFT: u32 = 4;
pub const DA7219_ALC_RELEASE_MASK: u32 = 0xF << 4;
pub const DA7219_ALC_RELEASE_MAX: u32 = 11;
/* DA7219_ALC_CTRL3 = 0x9B */
pub const DA7219_ALC_HOLD_SHIFT: u32 = 0;
pub const DA7219_ALC_HOLD_MASK: u32 = 0xF << 0;
pub const DA7219_ALC_HOLD_MAX: u32 = 16;
pub const DA7219_ALC_INTEG_ATTACK_SHIFT: u32 = 4;
pub const DA7219_ALC_INTEG_ATTACK_MASK: u32 = 0x3 << 4;
pub const DA7219_ALC_INTEG_RELEASE_SHIFT: u32 = 6;
pub const DA7219_ALC_INTEG_RELEASE_MASK: u32 = 0x3 << 6;
pub const DA7219_ALC_INTEG_MAX: u32 = 4;
/* DA7219_ALC_NOISE = 0x9C */
pub const DA7219_ALC_NOISE_SHIFT: u32 = 0;
pub const DA7219_ALC_NOISE_MASK: u32 = 0x3F << 0;
pub const DA7219_ALC_THRESHOLD_MAX: u32 = 0x3F;
/* DA7219_ALC_TARGET_MIN = 0x9D */
pub const DA7219_ALC_THRESHOLD_MIN_SHIFT: u32 = 0;
pub const DA7219_ALC_THRESHOLD_MIN_MASK: u32 = 0x3F << 0;
/* DA7219_ALC_TARGET_MAX = 0x9E */
pub const DA7219_ALC_THRESHOLD_MAX_SHIFT: u32 = 0;
pub const DA7219_ALC_THRESHOLD_MAX_MASK: u32 = 0x3F << 0;
/* DA7219_ALC_GAIN_LIMITS = 0x9F */
pub const DA7219_ALC_ATTEN_MAX_SHIFT: u32 = 0;
pub const DA7219_ALC_ATTEN_MAX_MASK: u32 = 0xF << 0;
pub const DA7219_ALC_GAIN_MAX_SHIFT: u32 = 4;
pub const DA7219_ALC_GAIN_MAX_MASK: u32 = 0xF << 4;
pub const DA7219_ALC_ATTEN_GAIN_MAX: u32 = 0xF;
/* DA7219_ALC_ANA_GAIN_LIMITS = 0xA0 */
pub const DA7219_ALC_ANA_GAIN_MIN_SHIFT: u32 = 0;
pub const DA7219_ALC_ANA_GAIN_MIN_MASK: u32 = 0x7 << 0;
pub const DA7219_ALC_ANA_GAIN_MIN: u32 = 0x1;
pub const DA7219_ALC_ANA_GAIN_MAX_SHIFT: u32 = 4;
pub const DA7219_ALC_ANA_GAIN_MAX_MASK: u32 = 0x7 << 4;
pub const DA7219_ALC_ANA_GAIN_MAX: u32 = 0x7;
/* DA7219_ALC_ANTICLIP_CTRL = 0xA1 */
pub const DA7219_ALC_ANTICLIP_STEP_SHIFT: u32 = 0;
pub const DA7219_ALC_ANTICLIP_STEP_MASK: u32 = 0x3 << 0;
pub const DA7219_ALC_ANTICLIP_STEP_MAX: u32 = 4;
pub const DA7219_ALC_ANTIPCLIP_EN_SHIFT: u32 = 7;
pub const DA7219_ALC_ANTIPCLIP_EN_MASK: u32 = 0x1 << 7;
/* DA7219_ALC_ANTICLIP_LEVEL = 0xA2 */
pub const DA7219_ALC_ANTICLIP_LEVEL_SHIFT: u32 = 0;
pub const DA7219_ALC_ANTICLIP_LEVEL_MASK: u32 = 0x7F << 0;
/* DA7219_ALC_OFFSET_AUTO_M_L = 0xA3 */
pub const DA7219_ALC_OFFSET_AUTO_M_L_SHIFT: u32 = 0;
pub const DA7219_ALC_OFFSET_AUTO_M_L_MASK: u32 = 0xFF << 0;
/* DA7219_ALC_OFFSET_AUTO_U_L = 0xA4 */
pub const DA7219_ALC_OFFSET_AUTO_U_L_SHIFT: u32 = 0;
pub const DA7219_ALC_OFFSET_AUTO_U_L_MASK: u32 = 0xF << 0;
/* DA7219_DAC_NG_SETUP_TIME = 0xAF */
pub const DA7219_DAC_NG_SETUP_TIME_SHIFT: u32 = 0;
pub const DA7219_DAC_NG_SETUP_TIME_MASK: u32 = 0x3 << 0;
pub const DA7219_DAC_NG_SETUP_TIME_MAX: u32 = 4;
pub const DA7219_DAC_NG_RAMPUP_RATE_SHIFT: u32 = 2;
pub const DA7219_DAC_NG_RAMPUP_RATE_MASK: u32 = 0x1 << 2;
pub const DA7219_DAC_NG_RAMPDN_RATE_SHIFT: u32 = 3;
pub const DA7219_DAC_NG_RAMPDN_RATE_MASK: u32 = 0x1 << 3;
pub const DA7219_DAC_NG_RAMP_RATE_MAX: u32 = 2;
/* DA7219_DAC_NG_OFF_THRESH = 0xB0 */
pub const DA7219_DAC_NG_OFF_THRESHOLD_SHIFT: u32 = 0;
pub const DA7219_DAC_NG_OFF_THRESHOLD_MASK: u32 = 0x7 << 0;
pub const DA7219_DAC_NG_THRESHOLD_MAX: u32 = 0x7;
/* DA7219_DAC_NG_ON_THRESH = 0xB1 */
pub const DA7219_DAC_NG_ON_THRESHOLD_SHIFT: u32 = 0;
pub const DA7219_DAC_NG_ON_THRESHOLD_MASK: u32 = 0x7 << 0;
/* DA7219_DAC_NG_CTRL = 0xB2 */
pub const DA7219_DAC_NG_EN_SHIFT: u32 = 7;
pub const DA7219_DAC_NG_EN_MASK: u32 = 0x1 << 7;
/* DA7219_TONE_GEN_CFG1 = 0xB4 */
pub const DA7219_DTMF_REG_SHIFT: u32 = 0;
pub const DA7219_DTMF_REG_MASK: u32 = 0xF << 0;
pub const DA7219_DTMF_REG_MAX: u32 = 16;
pub const DA7219_DTMF_EN_SHIFT: u32 = 4;
pub const DA7219_DTMF_EN_MASK: u32 = 0x1 << 4;
pub const DA7219_START_STOPN_SHIFT: u32 = 7;
pub const DA7219_START_STOPN_MASK: u32 = 0x1 << 7;
/* DA7219_TONE_GEN_CFG2 = 0xB5 */
pub const DA7219_SWG_SEL_SHIFT: u32 = 0;
pub const DA7219_SWG_SEL_MASK: u32 = 0x3 << 0;
pub const DA7219_SWG_SEL_MAX: u32 = 4;
pub const DA7219_SWG_SEL_SRAMP: u32 = 0x3 << 0;
pub const DA7219_TONE_GEN_GAIN_SHIFT: u32 = 4;
pub const DA7219_TONE_GEN_GAIN_MASK: u32 = 0xF << 4;
pub const DA7219_TONE_GEN_GAIN_MAX: u32 = 0xF;
pub const DA7219_TONE_GEN_GAIN_MINUS_9DB: u32 = 0x3 << 4;
pub const DA7219_TONE_GEN_GAIN_MINUS_15DB: u32 = 0x5 << 4;
/* DA7219_TONE_GEN_CYCLES = 0xB6 */
pub const DA7219_BEEP_CYCLES_SHIFT: u32 = 0;
pub const DA7219_BEEP_CYCLES_MASK: u32 = 0x7 << 0;
/* DA7219_TONE_GEN_FREQ1_L = 0xB7 */
pub const DA7219_FREQ1_L_SHIFT: u32 = 0;
pub const DA7219_FREQ1_L_MASK: u32 = 0xFF << 0;
pub const DA7219_FREQ_MAX: u32 = 0xFFFF;
/* DA7219_TONE_GEN_FREQ1_U = 0xB8 */
pub const DA7219_FREQ1_U_SHIFT: u32 = 0;
pub const DA7219_FREQ1_U_MASK: u32 = 0xFF << 0;
/* DA7219_TONE_GEN_FREQ2_L = 0xB9 */
pub const DA7219_FREQ2_L_SHIFT: u32 = 0;
pub const DA7219_FREQ2_L_MASK: u32 = 0xFF << 0;
/* DA7219_TONE_GEN_FREQ2_U = 0xBA */
pub const DA7219_FREQ2_U_SHIFT: u32 = 0;
pub const DA7219_FREQ2_U_MASK: u32 = 0xFF << 0;
/* DA7219_TONE_GEN_ON_PER = 0xBB */
pub const DA7219_BEEP_ON_PER_SHIFT: u32 = 0;
pub const DA7219_BEEP_ON_PER_MASK: u32 = 0x3F << 0;
pub const DA7219_BEEP_ON_OFF_MAX: u32 = 0x3F;
/* DA7219_TONE_GEN_OFF_PER = 0xBC */
pub const DA7219_BEEP_OFF_PER_SHIFT: u32 = 0;
pub const DA7219_BEEP_OFF_PER_MASK: u32 = 0x3F << 0;
/* DA7219_SYSTEM_STATUS = 0xE0 */
pub const DA7219_SC1_BUSY_SHIFT: u32 = 0;
pub const DA7219_SC1_BUSY_MASK: u32 = 0x1 << 0;
pub const DA7219_SC2_BUSY_SHIFT: u32 = 1;
pub const DA7219_SC2_BUSY_MASK: u32 = 0x1 << 1;
/* DA7219_SYSTEM_ACTIVE = 0xFD */
pub const DA7219_SYSTEM_ACTIVE_SHIFT: u32 = 0;
pub const DA7219_SYSTEM_ACTIVE_MASK: u32 = 0x1 << 0;
/*
 * General defines & data
 */
/* Register inversion */
pub const DA7219_NO_INVERT: u32 = 0;
pub const DA7219_INVERT: u32 = 1;
/* Byte related defines */
pub const DA7219_BYTE_SHIFT: u32 = 8;
pub const DA7219_BYTE_MASK: u32 = 0xFF;
/* PLL Output Frequencies */
pub const DA7219_PLL_FREQ_OUT_90316: u32 = 90316800;
pub const DA7219_PLL_FREQ_OUT_98304: u32 = 98304000;
/* PLL Frequency Dividers */
pub const DA7219_PLL_INDIV_2_TO_4_5_MHZ_VAL: u32 = 1;
pub const DA7219_PLL_INDIV_4_5_TO_9_MHZ_VAL: u32 = 2;
pub const DA7219_PLL_INDIV_9_TO_18_MHZ_VAL: u32 = 4;
pub const DA7219_PLL_INDIV_18_TO_36_MHZ_VAL: u32 = 8;
pub const DA7219_PLL_INDIV_36_TO_54_MHZ_VAL: u32 = 16;
/* SRM */
pub const DA7219_SRM_CHECK_RETRIES: u32 = 8;
/* System Controller */
pub const DA7219_SYS_STAT_CHECK_RETRIES: u32 = 6;
pub const DA7219_SYS_STAT_CHECK_DELAY: u32 = 50;
/* Power up/down Delays */
pub const DA7219_SETTLING_DELAY: u32 = 40;
pub const DA7219_MIN_GAIN_DELAY: u32 = 30;
pub const DA7219_MIC_PGA_BASE_DELAY: u32 = 100;
pub const DA7219_MIC_PGA_OFFSET_DELAY: u32 = 40;
/* Regulators */
/* Private data */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum da7219_clk_src {
    DA7219_CLKSRC_MCLK = 0,
    DA7219_CLKSRC_MCLK_SQR = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum da7219_sys_clk {
    DA7219_SYSCLK_MCLK = 0,
    DA7219_SYSCLK_PLL = 1,
    DA7219_SYSCLK_PLL_SRM = 2,
}

/* Regulators */
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum da7219_supplies {
    DA7219_SUPPLY_VDD = 0,
    DA7219_SUPPLY_VDDMIC = 1,
    DA7219_SUPPLY_VDDIO = 2,
    DA7219_NUM_SUPPLIES = 3,
}

/* Private data */
#[repr(C)]
pub struct da7219_priv {
    pub component: *mut snd_soc_component,
    pub aad: *mut da7219_aad_priv,
    pub pdata: *mut da7219_pdata,

    pub wakeup_source: bool,
    pub supplies: [regulator_bulk_data; DA7219_NUM_SUPPLIES as usize],
    pub regmap: *mut regmap,
    pub ctrl_lock: mutex,
    pub pll_lock: mutex,

    /* CONFIG_COMMON_CLK: present in C only when that build-time option is enabled. */
    pub dai_clks_hw: [clk_hw; DA7219_DAI_NUM_CLKS as usize],
    pub clk_hw_data: *mut clk_hw_onecell_data,

    pub dai_clks_lookup: [*mut clk_lookup; DA7219_DAI_NUM_CLKS as usize],
    pub dai_clks: [*mut clk; DA7219_DAI_NUM_CLKS as usize],

    pub mclk: *mut clk,
    pub mclk_rate: c_uint,
    pub clk_src: c_int,

    pub master: bool,
    pub tdm_en: bool,
    pub alc_en: bool,
    pub micbias_on_event: bool,
    pub mic_pga_delay: c_uint,
    pub gain_ramp_ctrl: u8,
}

unsafe extern "C" {
    pub fn da7219_set_pll(
        component: *mut snd_soc_component,
        source: c_int,
        fout: c_uint,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
