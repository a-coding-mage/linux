/* SPDX-License-Identifier: GPL-2.0 */
/*
 * rt1016.h  --  RT1016 ALSA SoC audio amplifier driver
 *
 * Copyright 2020 Realtek Semiconductor Corp.
 * Author: Oder Chiou <oder_chiou@realtek.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

/* C header guard and include syntax omitted in Rust translation. */

pub const RT1016_DEVICE_ID_VAL: u32 = 0x6595;

pub const RT1016_RESET: u32 = 0x00;
pub const RT1016_PADS_CTRL_1: u32 = 0x01;
pub const RT1016_PADS_CTRL_2: u32 = 0x02;
pub const RT1016_I2C_CTRL: u32 = 0x03;
pub const RT1016_VOL_CTRL_1: u32 = 0x04;
pub const RT1016_VOL_CTRL_2: u32 = 0x05;
pub const RT1016_VOL_CTRL_3: u32 = 0x06;
pub const RT1016_ANA_CTRL_1: u32 = 0x07;
pub const RT1016_MUX_SEL: u32 = 0x08;
pub const RT1016_RX_I2S_CTRL: u32 = 0x09;
pub const RT1016_ANA_FLAG: u32 = 0x0a;
pub const RT1016_VERSION2_ID: u32 = 0x0c;
pub const RT1016_VERSION1_ID: u32 = 0x0d;
pub const RT1016_VENDER_ID: u32 = 0x0e;
pub const RT1016_DEVICE_ID: u32 = 0x0f;
pub const RT1016_ANA_CTRL_2: u32 = 0x11;
pub const RT1016_TEST_SIGNAL: u32 = 0x1c;
pub const RT1016_TEST_CTRL_1: u32 = 0x1d;
pub const RT1016_TEST_CTRL_2: u32 = 0x1e;
pub const RT1016_TEST_CTRL_3: u32 = 0x1f;
pub const RT1016_CLOCK_1: u32 = 0x20;
pub const RT1016_CLOCK_2: u32 = 0x21;
pub const RT1016_CLOCK_3: u32 = 0x22;
pub const RT1016_CLOCK_4: u32 = 0x23;
pub const RT1016_CLOCK_5: u32 = 0x24;
pub const RT1016_CLOCK_6: u32 = 0x25;
pub const RT1016_CLOCK_7: u32 = 0x26;
pub const RT1016_I2S_CTRL: u32 = 0x40;
pub const RT1016_DAC_CTRL_1: u32 = 0x60;
pub const RT1016_SC_CTRL_1: u32 = 0x80;
pub const RT1016_SC_CTRL_2: u32 = 0x81;
pub const RT1016_SC_CTRL_3: u32 = 0x82;
pub const RT1016_SC_CTRL_4: u32 = 0x83;
pub const RT1016_SIL_DET: u32 = 0xa0;
pub const RT1016_SYS_CLK: u32 = 0xc0;
pub const RT1016_BIAS_CUR: u32 = 0xc1;
pub const RT1016_DAC_CTRL_2: u32 = 0xc2;
pub const RT1016_LDO_CTRL: u32 = 0xc3;
pub const RT1016_CLASSD_1: u32 = 0xc4;
pub const RT1016_PLL1: u32 = 0xc5;
pub const RT1016_PLL2: u32 = 0xc6;
pub const RT1016_PLL3: u32 = 0xc7;
pub const RT1016_CLASSD_2: u32 = 0xc8;
pub const RT1016_CLASSD_OUT: u32 = 0xc9;
pub const RT1016_CLASSD_3: u32 = 0xca;
pub const RT1016_CLASSD_4: u32 = 0xcb;
pub const RT1016_CLASSD_5: u32 = 0xcc;
pub const RT1016_PWR_CTRL: u32 = 0xcf;

/* global definition */
pub const RT1016_L_VOL_MASK: u32 = 0xff << 8;
pub const RT1016_L_VOL_SFT: u32 = 8;
pub const RT1016_R_VOL_MASK: u32 = 0xff;
pub const RT1016_R_VOL_SFT: u32 = 0;

/* 0x04 */
pub const RT1016_DA_MUTE_L_SFT: u32 = 7;
pub const RT1016_DA_MUTE_R_SFT: u32 = 6;

