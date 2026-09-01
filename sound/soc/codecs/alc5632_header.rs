/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * alc5632.h  --  ALC5632 ALSA SoC Audio Codec
 *
 * Copyright (C) 2011 The AC100 Kernel Team <ac100@lists.lauchpad.net>
 *
 * Authors:  Leon Romanovsky <leon@leon.nu>
 *           Andrey Danin <danindrey@mail.ru>
 *           Ilya Petrov <ilya.muromec@gmail.com>
 *           Marc Dietrich <marvin24@gmx.de>
 *
 * Based on alc5623.h by Arnaud Patard
 */

pub const ALC5632_RESET: u32 = 0x00;
/* speaker output vol		   2    2           */
/* line output vol                      4    2      */
/* HP output vol		   4    0    4      */
pub const ALC5632_SPK_OUT_VOL: u32 = 0x02; /* spe out vol */
pub const ALC5632_SPK_OUT_VOL_STEP: f64 = 1.5;
pub const ALC5632_HP_OUT_VOL: u32 = 0x04; /* hp out vol */
pub const ALC5632_AUX_OUT_VOL: u32 = 0x06; /* aux out vol */
pub const ALC5632_PHONE_IN_VOL: u32 = 0x08; /* phone in vol */
pub const ALC5632_LINE_IN_VOL: u32 = 0x0A; /* line in vol */
pub const ALC5632_STEREO_DAC_IN_VOL: u32 = 0x0C; /* stereo dac in vol */
pub const ALC5632_MIC_VOL: u32 = 0x0E; /* mic in vol */
/* stero dac/mic routing */
pub const ALC5632_MIC_ROUTING_CTRL: u32 = 0x10;
pub const ALC5632_MIC_ROUTE_MONOMIX: u32 = 1 << 0;
pub const ALC5632_MIC_ROUTE_SPK: u32 = 1 << 1;
pub const ALC5632_MIC_ROUTE_HP: u32 = 1 << 2;

pub const ALC5632_ADC_REC_GAIN: u32 = 0x12; /* rec gain */
pub const ALC5632_ADC_REC_GAIN_RANGE: u32 = 0x1F1F;
pub const ALC5632_ADC_REC_GAIN_BASE: f64 = -16.5;
pub const ALC5632_ADC_REC_GAIN_STEP: f64 = 1.5;

pub const ALC5632_ADC_REC_MIXER: u32 = 0x14; /* mixer control */
pub const ALC5632_ADC_REC_MIC1: u32 = 1 << 6;
pub const ALC5632_ADC_REC_MIC2: u32 = 1 << 5;
pub const ALC5632_ADC_REC_LINE_IN: u32 = 1 << 4;
pub const ALC5632_ADC_REC_AUX: u32 = 1 << 3;
pub const ALC5632_ADC_REC_HP: u32 = 1 << 2;
pub const ALC5632_ADC_REC_SPK: u32 = 1 << 1;
pub const ALC5632_ADC_REC_MONOMIX: u32 = 1 << 0;

pub const ALC5632_VOICE_DAC_VOL: u32 = 0x18; /* voice dac vol */
pub const ALC5632_I2S_OUT_CTL: u32 = 0x1A; /* undocumented reg. found in path scheme */
/* ALC5632_OUTPUT_MIXER_CTRL :			*/
/* same remark as for reg 2 line vs speaker	*/
pub const ALC5632_OUTPUT_MIXER_CTRL: u32 = 0x1C; /* out mix ctrl */
pub const ALC5632_OUTPUT_MIXER_RP: u32 = 1 << 14;
pub const ALC5632_OUTPUT_MIXER_WEEK: u32 = 1 << 12;
pub const ALC5632_OUTPUT_MIXER_HP: u32 = 1 << 10;
pub const ALC5632_OUTPUT_MIXER_AUX_SPK: u32 = 2 << 6;
pub const ALC5632_OUTPUT_MIXER_AUX_HP_LR: u32 = 1 << 6;
pub const ALC5632_OUTPUT_MIXER_HP_R: u32 = 1 << 8;
pub const ALC5632_OUTPUT_MIXER_HP_L: u32 = 1 << 9;

