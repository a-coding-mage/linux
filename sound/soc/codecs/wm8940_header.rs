/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wm8940.h -- WM8940 Soc Audio driver
 */

#[repr(C)]
pub struct wm8940_setup_data {
    /* Vref to analogue output resistance */
    /* C bitfield: unsigned int vroi:1; */
    pub vroi: ::core::ffi::c_uint,
}

pub const WM8940_VROI_1K: ::core::ffi::c_uint = 0;
pub const WM8940_VROI_30K: ::core::ffi::c_uint = 1;

/* WM8940 register space */
pub const WM8940_SOFTRESET: ::core::ffi::c_uint = 0x00;
pub const WM8940_POWER1: ::core::ffi::c_uint = 0x01;
pub const WM8940_POWER2: ::core::ffi::c_uint = 0x02;
pub const WM8940_POWER3: ::core::ffi::c_uint = 0x03;
pub const WM8940_IFACE: ::core::ffi::c_uint = 0x04;
pub const WM8940_COMPANDINGCTL: ::core::ffi::c_uint = 0x05;
pub const WM8940_CLOCK: ::core::ffi::c_uint = 0x06;
pub const WM8940_ADDCNTRL: ::core::ffi::c_uint = 0x07;
pub const WM8940_GPIO: ::core::ffi::c_uint = 0x08;
pub const WM8940_CTLINT: ::core::ffi::c_uint = 0x09;
pub const WM8940_DAC: ::core::ffi::c_uint = 0x0A;
pub const WM8940_DACVOL: ::core::ffi::c_uint = 0x0B;

pub const WM8940_ADC: ::core::ffi::c_uint = 0x0E;
pub const WM8940_ADCVOL: ::core::ffi::c_uint = 0x0F;
pub const WM8940_NOTCH1: ::core::ffi::c_uint = 0x10;
pub const WM8940_NOTCH2: ::core::ffi::c_uint = 0x11;
pub const WM8940_NOTCH3: ::core::ffi::c_uint = 0x12;
pub const WM8940_NOTCH4: ::core::ffi::c_uint = 0x13;
pub const WM8940_NOTCH5: ::core::ffi::c_uint = 0x14;
pub const WM8940_NOTCH6: ::core::ffi::c_uint = 0x15;
pub const WM8940_NOTCH7: ::core::ffi::c_uint = 0x16;
pub const WM8940_NOTCH8: ::core::ffi::c_uint = 0x17;
pub const WM8940_DACLIM1: ::core::ffi::c_uint = 0x18;
pub const WM8940_DACLIM2: ::core::ffi::c_uint = 0x19;

pub const WM8940_ALC1: ::core::ffi::c_uint = 0x20;
pub const WM8940_ALC2: ::core::ffi::c_uint = 0x21;
pub const WM8940_ALC3: ::core::ffi::c_uint = 0x22;
pub const WM8940_NOISEGATE: ::core::ffi::c_uint = 0x23;
pub const WM8940_PLLN: ::core::ffi::c_uint = 0x24;
pub const WM8940_PLLK1: ::core::ffi::c_uint = 0x25;
pub const WM8940_PLLK2: ::core::ffi::c_uint = 0x26;
pub const WM8940_PLLK3: ::core::ffi::c_uint = 0x27;

pub const WM8940_ALC4: ::core::ffi::c_uint = 0x2A;

pub const WM8940_INPUTCTL: ::core::ffi::c_uint = 0x2C;
pub const WM8940_PGAGAIN: ::core::ffi::c_uint = 0x2D;

pub const WM8940_ADCBOOST: ::core::ffi::c_uint = 0x2F;

pub const WM8940_OUTPUTCTL: ::core::ffi::c_uint = 0x31;
pub const WM8940_SPKMIX: ::core::ffi::c_uint = 0x32;

pub const WM8940_SPKVOL: ::core::ffi::c_uint = 0x36;

pub const WM8940_MONOMIX: ::core::ffi::c_uint = 0x38;

pub const WM8940_CACHEREGNUM: ::core::ffi::c_uint = 0x57;

/* Clock divider Id's */
pub const WM8940_BCLKDIV: ::core::ffi::c_uint = 0;
pub const WM8940_MCLKDIV: ::core::ffi::c_uint = 1;
pub const WM8940_OPCLKDIV: ::core::ffi::c_uint = 2;

/* MCLK clock dividers */
pub const WM8940_MCLKDIV_1: ::core::ffi::c_uint = 0;
pub const WM8940_MCLKDIV_1_5: ::core::ffi::c_uint = 1;
pub const WM8940_MCLKDIV_2: ::core::ffi::c_uint = 2;
pub const WM8940_MCLKDIV_3: ::core::ffi::c_uint = 3;
pub const WM8940_MCLKDIV_4: ::core::ffi::c_uint = 4;
pub const WM8940_MCLKDIV_6: ::core::ffi::c_uint = 5;
pub const WM8940_MCLKDIV_8: ::core::ffi::c_uint = 6;
pub const WM8940_MCLKDIV_12: ::core::ffi::c_uint = 7;

/* BCLK clock dividers */
pub const WM8940_BCLKDIV_1: ::core::ffi::c_uint = 0;
pub const WM8940_BCLKDIV_2: ::core::ffi::c_uint = 1;
pub const WM8940_BCLKDIV_4: ::core::ffi::c_uint = 2;
pub const WM8940_BCLKDIV_8: ::core::ffi::c_uint = 3;
pub const WM8940_BCLKDIV_16: ::core::ffi::c_uint = 4;
pub const WM8940_BCLKDIV_32: ::core::ffi::c_uint = 5;

/* PLL Out Dividers */
pub const WM8940_OPCLKDIV_1: ::core::ffi::c_uint = 0;
pub const WM8940_OPCLKDIV_2: ::core::ffi::c_uint = 1;
pub const WM8940_OPCLKDIV_3: ::core::ffi::c_uint = 2;
pub const WM8940_OPCLKDIV_4: ::core::ffi::c_uint = 3;

/* Chip ID */
pub const WM8940_CHIP_ID: ::core::ffi::c_uint = 0x8940;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
