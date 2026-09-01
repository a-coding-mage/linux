/* SPDX-License-Identifier: GPL-2.0
 *
 * mt8186-afe-clk.h  --  Mediatek 8186 afe clock ctrl definition
 *
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Jiaxin Yu <jiaxin.yu@mediatek.com>
 */

/* Header guard removed in Rust translation: _MT8186_AFE_CLOCK_CTRL_H_ */

pub const PERI_BUS_DCM_CTRL: u32 = 0x74;

/* APLL */
pub const APLL1_W_NAME: &str = "APLL1";
pub const APLL2_W_NAME: &str = "APLL2";

pub const MT8186_APLL1: i32 = 0;
pub const MT8186_APLL2: i32 = 1;

pub const CLK_AFE: i32 = 0;
pub const CLK_DAC: i32 = 1;
pub const CLK_DAC_PREDIS: i32 = 2;
pub const CLK_ADC: i32 = 3;
pub const CLK_TML: i32 = 4;
pub const CLK_APLL22M: i32 = 5;
pub const CLK_APLL24M: i32 = 6;
pub const CLK_APLL1_TUNER: i32 = 7;
pub const CLK_APLL2_TUNER: i32 = 8;
pub const CLK_TDM: i32 = 9;
pub const CLK_NLE: i32 = 10;
pub const CLK_DAC_HIRES: i32 = 11;
pub const CLK_ADC_HIRES: i32 = 12;
pub const CLK_I2S1_BCLK: i32 = 13;
pub const CLK_I2S2_BCLK: i32 = 14;
pub const CLK_I2S3_BCLK: i32 = 15;
pub const CLK_I2S4_BCLK: i32 = 16;
pub const CLK_CONNSYS_I2S_ASRC: i32 = 17;
pub const CLK_GENERAL1_ASRC: i32 = 18;
pub const CLK_GENERAL2_ASRC: i32 = 19;
pub const CLK_ADC_HIRES_TML: i32 = 20;
pub const CLK_ADDA6_ADC: i32 = 21;
pub const CLK_ADDA6_ADC_HIRES: i32 = 22;
pub const CLK_3RD_DAC: i32 = 23;
pub const CLK_3RD_DAC_PREDIS: i32 = 24;
pub const CLK_3RD_DAC_TML: i32 = 25;
pub const CLK_3RD_DAC_HIRES: i32 = 26;
pub const CLK_ETDM_IN1_BCLK: i32 = 27;
pub const CLK_ETDM_OUT1_BCLK: i32 = 28;
pub const CLK_INFRA_SYS_AUDIO: i32 = 29;
pub const CLK_INFRA_AUDIO_26M: i32 = 30;
pub const CLK_MUX_AUDIO: i32 = 31;
pub const CLK_MUX_AUDIOINTBUS: i32 = 32;
pub const CLK_TOP_MAINPLL_D2_D4: i32 = 33;
/* apll related mux */
pub const CLK_TOP_MUX_AUD_1: i32 = 34;
pub const CLK_TOP_APLL1_CK: i32 = 35;
pub const CLK_TOP_MUX_AUD_2: i32 = 36;
pub const CLK_TOP_APLL2_CK: i32 = 37;
pub const CLK_TOP_MUX_AUD_ENG1: i32 = 38;
pub const CLK_TOP_APLL1_D8: i32 = 39;
pub const CLK_TOP_MUX_AUD_ENG2: i32 = 40;
pub const CLK_TOP_APLL2_D8: i32 = 41;
pub const CLK_TOP_MUX_AUDIO_H: i32 = 42;
pub const CLK_TOP_I2S0_M_SEL: i32 = 43;
pub const CLK_TOP_I2S1_M_SEL: i32 = 44;
pub const CLK_TOP_I2S2_M_SEL: i32 = 45;
pub const CLK_TOP_I2S4_M_SEL: i32 = 46;
pub const CLK_TOP_TDM_M_SEL: i32 = 47;
pub const CLK_TOP_APLL12_DIV0: i32 = 48;
pub const CLK_TOP_APLL12_DIV1: i32 = 49;
pub const CLK_TOP_APLL12_DIV2: i32 = 50;
pub const CLK_TOP_APLL12_DIV4: i32 = 51;
pub const CLK_TOP_APLL12_DIV_TDM: i32 = 52;
pub const CLK_CLK26M: i32 = 53;
pub const CLK_NUM: i32 = 54;

#[repr(C)]
pub struct mtk_base_afe {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn mt8186_set_audio_int_bus_parent(afe: *mut mtk_base_afe, clk_id: i32) -> i32;
    pub fn mt8186_init_clock(afe: *mut mtk_base_afe) -> i32;
    pub fn mt8186_afe_enable_cgs(afe: *mut mtk_base_afe) -> i32;
    pub fn mt8186_afe_disable_cgs(afe: *mut mtk_base_afe);
    pub fn mt8186_afe_enable_clock(afe: *mut mtk_base_afe) -> i32;
    pub fn mt8186_afe_disable_clock(afe: *mut mtk_base_afe);

    pub fn mt8186_apll1_enable(afe: *mut mtk_base_afe) -> i32;
    pub fn mt8186_apll1_disable(afe: *mut mtk_base_afe);

    pub fn mt8186_apll2_enable(afe: *mut mtk_base_afe) -> i32;
    pub fn mt8186_apll2_disable(afe: *mut mtk_base_afe);

    pub fn mt8186_get_apll_rate(afe: *mut mtk_base_afe, apll: i32) -> i32;
    pub fn mt8186_get_apll_by_rate(afe: *mut mtk_base_afe, rate: i32) -> i32;
    pub fn mt8186_get_apll_by_name(afe: *mut mtk_base_afe, name: *const ::core::ffi::c_char) -> i32;

    /* these will be replaced by using CCF */
    pub fn mt8186_mck_enable(afe: *mut mtk_base_afe, mck_id: i32, rate: i32) -> i32;
    pub fn mt8186_mck_disable(afe: *mut mtk_base_afe, mck_id: i32);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
