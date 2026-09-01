/* SPDX-License-Identifier: GPL-2.0 */
/*
 * nau8822.h  --  NAU8822 ALSA SoC Audio driver
 *
 * Copyright 2017 Nuvoton Technology Crop.
 *
 * Author: David Lin <ctlin0@nuvoton.com>
 * Co-author: John Hsu <kchsu0@nuvoton.com>
 * Co-author: Seven Li <wtli@nuvoton.com>
 */

pub const NAU8822_REG_RESET: u32 = 0x00;
pub const NAU8822_REG_POWER_MANAGEMENT_1: u32 = 0x01;
pub const NAU8822_REG_POWER_MANAGEMENT_2: u32 = 0x02;
pub const NAU8822_REG_POWER_MANAGEMENT_3: u32 = 0x03;
pub const NAU8822_REG_AUDIO_INTERFACE: u32 = 0x04;
pub const NAU8822_REG_COMPANDING_CONTROL: u32 = 0x05;
pub const NAU8822_REG_CLOCKING: u32 = 0x06;
pub const NAU8822_REG_ADDITIONAL_CONTROL: u32 = 0x07;
pub const NAU8822_REG_GPIO_CONTROL: u32 = 0x08;
pub const NAU8822_REG_JACK_DETECT_CONTROL_1: u32 = 0x09;
pub const NAU8822_REG_DAC_CONTROL: u32 = 0x0A;
pub const NAU8822_REG_LEFT_DAC_DIGITAL_VOLUME: u32 = 0x0B;
pub const NAU8822_REG_RIGHT_DAC_DIGITAL_VOLUME: u32 = 0x0C;
pub const NAU8822_REG_JACK_DETECT_CONTROL_2: u32 = 0x0D;
pub const NAU8822_REG_ADC_CONTROL: u32 = 0x0E;
pub const NAU8822_REG_LEFT_ADC_DIGITAL_VOLUME: u32 = 0x0F;
pub const NAU8822_REG_RIGHT_ADC_DIGITAL_VOLUME: u32 = 0x10;
pub const NAU8822_REG_EQ1: u32 = 0x12;
pub const NAU8822_REG_EQ2: u32 = 0x13;
pub const NAU8822_REG_EQ3: u32 = 0x14;
pub const NAU8822_REG_EQ4: u32 = 0x15;
pub const NAU8822_REG_EQ5: u32 = 0x16;
pub const NAU8822_REG_DAC_LIMITER_1: u32 = 0x18;
pub const NAU8822_REG_DAC_LIMITER_2: u32 = 0x19;
pub const NAU8822_REG_NOTCH_FILTER_1: u32 = 0x1B;
pub const NAU8822_REG_NOTCH_FILTER_2: u32 = 0x1C;
pub const NAU8822_REG_NOTCH_FILTER_3: u32 = 0x1D;
pub const NAU8822_REG_NOTCH_FILTER_4: u32 = 0x1E;
pub const NAU8822_REG_ALC_CONTROL_1: u32 = 0x20;
pub const NAU8822_REG_ALC_CONTROL_2: u32 = 0x21;
pub const NAU8822_REG_ALC_CONTROL_3: u32 = 0x22;
pub const NAU8822_REG_NOISE_GATE: u32 = 0x23;
pub const NAU8822_REG_PLL_N: u32 = 0x24;
pub const NAU8822_REG_PLL_K1: u32 = 0x25;
pub const NAU8822_REG_PLL_K2: u32 = 0x26;
pub const NAU8822_REG_PLL_K3: u32 = 0x27;
pub const NAU8822_REG_3D_CONTROL: u32 = 0x29;
pub const NAU8822_REG_RIGHT_SPEAKER_CONTROL: u32 = 0x2B;
pub const NAU8822_REG_INPUT_CONTROL: u32 = 0x2C;
pub const NAU8822_REG_LEFT_INP_PGA_CONTROL: u32 = 0x2D;
pub const NAU8822_REG_RIGHT_INP_PGA_CONTROL: u32 = 0x2E;
pub const NAU8822_REG_LEFT_ADC_BOOST_CONTROL: u32 = 0x2F;
pub const NAU8822_REG_RIGHT_ADC_BOOST_CONTROL: u32 = 0x30;
pub const NAU8822_REG_OUTPUT_CONTROL: u32 = 0x31;
pub const NAU8822_REG_LEFT_MIXER_CONTROL: u32 = 0x32;
pub const NAU8822_REG_RIGHT_MIXER_CONTROL: u32 = 0x33;
pub const NAU8822_REG_LHP_VOLUME: u32 = 0x34;
pub const NAU8822_REG_RHP_VOLUME: u32 = 0x35;
pub const NAU8822_REG_LSPKOUT_VOLUME: u32 = 0x36;
pub const NAU8822_REG_RSPKOUT_VOLUME: u32 = 0x37;
pub const NAU8822_REG_AUX2_MIXER: u32 = 0x38;
pub const NAU8822_REG_AUX1_MIXER: u32 = 0x39;
pub const NAU8822_REG_POWER_MANAGEMENT_4: u32 = 0x3A;
pub const NAU8822_REG_LEFT_TIME_SLOT: u32 = 0x3B;
pub const NAU8822_REG_MISC: u32 = 0x3C;
pub const NAU8822_REG_RIGHT_TIME_SLOT: u32 = 0x3D;
pub const NAU8822_REG_DEVICE_REVISION: u32 = 0x3E;
pub const NAU8822_REG_DEVICE_ID: u32 = 0x3F;
pub const NAU8822_REG_DAC_DITHER: u32 = 0x41;
pub const NAU8822_REG_ALC_ENHANCE_1: u32 = 0x46;
pub const NAU8822_REG_ALC_ENHANCE_2: u32 = 0x47;
pub const NAU8822_REG_192KHZ_SAMPLING: u32 = 0x48;
pub const NAU8822_REG_MISC_CONTROL: u32 = 0x49;
pub const NAU8822_REG_INPUT_TIEOFF: u32 = 0x4A;
pub const NAU8822_REG_POWER_REDUCTION: u32 = 0x4B;
pub const NAU8822_REG_AGC_PEAK2PEAK: u32 = 0x4C;
pub const NAU8822_REG_AGC_PEAK_DETECT: u32 = 0x4D;
pub const NAU8822_REG_AUTOMUTE_CONTROL: u32 = 0x4E;
pub const NAU8822_REG_OUTPUT_TIEOFF: u32 = 0x4F;
pub const NAU8822_REG_MAX_REGISTER: u32 = NAU8822_REG_OUTPUT_TIEOFF;

