// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio Control
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Jiaxin Yu <jiaxin.yu@mediatek.com>

// C dependency intent: #include "mt8186-afe-common.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

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
pub struct mt8186_afe_private {
    pub dai_priv: *mut *mut c_void,
}

pub type gfp_t = c_uint;

unsafe extern "C" {
    static MT8186_DAI_PCM: c_int;
    static MT8186_DAI_TDM_IN: c_int;
    static GFP_KERNEL: gfp_t;
    static ENOMEM: c_int;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: c_int, gfp: gfp_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: c_int) -> *mut c_void;
}

pub const MTK_AFE_RATE_8K: c_uint = 0;
pub const MTK_AFE_RATE_11K: c_uint = 1;
pub const MTK_AFE_RATE_12K: c_uint = 2;
pub const MTK_AFE_RATE_384K: c_uint = 3;
pub const MTK_AFE_RATE_16K: c_uint = 4;
pub const MTK_AFE_RATE_22K: c_uint = 5;
pub const MTK_AFE_RATE_24K: c_uint = 6;
pub const MTK_AFE_RATE_352K: c_uint = 7;
pub const MTK_AFE_RATE_32K: c_uint = 8;
pub const MTK_AFE_RATE_44K: c_uint = 9;
pub const MTK_AFE_RATE_48K: c_uint = 10;
pub const MTK_AFE_RATE_88K: c_uint = 11;
pub const MTK_AFE_RATE_96K: c_uint = 12;
pub const MTK_AFE_RATE_176K: c_uint = 13;
pub const MTK_AFE_RATE_192K: c_uint = 14;
pub const MTK_AFE_RATE_260K: c_uint = 15;

pub const MTK_AFE_PCM_RATE_8K: c_uint = 0;
pub const MTK_AFE_PCM_RATE_16K: c_uint = 1;
pub const MTK_AFE_PCM_RATE_32K: c_uint = 2;
pub const MTK_AFE_PCM_RATE_48K: c_uint = 3;

pub const MTK_AFE_TDM_RATE_8K: c_uint = 0;
pub const MTK_AFE_TDM_RATE_12K: c_uint = 1;
pub const MTK_AFE_TDM_RATE_16K: c_uint = 2;
pub const MTK_AFE_TDM_RATE_24K: c_uint = 3;
pub const MTK_AFE_TDM_RATE_32K: c_uint = 4;
pub const MTK_AFE_TDM_RATE_48K: c_uint = 5;
pub const MTK_AFE_TDM_RATE_64K: c_uint = 6;
pub const MTK_AFE_TDM_RATE_96K: c_uint = 7;
pub const MTK_AFE_TDM_RATE_128K: c_uint = 8;
pub const MTK_AFE_TDM_RATE_192K: c_uint = 9;
pub const MTK_AFE_TDM_RATE_256K: c_uint = 10;
pub const MTK_AFE_TDM_RATE_384K: c_uint = 11;
pub const MTK_AFE_TDM_RATE_11K: c_uint = 12;
pub const MTK_AFE_TDM_RATE_22K: c_uint = 13;
pub const MTK_AFE_TDM_RATE_44K: c_uint = 14;
pub const MTK_AFE_TDM_RATE_88K: c_uint = 15;
pub const MTK_AFE_TDM_RATE_176K: c_uint = 16;
pub const MTK_AFE_TDM_RATE_352K: c_uint = 17;

pub const MTK_AFE_TDM_RELATCH_RATE_8K: c_uint = 0;
pub const MTK_AFE_TDM_RELATCH_RATE_11K: c_uint = 1;
pub const MTK_AFE_TDM_RELATCH_RATE_12K: c_uint = 2;
pub const MTK_AFE_TDM_RELATCH_RATE_16K: c_uint = 3;
pub const MTK_AFE_TDM_RELATCH_RATE_22K: c_uint = 4;
pub const MTK_AFE_TDM_RELATCH_RATE_24K: c_uint = 5;
pub const MTK_AFE_TDM_RELATCH_RATE_32K: c_uint = 6;
pub const MTK_AFE_TDM_RELATCH_RATE_44K: c_uint = 7;
pub const MTK_AFE_TDM_RELATCH_RATE_48K: c_uint = 8;
pub const MTK_AFE_TDM_RELATCH_RATE_88K: c_uint = 9;
pub const MTK_AFE_TDM_RELATCH_RATE_96K: c_uint = 10;
pub const MTK_AFE_TDM_RELATCH_RATE_176K: c_uint = 11;
pub const MTK_AFE_TDM_RELATCH_RATE_192K: c_uint = 12;
pub const MTK_AFE_TDM_RELATCH_RATE_352K: c_uint = 13;
pub const MTK_AFE_TDM_RELATCH_RATE_384K: c_uint = 14;

#[no_mangle]
pub unsafe extern "C" fn mt8186_general_rate_transform(
    dev: *mut device,
    rate: c_uint,
) -> c_uint {
    match rate {
        8000 => return MTK_AFE_RATE_8K,
        11025 => return MTK_AFE_RATE_11K,
        12000 => return MTK_AFE_RATE_12K,
        16000 => return MTK_AFE_RATE_16K,
        22050 => return MTK_AFE_RATE_22K,
        24000 => return MTK_AFE_RATE_24K,
        32000 => return MTK_AFE_RATE_32K,
        44100 => return MTK_AFE_RATE_44K,
        48000 => return MTK_AFE_RATE_48K,
        88200 => return MTK_AFE_RATE_88K,
        96000 => return MTK_AFE_RATE_96K,
        176400 => return MTK_AFE_RATE_176K,
        192000 => return MTK_AFE_RATE_192K,
        260000 => return MTK_AFE_RATE_260K,
        352800 => return MTK_AFE_RATE_352K,
        384000 => return MTK_AFE_RATE_384K,
        _ => {
            dev_err(
                dev,
                c"%s(), rate %u invalid, use %d!!!\n".as_ptr(),
                c"mt8186_general_rate_transform".as_ptr(),
                rate,
                MTK_AFE_RATE_48K as c_int,
            );
        }
    }

    MTK_AFE_RATE_48K
}

