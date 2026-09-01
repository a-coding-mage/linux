/* SPDX-License-Identifier: GPL-2.0 */
/*
 * rt1308.h  --  RT1308 ALSA SoC amplifier component driver
 *
 * Copyright 2019 Realtek Semiconductor Corp.
 * Author: Derek Fang <derek.fang@realtek.com>
 *
 */

pub const RT1308_DEVICE_ID_NUM: u32 = 0x10ec1300;

pub const RT1308_RESET: u32 = 0x00;
pub const RT1308_RESET_N: u32 = 0x01;
pub const RT1308_CLK_GATING: u32 = 0x02;
pub const RT1308_PLL_1: u32 = 0x03;
pub const RT1308_PLL_2: u32 = 0x04;
pub const RT1308_PLL_INT: u32 = 0x05;
pub const RT1308_CLK_1: u32 = 0x06;
pub const RT1308_DATA_PATH: u32 = 0x07;
pub const RT1308_CLK_2: u32 = 0x08;
pub const RT1308_SIL_DET: u32 = 0x09;
pub const RT1308_CLK_DET: u32 = 0x0a;
pub const RT1308_DC_DET: u32 = 0x0b;
pub const RT1308_DC_DET_THRES: u32 = 0x0c;
pub const RT1308_DAC_SET: u32 = 0x10;
pub const RT1308_SRC_SET: u32 = 0x11;
pub const RT1308_DAC_BUF: u32 = 0x12;
pub const RT1308_ADC_SET: u32 = 0x13;
pub const RT1308_ADC_SET_INT: u32 = 0x14;
pub const RT1308_I2S_SET_1: u32 = 0x15;
pub const RT1308_I2S_SET_2: u32 = 0x16;
pub const RT1308_I2C_I2S_SDW_SET: u32 = 0x17;
pub const RT1308_SDW_REG_RW: u32 = 0x18;
pub const RT1308_SDW_REG_RDATA: u32 = 0x19;
pub const RT1308_IV_SENSE: u32 = 0x1a;
pub const RT1308_I2S_TX_DAC_SET: u32 = 0x1b;
pub const RT1308_AD_FILTER_SET: u32 = 0x1c;
pub const RT1308_DC_CAL_1: u32 = 0x20;
pub const RT1308_DC_CAL_2: u32 = 0x21;
pub const RT1308_DC_CAL_L_OFFSET: u32 = 0x22;
pub const RT1308_DC_CAL_R_OFFSET: u32 = 0x23;
pub const RT1308_PVDD_OFFSET_CTL: u32 = 0x24;
pub const RT1308_PVDD_OFFSET_L: u32 = 0x25;
pub const RT1308_PVDD_OFFSET_R: u32 = 0x26;
pub const RT1308_PVDD_OFFSET_PBTL: u32 = 0x27;
pub const RT1308_PVDD_OFFSET_PVDD: u32 = 0x28;
pub const RT1308_CAL_OFFSET_DAC_PBTL: u32 = 0x29;
pub const RT1308_CAL_OFFSET_DAC_L: u32 = 0x2a;
pub const RT1308_CAL_OFFSET_DAC_R: u32 = 0x2b;
pub const RT1308_CAL_OFFSET_PWM_L: u32 = 0x2c;
pub const RT1308_CAL_OFFSET_PWM_R: u32 = 0x2d;
pub const RT1308_CAL_PWM_VOS_ADC_L: u32 = 0x2e;
pub const RT1308_CAL_PWM_VOS_ADC_R: u32 = 0x2f;
pub const RT1308_CLASS_D_SET_1: u32 = 0x30;
pub const RT1308_CLASS_D_SET_2: u32 = 0x31;
pub const RT1308_POWER: u32 = 0x32;
pub const RT1308_LDO: u32 = 0x33;
pub const RT1308_VREF: u32 = 0x34;
pub const RT1308_MBIAS: u32 = 0x35;
pub const RT1308_POWER_STATUS: u32 = 0x36;
pub const RT1308_POWER_INT: u32 = 0x37;
pub const RT1308_SINE_TONE_GEN_1: u32 = 0x50;
pub const RT1308_SINE_TONE_GEN_2: u32 = 0x51;
pub const RT1308_BQ_SET: u32 = 0x54;
pub const RT1308_BQ_PARA_UPDATE: u32 = 0x55;
pub const RT1308_BQ_PRE_VOL_L: u32 = 0x56;
pub const RT1308_BQ_PRE_VOL_R: u32 = 0x57;
pub const RT1308_BQ_POST_VOL_L: u32 = 0x58;
pub const RT1308_BQ_POST_VOL_R: u32 = 0x59;
pub const RT1308_BQ1_L_H0: u32 = 0x5b;
pub const RT1308_BQ1_L_B1: u32 = 0x5c;
pub const RT1308_BQ1_L_B2: u32 = 0x5d;
pub const RT1308_BQ1_L_A1: u32 = 0x5e;
pub const RT1308_BQ1_L_A2: u32 = 0x5f;
pub const RT1308_BQ1_R_H0: u32 = 0x60;
pub const RT1308_BQ1_R_B1: u32 = 0x61;
pub const RT1308_BQ1_R_B2: u32 = 0x62;
pub const RT1308_BQ1_R_A1: u32 = 0x63;
pub const RT1308_BQ1_R_A2: u32 = 0x64;
pub const RT1308_BQ2_L_H0: u32 = 0x65;
pub const RT1308_BQ2_L_B1: u32 = 0x66;
pub const RT1308_BQ2_L_B2: u32 = 0x67;
pub const RT1308_BQ2_L_A1: u32 = 0x68;
pub const RT1308_BQ2_L_A2: u32 = 0x69;
pub const RT1308_BQ2_R_H0: u32 = 0x6a;
pub const RT1308_BQ2_R_B1: u32 = 0x6b;
pub const RT1308_BQ2_R_B2: u32 = 0x6c;
pub const RT1308_BQ2_R_A1: u32 = 0x6d;
pub const RT1308_BQ2_R_A2: u32 = 0x6e;
pub const RT1308_VEN_DEV_ID: u32 = 0x70;
pub const RT1308_VERSION_ID: u32 = 0x71;
pub const RT1308_SPK_BOUND: u32 = 0x72;
pub const RT1308_BQ1_EQ_L_1: u32 = 0x73;
pub const RT1308_BQ1_EQ_L_2: u32 = 0x74;
pub const RT1308_BQ1_EQ_L_3: u32 = 0x75;
pub const RT1308_BQ1_EQ_R_1: u32 = 0x76;
pub const RT1308_BQ1_EQ_R_2: u32 = 0x77;
pub const RT1308_BQ1_EQ_R_3: u32 = 0x78;
pub const RT1308_BQ2_EQ_L_1: u32 = 0x79;
pub const RT1308_BQ2_EQ_L_2: u32 = 0x7a;
pub const RT1308_BQ2_EQ_L_3: u32 = 0x7b;
pub const RT1308_BQ2_EQ_R_1: u32 = 0x7c;
pub const RT1308_BQ2_EQ_R_2: u32 = 0x7d;
pub const RT1308_BQ2_EQ_R_3: u32 = 0x7e;
pub const RT1308_EFUSE_1: u32 = 0x7f;
pub const RT1308_EFUSE_2: u32 = 0x80;
pub const RT1308_EFUSE_PROG_PVDD_L: u32 = 0x81;
pub const RT1308_EFUSE_PROG_PVDD_R: u32 = 0x82;
pub const RT1308_EFUSE_PROG_R0_L: u32 = 0x83;
pub const RT1308_EFUSE_PROG_R0_R: u32 = 0x84;
pub const RT1308_EFUSE_PROG_DEV: u32 = 0x85;
pub const RT1308_EFUSE_READ_PVDD_L: u32 = 0x86;
pub const RT1308_EFUSE_READ_PVDD_R: u32 = 0x87;
pub const RT1308_EFUSE_READ_PVDD_PTBL: u32 = 0x88;
pub const RT1308_EFUSE_READ_DEV: u32 = 0x89;
pub const RT1308_EFUSE_READ_R0: u32 = 0x8a;
pub const RT1308_EFUSE_READ_ADC_L: u32 = 0x8b;
pub const RT1308_EFUSE_READ_ADC_R: u32 = 0x8c;
pub const RT1308_EFUSE_READ_ADC_PBTL: u32 = 0x8d;
pub const RT1308_EFUSE_RESERVE: u32 = 0x8e;
pub const RT1308_PADS_1: u32 = 0x90;
pub const RT1308_PADS_2: u32 = 0x91;
pub const RT1308_TEST_MODE: u32 = 0xa0;
pub const RT1308_TEST_1: u32 = 0xa1;
pub const RT1308_TEST_2: u32 = 0xa2;
pub const RT1308_TEST_3: u32 = 0xa3;
pub const RT1308_TEST_4: u32 = 0xa4;
pub const RT1308_EFUSE_DATA_0_MSB: u32 = 0xb0;
pub const RT1308_EFUSE_DATA_0_LSB: u32 = 0xb1;
pub const RT1308_EFUSE_DATA_1_MSB: u32 = 0xb2;
pub const RT1308_EFUSE_DATA_1_LSB: u32 = 0xb3;
pub const RT1308_EFUSE_DATA_2_MSB: u32 = 0xb4;
pub const RT1308_EFUSE_DATA_2_LSB: u32 = 0xb5;
pub const RT1308_EFUSE_DATA_3_MSB: u32 = 0xb6;
pub const RT1308_EFUSE_DATA_3_LSB: u32 = 0xb7;
pub const RT1308_EFUSE_DATA_TEST_MSB: u32 = 0xb8;
pub const RT1308_EFUSE_DATA_TEST_LSB: u32 = 0xb9;
pub const RT1308_EFUSE_STATUS_1: u32 = 0xba;
pub const RT1308_EFUSE_STATUS_2: u32 = 0xbb;
pub const RT1308_TCON_1: u32 = 0xc0;
pub const RT1308_TCON_2: u32 = 0xc1;
pub const RT1308_DUMMY_REG: u32 = 0xf0;
pub const RT1308_MAX_REG: u32 = 0xff;

