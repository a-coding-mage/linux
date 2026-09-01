/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * NAU85L40 ALSA SoC audio driver
 *
 * Copyright 2016 Nuvoton Technology Corp.
 * Author: John Hsu <KCHSU0@nuvoton.com>
 */

pub const NAU8540_REG_SW_RESET: u32 = 0x00;
pub const NAU8540_REG_POWER_MANAGEMENT: u32 = 0x01;
pub const NAU8540_REG_CLOCK_CTRL: u32 = 0x02;
pub const NAU8540_REG_CLOCK_SRC: u32 = 0x03;
pub const NAU8540_REG_FLL1: u32 = 0x04;
pub const NAU8540_REG_FLL2: u32 = 0x05;
pub const NAU8540_REG_FLL3: u32 = 0x06;
pub const NAU8540_REG_FLL4: u32 = 0x07;
pub const NAU8540_REG_FLL5: u32 = 0x08;
pub const NAU8540_REG_FLL6: u32 = 0x09;
pub const NAU8540_REG_FLL_VCO_RSV: u32 = 0x0A;
pub const NAU8540_REG_PCM_CTRL0: u32 = 0x10;
pub const NAU8540_REG_PCM_CTRL1: u32 = 0x11;
pub const NAU8540_REG_PCM_CTRL2: u32 = 0x12;
pub const NAU8540_REG_PCM_CTRL3: u32 = 0x13;
pub const NAU8540_REG_PCM_CTRL4: u32 = 0x14;
pub const NAU8540_REG_ALC_CONTROL_1: u32 = 0x20;
pub const NAU8540_REG_ALC_CONTROL_2: u32 = 0x21;
pub const NAU8540_REG_ALC_CONTROL_3: u32 = 0x22;
pub const NAU8540_REG_ALC_CONTROL_4: u32 = 0x23;
pub const NAU8540_REG_ALC_CONTROL_5: u32 = 0x24;
pub const NAU8540_REG_ALC_GAIN_CH12: u32 = 0x2D;
pub const NAU8540_REG_ALC_GAIN_CH34: u32 = 0x2E;
pub const NAU8540_REG_ALC_STATUS: u32 = 0x2F;
pub const NAU8540_REG_NOTCH_FIL1_CH1: u32 = 0x30;
pub const NAU8540_REG_NOTCH_FIL2_CH1: u32 = 0x31;
pub const NAU8540_REG_NOTCH_FIL1_CH2: u32 = 0x32;
pub const NAU8540_REG_NOTCH_FIL2_CH2: u32 = 0x33;
pub const NAU8540_REG_NOTCH_FIL1_CH3: u32 = 0x34;
pub const NAU8540_REG_NOTCH_FIL2_CH3: u32 = 0x35;
pub const NAU8540_REG_NOTCH_FIL1_CH4: u32 = 0x36;
pub const NAU8540_REG_NOTCH_FIL2_CH4: u32 = 0x37;
pub const NAU8540_REG_HPF_FILTER_CH12: u32 = 0x38;
pub const NAU8540_REG_HPF_FILTER_CH34: u32 = 0x39;
pub const NAU8540_REG_ADC_SAMPLE_RATE: u32 = 0x3A;
pub const NAU8540_REG_DIGITAL_GAIN_CH1: u32 = 0x40;
pub const NAU8540_REG_DIGITAL_GAIN_CH2: u32 = 0x41;
pub const NAU8540_REG_DIGITAL_GAIN_CH3: u32 = 0x42;
pub const NAU8540_REG_DIGITAL_GAIN_CH4: u32 = 0x43;
pub const NAU8540_REG_DIGITAL_MUX: u32 = 0x44;
pub const NAU8540_REG_P2P_CH1: u32 = 0x48;
pub const NAU8540_REG_P2P_CH2: u32 = 0x49;
pub const NAU8540_REG_P2P_CH3: u32 = 0x4A;
pub const NAU8540_REG_P2P_CH4: u32 = 0x4B;
pub const NAU8540_REG_PEAK_CH1: u32 = 0x4C;
pub const NAU8540_REG_PEAK_CH2: u32 = 0x4D;
pub const NAU8540_REG_PEAK_CH3: u32 = 0x4E;
pub const NAU8540_REG_PEAK_CH4: u32 = 0x4F;
pub const NAU8540_REG_GPIO_CTRL: u32 = 0x50;
pub const NAU8540_REG_MISC_CTRL: u32 = 0x51;
pub const NAU8540_REG_I2C_CTRL: u32 = 0x52;
pub const NAU8540_REG_I2C_DEVICE_ID: u32 = 0x58;
pub const NAU8540_REG_RST: u32 = 0x5A;
pub const NAU8540_REG_VMID_CTRL: u32 = 0x60;
pub const NAU8540_REG_MUTE: u32 = 0x61;
pub const NAU8540_REG_ANALOG_ADC1: u32 = 0x64;
pub const NAU8540_REG_ANALOG_ADC2: u32 = 0x65;
pub const NAU8540_REG_ANALOG_PWR: u32 = 0x66;
pub const NAU8540_REG_MIC_BIAS: u32 = 0x67;
pub const NAU8540_REG_REFERENCE: u32 = 0x68;
pub const NAU8540_REG_FEPGA1: u32 = 0x69;
pub const NAU8540_REG_FEPGA2: u32 = 0x6A;
pub const NAU8540_REG_FEPGA3: u32 = 0x6B;
pub const NAU8540_REG_FEPGA4: u32 = 0x6C;
pub const NAU8540_REG_PWR: u32 = 0x6D;
pub const NAU8540_REG_MAX: u32 = NAU8540_REG_PWR;