pub const ALC5632_MIC_CTRL: u32 = 0x22; /* mic phone ctrl */
pub const ALC5632_MIC_BOOST_BYPASS: u32 = 0;
pub const ALC5632_MIC_BOOST_20DB: u32 = 1;
pub const ALC5632_MIC_BOOST_30DB: u32 = 2;
pub const ALC5632_MIC_BOOST_40DB: u32 = 3;

pub const ALC5632_DIGI_BOOST_CTRL: u32 = 0x24; /* digi mic / bost ctl */
pub const ALC5632_MIC_BOOST_RANGE: u32 = 7;
pub const ALC5632_MIC_BOOST_STEP: u32 = 6;
pub const ALC5632_PWR_DOWN_CTRL_STATUS: u32 = 0x26;
pub const ALC5632_PWR_DOWN_CTRL_STATUS_MASK: u32 = 0xEF00;
pub const ALC5632_PWR_VREF_PR3: u32 = 1 << 11;
pub const ALC5632_PWR_VREF_PR2: u32 = 1 << 10;
pub const ALC5632_PWR_VREF_STATUS: u32 = 1 << 3;
pub const ALC5632_PWR_AMIX_STATUS: u32 = 1 << 2;
pub const ALC5632_PWR_DAC_STATUS: u32 = 1 << 1;
pub const ALC5632_PWR_ADC_STATUS: u32 = 1 << 0;
/* stereo/voice DAC / stereo adc func ctrl */
pub const ALC5632_DAC_FUNC_SELECT: u32 = 0x2E;

/* Main serial data port ctrl (i2s) */
pub const ALC5632_DAI_CONTROL: u32 = 0x34;

pub const ALC5632_DAI_SDP_MASTER_MODE: u32 = 0 << 15;
pub const ALC5632_DAI_SDP_SLAVE_MODE: u32 = 1 << 15;
pub const ALC5632_DAI_SADLRCK_MODE: u32 = 1 << 14;
/* 0:voice, 1:main */
pub const ALC5632_DAI_MAIN_I2S_SYSCLK_SEL: u32 = 1 << 8;
pub const ALC5632_DAI_MAIN_I2S_BCLK_POL_CTRL: u32 = 1 << 7;
/* 0:normal, 1:invert */
pub const ALC5632_DAI_MAIN_I2S_LRCK_INV: u32 = 1 << 6;
pub const ALC5632_DAI_I2S_DL_MASK: u32 = 3 << 2;
pub const ALC5632_DAI_I2S_DL_8: u32 = 3 << 2;
pub const ALC5632_DAI_I2S_DL_24: u32 = 2 << 2;
pub const ALC5632_DAI_I2S_DL_20: u32 = 1 << 2;
pub const ALC5632_DAI_I2S_DL_16: u32 = 0 << 2;
pub const ALC5632_DAI_I2S_DF_MASK: u32 = 3 << 0;
pub const ALC5632_DAI_I2S_DF_PCM_B: u32 = 3 << 0;
pub const ALC5632_DAI_I2S_DF_PCM_A: u32 = 2 << 0;
pub const ALC5632_DAI_I2S_DF_LEFT: u32 = 1 << 0;
pub const ALC5632_DAI_I2S_DF_I2S: u32 = 0 << 0;
/* extend serial data port control (VoDAC_i2c/pcm) */
pub const ALC5632_DAI_CONTROL2: u32 = 0x36;
/* 0:gpio func, 1:voice pcm */
pub const ALC5632_DAI_VOICE_PCM_ENABLE: u32 = 1 << 15;
/* 0:master, 1:slave */
pub const ALC5632_DAI_VOICE_MODE_SEL: u32 = 1 << 14;
/* 0:disable, 1:enable */
pub const ALC5632_DAI_HPF_CLK_CTRL: u32 = 1 << 13;
/* 0:main, 1:voice */
pub const ALC5632_DAI_VOICE_I2S_SYSCLK_SEL: u32 = 1 << 8;
/* 0:normal, 1:invert */
pub const ALC5632_DAI_VOICE_VBCLK_SYSCLK_SEL: u32 = 1 << 7;
/* 0:normal, 1:invert */
pub const ALC5632_DAI_VOICE_I2S_LR_INV: u32 = 1 << 6;
pub const ALC5632_DAI_VOICE_DL_MASK: u32 = 3 << 2;
pub const ALC5632_DAI_VOICE_DL_16: u32 = 0 << 2;
pub const ALC5632_DAI_VOICE_DL_20: u32 = 1 << 2;
pub const ALC5632_DAI_VOICE_DL_24: u32 = 2 << 2;
pub const ALC5632_DAI_VOICE_DL_8: u32 = 3 << 2;
pub const ALC5632_DAI_VOICE_DF_MASK: u32 = 3 << 0;
pub const ALC5632_DAI_VOICE_DF_I2S: u32 = 0 << 0;
pub const ALC5632_DAI_VOICE_DF_LEFT: u32 = 1 << 0;
pub const ALC5632_DAI_VOICE_DF_PCM_A: u32 = 2 << 0;
pub const ALC5632_DAI_VOICE_DF_PCM_B: u32 = 3 << 0;

