/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * max9867.h -- MAX9867 ALSA SoC Audio driver
 *
 * Copyright 2013-2015 Maxim Integrated Products
 */

/* MAX9867 register space */

pub const MAX9867_STATUS: u32 = 0x00;
pub const MAX9867_JACKSTATUS: u32 = 0x01;
pub const MAX9867_AUXHIGH: u32 = 0x02;
pub const MAX9867_AUXLOW: u32 = 0x03;
pub const MAX9867_INTEN: u32 = 0x04;
pub const MAX9867_SYSCLK: u32 = 0x05;
pub const MAX9867_FREQ_MASK: u32 = 0xF;
pub const MAX9867_PSCLK_SHIFT: u32 = 0x4;
pub const MAX9867_PSCLK_WIDTH: u32 = 0x2;
pub const MAX9867_PSCLK_MASK: u32 = 0x03 << MAX9867_PSCLK_SHIFT;
pub const MAX9867_PSCLK_10_20: u32 = 0x1;
pub const MAX9867_PSCLK_20_40: u32 = 0x2;
pub const MAX9867_PSCLK_40_60: u32 = 0x3;
pub const MAX9867_AUDIOCLKHIGH: u32 = 0x06;
pub const MAX9867_NI_HIGH_MASK: u32 = 0x7F;
pub const MAX9867_NI_LOW_MASK: u32 = 0xFE;
pub const MAX9867_PLL: u32 = 1 << 7;
pub const MAX9867_AUDIOCLKLOW: u32 = 0x07;
pub const MAX9867_RAPID_LOCK: u32 = 0x01;
pub const MAX9867_IFC1A: u32 = 0x08;
pub const MAX9867_MASTER: u32 = 1 << 7;
pub const MAX9867_I2S_DLY: u32 = 1 << 4;
pub const MAX9867_SDOUT_HIZ: u32 = 1 << 3;
pub const MAX9867_TDM_MODE: u32 = 1 << 2;
pub const MAX9867_WCI_MODE: u32 = 1 << 6;
pub const MAX9867_BCI_MODE: u32 = 1 << 5;
pub const MAX9867_IFC1B: u32 = 0x09;
pub const MAX9867_IFC1B_BCLK_MASK: u32 = 7;
pub const MAX9867_IFC1B_64X: u32 = 0x01;
pub const MAX9867_IFC1B_48X: u32 = 0x02;
pub const MAX9867_IFC1B_PCLK_2: u32 = 0x04;
pub const MAX9867_IFC1B_PCLK_4: u32 = 0x05;
pub const MAX9867_IFC1B_PCLK_8: u32 = 0x06;
pub const MAX9867_IFC1B_PCLK_16: u32 = 0x07;
pub const MAX9867_CODECFLTR: u32 = 0x0a;
pub const MAX9867_CODECFLTR_MODE: u32 = 1 << 7;
pub const MAX9867_SIDETONE: u32 = 0x0b;
pub const MAX9867_DACLEVEL: u32 = 0x0c;
pub const MAX9867_ADCLEVEL: u32 = 0x0d;
pub const MAX9867_LEFTLINELVL: u32 = 0x0e;
pub const MAX9867_RIGHTLINELVL: u32 = 0x0f;
pub const MAX9867_LEFTVOL: u32 = 0x10;
pub const MAX9867_RIGHTVOL: u32 = 0x11;
pub const MAX9867_LEFTMICGAIN: u32 = 0x12;
pub const MAX9867_RIGHTMICGAIN: u32 = 0x13;
pub const MAX9867_INPUTCONFIG: u32 = 0x14;
pub const MAX9867_MICCONFIG: u32 = 0x15;
pub const MAX9867_MODECONFIG: u32 = 0x16;
pub const MAX9867_PWRMAN: u32 = 0x17;
pub const MAX9867_PWRMAN_SHDN: u32 = 1 << 7;
pub const MAX9867_REVISION: u32 = 0xff;

pub const MAX9867_CACHEREGNUM: u32 = 10;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
