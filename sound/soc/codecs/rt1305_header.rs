/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * RT1305.h  --  RT1305 ALSA SoC amplifier component driver
 *
 * Copyright 2018 Realtek Semiconductor Corp.
 * Author: Shuming Fan <shumingf@realtek.com>
 */

pub const RT1305_DEVICE_ID_NUM: u32 = 0x6251;

pub const RT1305_RESET: u32 = 0x00;
pub const RT1305_CLK_1: u32 = 0x04;
pub const RT1305_CLK_2: u32 = 0x05;
pub const RT1305_CLK_3: u32 = 0x06;
pub const RT1305_DFLL_REG: u32 = 0x07;
pub const RT1305_CAL_EFUSE_CLOCK: u32 = 0x08;
pub const RT1305_PLL0_1: u32 = 0x0a;
pub const RT1305_PLL0_2: u32 = 0x0b;
pub const RT1305_PLL1_1: u32 = 0x0c;
pub const RT1305_PLL1_2: u32 = 0x0d;
pub const RT1305_MIXER_CTRL_1: u32 = 0x10;
pub const RT1305_MIXER_CTRL_2: u32 = 0x11;
pub const RT1305_DAC_SET_1: u32 = 0x12;
pub const RT1305_DAC_SET_2: u32 = 0x14;
pub const RT1305_ADC_SET_1: u32 = 0x16;
pub const RT1305_ADC_SET_2: u32 = 0x17;
pub const RT1305_ADC_SET_3: u32 = 0x18;
pub const RT1305_PATH_SET: u32 = 0x20;
pub const RT1305_SPDIF_IN_SET_1: u32 = 0x22;
pub const RT1305_SPDIF_IN_SET_2: u32 = 0x24;
pub const RT1305_SPDIF_IN_SET_3: u32 = 0x26;
pub const RT1305_SPDIF_OUT_SET_1: u32 = 0x28;
pub const RT1305_SPDIF_OUT_SET_2: u32 = 0x2a;
pub const RT1305_SPDIF_OUT_SET_3: u32 = 0x2b;
pub const RT1305_I2S_SET_1: u32 = 0x2d;
pub const RT1305_I2S_SET_2: u32 = 0x2e;
pub const RT1305_PBTL_MONO_MODE_SRC: u32 = 0x2f;
pub const RT1305_MANUALLY_I2C_DEVICE: u32 = 0x32;
pub const RT1305_POWER_STATUS: u32 = 0x39;
pub const RT1305_POWER_CTRL_1: u32 = 0x3a;
pub const RT1305_POWER_CTRL_2: u32 = 0x3b;
pub const RT1305_POWER_CTRL_3: u32 = 0x3c;
pub const RT1305_POWER_CTRL_4: u32 = 0x3d;
pub const RT1305_POWER_CTRL_5: u32 = 0x3e;
pub const RT1305_CLOCK_DETECT: u32 = 0x3f;
pub const RT1305_BIQUAD_SET_1: u32 = 0x40;
pub const RT1305_BIQUAD_SET_2: u32 = 0x42;
pub const RT1305_ADJUSTED_HPF_1: u32 = 0x46;
pub const RT1305_ADJUSTED_HPF_2: u32 = 0x47;
pub const RT1305_EQ_SET_1: u32 = 0x4b;
pub const RT1305_EQ_SET_2: u32 = 0x4c;
pub const RT1305_SPK_TEMP_PROTECTION_0: u32 = 0x4f;
pub const RT1305_SPK_TEMP_PROTECTION_1: u32 = 0x50;
pub const RT1305_SPK_TEMP_PROTECTION_2: u32 = 0x51;
pub const RT1305_SPK_TEMP_PROTECTION_3: u32 = 0x52;
pub const RT1305_SPK_DC_DETECT_1: u32 = 0x53;
pub const RT1305_SPK_DC_DETECT_2: u32 = 0x54;
pub const RT1305_LOUDNESS: u32 = 0x58;
pub const RT1305_THERMAL_FOLD_BACK_1: u32 = 0x5e;
pub const RT1305_THERMAL_FOLD_BACK_2: u32 = 0x5f;
pub const RT1305_SILENCE_DETECT: u32 = 0x60;
pub const RT1305_ALC_DRC_1: u32 = 0x62;
pub const RT1305_ALC_DRC_2: u32 = 0x63;
pub const RT1305_ALC_DRC_3: u32 = 0x64;
pub const RT1305_ALC_DRC_4: u32 = 0x65;
pub const RT1305_PRIV_INDEX: u32 = 0x6a;
pub const RT1305_PRIV_DATA: u32 = 0x6c;
pub const RT1305_SPK_EXCURSION_LIMITER_7: u32 = 0x76;
pub const RT1305_VERSION_ID: u32 = 0x7a;
pub const RT1305_VENDOR_ID: u32 = 0x7c;
pub const RT1305_DEVICE_ID: u32 = 0x7e;
pub const RT1305_EFUSE_1: u32 = 0x80;
pub const RT1305_EFUSE_2: u32 = 0x81;
pub const RT1305_EFUSE_3: u32 = 0x82;
pub const RT1305_DC_CALIB_1: u32 = 0x90;
pub const RT1305_DC_CALIB_2: u32 = 0x91;
pub const RT1305_DC_CALIB_3: u32 = 0x92;
pub const RT1305_DAC_OFFSET_1: u32 = 0x93;
pub const RT1305_DAC_OFFSET_2: u32 = 0x94;
pub const RT1305_DAC_OFFSET_3: u32 = 0x95;
pub const RT1305_DAC_OFFSET_4: u32 = 0x96;
pub const RT1305_DAC_OFFSET_5: u32 = 0x97;
pub const RT1305_DAC_OFFSET_6: u32 = 0x98;
pub const RT1305_DAC_OFFSET_7: u32 = 0x99;
pub const RT1305_DAC_OFFSET_8: u32 = 0x9a;
pub const RT1305_DAC_OFFSET_9: u32 = 0x9b;
pub const RT1305_DAC_OFFSET_10: u32 = 0x9c;
pub const RT1305_DAC_OFFSET_11: u32 = 0x9d;
pub const RT1305_DAC_OFFSET_12: u32 = 0x9e;
pub const RT1305_DAC_OFFSET_13: u32 = 0x9f;
pub const RT1305_DAC_OFFSET_14: u32 = 0xa0;
pub const RT1305_TRIM_1: u32 = 0xb0;
pub const RT1305_TRIM_2: u32 = 0xb1;
pub const RT1305_TUNE_INTERNAL_OSC: u32 = 0xb2;
pub const RT1305_BIQUAD1_H0_L_28_16: u32 = 0xc0;
pub const RT1305_BIQUAD3_A2_R_15_0: u32 = 0xfb;
pub const RT1305_MAX_REG: u32 = 0xff;

