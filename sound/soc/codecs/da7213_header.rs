/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * da7213.h - DA7213 ASoC Codec Driver
 *
 * Copyright (c) 2013 Dialog Semiconductor
 *
 * Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
 * Author: David Rau <David.Rau.opensource@dm.renesas.com>
 */

/* C header dependencies: linux/clk.h, linux/regmap.h,
 * linux/regulator/consumer.h, sound/da7213.h
 */

/*
 * Registers
 */

/* Status Registers */
pub const DA7213_STATUS1: u32 = 0x02;
pub const DA7213_PLL_STATUS: u32 = 0x03;
pub const DA7213_AUX_L_GAIN_STATUS: u32 = 0x04;
pub const DA7213_AUX_R_GAIN_STATUS: u32 = 0x05;
pub const DA7213_MIC_1_GAIN_STATUS: u32 = 0x06;
pub const DA7213_MIC_2_GAIN_STATUS: u32 = 0x07;
pub const DA7213_MIXIN_L_GAIN_STATUS: u32 = 0x08;
pub const DA7213_MIXIN_R_GAIN_STATUS: u32 = 0x09;
pub const DA7213_ADC_L_GAIN_STATUS: u32 = 0x0A;
pub const DA7213_ADC_R_GAIN_STATUS: u32 = 0x0B;
pub const DA7213_DAC_L_GAIN_STATUS: u32 = 0x0C;
pub const DA7213_DAC_R_GAIN_STATUS: u32 = 0x0D;
pub const DA7213_HP_L_GAIN_STATUS: u32 = 0x0E;
pub const DA7213_HP_R_GAIN_STATUS: u32 = 0x0F;
pub const DA7213_LINE_GAIN_STATUS: u32 = 0x10;

/* System Initialisation Registers */
pub const DA7213_DIG_ROUTING_DAI: u32 = 0x21;
pub const DA7213_SR: u32 = 0x22;
pub const DA7213_REFERENCES: u32 = 0x23;
pub const DA7213_PLL_FRAC_TOP: u32 = 0x24;
pub const DA7213_PLL_FRAC_BOT: u32 = 0x25;
pub const DA7213_PLL_INTEGER: u32 = 0x26;
pub const DA7213_PLL_CTRL: u32 = 0x27;
pub const DA7213_DAI_CLK_MODE: u32 = 0x28;
pub const DA7213_DAI_CTRL: u32 = 0x29;
pub const DA7213_DIG_ROUTING_DAC: u32 = 0x2A;
pub const DA7213_ALC_CTRL1: u32 = 0x2B;

/* Input - Gain, Select and Filter Registers */
pub const DA7213_AUX_L_GAIN: u32 = 0x30;
pub const DA7213_AUX_R_GAIN: u32 = 0x31;
pub const DA7213_MIXIN_L_SELECT: u32 = 0x32;
pub const DA7213_MIXIN_R_SELECT: u32 = 0x33;
pub const DA7213_MIXIN_L_GAIN: u32 = 0x34;
pub const DA7213_MIXIN_R_GAIN: u32 = 0x35;
pub const DA7213_ADC_L_GAIN: u32 = 0x36;
pub const DA7213_ADC_R_GAIN: u32 = 0x37;
pub const DA7213_ADC_FILTERS1: u32 = 0x38;
pub const DA7213_MIC_1_GAIN: u32 = 0x39;
pub const DA7213_MIC_2_GAIN: u32 = 0x3A;

/* Output - Gain, Select and Filter Registers */
pub const DA7213_DAC_FILTERS5: u32 = 0x40;
pub const DA7213_DAC_FILTERS2: u32 = 0x41;
pub const DA7213_DAC_FILTERS3: u32 = 0x42;
pub const DA7213_DAC_FILTERS4: u32 = 0x43;
pub const DA7213_DAC_FILTERS1: u32 = 0x44;
pub const DA7213_DAC_L_GAIN: u32 = 0x45;
pub const DA7213_DAC_R_GAIN: u32 = 0x46;
pub const DA7213_CP_CTRL: u32 = 0x47;
pub const DA7213_HP_L_GAIN: u32 = 0x48;
pub const DA7213_HP_R_GAIN: u32 = 0x49;
pub const DA7213_LINE_GAIN: u32 = 0x4A;
pub const DA7213_MIXOUT_L_SELECT: u32 = 0x4B;
pub const DA7213_MIXOUT_R_SELECT: u32 = 0x4C;

/* System Controller Registers */
pub const DA7213_SYSTEM_MODES_INPUT: u32 = 0x50;
pub const DA7213_SYSTEM_MODES_OUTPUT: u32 = 0x51;

