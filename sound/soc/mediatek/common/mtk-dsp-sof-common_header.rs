/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mtk-dsp-sof-common.h  --  MediaTek dsp sof common definition
 *
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Chunxu Li <chunxu.li@mediatek.com>
 */

/* Dependency intent from C header: #include <sound/soc.h> */

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sof_conn_stream {
    pub normal_link: *const c_char,
    pub sof_link: *const c_char,
    pub sof_dma: *const c_char,
    pub stream_dir: c_int,
}

#[repr(C)]
pub struct mtk_dai_link {
    pub name: *const c_char,
    pub be_hw_params_fixup: Option<
        unsafe extern "C" fn(
            rtd: *mut snd_soc_pcm_runtime,
            params: *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
    pub list: list_head,
}

#[repr(C)]
pub struct mtk_sof_priv {
    pub conn_streams: *const sof_conn_stream,
    pub num_streams: c_int,
    pub sof_dai_link_fixup: Option<
        unsafe extern "C" fn(
            rtd: *mut snd_soc_pcm_runtime,
            params: *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
}

unsafe extern "C" {
    pub fn mtk_sof_dai_link_fixup(
        rtd: *mut snd_soc_pcm_runtime,
        params: *mut snd_pcm_hw_params,
    ) -> c_int;
    pub fn mtk_sof_card_probe(card: *mut snd_soc_card) -> c_int;
    pub fn mtk_sof_card_late_probe(card: *mut snd_soc_card) -> c_int;
    pub fn mtk_sof_dailink_parse_of(
        dev: *mut device,
        card: *mut snd_soc_card,
        propname: *const c_char,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