/* POWER_MANAGEMENT (0x01) */
pub const NAU8540_ADC_ALL_EN: u32 = 0xf;
pub const NAU8540_ADC4_EN: u32 = 0x1 << 3;
pub const NAU8540_ADC3_EN: u32 = 0x1 << 2;
pub const NAU8540_ADC2_EN: u32 = 0x1 << 1;
pub const NAU8540_ADC1_EN: u32 = 0x1;

/* CLOCK_CTRL (0x02) */
pub const NAU8540_CLK_ADC_EN: u32 = 0x1 << 15;
pub const NAU8540_CLK_AGC_EN: u32 = 0x1 << 3;
pub const NAU8540_CLK_I2S_EN: u32 = 0x1 << 1;

/* CLOCK_SRC (0x03) */
pub const NAU8540_CLK_SRC_SFT: u32 = 15;
pub const NAU8540_CLK_SRC_MASK: u32 = 1 << NAU8540_CLK_SRC_SFT;
pub const NAU8540_CLK_SRC_VCO: u32 = 1 << NAU8540_CLK_SRC_SFT;
pub const NAU8540_CLK_SRC_MCLK: u32 = 0 << NAU8540_CLK_SRC_SFT;
pub const NAU8540_CLK_ADC_SRC_SFT: u32 = 6;
pub const NAU8540_CLK_ADC_SRC_MASK: u32 = 0x3 << NAU8540_CLK_ADC_SRC_SFT;
pub const NAU8540_CLK_MCLK_SRC_MASK: u32 = 0xf;

/* FLL1 (0x04) */
pub const NAU8540_ICTRL_LATCH_SFT: u32 = 10;
pub const NAU8540_ICTRL_LATCH_MASK: u32 = 0x7 << NAU8540_ICTRL_LATCH_SFT;
pub const NAU8540_FLL_RATIO_MASK: u32 = 0x7f;

/* FLL3 (0x06) */
pub const NAU8540_GAIN_ERR_SFT: u32 = 12;
pub const NAU8540_GAIN_ERR_MASK: u32 = 0xf << NAU8540_GAIN_ERR_SFT;
pub const NAU8540_FLL_CLK_SRC_SFT: u32 = 10;
pub const NAU8540_FLL_CLK_SRC_MASK: u32 = 0x3 << NAU8540_FLL_CLK_SRC_SFT;
pub const NAU8540_FLL_CLK_SRC_MCLK: u32 = 0 << NAU8540_FLL_CLK_SRC_SFT;
pub const NAU8540_FLL_CLK_SRC_BLK: u32 = 0x2 << NAU8540_FLL_CLK_SRC_SFT;
pub const NAU8540_FLL_CLK_SRC_FS: u32 = 0x3 << NAU8540_FLL_CLK_SRC_SFT;
pub const NAU8540_FLL_INTEGER_MASK: u32 = 0x3ff;

/* FLL4 (0x07) */
pub const NAU8540_FLL_REF_DIV_SFT: u32 = 10;
pub const NAU8540_FLL_REF_DIV_MASK: u32 = 0x3 << NAU8540_FLL_REF_DIV_SFT;

/* FLL5 (0x08) */
pub const NAU8540_FLL_PDB_DAC_EN: u32 = 0x1 << 15;
pub const NAU8540_FLL_LOOP_FTR_EN: u32 = 0x1 << 14;
pub const NAU8540_FLL_CLK_SW_MASK: u32 = 0x1 << 13;
pub const NAU8540_FLL_CLK_SW_N2: u32 = 0x1 << 13;
pub const NAU8540_FLL_CLK_SW_REF: u32 = 0x0 << 13;
pub const NAU8540_FLL_FTR_SW_MASK: u32 = 0x1 << 12;
pub const NAU8540_FLL_FTR_SW_ACCU: u32 = 0x1 << 12;
pub const NAU8540_FLL_FTR_SW_FILTER: u32 = 0x0 << 12;