/* Control Registers */
pub const DA7213_AUX_L_CTRL: u32 = 0x60;
pub const DA7213_AUX_R_CTRL: u32 = 0x61;
pub const DA7213_MICBIAS_CTRL: u32 = 0x62;
pub const DA7213_MIC_1_CTRL: u32 = 0x63;
pub const DA7213_MIC_2_CTRL: u32 = 0x64;
pub const DA7213_MIXIN_L_CTRL: u32 = 0x65;
pub const DA7213_MIXIN_R_CTRL: u32 = 0x66;
pub const DA7213_ADC_L_CTRL: u32 = 0x67;
pub const DA7213_ADC_R_CTRL: u32 = 0x68;
pub const DA7213_DAC_L_CTRL: u32 = 0x69;
pub const DA7213_DAC_R_CTRL: u32 = 0x6A;
pub const DA7213_HP_L_CTRL: u32 = 0x6B;
pub const DA7213_HP_R_CTRL: u32 = 0x6C;
pub const DA7213_LINE_CTRL: u32 = 0x6D;
pub const DA7213_MIXOUT_L_CTRL: u32 = 0x6E;
pub const DA7213_MIXOUT_R_CTRL: u32 = 0x6F;

/* Configuration Registers */
pub const DA7213_LDO_CTRL: u32 = 0x90;
pub const DA7213_IO_CTRL: u32 = 0x91;
pub const DA7213_GAIN_RAMP_CTRL: u32 = 0x92;
pub const DA7213_MIC_CONFIG: u32 = 0x93;
pub const DA7213_PC_COUNT: u32 = 0x94;
pub const DA7213_CP_VOL_THRESHOLD1: u32 = 0x95;
pub const DA7213_CP_DELAY: u32 = 0x96;
pub const DA7213_CP_DETECTOR: u32 = 0x97;
pub const DA7213_DAI_OFFSET: u32 = 0x98;
pub const DA7213_DIG_CTRL: u32 = 0x99;
pub const DA7213_ALC_CTRL2: u32 = 0x9A;
pub const DA7213_ALC_CTRL3: u32 = 0x9B;
pub const DA7213_ALC_NOISE: u32 = 0x9C;
pub const DA7213_ALC_TARGET_MIN: u32 = 0x9D;
pub const DA7213_ALC_TARGET_MAX: u32 = 0x9E;
pub const DA7213_ALC_GAIN_LIMITS: u32 = 0x9F;
pub const DA7213_ALC_ANA_GAIN_LIMITS: u32 = 0xA0;
pub const DA7213_ALC_ANTICLIP_CTRL: u32 = 0xA1;
pub const DA7213_ALC_ANTICLIP_LEVEL: u32 = 0xA2;

pub const DA7213_ALC_OFFSET_AUTO_M_L: u32 = 0xA3;
pub const DA7213_ALC_OFFSET_AUTO_U_L: u32 = 0xA4;
pub const DA7213_ALC_OFFSET_MAN_M_L: u32 = 0xA6;
pub const DA7213_ALC_OFFSET_MAN_U_L: u32 = 0xA7;
pub const DA7213_ALC_OFFSET_AUTO_M_R: u32 = 0xA8;
pub const DA7213_ALC_OFFSET_AUTO_U_R: u32 = 0xA9;
pub const DA7213_ALC_OFFSET_MAN_M_R: u32 = 0xAB;
pub const DA7213_ALC_OFFSET_MAN_U_R: u32 = 0xAC;
pub const DA7213_ALC_CIC_OP_LVL_CTRL: u32 = 0xAD;
pub const DA7213_ALC_CIC_OP_LVL_DATA: u32 = 0xAE;
pub const DA7213_DAC_NG_SETUP_TIME: u32 = 0xAF;
pub const DA7213_DAC_NG_OFF_THRESHOLD: u32 = 0xB0;
pub const DA7213_DAC_NG_ON_THRESHOLD: u32 = 0xB1;
pub const DA7213_DAC_NG_CTRL: u32 = 0xB2;

pub const DA7213_TONE_GEN_CFG1: u32 = 0xB4;
pub const DA7213_TONE_GEN_CFG2: u32 = 0xB5;
pub const DA7213_TONE_GEN_CYCLES: u32 = 0xB6;
pub const DA7213_TONE_GEN_FREQ1_L: u32 = 0xB7;
pub const DA7213_TONE_GEN_FREQ1_U: u32 = 0xB8;
pub const DA7213_TONE_GEN_FREQ2_L: u32 = 0xB9;
pub const DA7213_TONE_GEN_FREQ2_U: u32 = 0xBA;
pub const DA7213_TONE_GEN_ON_PER: u32 = 0xBB;
pub const DA7213_TONE_GEN_OFF_PER: u32 = 0xBC;

/*
 * Bit fields
 */

pub const DA7213_SWITCH_EN_MAX: u32 = 0x1;

/* DA7213_PLL_STATUS = 0x03 */
pub const DA7213_PLL_SRM_LOCK: u32 = 0x1 << 1;

