/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wm8900.h  --  WM890 Soc Audio driver
 */

pub const WM8900_FLL: i32 = 1;

pub const WM8900_BCLK_DIV: i32 = 1;
pub const WM8900_ADC_CLKDIV: i32 = 2;
pub const WM8900_DAC_CLKDIV: i32 = 3;
pub const WM8900_ADC_LRCLK: i32 = 4;
pub const WM8900_DAC_LRCLK: i32 = 5;
pub const WM8900_OPCLK_DIV: i32 = 6;
pub const WM8900_LRCLK_MODE: i32 = 7;

pub const WM8900_BCLK_DIV_1: i32 = 0x00;
pub const WM8900_BCLK_DIV_1_5: i32 = 0x02;
pub const WM8900_BCLK_DIV_2: i32 = 0x04;
pub const WM8900_BCLK_DIV_3: i32 = 0x06;
pub const WM8900_BCLK_DIV_4: i32 = 0x08;
pub const WM8900_BCLK_DIV_5_5: i32 = 0x0a;
pub const WM8900_BCLK_DIV_6: i32 = 0x0c;
pub const WM8900_BCLK_DIV_8: i32 = 0x0e;
pub const WM8900_BCLK_DIV_11: i32 = 0x10;
pub const WM8900_BCLK_DIV_12: i32 = 0x12;
pub const WM8900_BCLK_DIV_16: i32 = 0x14;
pub const WM8900_BCLK_DIV_22: i32 = 0x16;
pub const WM8900_BCLK_DIV_24: i32 = 0x18;
pub const WM8900_BCLK_DIV_32: i32 = 0x1a;
pub const WM8900_BCLK_DIV_44: i32 = 0x1c;
pub const WM8900_BCLK_DIV_48: i32 = 0x1e;

pub const WM8900_ADC_CLKDIV_1: i32 = 0x00;
pub const WM8900_ADC_CLKDIV_1_5: i32 = 0x20;
pub const WM8900_ADC_CLKDIV_2: i32 = 0x40;
pub const WM8900_ADC_CLKDIV_3: i32 = 0x60;
pub const WM8900_ADC_CLKDIV_4: i32 = 0x80;
pub const WM8900_ADC_CLKDIV_5_5: i32 = 0xa0;
pub const WM8900_ADC_CLKDIV_6: i32 = 0xc0;

pub const WM8900_DAC_CLKDIV_1: i32 = 0x00;
pub const WM8900_DAC_CLKDIV_1_5: i32 = 0x04;
pub const WM8900_DAC_CLKDIV_2: i32 = 0x08;
pub const WM8900_DAC_CLKDIV_3: i32 = 0x0c;
pub const WM8900_DAC_CLKDIV_4: i32 = 0x10;
pub const WM8900_DAC_CLKDIV_5_5: i32 = 0x14;
pub const WM8900_DAC_CLKDIV_6: i32 = 0x18;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
