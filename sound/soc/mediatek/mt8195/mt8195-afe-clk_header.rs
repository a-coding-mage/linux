/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mt8195-afe-clk.h  --  Mediatek 8195 afe clock ctrl definition
 *
 * Copyright (c) 2021 MediaTek Inc.
 * Author: Bicycle Tsai <bicycle.tsai@mediatek.com>
 *         Trevor Wu <trevor.wu@mediatek.com>
 */

/* Header guard omitted in Rust. */

/* xtal */
pub const MT8195_CLK_XTAL_26M: i32 = 0;
/* divider */
pub const MT8195_CLK_TOP_APLL1: i32 = 1;
pub const MT8195_CLK_TOP_APLL2: i32 = 2;
pub const MT8195_CLK_TOP_APLL12_DIV0: i32 = 3;
pub const MT8195_CLK_TOP_APLL12_DIV1: i32 = 4;
pub const MT8195_CLK_TOP_APLL12_DIV2: i32 = 5;
pub const MT8195_CLK_TOP_APLL12_DIV3: i32 = 6;
pub const MT8195_CLK_TOP_APLL12_DIV9: i32 = 7;
/* mux */
pub const MT8195_CLK_TOP_A1SYS_HP_SEL: i32 = 8;
pub const MT8195_CLK_TOP_AUD_INTBUS_SEL: i32 = 9;
pub const MT8195_CLK_TOP_AUDIO_H_SEL: i32 = 10;
pub const MT8195_CLK_TOP_AUDIO_LOCAL_BUS_SEL: i32 = 11;
pub const MT8195_CLK_TOP_DPTX_M_SEL: i32 = 12;
pub const MT8195_CLK_TOP_I2SO1_M_SEL: i32 = 13;
pub const MT8195_CLK_TOP_I2SO2_M_SEL: i32 = 14;
pub const MT8195_CLK_TOP_I2SI1_M_SEL: i32 = 15;
pub const MT8195_CLK_TOP_I2SI2_M_SEL: i32 = 16;
/* clock gate */
pub const MT8195_CLK_INFRA_AO_AUDIO_26M_B: i32 = 17;
pub const MT8195_CLK_SCP_ADSP_AUDIODSP: i32 = 18;
pub const MT8195_CLK_AUD_AFE: i32 = 19;
pub const MT8195_CLK_AUD_APLL1_TUNER: i32 = 20;
pub const MT8195_CLK_AUD_APLL2_TUNER: i32 = 21;
pub const MT8195_CLK_AUD_APLL: i32 = 22;
pub const MT8195_CLK_AUD_APLL2: i32 = 23;
pub const MT8195_CLK_AUD_DAC: i32 = 24;
pub const MT8195_CLK_AUD_ADC: i32 = 25;
pub const MT8195_CLK_AUD_DAC_HIRES: i32 = 26;
pub const MT8195_CLK_AUD_A1SYS_HP: i32 = 27;
pub const MT8195_CLK_AUD_ADC_HIRES: i32 = 28;
pub const MT8195_CLK_AUD_ADDA6_ADC: i32 = 29;
pub const MT8195_CLK_AUD_ADDA6_ADC_HIRES: i32 = 30;
pub const MT8195_CLK_AUD_I2SIN: i32 = 31;
pub const MT8195_CLK_AUD_TDM_IN: i32 = 32;
pub const MT8195_CLK_AUD_I2S_OUT: i32 = 33;
pub const MT8195_CLK_AUD_TDM_OUT: i32 = 34;
pub const MT8195_CLK_AUD_HDMI_OUT: i32 = 35;
pub const MT8195_CLK_AUD_ASRC11: i32 = 36;
pub const MT8195_CLK_AUD_ASRC12: i32 = 37;
pub const MT8195_CLK_AUD_A1SYS: i32 = 38;
pub const MT8195_CLK_AUD_A2SYS: i32 = 39;
pub const MT8195_CLK_AUD_PCMIF: i32 = 40;
pub const MT8195_CLK_AUD_MEMIF_UL1: i32 = 41;
pub const MT8195_CLK_AUD_MEMIF_UL2: i32 = 42;
pub const MT8195_CLK_AUD_MEMIF_UL3: i32 = 43;
pub const MT8195_CLK_AUD_MEMIF_UL4: i32 = 44;
pub const MT8195_CLK_AUD_MEMIF_UL5: i32 = 45;
pub const MT8195_CLK_AUD_MEMIF_UL6: i32 = 46;
pub const MT8195_CLK_AUD_MEMIF_UL8: i32 = 47;
pub const MT8195_CLK_AUD_MEMIF_UL9: i32 = 48;
pub const MT8195_CLK_AUD_MEMIF_UL10: i32 = 49;
pub const MT8195_CLK_AUD_MEMIF_DL2: i32 = 50;
pub const MT8195_CLK_AUD_MEMIF_DL3: i32 = 51;
pub const MT8195_CLK_AUD_MEMIF_DL6: i32 = 52;
pub const MT8195_CLK_AUD_MEMIF_DL7: i32 = 53;
pub const MT8195_CLK_AUD_MEMIF_DL8: i32 = 54;
pub const MT8195_CLK_AUD_MEMIF_DL10: i32 = 55;
pub const MT8195_CLK_AUD_MEMIF_DL11: i32 = 56;
pub const MT8195_CLK_NUM: i32 = 57;