/* DA7213_SR = 0x22 */
pub const DA7213_SR_8000: u32 = 0x1 << 0;
pub const DA7213_SR_11025: u32 = 0x2 << 0;
pub const DA7213_SR_12000: u32 = 0x3 << 0;
pub const DA7213_SR_16000: u32 = 0x5 << 0;
pub const DA7213_SR_22050: u32 = 0x6 << 0;
pub const DA7213_SR_24000: u32 = 0x7 << 0;
pub const DA7213_SR_32000: u32 = 0x9 << 0;
pub const DA7213_SR_44100: u32 = 0xA << 0;
pub const DA7213_SR_48000: u32 = 0xB << 0;
pub const DA7213_SR_88200: u32 = 0xE << 0;
pub const DA7213_SR_96000: u32 = 0xF << 0;

/* DA7213_REFERENCES = 0x23 */
pub const DA7213_BIAS_EN: u32 = 0x1 << 3;
pub const DA7213_VMID_EN: u32 = 0x1 << 7;

/* DA7213_PLL_CTRL = 0x27 */
pub const DA7213_PLL_INDIV_5_TO_9_MHZ: u32 = 0x0 << 2;
pub const DA7213_PLL_INDIV_9_TO_18_MHZ: u32 = 0x1 << 2;
pub const DA7213_PLL_INDIV_18_TO_36_MHZ: u32 = 0x2 << 2;
pub const DA7213_PLL_INDIV_36_TO_54_MHZ: u32 = 0x3 << 2;
pub const DA7213_PLL_INDIV_MASK: u32 = 0x3 << 2;
pub const DA7213_PLL_MCLK_SQR_EN: u32 = 0x1 << 4;
pub const DA7213_PLL_32K_MODE: u32 = 0x1 << 5;
pub const DA7213_PLL_SRM_EN: u32 = 0x1 << 6;
pub const DA7213_PLL_EN: u32 = 0x1 << 7;
pub const DA7213_PLL_MODE_MASK: u32 = 0x7 << 5;

/* DA7213_DAI_CLK_MODE = 0x28 */
pub const DA7213_DAI_BCLKS_PER_WCLK_32: u32 = 0x0 << 0;
pub const DA7213_DAI_BCLKS_PER_WCLK_64: u32 = 0x1 << 0;
pub const DA7213_DAI_BCLKS_PER_WCLK_128: u32 = 0x2 << 0;
pub const DA7213_DAI_BCLKS_PER_WCLK_256: u32 = 0x3 << 0;
pub const DA7213_DAI_BCLKS_PER_WCLK_MASK: u32 = 0x3 << 0;
pub const DA7213_DAI_CLK_POL_INV: u32 = 0x1 << 2;
pub const DA7213_DAI_CLK_POL_MASK: u32 = 0x1 << 2;
pub const DA7213_DAI_WCLK_POL_INV: u32 = 0x1 << 3;
pub const DA7213_DAI_WCLK_POL_MASK: u32 = 0x1 << 3;
pub const DA7213_DAI_CLK_EN_MASK: u32 = 0x1 << 7;

/* DA7213_DAI_CTRL = 0x29 */
pub const DA7213_DAI_FORMAT_I2S_MODE: u32 = 0x0 << 0;
pub const DA7213_DAI_FORMAT_LEFT_J: u32 = 0x1 << 0;
pub const DA7213_DAI_FORMAT_RIGHT_J: u32 = 0x2 << 0;
pub const DA7213_DAI_FORMAT_DSP: u32 = 0x3 << 0;
pub const DA7213_DAI_FORMAT_MASK: u32 = 0x3 << 0;
pub const DA7213_DAI_WORD_LENGTH_S16_LE: u32 = 0x0 << 2;
pub const DA7213_DAI_WORD_LENGTH_S20_LE: u32 = 0x1 << 2;
pub const DA7213_DAI_WORD_LENGTH_S24_LE: u32 = 0x2 << 2;
pub const DA7213_DAI_WORD_LENGTH_S32_LE: u32 = 0x3 << 2;
pub const DA7213_DAI_WORD_LENGTH_MASK: u32 = 0x3 << 2;
pub const DA7213_DAI_MONO_MODE_EN: u32 = 0x1 << 4;
pub const DA7213_DAI_MONO_MODE_MASK: u32 = 0x1 << 4;
pub const DA7213_DAI_EN_SHIFT: u32 = 7;

/* DA7213_DIG_ROUTING_DAI = 0x21 */
pub const DA7213_DAI_L_SRC_SHIFT: u32 = 0;
pub const DA7213_DAI_R_SRC_SHIFT: u32 = 4;
pub const DA7213_DAI_SRC_MAX: u32 = 4;

/* DA7213_DIG_ROUTING_DAC = 0x2A */
pub const DA7213_DAC_L_SRC_SHIFT: u32 = 0;
pub const DA7213_DAC_L_MONO_SHIFT: u32 = 3;
pub const DA7213_DAC_R_SRC_SHIFT: u32 = 4;
pub const DA7213_DAC_R_MONO_SHIFT: u32 = 7;
pub const DA7213_DAC_SRC_MAX: u32 = 4;
pub const DA7213_DAC_MONO_MAX: u32 = 0x1;

