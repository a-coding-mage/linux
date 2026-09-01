/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Audio Codec driver supporting:
 *  AD1835A, AD1836, AD1837A, AD1838A, AD1839A
 *
 * Copyright 2009-2011 Analog Devices Inc.
 */

pub const AD1836_DAC_CTRL1: u32 = 0;
pub const AD1836_DAC_POWERDOWN: u32 = 2;
pub const AD1836_DAC_SERFMT_MASK: u32 = 0xE0;
pub const AD1836_DAC_SERFMT_PCK256: u32 = 0x4 << 5;
pub const AD1836_DAC_SERFMT_PCK128: u32 = 0x5 << 5;
pub const AD1836_DAC_WORD_LEN_MASK: u32 = 0x18;
pub const AD1836_DAC_WORD_LEN_OFFSET: u32 = 3;

pub const AD1836_DAC_CTRL2: u32 = 1;

/* These macros are one-based. So AD183X_MUTE_LEFT(1) will return the mute bit
 * for the first ADC/DAC */
pub const fn AD1836_MUTE_LEFT(x: u32) -> u32 {
    (x * 2) - 2
}

pub const fn AD1836_MUTE_RIGHT(x: u32) -> u32 {
    (x * 2) - 1
}

pub const fn AD1836_DAC_L_VOL(x: u32) -> u32 {
    x * 2
}

pub const fn AD1836_DAC_R_VOL(x: u32) -> u32 {
    1 + (x * 2)
}

pub const AD1836_ADC_CTRL1: u32 = 12;
pub const AD1836_ADC_POWERDOWN: u32 = 7;
pub const AD1836_ADC_HIGHPASS_FILTER: u32 = 8;

pub const AD1836_ADC_CTRL2: u32 = 13;
pub const AD1836_ADC_WORD_LEN_MASK: u32 = 0x30;
pub const AD1836_ADC_WORD_OFFSET: u32 = 4;
pub const AD1836_ADC_SERFMT_MASK: u32 = 7 << 6;
pub const AD1836_ADC_SERFMT_PCK256: u32 = 0x4 << 6;
pub const AD1836_ADC_SERFMT_PCK128: u32 = 0x5 << 6;
pub const AD1836_ADC_AUX: u32 = 0x6 << 6;

pub const AD1836_ADC_CTRL3: u32 = 14;

pub const AD1836_NUM_REGS: u32 = 16;

pub const AD1836_WORD_LEN_24: u32 = 0x0;
pub const AD1836_WORD_LEN_20: u32 = 0x1;
pub const AD1836_WORD_LEN_16: u32 = 0x2;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
