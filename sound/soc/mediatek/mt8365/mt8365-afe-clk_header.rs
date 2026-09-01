/* SPDX-License-Identifier: GPL-2.0
 *
 * MediaTek 8365 AFE clock control definitions
 *
 * Copyright (c) 2024 MediaTek Inc.
 * Authors: Jia Zeng <jia.zeng@mediatek.com>
 *          Alexandre Mergnat <amergnat@baylibre.com>
 */

use core::ffi::{c_int, c_uint};

#[repr(C)]
pub struct mtk_base_afe {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn mt8365_afe_init_audio_clk(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8365_afe_disable_clk(afe: *mut mtk_base_afe, clk: *mut clk);
    pub fn mt8365_afe_set_clk_rate(
        afe: *mut mtk_base_afe,
        clk: *mut clk,
        rate: c_uint,
    ) -> c_int;
    pub fn mt8365_afe_set_clk_parent(
        afe: *mut mtk_base_afe,
        clk: *mut clk,
        parent: *mut clk,
    ) -> c_int;
    pub fn mt8365_afe_enable_top_cg(afe: *mut mtk_base_afe, cg_type: c_uint) -> c_int;
    pub fn mt8365_afe_disable_top_cg(afe: *mut mtk_base_afe, cg_type: c_uint) -> c_int;
    pub fn mt8365_afe_enable_main_clk(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8365_afe_disable_main_clk(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8365_afe_emi_clk_on(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8365_afe_emi_clk_off(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8365_afe_enable_afe_on(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8365_afe_disable_afe_on(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8365_afe_enable_apll_tuner_cfg(afe: *mut mtk_base_afe, apll: c_uint) -> c_int;
    pub fn mt8365_afe_disable_apll_tuner_cfg(afe: *mut mtk_base_afe, apll: c_uint) -> c_int;
    pub fn mt8365_afe_enable_apll_associated_cfg(
        afe: *mut mtk_base_afe,
        apll: c_uint,
    ) -> c_int;
    pub fn mt8365_afe_disable_apll_associated_cfg(
        afe: *mut mtk_base_afe,
        apll: c_uint,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