/* FLL6 (0x9) */
pub const NAU8540_DCO_EN: u32 = 0x1 << 15;
pub const NAU8540_SDM_EN: u32 = 0x1 << 14;
pub const NAU8540_CUTOFF500: u32 = 0x1 << 13;

/* PCM_CTRL0 (0x10) */
pub const NAU8540_I2S_BP_SFT: u32 = 7;
pub const NAU8540_I2S_BP_INV: u32 = 0x1 << NAU8540_I2S_BP_SFT;
pub const NAU8540_I2S_PCMB_SFT: u32 = 6;
pub const NAU8540_I2S_PCMB_EN: u32 = 0x1 << NAU8540_I2S_PCMB_SFT;
pub const NAU8540_I2S_DL_SFT: u32 = 2;
pub const NAU8540_I2S_DL_MASK: u32 = 0x3 << NAU8540_I2S_DL_SFT;
pub const NAU8540_I2S_DL_16: u32 = 0 << NAU8540_I2S_DL_SFT;
pub const NAU8540_I2S_DL_20: u32 = 0x1 << NAU8540_I2S_DL_SFT;
pub const NAU8540_I2S_DL_24: u32 = 0x2 << NAU8540_I2S_DL_SFT;
pub const NAU8540_I2S_DL_32: u32 = 0x3 << NAU8540_I2S_DL_SFT;
pub const NAU8540_I2S_DF_MASK: u32 = 0x3;
pub const NAU8540_I2S_DF_RIGTH: u32 = 0;
pub const NAU8540_I2S_DF_LEFT: u32 = 0x1;
pub const NAU8540_I2S_DF_I2S: u32 = 0x2;
pub const NAU8540_I2S_DF_PCM_AB: u32 = 0x3;

/* PCM_CTRL1 (0x11) */
pub const NAU8540_I2S_DO12_TRI: u32 = 0x1 << 15;
pub const NAU8540_I2S_LRC_DIV_SFT: u32 = 12;
pub const NAU8540_I2S_LRC_DIV_MASK: u32 = 0x3 << NAU8540_I2S_LRC_DIV_SFT;
pub const NAU8540_I2S_DO12_OE: u32 = 0x1 << 4;
pub const NAU8540_I2S_MS_SFT: u32 = 3;
pub const NAU8540_I2S_MS_MASK: u32 = 0x1 << NAU8540_I2S_MS_SFT;
pub const NAU8540_I2S_MS_MASTER: u32 = 0x1 << NAU8540_I2S_MS_SFT;
pub const NAU8540_I2S_MS_SLAVE: u32 = 0x0 << NAU8540_I2S_MS_SFT;
pub const NAU8540_I2S_BLK_DIV_MASK: u32 = 0x7;

/* PCM_CTRL1 (0x12) */
pub const NAU8540_I2S_DO34_TRI: u32 = 0x1 << 15;
pub const NAU8540_I2S_DO34_OE: u32 = 0x1 << 11;
pub const NAU8540_I2S_TSLOT_L_MASK: u32 = 0x3ff;

/* PCM_CTRL4 (0x14) */
pub const NAU8540_TDM_MODE: u32 = 0x1 << 15;
pub const NAU8540_TDM_OFFSET_EN: u32 = 0x1 << 14;
pub const NAU8540_TDM_TX_MASK: u32 = 0xf;

/* ALC_CONTROL_3 (0x22) */
pub const NAU8540_ALC_CH1_EN: u32 = 0x1 << 12;
pub const NAU8540_ALC_CH2_EN: u32 = 0x1 << 13;
pub const NAU8540_ALC_CH3_EN: u32 = 0x1 << 14;
pub const NAU8540_ALC_CH4_EN: u32 = 0x1 << 15;
pub const NAU8540_ALC_CH_ALL_EN: u32 = 0xf << 12;

/* ADC_SAMPLE_RATE (0x3A) */
pub const NAU8540_CH_SYNC: u32 = 0x1 << 14;
pub const NAU8540_ADC_OSR_MASK: u32 = 0x3;
pub const NAU8540_ADC_OSR_256: u32 = 0x3;
pub const NAU8540_ADC_OSR_128: u32 = 0x2;
pub const NAU8540_ADC_OSR_64: u32 = 0x1;
pub const NAU8540_ADC_OSR_32: u32 = 0x0;

