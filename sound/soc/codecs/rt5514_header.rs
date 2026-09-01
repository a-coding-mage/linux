/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt5514.h  --  RT5514 ALSA SoC audio driver
 *
 * Copyright 2015 Realtek Microelectronics
 * Author: Oder Chiou <oder_chiou@realtek.com>
 */

/* C header dependencies: <linux/clk.h>, <sound/rt5514.h> */

pub const RT5514_DEVICE_ID: u32 = 0x10ec5514;

pub const RT5514_RESET: u32 = 0x2000;
pub const RT5514_PWR_ANA1: u32 = 0x2004;
pub const RT5514_PWR_ANA2: u32 = 0x2008;
pub const RT5514_I2S_CTRL1: u32 = 0x2010;
pub const RT5514_I2S_CTRL2: u32 = 0x2014;
pub const RT5514_VAD_CTRL6: u32 = 0x2030;
pub const RT5514_EXT_VAD_CTRL: u32 = 0x206c;
pub const RT5514_DIG_IO_CTRL: u32 = 0x2070;
pub const RT5514_PAD_CTRL1: u32 = 0x2080;
pub const RT5514_DMIC_DATA_CTRL: u32 = 0x20a0;
pub const RT5514_DIG_SOURCE_CTRL: u32 = 0x20a4;
pub const RT5514_SRC_CTRL: u32 = 0x20ac;
pub const RT5514_DOWNFILTER2_CTRL1: u32 = 0x20d0;
pub const RT5514_PLL_SOURCE_CTRL: u32 = 0x2100;
pub const RT5514_CLK_CTRL1: u32 = 0x2104;
pub const RT5514_CLK_CTRL2: u32 = 0x2108;
pub const RT5514_PLL3_CALIB_CTRL1: u32 = 0x2110;
pub const RT5514_PLL3_CALIB_CTRL4: u32 = 0x2120;
pub const RT5514_PLL3_CALIB_CTRL5: u32 = 0x2124;
pub const RT5514_PLL3_CALIB_CTRL6: u32 = 0x2128;
pub const RT5514_DELAY_BUF_CTRL1: u32 = 0x2140;
pub const RT5514_DELAY_BUF_CTRL3: u32 = 0x2148;
pub const RT5514_ASRC_IN_CTRL1: u32 = 0x2180;
pub const RT5514_DOWNFILTER0_CTRL1: u32 = 0x2190;
pub const RT5514_DOWNFILTER0_CTRL2: u32 = 0x2194;
pub const RT5514_DOWNFILTER0_CTRL3: u32 = 0x2198;
pub const RT5514_DOWNFILTER1_CTRL1: u32 = 0x21a0;
pub const RT5514_DOWNFILTER1_CTRL2: u32 = 0x21a4;
pub const RT5514_DOWNFILTER1_CTRL3: u32 = 0x21a8;
pub const RT5514_ANA_CTRL_LDO10: u32 = 0x2200;
pub const RT5514_ANA_CTRL_LDO18_16: u32 = 0x2204;
pub const RT5514_ANA_CTRL_ADC12: u32 = 0x2210;
pub const RT5514_ANA_CTRL_ADC21: u32 = 0x2214;
pub const RT5514_ANA_CTRL_ADC22: u32 = 0x2218;
pub const RT5514_ANA_CTRL_ADC23: u32 = 0x221c;
pub const RT5514_ANA_CTRL_MICBST: u32 = 0x2220;
pub const RT5514_ANA_CTRL_ADCFED: u32 = 0x2224;
pub const RT5514_ANA_CTRL_INBUF: u32 = 0x2228;
pub const RT5514_ANA_CTRL_VREF: u32 = 0x222c;
pub const RT5514_ANA_CTRL_PLL3: u32 = 0x2240;
pub const RT5514_ANA_CTRL_PLL1_1: u32 = 0x2260;
pub const RT5514_ANA_CTRL_PLL1_2: u32 = 0x2264;
pub const RT5514_DMIC_LP_CTRL: u32 = 0x2e00;
pub const RT5514_MISC_CTRL_DSP: u32 = 0x2e04;
pub const RT5514_DSP_CTRL1: u32 = 0x2f00;
pub const RT5514_DSP_CTRL3: u32 = 0x2f08;
pub const RT5514_DSP_CTRL4: u32 = 0x2f10;
pub const RT5514_VENDOR_ID1: u32 = 0x2ff0;
pub const RT5514_VENDOR_ID2: u32 = 0x2ff4;