/* PLL1 M/N/K Code-1 (0x03) */
pub const RT1308_PLL1_K_SFT: u32 = 24;
pub const RT1308_PLL1_K_MASK: u32 = 0x1f << 24;
pub const RT1308_PLL1_M_BYPASS_MASK: u32 = 0x1 << 23;
pub const RT1308_PLL1_M_BYPASS_SFT: u32 = 23;
pub const RT1308_PLL1_M_BYPASS: u32 = 0x1 << 23;
pub const RT1308_PLL1_M_MASK: u32 = 0x3f << 16;
pub const RT1308_PLL1_M_SFT: u32 = 16;
pub const RT1308_PLL1_N_MASK: u32 = 0x7f << 8;
pub const RT1308_PLL1_N_SFT: u32 = 8;

/* CLOCK-1 (0x06) */
pub const RT1308_DIV_FS_SYS_MASK: u32 = 0xf << 28;
pub const RT1308_DIV_FS_SYS_SFT: u32 = 28;
pub const RT1308_SEL_FS_SYS_MASK: u32 = 0x7 << 24;
pub const RT1308_SEL_FS_SYS_SFT: u32 = 24;
pub const RT1308_SEL_FS_SYS_SRC_MCLK: u32 = 0x0 << 24;
pub const RT1308_SEL_FS_SYS_SRC_BCLK: u32 = 0x1 << 24;
pub const RT1308_SEL_FS_SYS_SRC_PLL: u32 = 0x2 << 24;
pub const RT1308_SEL_FS_SYS_SRC_RCCLK: u32 = 0x4 << 24;

