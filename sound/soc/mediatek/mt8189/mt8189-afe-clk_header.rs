// SPDX-License-Identifier: GPL-2.0
/*
 * mt8189-afe-clk.h  --  Mediatek 8189 afe clock ctrl definition
 *
 * Copyright (c) 2025 MediaTek Inc.
 * Author: Darren Ye <darren.ye@mediatek.com>
 */

use core::ffi::{c_char, c_int};

/* APLL */
pub const APLL1_W_NAME: &str = "APLL1";
pub const APLL2_W_NAME: &str = "APLL2";

pub const MT8189_APLL1: c_int = 0;
pub const MT8189_APLL2: c_int = 1;

pub const MT8189_CLK_TOP_MUX_AUDIOINTBUS: c_int = 0;
pub const MT8189_CLK_TOP_MUX_AUD_ENG1: c_int = 1;
pub const MT8189_CLK_TOP_MUX_AUD_ENG2: c_int = 2;
pub const MT8189_CLK_TOP_MUX_AUDIO_H: c_int = 3;
/* pll */
pub const MT8189_CLK_TOP_APLL1_CK: c_int = 4;
pub const MT8189_CLK_TOP_APLL2_CK: c_int = 5;
/* divider */
pub const MT8189_CLK_TOP_APLL1_D4: c_int = 6;
pub const MT8189_CLK_TOP_APLL2_D4: c_int = 7;
pub const MT8189_CLK_TOP_APLL12_DIV_I2SIN0: c_int = 8;
pub const MT8189_CLK_TOP_APLL12_DIV_I2SIN1: c_int = 9;
pub const MT8189_CLK_TOP_APLL12_DIV_I2SOUT0: c_int = 10;
pub const MT8189_CLK_TOP_APLL12_DIV_I2SOUT1: c_int = 11;
pub const MT8189_CLK_TOP_APLL12_DIV_FMI2S: c_int = 12;
pub const MT8189_CLK_TOP_APLL12_DIV_TDMOUT_M: c_int = 13;
pub const MT8189_CLK_TOP_APLL12_DIV_TDMOUT_B: c_int = 14;
/* mux */
pub const MT8189_CLK_TOP_MUX_AUD_1: c_int = 15;
pub const MT8189_CLK_TOP_MUX_AUD_2: c_int = 16;
pub const MT8189_CLK_TOP_I2SIN0_M_SEL: c_int = 17;
pub const MT8189_CLK_TOP_I2SIN1_M_SEL: c_int = 18;
pub const MT8189_CLK_TOP_I2SOUT0_M_SEL: c_int = 19;
pub const MT8189_CLK_TOP_I2SOUT1_M_SEL: c_int = 20;
pub const MT8189_CLK_TOP_FMI2S_M_SEL: c_int = 21;
pub const MT8189_CLK_TOP_TDMOUT_M_SEL: c_int = 22;
/* top 26m */
pub const MT8189_CLK_TOP_CLK26M: c_int = 23;
/* peri */
pub const MT8189_CLK_PERAO_AUDIO_SLV_CK_PERI: c_int = 24;
pub const MT8189_CLK_PERAO_AUDIO_MST_CK_PERI: c_int = 25;
pub const MT8189_CLK_PERAO_INTBUS_CK_PERI: c_int = 26;
pub const MT8189_CLK_NUM: c_int = 27;

#[repr(C)]
pub struct mtk_base_afe {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn mt8189_mck_enable(afe: *mut mtk_base_afe, mck_id: c_int, rate: c_int) -> c_int;
    pub fn mt8189_mck_disable(afe: *mut mtk_base_afe, mck_id: c_int) -> c_int;
    pub fn mt8189_get_apll_rate(afe: *mut mtk_base_afe, apll: c_int) -> c_int;
    pub fn mt8189_get_apll_by_rate(afe: *mut mtk_base_afe, rate: c_int) -> c_int;
    pub fn mt8189_get_apll_by_name(afe: *mut mtk_base_afe, name: *const c_char) -> c_int;
    pub fn mt8189_init_clock(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8189_afe_enable_clk(afe: *mut mtk_base_afe, clk: *mut clk) -> c_int;
    pub fn mt8189_afe_disable_clk(afe: *mut mtk_base_afe, clk: *mut clk);
    pub fn mt8189_apll1_enable(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8189_apll1_disable(afe: *mut mtk_base_afe);
    pub fn mt8189_apll2_enable(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8189_apll2_disable(afe: *mut mtk_base_afe);
    pub fn mt8189_afe_enable_main_clock(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8189_afe_disable_main_clock(afe: *mut mtk_base_afe);
    pub fn mt8189_afe_enable_reg_rw_clk(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8189_afe_disable_reg_rw_clk(afe: *mut mtk_base_afe) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
