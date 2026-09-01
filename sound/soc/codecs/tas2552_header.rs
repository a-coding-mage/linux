/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * tas2552.h - ALSA SoC Texas Instruments TAS2552 Mono Audio Amplifier
 *
 * Copyright (C) 2014 Texas Instruments Incorporated -  https://www.ti.com
 *
 * Author: Dan Murphy <dmurphy@ti.com>
 */

/* Register Address Map */
pub const TAS2552_DEVICE_STATUS: u32 = 0x00;
pub const TAS2552_CFG_1: u32 = 0x01;
pub const TAS2552_CFG_2: u32 = 0x02;
pub const TAS2552_CFG_3: u32 = 0x03;
pub const TAS2552_DOUT: u32 = 0x04;
pub const TAS2552_SER_CTRL_1: u32 = 0x05;
pub const TAS2552_SER_CTRL_2: u32 = 0x06;
pub const TAS2552_OUTPUT_DATA: u32 = 0x07;
pub const TAS2552_PLL_CTRL_1: u32 = 0x08;
pub const TAS2552_PLL_CTRL_2: u32 = 0x09;
pub const TAS2552_PLL_CTRL_3: u32 = 0x0a;
pub const TAS2552_BTIP: u32 = 0x0b;
pub const TAS2552_BTS_CTRL: u32 = 0x0c;
pub const TAS2552_RESERVED_0D: u32 = 0x0d;
pub const TAS2552_LIMIT_RATE_HYS: u32 = 0x0e;
pub const TAS2552_LIMIT_RELEASE: u32 = 0x0f;
pub const TAS2552_LIMIT_INT_COUNT: u32 = 0x10;
pub const TAS2552_PDM_CFG: u32 = 0x11;
pub const TAS2552_PGA_GAIN: u32 = 0x12;
pub const TAS2552_EDGE_RATE_CTRL: u32 = 0x13;
pub const TAS2552_BOOST_APT_CTRL: u32 = 0x14;
pub const TAS2552_VER_NUM: u32 = 0x16;
pub const TAS2552_VBAT_DATA: u32 = 0x19;
pub const TAS2552_MAX_REG: u32 = TAS2552_VBAT_DATA;

/* CFG1 Register Masks */
pub const TAS2552_DEV_RESET: u32 = 1 << 0;
pub const TAS2552_SWS: u32 = 1 << 1;
pub const TAS2552_MUTE: u32 = 1 << 2;
pub const TAS2552_PLL_SRC_MCLK: u32 = 0x0 << 4;
pub const TAS2552_PLL_SRC_BCLK: u32 = 0x1 << 4;
pub const TAS2552_PLL_SRC_IVCLKIN: u32 = 0x2 << 4;
pub const TAS2552_PLL_SRC_1_8_FIXED: u32 = 0x3 << 4;
pub const TAS2552_PLL_SRC_MASK: u32 = TAS2552_PLL_SRC_1_8_FIXED;

/* CFG2 Register Masks */
pub const TAS2552_CLASSD_EN: u32 = 1 << 7;
pub const TAS2552_BOOST_EN: u32 = 1 << 6;
pub const TAS2552_APT_EN: u32 = 1 << 5;
pub const TAS2552_PLL_ENABLE: u32 = 1 << 3;
pub const TAS2552_LIM_EN: u32 = 1 << 2;
pub const TAS2552_IVSENSE_EN: u32 = 1 << 1;

/* CFG3 Register Masks */
pub const TAS2552_WCLK_FREQ_8KHZ: u32 = 0x0 << 0;
pub const TAS2552_WCLK_FREQ_11_12KHZ: u32 = 0x1 << 0;
pub const TAS2552_WCLK_FREQ_16KHZ: u32 = 0x2 << 0;
pub const TAS2552_WCLK_FREQ_22_24KHZ: u32 = 0x3 << 0;
pub const TAS2552_WCLK_FREQ_32KHZ: u32 = 0x4 << 0;
pub const TAS2552_WCLK_FREQ_44_48KHZ: u32 = 0x5 << 0;
pub const TAS2552_WCLK_FREQ_88_96KHZ: u32 = 0x6 << 0;
pub const TAS2552_WCLK_FREQ_176_192KHZ: u32 = 0x7 << 0;
pub const TAS2552_WCLK_FREQ_MASK: u32 = TAS2552_WCLK_FREQ_176_192KHZ;
pub const TAS2552_DIN_SRC_SEL_MUTED: u32 = 0x0 << 3;
pub const TAS2552_DIN_SRC_SEL_LEFT: u32 = 0x1 << 3;
pub const TAS2552_DIN_SRC_SEL_RIGHT: u32 = 0x2 << 3;
pub const TAS2552_DIN_SRC_SEL_AVG_L_R: u32 = 0x3 << 3;
pub const TAS2552_PDM_IN_SEL: u32 = 1 << 5;
pub const TAS2552_I2S_OUT_SEL: u32 = 1 << 6;
pub const TAS2552_ANALOG_IN_SEL: u32 = 1 << 7;

