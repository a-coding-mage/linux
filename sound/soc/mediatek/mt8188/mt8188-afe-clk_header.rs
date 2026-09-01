/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mt8188-afe-clk.h  --  MediaTek 8188 afe clock ctrl definition
 *
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Bicycle Tsai <bicycle.tsai@mediatek.com>
 *         Trevor Wu <trevor.wu@mediatek.com>
 *         Chun-Chia Chiu <chun-chia.chiu@mediatek.com>
 */

use core::ffi::{c_char, c_int, c_uint};

/* APLL */
pub const APLL1_W_NAME: &str = "APLL1";
pub const APLL2_W_NAME: &str = "APLL2";

/* xtal */
pub const MT8188_CLK_XTAL_26M: c_int = 0;
/* pll */
pub const MT8188_CLK_APMIXED_APLL1: c_int = 1;
pub const MT8188_CLK_APMIXED_APLL2: c_int = 2;
/* divider */
pub const MT8188_CLK_TOP_APLL1_D4: c_int = 3;
pub const MT8188_CLK_TOP_APLL2_D4: c_int = 4;
pub const MT8188_CLK_TOP_APLL12_DIV0: c_int = 5;
pub const MT8188_CLK_TOP_APLL12_DIV1: c_int = 6;
pub const MT8188_CLK_TOP_APLL12_DIV2: c_int = 7;
pub const MT8188_CLK_TOP_APLL12_DIV3: c_int = 8;
pub const MT8188_CLK_TOP_APLL12_DIV4: c_int = 9;
pub const MT8188_CLK_TOP_APLL12_DIV9: c_int = 10;
/* mux */
pub const MT8188_CLK_TOP_A1SYS_HP_SEL: c_int = 11;
pub const MT8188_CLK_TOP_A2SYS_SEL: c_int = 12;
pub const MT8188_CLK_TOP_AUD_IEC_SEL: c_int = 13;
pub const MT8188_CLK_TOP_AUD_INTBUS_SEL: c_int = 14;
pub const MT8188_CLK_TOP_AUDIO_H_SEL: c_int = 15;
pub const MT8188_CLK_TOP_AUDIO_LOCAL_BUS_SEL: c_int = 16;
pub const MT8188_CLK_TOP_DPTX_M_SEL: c_int = 17;
pub const MT8188_CLK_TOP_I2SO1_M_SEL: c_int = 18;
pub const MT8188_CLK_TOP_I2SO2_M_SEL: c_int = 19;
pub const MT8188_CLK_TOP_I2SI1_M_SEL: c_int = 20;
pub const MT8188_CLK_TOP_I2SI2_M_SEL: c_int = 21;
/* clock gate */
pub const MT8188_CLK_ADSP_AUDIO_26M: c_int = 22;
pub const MT8188_CLK_AUD_AFE: c_int = 23;
pub const MT8188_CLK_AUD_APLL1_TUNER: c_int = 24;
pub const MT8188_CLK_AUD_APLL2_TUNER: c_int = 25;
pub const MT8188_CLK_AUD_TOP0_SPDF: c_int = 26;
pub const MT8188_CLK_AUD_APLL: c_int = 27;
pub const MT8188_CLK_AUD_APLL2: c_int = 28;
pub const MT8188_CLK_AUD_DAC: c_int = 29;
pub const MT8188_CLK_AUD_ADC: c_int = 30;
pub const MT8188_CLK_AUD_DAC_HIRES: c_int = 31;
pub const MT8188_CLK_AUD_A1SYS_HP: c_int = 32;
pub const MT8188_CLK_AUD_AFE_DMIC1: c_int = 33;
pub const MT8188_CLK_AUD_AFE_DMIC2: c_int = 34;
pub const MT8188_CLK_AUD_AFE_DMIC3: c_int = 35;
pub const MT8188_CLK_AUD_AFE_DMIC4: c_int = 36;
pub const MT8188_CLK_AUD_ADC_HIRES: c_int = 37;
pub const MT8188_CLK_AUD_DMIC_HIRES1: c_int = 38;
pub const MT8188_CLK_AUD_DMIC_HIRES2: c_int = 39;
pub const MT8188_CLK_AUD_DMIC_HIRES3: c_int = 40;
pub const MT8188_CLK_AUD_DMIC_HIRES4: c_int = 41;
pub const MT8188_CLK_AUD_I2SIN: c_int = 42;
pub const MT8188_CLK_AUD_TDM_IN: c_int = 43;
pub const MT8188_CLK_AUD_I2S_OUT: c_int = 44;
pub const MT8188_CLK_AUD_TDM_OUT: c_int = 45;
pub const MT8188_CLK_AUD_HDMI_OUT: c_int = 46;
pub const MT8188_CLK_AUD_ASRC11: c_int = 47;
pub const MT8188_CLK_AUD_ASRC12: c_int = 48;
pub const MT8188_CLK_AUD_A1SYS: c_int = 49;
pub const MT8188_CLK_AUD_A2SYS: c_int = 50;
pub const MT8188_CLK_AUD_PCMIF: c_int = 51;
pub const MT8188_CLK_AUD_MEMIF_UL1: c_int = 52;
pub const MT8188_CLK_AUD_MEMIF_UL2: c_int = 53;
pub const MT8188_CLK_AUD_MEMIF_UL3: c_int = 54;
pub const MT8188_CLK_AUD_MEMIF_UL4: c_int = 55;
pub const MT8188_CLK_AUD_MEMIF_UL5: c_int = 56;
pub const MT8188_CLK_AUD_MEMIF_UL6: c_int = 57;
pub const MT8188_CLK_AUD_MEMIF_UL8: c_int = 58;
pub const MT8188_CLK_AUD_MEMIF_UL9: c_int = 59;
pub const MT8188_CLK_AUD_MEMIF_UL10: c_int = 60;
pub const MT8188_CLK_AUD_MEMIF_DL2: c_int = 61;
pub const MT8188_CLK_AUD_MEMIF_DL3: c_int = 62;
pub const MT8188_CLK_AUD_MEMIF_DL6: c_int = 63;
pub const MT8188_CLK_AUD_MEMIF_DL7: c_int = 64;
pub const MT8188_CLK_AUD_MEMIF_DL8: c_int = 65;
pub const MT8188_CLK_AUD_MEMIF_DL10: c_int = 66;
pub const MT8188_CLK_AUD_MEMIF_DL11: c_int = 67;
pub const MT8188_CLK_NUM: c_int = 68;