pub const RT5514_DSP_MAPPING: u32 = 0x18000000;

/* RT5514_PWR_ANA1 (0x2004) */
pub const RT5514_POW_LDO18_IN: u32 = 0x1 << 5;
pub const RT5514_POW_LDO18_IN_BIT: u32 = 5;
pub const RT5514_POW_LDO18_ADC: u32 = 0x1 << 4;
pub const RT5514_POW_LDO18_ADC_BIT: u32 = 4;
pub const RT5514_POW_LDO21: u32 = 0x1 << 3;
pub const RT5514_POW_LDO21_BIT: u32 = 3;
pub const RT5514_POW_BG_LDO18_IN: u32 = 0x1 << 2;
pub const RT5514_POW_BG_LDO18_IN_BIT: u32 = 2;
pub const RT5514_POW_BG_LDO21: u32 = 0x1 << 1;
pub const RT5514_POW_BG_LDO21_BIT: u32 = 1;

/* RT5514_PWR_ANA2 (0x2008) */
pub const RT5514_POW_PLL1: u32 = 0x1 << 18;
pub const RT5514_POW_PLL1_BIT: u32 = 18;
pub const RT5514_POW_PLL1_LDO: u32 = 0x1 << 16;
pub const RT5514_POW_PLL1_LDO_BIT: u32 = 16;
pub const RT5514_POW_BG_MBIAS: u32 = 0x1 << 15;
pub const RT5514_POW_BG_MBIAS_BIT: u32 = 15;
pub const RT5514_POW_MBIAS: u32 = 0x1 << 14;
pub const RT5514_POW_MBIAS_BIT: u32 = 14;
pub const RT5514_POW_VREF2: u32 = 0x1 << 13;
pub const RT5514_POW_VREF2_BIT: u32 = 13;
pub const RT5514_POW_VREF1: u32 = 0x1 << 12;
pub const RT5514_POW_VREF1_BIT: u32 = 12;
pub const RT5514_POWR_LDO16: u32 = 0x1 << 11;
pub const RT5514_POWR_LDO16_BIT: u32 = 11;
pub const RT5514_POWL_LDO16: u32 = 0x1 << 10;
pub const RT5514_POWL_LDO16_BIT: u32 = 10;
pub const RT5514_POW_ADC2: u32 = 0x1 << 9;
pub const RT5514_POW_ADC2_BIT: u32 = 9;
pub const RT5514_POW_INPUT_BUF: u32 = 0x1 << 8;
pub const RT5514_POW_INPUT_BUF_BIT: u32 = 8;
pub const RT5514_POW_ADC1_R: u32 = 0x1 << 7;
pub const RT5514_POW_ADC1_R_BIT: u32 = 7;
pub const RT5514_POW_ADC1_L: u32 = 0x1 << 6;
pub const RT5514_POW_ADC1_L_BIT: u32 = 6;
pub const RT5514_POW2_BSTR: u32 = 0x1 << 5;
pub const RT5514_POW2_BSTR_BIT: u32 = 5;
pub const RT5514_POW2_BSTL: u32 = 0x1 << 4;
pub const RT5514_POW2_BSTL_BIT: u32 = 4;
pub const RT5514_POW_BSTR: u32 = 0x1 << 3;
pub const RT5514_POW_BSTR_BIT: u32 = 3;
pub const RT5514_POW_BSTL: u32 = 0x1 << 2;
pub const RT5514_POW_BSTL_BIT: u32 = 2;
pub const RT5514_POW_ADCFEDR: u32 = 0x1 << 1;
pub const RT5514_POW_ADCFEDR_BIT: u32 = 1;
pub const RT5514_POW_ADCFEDL: u32 = 0x1 << 0;
pub const RT5514_POW_ADCFEDL_BIT: u32 = 0;