pub const ALC5632_PWR_MANAG_ADD1: u32 = 0x3A;
pub const ALC5632_PWR_MANAG_ADD1_MASK: u32 = 0xEFFF;
pub const ALC5632_PWR_ADD1_DAC_L_EN: u32 = 1 << 15;
pub const ALC5632_PWR_ADD1_DAC_R_EN: u32 = 1 << 14;
pub const ALC5632_PWR_ADD1_ZERO_CROSS: u32 = 1 << 13;
pub const ALC5632_PWR_ADD1_MAIN_I2S_EN: u32 = 1 << 11;
pub const ALC5632_PWR_ADD1_SPK_AMP_EN: u32 = 1 << 10;
pub const ALC5632_PWR_ADD1_HP_OUT_AMP: u32 = 1 << 9;
pub const ALC5632_PWR_ADD1_HP_OUT_ENH_AMP: u32 = 1 << 8;
pub const ALC5632_PWR_ADD1_VOICE_DAC_MIX: u32 = 1 << 7;
pub const ALC5632_PWR_ADD1_SOFTGEN_EN: u32 = 1 << 6;
pub const ALC5632_PWR_ADD1_MIC1_SHORT_CURR: u32 = 1 << 5;
pub const ALC5632_PWR_ADD1_MIC2_SHORT_CURR: u32 = 1 << 4;
pub const ALC5632_PWR_ADD1_MIC1_EN: u32 = 1 << 3;
pub const ALC5632_PWR_ADD1_MIC2_EN: u32 = 1 << 2;
pub const ALC5632_PWR_ADD1_MAIN_BIAS: u32 = 1 << 1;
pub const ALC5632_PWR_ADD1_DAC_REF: u32 = 1 << 0;

