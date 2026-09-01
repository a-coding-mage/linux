// SPDX-License-Identifier: GPL-2.0
/*
 * MediaTek ALSA SoC Audio DAI ADDA Common
 *
 * Copyright (c) 2021 MediaTek Inc.
 * Copyright (c) 2024 Collabora Ltd.
 *         AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

// C dependencies:
// #include <linux/delay.h>
// #include <linux/dev_printk.h>
// #include "mtk-base-afe.h"
// #include "mtk-dai-adda-common.h"

#[repr(C)]
pub struct mtk_base_afe {
    pub dev: *mut core::ffi::c_void,
}

extern "C" {
    static MTK_AFE_ADDA_DL_RATE_8K: u32;
    static MTK_AFE_ADDA_DL_RATE_11K: u32;
    static MTK_AFE_ADDA_DL_RATE_12K: u32;
    static MTK_AFE_ADDA_DL_RATE_16K: u32;
    static MTK_AFE_ADDA_DL_RATE_22K: u32;
    static MTK_AFE_ADDA_DL_RATE_24K: u32;
    static MTK_AFE_ADDA_DL_RATE_32K: u32;
    static MTK_AFE_ADDA_DL_RATE_44K: u32;
    static MTK_AFE_ADDA_DL_RATE_48K: u32;
    static MTK_AFE_ADDA_DL_RATE_96K: u32;
    static MTK_AFE_ADDA_DL_RATE_192K: u32;

    static MTK_AFE_ADDA_UL_RATE_8K: u32;
    static MTK_AFE_ADDA_UL_RATE_16K: u32;
    static MTK_AFE_ADDA_UL_RATE_32K: u32;
    static MTK_AFE_ADDA_UL_RATE_48K: u32;
    static MTK_AFE_ADDA_UL_RATE_96K: u32;
    static MTK_AFE_ADDA_UL_RATE_192K: u32;

    fn dev_info(dev: *mut core::ffi::c_void, fmt: *const core::ffi::c_char, ...);
}

#[no_mangle]
pub unsafe extern "C" fn mtk_adda_dl_rate_transform(
    afe: *mut mtk_base_afe,
    rate: u32,
) -> core::ffi::c_uint {
    match rate {
        8000 => MTK_AFE_ADDA_DL_RATE_8K,
        11025 => MTK_AFE_ADDA_DL_RATE_11K,
        12000 => MTK_AFE_ADDA_DL_RATE_12K,
        16000 => MTK_AFE_ADDA_DL_RATE_16K,
        22050 => MTK_AFE_ADDA_DL_RATE_22K,
        24000 => MTK_AFE_ADDA_DL_RATE_24K,
        32000 => MTK_AFE_ADDA_DL_RATE_32K,
        44100 => MTK_AFE_ADDA_DL_RATE_44K,
        48000 => MTK_AFE_ADDA_DL_RATE_48K,
        96000 => MTK_AFE_ADDA_DL_RATE_96K,
        192000 => MTK_AFE_ADDA_DL_RATE_192K,
        _ => {
            dev_info(
                (*afe).dev,
                b"%s(), rate %d invalid, use 48kHz!!!\n\0".as_ptr() as *const core::ffi::c_char,
                b"mtk_adda_dl_rate_transform\0".as_ptr() as *const core::ffi::c_char,
                rate,
            );
            MTK_AFE_ADDA_DL_RATE_48K
        }
    }
}

// EXPORT_SYMBOL_GPL(mtk_adda_dl_rate_transform);

#[no_mangle]
pub unsafe extern "C" fn mtk_adda_ul_rate_transform(
    afe: *mut mtk_base_afe,
    rate: u32,
) -> core::ffi::c_uint {
    match rate {
        8000 => MTK_AFE_ADDA_UL_RATE_8K,
        16000 => MTK_AFE_ADDA_UL_RATE_16K,
        32000 => MTK_AFE_ADDA_UL_RATE_32K,
        48000 => MTK_AFE_ADDA_UL_RATE_48K,
        96000 => MTK_AFE_ADDA_UL_RATE_96K,
        192000 => MTK_AFE_ADDA_UL_RATE_192K,
        _ => {
            dev_info(
                (*afe).dev,
                b"%s(), rate %d invalid, use 48kHz!!!\n\0".as_ptr() as *const core::ffi::c_char,
                b"mtk_adda_ul_rate_transform\0".as_ptr() as *const core::ffi::c_char,
                rate,
            );
            MTK_AFE_ADDA_UL_RATE_48K
        }
    }
}

// EXPORT_SYMBOL_GPL(mtk_adda_ul_rate_transform);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