/* RT5514_I2S_CTRL1 (0x2010) */
pub const RT5514_TDM_MODE2: u32 = 0x1 << 30;
pub const RT5514_TDM_MODE2_SFT: u32 = 30;
pub const RT5514_TDM_MODE: u32 = 0x1 << 28;
pub const RT5514_TDM_MODE_SFT: u32 = 28;
pub const RT5514_I2S_LR_MASK: u32 = 0x1 << 26;
pub const RT5514_I2S_LR_SFT: u32 = 26;
pub const RT5514_I2S_LR_NOR: u32 = 0x0 << 26;
pub const RT5514_I2S_LR_INV: u32 = 0x1 << 26;
pub const RT5514_I2S_BP_MASK: u32 = 0x1 << 25;
pub const RT5514_I2S_BP_SFT: u32 = 25;
pub const RT5514_I2S_BP_NOR: u32 = 0x0 << 25;
pub const RT5514_I2S_BP_INV: u32 = 0x1 << 25;
pub const RT5514_I2S_DF_MASK: u32 = 0x7 << 16;
pub const RT5514_I2S_DF_SFT: u32 = 16;
pub const RT5514_I2S_DF_I2S: u32 = 0x0 << 16;
pub const RT5514_I2S_DF_LEFT: u32 = 0x1 << 16;
pub const RT5514_I2S_DF_PCM_A: u32 = 0x2 << 16;
pub const RT5514_I2S_DF_PCM_B: u32 = 0x3 << 16;
pub const RT5514_TDMSLOT_SEL_RX_MASK: u32 = 0x3 << 10;
pub const RT5514_TDMSLOT_SEL_RX_SFT: u32 = 10;
pub const RT5514_TDMSLOT_SEL_RX_4CH: u32 = 0x1 << 10;
pub const RT5514_TDMSLOT_SEL_RX_6CH: u32 = 0x2 << 10;
pub const RT5514_TDMSLOT_SEL_RX_8CH: u32 = 0x3 << 10;
pub const RT5514_CH_LEN_RX_MASK: u32 = 0x3 << 8;
pub const RT5514_CH_LEN_RX_SFT: u32 = 8;
pub const RT5514_CH_LEN_RX_16: u32 = 0x0 << 8;
pub const RT5514_CH_LEN_RX_20: u32 = 0x1 << 8;
pub const RT5514_CH_LEN_RX_24: u32 = 0x2 << 8;
pub const RT5514_CH_LEN_RX_32: u32 = 0x3 << 8;
pub const RT5514_TDMSLOT_SEL_TX_MASK: u32 = 0x3 << 6;
pub const RT5514_TDMSLOT_SEL_TX_SFT: u32 = 6;
pub const RT5514_TDMSLOT_SEL_TX_4CH: u32 = 0x1 << 6;
pub const RT5514_TDMSLOT_SEL_TX_6CH: u32 = 0x2 << 6;
pub const RT5514_TDMSLOT_SEL_TX_8CH: u32 = 0x3 << 6;
pub const RT5514_CH_LEN_TX_MASK: u32 = 0x3 << 4;
pub const RT5514_CH_LEN_TX_SFT: u32 = 4;
pub const RT5514_CH_LEN_TX_16: u32 = 0x0 << 4;
pub const RT5514_CH_LEN_TX_20: u32 = 0x1 << 4;
pub const RT5514_CH_LEN_TX_24: u32 = 0x2 << 4;
pub const RT5514_CH_LEN_TX_32: u32 = 0x3 << 4;
pub const RT5514_I2S_DL_MASK: u32 = 0x3 << 0;
pub const RT5514_I2S_DL_SFT: u32 = 0;
pub const RT5514_I2S_DL_16: u32 = 0x0 << 0;
pub const RT5514_I2S_DL_20: u32 = 0x1 << 0;
pub const RT5514_I2S_DL_24: u32 = 0x2 << 0;
pub const RT5514_I2S_DL_8: u32 = 0x3 << 0;

