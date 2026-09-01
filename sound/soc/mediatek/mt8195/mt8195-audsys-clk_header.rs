// SPDX-License-Identifier: GPL-2.0
/*
 * mt8195-audsys-clk.h  --  Mediatek 8195 audsys clock definition
 *
 * Copyright (c) 2021 MediaTek Inc.
 * Author: Trevor Wu <trevor.wu@mediatek.com>
 */

unsafe extern "C" {
    pub fn mt8195_audsys_clk_register(afe: *mut mtk_base_afe) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
