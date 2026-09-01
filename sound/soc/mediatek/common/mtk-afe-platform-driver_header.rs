/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mtk-afe-platform-driver.h  --  Mediatek afe platform driver definition
 *
 * Copyright (c) 2016 MediaTek Inc.
 * Author: Garlic Tseng <garlic.tseng@mediatek.com>
 */

pub const AFE_PCM_NAME: &[u8] = b"mtk-afe-pcm\0";

unsafe extern "C" {
    pub static mtk_afe_pcm_platform: snd_soc_component_driver;
}

pub const fn MTK_ALIGN_16BYTES(x: u64) -> u64 {
    x & GENMASK_ULL(39, 4)
}

#[repr(C)]
pub struct mtk_base_afe {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn mtk_afe_pcm_pointer(
        component: *mut snd_soc_component,
        substream: *mut snd_pcm_substream,
    ) -> snd_pcm_uframes_t;
    pub fn mtk_afe_pcm_new(
        component: *mut snd_soc_component,
        rtd: *mut snd_soc_pcm_runtime,
    ) -> ::core::ffi::c_int;

    pub fn mtk_afe_combine_sub_dai(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
    pub fn mtk_afe_add_sub_dai_control(component: *mut snd_soc_component) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