/* RT5514_I2S_CTRL2 (0x2014) */
pub const RT5514_TDM_DOCKING_MODE: u32 = 0x1 << 31;
pub const RT5514_TDM_DOCKING_MODE_SFT: u32 = 31;
pub const RT5514_TDM_DOCKING_VALID_CH_MASK: u32 = 0x1 << 29;
pub const RT5514_TDM_DOCKING_VALID_CH_SFT: u32 = 29;
pub const RT5514_TDM_DOCKING_VALID_CH2: u32 = 0x0 << 29;
pub const RT5514_TDM_DOCKING_VALID_CH4: u32 = 0x1 << 29;
pub const RT5514_TDM_DOCKING_START_MASK: u32 = 0x1 << 28;
pub const RT5514_TDM_DOCKING_START_SFT: u32 = 28;
pub const RT5514_TDM_DOCKING_START_SLOT0: u32 = 0x0 << 28;
pub const RT5514_TDM_DOCKING_START_SLOT4: u32 = 0x1 << 28;

/* RT5514_DIG_SOURCE_CTRL (0x20a4) */
pub const RT5514_AD1_DMIC_INPUT_SEL: u32 = 0x1 << 1;
pub const RT5514_AD1_DMIC_INPUT_SEL_SFT: u32 = 1;
pub const RT5514_AD0_DMIC_INPUT_SEL: u32 = 0x1 << 0;
pub const RT5514_AD0_DMIC_INPUT_SEL_SFT: u32 = 0;

/* RT5514_PLL_SOURCE_CTRL (0x2100) */
pub const RT5514_PLL_1_SEL_MASK: u32 = 0x7 << 12;
pub const RT5514_PLL_1_SEL_SFT: u32 = 12;
pub const RT5514_PLL_1_SEL_SCLK: u32 = 0x3 << 12;
pub const RT5514_PLL_1_SEL_MCLK: u32 = 0x4 << 12;

/* RT5514_CLK_CTRL1 (0x2104) */
pub const RT5514_CLK_AD_ANA1_EN: u32 = 0x1 << 31;
pub const RT5514_CLK_AD_ANA1_EN_BIT: u32 = 31;
pub const RT5514_CLK_AD1_EN: u32 = 0x1 << 24;
pub const RT5514_CLK_AD1_EN_BIT: u32 = 24;
pub const RT5514_CLK_AD0_EN: u32 = 0x1 << 23;
pub const RT5514_CLK_AD0_EN_BIT: u32 = 23;
pub const RT5514_CLK_DMIC_OUT_SEL_MASK: u32 = 0x7 << 8;
pub const RT5514_CLK_DMIC_OUT_SEL_SFT: u32 = 8;
pub const RT5514_CLK_AD_ANA1_SEL_MASK: u32 = 0xf << 0;
pub const RT5514_CLK_AD_ANA1_SEL_SFT: u32 = 0;

/* RT5514_CLK_CTRL2 (0x2108) */
pub const RT5514_CLK_AD1_ASRC_EN: u32 = 0x1 << 17;
pub const RT5514_CLK_AD1_ASRC_EN_BIT: u32 = 17;
pub const RT5514_CLK_AD0_ASRC_EN: u32 = 0x1 << 16;
pub const RT5514_CLK_AD0_ASRC_EN_BIT: u32 = 16;
pub const RT5514_CLK_SYS_DIV_OUT_MASK: u32 = 0x7 << 8;
pub const RT5514_CLK_SYS_DIV_OUT_SFT: u32 = 8;
pub const RT5514_SEL_ADC_OSR_MASK: u32 = 0x7 << 4;
pub const RT5514_SEL_ADC_OSR_SFT: u32 = 4;
pub const RT5514_CLK_SYS_PRE_SEL_MASK: u32 = 0x3 << 0;
pub const RT5514_CLK_SYS_PRE_SEL_SFT: u32 = 0;
pub const RT5514_CLK_SYS_PRE_SEL_MCLK: u32 = 0x2 << 0;
pub const RT5514_CLK_SYS_PRE_SEL_PLL: u32 = 0x3 << 0;