/* 0x20 */
pub const RT1016_CLK_SYS_SEL_MASK: u32 = 0x1 << 15;
pub const RT1016_CLK_SYS_SEL_SFT: u32 = 15;
pub const RT1016_CLK_SYS_SEL_MCLK: u32 = 0x0 << 15;
pub const RT1016_CLK_SYS_SEL_PLL: u32 = 0x1 << 15;
pub const RT1016_PLL_SEL_MASK: u32 = 0x1 << 13;
pub const RT1016_PLL_SEL_SFT: u32 = 13;
pub const RT1016_PLL_SEL_MCLK: u32 = 0x0 << 13;
pub const RT1016_PLL_SEL_BCLK: u32 = 0x1 << 13;

/* 0x21 */
pub const RT1016_FS_PD_MASK: u32 = 0x7 << 13;
pub const RT1016_FS_PD_SFT: u32 = 13;
pub const RT1016_OSR_PD_MASK: u32 = 0x3 << 10;
pub const RT1016_OSR_PD_SFT: u32 = 10;

/* 0x22 */
pub const RT1016_PWR_DAC_FILTER: u32 = 0x1 << 11;
pub const RT1016_PWR_DAC_FILTER_BIT: u32 = 11;
pub const RT1016_PWR_DACMOD: u32 = 0x1 << 10;
pub const RT1016_PWR_DACMOD_BIT: u32 = 10;
pub const RT1016_PWR_CLK_FIFO: u32 = 0x1 << 9;
pub const RT1016_PWR_CLK_FIFO_BIT: u32 = 9;
pub const RT1016_PWR_CLK_PUREDC: u32 = 0x1 << 8;
pub const RT1016_PWR_CLK_PUREDC_BIT: u32 = 8;
pub const RT1016_PWR_SIL_DET: u32 = 0x1 << 7;
pub const RT1016_PWR_SIL_DET_BIT: u32 = 7;
pub const RT1016_PWR_RC_25M: u32 = 0x1 << 6;
pub const RT1016_PWR_RC_25M_BIT: u32 = 6;
pub const RT1016_PWR_PLL1: u32 = 0x1 << 5;
pub const RT1016_PWR_PLL1_BIT: u32 = 5;
pub const RT1016_PWR_ANA_CTRL: u32 = 0x1 << 4;
pub const RT1016_PWR_ANA_CTRL_BIT: u32 = 4;
pub const RT1016_PWR_CLK_SYS: u32 = 0x1 << 3;
pub const RT1016_PWR_CLK_SYS_BIT: u32 = 3;

/* 0x23 */
pub const RT1016_PWR_LRCK_DET: u32 = 0x1 << 15;
pub const RT1016_PWR_LRCK_DET_BIT: u32 = 15;
pub const RT1016_PWR_BCLK_DET: u32 = 0x1 << 11;
pub const RT1016_PWR_BCLK_DET_BIT: u32 = 11;

/* 0x40 */
pub const RT1016_I2S_BCLK_MS_MASK: u32 = 0x1 << 15;
pub const RT1016_I2S_BCLK_MS_SFT: u32 = 15;
pub const RT1016_I2S_BCLK_MS_32: u32 = 0x0 << 15;
pub const RT1016_I2S_BCLK_MS_64: u32 = 0x1 << 15;
pub const RT1016_I2S_BCLK_POL_MASK: u32 = 0x1 << 13;
pub const RT1016_I2S_BCLK_POL_SFT: u32 = 13;
pub const RT1016_I2S_BCLK_POL_NOR: u32 = 0x0 << 13;
pub const RT1016_I2S_BCLK_POL_INV: u32 = 0x1 << 13;
pub const RT1016_I2S_DATA_SWAP_MASK: u32 = 0x1 << 10;
pub const RT1016_I2S_DATA_SWAP_SFT: u32 = 10;
pub const RT1016_I2S_DL_MASK: u32 = 0x7 << 4;
pub const RT1016_I2S_DL_SFT: u32 = 4;
pub const RT1016_I2S_DL_16: u32 = 0x1 << 4;
pub const RT1016_I2S_DL_20: u32 = 0x2 << 4;
pub const RT1016_I2S_DL_24: u32 = 0x3 << 4;
pub const RT1016_I2S_DL_32: u32 = 0x4 << 4;
pub const RT1016_I2S_MS_MASK: u32 = 0x1 << 3;
pub const RT1016_I2S_MS_SFT: u32 = 3;
pub const RT1016_I2S_MS_M: u32 = 0x0 << 3;
pub const RT1016_I2S_MS_S: u32 = 0x1 << 3;
pub const RT1016_I2S_DF_MASK: u32 = 0x7 << 0;
pub const RT1016_I2S_DF_SFT: u32 = 0;
pub const RT1016_I2S_DF_I2S: u32 = 0x0;
pub const RT1016_I2S_DF_LEFT: u32 = 0x1;
pub const RT1016_I2S_DF_PCM_A: u32 = 0x2;
pub const RT1016_I2S_DF_PCM_B: u32 = 0x3;