/* NAU8822_REG_POWER_MANAGEMENT_1 (0x1) */
pub const NAU8822_REFIMP_MASK: u32 = 0x3;
pub const NAU8822_REFIMP_80K: u32 = 0x1;
pub const NAU8822_REFIMP_300K: u32 = 0x2;
pub const NAU8822_REFIMP_3K: u32 = 0x3;
pub const NAU8822_IOBUF_EN: u32 = 0x1 << 2;
pub const NAU8822_ABIAS_EN: u32 = 0x1 << 3;
pub const NAU8822_PLL_EN_MASK: u32 = 0x1 << 5;
pub const NAU8822_PLL_ON: u32 = 0x1 << 5;
pub const NAU8822_PLL_OFF: u32 = 0x0 << 5;

/* NAU8822_REG_AUDIO_INTERFACE (0x4) */
pub const NAU8822_AIFMT_MASK: u32 = 0x3 << 3;
pub const NAU8822_WLEN_MASK: u32 = 0x3 << 5;
pub const NAU8822_WLEN_20: u32 = 0x1 << 5;
pub const NAU8822_WLEN_24: u32 = 0x2 << 5;
pub const NAU8822_WLEN_32: u32 = 0x3 << 5;
pub const NAU8822_LRP_MASK: u32 = 0x1 << 7;
pub const NAU8822_BCLKP_MASK: u32 = 0x1 << 8;

/* NAU8822_REG_COMPANDING_CONTROL (0x5) */
pub const NAU8822_ADDAP_SFT: u32 = 0;
pub const NAU8822_ADCCM_SFT: u32 = 1;
pub const NAU8822_DACCM_SFT: u32 = 3;

/* NAU8822_REG_CLOCKING (0x6) */
pub const NAU8822_CLKIOEN_MASK: u32 = 0x1;
pub const NAU8822_CLK_MASTER: u32 = 0x1;
pub const NAU8822_CLK_SLAVE: u32 = 0x0;
pub const NAU8822_MCLKSEL_SFT: u32 = 5;
pub const NAU8822_MCLKSEL_MASK: u32 = 0x7 << 5;
pub const NAU8822_BCLKSEL_SFT: u32 = 2;
pub const NAU8822_BCLKSEL_MASK: u32 = 0x7 << 2;
pub const NAU8822_BCLKDIV_1: u32 = 0x0 << 2;
pub const NAU8822_BCLKDIV_2: u32 = 0x1 << 2;
pub const NAU8822_BCLKDIV_4: u32 = 0x2 << 2;
pub const NAU8822_BCLKDIV_8: u32 = 0x3 << 2;
pub const NAU8822_BCLKDIV_16: u32 = 0x4 << 2;
pub const NAU8822_CLKM_MASK: u32 = 0x1 << 8;
pub const NAU8822_CLKM_MCLK: u32 = 0x0 << 8;
pub const NAU8822_CLKM_PLL: u32 = 0x1 << 8;

