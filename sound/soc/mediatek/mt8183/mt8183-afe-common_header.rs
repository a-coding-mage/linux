// SPDX-License-Identifier: GPL-2.0
/*
 * mt8183-afe-common.h  --  Mediatek 8183 audio driver definitions
 *
 * Copyright (c) 2018 MediaTek Inc.
 * Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>
 */

// C dependencies:
// #include <sound/soc.h>
// #include <linux/list.h>
// #include <linux/regmap.h>
// #include "../common/mtk-base-afe.h"

pub const MT8183_MEMIF_DL1: u32 = 0;
pub const MT8183_MEMIF_DL2: u32 = 1;
pub const MT8183_MEMIF_DL3: u32 = 2;
pub const MT8183_MEMIF_VUL12: u32 = 3;
pub const MT8183_MEMIF_VUL2: u32 = 4;
pub const MT8183_MEMIF_AWB: u32 = 5;
pub const MT8183_MEMIF_AWB2: u32 = 6;
pub const MT8183_MEMIF_MOD_DAI: u32 = 7;
pub const MT8183_MEMIF_HDMI: u32 = 8;
pub const MT8183_MEMIF_NUM: u32 = 9;
pub const MT8183_DAI_ADDA: u32 = MT8183_MEMIF_NUM;
pub const MT8183_DAI_PCM_1: u32 = 10;
pub const MT8183_DAI_PCM_2: u32 = 11;
pub const MT8183_DAI_I2S_0: u32 = 12;
pub const MT8183_DAI_I2S_1: u32 = 13;
pub const MT8183_DAI_I2S_2: u32 = 14;
pub const MT8183_DAI_I2S_3: u32 = 15;
pub const MT8183_DAI_I2S_5: u32 = 16;
pub const MT8183_DAI_TDM: u32 = 17;
pub const MT8183_DAI_HOSTLESS_LPBK: u32 = 18;
pub const MT8183_DAI_HOSTLESS_SPEECH: u32 = 19;
pub const MT8183_DAI_NUM: u32 = 20;

pub const MT8183_IRQ_0: u32 = 0;
pub const MT8183_IRQ_1: u32 = 1;
pub const MT8183_IRQ_2: u32 = 2;
pub const MT8183_IRQ_3: u32 = 3;
pub const MT8183_IRQ_4: u32 = 4;
pub const MT8183_IRQ_5: u32 = 5;
pub const MT8183_IRQ_6: u32 = 6;
pub const MT8183_IRQ_7: u32 = 7;
pub const MT8183_IRQ_8: u32 = 8; /* hw bundle to TDM */
pub const MT8183_IRQ_11: u32 = 9;
pub const MT8183_IRQ_12: u32 = 10;
pub const MT8183_IRQ_NUM: u32 = 11;

pub const MT8183_MTKAIF_PROTOCOL_1: u32 = 0;
pub const MT8183_MTKAIF_PROTOCOL_2: u32 = 1;
pub const MT8183_MTKAIF_PROTOCOL_2_CLK_P2: u32 = 2;

/* MCLK */
pub const MT8183_I2S0_MCK: u32 = 0;
pub const MT8183_I2S1_MCK: u32 = 1;
pub const MT8183_I2S2_MCK: u32 = 2;
pub const MT8183_I2S3_MCK: u32 = 3;
pub const MT8183_I2S4_MCK: u32 = 4;
pub const MT8183_I2S4_BCK: u32 = 5;
pub const MT8183_I2S5_MCK: u32 = 6;
pub const MT8183_MCK_NUM: u32 = 7;

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
pub struct mt8183_afe_private {
    pub clk: *mut *mut clk,

    pub pm_runtime_bypass_reg_ctl: ::core::ffi::c_int,

    /* dai */
    pub dai_priv: [*mut ::core::ffi::c_void; MT8183_DAI_NUM as usize],

    /* adda */
    pub mtkaif_protocol: ::core::ffi::c_int,
    pub mtkaif_calibration_ok: ::core::ffi::c_int,
    pub mtkaif_chosen_phase: [::core::ffi::c_int; 4],
    pub mtkaif_phase_cycle: [::core::ffi::c_int; 4],
    pub mtkaif_calibration_num_phase: ::core::ffi::c_int,
    pub mtkaif_dmic: ::core::ffi::c_int,

    /* mck */
    pub mck_rate: [::core::ffi::c_int; MT8183_MCK_NUM as usize],
}

unsafe extern "C" {
    pub fn mt8183_general_rate_transform(
        dev: *mut device,
        rate: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_uint;
    pub fn mt8183_rate_transform(
        dev: *mut device,
        rate: ::core::ffi::c_uint,
        aud_blk: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uint;

    pub fn mt8183_dai_i2s_set_share(
        afe: *mut mtk_base_afe,
        main_i2s_name: *const ::core::ffi::c_char,
        secondary_i2s_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;

    /* dai register */
    pub fn mt8183_dai_adda_register(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
    pub fn mt8183_dai_pcm_register(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
    pub fn mt8183_dai_i2s_register(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
    pub fn mt8183_dai_tdm_register(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
    pub fn mt8183_dai_hostless_register(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
