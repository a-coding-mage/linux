/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Functions and registers to access AC100 codec / RTC combo IC.
 *
 * Copyright (C) 2016 Chen-Yu Tsai
 *
 * Chen-Yu Tsai <wens@csie.org>
 */

/* Dependency supplied externally: linux device and regmap types. */
#[repr(C)]
pub struct ac100_dev {
    pub dev: *mut device,
    pub regmap: *mut regmap,
}

/* Audio codec related registers */
pub const AC100_CHIP_AUDIO_RST: u32 = 0x00;
pub const AC100_PLL_CTRL1: u32 = 0x01;
pub const AC100_PLL_CTRL2: u32 = 0x02;
pub const AC100_SYSCLK_CTRL: u32 = 0x03;
pub const AC100_MOD_CLK_ENA: u32 = 0x04;
pub const AC100_MOD_RST_CTRL: u32 = 0x05;
pub const AC100_I2S_SR_CTRL: u32 = 0x06;

/* I2S1 interface */
pub const AC100_I2S1_CLK_CTRL: u32 = 0x10;
pub const AC100_I2S1_SND_OUT_CTRL: u32 = 0x11;
pub const AC100_I2S1_SND_IN_CTRL: u32 = 0x12;
pub const AC100_I2S1_MXR_SRC: u32 = 0x13;
pub const AC100_I2S1_VOL_CTRL1: u32 = 0x14;
pub const AC100_I2S1_VOL_CTRL2: u32 = 0x15;
pub const AC100_I2S1_VOL_CTRL3: u32 = 0x16;
pub const AC100_I2S1_VOL_CTRL4: u32 = 0x17;
pub const AC100_I2S1_MXR_GAIN: u32 = 0x18;

/* I2S2 interface */
pub const AC100_I2S2_CLK_CTRL: u32 = 0x20;
pub const AC100_I2S2_SND_OUT_CTRL: u32 = 0x21;
pub const AC100_I2S2_SND_IN_CTRL: u32 = 0x22;
pub const AC100_I2S2_MXR_SRC: u32 = 0x23;
pub const AC100_I2S2_VOL_CTRL1: u32 = 0x24;
pub const AC100_I2S2_VOL_CTRL2: u32 = 0x25;
pub const AC100_I2S2_VOL_CTRL3: u32 = 0x26;
pub const AC100_I2S2_VOL_CTRL4: u32 = 0x27;
pub const AC100_I2S2_MXR_GAIN: u32 = 0x28;

/* I2S3 interface */
pub const AC100_I2S3_CLK_CTRL: u32 = 0x30;
pub const AC100_I2S3_SND_OUT_CTRL: u32 = 0x31;
pub const AC100_I2S3_SND_IN_CTRL: u32 = 0x32;
pub const AC100_I2S3_SIG_PATH_CTRL: u32 = 0x33;

/* ADC digital controls */
pub const AC100_ADC_DIG_CTRL: u32 = 0x40;
pub const AC100_ADC_VOL_CTRL: u32 = 0x41;

/* HMIC plug sensing / key detection */
pub const AC100_HMIC_CTRL1: u32 = 0x44;
pub const AC100_HMIC_CTRL2: u32 = 0x45;
pub const AC100_HMIC_STATUS: u32 = 0x46;

/* DAC digital controls */
pub const AC100_DAC_DIG_CTRL: u32 = 0x48;
pub const AC100_DAC_VOL_CTRL: u32 = 0x49;
pub const AC100_DAC_MXR_SRC: u32 = 0x4c;
pub const AC100_DAC_MXR_GAIN: u32 = 0x4d;