/* DA7213_ALC_CTRL1 = 0x2B */
pub const DA7213_ALC_OFFSET_EN_SHIFT: u32 = 0;
pub const DA7213_ALC_OFFSET_EN_MAX: u32 = 0x1;
pub const DA7213_ALC_OFFSET_EN: u32 = 0x1 << 0;
pub const DA7213_ALC_SYNC_MODE: u32 = 0x1 << 1;
pub const DA7213_ALC_CALIB_MODE_MAN: u32 = 0x1 << 2;
pub const DA7213_ALC_L_EN_SHIFT: u32 = 3;
pub const DA7213_ALC_AUTO_CALIB_EN: u32 = 0x1 << 4;
pub const DA7213_ALC_CALIB_OVERFLOW: u32 = 0x1 << 5;
pub const DA7213_ALC_R_EN_SHIFT: u32 = 7;
pub const DA7213_ALC_EN_MAX: u32 = 0x1;

/* DA7213_AUX_L/R_GAIN = 0x30/0x31 */
pub const DA7213_AUX_AMP_GAIN_SHIFT: u32 = 0;
pub const DA7213_AUX_AMP_GAIN_MAX: u32 = 0x3F;

/* DA7213_MIXIN_L/R_SELECT = 0x32/0x33 */
pub const DA7213_DMIC_EN_SHIFT: u32 = 7;
pub const DA7213_DMIC_EN_MAX: u32 = 0x1;

/* DA7213_MIXIN_L_SELECT = 0x32 */
pub const DA7213_MIXIN_L_MIX_SELECT_AUX_L_SHIFT: u32 = 0;
pub const DA7213_MIXIN_L_MIX_SELECT_MIC_1_SHIFT: u32 = 1;
pub const DA7213_MIXIN_L_MIX_SELECT_MIC_1: u32 = 0x1 << 1;
pub const DA7213_MIXIN_L_MIX_SELECT_MIC_2_SHIFT: u32 = 2;
pub const DA7213_MIXIN_L_MIX_SELECT_MIC_2: u32 = 0x1 << 2;
pub const DA7213_MIXIN_L_MIX_SELECT_MIXIN_R_SHIFT: u32 = 3;
pub const DA7213_MIXIN_L_MIX_SELECT_MAX: u32 = 0x1;

/* DA7213_MIXIN_R_SELECT =  0x33 */
pub const DA7213_MIXIN_R_MIX_SELECT_AUX_R_SHIFT: u32 = 0;
pub const DA7213_MIXIN_R_MIX_SELECT_MIC_2_SHIFT: u32 = 1;
pub const DA7213_MIXIN_R_MIX_SELECT_MIC_2: u32 = 0x1 << 1;
pub const DA7213_MIXIN_R_MIX_SELECT_MIC_1_SHIFT: u32 = 2;
pub const DA7213_MIXIN_R_MIX_SELECT_MIC_1: u32 = 0x1 << 2;
pub const DA7213_MIXIN_R_MIX_SELECT_MIXIN_L_SHIFT: u32 = 3;
pub const DA7213_MIXIN_R_MIX_SELECT_MAX: u32 = 0x1;
pub const DA7213_MIC_BIAS_OUTPUT_SELECT_2: u32 = 0x1 << 6;

/* DA7213_MIXIN_L/R_GAIN = 0x34/0x35 */
pub const DA7213_MIXIN_AMP_GAIN_SHIFT: u32 = 0;
pub const DA7213_MIXIN_AMP_GAIN_MAX: u32 = 0xF;

/* DA7213_ADC_L/R_GAIN = 0x36/0x37 */
pub const DA7213_ADC_AMP_GAIN_SHIFT: u32 = 0;
pub const DA7213_ADC_AMP_GAIN_MAX: u32 = 0x7F;

/* DA7213_ADC/DAC_FILTERS1 = 0x38/0x44 */
pub const DA7213_VOICE_HPF_CORNER_SHIFT: u32 = 0;
pub const DA7213_VOICE_HPF_CORNER_MAX: u32 = 8;
pub const DA7213_VOICE_EN_SHIFT: u32 = 3;
pub const DA7213_VOICE_EN_MAX: u32 = 0x1;
pub const DA7213_AUDIO_HPF_CORNER_SHIFT: u32 = 4;
pub const DA7213_AUDIO_HPF_CORNER_MAX: u32 = 4;
pub const DA7213_HPF_EN_SHIFT: u32 = 7;
pub const DA7213_HPF_EN_MAX: u32 = 0x1;

/* DA7213_MIC_1/2_GAIN = 0x39/0x3A */
pub const DA7213_MIC_AMP_GAIN_SHIFT: u32 = 0;
pub const DA7213_MIC_AMP_GAIN_MAX: u32 = 0x7;

/* DA7213_DAC_FILTERS5 = 0x40 */
pub const DA7213_DAC_SOFTMUTE_EN_SHIFT: u32 = 7;
pub const DA7213_DAC_SOFTMUTE_EN_MAX: u32 = 0x1;
pub const DA7213_DAC_SOFTMUTE_RATE_SHIFT: u32 = 4;
pub const DA7213_DAC_SOFTMUTE_RATE_MAX: u32 = 7;