/* CLOCK-2 (0x08) */
pub const RT1308_DIV_PRE_PLL_MASK: u32 = 0xf << 28;
pub const RT1308_DIV_PRE_PLL_SFT: u32 = 28;
pub const RT1308_SEL_PLL_SRC_MASK: u32 = 0x7 << 24;
pub const RT1308_SEL_PLL_SRC_SFT: u32 = 24;
pub const RT1308_SEL_PLL_SRC_MCLK: u32 = 0x0 << 24;
pub const RT1308_SEL_PLL_SRC_BCLK: u32 = 0x1 << 24;
pub const RT1308_SEL_PLL_SRC_RCCLK: u32 = 0x4 << 24;

/* Clock Detect (0x0a) */
pub const RT1308_MCLK_DET_EN_MASK: u32 = 0x1 << 25;
pub const RT1308_MCLK_DET_EN_SFT: u32 = 25;
pub const RT1308_MCLK_DET_EN: u32 = 0x1 << 25;
pub const RT1308_BCLK_DET_EN_MASK: u32 = 0x1 << 24;
pub const RT1308_BCLK_DET_EN_SFT: u32 = 24;
pub const RT1308_BCLK_DET_EN: u32 = 0x1 << 24;

/* DAC Setting (0x10) */
pub const RT1308_DVOL_MUTE_R_EN_SFT: u32 = 7;
pub const RT1308_DVOL_MUTE_L_EN_SFT: u32 = 6;

