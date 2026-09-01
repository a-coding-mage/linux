/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt1318.h -- Platform data for RT1318
 *
 * Copyright 2024 Realtek Semiconductor Corp.
 */
/* C dependency: <sound/rt1318.h> */

#[repr(C)]
pub struct rt1318_priv {
    pub component: *mut snd_soc_component,
    pub pdata: rt1318_platform_data,
    pub cali_work: work_struct,
    pub regmap: *mut regmap,

    pub r0_l_integer: u32,
    pub r0_l_factor: u32,
    pub r0_r_integer: u32,
    pub r0_r_factor: u32,
    pub rt1318_init: i32,
    pub rt1318_dvol: i32,
    pub sysclk_src: i32,
    pub sysclk: i32,
    pub lrck: i32,
    pub bclk: i32,
    pub master: i32,
    pub pll_src: i32,
    pub pll_in: i32,
    pub pll_out: i32,
}

pub const RT1318_PLL_INP_MAX: u32 = 40000000;
pub const RT1318_PLL_INP_MIN: u32 = 256000;
pub const RT1318_PLL_N_MAX: u32 = 0x1ff;
pub const RT1318_PLL_K_MAX: u32 = 0x1f;
pub const RT1318_PLL_M_MAX: u32 = 0x1f;

pub const RT1318_LRCLK_192000: u32 = 192000;
pub const RT1318_LRCLK_96000: u32 = 96000;
pub const RT1318_LRCLK_48000: u32 = 48000;
pub const RT1318_LRCLK_44100: u32 = 44100;
pub const RT1318_LRCLK_16000: u32 = 16000;
pub const RT1318_DVOL_STEP: u32 = 383;

pub const RT1318_CLK1: u32 = 0xc001;
pub const RT1318_CLK2: u32 = 0xc003;
pub const RT1318_CLK3: u32 = 0xc004;
pub const RT1318_CLK4: u32 = 0xc005;
pub const RT1318_CLK5: u32 = 0xc006;
pub const RT1318_CLK6: u32 = 0xc007;
pub const RT1318_CLK7: u32 = 0xc008;
pub const RT1318_PWR_STA1: u32 = 0xc121;
pub const RT1318_SPK_VOL_TH: u32 = 0xc130;
pub const RT1318_TCON: u32 = 0xc203;
pub const RT1318_SRC_TCON: u32 = 0xc204;
pub const RT1318_TCON_RELATE: u32 = 0xc206;
pub const RT1318_DA_VOL_L_8: u32 = 0xc20b;
pub const RT1318_DA_VOL_L_1_7: u32 = 0xc20c;
pub const RT1318_DA_VOL_R_8: u32 = 0xc20d;
pub const RT1318_DA_VOL_R_1_7: u32 = 0xc20e;
pub const RT1318_FEEDBACK_PATH: u32 = 0xc321;
pub const RT1318_STP_TEMP_L: u32 = 0xdb00;
pub const RT1318_STP_SEL_L: u32 = 0xdb08;
pub const RT1318_STP_R0_EN_L: u32 = 0xdb12;
pub const RT1318_R0_CMP_L_FLAG: u32 = 0xdb35;
pub const RT1318_PRE_R0_L_24: u32 = 0xdbb5;
pub const RT1318_PRE_R0_L_23_16: u32 = 0xdbb6;
pub const RT1318_PRE_R0_L_15_8: u32 = 0xdbb7;
pub const RT1318_PRE_R0_L_7_0: u32 = 0xdbb8;
pub const RT1318_R0_L_24: u32 = 0xdbc5;
pub const RT1318_R0_L_23_16: u32 = 0xdbc6;
pub const RT1318_R0_L_15_8: u32 = 0xdbc7;
pub const RT1318_R0_L_7_0: u32 = 0xdbc8;
pub const RT1318_STP_SEL_R: u32 = 0xdd08;
pub const RT1318_STP_R0_EN_R: u32 = 0xdd12;
pub const RT1318_R0_CMP_R_FLAG: u32 = 0xdd35;
pub const RT1318_PRE_R0_R_24: u32 = 0xddb5;
pub const RT1318_PRE_R0_R_23_16: u32 = 0xddb6;
pub const RT1318_PRE_R0_R_15_8: u32 = 0xddb7;
pub const RT1318_PRE_R0_R_7_0: u32 = 0xddb8;
pub const RT1318_R0_R_24: u32 = 0xddc5;
pub const RT1318_R0_R_23_16: u32 = 0xddc6;
pub const RT1318_R0_R_15_8: u32 = 0xddc7;
pub const RT1318_R0_R_7_0: u32 = 0xddc8;
pub const RT1318_DEV_ID1: u32 = 0xf012;
pub const RT1318_DEV_ID2: u32 = 0xf013;
pub const RT1318_PLL1_K: u32 = 0xf20d;
pub const RT1318_PLL1_M: u32 = 0xf20f;
pub const RT1318_PLL1_N_8: u32 = 0xf211;
pub const RT1318_PLL1_N_7_0: u32 = 0xf212;
pub const RT1318_SINE_GEN0: u32 = 0xf800;
pub const RT1318_TDM_CTRL1: u32 = 0xf900;
pub const RT1318_TDM_CTRL2: u32 = 0xf901;
pub const RT1318_TDM_CTRL3: u32 = 0xf902;
pub const RT1318_TDM_CTRL9: u32 = 0xf908;

