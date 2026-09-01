// SPDX-License-Identifier: GPL-2.0
/*
 * mt7986-wm8960.c  --  MT7986-WM8960 ALSA SoC machine driver
 *
 * Copyright (c) 2023 MediaTek Inc.
 * Authors: Vic Wu <vic.wu@mediatek.com>
 *          Maso Huang <maso.huang@mediatek.com>
 */

// Rust translation of dependencies from:
// #include <linux/module.h>
// #include <sound/soc.h>
// #include "mt7986-afe-common.h"

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub trigger: [c_int; 2],
    pub dynamic: c_int,
    pub playback_only: c_int,
    pub capture_only: c_int,
    pub no_pcm: c_int,
    pub dai_fmt: u32,
    pub platforms: *mut snd_soc_dai_link_component,
    pub codecs: *mut snd_soc_dai_link_component,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut module,
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

unsafe extern "C" {
    static mut THIS_MODULE: module;

    fn of_get_child_by_name(
        node: *mut device_node,
        name: *const c_char,
    ) -> *mut device_node;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char)
        -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

const EINVAL: c_int = 22;
const SND_SOC_DPCM_TRIGGER_POST: c_int = snd_soc_dpcm_trigger_post!();
const SND_SOC_DAIFMT_I2S: u32 = snd_soc_daifmt_i2s!();
const SND_SOC_DAIFMT_NB_NF: u32 = snd_soc_daifmt_nb_nf!();
const SND_SOC_DAIFMT_CBC_CFC: u32 = snd_soc_daifmt_cbc_cfc!();
const SND_SOC_DAIFMT_GATED: u32 = snd_soc_daifmt_gated!();

static mt7986_wm8960_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_hp!(c"Headphone".as_ptr(), ptr::null_mut()),
    snd_soc_dapm_mic!(c"AMIC".as_ptr(), ptr::null_mut()),
];

static mt7986_wm8960_controls: [snd_kcontrol_new; 2] = [
    soc_dapm_pin_switch!(c"Headphone".as_ptr()),
    soc_dapm_pin_switch!(c"AMIC".as_ptr()),
];

snd_soc_dailink_defs!(
    playback,
    dailink_comp_array!(comp_cpu!(c"DL1".as_ptr())),
    dailink_comp_array!(comp_dummy!()),
    dailink_comp_array!(comp_empty!())
);

snd_soc_dailink_defs!(
    capture,
    dailink_comp_array!(comp_cpu!(c"UL1".as_ptr())),
    dailink_comp_array!(comp_dummy!()),
    dailink_comp_array!(comp_empty!())
);

snd_soc_dailink_defs!(
    codec,
    dailink_comp_array!(comp_cpu!(c"ETDM".as_ptr())),
    dailink_comp_array!(comp_codec!(ptr::null(), c"wm8960-hifi".as_ptr())),
    dailink_comp_array!(comp_empty!())
);

static mut mt7986_wm8960_dai_links: [snd_soc_dai_link; 3] = [
    /* FE */
    snd_soc_dai_link {
        name: c"wm8960-playback".as_ptr(),
        stream_name: c"wm8960-playback".as_ptr(),
        trigger: [
            SND_SOC_DPCM_TRIGGER_POST,
            SND_SOC_DPCM_TRIGGER_POST,
        ],
        dynamic: 1,
        playback_only: 1,
        capture_only: 0,
        no_pcm: 0,
        dai_fmt: 0,
        platforms: snd_soc_dailink_reg_platforms!(playback),
        codecs: snd_soc_dailink_reg_codecs!(playback),
    },
    snd_soc_dai_link {
        name: c"wm8960-capture".as_ptr(),
        stream_name: c"wm8960-capture".as_ptr(),
        trigger: [
            SND_SOC_DPCM_TRIGGER_POST,
            SND_SOC_DPCM_TRIGGER_POST,
        ],
        dynamic: 1,
        playback_only: 0,
        capture_only: 1,
        no_pcm: 0,
        dai_fmt: 0,
        platforms: snd_soc_dailink_reg_platforms!(capture),
        codecs: snd_soc_dailink_reg_codecs!(capture),
    },
    /* BE */
    snd_soc_dai_link {
        name: c"wm8960-codec".as_ptr(),
        stream_name: ptr::null(),
        trigger: [0, 0],
        dynamic: 0,
        playback_only: 0,
        capture_only: 0,
        no_pcm: 1,
        dai_fmt: SND_SOC_DAIFMT_I2S
            | SND_SOC_DAIFMT_NB_NF
            | SND_SOC_DAIFMT_CBC_CFC
            | SND_SOC_DAIFMT_GATED,
        platforms: snd_soc_dailink_reg_platforms!(codec),
        codecs: snd_soc_dailink_reg_codecs!(codec),
    },
];