/* I2S Setting-1 (0x15) */
pub const RT1308_I2S_DF_SEL_MASK: u32 = 0x3 << 12;
pub const RT1308_I2S_DF_SEL_SFT: u32 = 12;
pub const RT1308_I2S_DF_SEL_I2S: u32 = 0x0 << 12;
pub const RT1308_I2S_DF_SEL_LEFT: u32 = 0x1 << 12;
pub const RT1308_I2S_DF_SEL_PCM_A: u32 = 0x2 << 12;
pub const RT1308_I2S_DF_SEL_PCM_B: u32 = 0x3 << 12;
pub const RT1308_I2S_DL_RX_SEL_MASK: u32 = 0x7 << 4;
pub const RT1308_I2S_DL_RX_SEL_SFT: u32 = 4;
pub const RT1308_I2S_DL_RX_SEL_16B: u32 = 0x0 << 4;
pub const RT1308_I2S_DL_RX_SEL_20B: u32 = 0x1 << 4;
pub const RT1308_I2S_DL_RX_SEL_24B: u32 = 0x2 << 4;
pub const RT1308_I2S_DL_RX_SEL_32B: u32 = 0x3 << 4;
pub const RT1308_I2S_DL_RX_SEL_8B: u32 = 0x4 << 4;
pub const RT1308_I2S_DL_TX_SEL_MASK: u32 = 0x7 << 0;
pub const RT1308_I2S_DL_TX_SEL_SFT: u32 = 0;
pub const RT1308_I2S_DL_TX_SEL_16B: u32 = 0x0 << 0;
pub const RT1308_I2S_DL_TX_SEL_20B: u32 = 0x1 << 0;
pub const RT1308_I2S_DL_TX_SEL_24B: u32 = 0x2 << 0;
pub const RT1308_I2S_DL_TX_SEL_32B: u32 = 0x3 << 0;
pub const RT1308_I2S_DL_TX_SEL_8B: u32 = 0x4 << 0;

/* I2S Setting-2 (0x16) */
pub const RT1308_I2S_DL_SEL_MASK: u32 = 0x7 << 24;
pub const RT1308_I2S_DL_SEL_SFT: u32 = 24;
pub const RT1308_I2S_DL_SEL_16B: u32 = 0x0 << 24;
pub const RT1308_I2S_DL_SEL_20B: u32 = 0x1 << 24;
pub const RT1308_I2S_DL_SEL_24B: u32 = 0x2 << 24;
pub const RT1308_I2S_DL_SEL_32B: u32 = 0x3 << 24;
pub const RT1308_I2S_DL_SEL_8B: u32 = 0x4 << 24;
pub const RT1308_I2S_BCLK_MASK: u32 = 0x1 << 14;
pub const RT1308_I2S_BCLK_SFT: u32 = 14;
pub const RT1308_I2S_BCLK_NORMAL: u32 = 0x0 << 14;
pub const RT1308_I2S_BCLK_INV: u32 = 0x1 << 14;