/*  RT5514_DOWNFILTER_CTRL (0x2190 0x2194 0x21a0 0x21a4) */
pub const RT5514_AD_DMIC_MIX: u32 = 0x1 << 11;
pub const RT5514_AD_DMIC_MIX_BIT: u32 = 11;
pub const RT5514_AD_AD_MIX: u32 = 0x1 << 10;
pub const RT5514_AD_AD_MIX_BIT: u32 = 10;
pub const RT5514_AD_AD_MUTE: u32 = 0x1 << 7;
pub const RT5514_AD_AD_MUTE_BIT: u32 = 7;
pub const RT5514_AD_GAIN_MASK: u32 = 0x3f << 1;
pub const RT5514_AD_GAIN_SFT: u32 = 1;

/*  RT5514_ANA_CTRL_MICBST (0x2220) */
pub const RT5514_SEL_BSTL_MASK: u32 = 0xf << 4;
pub const RT5514_SEL_BSTL_SFT: u32 = 4;
pub const RT5514_SEL_BSTR_MASK: u32 = 0xf << 0;
pub const RT5514_SEL_BSTR_SFT: u32 = 0;

/*  RT5514_ANA_CTRL_PLL1_1 (0x2260) */
pub const RT5514_PLL_K_MAX: u32 = 0x1f;
pub const RT5514_PLL_K_MASK: u32 = RT5514_PLL_K_MAX << 16;
pub const RT5514_PLL_K_SFT: u32 = 16;
pub const RT5514_PLL_N_MAX: u32 = 0x1ff;
pub const RT5514_PLL_N_MASK: u32 = RT5514_PLL_N_MAX << 7;
pub const RT5514_PLL_N_SFT: u32 = 4;
pub const RT5514_PLL_M_MAX: u32 = 0xf;
pub const RT5514_PLL_M_MASK: u32 = RT5514_PLL_M_MAX << 0;
pub const RT5514_PLL_M_SFT: u32 = 0;

/*  RT5514_ANA_CTRL_PLL1_2 (0x2264) */
pub const RT5514_PLL_M_BP: u32 = 0x1 << 2;
pub const RT5514_PLL_M_BP_SFT: u32 = 2;
pub const RT5514_PLL_K_BP: u32 = 0x1 << 1;
pub const RT5514_PLL_K_BP_SFT: u32 = 1;
pub const RT5514_EN_LDO_PLL1: u32 = 0x1 << 0;
pub const RT5514_EN_LDO_PLL1_BIT: u32 = 0;

pub const RT5514_PLL_INP_MAX: u32 = 40000000;
pub const RT5514_PLL_INP_MIN: u32 = 256000;

pub const RT5514_FIRMWARE1: &str = "rt5514_dsp_fw1.bin";
pub const RT5514_FIRMWARE2: &str = "rt5514_dsp_fw2.bin";

/* System Clock Source */
pub const RT5514_SCLK_S_MCLK: u32 = 0;
pub const RT5514_SCLK_S_PLL1: u32 = 1;

/* PLL1 Source */
pub const RT5514_PLL1_S_MCLK: u32 = 0;
pub const RT5514_PLL1_S_BCLK: u32 = 1;

#[repr(C)]
pub struct rt5514_platform_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
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

#[repr(C)]
pub struct rt5514_priv {
    pub pdata: rt5514_platform_data,
    pub component: *mut snd_soc_component,
    pub i2c_regmap: *mut regmap,
    pub regmap: *mut regmap,
    pub mclk: *mut clk,
    pub dsp_calib_clk: *mut clk,
    pub sysclk: i32,
    pub sysclk_src: i32,
    pub lrck: i32,
    pub bclk: i32,
    pub pll_src: i32,
    pub pll_in: i32,
    pub pll_out: i32,
    pub dsp_enabled: i32,
    pub pll3_cal_value: u32,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