/* CLOCK-1 (0x04) */
pub const RT1305_SEL_PLL_SRC_2_MASK: u32 = 0x1 << 15;
pub const RT1305_SEL_PLL_SRC_2_SFT: u32 = 15;
pub const RT1305_SEL_PLL_SRC_2_MCLK: u32 = 0x0 << 15;
pub const RT1305_SEL_PLL_SRC_2_RCCLK: u32 = 0x1 << 15;
pub const RT1305_DIV_PLL_SRC_2_MASK: u32 = 0x3 << 13;
pub const RT1305_DIV_PLL_SRC_2_SFT: u32 = 13;
pub const RT1305_SEL_PLL_SRC_1_MASK: u32 = 0x3 << 10;
pub const RT1305_SEL_PLL_SRC_1_SFT: u32 = 10;
pub const RT1305_SEL_PLL_SRC_1_PLL2: u32 = 0x0 << 10;
pub const RT1305_SEL_PLL_SRC_1_BCLK: u32 = 0x1 << 10;
pub const RT1305_SEL_PLL_SRC_1_DFLL: u32 = 0x2 << 10;
pub const RT1305_SEL_FS_SYS_PRE_MASK: u32 = 0x3 << 8;
pub const RT1305_SEL_FS_SYS_PRE_SFT: u32 = 8;
pub const RT1305_SEL_FS_SYS_PRE_MCLK: u32 = 0x0 << 8;
pub const RT1305_SEL_FS_SYS_PRE_PLL: u32 = 0x1 << 8;
pub const RT1305_SEL_FS_SYS_PRE_RCCLK: u32 = 0x2 << 8;
pub const RT1305_DIV_FS_SYS_MASK: u32 = 0x7 << 4;
pub const RT1305_DIV_FS_SYS_SFT: u32 = 4;

/* PLL1M/N/K Code-1 (0x0c) */
pub const RT1305_PLL_1_M_SFT: u32 = 12;
pub const RT1305_PLL_1_M_BYPASS_MASK: u32 = 0x1 << 11;
pub const RT1305_PLL_1_M_BYPASS_SFT: u32 = 11;
pub const RT1305_PLL_1_M_BYPASS: u32 = 0x1 << 11;
pub const RT1305_PLL_1_N_MASK: u32 = 0x1ff << 0;

/* DAC Setting (0x14) */
pub const RT1305_DVOL_MUTE_L_EN_SFT: u32 = 15;
pub const RT1305_DVOL_MUTE_R_EN_SFT: u32 = 14;