/* Analog controls */
pub const AC100_ADC_APC_CTRL: u32 = 0x50;
pub const AC100_ADC_SRC: u32 = 0x51;
pub const AC100_ADC_SRC_BST_CTRL: u32 = 0x52;
pub const AC100_OUT_MXR_DAC_A_CTRL: u32 = 0x53;
pub const AC100_OUT_MXR_SRC: u32 = 0x54;
pub const AC100_OUT_MXR_SRC_BST: u32 = 0x55;
pub const AC100_HPOUT_CTRL: u32 = 0x56;
pub const AC100_ERPOUT_CTRL: u32 = 0x57;
pub const AC100_SPKOUT_CTRL: u32 = 0x58;
pub const AC100_LINEOUT_CTRL: u32 = 0x59;

/* ADC digital audio processing (high pass filter & auto gain control */
pub const AC100_ADC_DAP_L_STA: u32 = 0x80;
pub const AC100_ADC_DAP_R_STA: u32 = 0x81;
pub const AC100_ADC_DAP_L_CTRL: u32 = 0x82;
pub const AC100_ADC_DAP_R_CTRL: u32 = 0x83;
pub const AC100_ADC_DAP_L_T_L: u32 = 0x84; /* Left Target Level */
pub const AC100_ADC_DAP_R_T_L: u32 = 0x85; /* Right Target Level */
pub const AC100_ADC_DAP_L_H_A_C: u32 = 0x86; /* Left High Avg. Coef */
pub const AC100_ADC_DAP_L_L_A_C: u32 = 0x87; /* Left Low Avg. Coef */
pub const AC100_ADC_DAP_R_H_A_C: u32 = 0x88; /* Right High Avg. Coef */
pub const AC100_ADC_DAP_R_L_A_C: u32 = 0x89; /* Right Low Avg. Coef */
pub const AC100_ADC_DAP_L_D_T: u32 = 0x8a; /* Left Decay Time */
pub const AC100_ADC_DAP_L_A_T: u32 = 0x8b; /* Left Attack Time */
pub const AC100_ADC_DAP_R_D_T: u32 = 0x8c; /* Right Decay Time */
pub const AC100_ADC_DAP_R_A_T: u32 = 0x8d; /* Right Attack Time */
pub const AC100_ADC_DAP_N_TH: u32 = 0x8e; /* Noise Threshold */
pub const AC100_ADC_DAP_L_H_N_A_C: u32 = 0x8f; /* Left High Noise Avg. Coef */
pub const AC100_ADC_DAP_L_L_N_A_C: u32 = 0x90; /* Left Low Noise Avg. Coef */
pub const AC100_ADC_DAP_R_H_N_A_C: u32 = 0x91; /* Right High Noise Avg. Coef */
pub const AC100_ADC_DAP_R_L_N_A_C: u32 = 0x92; /* Right Low Noise Avg. Coef */
pub const AC100_ADC_DAP_H_HPF_C: u32 = 0x93; /* High High-Pass-Filter Coef */
pub const AC100_ADC_DAP_L_HPF_C: u32 = 0x94; /* Low High-Pass-Filter Coef */
pub const AC100_ADC_DAP_OPT: u32 = 0x95; /* AGC Optimum */