pub const MT8188_AUD_PLL1: c_int = 0;
pub const MT8188_AUD_PLL2: c_int = 1;
pub const MT8188_AUD_PLL3: c_int = 2;
pub const MT8188_AUD_PLL4: c_int = 3;
pub const MT8188_AUD_PLL5: c_int = 4;
pub const MT8188_AUD_PLL_NUM: c_int = 5;

pub const MT8188_MCK_SEL_26M: c_int = 0;
pub const MT8188_MCK_SEL_APLL1: c_int = 1;
pub const MT8188_MCK_SEL_APLL2: c_int = 2;
pub const MT8188_MCK_SEL_APLL3: c_int = 3;
pub const MT8188_MCK_SEL_APLL4: c_int = 4;
pub const MT8188_MCK_SEL_APLL5: c_int = 5;
pub const MT8188_MCK_SEL_NUM: c_int = 6;

#[repr(C)]
pub struct mtk_base_afe {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn mt8188_afe_get_mclk_source_clk_id(sel: c_int) -> c_int;
    pub fn mt8188_afe_get_mclk_source_rate(afe: *mut mtk_base_afe, apll: c_int) -> c_int;
    pub fn mt8188_afe_get_default_mclk_source_by_rate(rate: c_int) -> c_int;
    pub fn mt8188_get_apll_by_rate(afe: *mut mtk_base_afe, rate: c_int) -> c_int;
    pub fn mt8188_get_apll_by_name(afe: *mut mtk_base_afe, name: *const c_char) -> c_int;
    pub fn mt8188_afe_init_clock(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8188_afe_enable_clk(afe: *mut mtk_base_afe, clk: *mut clk) -> c_int;
    pub fn mt8188_afe_disable_clk(afe: *mut mtk_base_afe, clk: *mut clk);
    pub fn mt8188_afe_set_clk_rate(
        afe: *mut mtk_base_afe,
        clk: *mut clk,
        rate: c_uint,
    ) -> c_int;
    pub fn mt8188_afe_set_clk_parent(
        afe: *mut mtk_base_afe,
        clk: *mut clk,
        parent: *mut clk,
    ) -> c_int;
    pub fn mt8188_apll1_enable(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8188_apll1_disable(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8188_apll2_enable(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8188_apll2_disable(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8188_afe_enable_main_clock(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8188_afe_disable_main_clock(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8188_afe_enable_reg_rw_clk(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8188_afe_disable_reg_rw_clk(afe: *mut mtk_base_afe) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