unsafe fn tdm_rate_transform(dev: *mut device, rate: c_uint) -> c_uint {
    match rate {
        8000 => return MTK_AFE_TDM_RATE_8K,
        11025 => return MTK_AFE_TDM_RATE_11K,
        12000 => return MTK_AFE_TDM_RATE_12K,
        16000 => return MTK_AFE_TDM_RATE_16K,
        22050 => return MTK_AFE_TDM_RATE_22K,
        24000 => return MTK_AFE_TDM_RATE_24K,
        32000 => return MTK_AFE_TDM_RATE_32K,
        44100 => return MTK_AFE_TDM_RATE_44K,
        48000 => return MTK_AFE_TDM_RATE_48K,
        64000 => return MTK_AFE_TDM_RATE_64K,
        88200 => return MTK_AFE_TDM_RATE_88K,
        96000 => return MTK_AFE_TDM_RATE_96K,
        128000 => return MTK_AFE_TDM_RATE_128K,
        176400 => return MTK_AFE_TDM_RATE_176K,
        192000 => return MTK_AFE_TDM_RATE_192K,
        256000 => return MTK_AFE_TDM_RATE_256K,
        352800 => return MTK_AFE_TDM_RATE_352K,
        384000 => return MTK_AFE_TDM_RATE_384K,
        _ => {
            dev_err(
                dev,
                c"%s(), rate %u invalid, use %d!!!\n".as_ptr(),
                c"tdm_rate_transform".as_ptr(),
                rate,
                MTK_AFE_TDM_RATE_48K as c_int,
            );
        }
    }

    MTK_AFE_TDM_RATE_48K
}

unsafe fn pcm_rate_transform(dev: *mut device, rate: c_uint) -> c_uint {
    match rate {
        8000 => return MTK_AFE_PCM_RATE_8K,
        16000 => return MTK_AFE_PCM_RATE_16K,
        32000 => return MTK_AFE_PCM_RATE_32K,
        48000 => return MTK_AFE_PCM_RATE_48K,
        _ => {
            dev_err(
                dev,
                c"%s(), rate %u invalid, use %d!!!\n".as_ptr(),
                c"pcm_rate_transform".as_ptr(),
                rate,
                MTK_AFE_PCM_RATE_48K as c_int,
            );
        }
    }

    MTK_AFE_PCM_RATE_48K
}

#[no_mangle]
pub unsafe extern "C" fn mt8186_tdm_relatch_rate_transform(
    dev: *mut device,
    rate: c_uint,
) -> c_uint {
    match rate {
        8000 => return MTK_AFE_TDM_RELATCH_RATE_8K,
        11025 => return MTK_AFE_TDM_RELATCH_RATE_11K,
        12000 => return MTK_AFE_TDM_RELATCH_RATE_12K,
        16000 => return MTK_AFE_TDM_RELATCH_RATE_16K,
        22050 => return MTK_AFE_TDM_RELATCH_RATE_22K,
        24000 => return MTK_AFE_TDM_RELATCH_RATE_24K,
        32000 => return MTK_AFE_TDM_RELATCH_RATE_32K,
        44100 => return MTK_AFE_TDM_RELATCH_RATE_44K,
        48000 => return MTK_AFE_TDM_RELATCH_RATE_48K,
        88200 => return MTK_AFE_TDM_RELATCH_RATE_88K,
        96000 => return MTK_AFE_TDM_RELATCH_RATE_96K,
        176400 => return MTK_AFE_TDM_RELATCH_RATE_176K,
        192000 => return MTK_AFE_TDM_RELATCH_RATE_192K,
        352800 => return MTK_AFE_TDM_RELATCH_RATE_352K,
        384000 => return MTK_AFE_TDM_RELATCH_RATE_384K,
        _ => {
            dev_err(
                dev,
                c"%s(), rate %u invalid, use %d!!!\n".as_ptr(),
                c"mt8186_tdm_relatch_rate_transform".as_ptr(),
                rate,
                MTK_AFE_TDM_RELATCH_RATE_48K as c_int,
            );
        }
    }

    MTK_AFE_TDM_RELATCH_RATE_48K
}

#[no_mangle]
pub unsafe extern "C" fn mt8186_rate_transform(
    dev: *mut device,
    rate: c_uint,
    aud_blk: c_int,
) -> c_uint {
    if aud_blk == MT8186_DAI_PCM {
        return pcm_rate_transform(dev, rate);
    }
    if aud_blk == MT8186_DAI_TDM_IN {
        return tdm_rate_transform(dev, rate);
    }
    mt8186_general_rate_transform(dev, rate)
}

#[no_mangle]
pub unsafe extern "C" fn mt8186_dai_set_priv(
    afe: *mut mtk_base_afe,
    id: c_int,
    priv_size: c_int,
    priv_data: *const c_void,
) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;
    let temp_data: *mut c_void;

    temp_data = devm_kzalloc((*afe).dev, priv_size, GFP_KERNEL);
    if temp_data.is_null() {
        return -ENOMEM;
    }

    if !priv_data.is_null() {
        memcpy(temp_data, priv_data, priv_size);
    }

    *(*afe_priv).dai_priv.offset(id as isize) = temp_data;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