/* I2S Setting-1 (0x2d) */
pub const RT1305_SEL_I2S_OUT_MODE_MASK: u32 = 0x1 << 15;
pub const RT1305_SEL_I2S_OUT_MODE_SFT: u32 = 15;
pub const RT1305_SEL_I2S_OUT_MODE_S: u32 = 0x0 << 15;
pub const RT1305_SEL_I2S_OUT_MODE_M: u32 = 0x1 << 15;

/* I2S Setting-2 (0x2e) */
pub const RT1305_I2S_DF_SEL_MASK: u32 = 0x3 << 12;
pub const RT1305_I2S_DF_SEL_SFT: u32 = 12;
pub const RT1305_I2S_DF_SEL_I2S: u32 = 0x0 << 12;
pub const RT1305_I2S_DF_SEL_LEFT: u32 = 0x1 << 12;
pub const RT1305_I2S_DF_SEL_PCM_A: u32 = 0x2 << 12;
pub const RT1305_I2S_DF_SEL_PCM_B: u32 = 0x3 << 12;
pub const RT1305_I2S_DL_SEL_MASK: u32 = 0x3 << 10;
pub const RT1305_I2S_DL_SEL_SFT: u32 = 10;
pub const RT1305_I2S_DL_SEL_16B: u32 = 0x0 << 10;
pub const RT1305_I2S_DL_SEL_20B: u32 = 0x1 << 10;
pub const RT1305_I2S_DL_SEL_24B: u32 = 0x2 << 10;
pub const RT1305_I2S_DL_SEL_8B: u32 = 0x3 << 10;
pub const RT1305_I2S_BCLK_MASK: u32 = 0x1 << 9;
pub const RT1305_I2S_BCLK_SFT: u32 = 9;
pub const RT1305_I2S_BCLK_NORMAL: u32 = 0x0 << 9;
pub const RT1305_I2S_BCLK_INV: u32 = 0x1 << 9;

/* Power Control-1 (0x3a) */
pub const RT1305_POW_PDB_JD_MASK: u32 = 0x1 << 12;
pub const RT1305_POW_PDB_JD: u32 = 0x1 << 12;
pub const RT1305_POW_PDB_JD_BIT: u32 = 12;
pub const RT1305_POW_PLL0_EN: u32 = 0x1 << 11;
pub const RT1305_POW_PLL0_EN_BIT: u32 = 11;
pub const RT1305_POW_PLL1_EN: u32 = 0x1 << 10;
pub const RT1305_POW_PLL1_EN_BIT: u32 = 10;
pub const RT1305_POW_PDB_JD_POLARITY: u32 = 0x1 << 9;
pub const RT1305_POW_PDB_JD_POLARITY_BIT: u32 = 9;
pub const RT1305_POW_MBIAS_LV: u32 = 0x1 << 8;
pub const RT1305_POW_MBIAS_LV_BIT: u32 = 8;
pub const RT1305_POW_BG_MBIAS_LV: u32 = 0x1 << 7;
pub const RT1305_POW_BG_MBIAS_LV_BIT: u32 = 7;
pub const RT1305_POW_LDO2: u32 = 0x1 << 6;
pub const RT1305_POW_LDO2_BIT: u32 = 6;
pub const RT1305_POW_BG2: u32 = 0x1 << 5;
pub const RT1305_POW_BG2_BIT: u32 = 5;
pub const RT1305_POW_LDO2_IB2: u32 = 0x1 << 4;
pub const RT1305_POW_LDO2_IB2_BIT: u32 = 4;
pub const RT1305_POW_VREF: u32 = 0x1 << 3;
pub const RT1305_POW_VREF_BIT: u32 = 3;
pub const RT1305_POW_VREF1: u32 = 0x1 << 2;
pub const RT1305_POW_VREF1_BIT: u32 = 2;
pub const RT1305_POW_VREF2: u32 = 0x1 << 1;
pub const RT1305_POW_VREF2_BIT: u32 = 1;

