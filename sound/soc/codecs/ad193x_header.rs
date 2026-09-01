/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * AD193X Audio Codec driver
 *
 * Copyright 2010 Analog Devices Inc.
 */

// C dependency intent: #include <linux/regmap.h>

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ad193x_type {
    AD193X,
    AD1933,
    AD1934,
}

unsafe extern "C" {
    pub static ad193x_regmap_config: regmap_config;
    pub fn ad193x_probe(
        dev: *mut device,
        regmap: *mut regmap,
        type_: ad193x_type,
    ) -> core::ffi::c_int;
}

pub const AD193X_PLL_CLK_CTRL0: u32 = 0x00;
pub const AD193X_PLL_POWERDOWN: u32 = 0x01;
pub const AD193X_PLL_INPUT_MASK: u32 = 0x6;
pub const AD193X_PLL_INPUT_256: u32 = 0 << 1;
pub const AD193X_PLL_INPUT_384: u32 = 1 << 1;
pub const AD193X_PLL_INPUT_512: u32 = 2 << 1;
pub const AD193X_PLL_INPUT_768: u32 = 3 << 1;
pub const AD193X_PLL_CLK_CTRL1: u32 = 0x01;
pub const AD193X_PLL_SRC_MASK: u32 = 0x03;
pub const AD193X_PLL_DAC_SRC_PLL: u32 = 0;
pub const AD193X_PLL_DAC_SRC_MCLK: u32 = 1;
pub const AD193X_PLL_CLK_SRC_PLL: u32 = 0 << 1;
pub const AD193X_PLL_CLK_SRC_MCLK: u32 = 1 << 1;
pub const AD193X_DAC_CTRL0: u32 = 0x02;
pub const AD193X_DAC_POWERDOWN: u32 = 0x01;
pub const AD193X_DAC_SR_MASK: u32 = 0x06;
pub const AD193X_DAC_SR_48: u32 = 0 << 1;
pub const AD193X_DAC_SR_96: u32 = 1 << 1;
pub const AD193X_DAC_SR_192: u32 = 2 << 1;
pub const AD193X_DAC_SERFMT_MASK: u32 = 0xC0;
pub const AD193X_DAC_SERFMT_STEREO: u32 = 0 << 6;
pub const AD193X_DAC_SERFMT_TDM: u32 = 1 << 6;
pub const AD193X_DAC_CTRL1: u32 = 0x03;
pub const AD193X_DAC_CHAN_SHFT: u32 = 1;
pub const AD193X_DAC_CHAN_MASK: u32 = 3 << AD193X_DAC_CHAN_SHFT;
pub const AD193X_DAC_LCR_MASTER: u32 = 1 << 4;
pub const AD193X_DAC_BCLK_MASTER: u32 = 1 << 5;
pub const AD193X_DAC_LEFT_HIGH: u32 = 1 << 3;
pub const AD193X_DAC_BCLK_INV: u32 = 1 << 7;
pub const AD193X_DAC_FMT_MASK: u32 =
    AD193X_DAC_LCR_MASTER | AD193X_DAC_BCLK_MASTER | AD193X_DAC_LEFT_HIGH | AD193X_DAC_BCLK_INV;
pub const AD193X_DAC_CTRL2: u32 = 0x04;
pub const AD193X_DAC_WORD_LEN_SHFT: u32 = 3;
pub const AD193X_DAC_WORD_LEN_MASK: u32 = 0x18;
pub const AD193X_DAC_MASTER_MUTE: u32 = 1;
pub const AD193X_DAC_CHNL_MUTE: u32 = 0x05;
pub const AD193X_DACL1_MUTE: u32 = 0;
pub const AD193X_DACR1_MUTE: u32 = 1;
pub const AD193X_DACL2_MUTE: u32 = 2;
pub const AD193X_DACR2_MUTE: u32 = 3;
pub const AD193X_DACL3_MUTE: u32 = 4;
pub const AD193X_DACR3_MUTE: u32 = 5;
pub const AD193X_DACL4_MUTE: u32 = 6;
pub const AD193X_DACR4_MUTE: u32 = 7;
pub const AD193X_DAC_L1_VOL: u32 = 0x06;
pub const AD193X_DAC_R1_VOL: u32 = 0x07;
pub const AD193X_DAC_L2_VOL: u32 = 0x08;
pub const AD193X_DAC_R2_VOL: u32 = 0x09;
pub const AD193X_DAC_L3_VOL: u32 = 0x0a;
pub const AD193X_DAC_R3_VOL: u32 = 0x0b;
pub const AD193X_DAC_L4_VOL: u32 = 0x0c;
pub const AD193X_DAC_R4_VOL: u32 = 0x0d;
pub const AD193X_ADC_CTRL0: u32 = 0x0e;
pub const AD193X_ADC_POWERDOWN: u32 = 0x01;
pub const AD193X_ADC_HIGHPASS_FILTER: u32 = 1;
pub const AD193X_ADCL1_MUTE: u32 = 2;
pub const AD193X_ADCR1_MUTE: u32 = 3;
pub const AD193X_ADCL2_MUTE: u32 = 4;
pub const AD193X_ADCR2_MUTE: u32 = 5;
pub const AD193X_ADC_CTRL1: u32 = 0x0f;
pub const AD193X_ADC_SERFMT_MASK: u32 = 0x60;
pub const AD193X_ADC_SERFMT_STEREO: u32 = 0 << 5;
pub const AD193X_ADC_SERFMT_TDM: u32 = 1 << 5;
pub const AD193X_ADC_SERFMT_AUX: u32 = 2 << 5;
pub const AD193X_ADC_WORD_LEN_MASK: u32 = 0x3;
pub const AD193X_ADC_CTRL2: u32 = 0x10;
pub const AD193X_ADC_CHAN_SHFT: u32 = 4;
pub const AD193X_ADC_CHAN_MASK: u32 = 3 << AD193X_ADC_CHAN_SHFT;
pub const AD193X_ADC_LCR_MASTER: u32 = 1 << 3;
pub const AD193X_ADC_BCLK_MASTER: u32 = 1 << 6;
pub const AD193X_ADC_LEFT_HIGH: u32 = 1 << 2;
pub const AD193X_ADC_BCLK_INV: u32 = 1 << 1;
pub const AD193X_ADC_FMT_MASK: u32 =
    AD193X_ADC_LCR_MASTER | AD193X_ADC_BCLK_MASTER | AD193X_ADC_LEFT_HIGH | AD193X_ADC_BCLK_INV;

pub const AD193X_2_CHANNELS: u32 = 0;
pub const AD193X_4_CHANNELS: u32 = 1;
pub const AD193X_8_CHANNELS: u32 = 2;
pub const AD193X_16_CHANNELS: u32 = 3;

pub const AD193X_NUM_REGS: u32 = 17;

pub const AD193X_SYSCLK_PLL: u32 = 0;
pub const AD193X_SYSCLK_MCLK: u32 = 1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
