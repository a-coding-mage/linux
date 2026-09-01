/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mt8183-afe-clk.h  --  Mediatek 8183 afe clock ctrl definition
 *
 * Copyright (c) 2018 MediaTek Inc.
 * Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>
 */

/* APLL */
pub const APLL1_W_NAME: &[u8; 6] = b"APLL1\0";
pub const APLL2_W_NAME: &[u8; 6] = b"APLL2\0";

pub const MT8183_APLL1: ::core::ffi::c_int = 0;
pub const MT8183_APLL2: ::core::ffi::c_int = 1;

#[repr(C)]
pub struct mtk_base_afe {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn mt8183_init_clock(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
    pub fn mt8183_afe_enable_clock(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
    pub fn mt8183_afe_disable_clock(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;

    pub fn mt8183_apll1_enable(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
    pub fn mt8183_apll1_disable(afe: *mut mtk_base_afe);

    pub fn mt8183_apll2_enable(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
    pub fn mt8183_apll2_disable(afe: *mut mtk_base_afe);

    pub fn mt8183_get_apll_rate(
        afe: *mut mtk_base_afe,
        apll: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn mt8183_get_apll_by_rate(
        afe: *mut mtk_base_afe,
        rate: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn mt8183_get_apll_by_name(
        afe: *mut mtk_base_afe,
        name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;

    pub fn mt8183_mck_enable(
        afe: *mut mtk_base_afe,
        mck_id: ::core::ffi::c_int,
        rate: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn mt8183_mck_disable(afe: *mut mtk_base_afe, mck_id: ::core::ffi::c_int);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
