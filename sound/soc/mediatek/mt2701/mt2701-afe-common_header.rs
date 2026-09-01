// SPDX-License-Identifier: GPL-2.0
/*
 * mt2701-afe-common.h  --  Mediatek 2701 audio driver definitions
 *
 * Copyright (c) 2016 MediaTek Inc.
 * Author: Garlic Tseng <garlic.tseng@mediatek.com>
 */

// C header dependencies:
// #include <sound/soc.h>
// #include <linux/clk.h>
// #include <linux/regmap.h>
// #include "mt2701-reg.h"
// #include "../common/mtk-base-afe.h"

use core::ffi::c_int;

pub const MT2701_PLL_DOMAIN_0_RATE: c_int = 98304000;
pub const MT2701_PLL_DOMAIN_1_RATE: c_int = 90316800;

pub const MT2701_MEMIF_DL1: c_int = 0;
pub const MT2701_MEMIF_DL2: c_int = 1;
pub const MT2701_MEMIF_DL3: c_int = 2;
pub const MT2701_MEMIF_DL4: c_int = 3;
pub const MT2701_MEMIF_DL5: c_int = 4;
pub const MT2701_MEMIF_DL_SINGLE_NUM: c_int = 5;
pub const MT2701_MEMIF_DLM: c_int = MT2701_MEMIF_DL_SINGLE_NUM;
pub const MT2701_MEMIF_UL1: c_int = 6;
pub const MT2701_MEMIF_UL2: c_int = 7;
pub const MT2701_MEMIF_UL3: c_int = 8;
pub const MT2701_MEMIF_UL4: c_int = 9;
pub const MT2701_MEMIF_UL5: c_int = 10;
pub const MT2701_MEMIF_DLBT: c_int = 11;
pub const MT2701_MEMIF_ULBT: c_int = 12;
pub const MT2701_MEMIF_HDMI: c_int = 13;
pub const MT2701_MEMIF_NUM: c_int = 14;
pub const MT2701_IO_I2S: c_int = MT2701_MEMIF_NUM;
pub const MT2701_IO_2ND_I2S: c_int = 15;
pub const MT2701_IO_3RD_I2S: c_int = 16;
pub const MT2701_IO_4TH_I2S: c_int = 17;
pub const MT2701_IO_5TH_I2S: c_int = 18;
pub const MT2701_IO_6TH_I2S: c_int = 19;
pub const MT2701_IO_MRG: c_int = 20;
pub const MT2701_IO_HDMI: c_int = 21;

pub const MT2701_IRQ_ASYS_IRQ1: c_int = 0;
pub const MT2701_IRQ_ASYS_IRQ2: c_int = 1;
pub const MT2701_IRQ_ASYS_IRQ3: c_int = 2;
pub const MT2701_IRQ_ASYS_END: c_int = 3;

pub type audio_base_clock = c_int;

pub const MT2701_INFRA_SYS_AUDIO: audio_base_clock = 0;
pub const MT2701_TOP_AUD_MCLK_SRC0: audio_base_clock = 1;
pub const MT2701_TOP_AUD_MCLK_SRC1: audio_base_clock = 2;
pub const MT2701_TOP_AUD_A1SYS: audio_base_clock = 3;
pub const MT2701_TOP_AUD_A2SYS: audio_base_clock = 4;
pub const MT2701_AUDSYS_AFE: audio_base_clock = 5;
pub const MT2701_AUDSYS_AFE_CONN: audio_base_clock = 6;
pub const MT2701_AUDSYS_A1SYS: audio_base_clock = 7;
pub const MT2701_AUDSYS_A2SYS: audio_base_clock = 8;
pub const MT2701_BASE_CLK_NUM: audio_base_clock = 9;

#[repr(C)]
pub struct mt2701_i2s_data {
    pub i2s_ctrl_reg: c_int,
    pub i2s_asrc_fs_shift: c_int,
    pub i2s_asrc_fs_mask: c_int,
}

#[repr(C)]
pub struct mt2701_i2s_path {
    pub mclk_rate: c_int,
    pub on: [c_int; MTK_STREAM_NUM],
    pub occupied: [c_int; MTK_STREAM_NUM],
    pub i2s_data: [*const mt2701_i2s_data; MTK_STREAM_NUM],
    pub hop_ck: [*mut clk; MTK_STREAM_NUM],
    pub sel_ck: *mut clk,
    pub div_ck: *mut clk,
    pub mclk_ck: *mut clk,
    pub asrco_ck: *mut clk,
}

#[repr(C)]
pub struct mt2701_soc_variants {
    pub has_one_heart_mode: bool,
    pub i2s_num: c_int,
}

#[repr(C)]
pub struct mt2701_afe_private {
    pub base_ck: [*mut clk; MT2701_BASE_CLK_NUM as usize],
    pub mrgif_ck: *mut clk,
    pub hadds2pll_ck: *mut clk,
    pub audio_hdmi_ck: *mut clk,
    pub audio_spdf_ck: *mut clk,
    pub audio_apll_ck: *mut clk,
    pub mrg_enable: [bool; MTK_STREAM_NUM],

    pub soc: *const mt2701_soc_variants,
    // Flexible array member in C: struct mt2701_i2s_path i2s_path[];
    pub i2s_path: [mt2701_i2s_path; 0],
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
