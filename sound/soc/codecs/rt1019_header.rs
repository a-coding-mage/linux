/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt1019.h  --  RT1019 ALSA SoC audio amplifier driver
 *
 * Copyright(c) 2021 Realtek Semiconductor Corp.
 */

pub const RT1019_DEVICE_ID_VAL: u32 = 0x1019;
pub const RT1019_DEVICE_ID_VAL2: u32 = 0x6731;

pub const RT1019_RESET: u32 = 0x0000;
pub const RT1019_IDS_CTRL: u32 = 0x0011;
pub const RT1019_ASEL_CTRL: u32 = 0x0013;
pub const RT1019_PWR_STRP_2: u32 = 0x0019;
pub const RT1019_BEEP_TONE: u32 = 0x001b;
pub const RT1019_VER_ID: u32 = 0x005c;
pub const RT1019_VEND_ID_1: u32 = 0x005e;
pub const RT1019_VEND_ID_2: u32 = 0x005f;
pub const RT1019_DEV_ID_1: u32 = 0x0061;
pub const RT1019_DEV_ID_2: u32 = 0x0062;
pub const RT1019_SDB_CTRL: u32 = 0x0066;
pub const RT1019_CLK_TREE_1: u32 = 0x0100;
pub const RT1019_CLK_TREE_2: u32 = 0x0101;
pub const RT1019_CLK_TREE_3: u32 = 0x0102;
pub const RT1019_PLL_1: u32 = 0x0311;
pub const RT1019_PLL_2: u32 = 0x0312;
pub const RT1019_PLL_3: u32 = 0x0313;
pub const RT1019_TDM_1: u32 = 0x0400;
pub const RT1019_TDM_2: u32 = 0x0401;
pub const RT1019_TDM_3: u32 = 0x0402;
pub const RT1019_DMIX_MONO_1: u32 = 0x0504;
pub const RT1019_DMIX_MONO_2: u32 = 0x0505;
pub const RT1019_BEEP_1: u32 = 0x0b00;
pub const RT1019_BEEP_2: u32 = 0x0b01;

/* 0x0019 Power On Strap Control-2 */
pub const RT1019_AUTO_BITS_SEL_MASK: u32 = 0x1 << 5;
pub const RT1019_AUTO_BITS_SEL_AUTO: u32 = 0x1 << 5;
pub const RT1019_AUTO_BITS_SEL_MANU: u32 = 0x0 << 5;
pub const RT1019_AUTO_CLK_SEL_MASK: u32 = 0x1 << 4;
pub const RT1019_AUTO_CLK_SEL_AUTO: u32 = 0x1 << 4;
pub const RT1019_AUTO_CLK_SEL_MANU: u32 = 0x0 << 4;

/* 0x0100 Clock Tree Control-1 */
pub const RT1019_CLK_SYS_PRE_SEL_MASK: u32 = 0x1 << 7;
pub const RT1019_CLK_SYS_PRE_SEL_SFT: u32 = 7;
pub const RT1019_CLK_SYS_PRE_SEL_BCLK: u32 = 0x0 << 7;
pub const RT1019_CLK_SYS_PRE_SEL_PLL: u32 = 0x1 << 7;
pub const RT1019_PLL_SRC_MASK: u32 = 0x1 << 4;
pub const RT1019_PLL_SRC_SFT: u32 = 4;
pub const RT1019_PLL_SRC_SEL_BCLK: u32 = 0x0 << 4;
pub const RT1019_PLL_SRC_SEL_RC: u32 = 0x1 << 4;
pub const RT1019_SEL_FIFO_MASK: u32 = 0x3 << 2;
pub const RT1019_SEL_FIFO_DIV1: u32 = 0x0 << 2;
pub const RT1019_SEL_FIFO_DIV2: u32 = 0x1 << 2;
pub const RT1019_SEL_FIFO_DIV4: u32 = 0x2 << 2;

/* 0x0101 clock tree control-2 */
pub const RT1019_SYS_DIV_DA_FIL_MASK: u32 = 0x7 << 5;
pub const RT1019_SYS_DIV_DA_FIL_DIV1: u32 = 0x2 << 5;
pub const RT1019_SYS_DIV_DA_FIL_DIV2: u32 = 0x3 << 5;
pub const RT1019_SYS_DIV_DA_FIL_DIV4: u32 = 0x4 << 5;
pub const RT1019_SYS_DA_OSR_MASK: u32 = 0x3 << 2;
pub const RT1019_SYS_DA_OSR_DIV1: u32 = 0x0 << 2;
pub const RT1019_SYS_DA_OSR_DIV2: u32 = 0x1 << 2;
pub const RT1019_SYS_DA_OSR_DIV4: u32 = 0x2 << 2;
pub const RT1019_ASRC_256FS_MASK: u32 = 0x3;
pub const RT1019_ASRC_256FS_DIV1: u32 = 0x0;
pub const RT1019_ASRC_256FS_DIV2: u32 = 0x1;
pub const RT1019_ASRC_256FS_DIV4: u32 = 0x2;