/* DA7213_DAC_FILTERS2/3/4 = 0x41/0x42/0x43 */
pub const DA7213_DAC_EQ_BAND_MAX: u32 = 0xF;

/* DA7213_DAC_FILTERS2 = 0x41 */
pub const DA7213_DAC_EQ_BAND1_SHIFT: u32 = 0;
pub const DA7213_DAC_EQ_BAND2_SHIFT: u32 = 4;

/* DA7213_DAC_FILTERS2 = 0x42 */
pub const DA7213_DAC_EQ_BAND3_SHIFT: u32 = 0;
pub const DA7213_DAC_EQ_BAND4_SHIFT: u32 = 4;

/* DA7213_DAC_FILTERS4 = 0x43 */
pub const DA7213_DAC_EQ_BAND5_SHIFT: u32 = 0;
pub const DA7213_DAC_EQ_EN_SHIFT: u32 = 7;
pub const DA7213_DAC_EQ_EN_MAX: u32 = 0x1;

/* DA7213_DAC_L/R_GAIN = 0x45/0x46 */
pub const DA7213_DAC_AMP_GAIN_SHIFT: u32 = 0;
pub const DA7213_DAC_AMP_GAIN_MAX: u32 = 0x7F;

/* DA7213_HP_L/R_GAIN = 0x45/0x46 */
pub const DA7213_HP_AMP_GAIN_SHIFT: u32 = 0;
pub const DA7213_HP_AMP_GAIN_MAX: u32 = 0x3F;

/* DA7213_CP_CTRL = 0x47 */
pub const DA7213_CP_EN_SHIFT: u32 = 7;

/* DA7213_LINE_GAIN = 0x4A */
pub const DA7213_LINE_AMP_GAIN_SHIFT: u32 = 0;
pub const DA7213_LINE_AMP_GAIN_MAX: u32 = 0x3F;

/* DA7213_MIXOUT_L_SELECT = 0x4B */
pub const DA7213_MIXOUT_L_MIX_SELECT_AUX_L_SHIFT: u32 = 0;
pub const DA7213_MIXOUT_L_MIX_SELECT_MIXIN_L_SHIFT: u32 = 1;
pub const DA7213_MIXOUT_L_MIX_SELECT_MIXIN_R_SHIFT: u32 = 2;
pub const DA7213_MIXOUT_L_MIX_SELECT_DAC_L_SHIFT: u32 = 3;
pub const DA7213_MIXOUT_L_MIX_SELECT_AUX_L_INVERTED_SHIFT: u32 = 4;
pub const DA7213_MIXOUT_L_MIX_SELECT_MIXIN_L_INVERTED_SHIFT: u32 = 5;
pub const DA7213_MIXOUT_L_MIX_SELECT_MIXIN_R_INVERTED_SHIFT: u32 = 6;
pub const DA7213_MIXOUT_L_MIX_SELECT_MAX: u32 = 0x1;

/* DA7213_MIXOUT_R_SELECT = 0x4C */
pub const DA7213_MIXOUT_R_MIX_SELECT_AUX_R_SHIFT: u32 = 0;
pub const DA7213_MIXOUT_R_MIX_SELECT_MIXIN_R_SHIFT: u32 = 1;
pub const DA7213_MIXOUT_R_MIX_SELECT_MIXIN_L_SHIFT: u32 = 2;
pub const DA7213_MIXOUT_R_MIX_SELECT_DAC_R_SHIFT: u32 = 3;
pub const DA7213_MIXOUT_R_MIX_SELECT_AUX_R_INVERTED_SHIFT: u32 = 4;
pub const DA7213_MIXOUT_R_MIX_SELECT_MIXIN_R_INVERTED_SHIFT: u32 = 5;
pub const DA7213_MIXOUT_R_MIX_SELECT_MIXIN_L_INVERTED_SHIFT: u32 = 6;
pub const DA7213_MIXOUT_R_MIX_SELECT_MAX: u32 = 0x1;

/*
 * DA7213_AUX_L/R_CTRL = 0x60/0x61,
 * DA7213_MIC_1/2_CTRL = 0x63/0x64,
 * DA7213_MIXIN_L/R_CTRL = 0x65/0x66,
 * DA7213_ADC_L/R_CTRL = 0x65/0x66,
 * DA7213_DAC_L/R_CTRL = 0x69/0x6A,
 * DA7213_HP_L/R_CTRL = 0x6B/0x6C,
 * DA7213_LINE_CTRL = 0x6D
 */
pub const DA7213_MUTE_EN_SHIFT: u32 = 6;
pub const DA7213_MUTE_EN_MAX: u32 = 0x1;
pub const DA7213_MUTE_EN: u32 = 0x1 << 6;

/*
 * DA7213_AUX_L/R_CTRL = 0x60/0x61,
 * DA7213_MIXIN_L/R_CTRL = 0x65/0x66,
 * DA7213_ADC_L/R_CTRL = 0x65/0x66,
 * DA7213_DAC_L/R_CTRL = 0x69/0x6A,
 * DA7213_HP_L/R_CTRL = 0x6B/0x6C,
 * DA7213_LINE_CTRL = 0x6D
 */