/* Power Control-1 (0x32) */
pub const RT1308_POW_MBIAS20U: u32 = 0x1 << 31;
pub const RT1308_POW_MBIAS20U_BIT: u32 = 31;
pub const RT1308_POW_ALDO: u32 = 0x1 << 30;
pub const RT1308_POW_ALDO_BIT: u32 = 30;
pub const RT1308_POW_DBG: u32 = 0x1 << 29;
pub const RT1308_POW_DBG_BIT: u32 = 29;
pub const RT1308_POW_DACL: u32 = 0x1 << 28;
pub const RT1308_POW_DACL_BIT: u32 = 28;
pub const RT1308_POW_DAC1: u32 = 0x1 << 27;
pub const RT1308_POW_DAC1_BIT: u32 = 27;
pub const RT1308_POW_CLK25M: u32 = 0x1 << 26;
pub const RT1308_POW_CLK25M_BIT: u32 = 26;
pub const RT1308_POW_ADC_R: u32 = 0x1 << 25;
pub const RT1308_POW_ADC_R_BIT: u32 = 25;
pub const RT1308_POW_ADC_L: u32 = 0x1 << 24;
pub const RT1308_POW_ADC_L_BIT: u32 = 24;
pub const RT1308_POW_DLDO: u32 = 0x1 << 21;
pub const RT1308_POW_DLDO_BIT: u32 = 21;
pub const RT1308_POW_VREF: u32 = 0x1 << 20;
pub const RT1308_POW_VREF_BIT: u32 = 20;
pub const RT1308_POW_MIXER_R: u32 = 0x1 << 18;
pub const RT1308_POW_MIXER_R_BIT: u32 = 18;
pub const RT1308_POW_MIXER_L: u32 = 0x1 << 17;
pub const RT1308_POW_MIXER_L_BIT: u32 = 17;
pub const RT1308_POW_MBIAS4U: u32 = 0x1 << 16;
pub const RT1308_POW_MBIAS4U_BIT: u32 = 16;
pub const RT1308_POW_PLL2_LDO_EN: u32 = 0x1 << 12;
pub const RT1308_POW_PLL2_LDO_EN_BIT: u32 = 12;
pub const RT1308_POW_PLL2B_EN: u32 = 0x1 << 11;
pub const RT1308_POW_PLL2B_EN_BIT: u32 = 11;
pub const RT1308_POW_PLL2F_EN: u32 = 0x1 << 10;
pub const RT1308_POW_PLL2F_EN_BIT: u32 = 10;
pub const RT1308_POW_PLL2F2_EN: u32 = 0x1 << 9;
pub const RT1308_POW_PLL2F2_EN_BIT: u32 = 9;
pub const RT1308_POW_PLL2B2_EN: u32 = 0x1 << 8;
pub const RT1308_POW_PLL2B2_EN_BIT: u32 = 8;

/* Power Control-2 (0x36) */
pub const RT1308_POW_PDB_SRC_BIT: u32 = 0x1 << 27;
pub const RT1308_POW_PDB_MN_BIT: u32 = 0x1 << 25;
pub const RT1308_POW_PDB_REG_BIT: u32 = 0x1 << 24;

/* System Clock Source */
pub const RT1308_FS_SYS_S_MCLK: u32 = 0;
pub const RT1308_FS_SYS_S_BCLK: u32 = 1;
pub const RT1308_FS_SYS_S_PLL: u32 = 2;
pub const RT1308_FS_SYS_S_RCCLK: u32 = 3; /* 25.0 MHz */

/* PLL Source */
pub const RT1308_PLL_S_MCLK: u32 = 0;
pub const RT1308_PLL_S_BCLK: u32 = 1;
pub const RT1308_PLL_S_RCCLK: u32 = 2;

pub const RT1308_AIF1: u32 = 0;
pub const RT1308_AIFS: u32 = 1;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum rt1308_hw_ver {
    RT1308_VER_C = 2,
    RT1308_VER_D = 3,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
