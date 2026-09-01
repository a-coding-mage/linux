// SPDX-License-Identifier: GPL-2.0
/*
 * mt8196-afe-clk.h  --  Mediatek MT8196 AFE Clock Control definitions
 *
 * Copyright (c) 2025 MediaTek Inc.
 *  Author: Darren Ye <darren.ye@mediatek.com>
 */

pub const MT8196_AFE_26M: i32 = 26000000;
pub const MT8196_AUD_ENG1_CLK: i32 = 45158400;
pub const MT8196_AUD_ENG2_CLK: i32 = 49152000;

/* APLL */
pub const APLL1_W_NAME: &[u8; 6] = b"APLL1\0";
pub const APLL2_W_NAME: &[u8; 6] = b"APLL2\0";

pub const MT8196_APLL1: i32 = 0;
pub const MT8196_APLL2: i32 = 1;

/* vlp clk */
pub const MT8196_CLK_VLP_MUX_AUDIOINTBUS: i32 = 0;
pub const MT8196_CLK_VLP_MUX_AUD_ENG1: i32 = 1;
pub const MT8196_CLK_VLP_MUX_AUD_ENG2: i32 = 2;
pub const MT8196_CLK_VLP_MUX_AUDIO_H: i32 = 3;
/* pll */
pub const MT8196_CLK_TOP_APLL1_CK: i32 = 4;
pub const MT8196_CLK_TOP_APLL2_CK: i32 = 5;
/* divider */
pub const MT8196_CLK_TOP_APLL12_DIV_I2SIN0: i32 = 6;
pub const MT8196_CLK_TOP_APLL12_DIV_I2SIN1: i32 = 7;
pub const MT8196_CLK_TOP_APLL12_DIV_FMI2S: i32 = 8;
pub const MT8196_CLK_TOP_APLL12_DIV_TDMOUT_M: i32 = 9;
pub const MT8196_CLK_TOP_APLL12_DIV_TDMOUT_B: i32 = 10;
/* mux */
pub const MT8196_CLK_TOP_ADSP_SEL: i32 = 11;
pub const MT8196_CLK_NUM: i32 = 12;

#[repr(C)]
pub struct mtk_base_afe {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn mt8196_mck_enable(afe: *mut mtk_base_afe, mck_id: i32, rate: i32) -> i32;
    pub fn mt8196_mck_disable(afe: *mut mtk_base_afe, mck_id: i32) -> i32;
    pub fn mt8196_get_apll_rate(afe: *mut mtk_base_afe, apll: i32) -> i32;
    pub fn mt8196_get_apll_by_rate(afe: *mut mtk_base_afe, rate: i32) -> i32;
    pub fn mt8196_get_apll_by_name(afe: *mut mtk_base_afe, name: *const ::core::ffi::c_char) -> i32;
    pub fn mt8196_init_clock(afe: *mut mtk_base_afe) -> i32;
    pub fn mt8196_afe_enable_clk(afe: *mut mtk_base_afe, clk: *mut clk) -> i32;
    pub fn mt8196_afe_disable_clk(afe: *mut mtk_base_afe, clk: *mut clk);
    pub fn mt8196_apll1_enable(afe: *mut mtk_base_afe) -> i32;
    pub fn mt8196_apll1_disable(afe: *mut mtk_base_afe);
    pub fn mt8196_apll2_enable(afe: *mut mtk_base_afe) -> i32;
    pub fn mt8196_apll2_disable(afe: *mut mtk_base_afe);
    pub fn mt8196_afe_enable_main_clock(afe: *mut mtk_base_afe) -> i32;
    pub fn mt8196_afe_disable_main_clock(afe: *mut mtk_base_afe) -> i32;
    pub fn mt8196_afe_enable_reg_rw_clk(afe: *mut mtk_base_afe) -> i32;
    pub fn mt8196_afe_disable_reg_rw_clk(afe: *mut mtk_base_afe) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