pub const DA7213_GAIN_RAMP_EN_SHIFT: u32 = 5;
pub const DA7213_GAIN_RAMP_EN_MAX: u32 = 0x1;
pub const DA7213_GAIN_RAMP_EN: u32 = 0x1 << 5;

/*
 * DA7213_AUX_L/R_CTRL = 0x60/0x61,
 * DA7213_MIXIN_L/R_CTRL = 0x65/0x66,
 * DA7213_HP_L/R_CTRL = 0x6B/0x6C,
 * DA7213_LINE_CTRL = 0x6D
 */
pub const DA7213_ZC_EN_SHIFT: u32 = 4;
pub const DA7213_ZC_EN_MAX: u32 = 0x1;

/*
 * DA7213_AUX_L/R_CTRL = 0x60/0x61,
 * DA7213_MIC_1/2_CTRL = 0x63/0x64,
 * DA7213_MIXIN_L/R_CTRL = 0x65/0x66,
 * DA7213_HP_L/R_CTRL = 0x6B/0x6C,
 * DA7213_MIXOUT_L/R_CTRL = 0x6E/0x6F,
 * DA7213_LINE_CTRL = 0x6D
 */
pub const DA7213_AMP_EN_SHIFT: u32 = 7;

/* DA7213_MIC_1/2_CTRL = 0x63/0x64 */
pub const DA7213_MIC_AMP_IN_SEL_SHIFT: u32 = 2;
pub const DA7213_MIC_AMP_IN_SEL_MAX: u32 = 3;

/* DA7213_MICBIAS_CTRL = 0x62 */
pub const DA7213_MICBIAS1_LEVEL_SHIFT: u32 = 0;
pub const DA7213_MICBIAS1_LEVEL_MASK: u32 = 0x3 << 0;
pub const DA7213_MICBIAS1_EN_SHIFT: u32 = 3;
pub const DA7213_MICBIAS2_LEVEL_SHIFT: u32 = 4;
pub const DA7213_MICBIAS2_LEVEL_MASK: u32 = 0x3 << 4;
pub const DA7213_MICBIAS2_EN_SHIFT: u32 = 7;

/* DA7213_MIXIN_L/R_CTRL = 0x65/0x66 */
pub const DA7213_MIXIN_MIX_EN: u32 = 0x1 << 3;

/* DA7213_ADC_L/R_CTRL = 0x67/0x68 */
pub const DA7213_ADC_EN_SHIFT: u32 = 7;
pub const DA7213_ADC_EN: u32 = 0x1 << 7;

/* DA7213_DAC_L/R_CTRL =  0x69/0x6A*/
pub const DA7213_DAC_EN_SHIFT: u32 = 7;

/* DA7213_HP_L/R_CTRL = 0x6B/0x6C */
pub const DA7213_HP_AMP_OE: u32 = 0x1 << 3;

/* DA7213_LINE_CTRL = 0x6D */
pub const DA7213_LINE_AMP_OE: u32 = 0x1 << 3;

/* DA7213_MIXOUT_L/R_CTRL = 0x6E/0x6F */
pub const DA7213_MIXOUT_MIX_EN: u32 = 0x1 << 3;

/* DA7213_GAIN_RAMP_CTRL = 0x92 */
pub const DA7213_GAIN_RAMP_RATE_SHIFT: u32 = 0;
pub const DA7213_GAIN_RAMP_RATE_MAX: u32 = 4;

/* DA7213_MIC_CONFIG = 0x93 */
pub const DA7213_DMIC_DATA_SEL_SHIFT: u32 = 0;
pub const DA7213_DMIC_DATA_SEL_MASK: u32 = 0x1 << 0;
pub const DA7213_DMIC_SAMPLEPHASE_SHIFT: u32 = 1;
pub const DA7213_DMIC_SAMPLEPHASE_MASK: u32 = 0x1 << 1;
pub const DA7213_DMIC_CLK_RATE_SHIFT: u32 = 2;
pub const DA7213_DMIC_CLK_RATE_MASK: u32 = 0x1 << 2;

/* DA7213_PC_COUNT = 0x94 */
pub const DA7213_PC_FREERUN_MASK: u32 = 0x1 << 0;

/* DA7213_DIG_CTRL = 0x99 */
pub const DA7213_DAC_L_INV_SHIFT: u32 = 3;
pub const DA7213_DAC_R_INV_SHIFT: u32 = 7;
pub const DA7213_DAC_INV_MAX: u32 = 0x1;

/* DA7213_ALC_CTRL2 = 0x9A */
pub const DA7213_ALC_ATTACK_SHIFT: u32 = 0;
pub const DA7213_ALC_ATTACK_MAX: u32 = 13;
pub const DA7213_ALC_RELEASE_SHIFT: u32 = 4;
pub const DA7213_ALC_RELEASE_MAX: u32 = 11;