/* Power Control-2 (0x3b) */
pub const RT1305_POW_DISC_VREF: u32 = 1 << 15;
pub const RT1305_POW_DISC_VREF_BIT: u32 = 15;
pub const RT1305_POW_FASTB_VREF: u32 = 1 << 14;
pub const RT1305_POW_FASTB_VREF_BIT: u32 = 14;
pub const RT1305_POW_ULTRA_FAST_VREF: u32 = 1 << 13;
pub const RT1305_POW_ULTRA_FAST_VREF_BIT: u32 = 13;
pub const RT1305_POW_CKXEN_DAC: u32 = 1 << 12;
pub const RT1305_POW_CKXEN_DAC_BIT: u32 = 12;
pub const RT1305_POW_EN_CKGEN_DAC: u32 = 1 << 11;
pub const RT1305_POW_EN_CKGEN_DAC_BIT: u32 = 11;
pub const RT1305_POW_DAC1_L: u32 = 1 << 10;
pub const RT1305_POW_DAC1_L_BIT: u32 = 10;
pub const RT1305_POW_DAC1_R: u32 = 1 << 9;
pub const RT1305_POW_DAC1_R_BIT: u32 = 9;
pub const RT1305_POW_CLAMP: u32 = 1 << 8;
pub const RT1305_POW_CLAMP_BIT: u32 = 8;
pub const RT1305_POW_BUFL: u32 = 1 << 7;
pub const RT1305_POW_BUFL_BIT: u32 = 7;
pub const RT1305_POW_BUFR: u32 = 1 << 6;
pub const RT1305_POW_BUFR_BIT: u32 = 6;
pub const RT1305_POW_EN_CKGEN_ADC: u32 = 1 << 5;
pub const RT1305_POW_EN_CKGEN_ADC_BIT: u32 = 5;
pub const RT1305_POW_ADC3_L: u32 = 1 << 4;
pub const RT1305_POW_ADC3_L_BIT: u32 = 4;
pub const RT1305_POW_ADC3_R: u32 = 1 << 3;
pub const RT1305_POW_ADC3_R_BIT: u32 = 3;
pub const RT1305_POW_TRIOSC: u32 = 1 << 2;
pub const RT1305_POW_TRIOSC_BIT: u32 = 2;
pub const RT1305_POR_AVDD1: u32 = 1 << 1;
pub const RT1305_POR_AVDD1_BIT: u32 = 1;
pub const RT1305_POR_AVDD2: u32 = 1 << 0;
pub const RT1305_POR_AVDD2_BIT: u32 = 0;

/* Power Control-3 (0x3c) */
pub const RT1305_POW_VSENSE_RCH: u32 = 1 << 15;
pub const RT1305_POW_VSENSE_RCH_BIT: u32 = 15;
pub const RT1305_POW_VSENSE_LCH: u32 = 1 << 14;
pub const RT1305_POW_VSENSE_LCH_BIT: u32 = 14;
pub const RT1305_POW_ISENSE_RCH: u32 = 1 << 13;
pub const RT1305_POW_ISENSE_RCH_BIT: u32 = 13;
pub const RT1305_POW_ISENSE_LCH: u32 = 1 << 12;
pub const RT1305_POW_ISENSE_LCH_BIT: u32 = 12;
pub const RT1305_POW_POR_AVDD1: u32 = 1 << 11;
pub const RT1305_POW_POR_AVDD1_BIT: u32 = 11;
pub const RT1305_POW_POR_AVDD2: u32 = 1 << 10;
pub const RT1305_POW_POR_AVDD2_BIT: u32 = 10;
pub const RT1305_EN_K_HV: u32 = 1 << 9;
pub const RT1305_EN_K_HV_BIT: u32 = 9;
pub const RT1305_EN_PRE_K_HV: u32 = 1 << 8;
pub const RT1305_EN_PRE_K_HV_BIT: u32 = 8;
pub const RT1305_EN_EFUSE_1P8V: u32 = 1 << 7;
pub const RT1305_EN_EFUSE_1P8V_BIT: u32 = 7;
pub const RT1305_EN_EFUSE_5V: u32 = 1 << 6;
pub const RT1305_EN_EFUSE_5V_BIT: u32 = 6;
pub const RT1305_EN_VCM_6172: u32 = 1 << 5;
pub const RT1305_EN_VCM_6172_BIT: u32 = 5;
pub const RT1305_POR_EFUSE: u32 = 1 << 4;
pub const RT1305_POR_EFUSE_BIT: u32 = 4;

/* Clock Detect (0x3f) */
pub const RT1305_SEL_CLK_DET_SRC_MASK: u32 = 0x1 << 12;
pub const RT1305_SEL_CLK_DET_SRC_SFT: u32 = 12;
pub const RT1305_SEL_CLK_DET_SRC_MCLK: u32 = 0x0 << 12;
pub const RT1305_SEL_CLK_DET_SRC_BCLK: u32 = 0x1 << 12;

/* System Clock Source */
pub const RT1305_FS_SYS_PRE_S_MCLK: u32 = 0;
pub const RT1305_FS_SYS_PRE_S_PLL1: u32 = 1;
pub const RT1305_FS_SYS_PRE_S_RCCLK: u32 = 2; /* 98.304M Hz */

/* PLL Source 1/2 */
pub const RT1305_PLL1_S_BCLK: u32 = 0;
pub const RT1305_PLL2_S_MCLK: u32 = 1;
pub const RT1305_PLL2_S_RCCLK: u32 = 2; /* 98.304M Hz */

pub const RT1305_AIF1: u32 = 0;
pub const RT1305_AIFS: u32 = 1;

pub const R0_UPPER: u32 = 0x2E8BA2; /* 5.5 ohm */
pub const R0_LOWER: u32 = 0x666666; /* 2.5 ohm */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
