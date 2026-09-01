// SPDX-License-Identifier: GPL-2.0
/*
 * mt6797-afe-clk.h  --  Mediatek 6797 afe clock ctrl definition
 *
 * Copyright (c) 2018 MediaTek Inc.
 * Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>
 */

#[repr(C)]
pub struct mtk_base_afe {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn mt6797_init_clock(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
    pub fn mt6797_afe_enable_clock(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
    pub fn mt6797_afe_disable_clock(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