/* VMID_CTRL (0x60) */
pub const NAU8540_VMID_EN: u32 = 1 << 6;
pub const NAU8540_VMID_SEL_SFT: u32 = 4;
pub const NAU8540_VMID_SEL_MASK: u32 = 0x3 << NAU8540_VMID_SEL_SFT;

/* MUTE (0x61) */
pub const NAU8540_PGA_CH1_MUTE: u32 = 0x1;
pub const NAU8540_PGA_CH2_MUTE: u32 = 0x2;
pub const NAU8540_PGA_CH3_MUTE: u32 = 0x4;
pub const NAU8540_PGA_CH4_MUTE: u32 = 0x8;
pub const NAU8540_PGA_CH_ALL_MUTE: u32 = 0xf;

/* MIC_BIAS (0x67) */
pub const NAU8540_PU_PRE: u32 = 0x1 << 8;

/* REFERENCE (0x68) */
pub const NAU8540_PRECHARGE_DIS: u32 = 0x1 << 13;
pub const NAU8540_GLOBAL_BIAS_EN: u32 = 0x1 << 12;
pub const NAU8540_DISCHRG_EN: u32 = 0x1 << 11;

/* FEPGA1 (0x69) */
pub const NAU8540_FEPGA1_MODCH2_SHT_SFT: u32 = 7;
pub const NAU8540_FEPGA1_MODCH2_SHT: u32 = 0x1 << NAU8540_FEPGA1_MODCH2_SHT_SFT;
pub const NAU8540_FEPGA1_MODCH1_SHT_SFT: u32 = 3;
pub const NAU8540_FEPGA1_MODCH1_SHT: u32 = 0x1 << NAU8540_FEPGA1_MODCH1_SHT_SFT;

/* FEPGA2 (0x6A) */
pub const NAU8540_FEPGA2_MODCH4_SHT_SFT: u32 = 7;
pub const NAU8540_FEPGA2_MODCH4_SHT: u32 = 0x1 << NAU8540_FEPGA2_MODCH4_SHT_SFT;
pub const NAU8540_FEPGA2_MODCH3_SHT_SFT: u32 = 3;
pub const NAU8540_FEPGA2_MODCH3_SHT: u32 = 0x1 << NAU8540_FEPGA2_MODCH3_SHT_SFT;
pub const NAU8540_ACDC_CTL_SFT: u32 = 8;
pub const NAU8540_ACDC_CTL_MASK: u32 = 0xff << NAU8540_ACDC_CTL_SFT;
pub const NAU8540_ACDC_CTL_MIC4N_VREF: u32 = 0x1 << 15;
pub const NAU8540_ACDC_CTL_MIC4P_VREF: u32 = 0x1 << 14;
pub const NAU8540_ACDC_CTL_MIC3N_VREF: u32 = 0x1 << 13;
pub const NAU8540_ACDC_CTL_MIC3P_VREF: u32 = 0x1 << 12;
pub const NAU8540_ACDC_CTL_MIC2N_VREF: u32 = 0x1 << 11;
pub const NAU8540_ACDC_CTL_MIC2P_VREF: u32 = 0x1 << 10;
pub const NAU8540_ACDC_CTL_MIC1N_VREF: u32 = 0x1 << 9;
pub const NAU8540_ACDC_CTL_MIC1P_VREF: u32 = 0x1 << 8;

/* System Clock Source */
pub const NAU8540_CLK_DIS: u32 = 0;
pub const NAU8540_CLK_MCLK: u32 = 1;
pub const NAU8540_CLK_INTERNAL: u32 = 2;
pub const NAU8540_CLK_FLL_MCLK: u32 = 3;
pub const NAU8540_CLK_FLL_BLK: u32 = 4;
pub const NAU8540_CLK_FLL_FS: u32 = 5;

#[repr(C)]
pub struct nau8540 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
}

#[repr(C)]
pub struct nau8540_fll {
    pub mclk_src: ::core::ffi::c_int,
    pub ratio: ::core::ffi::c_int,
    pub fll_frac: ::core::ffi::c_int,
    pub fll_int: ::core::ffi::c_int,
    pub clk_ref_div: ::core::ffi::c_int,
}

#[repr(C)]
pub struct nau8540_fll_attr {
    pub param: ::core::ffi::c_uint,
    pub val: ::core::ffi::c_uint,
}

/* over sampling rate */
#[repr(C)]
pub struct nau8540_osr_attr {
    pub osr: ::core::ffi::c_uint,
    pub clk_src: ::core::ffi::c_uint,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