/* Clock-1  (0xC001) */
pub const RT1318_PLLIN_MASK: u32 = 0x7 << 4;
pub const RT1318_PLLIN_BCLK0: u32 = 0x0 << 4;
pub const RT1318_PLLIN_BCLK1: u32 = 0x1 << 4;
pub const RT1318_PLLIN_RC: u32 = 0x2 << 4;
pub const RT1318_PLLIN_MCLK: u32 = 0x3 << 4;
pub const RT1318_PLLIN_SDW1: u32 = 0x4 << 4;
pub const RT1318_PLLIN_SDW2: u32 = 0x5 << 4;
pub const RT1318_PLLIN_SDW3: u32 = 0x6 << 4;
pub const RT1318_PLLIN_SDW4: u32 = 0x7 << 4;
pub const RT1318_SYSCLK_SEL_MASK: u32 = 0x7 << 0;
pub const RT1318_SYSCLK_BCLK: u32 = 0x0 << 0;
pub const RT1318_SYSCLK_SDW: u32 = 0x1 << 0;
pub const RT1318_SYSCLK_PLL2F: u32 = 0x2 << 0;
pub const RT1318_SYSCLK_PLL2B: u32 = 0x3 << 0;
pub const RT1318_SYSCLK_MCLK: u32 = 0x4 << 0;
pub const RT1318_SYSCLK_RC1: u32 = 0x5 << 0;
pub const RT1318_SYSCLK_RC2: u32 = 0x6 << 0;
pub const RT1318_SYSCLK_RC3: u32 = 0x7 << 0;
/* Clock-2  (0xC003) */
pub const RT1318_DIV_AP_MASK: u32 = 0x3 << 4;
pub const RT1318_DIV_AP_SFT: u32 = 4;
pub const RT1318_DIV_AP_DIV1: u32 = 0x0 << 4;
pub const RT1318_DIV_AP_DIV2: u32 = 0x1 << 4;
pub const RT1318_DIV_AP_DIV4: u32 = 0x2 << 4;
pub const RT1318_DIV_AP_DIV8: u32 = 0x3 << 4;
pub const RT1318_DIV_DAMOD_MASK: u32 = 0x3 << 0;
pub const RT1318_DIV_DAMOD_SFT: u32 = 0;
pub const RT1318_DIV_DAMOD_DIV1: u32 = 0x0 << 0;
pub const RT1318_DIV_DAMOD_DIV2: u32 = 0x1 << 0;
pub const RT1318_DIV_DAMOD_DIV4: u32 = 0x2 << 0;
pub const RT1318_DIV_DAMOD_DIV8: u32 = 0x3 << 0;
/* Clock-3  (0xC004) */
pub const RT1318_AD_STO1_MASK: u32 = 0x7 << 4;
pub const RT1318_AD_STO1_SFT: u32 = 4;
pub const RT1318_AD_STO1_DIV1: u32 = 0x0 << 4;
pub const RT1318_AD_STO1_DIV2: u32 = 0x1 << 4;
pub const RT1318_AD_STO1_DIV4: u32 = 0x2 << 4;
pub const RT1318_AD_STO1_DIV8: u32 = 0x3 << 4;
pub const RT1318_AD_STO1_DIV16: u32 = 0x4 << 4;
pub const RT1318_AD_STO2_MASK: u32 = 0x7 << 0;
pub const RT1318_AD_STO2_SFT: u32 = 0;
pub const RT1318_AD_STO2_DIV1: u32 = 0x0 << 0;
pub const RT1318_AD_STO2_DIV2: u32 = 0x1 << 0;
pub const RT1318_AD_STO2_DIV4: u32 = 0x2 << 0;
pub const RT1318_AD_STO2_DIV8: u32 = 0x3 << 0;
pub const RT1318_AD_STO2_DIV16: u32 = 0x4 << 0;
/* Clock-4  (0xC005) */
pub const RT1318_AD_ANA_STO1_MASK: u32 = 0x7 << 4;
pub const RT1318_AD_ANA_STO1_SFT: u32 = 4;
pub const RT1318_AD_ANA_STO1_DIV1: u32 = 0x0 << 4;
pub const RT1318_AD_ANA_STO1_DIV2: u32 = 0x1 << 4;
pub const RT1318_AD_ANA_STO1_DIV4: u32 = 0x2 << 4;
pub const RT1318_AD_ANA_STO1_DIV8: u32 = 0x3 << 4;
pub const RT1318_AD_ANA_STO1_DIV16: u32 = 0x4 << 4;
pub const RT1318_AD_ANA_STO2_MASK: u32 = 0x7 << 0;
pub const RT1318_AD_ANA_STO2_DIV1: u32 = 0x0 << 0;
pub const RT1318_AD_ANA_STO2_DIV2: u32 = 0x1 << 0;
pub const RT1318_AD_ANA_STO2_DIV4: u32 = 0x2 << 0;
pub const RT1318_AD_ANA_STO2_DIV8: u32 = 0x3 << 0;
pub const RT1318_AD_ANA_STO2_DIV16: u32 = 0x4 << 0;
pub const RT1318_AD_ANA_STO2_SFT: u32 = 0;
/* Clock-5  (0xC006) */
pub const RT1318_DIV_FIFO_IN_MASK: u32 = 0x3 << 4;
pub const RT1318_DIV_FIFO_IN_SFT: u32 = 4;
pub const RT1318_DIV_FIFO_IN_DIV1: u32 = 0x0 << 4;
pub const RT1318_DIV_FIFO_IN_DIV2: u32 = 0x1 << 4;
pub const RT1318_DIV_FIFO_IN_DIV4: u32 = 0x2 << 4;
pub const RT1318_DIV_FIFO_IN_DIV8: u32 = 0x3 << 4;
pub const RT1318_DIV_FIFO_OUT_MASK: u32 = 0x3 << 0;
pub const RT1318_DIV_FIFO_OUT_DIV1: u32 = 0x0 << 0;
pub const RT1318_DIV_FIFO_OUT_DIV2: u32 = 0x1 << 0;
pub const RT1318_DIV_FIFO_OUT_DIV4: u32 = 0x2 << 0;
pub const RT1318_DIV_FIFO_OUT_DIV8: u32 = 0x3 << 0;
pub const RT1318_DIV_FIFO_OUT_SFT: u32 = 0;
/* Clock-6  (0xC007) */
pub const RT1318_DIV_NLMS_MASK: u32 = 0x3 << 6;
pub const RT1318_DIV_NLMS_SFT: u32 = 6;
pub const RT1318_DIV_NLMS_DIV1: u32 = 0x0 << 6;
pub const RT1318_DIV_NLMS_DIV2: u32 = 0x1 << 6;
pub const RT1318_DIV_NLMS_DIV4: u32 = 0x2 << 6;
pub const RT1318_DIV_NLMS_DIV8: u32 = 0x3 << 6;
pub const RT1318_DIV_AD_MONO_MASK: u32 = 0x7 << 3;
pub const RT1318_DIV_AD_MONO_SFT: u32 = 3;
pub const RT1318_DIV_AD_MONO_DIV1: u32 = 0x0 << 3;
pub const RT1318_DIV_AD_MONO_DIV2: u32 = 0x1 << 3;
pub const RT1318_DIV_AD_MONO_DIV4: u32 = 0x2 << 3;
pub const RT1318_DIV_AD_MONO_DIV8: u32 = 0x3 << 3;
pub const RT1318_DIV_AD_MONO_DIV16: u32 = 0x4 << 3;
pub const RT1318_DIV_POST_G_MASK: u32 = 0x7 << 0;
pub const RT1318_DIV_POST_G_SFT: u32 = 0;
pub const RT1318_DIV_POST_G_DIV1: u32 = 0x0 << 0;
pub const RT1318_DIV_POST_G_DIV2: u32 = 0x1 << 0;
pub const RT1318_DIV_POST_G_DIV4: u32 = 0x2 << 0;
pub const RT1318_DIV_POST_G_DIV8: u32 = 0x3 << 0;
pub const RT1318_DIV_POST_G_DIV16: u32 = 0x4 << 0;
/* Power Status 1  (0xC121) */
pub const RT1318_PDB_CTRL_MASK: u32 = 0x1;
pub const RT1318_PDB_CTRL_LOW: u32 = 0x0;
pub const RT1318_PDB_CTRL_HIGH: u32 = 0x1;
pub const RT1318_PDB_CTRL_SFT: u32 = 0;
/* SRC Tcon(0xc204) */
pub const RT1318_SRCIN_IN_SEL_MASK: u32 = 0x3 << 6;
pub const RT1318_SRCIN_IN_48K: u32 = 0x0 << 6;
pub const RT1318_SRCIN_IN_44P1: u32 = 0x1 << 6;
pub const RT1318_SRCIN_IN_32K: u32 = 0x2 << 6;
pub const RT1318_SRCIN_IN_16K: u32 = 0x3 << 6;
pub const RT1318_SRCIN_F12288_MASK: u32 = 0x3 << 4;
pub const RT1318_SRCIN_TCON1: u32 = 0x0 << 4;
pub const RT1318_SRCIN_TCON2: u32 = 0x1 << 4;
pub const RT1318_SRCIN_TCON4: u32 = 0x2 << 4;
pub const RT1318_SRCIN_TCON8: u32 = 0x3 << 4;
pub const RT1318_SRCIN_DACLK_MASK: u32 = 0x3 << 2;
pub const RT1318_DACLK_TCON1: u32 = 0x0 << 2;
pub const RT1318_DACLK_TCON2: u32 = 0x1 << 2;
pub const RT1318_DACLK_TCON4: u32 = 0x2 << 2;
pub const RT1318_DACLK_TCON8: u32 = 0x3 << 2;
/* R0 Compare Flag  (0xDB35) */
pub const RT1318_R0_RANGE_MASK: u32 = 0x1;
pub const RT1318_R0_OUTOFRANGE: u32 = 0x0;
pub const RT1318_R0_INRANGE: u32 = 0x1;
/* PLL internal setting (0xF20D), K value */
pub const RT1318_K_PLL1_MASK: u32 = 0x1f << 0;
/* PLL internal setting (0xF20F), M value */
pub const RT1318_M_PLL1_MASK: u32 = 0x1f << 0;
/* PLL internal setting (0xF211), N_8 value */
pub const RT1318_N_8_PLL1_MASK: u32 = 0x1 << 0;
/* PLL internal setting (0xF212), N_7_0 value */
pub const RT1318_N_7_0_PLL1_MASK: u32 = 0xff << 0;
/* TDM CTRL 1  (0xf900) */
pub const RT1318_TDM_BCLK_MASK: u32 = 0x1 << 7;
pub const RT1318_TDM_BCLK_NORM: u32 = 0x0 << 7;
pub const RT1318_TDM_BCLK_INV: u32 = 0x1 << 7;
pub const RT1318_I2S_FMT_MASK: u32 = 0x7 << 0;
pub const RT1318_FMT_I2S: u32 = 0x0 << 0;
pub const RT1318_FMT_LEFT_J: u32 = 0x1 << 0;
pub const RT1318_FMT_PCM_A_R: u32 = 0x2 << 0;
pub const RT1318_FMT_PCM_B_R: u32 = 0x3 << 0;
pub const RT1318_FMT_PCM_A_F: u32 = 0x6 << 0;
pub const RT1318_FMT_PCM_B_F: u32 = 0x7 << 0;
pub const RT1318_I2S_FMT_SFT: u32 = 0;
/* TDM CTRL 2  (0xf901) */
pub const RT1318_I2S_CH_TX_MASK: u32 = 0x3 << 6;
pub const RT1318_I2S_CH_TX_2CH: u32 = 0x0 << 6;
pub const RT1318_I2S_CH_TX_4CH: u32 = 0x1 << 6;
pub const RT1318_I2S_CH_TX_6CH: u32 = 0x2 << 6;
pub const RT1318_I2S_CH_TX_8CH: u32 = 0x3 << 6;
pub const RT1318_I2S_CH_RX_MASK: u32 = 0x3 << 4;
pub const RT1318_I2S_CH_RX_2CH: u32 = 0x0 << 4;
pub const RT1318_I2S_CH_RX_4CH: u32 = 0x1 << 4;
pub const RT1318_I2S_CH_RX_6CH: u32 = 0x2 << 4;
pub const RT1318_I2S_CH_RX_8CH: u32 = 0x3 << 4;
pub const RT1318_I2S_DL_MASK: u32 = 0x7;
pub const RT1318_I2S_DL_SFT: u32 = 0;
pub const RT1318_I2S_DL_16: u32 = 0x0;
pub const RT1318_I2S_DL_20: u32 = 0x1;
pub const RT1318_I2S_DL_24: u32 = 0x2;
pub const RT1318_I2S_DL_32: u32 = 0x3;
pub const RT1318_I2S_DL_8: u32 = 0x4;
/* TDM CTRL 3  (0xf902) */
pub const RT1318_I2S_TX_CHL_MASK: u32 = 0x7 << 4;
pub const RT1318_I2S_TX_CHL_SFT: u32 = 4;
pub const RT1318_I2S_TX_CHL_16: u32 = 0x0 << 4;
pub const RT1318_I2S_TX_CHL_20: u32 = 0x1 << 4;
pub const RT1318_I2S_TX_CHL_24: u32 = 0x2 << 4;
pub const RT1318_I2S_TX_CHL_32: u32 = 0x3 << 4;
pub const RT1318_I2S_TX_CHL_8: u32 = 0x4 << 4;
pub const RT1318_I2S_RX_CHL_MASK: u32 = 0x7 << 0;
pub const RT1318_I2S_RX_CHL_SFT: u32 = 0;
pub const RT1318_I2S_RX_CHL_16: u32 = 0x0 << 0;
pub const RT1318_I2S_RX_CHL_20: u32 = 0x1 << 0;
pub const RT1318_I2S_RX_CHL_24: u32 = 0x2 << 0;
pub const RT1318_I2S_RX_CHL_32: u32 = 0x3 << 0;
pub const RT1318_I2S_RX_CHL_8: u32 = 0x4 << 0;
/* TDM CTRL 9  (0xf908) */
pub const RT1318_TDM_I2S_TX_L_DAC1_1_MASK: u32 = 0x7 << 4;
pub const RT1318_TDM_I2S_TX_R_DAC1_1_MASK: u32 = 0x7;
pub const RT1318_TDM_I2S_TX_L_DAC1_1_SFT: u32 = 4;
pub const RT1318_TDM_I2S_TX_R_DAC1_1_SFT: u32 = 0;