pub const ALC5632_PWR_MANAG_ADD2: u32 = 0x3C;
pub const ALC5632_PWR_MANAG_ADD2_MASK: u32 = 0x7FFF;
pub const ALC5632_PWR_ADD2_PLL1: u32 = 1 << 15;
pub const ALC5632_PWR_ADD2_PLL2: u32 = 1 << 14;
pub const ALC5632_PWR_ADD2_VREF: u32 = 1 << 13;
pub const ALC5632_PWR_ADD2_OVT_DET: u32 = 1 << 12;
pub const ALC5632_PWR_ADD2_VOICE_DAC: u32 = 1 << 10;
pub const ALC5632_PWR_ADD2_L_DAC_CLK: u32 = 1 << 9;
pub const ALC5632_PWR_ADD2_R_DAC_CLK: u32 = 1 << 8;
pub const ALC5632_PWR_ADD2_L_ADC_CLK_GAIN: u32 = 1 << 7;
pub const ALC5632_PWR_ADD2_R_ADC_CLK_GAIN: u32 = 1 << 6;
pub const ALC5632_PWR_ADD2_L_HP_MIXER: u32 = 1 << 5;
pub const ALC5632_PWR_ADD2_R_HP_MIXER: u32 = 1 << 4;
pub const ALC5632_PWR_ADD2_SPK_MIXER: u32 = 1 << 3;
pub const ALC5632_PWR_ADD2_MONO_MIXER: u32 = 1 << 2;
pub const ALC5632_PWR_ADD2_L_ADC_REC_MIXER: u32 = 1 << 1;
pub const ALC5632_PWR_ADD2_R_ADC_REC_MIXER: u32 = 1 << 0;

pub const ALC5632_PWR_MANAG_ADD3: u32 = 0x3E;
pub const ALC5632_PWR_MANAG_ADD3_MASK: u32 = 0x7CFF;
pub const ALC5632_PWR_ADD3_AUXOUT_VOL: u32 = 1 << 14;
pub const ALC5632_PWR_ADD3_SPK_L_OUT: u32 = 1 << 13;
pub const ALC5632_PWR_ADD3_SPK_R_OUT: u32 = 1 << 12;
pub const ALC5632_PWR_ADD3_HP_L_OUT_VOL: u32 = 1 << 11;
pub const ALC5632_PWR_ADD3_HP_R_OUT_VOL: u32 = 1 << 10;
pub const ALC5632_PWR_ADD3_LINEIN_L_VOL: u32 = 1 << 7;
pub const ALC5632_PWR_ADD3_LINEIN_R_VOL: u32 = 1 << 6;
pub const ALC5632_PWR_ADD3_AUXIN_VOL: u32 = 1 << 5;
pub const ALC5632_PWR_ADD3_AUXIN_MIX: u32 = 1 << 4;
pub const ALC5632_PWR_ADD3_MIC1_VOL: u32 = 1 << 3;
pub const ALC5632_PWR_ADD3_MIC2_VOL: u32 = 1 << 2;
pub const ALC5632_PWR_ADD3_MIC1_BOOST_AD: u32 = 1 << 1;
pub const ALC5632_PWR_ADD3_MIC2_BOOST_AD: u32 = 1 << 0;

pub const ALC5632_GPCR1: u32 = 0x40;
pub const ALC5632_GPCR1_CLK_SYS_SRC_SEL_PLL1: u32 = 1 << 15;
pub const ALC5632_GPCR1_CLK_SYS_SRC_SEL_MCLK: u32 = 0 << 15;
pub const ALC5632_GPCR1_DAC_HI_FLT_EN: u32 = 1 << 10;
pub const ALC5632_GPCR1_SPK_AMP_CTRL: u32 = 7 << 1;
pub const ALC5632_GPCR1_VDD_100: u32 = 5 << 1;
pub const ALC5632_GPCR1_VDD_125: u32 = 4 << 1;
pub const ALC5632_GPCR1_VDD_150: u32 = 3 << 1;
pub const ALC5632_GPCR1_VDD_175: u32 = 2 << 1;
pub const ALC5632_GPCR1_VDD_200: u32 = 1 << 1;
pub const ALC5632_GPCR1_VDD_225: u32 = 0 << 1;

pub const ALC5632_GPCR2: u32 = 0x42;
pub const ALC5632_GPCR2_PLL1_SOUR_SEL: u32 = 3 << 12;
pub const ALC5632_PLL_FR_MCLK: u32 = 0 << 12;
pub const ALC5632_PLL_FR_BCLK: u32 = 2 << 12;
pub const ALC5632_PLL_FR_VBCLK: u32 = 3 << 12;
pub const ALC5632_GPCR2_CLK_PLL_PRE_DIV1: u32 = 0 << 0;

