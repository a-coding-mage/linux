/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2021 MediaTek Inc.
 * Copyright (c) 2024 Collabora Ltd.
 *         AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

/* Forward declaration from C: struct mtk_base_afe; */
#[repr(C)]
pub struct mtk_base_afe {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum adda_input_mode_rate {
    MTK_AFE_ADDA_DL_RATE_8K = 0,
    MTK_AFE_ADDA_DL_RATE_11K = 1,
    MTK_AFE_ADDA_DL_RATE_12K = 2,
    MTK_AFE_ADDA_DL_RATE_16K = 3,
    MTK_AFE_ADDA_DL_RATE_22K = 4,
    MTK_AFE_ADDA_DL_RATE_24K = 5,
    MTK_AFE_ADDA_DL_RATE_32K = 6,
    MTK_AFE_ADDA_DL_RATE_44K = 7,
    MTK_AFE_ADDA_DL_RATE_48K = 8,
    MTK_AFE_ADDA_DL_RATE_96K = 9,
    MTK_AFE_ADDA_DL_RATE_192K = 10,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum adda_voice_mode_rate {
    MTK_AFE_ADDA_UL_RATE_8K = 0,
    MTK_AFE_ADDA_UL_RATE_16K = 1,
    MTK_AFE_ADDA_UL_RATE_32K = 2,
    MTK_AFE_ADDA_UL_RATE_48K = 3,
    MTK_AFE_ADDA_UL_RATE_96K = 4,
    MTK_AFE_ADDA_UL_RATE_192K = 5,
    MTK_AFE_ADDA_UL_RATE_48K_HD = 6,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum adda_rxif_delay_data {
    DELAY_DATA_MISO1 = 0,
    DELAY_DATA_MISO0 = 1,
    DELAY_DATA_MISO2 = 1,
}

extern "C" {
    pub fn mtk_adda_dl_rate_transform(afe: *mut mtk_base_afe, rate: u32) -> ::core::ffi::c_uint;
    pub fn mtk_adda_ul_rate_transform(afe: *mut mtk_base_afe, rate: u32) -> ::core::ffi::c_uint;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