/* DA7213_ALC_CTRL3 = 0x9B */
pub const DA7213_ALC_HOLD_SHIFT: u32 = 0;
pub const DA7213_ALC_HOLD_MAX: u32 = 16;
pub const DA7213_ALC_INTEG_ATTACK_SHIFT: u32 = 4;
pub const DA7213_ALC_INTEG_RELEASE_SHIFT: u32 = 6;
pub const DA7213_ALC_INTEG_MAX: u32 = 4;

/*
 * DA7213_ALC_NOISE = 0x9C,
 * DA7213_ALC_TARGET_MIN/MAX = 0x9D/0x9E
 */
pub const DA7213_ALC_THRESHOLD_SHIFT: u32 = 0;
pub const DA7213_ALC_THRESHOLD_MAX: u32 = 0x3F;

/* DA7213_ALC_GAIN_LIMITS = 0x9F */
pub const DA7213_ALC_ATTEN_MAX_SHIFT: u32 = 0;
pub const DA7213_ALC_GAIN_MAX_SHIFT: u32 = 4;
pub const DA7213_ALC_ATTEN_GAIN_MAX_MAX: u32 = 0xF;

/* DA7213_ALC_ANA_GAIN_LIMITS = 0xA0 */
pub const DA7213_ALC_ANA_GAIN_MIN_SHIFT: u32 = 0;
pub const DA7213_ALC_ANA_GAIN_MAX_SHIFT: u32 = 4;
pub const DA7213_ALC_ANA_GAIN_MAX: u32 = 0x7;

/* DA7213_ALC_ANTICLIP_CTRL = 0xA1 */
pub const DA7213_ALC_ANTICLIP_EN_SHIFT: u32 = 7;
pub const DA7213_ALC_ANTICLIP_EN_MAX: u32 = 0x1;

/* DA7213_ALC_ANTICLIP_LEVEL = 0xA2 */
pub const DA7213_ALC_ANTICLIP_LEVEL_SHIFT: u32 = 0;
pub const DA7213_ALC_ANTICLIP_LEVEL_MAX: u32 = 0x7F;

/* DA7213_ALC_CIC_OP_LVL_CTRL = 0xAD */
pub const DA7213_ALC_DATA_MIDDLE: u32 = 0x2 << 0;
pub const DA7213_ALC_DATA_TOP: u32 = 0x3 << 0;
pub const DA7213_ALC_CIC_OP_CHANNEL_LEFT: u32 = 0x0 << 7;
pub const DA7213_ALC_CIC_OP_CHANNEL_RIGHT: u32 = 0x1 << 7;

/* DA7213_DAC_NG_SETUP_TIME = 0xAF */
pub const DA7213_DAC_NG_SETUP_TIME_SHIFT: u32 = 0;
pub const DA7213_DAC_NG_SETUP_TIME_MAX: u32 = 4;
pub const DA7213_DAC_NG_RAMPUP_RATE_SHIFT: u32 = 2;
pub const DA7213_DAC_NG_RAMPDN_RATE_SHIFT: u32 = 3;
pub const DA7213_DAC_NG_RAMP_RATE_MAX: u32 = 2;

/* DA7213_DAC_NG_OFF/ON_THRESH = 0xB0/0xB1 */
pub const DA7213_DAC_NG_THRESHOLD_SHIFT: u32 = 0;
pub const DA7213_DAC_NG_THRESHOLD_MAX: u32 = 0x7;

/* DA7213_DAC_NG_CTRL = 0xB2 */
pub const DA7213_DAC_NG_EN_SHIFT: u32 = 7;
pub const DA7213_DAC_NG_EN_MAX: u32 = 0x1;

/* DA7213_TONE_GEN_CFG1 = 0xB4 */
pub const DA7213_DTMF_REG_SHIFT: u32 = 0;
pub const DA7213_DTMF_REG_MASK: u32 = 0xF << 0;
pub const DA7213_DTMF_REG_MAX: u32 = 16;
pub const DA7213_DTMF_EN_SHIFT: u32 = 4;
pub const DA7213_DTMF_EN_MASK: u32 = 0x1 << 4;
pub const DA7213_START_STOPN_SHIFT: u32 = 7;
pub const DA7213_START_STOPN_MASK: u32 = 0x1 << 7;

/* DA7213_TONE_GEN_CFG2 = 0xB5 */
pub const DA7213_SWG_SEL_SHIFT: u32 = 0;
pub const DA7213_SWG_SEL_MASK: u32 = 0x3 << 0;
pub const DA7213_SWG_SEL_MAX: u32 = 4;
pub const DA7213_SWG_SEL_SRAMP: u32 = 0x3 << 0;
pub const DA7213_TONE_GEN_GAIN_SHIFT: u32 = 4;
pub const DA7213_TONE_GEN_GAIN_MASK: u32 = 0xF << 4;
pub const DA7213_TONE_GEN_GAIN_MAX: u32 = 0xF;
pub const DA7213_TONE_GEN_GAIN_MINUS_9DB: u32 = 0x3 << 4;
pub const DA7213_TONE_GEN_GAIN_MINUS_15DB: u32 = 0x5 << 4;