/* 0xa0 */
pub const RT1016_SIL_DET_EN: u32 = 0x1 << 15;
pub const RT1016_SIL_DET_EN_BIT: u32 = 15;

/* 0xc2 */
pub const RT1016_CKGEN_DAC: u32 = 0x1 << 13;
pub const RT1016_CKGEN_DAC_BIT: u32 = 13;

/* 0xc4 */
pub const RT1016_VCM_SLOW: u32 = 0x1 << 6;
pub const RT1016_VCM_SLOW_BIT: u32 = 6;

/* 0xc5 */
pub const RT1016_PLL_M_MAX: u32 = 0xf;
pub const RT1016_PLL_M_MASK: u32 = RT1016_PLL_M_MAX << 12;
pub const RT1016_PLL_M_SFT: u32 = 12;
pub const RT1016_PLL_M_BP: u32 = 0x1 << 11;
pub const RT1016_PLL_M_BP_SFT: u32 = 11;
pub const RT1016_PLL_N_MAX: u32 = 0x1ff;
pub const RT1016_PLL_N_MASK: u32 = RT1016_PLL_N_MAX << 0;
pub const RT1016_PLL_N_SFT: u32 = 0;

/* 0xc6 */
pub const RT1016_PLL2_EN: u32 = 0x1 << 15;
pub const RT1016_PLL2_EN_BIT: u32 = 15;
pub const RT1016_PLL_K_BP: u32 = 0x1 << 5;
pub const RT1016_PLL_K_BP_SFT: u32 = 5;
pub const RT1016_PLL_K_MAX: u32 = 0x1f;
pub const RT1016_PLL_K_MASK: u32 = RT1016_PLL_K_MAX;
pub const RT1016_PLL_K_SFT: u32 = 0;

/* 0xcf */
pub const RT1016_PWR_BG_1_2: u32 = 0x1 << 12;
pub const RT1016_PWR_BG_1_2_BIT: u32 = 12;
pub const RT1016_PWR_MBIAS_BG: u32 = 0x1 << 11;
pub const RT1016_PWR_MBIAS_BG_BIT: u32 = 11;
pub const RT1016_PWR_PLL: u32 = 0x1 << 9;
pub const RT1016_PWR_PLL_BIT: u32 = 9;
pub const RT1016_PWR_BASIC: u32 = 0x1 << 8;
pub const RT1016_PWR_BASIC_BIT: u32 = 8;
pub const RT1016_PWR_CLSD: u32 = 0x1 << 7;
pub const RT1016_PWR_CLSD_BIT: u32 = 7;
pub const RT1016_PWR_25M: u32 = 0x1 << 6;
pub const RT1016_PWR_25M_BIT: u32 = 6;
pub const RT1016_PWR_DACL: u32 = 0x1 << 4;
pub const RT1016_PWR_DACL_BIT: u32 = 4;
pub const RT1016_PWR_DACR: u32 = 0x1 << 3;
pub const RT1016_PWR_DACR_BIT: u32 = 3;
pub const RT1016_PWR_LDO2: u32 = 0x1 << 2;
pub const RT1016_PWR_LDO2_BIT: u32 = 2;
pub const RT1016_PWR_VREF: u32 = 0x1 << 1;
pub const RT1016_PWR_VREF_BIT: u32 = 1;
pub const RT1016_PWR_MBIAS: u32 = 0x1 << 0;
pub const RT1016_PWR_MBIAS_BIT: u32 = 0;

/* System Clock Source */
pub const RT1016_SCLK_S_MCLK: i32 = 0;
pub const RT1016_SCLK_S_PLL: i32 = 1;

/* PLL1 Source */
pub const RT1016_PLL_S_MCLK: i32 = 0;
pub const RT1016_PLL_S_BCLK: i32 = 1;

pub const RT1016_AIF1: i32 = 0;
pub const RT1016_AIFS: i32 = 1;

#[repr(C)]
pub struct rt1016_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub sysclk: i32,
    pub sysclk_src: i32,
    pub lrck: i32,
    pub bclk: i32,
    pub master: i32,
    pub pll_src: i32,
    pub pll_in: i32,
    pub pll_out: i32,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
