/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mt6797-afe-common.h  --  Mediatek 6797 audio driver definitions
 *
 * Copyright (c) 2018 MediaTek Inc.
 * Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>
 */

/* Dependencies in the original C header:
 * #include <sound/soc.h>
 * #include <linux/list.h>
 * #include <linux/regmap.h>
 * #include "../common/mtk-base-afe.h"
 */

pub const MT6797_MEMIF_DL1: ::core::ffi::c_int = 0;
pub const MT6797_MEMIF_DL2: ::core::ffi::c_int = 1;
pub const MT6797_MEMIF_DL3: ::core::ffi::c_int = 2;
pub const MT6797_MEMIF_VUL: ::core::ffi::c_int = 3;
pub const MT6797_MEMIF_AWB: ::core::ffi::c_int = 4;
pub const MT6797_MEMIF_VUL12: ::core::ffi::c_int = 5;
pub const MT6797_MEMIF_DAI: ::core::ffi::c_int = 6;
pub const MT6797_MEMIF_MOD_DAI: ::core::ffi::c_int = 7;
pub const MT6797_MEMIF_NUM: ::core::ffi::c_int = 8;
pub const MT6797_DAI_ADDA: ::core::ffi::c_int = MT6797_MEMIF_NUM;
pub const MT6797_DAI_PCM_1: ::core::ffi::c_int = 9;
pub const MT6797_DAI_PCM_2: ::core::ffi::c_int = 10;
pub const MT6797_DAI_HOSTLESS_LPBK: ::core::ffi::c_int = 11;
pub const MT6797_DAI_HOSTLESS_SPEECH: ::core::ffi::c_int = 12;
pub const MT6797_DAI_NUM: ::core::ffi::c_int = 13;

pub const MT6797_IRQ_1: ::core::ffi::c_int = 0;
pub const MT6797_IRQ_2: ::core::ffi::c_int = 1;
pub const MT6797_IRQ_3: ::core::ffi::c_int = 2;
pub const MT6797_IRQ_4: ::core::ffi::c_int = 3;
pub const MT6797_IRQ_7: ::core::ffi::c_int = 4;
pub const MT6797_IRQ_NUM: ::core::ffi::c_int = 5;

#[repr(C)]
pub struct clk {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct mtk_base_afe {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct mt6797_afe_private {
    pub clk: *mut *mut clk,
}

unsafe extern "C" {
    pub fn mt6797_general_rate_transform(
        dev: *mut device,
        rate: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_uint;
    pub fn mt6797_rate_transform(
        dev: *mut device,
        rate: ::core::ffi::c_uint,
        aud_blk: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uint;

    /* dai register */
    pub fn mt6797_dai_adda_register(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
    pub fn mt6797_dai_pcm_register(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
    pub fn mt6797_dai_hostless_register(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
