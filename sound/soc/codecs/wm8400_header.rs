/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * wm8400.h  --  audio driver for WM8400
 *
 * Copyright 2008 Wolfson Microelectronics PLC.
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

pub const WM8400_MCLK_DIV: u32 = 0;
pub const WM8400_DACCLK_DIV: u32 = 1;
pub const WM8400_ADCCLK_DIV: u32 = 2;
pub const WM8400_BCLK_DIV: u32 = 3;

pub const WM8400_MCLK_DIV_1: u32 = 0x400;
pub const WM8400_MCLK_DIV_2: u32 = 0x800;

pub const WM8400_DAC_CLKDIV_1: u32 = 0x00;
pub const WM8400_DAC_CLKDIV_1_5: u32 = 0x04;
pub const WM8400_DAC_CLKDIV_2: u32 = 0x08;
pub const WM8400_DAC_CLKDIV_3: u32 = 0x0c;
pub const WM8400_DAC_CLKDIV_4: u32 = 0x10;
pub const WM8400_DAC_CLKDIV_5_5: u32 = 0x14;
pub const WM8400_DAC_CLKDIV_6: u32 = 0x18;

pub const WM8400_ADC_CLKDIV_1: u32 = 0x00;
pub const WM8400_ADC_CLKDIV_1_5: u32 = 0x20;
pub const WM8400_ADC_CLKDIV_2: u32 = 0x40;
pub const WM8400_ADC_CLKDIV_3: u32 = 0x60;
pub const WM8400_ADC_CLKDIV_4: u32 = 0x80;
pub const WM8400_ADC_CLKDIV_5_5: u32 = 0xa0;
pub const WM8400_ADC_CLKDIV_6: u32 = 0xc0;

pub const WM8400_BCLK_DIV_1: u32 = 0x0 << 1;
pub const WM8400_BCLK_DIV_1_5: u32 = 0x1 << 1;
pub const WM8400_BCLK_DIV_2: u32 = 0x2 << 1;
pub const WM8400_BCLK_DIV_3: u32 = 0x3 << 1;
pub const WM8400_BCLK_DIV_4: u32 = 0x4 << 1;
pub const WM8400_BCLK_DIV_5_5: u32 = 0x5 << 1;
pub const WM8400_BCLK_DIV_6: u32 = 0x6 << 1;
pub const WM8400_BCLK_DIV_8: u32 = 0x7 << 1;
pub const WM8400_BCLK_DIV_11: u32 = 0x8 << 1;
pub const WM8400_BCLK_DIV_12: u32 = 0x9 << 1;
pub const WM8400_BCLK_DIV_16: u32 = 0xA << 1;
pub const WM8400_BCLK_DIV_22: u32 = 0xB << 1;
pub const WM8400_BCLK_DIV_24: u32 = 0xC << 1;
pub const WM8400_BCLK_DIV_32: u32 = 0xD << 1;
pub const WM8400_BCLK_DIV_44: u32 = 0xE << 1;
pub const WM8400_BCLK_DIV_48: u32 = 0xF << 1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