pub const MT8195_MCK_SEL_26M: i32 = 0;
pub const MT8195_MCK_SEL_APLL1: i32 = 1;
pub const MT8195_MCK_SEL_APLL2: i32 = 2;
pub const MT8195_MCK_SEL_APLL3: i32 = 3;
pub const MT8195_MCK_SEL_APLL4: i32 = 4;
pub const MT8195_MCK_SEL_APLL5: i32 = 5;
pub const MT8195_MCK_SEL_HDMIRX_APLL: i32 = 6;
pub const MT8195_MCK_SEL_NUM: i32 = 7;

pub const MT8195_AUD_PLL1: i32 = 0;
pub const MT8195_AUD_PLL2: i32 = 1;
pub const MT8195_AUD_PLL3: i32 = 2;
pub const MT8195_AUD_PLL4: i32 = 3;
pub const MT8195_AUD_PLL5: i32 = 4;
pub const MT8195_AUD_PLL_NUM: i32 = 5;

#[repr(C)]
pub struct mtk_base_afe {
    _private: [u8; 0],
}

/* Forward declaration supplied by other files. */
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn mt8195_afe_get_mclk_source_clk_id(sel: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn mt8195_afe_get_mclk_source_rate(
        afe: *mut mtk_base_afe,
        apll: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn mt8195_afe_get_default_mclk_source_by_rate(rate: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn mt8195_afe_init_clock(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
    pub fn mt8195_afe_enable_clk(afe: *mut mtk_base_afe, clk: *mut clk) -> ::core::ffi::c_int;
    pub fn mt8195_afe_disable_clk(afe: *mut mtk_base_afe, clk: *mut clk);
    pub fn mt8195_afe_prepare_clk(afe: *mut mtk_base_afe, clk: *mut clk) -> ::core::ffi::c_int;
    pub fn mt8195_afe_unprepare_clk(afe: *mut mtk_base_afe, clk: *mut clk);
    pub fn mt8195_afe_enable_clk_atomic(afe: *mut mtk_base_afe, clk: *mut clk) -> ::core::ffi::c_int;
    pub fn mt8195_afe_disable_clk_atomic(afe: *mut mtk_base_afe, clk: *mut clk);
    pub fn mt8195_afe_set_clk_rate(
        afe: *mut mtk_base_afe,
        clk: *mut clk,
        rate: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn mt8195_afe_set_clk_parent(
        afe: *mut mtk_base_afe,
        clk: *mut clk,
        parent: *mut clk,
    ) -> ::core::ffi::c_int;
    pub fn mt8195_afe_enable_main_clock(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
    pub fn mt8195_afe_disable_main_clock(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
    pub fn mt8195_afe_enable_reg_rw_clk(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
    pub fn mt8195_afe_disable_reg_rw_clk(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