/* DOUT Register Masks */
pub const TAS2552_SDOUT_TRISTATE: u32 = 1 << 2;

/* Serial Interface Control Register Masks */
pub const TAS2552_WORDLENGTH_16BIT: u32 = 0x0 << 0;
pub const TAS2552_WORDLENGTH_20BIT: u32 = 0x1 << 0;
pub const TAS2552_WORDLENGTH_24BIT: u32 = 0x2 << 0;
pub const TAS2552_WORDLENGTH_32BIT: u32 = 0x3 << 0;
pub const TAS2552_WORDLENGTH_MASK: u32 = TAS2552_WORDLENGTH_32BIT;
pub const TAS2552_DATAFORMAT_I2S: u32 = 0x0 << 2;
pub const TAS2552_DATAFORMAT_DSP: u32 = 0x1 << 2;
pub const TAS2552_DATAFORMAT_RIGHT_J: u32 = 0x2 << 2;
pub const TAS2552_DATAFORMAT_LEFT_J: u32 = 0x3 << 2;
pub const TAS2552_DATAFORMAT_MASK: u32 = TAS2552_DATAFORMAT_LEFT_J;
pub const TAS2552_CLKSPERFRAME_32: u32 = 0x0 << 4;
pub const TAS2552_CLKSPERFRAME_64: u32 = 0x1 << 4;
pub const TAS2552_CLKSPERFRAME_128: u32 = 0x2 << 4;
pub const TAS2552_CLKSPERFRAME_256: u32 = 0x3 << 4;
pub const TAS2552_CLKSPERFRAME_MASK: u32 = TAS2552_CLKSPERFRAME_256;
pub const TAS2552_BCLKDIR: u32 = 1 << 6;
pub const TAS2552_WCLKDIR: u32 = 1 << 7;

/* OUTPUT_DATA register */
pub const TAS2552_DATA_OUT_I_DATA: u32 = 0x0;
pub const TAS2552_DATA_OUT_V_DATA: u32 = 0x1;
pub const TAS2552_DATA_OUT_VBAT_DATA: u32 = 0x2;
pub const TAS2552_DATA_OUT_VBOOST_DATA: u32 = 0x3;
pub const TAS2552_DATA_OUT_PGA_GAIN: u32 = 0x4;
pub const TAS2552_DATA_OUT_IV_DATA: u32 = 0x5;
pub const TAS2552_DATA_OUT_VBAT_VBOOST_GAIN: u32 = 0x6;
pub const TAS2552_DATA_OUT_DISABLED: u32 = 0x7;
pub const fn TAS2552_L_DATA_OUT(x: u32) -> u32 {
    x << 0
}
pub const fn TAS2552_R_DATA_OUT(x: u32) -> u32 {
    x << 3
}
pub const TAS2552_PDM_DATA_SEL_I: u32 = 0x0 << 6;
pub const TAS2552_PDM_DATA_SEL_V: u32 = 0x1 << 6;
pub const TAS2552_PDM_DATA_SEL_I_V: u32 = 0x2 << 6;
pub const TAS2552_PDM_DATA_SEL_V_I: u32 = 0x3 << 6;
pub const TAS2552_PDM_DATA_SEL_MASK: u32 = TAS2552_PDM_DATA_SEL_V_I;

/* PDM CFG Register */
pub const TAS2552_PDM_CLK_SEL_PLL: u32 = 0x0 << 0;
pub const TAS2552_PDM_CLK_SEL_IVCLKIN: u32 = 0x1 << 0;
pub const TAS2552_PDM_CLK_SEL_BCLK: u32 = 0x2 << 0;
pub const TAS2552_PDM_CLK_SEL_MCLK: u32 = 0x3 << 0;
pub const TAS2552_PDM_CLK_SEL_MASK: u32 = TAS2552_PDM_CLK_SEL_MCLK;
pub const TAS2552_PDM_DATA_ES: u32 = 1 << 2;

/* Boost Auto-pass through register */
pub const TAS2552_APT_DELAY_50: u32 = 0x0 << 0;
pub const TAS2552_APT_DELAY_75: u32 = 0x1 << 0;
pub const TAS2552_APT_DELAY_125: u32 = 0x2 << 0;
pub const TAS2552_APT_DELAY_200: u32 = 0x3 << 0;
pub const TAS2552_APT_THRESH_05_02: u32 = 0x0 << 2;
pub const TAS2552_APT_THRESH_10_07: u32 = 0x1 << 2;
pub const TAS2552_APT_THRESH_14_11: u32 = 0x2 << 2;
pub const TAS2552_APT_THRESH_20_17: u32 = 0x3 << 2;

/* PLL Control Register */
pub const TAS2552_PLL_J_MASK: u32 = 0x7f;
pub const fn TAS2552_PLL_D_UPPER(x: u32) -> u32 {
    (x >> 8) & 0x3f
}
pub const fn TAS2552_PLL_D_LOWER(x: u32) -> u32 {
    x & 0xff
}
pub const TAS2552_PLL_BYPASS: u32 = 1 << 7;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
