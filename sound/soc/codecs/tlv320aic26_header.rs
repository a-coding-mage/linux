/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Texas Instruments TLV320AIC26 low power audio CODEC
 * register definitions
 *
 * Copyright (C) 2008 Secret Lab Technologies Ltd.
 */

/* AIC26 Registers */
pub const fn AIC26_PAGE_ADDR(page: u32, offset: u32) -> u32 {
    (page << 11) | (offset << 5)
}

/* Page 0: Auxiliary data registers */
pub const AIC26_REG_BAT1: u32 = AIC26_PAGE_ADDR(0, 0x05);
pub const AIC26_REG_BAT2: u32 = AIC26_PAGE_ADDR(0, 0x06);
pub const AIC26_REG_AUX: u32 = AIC26_PAGE_ADDR(0, 0x07);
pub const AIC26_REG_TEMP1: u32 = AIC26_PAGE_ADDR(0, 0x09);
pub const AIC26_REG_TEMP2: u32 = AIC26_PAGE_ADDR(0, 0x0A);

/* Page 1: Auxiliary control registers */
pub const AIC26_REG_AUX_ADC: u32 = AIC26_PAGE_ADDR(1, 0x00);
pub const AIC26_REG_STATUS: u32 = AIC26_PAGE_ADDR(1, 0x01);
pub const AIC26_REG_REFERENCE: u32 = AIC26_PAGE_ADDR(1, 0x03);
pub const AIC26_REG_RESET: u32 = AIC26_PAGE_ADDR(1, 0x04);

/* Page 2: Audio control registers */
pub const AIC26_REG_AUDIO_CTRL1: u32 = AIC26_PAGE_ADDR(2, 0x00);
pub const AIC26_REG_ADC_GAIN: u32 = AIC26_PAGE_ADDR(2, 0x01);
pub const AIC26_REG_DAC_GAIN: u32 = AIC26_PAGE_ADDR(2, 0x02);
pub const AIC26_REG_SIDETONE: u32 = AIC26_PAGE_ADDR(2, 0x03);
pub const AIC26_REG_AUDIO_CTRL2: u32 = AIC26_PAGE_ADDR(2, 0x04);
pub const AIC26_REG_POWER_CTRL: u32 = AIC26_PAGE_ADDR(2, 0x05);
pub const AIC26_REG_AUDIO_CTRL3: u32 = AIC26_PAGE_ADDR(2, 0x06);

pub const AIC26_REG_FILTER_COEFF_L_N0: u32 = AIC26_PAGE_ADDR(2, 0x07);
pub const AIC26_REG_FILTER_COEFF_L_N1: u32 = AIC26_PAGE_ADDR(2, 0x08);
pub const AIC26_REG_FILTER_COEFF_L_N2: u32 = AIC26_PAGE_ADDR(2, 0x09);
pub const AIC26_REG_FILTER_COEFF_L_N3: u32 = AIC26_PAGE_ADDR(2, 0x0A);
pub const AIC26_REG_FILTER_COEFF_L_N4: u32 = AIC26_PAGE_ADDR(2, 0x0B);
pub const AIC26_REG_FILTER_COEFF_L_N5: u32 = AIC26_PAGE_ADDR(2, 0x0C);
pub const AIC26_REG_FILTER_COEFF_L_D1: u32 = AIC26_PAGE_ADDR(2, 0x0D);
pub const AIC26_REG_FILTER_COEFF_L_D2: u32 = AIC26_PAGE_ADDR(2, 0x0E);
pub const AIC26_REG_FILTER_COEFF_L_D4: u32 = AIC26_PAGE_ADDR(2, 0x0F);
pub const AIC26_REG_FILTER_COEFF_L_D5: u32 = AIC26_PAGE_ADDR(2, 0x10);
pub const AIC26_REG_FILTER_COEFF_R_N0: u32 = AIC26_PAGE_ADDR(2, 0x11);
pub const AIC26_REG_FILTER_COEFF_R_N1: u32 = AIC26_PAGE_ADDR(2, 0x12);
pub const AIC26_REG_FILTER_COEFF_R_N2: u32 = AIC26_PAGE_ADDR(2, 0x13);
pub const AIC26_REG_FILTER_COEFF_R_N3: u32 = AIC26_PAGE_ADDR(2, 0x14);
pub const AIC26_REG_FILTER_COEFF_R_N4: u32 = AIC26_PAGE_ADDR(2, 0x15);
pub const AIC26_REG_FILTER_COEFF_R_N5: u32 = AIC26_PAGE_ADDR(2, 0x16);
pub const AIC26_REG_FILTER_COEFF_R_D1: u32 = AIC26_PAGE_ADDR(2, 0x17);
pub const AIC26_REG_FILTER_COEFF_R_D2: u32 = AIC26_PAGE_ADDR(2, 0x18);
pub const AIC26_REG_FILTER_COEFF_R_D4: u32 = AIC26_PAGE_ADDR(2, 0x19);
pub const AIC26_REG_FILTER_COEFF_R_D5: u32 = AIC26_PAGE_ADDR(2, 0x1A);

pub const AIC26_REG_PLL_PROG1: u32 = AIC26_PAGE_ADDR(2, 0x1B);
pub const AIC26_REG_PLL_PROG2: u32 = AIC26_PAGE_ADDR(2, 0x1C);
pub const AIC26_REG_AUDIO_CTRL4: u32 = AIC26_PAGE_ADDR(2, 0x1D);
pub const AIC26_REG_AUDIO_CTRL5: u32 = AIC26_PAGE_ADDR(2, 0x1E);

/* fsref dividers; used in register 'Audio Control 1' */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum aic26_divisors {
    AIC26_DIV_1 = 0,
    AIC26_DIV_1_5 = 1,
    AIC26_DIV_2 = 2,
    AIC26_DIV_3 = 3,
    AIC26_DIV_4 = 4,
    AIC26_DIV_5 = 5,
    AIC26_DIV_5_5 = 6,
    AIC26_DIV_6 = 7,
}

/* Digital data format */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum aic26_datfm {
    AIC26_DATFM_I2S = 0 << 8,
    AIC26_DATFM_DSP = 1 << 8,
    AIC26_DATFM_RIGHTJ = 2 << 8, /* right justified */
    AIC26_DATFM_LEFTJ = 3 << 8,  /* left justified */
}

/* Sample word length in bits; used in register 'Audio Control 1' */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum aic26_wlen {
    AIC26_WLEN_16 = 0 << 10,
    AIC26_WLEN_20 = 1 << 10,
    AIC26_WLEN_24 = 2 << 10,
    AIC26_WLEN_32 = 3 << 10,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