/* DAC digital audio processing (high pass filter & dynamic range control) */
pub const AC100_DAC_DAP_CTRL: u32 = 0xa0;
pub const AC100_DAC_DAP_H_HPF_C: u32 = 0xa1; /* High High-Pass-Filter Coef */
pub const AC100_DAC_DAP_L_HPF_C: u32 = 0xa2; /* Low High-Pass-Filter Coef */
pub const AC100_DAC_DAP_L_H_E_A_C: u32 = 0xa3; /* Left High Energy Avg Coef */
pub const AC100_DAC_DAP_L_L_E_A_C: u32 = 0xa4; /* Left Low Energy Avg Coef */
pub const AC100_DAC_DAP_R_H_E_A_C: u32 = 0xa5; /* Right High Energy Avg Coef */
pub const AC100_DAC_DAP_R_L_E_A_C: u32 = 0xa6; /* Right Low Energy Avg Coef */
pub const AC100_DAC_DAP_H_G_D_T_C: u32 = 0xa7; /* High Gain Delay Time Coef */
pub const AC100_DAC_DAP_L_G_D_T_C: u32 = 0xa8; /* Low Gain Delay Time Coef */
pub const AC100_DAC_DAP_H_G_A_T_C: u32 = 0xa9; /* High Gain Attack Time Coef */
pub const AC100_DAC_DAP_L_G_A_T_C: u32 = 0xaa; /* Low Gain Attack Time Coef */
pub const AC100_DAC_DAP_H_E_TH: u32 = 0xab; /* High Energy Threshold */
pub const AC100_DAC_DAP_L_E_TH: u32 = 0xac; /* Low Energy Threshold */
pub const AC100_DAC_DAP_H_G_K: u32 = 0xad; /* High Gain K parameter */
pub const AC100_DAC_DAP_L_G_K: u32 = 0xae; /* Low Gain K parameter */
pub const AC100_DAC_DAP_H_G_OFF: u32 = 0xaf; /* High Gain offset */
pub const AC100_DAC_DAP_L_G_OFF: u32 = 0xb0; /* Low Gain offset */
pub const AC100_DAC_DAP_OPT: u32 = 0xb1; /* DRC optimum */

/* Digital audio processing enable */
pub const AC100_ADC_DAP_ENA: u32 = 0xb4;
pub const AC100_DAC_DAP_ENA: u32 = 0xb5;

/* SRC control */
pub const AC100_SRC1_CTRL1: u32 = 0xb8;
pub const AC100_SRC1_CTRL2: u32 = 0xb9;
pub const AC100_SRC1_CTRL3: u32 = 0xba;
pub const AC100_SRC1_CTRL4: u32 = 0xbb;
pub const AC100_SRC2_CTRL1: u32 = 0xbc;
pub const AC100_SRC2_CTRL2: u32 = 0xbd;
pub const AC100_SRC2_CTRL3: u32 = 0xbe;
pub const AC100_SRC2_CTRL4: u32 = 0xbf;

/* RTC clk control */
pub const AC100_CLK32K_ANALOG_CTRL: u32 = 0xc0;
pub const AC100_CLKOUT_CTRL1: u32 = 0xc1;
pub const AC100_CLKOUT_CTRL2: u32 = 0xc2;
pub const AC100_CLKOUT_CTRL3: u32 = 0xc3;

/* RTC module */
pub const AC100_RTC_RST: u32 = 0xc6;
pub const AC100_RTC_CTRL: u32 = 0xc7;
pub const AC100_RTC_SEC: u32 = 0xc8; /* second */
pub const AC100_RTC_MIN: u32 = 0xc9; /* minute */
pub const AC100_RTC_HOU: u32 = 0xca; /* hour */
pub const AC100_RTC_WEE: u32 = 0xcb; /* weekday */
pub const AC100_RTC_DAY: u32 = 0xcc; /* day */
pub const AC100_RTC_MON: u32 = 0xcd; /* month */
pub const AC100_RTC_YEA: u32 = 0xce; /* year */
pub const AC100_RTC_UPD: u32 = 0xcf; /* update trigger */

/* RTC alarm */
pub const AC100_ALM_INT_ENA: u32 = 0xd0;
pub const AC100_ALM_INT_STA: u32 = 0xd1;
pub const AC100_ALM_SEC: u32 = 0xd8;
pub const AC100_ALM_MIN: u32 = 0xd9;
pub const AC100_ALM_HOU: u32 = 0xda;
pub const AC100_ALM_WEE: u32 = 0xdb;
pub const AC100_ALM_DAY: u32 = 0xdc;
pub const AC100_ALM_MON: u32 = 0xdd;
pub const AC100_ALM_YEA: u32 = 0xde;
pub const AC100_ALM_UPD: u32 = 0xdf;

/* RTC general purpose register 0 ~ 15 */
#[inline]
pub const fn AC100_RTC_GP(x: u32) -> u32 {
    0xe0u32 + x
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
