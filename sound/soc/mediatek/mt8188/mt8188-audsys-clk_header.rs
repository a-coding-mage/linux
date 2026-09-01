// SPDX-License-Identifier: GPL-2.0
/*
 * mt8188-audsys-clk.h  --  MediaTek 8188 audsys clock definition
 *
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Chun-Chia Chiu <chun-chia.chiu@mediatek.com>
 */

// C header guard removed in Rust translation:
// _MT8188_AUDSYS_CLK_H_

unsafe extern "C" {
    pub fn mt8188_audsys_clk_register(
        afe: *mut crate::mtk_base_afe,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
