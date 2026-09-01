/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mt2701-afe-clock-ctrl.h  --  Mediatek 2701 afe clock ctrl definition
 *
 * Copyright (c) 2016 MediaTek Inc.
 * Author: Garlic Tseng <garlic.tseng@mediatek.com>
 *	   Ryder Lee <ryder.lee@mediatek.com>
 */

use core::ffi::c_int;

#[repr(C)]
pub struct mtk_base_afe {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mt2701_i2s_path {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn mt2701_init_clock(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt2701_afe_enable_clock(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt2701_afe_disable_clock(afe: *mut mtk_base_afe) -> c_int;

    pub fn mt2701_afe_enable_i2s(
        afe: *mut mtk_base_afe,
        i2s_path: *mut mt2701_i2s_path,
        dir: c_int,
    ) -> c_int;
    pub fn mt2701_afe_disable_i2s(
        afe: *mut mtk_base_afe,
        i2s_path: *mut mt2701_i2s_path,
        dir: c_int,
    );
    pub fn mt2701_afe_enable_mclk(afe: *mut mtk_base_afe, id: c_int) -> c_int;
    pub fn mt2701_afe_disable_mclk(afe: *mut mtk_base_afe, id: c_int);

    pub fn mt2701_enable_btmrg_clk(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt2701_disable_btmrg_clk(afe: *mut mtk_base_afe);

    pub fn mt2701_mclk_configuration(afe: *mut mtk_base_afe, id: c_int) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