/* 0x0102 clock tree control-3 */
pub const RT1019_SEL_CLK_CAL_MASK: u32 = 0x3 << 6;
pub const RT1019_SEL_CLK_CAL_DIV1: u32 = 0x0 << 6;
pub const RT1019_SEL_CLK_CAL_DIV2: u32 = 0x1 << 6;
pub const RT1019_SEL_CLK_CAL_DIV4: u32 = 0x2 << 6;

/* 0x0311 PLL-1 */
pub const RT1019_PLL_M_MASK: u32 = 0xf << 4;
pub const RT1019_PLL_M_SFT: u32 = 4;
pub const RT1019_PLL_M_BP_MASK: u32 = 0x1 << 1;
pub const RT1019_PLL_M_BP_SFT: u32 = 1;
pub const RT1019_PLL_Q_8_8_MASK: u32 = 0x1;

/* 0x0312 PLL-2 */
pub const RT1019_PLL_Q_7_0_MASK: u32 = 0xff;

/* 0x0313 PLL-3 */
pub const RT1019_PLL_K_MASK: u32 = 0x1f;

/* 0x0400 TDM Control-1 */
pub const RT1019_TDM_BCLK_MASK: u32 = 0x1 << 6;
pub const RT1019_TDM_BCLK_NORM: u32 = 0x0 << 6;
pub const RT1019_TDM_BCLK_INV: u32 = 0x1 << 6;
pub const RT1019_TDM_CL_MASK: u32 = 0x7;
pub const RT1019_TDM_CL_8: u32 = 0x4;
pub const RT1019_TDM_CL_32: u32 = 0x3;
pub const RT1019_TDM_CL_24: u32 = 0x2;
pub const RT1019_TDM_CL_20: u32 = 0x1;
pub const RT1019_TDM_CL_16: u32 = 0x0;

/* 0x0401 TDM Control-2 */
pub const RT1019_I2S_CH_TX_MASK: u32 = 0x3 << 6;
pub const RT1019_I2S_CH_TX_SFT: u32 = 6;
pub const RT1019_I2S_TX_2CH: u32 = 0x0 << 6;
pub const RT1019_I2S_TX_4CH: u32 = 0x1 << 6;
pub const RT1019_I2S_TX_6CH: u32 = 0x2 << 6;
pub const RT1019_I2S_TX_8CH: u32 = 0x3 << 6;
pub const RT1019_I2S_DF_MASK: u32 = 0x7 << 3;
pub const RT1019_I2S_DF_SFT: u32 = 3;
pub const RT1019_I2S_DF_I2S: u32 = 0x0 << 3;
pub const RT1019_I2S_DF_LEFT: u32 = 0x1 << 3;
pub const RT1019_I2S_DF_PCM_A_R: u32 = 0x2 << 3;
pub const RT1019_I2S_DF_PCM_B_R: u32 = 0x3 << 3;
pub const RT1019_I2S_DF_PCM_A_F: u32 = 0x6 << 3;
pub const RT1019_I2S_DF_PCM_B_F: u32 = 0x7 << 3;
pub const RT1019_I2S_DL_MASK: u32 = 0x7;
pub const RT1019_I2S_DL_SFT: u32 = 0;
pub const RT1019_I2S_DL_16: u32 = 0x0;
pub const RT1019_I2S_DL_20: u32 = 0x1;
pub const RT1019_I2S_DL_24: u32 = 0x2;
pub const RT1019_I2S_DL_32: u32 = 0x3;
pub const RT1019_I2S_DL_8: u32 = 0x4;

/* TDM1 Control-3 (0x0402) */
pub const RT1019_TDM_I2S_TX_L_DAC1_1_MASK: u32 = 0x7 << 4;
pub const RT1019_TDM_I2S_TX_R_DAC1_1_MASK: u32 = 0x7;
pub const RT1019_TDM_I2S_TX_L_DAC1_1_SFT: u32 = 4;
pub const RT1019_TDM_I2S_TX_R_DAC1_1_SFT: u32 = 0;

/* System Clock Source */
pub const RT1019_SCLK_S_BCLK: i32 = 0;
pub const RT1019_SCLK_S_PLL: i32 = 1;

/* PLL1 Source */
pub const RT1019_PLL_S_BCLK: i32 = 0;
pub const RT1019_PLL_S_RC25M: i32 = 1;

pub const RT1019_AIF1: i32 = 0;
pub const RT1019_AIFS: i32 = 1;

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rt1019_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub sysclk: ::std::os::raw::c_int,
    pub sysclk_src: ::std::os::raw::c_int,
    pub lrck: ::std::os::raw::c_int,
    pub bclk: ::std::os::raw::c_int,
    pub pll_src: ::std::os::raw::c_int,
    pub pll_in: ::std::os::raw::c_int,
    pub pll_out: ::std::os::raw::c_int,
    pub bclk_ratio: ::std::os::raw::c_uint,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
