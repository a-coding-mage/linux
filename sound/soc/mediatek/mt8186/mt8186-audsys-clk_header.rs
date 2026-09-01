/* SPDX-License-Identifier: GPL-2.0
 *
 * mt8186-audsys-clk.h  --  Mediatek 8186 audsys clock definition
 *
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Trevor Wu <trevor.wu@mediatek.com>
 */

#[repr(C)]
pub struct mtk_base_afe {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn mt8186_audsys_clk_register(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
