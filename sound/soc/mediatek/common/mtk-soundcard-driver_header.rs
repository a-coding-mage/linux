/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mtk-soundcard-driver.h  --  MediaTek soundcard driver common definition
 *
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Trevor Wu <trevor.wu@mediatek.com>
 */

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct mtk_sof_priv {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtk_soc_card_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum mtk_pcm_constraint_type {
    MTK_CONSTRAINT_PLAYBACK,
    MTK_CONSTRAINT_CAPTURE,
    MTK_CONSTRAINT_HDMIDP,
    MTK_CONSTRAINT_MAX,
}

#[repr(C)]
pub struct mtk_pcm_constraints_data {
    pub channels: *const snd_pcm_hw_constraint_list,
    pub rates: *const snd_pcm_hw_constraint_list,
}

#[repr(C)]
pub struct mtk_platform_card_data {
    pub card: *mut snd_soc_card,
    pub jacks: *mut snd_soc_jack,
    pub pcm_constraints: *const mtk_pcm_constraints_data,
    pub num_jacks: u8,
    pub num_pcm_constraints: u8,
    pub flags: u8,
}

#[repr(C)]
pub struct mtk_soundcard_pdata {
    pub card_name: *const c_char,
    pub card_data: *mut mtk_platform_card_data,
    pub sof_priv: *const mtk_sof_priv,
    pub soc_probe: Option<
        unsafe extern "C" fn(card_data: *mut mtk_soc_card_data, legacy: bool) -> c_int,
    >,
}

unsafe extern "C" {
    /* Common playback/capture card startup ops */
    pub static mtk_soundcard_common_playback_ops: snd_soc_ops;
    pub static mtk_soundcard_common_capture_ops: snd_soc_ops;

    /* Exported for custom/extended soundcard startup ops */
    pub fn mtk_soundcard_startup(
        substream: *mut snd_pcm_substream,
        ctype: mtk_pcm_constraint_type,
    ) -> c_int;

    pub fn parse_dai_link_info(card: *mut snd_soc_card) -> c_int;
    pub fn clean_card_reference(card: *mut snd_soc_card);
    pub fn mtk_soundcard_common_probe(pdev: *mut platform_device) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
