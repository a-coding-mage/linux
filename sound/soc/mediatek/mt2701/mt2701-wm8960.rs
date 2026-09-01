// SPDX-License-Identifier: GPL-2.0
/*
 * mt2701-wm8960.c  --  MT2701 WM8960 ALSA SoC machine driver
 *
 * Copyright (c) 2017 MediaTek Inc.
 * Author: Ryder Lee <ryder.lee@mediatek.com>
 */

/* Dependencies from the original C file:
 * <linux/module.h>
 * <sound/soc.h>
 * "mt2701-afe-common.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
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
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            params: *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub trigger: [c_uint; 2],
    pub dynamic: c_uint,
    pub playback_only: c_uint,
    pub capture_only: c_uint,
    pub no_pcm: c_uint,
    pub dai_fmt: c_uint,
    pub ops: *const snd_soc_ops,
    pub platforms: *mut snd_soc_dai_link_component,
    pub codecs: *mut snd_soc_dai_link_component,
    pub cpus: *mut snd_soc_dai_link_component,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dev: *mut device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    #[cfg(CONFIG_OF)]
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> c_int>,
}

const EINVAL: c_int = 22;
const SND_SOC_CLOCK_OUT: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 1;
const SND_SOC_DPCM_TRIGGER_POST: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_GATED: c_uint = 0;

extern "C" {
    static mut THIS_MODULE: *mut c_void;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn of_node_put(node: *mut device_node);
}

/* Original C uses SND_SOC_DAPM_HP and SND_SOC_DAPM_MIC initializers. */
static mt2701_wm8960_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

/* Original C uses SOC_DAPM_PIN_SWITCH initializers. */
static mt2701_wm8960_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

unsafe extern "C" fn mt2701_wm8960_be_ops_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let cpu_dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mclk_rate: c_uint;
    let rate: c_uint = params_rate(params);
    let div_mclk_over_bck: c_uint = if rate > 192000 { 2 } else { 4 };
    let div_bck_over_lrck: c_uint = 64;

    mclk_rate = rate
        .wrapping_mul(div_bck_over_lrck)
        .wrapping_mul(div_mclk_over_bck);

    snd_soc_dai_set_sysclk(cpu_dai, 0, mclk_rate, SND_SOC_CLOCK_OUT);
    snd_soc_dai_set_sysclk(codec_dai, 0, mclk_rate, SND_SOC_CLOCK_IN);

    0
}

static mt2701_wm8960_be_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(mt2701_wm8960_be_ops_hw_params),
};

/* SND_SOC_DAILINK_DEFS(playback, ...). */
static mut playback_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"PCMO0\0".as_ptr() as *const c_char,
    of_node: ptr::null_mut(),
}];
static mut playback_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut playback_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    of_node: ptr::null_mut(),
}];

/* SND_SOC_DAILINK_DEFS(capture, ...). */
static mut capture_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"PCM0\0".as_ptr() as *const c_char,
    of_node: ptr::null_mut(),
}];
static mut capture_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut capture_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    of_node: ptr::null_mut(),
}];

/* SND_SOC_DAILINK_DEFS(codec, ...). */
static mut codec_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"I2S0\0".as_ptr() as *const c_char,
    of_node: ptr::null_mut(),
}];
static mut codec_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut codec_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut mt2701_wm8960_dai_links: [snd_soc_dai_link; 3] = [
    /* FE */
    snd_soc_dai_link {
        name: b"wm8960-playback\0".as_ptr() as *const c_char,
        stream_name: b"wm8960-playback\0".as_ptr() as *const c_char,
        trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
        dynamic: 1,
        playback_only: 1,
        capture_only: 0,
        no_pcm: 0,
        dai_fmt: 0,
        ops: ptr::null(),
        cpus: playback_cpus.as_mut_ptr(),
        codecs: playback_codecs.as_mut_ptr(),
        platforms: playback_platforms.as_mut_ptr(),
    },
    snd_soc_dai_link {
        name: b"wm8960-capture\0".as_ptr() as *const c_char,
        stream_name: b"wm8960-capture\0".as_ptr() as *const c_char,
        trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
        dynamic: 1,
        playback_only: 0,
        capture_only: 1,
        no_pcm: 0,
        dai_fmt: 0,
        ops: ptr::null(),
        cpus: capture_cpus.as_mut_ptr(),
        codecs: capture_codecs.as_mut_ptr(),
        platforms: capture_platforms.as_mut_ptr(),
    },
    /* BE */
    snd_soc_dai_link {
        name: b"wm8960-codec\0".as_ptr() as *const c_char,
        stream_name: ptr::null(),
        trigger: [0, 0],
        dynamic: 0,
        playback_only: 0,
        capture_only: 0,
        no_pcm: 1,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_GATED,
        ops: &mt2701_wm8960_be_ops,
        cpus: codec_cpus.as_mut_ptr(),
        codecs: codec_codecs.as_mut_ptr(),
        platforms: codec_platforms.as_mut_ptr(),
    },
];

