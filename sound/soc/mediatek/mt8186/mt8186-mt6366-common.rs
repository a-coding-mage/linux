// SPDX-License-Identifier: GPL-2.0
//
// mt8186-mt6366-common.c
//	--  MT8186 MT6366 ALSA common driver
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Jiaxin Yu <jiaxin.yu@mediatek.com>
//
// C dependencies:
// <sound/soc.h>
// "../../codecs/mt6358.h"
// "../common/mtk-afe-platform-driver.h"
// "mt8186-afe-common.h"
// "mt8186-mt6366-common.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct mtk_base_afe {
    pub platform_priv: *mut mt8186_afe_private,
}

#[repr(C)]
pub struct mt8186_afe_private {
    pub mtkaif_protocol: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
}

pub const MT6358_MTKAIF_PROTOCOL_1: c_int = 1;

unsafe extern "C" {
    static AFE_PCM_NAME: *const c_char;

    fn snd_soc_rtdcom_lookup(
        rtd: *mut snd_soc_pcm_runtime,
        name: *const c_char,
    ) -> *mut snd_soc_component;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut mtk_base_afe;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn mt6358_set_mtkaif_protocol(component: *mut snd_soc_component, protocol: c_int);
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn snd_soc_of_get_dai_link_codecs(
        dev: *mut device,
        node: *mut device_node,
        link: *mut snd_soc_dai_link,
    ) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8186_mt6366_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let cmpnt_afe: *mut snd_soc_component = unsafe { snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME) };
    let cmpnt_codec: *mut snd_soc_component =
        unsafe { (*snd_soc_rtd_to_codec(rtd, 0)).component };
    let afe: *mut mtk_base_afe = unsafe { snd_soc_component_get_drvdata(cmpnt_afe) };
    let afe_priv: *mut mt8186_afe_private = unsafe { (*afe).platform_priv };
    let dapm: *mut snd_soc_dapm_context =
        unsafe { snd_soc_card_to_dapm((*rtd).card) };
    let ret: c_int;

    /* set mtkaif protocol */
    unsafe {
        mt6358_set_mtkaif_protocol(cmpnt_codec, MT6358_MTKAIF_PROTOCOL_1);
        (*afe_priv).mtkaif_protocol = MT6358_MTKAIF_PROTOCOL_1;
    }

    ret = unsafe { snd_soc_dapm_sync(dapm) };
    if ret != 0 {
        unsafe {
            dev_err(
                (*rtd).dev,
                c"failed to snd_soc_dapm_sync\n".as_ptr(),
            );
        }
        return ret;
    }

    return 0;
}
// EXPORT_SYMBOL_GPL(mt8186_mt6366_init);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8186_mt6366_card_set_be_link(
    dev: *mut device,
    link: *mut snd_soc_dai_link,
    node: *mut device_node,
    link_name: *mut c_char,
) -> c_int {
    let ret: c_int;

    if !node.is_null() && unsafe { strcmp((*link).name, link_name) } == 0 {
        ret = unsafe { snd_soc_of_get_dai_link_codecs(dev, node, link) };
        if ret < 0 {
            return unsafe { dev_err_probe(dev, ret, c"get dai link codecs fail\n".as_ptr()) };
        }
    }

    return 0;
}
// EXPORT_SYMBOL_GPL(mt8186_mt6366_card_set_be_link);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