pub const ALC5632_PLL1_CTRL: u32 = 0x44;
pub const fn ALC5632_PLL1_CTRL_N_VAL(n: u32) -> u32 {
    (n & 0x0f) << 8
}
pub const ALC5632_PLL1_M_BYPASS: u32 = 1 << 7;
pub const fn ALC5632_PLL1_CTRL_K_VAL(k: u32) -> u32 {
    (k & 0x07) << 4
}
pub const fn ALC5632_PLL1_CTRL_M_VAL(m: u32) -> u32 {
    (m & 0x0f) << 0
}

pub const ALC5632_PLL2_CTRL: u32 = 0x46;
pub const ALC5632_PLL2_EN: u32 = 1 << 15;
pub const ALC5632_PLL2_RATIO: u32 = 0 << 15;

pub const ALC5632_GPIO_PIN_CONFIG: u32 = 0x4C;
pub const ALC5632_GPIO_PIN_POLARITY: u32 = 0x4E;
pub const ALC5632_GPIO_PIN_STICKY: u32 = 0x50;
pub const ALC5632_GPIO_PIN_WAKEUP: u32 = 0x52;
pub const ALC5632_GPIO_PIN_STATUS: u32 = 0x54;
pub const ALC5632_GPIO_PIN_SHARING: u32 = 0x56;
pub const ALC5632_OVER_CURR_STATUS: u32 = 0x58;
pub const ALC5632_SOFTVOL_CTRL: u32 = 0x5A;
pub const ALC5632_GPIO_OUPUT_PIN_CTRL: u32 = 0x5C;

pub const ALC5632_MISC_CTRL: u32 = 0x5E;
pub const ALC5632_MISC_DISABLE_FAST_VREG: u32 = 1 << 15;
pub const ALC5632_MISC_AVC_TRGT_SEL: u32 = 3 << 12;
pub const ALC5632_MISC_AVC_TRGT_RIGHT: u32 = 1 << 12;
pub const ALC5632_MISC_AVC_TRGT_LEFT: u32 = 2 << 12;
pub const ALC5632_MISC_AVC_TRGT_BOTH: u32 = 3 << 12;
pub const ALC5632_MISC_HP_DEPOP_MODE1_EN: u32 = 1 << 9;
pub const ALC5632_MISC_HP_DEPOP_MODE2_EN: u32 = 1 << 8;
pub const ALC5632_MISC_HP_DEPOP_MUTE_L: u32 = 1 << 7;
pub const ALC5632_MISC_HP_DEPOP_MUTE_R: u32 = 1 << 6;
pub const ALC5632_MISC_HP_DEPOP_MUTE: u32 = 1 << 5;
pub const ALC5632_MISC_GPIO_WAKEUP_CTRL: u32 = 1 << 1;
pub const ALC5632_MISC_IRQOUT_INV_CTRL: u32 = 1 << 0;

pub const ALC5632_DAC_CLK_CTRL1: u32 = 0x60;
pub const ALC5632_DAC_CLK_CTRL2: u32 = 0x62;
pub const ALC5632_DAC_CLK_CTRL2_DIV1_2: u32 = 1 << 0;
pub const ALC5632_VOICE_DAC_PCM_CLK_CTRL1: u32 = 0x64;
pub const ALC5632_PSEUDO_SPATIAL_CTRL: u32 = 0x68;
pub const ALC5632_HID_CTRL_INDEX: u32 = 0x6A;
pub const ALC5632_HID_CTRL_DATA: u32 = 0x6C;
pub const ALC5632_EQ_CTRL: u32 = 0x6E;

/* undocumented */
pub const ALC5632_VENDOR_ID1: u32 = 0x7C;
pub const ALC5632_VENDOR_ID2: u32 = 0x7E;

pub const ALC5632_MAX_REGISTER: u32 = 0x7E;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
