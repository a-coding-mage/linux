/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wm8510.h  --  WM8510 Soc Audio driver
 */

use core::ffi::c_int;

/* WM8510 register space */

pub const WM8510_RESET: u32 = 0x0;
pub const WM8510_POWER1: u32 = 0x1;
pub const WM8510_POWER2: u32 = 0x2;
pub const WM8510_POWER3: u32 = 0x3;
pub const WM8510_IFACE: u32 = 0x4;
pub const WM8510_COMP: u32 = 0x5;
pub const WM8510_CLOCK: u32 = 0x6;
pub const WM8510_ADD: u32 = 0x7;
pub const WM8510_GPIO: u32 = 0x8;
pub const WM8510_DAC: u32 = 0xa;
pub const WM8510_DACVOL: u32 = 0xb;
pub const WM8510_ADC: u32 = 0xe;
pub const WM8510_ADCVOL: u32 = 0xf;
pub const WM8510_EQ1: u32 = 0x12;
pub const WM8510_EQ2: u32 = 0x13;
pub const WM8510_EQ3: u32 = 0x14;
pub const WM8510_EQ4: u32 = 0x15;
pub const WM8510_EQ5: u32 = 0x16;
pub const WM8510_DACLIM1: u32 = 0x18;
pub const WM8510_DACLIM2: u32 = 0x19;
pub const WM8510_NOTCH1: u32 = 0x1b;
pub const WM8510_NOTCH2: u32 = 0x1c;
pub const WM8510_NOTCH3: u32 = 0x1d;
pub const WM8510_NOTCH4: u32 = 0x1e;
pub const WM8510_ALC1: u32 = 0x20;
pub const WM8510_ALC2: u32 = 0x21;
pub const WM8510_ALC3: u32 = 0x22;
pub const WM8510_NGATE: u32 = 0x23;
pub const WM8510_PLLN: u32 = 0x24;
pub const WM8510_PLLK1: u32 = 0x25;
pub const WM8510_PLLK2: u32 = 0x26;
pub const WM8510_PLLK3: u32 = 0x27;
pub const WM8510_ATTEN: u32 = 0x28;
pub const WM8510_INPUT: u32 = 0x2c;
pub const WM8510_INPPGA: u32 = 0x2d;
pub const WM8510_ADCBOOST: u32 = 0x2f;
pub const WM8510_OUTPUT: u32 = 0x31;
pub const WM8510_SPKMIX: u32 = 0x32;
pub const WM8510_SPKVOL: u32 = 0x36;
pub const WM8510_MONOMIX: u32 = 0x38;

pub const WM8510_CACHEREGNUM: u32 = 57;

/* Clock divider Id's */
pub const WM8510_OPCLKDIV: u32 = 0;
pub const WM8510_MCLKDIV: u32 = 1;
pub const WM8510_ADCCLK: u32 = 2;
pub const WM8510_DACCLK: u32 = 3;
pub const WM8510_BCLKDIV: u32 = 4;

/* DAC clock dividers */
pub const WM8510_DACCLK_F2: u32 = 1 << 3;
pub const WM8510_DACCLK_F4: u32 = 0 << 3;

/* ADC clock dividers */
pub const WM8510_ADCCLK_F2: u32 = 1 << 3;
pub const WM8510_ADCCLK_F4: u32 = 0 << 3;

/* PLL Out dividers */
pub const WM8510_OPCLKDIV_1: u32 = 0 << 4;
pub const WM8510_OPCLKDIV_2: u32 = 1 << 4;
pub const WM8510_OPCLKDIV_3: u32 = 2 << 4;
pub const WM8510_OPCLKDIV_4: u32 = 3 << 4;

/* BCLK clock dividers */
pub const WM8510_BCLKDIV_1: u32 = 0 << 2;
pub const WM8510_BCLKDIV_2: u32 = 1 << 2;
pub const WM8510_BCLKDIV_4: u32 = 2 << 2;
pub const WM8510_BCLKDIV_8: u32 = 3 << 2;
pub const WM8510_BCLKDIV_16: u32 = 4 << 2;
pub const WM8510_BCLKDIV_32: u32 = 5 << 2;

/* MCLK clock dividers */
pub const WM8510_MCLKDIV_1: u32 = 0 << 5;
pub const WM8510_MCLKDIV_1_5: u32 = 1 << 5;
pub const WM8510_MCLKDIV_2: u32 = 2 << 5;
pub const WM8510_MCLKDIV_3: u32 = 3 << 5;
pub const WM8510_MCLKDIV_4: u32 = 4 << 5;
pub const WM8510_MCLKDIV_6: u32 = 5 << 5;
pub const WM8510_MCLKDIV_8: u32 = 6 << 5;
pub const WM8510_MCLKDIV_12: u32 = 7 << 5;

#[repr(C)]
pub struct wm8510_setup_data {
    pub spi: c_int,
    pub i2c_bus: c_int,
    pub i2c_address: u16,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
