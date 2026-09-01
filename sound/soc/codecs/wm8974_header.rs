// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8974.h  --  WM8974 Soc Audio driver
 */

/* WM8974 register space */

pub const WM8974_RESET: i32 = 0x0;
pub const WM8974_POWER1: i32 = 0x1;
pub const WM8974_POWER2: i32 = 0x2;
pub const WM8974_POWER3: i32 = 0x3;
pub const WM8974_IFACE: i32 = 0x4;
pub const WM8974_COMP: i32 = 0x5;
pub const WM8974_CLOCK: i32 = 0x6;
pub const WM8974_ADD: i32 = 0x7;
pub const WM8974_GPIO: i32 = 0x8;
pub const WM8974_DAC: i32 = 0xa;
pub const WM8974_DACVOL: i32 = 0xb;
pub const WM8974_ADC: i32 = 0xe;
pub const WM8974_ADCVOL: i32 = 0xf;
pub const WM8974_EQ1: i32 = 0x12;
pub const WM8974_EQ2: i32 = 0x13;
pub const WM8974_EQ3: i32 = 0x14;
pub const WM8974_EQ4: i32 = 0x15;
pub const WM8974_EQ5: i32 = 0x16;
pub const WM8974_DACLIM1: i32 = 0x18;
pub const WM8974_DACLIM2: i32 = 0x19;
pub const WM8974_NOTCH1: i32 = 0x1b;
pub const WM8974_NOTCH2: i32 = 0x1c;
pub const WM8974_NOTCH3: i32 = 0x1d;
pub const WM8974_NOTCH4: i32 = 0x1e;
pub const WM8974_ALC1: i32 = 0x20;
pub const WM8974_ALC2: i32 = 0x21;
pub const WM8974_ALC3: i32 = 0x22;
pub const WM8974_NGATE: i32 = 0x23;
pub const WM8974_PLLN: i32 = 0x24;
pub const WM8974_PLLK1: i32 = 0x25;
pub const WM8974_PLLK2: i32 = 0x26;
pub const WM8974_PLLK3: i32 = 0x27;
pub const WM8974_ATTEN: i32 = 0x28;
pub const WM8974_INPUT: i32 = 0x2c;
pub const WM8974_INPPGA: i32 = 0x2d;
pub const WM8974_ADCBOOST: i32 = 0x2f;
pub const WM8974_OUTPUT: i32 = 0x31;
pub const WM8974_SPKMIX: i32 = 0x32;
pub const WM8974_SPKVOL: i32 = 0x36;
pub const WM8974_MONOMIX: i32 = 0x38;

pub const WM8974_CACHEREGNUM: i32 = 57;

/* Clock divider Id's */
pub const WM8974_OPCLKDIV: i32 = 0;
pub const WM8974_MCLKDIV: i32 = 1;
pub const WM8974_BCLKDIV: i32 = 2;

/* PLL Out dividers */
pub const WM8974_OPCLKDIV_1: i32 = 0 << 4;
pub const WM8974_OPCLKDIV_2: i32 = 1 << 4;
pub const WM8974_OPCLKDIV_3: i32 = 2 << 4;
pub const WM8974_OPCLKDIV_4: i32 = 3 << 4;

/* BCLK clock dividers */
pub const WM8974_BCLKDIV_1: i32 = 0 << 2;
pub const WM8974_BCLKDIV_2: i32 = 1 << 2;
pub const WM8974_BCLKDIV_4: i32 = 2 << 2;
pub const WM8974_BCLKDIV_8: i32 = 3 << 2;
pub const WM8974_BCLKDIV_16: i32 = 4 << 2;
pub const WM8974_BCLKDIV_32: i32 = 5 << 2;

/* MCLK clock dividers */
pub const WM8974_MCLKDIV_1: i32 = 0 << 5;
pub const WM8974_MCLKDIV_1_5: i32 = 1 << 5;
pub const WM8974_MCLKDIV_2: i32 = 2 << 5;
pub const WM8974_MCLKDIV_3: i32 = 3 << 5;
pub const WM8974_MCLKDIV_4: i32 = 4 << 5;
pub const WM8974_MCLKDIV_6: i32 = 5 << 5;
pub const WM8974_MCLKDIV_8: i32 = 6 << 5;
pub const WM8974_MCLKDIV_12: i32 = 7 << 5;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