pub const RT1318_REG_DISP_LEN: u32 = 23;

/* System Clock Source */
pub const RT1318_SCLK_S_BCLK: i32 = 0;
pub const RT1318_SCLK_S_SDW: i32 = 1;
pub const RT1318_SCLK_S_PLL2F: i32 = 2;
pub const RT1318_SCLK_S_PLL2B: i32 = 3;
pub const RT1318_SCLK_S_MCLK: i32 = 4;
pub const RT1318_SCLK_S_RC0: i32 = 5;
pub const RT1318_SCLK_S_RC1: i32 = 6;
pub const RT1318_SCLK_S_RC2: i32 = 7;

/* PLL Source */
pub const RT1318_PLL_S_BCLK0: i32 = 0;
pub const RT1318_PLL_S_BCLK1: i32 = 1;
pub const RT1318_PLL_S_RC: i32 = 2;
pub const RT1318_PLL_S_MCLK: i32 = 3;
pub const RT1318_PLL_S_SDW_IN_PLL: i32 = 4;
pub const RT1318_PLL_S_SDW_0: i32 = 5;
pub const RT1318_PLL_S_SDW_1: i32 = 6;
pub const RT1318_PLL_S_SDW_2: i32 = 7;

/* TDM channel */
pub const RT1318_2CH: i32 = 0;
pub const RT1318_4CH: i32 = 1;
pub const RT1318_6CH: i32 = 2;
pub const RT1318_8CH: i32 = 3;

/* R0 calibration result */
pub const RT1318_R0_OUT_OF_RANGE: i32 = 0;
pub const RT1318_R0_IN_RANGE: i32 = 1;
pub const RT1318_R0_CALIB_NOT_DONE: i32 = 2;

/* PLL pre-defined M/N/K */

#[repr(C)]
pub struct pll_calc_map {
    pub pll_in: u32,
    pub pll_out: u32,
    pub k: i32,
    pub n: i32,
    pub m: i32,
    pub m_bp: bool,
    pub k_bp: bool,
}

#[repr(C)]
pub struct rt1318_pll_code {
    pub m_bp: bool, /* Indicates bypass m code or not. */
    pub k_bp: bool, /* Indicates bypass k code or not. */
    pub m_code: i32,
    pub n_code: i32,
    pub k_code: i32,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