static mut mt2701_wm8960_card: snd_soc_card = snd_soc_card {
    name: b"mt2701-wm8960\0".as_ptr() as *const c_char,
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { mt2701_wm8960_dai_links.as_mut_ptr() },
    num_links: 3,
    controls: mt2701_wm8960_controls.as_ptr(),
    num_controls: 2,
    dapm_widgets: mt2701_wm8960_widgets.as_ptr(),
    num_dapm_widgets: 2,
    dev: ptr::null_mut(),
};

unsafe extern "C" fn mt2701_wm8960_machine_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card = &mut mt2701_wm8960_card;
    let platform_node: *mut device_node;
    let codec_node: *mut device_node;
    let mut dai_link: *mut snd_soc_dai_link;
    let mut ret: c_int;
    let mut i: c_int;

    platform_node = of_parse_phandle(
        (*pdev).dev.of_node,
        b"mediatek,platform\0".as_ptr() as *const c_char,
        0,
    );
    if platform_node.is_null() {
        dev_err(
            &mut (*pdev).dev,
            b"Property 'platform' missing or invalid\n\0".as_ptr() as *const c_char,
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
        (*(*dai_link).platforms).of_node = platform_node;
        i += 1;
    }

    (*card).dev = &mut (*pdev).dev;

    codec_node = of_parse_phandle(
        (*pdev).dev.of_node,
        b"mediatek,audio-codec\0".as_ptr() as *const c_char,
        0,
    );
    if codec_node.is_null() {
        dev_err(
            &mut (*pdev).dev,
            b"Property 'audio-codec' missing or invalid\n\0".as_ptr() as *const c_char,
        );
        ret = -EINVAL;
        goto_put_platform_node(platform_node);
        return ret;
    }

    i = 0;
    while i < (*card).num_links {
        dai_link = (*card).dai_link.offset(i as isize);
        if !(*(*dai_link).codecs).name.is_null() {
            i += 1;
            continue;
        }
        (*(*dai_link).codecs).of_node = codec_node;
        i += 1;
    }

    ret = snd_soc_of_parse_audio_routing(card, b"audio-routing\0".as_ptr() as *const c_char);
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"failed to parse audio-routing: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        goto_put_codec_node(codec_node, platform_node);
        return ret;
    }

    ret = devm_snd_soc_register_card(&mut (*pdev).dev, card);
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"%s snd_soc_register_card fail %d\n\0".as_ptr() as *const c_char,
            b"mt2701_wm8960_machine_probe\0".as_ptr() as *const c_char,
            ret,
        );
    }

    of_node_put(codec_node);
    of_node_put(platform_node);
    ret
}

unsafe fn goto_put_codec_node(codec_node: *mut device_node, platform_node: *mut device_node) {
    of_node_put(codec_node);
    of_node_put(platform_node);
}

unsafe fn goto_put_platform_node(platform_node: *mut device_node) {
    of_node_put(platform_node);
}

#[cfg(CONFIG_OF)]
static mt2701_wm8960_machine_dt_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"mediatek,mt2701-wm8960-machine\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, mt2701_wm8960_machine_dt_match); */

static mut mt2701_wm8960_machine: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: b"mt2701-wm8960\0".as_ptr() as *const c_char,
        #[cfg(CONFIG_OF)]
        of_match_table: mt2701_wm8960_machine_dt_match.as_ptr(),
    },
    probe: Some(mt2701_wm8960_machine_probe),
};

/* module_platform_driver(mt2701_wm8960_machine); */

/* Module information */
/* MODULE_DESCRIPTION("MT2701 WM8960 ALSA SoC machine driver"); */
/* MODULE_AUTHOR("Ryder Lee <ryder.lee@mediatek.com>"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("mt2701 wm8960 soc card"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