static mut mt7986_wm8960_card: snd_soc_card = snd_soc_card {
    name: c"mt7986-wm8960".as_ptr(),
    owner: unsafe { &raw mut THIS_MODULE },
    dev: ptr::null_mut(),
    dai_link: unsafe { mt7986_wm8960_dai_links.as_mut_ptr() },
    num_links: 3,
    controls: mt7986_wm8960_controls.as_ptr(),
    num_controls: 2,
    dapm_widgets: mt7986_wm8960_widgets.as_ptr(),
    num_dapm_widgets: 2,
};

unsafe extern "C" fn mt7986_wm8960_machine_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card = &raw mut mt7986_wm8960_card;
    let mut dai_link: *mut snd_soc_dai_link;
    let platform: *mut device_node;
    let codec: *mut device_node;
    let platform_dai_node: *mut device_node;
    let codec_dai_node: *mut device_node;
    let mut ret: c_int;
    let mut i: c_int;

    (*card).dev = &raw mut (*pdev).dev;

    platform = of_get_child_by_name((*pdev).dev.of_node, c"platform".as_ptr());

    if !platform.is_null() {
        platform_dai_node = of_parse_phandle(platform, c"sound-dai".as_ptr(), 0);
        of_node_put(platform);

        if platform_dai_node.is_null() {
            dev_err(
                &raw mut (*pdev).dev,
                c"Failed to parse platform/sound-dai property\n".as_ptr(),
            );
            return -EINVAL;
        }
    } else {
        dev_err(
            &raw mut (*pdev).dev,
            c"Property 'platform' missing or invalid\n".as_ptr(),
        );
        return -EINVAL;
    }

    i = 0;
    while i < (*card).num_links {
        dai_link = (*card).dai_link.offset(i as isize);
        if !(*(*dai_link).platforms).name.is_null() {
            i += 1;
            continue;
        }
        (*(*dai_link).platforms).of_node = platform_dai_node;
        i += 1;
    }

    codec = of_get_child_by_name((*pdev).dev.of_node, c"codec".as_ptr());

    if !codec.is_null() {
        codec_dai_node = of_parse_phandle(codec, c"sound-dai".as_ptr(), 0);
        of_node_put(codec);

        if codec_dai_node.is_null() {
            of_node_put(platform_dai_node);
            dev_err(
                &raw mut (*pdev).dev,
                c"Failed to parse codec/sound-dai property\n".as_ptr(),
            );
            return -EINVAL;
        }
    } else {
        of_node_put(platform_dai_node);
        dev_err(
            &raw mut (*pdev).dev,
            c"Property 'codec' missing or invalid\n".as_ptr(),
        );
        return -EINVAL;
    }

    i = 0;
    while i < (*card).num_links {
        dai_link = (*card).dai_link.offset(i as isize);
        if !(*(*dai_link).codecs).name.is_null() {
            i += 1;
            continue;
        }
        (*(*dai_link).codecs).of_node = codec_dai_node;
        i += 1;
    }

    ret = snd_soc_of_parse_audio_routing(card, c"audio-routing".as_ptr());
    if ret != 0 {
        dev_err(
            &raw mut (*pdev).dev,
            c"Failed to parse audio-routing: %d\n".as_ptr(),
            ret,
        );
        of_node_put(platform_dai_node);
        of_node_put(codec_dai_node);
        return ret;
    }

    ret = devm_snd_soc_register_card(&raw mut (*pdev).dev, card);
    if ret != 0 {
        dev_err_probe(
            &raw mut (*pdev).dev,
            ret,
            c"%s snd_soc_register_card fail\n".as_ptr(),
            c"mt7986_wm8960_machine_probe".as_ptr(),
        );
    }

    of_node_put(platform_dai_node);
    of_node_put(codec_dai_node);
    ret
}

static mt7986_wm8960_machine_dt_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"mediatek,mt7986-wm8960-sound".as_ptr(),
    },
    of_device_id {
        /* sentinel */
        compatible: ptr::null(),
    },
];
module_device_table!(of, mt7986_wm8960_machine_dt_match);

static mut mt7986_wm8960_machine: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: c"mt7986-wm8960".as_ptr(),
        of_match_table: mt7986_wm8960_machine_dt_match.as_ptr(),
    },
    probe: Some(mt7986_wm8960_machine_probe),
};

module_platform_driver!(mt7986_wm8960_machine);

/* Module information */
module_description!(c"MT7986 WM8960 ALSA SoC machine driver".as_ptr());
module_author!(c"Vic Wu <vic.wu@mediatek.com>".as_ptr());
module_license!(c"GPL".as_ptr());
module_alias!(c"mt7986 wm8960 soc card".as_ptr());

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