/* NAU8822_REG_ADDITIONAL_CONTROL (0x08) */
pub const NAU8822_SMPLR_SFT: u32 = 1;
pub const NAU8822_SMPLR_MASK: u32 = 0x7 << 1;
pub const NAU8822_SMPLR_48K: u32 = 0x0 << 1;
pub const NAU8822_SMPLR_32K: u32 = 0x1 << 1;
pub const NAU8822_SMPLR_24K: u32 = 0x2 << 1;
pub const NAU8822_SMPLR_16K: u32 = 0x3 << 1;
pub const NAU8822_SMPLR_12K: u32 = 0x4 << 1;
pub const NAU8822_SMPLR_8K: u32 = 0x5 << 1;

/* NAU8822_REG_EQ1 (0x12) */
pub const NAU8822_EQ1GC_SFT: u32 = 0;
pub const NAU8822_EQ1CF_SFT: u32 = 5;
pub const NAU8822_EQM_SFT: u32 = 8;

/* NAU8822_REG_EQ2 (0x13) */
pub const NAU8822_EQ2GC_SFT: u32 = 0;
pub const NAU8822_EQ2CF_SFT: u32 = 5;
pub const NAU8822_EQ2BW_SFT: u32 = 8;

/* NAU8822_REG_EQ3 (0x14) */
pub const NAU8822_EQ3GC_SFT: u32 = 0;
pub const NAU8822_EQ3CF_SFT: u32 = 5;
pub const NAU8822_EQ3BW_SFT: u32 = 8;

/* NAU8822_REG_EQ4 (0x15) */
pub const NAU8822_EQ4GC_SFT: u32 = 0;
pub const NAU8822_EQ4CF_SFT: u32 = 5;
pub const NAU8822_EQ4BW_SFT: u32 = 8;

/* NAU8822_REG_EQ5 (0x16) */
pub const NAU8822_EQ5GC_SFT: u32 = 0;
pub const NAU8822_EQ5CF_SFT: u32 = 5;

/* NAU8822_REG_ALC_CONTROL_1 (0x20) */
pub const NAU8822_ALCMINGAIN_SFT: u32 = 0;
pub const NAU8822_ALCMXGAIN_SFT: u32 = 3;
pub const NAU8822_ALCEN_SFT: u32 = 7;

/* NAU8822_REG_ALC_CONTROL_2 (0x21) */
pub const NAU8822_ALCSL_SFT: u32 = 0;
pub const NAU8822_ALCHT_SFT: u32 = 4;

/* NAU8822_REG_ALC_CONTROL_3 (0x22) */
pub const NAU8822_ALCATK_SFT: u32 = 0;
pub const NAU8822_ALCDCY_SFT: u32 = 4;
pub const NAU8822_ALCM_SFT: u32 = 8;

/* NAU8822_REG_PLL_N (0x24) */
pub const NAU8822_PLLMCLK_DIV2: u32 = 0x1 << 4;
pub const NAU8822_PLLN_MASK: u32 = 0xF;

pub const NAU8822_PLLK1_SFT: u32 = 18;
pub const NAU8822_PLLK1_MASK: u32 = 0x3F;

/* NAU8822_REG_PLL_K2 (0x26) */
pub const NAU8822_PLLK2_SFT: u32 = 9;
pub const NAU8822_PLLK2_MASK: u32 = 0x1FF;

/* NAU8822_REG_PLL_K3 (0x27) */
pub const NAU8822_PLLK3_MASK: u32 = 0x1FF;

/* NAU8822_REG_RIGHT_SPEAKER_CONTROL (0x2B) */
pub const NAU8822_RMIXMUT: u32 = 0x20;
pub const NAU8822_RSUBBYP: u32 = 0x10;

pub const NAU8822_RAUXRSUBG_SFT: u32 = 1;
pub const NAU8822_RAUXRSUBG_MASK: u32 = 0x0E;

pub const NAU8822_RAUXSMUT: u32 = 0x01;

/* System Clock Source */
pub const NAU8822_CLK_MCLK: i32 = 0;
pub const NAU8822_CLK_PLL: i32 = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct nau8822_pll {
    pub pre_factor: i32,
    pub mclk_scaler: i32,
    pub pll_frac: i32,
    pub pll_int: i32,
    pub freq_in: i32,
    pub freq_out: i32,
}

pub const NAU8822_NUM_SUPPLIES: usize = 4;

/* External C dependency types from included kernel headers in the original repository. */
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

/* Codec Private Data */
#[repr(C)]
pub struct nau8822 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub mclk: *mut clk,
    pub pll: nau8822_pll,
    pub sysclk: i32,
    pub div_id: i32,
    pub supplies: [regulator_bulk_data; NAU8822_NUM_SUPPLIES],
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
