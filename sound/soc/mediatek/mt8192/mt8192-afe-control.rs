// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio Control
//
// Copyright (c) 2020 MediaTek Inc.
// Author: Shane Chien <shane.chien@mediatek.com>
//

// C dependency intent: #include "mt8192-afe-common.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtk_base_afe {
    pub dev: *mut device,
    pub platform_priv: *mut c_void,
}

#[repr(C)]
pub struct mt8192_afe_private {
    pub dai_priv: *mut *mut c_void,
}

unsafe extern "C" {
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: c_int, flags: c_uint) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: c_int) -> *mut c_void;
}

// External constants supplied by mt8192-afe-common.h and kernel headers.
unsafe extern "C" {
    static GFP_KERNEL: c_uint;
    static MT8192_MEMIF_DAI: c_int;
    static MT8192_MEMIF_MOD_DAI: c_int;
    static MT8192_DAI_PCM_1: c_int;
    static MT8192_DAI_PCM_2: c_int;
}

const ENOMEM: c_int = 12;

const MTK_AFE_RATE_8K: c_uint = 0;
const MTK_AFE_RATE_11K: c_uint = 1;
const MTK_AFE_RATE_12K: c_uint = 2;
const MTK_AFE_RATE_384K: c_uint = 3;
const MTK_AFE_RATE_16K: c_uint = 4;
const MTK_AFE_RATE_22K: c_uint = 5;
const MTK_AFE_RATE_24K: c_uint = 6;
const MTK_AFE_RATE_352K: c_uint = 7;
const MTK_AFE_RATE_32K: c_uint = 8;
const MTK_AFE_RATE_44K: c_uint = 9;
const MTK_AFE_RATE_48K: c_uint = 10;
const MTK_AFE_RATE_88K: c_uint = 11;
const MTK_AFE_RATE_96K: c_uint = 12;
const MTK_AFE_RATE_176K: c_uint = 13;
const MTK_AFE_RATE_192K: c_uint = 14;
const MTK_AFE_RATE_260K: c_uint = 15;

const MTK_AFE_DAI_MEMIF_RATE_8K: c_uint = 0;
const MTK_AFE_DAI_MEMIF_RATE_16K: c_uint = 1;
const MTK_AFE_DAI_MEMIF_RATE_32K: c_uint = 2;
const MTK_AFE_DAI_MEMIF_RATE_48K: c_uint = 3;

const MTK_AFE_PCM_RATE_8K: c_uint = 0;
const MTK_AFE_PCM_RATE_16K: c_uint = 1;
const MTK_AFE_PCM_RATE_32K: c_uint = 2;
const MTK_AFE_PCM_RATE_48K: c_uint = 3;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8192_general_rate_transform(
    dev: *mut device,
    rate: c_uint,
) -> c_uint {
    match rate {
        8000 => MTK_AFE_RATE_8K,
        11025 => MTK_AFE_RATE_11K,
        12000 => MTK_AFE_RATE_12K,
        16000 => MTK_AFE_RATE_16K,
        22050 => MTK_AFE_RATE_22K,
        24000 => MTK_AFE_RATE_24K,
        32000 => MTK_AFE_RATE_32K,
        44100 => MTK_AFE_RATE_44K,
        48000 => MTK_AFE_RATE_48K,
        88200 => MTK_AFE_RATE_88K,
        96000 => MTK_AFE_RATE_96K,
        176400 => MTK_AFE_RATE_176K,
        192000 => MTK_AFE_RATE_192K,
        260000 => MTK_AFE_RATE_260K,
        352800 => MTK_AFE_RATE_352K,
        384000 => MTK_AFE_RATE_384K,
        _ => {
            unsafe {
                dev_warn(
                    dev,
                    b"%s(), rate %u invalid, use %d!!!\n\0".as_ptr() as *const c_char,
                    b"mt8192_general_rate_transform\0".as_ptr() as *const c_char,
                    rate,
                    MTK_AFE_RATE_48K as c_int,
                );
            }
            MTK_AFE_RATE_48K
        }
    }
}

unsafe extern "C" fn dai_memif_rate_transform(dev: *mut device, rate: c_uint) -> c_uint {
    match rate {
        8000 => MTK_AFE_DAI_MEMIF_RATE_8K,
        16000 => MTK_AFE_DAI_MEMIF_RATE_16K,
        32000 => MTK_AFE_DAI_MEMIF_RATE_32K,
        48000 => MTK_AFE_DAI_MEMIF_RATE_48K,
        _ => {
            unsafe {
                dev_warn(
                    dev,
                    b"%s(), rate %u invalid, use %d!!!\n\0".as_ptr() as *const c_char,
                    b"dai_memif_rate_transform\0".as_ptr() as *const c_char,
                    rate,
                    MTK_AFE_DAI_MEMIF_RATE_16K as c_int,
                );
            }
            MTK_AFE_DAI_MEMIF_RATE_16K
        }
    }
}

unsafe extern "C" fn pcm_rate_transform(dev: *mut device, rate: c_uint) -> c_uint {
    match rate {
        8000 => MTK_AFE_PCM_RATE_8K,
        16000 => MTK_AFE_PCM_RATE_16K,
        32000 => MTK_AFE_PCM_RATE_32K,
        48000 => MTK_AFE_PCM_RATE_48K,
        _ => {
            unsafe {
                dev_warn(
                    dev,
                    b"%s(), rate %u invalid, use %d!!!\n\0".as_ptr() as *const c_char,
                    b"pcm_rate_transform\0".as_ptr() as *const c_char,
                    rate,
                    MTK_AFE_PCM_RATE_32K as c_int,
                );
            }
            MTK_AFE_PCM_RATE_32K
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8192_rate_transform(
    dev: *mut device,
    rate: c_uint,
    aud_blk: c_int,
) -> c_uint {
    if aud_blk == unsafe { MT8192_MEMIF_DAI } || aud_blk == unsafe { MT8192_MEMIF_MOD_DAI } {
        unsafe { dai_memif_rate_transform(dev, rate) }
    } else if aud_blk == unsafe { MT8192_DAI_PCM_1 } || aud_blk == unsafe { MT8192_DAI_PCM_2 } {
        unsafe { pcm_rate_transform(dev, rate) }
    } else {
        unsafe { mt8192_general_rate_transform(dev, rate) }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8192_dai_set_priv(
    afe: *mut mtk_base_afe,
    id: c_int,
    priv_size: c_int,
    priv_data: *const c_void,
) -> c_int {
    let afe_priv = unsafe { (*afe).platform_priv as *mut mt8192_afe_private };
    let temp_data: *mut c_void;

    temp_data = unsafe { devm_kzalloc((*afe).dev, priv_size, GFP_KERNEL) };
    if temp_data.is_null() {
        return -ENOMEM;
    }

    if !priv_data.is_null() {
        unsafe {
            memcpy(temp_data, priv_data, priv_size);
        }
    }

    unsafe {
        *(*afe_priv).dai_priv.offset(id as isize) = temp_data;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