/* DA7213_TONE_GEN_CYCLES = 0xB6 */
pub const DA7213_BEEP_CYCLES_SHIFT: u32 = 0;
pub const DA7213_BEEP_CYCLES_MASK: u32 = 0x7 << 0;

/* DA7213_TONE_GEN_FREQ1_L = 0xB7 */
pub const DA7213_FREQ1_L_SHIFT: u32 = 0;
pub const DA7213_FREQ1_L_MASK: u32 = 0xFF << 0;
pub const DA7213_FREQ_MAX: u32 = 0xFFFF;

/* DA7213_TONE_GEN_FREQ1_U = 0xB8 */
pub const DA7213_FREQ1_U_SHIFT: u32 = 0;
pub const DA7213_FREQ1_U_MASK: u32 = 0xFF << 0;

/* DA7213_TONE_GEN_FREQ2_L = 0xB9 */
pub const DA7213_FREQ2_L_SHIFT: u32 = 0;
pub const DA7213_FREQ2_L_MASK: u32 = 0xFF << 0;

/* DA7213_TONE_GEN_FREQ2_U = 0xBA */
pub const DA7213_FREQ2_U_SHIFT: u32 = 0;
pub const DA7213_FREQ2_U_MASK: u32 = 0xFF << 0;

/* DA7213_TONE_GEN_ON_PER = 0xBB */
pub const DA7213_BEEP_ON_PER_SHIFT: u32 = 0;
pub const DA7213_BEEP_ON_PER_MASK: u32 = 0x3F << 0;
pub const DA7213_BEEP_ON_OFF_MAX: u32 = 0x3F;

/* DA7213_TONE_GEN_OFF_PER = 0xBC */
pub const DA7213_BEEP_OFF_PER_SHIFT: u32 = 0;
pub const DA7213_BEEP_OFF_PER_MASK: u32 = 0x3F << 0;

/*
 * General defines
 */

/* Register inversion */
pub const DA7213_NO_INVERT: u32 = 0;
pub const DA7213_INVERT: u32 = 1;

/* Byte related defines */
pub const DA7213_BYTE_SHIFT: u32 = 8;
pub const DA7213_BYTE_MASK: u32 = 0xFF;

/* ALC related */
pub const DA7213_ALC_OFFSET_15_8: u32 = 0x00FF00;
pub const DA7213_ALC_OFFSET_19_16: u32 = 0x0F0000;
pub const DA7213_ALC_AVG_ITERATIONS: u32 = 5;

/* PLL related */
pub const DA7213_PLL_FREQ_OUT_90316800: u32 = 90316800;
pub const DA7213_PLL_FREQ_OUT_98304000: u32 = 98304000;
pub const DA7213_PLL_FREQ_OUT_94310400: u32 = 94310400;
pub const DA7213_PLL_INDIV_5_TO_9_MHZ_VAL: u32 = 2;
pub const DA7213_PLL_INDIV_9_TO_18_MHZ_VAL: u32 = 4;
pub const DA7213_PLL_INDIV_18_TO_36_MHZ_VAL: u32 = 8;
pub const DA7213_PLL_INDIV_36_TO_54_MHZ_VAL: u32 = 16;
pub const DA7213_SRM_CHECK_RETRIES: u32 = 8;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum da7213_clk_src {
    DA7213_CLKSRC_MCLK = 0,
    DA7213_CLKSRC_MCLK_SQR = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum da7213_sys_clk {
    DA7213_SYSCLK_MCLK = 0,
    DA7213_SYSCLK_PLL = 1,
    DA7213_SYSCLK_PLL_SRM = 2,
    DA7213_SYSCLK_PLL_32KHZ = 3,
}

/* Regulators */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum da7213_supplies {
    DA7213_SUPPLY_VDDA = 0,
    DA7213_SUPPLY_VDDIO = 1,
    DA7213_NUM_SUPPLIES = 2,
}

pub const DA7213_NUM_SUPPLIES: usize = da7213_supplies::DA7213_NUM_SUPPLIES as usize;

/* External C types supplied by included headers. */
pub enum regmap {}
pub enum device {}
pub enum mutex {}
pub enum regulator_bulk_data {}
pub enum clk {}
pub enum da7213_platform_data {}

/* Codec private data */
#[repr(C)]
pub struct da7213_priv {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub ctrl_lock: mutex,
    pub supplies: [regulator_bulk_data; DA7213_NUM_SUPPLIES],
    pub mclk: *mut clk,
    pub mclk_rate: ::std::os::raw::c_uint,
    pub out_rate: ::std::os::raw::c_uint,
    pub fin_min_rate: ::std::os::raw::c_uint,
    pub clk_src: ::std::os::raw::c_int,
    pub master: bool,
    pub alc_calib_auto: bool,
    pub alc_en: bool,
    pub fixed_clk_auto_pll: bool,
    pub pdata: *mut da7213_platform_data,
    pub fmt: ::std::os::raw::c_int,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
