// SPDX-License-Identifier: GPL-2.0
/*
 * mt8173-max98090.c  --  MT8173 MAX98090 ALSA SoC machine driver
 *
 * Copyright (c) 2015 MediaTek Inc.
 * Author: Koro Chen <koro.chen@mediatek.com>
 */

// Dependencies from:
// #include <linux/module.h>
// #include <sound/soc.h>
// #include <sound/jack.h>

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
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
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
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
pub struct snd_soc_component {
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
    pub dynamic: c_uint,
    pub playback_only: c_uint,
    pub capture_only: c_uint,
    pub no_pcm: c_uint,
    pub init: Option<unsafe extern "C" fn(runtime: *mut snd_soc_pcm_runtime) -> c_int>,
    pub ops: *const snd_soc_ops,
    pub dai_fmt: c_uint,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut module,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub dev: *mut device,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SND_JACK_HEADSET: c_int = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DPCM_TRIGGER_POST: c_int = 1;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const EINVAL: c_int = 22;

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

static mut mt8173_max98090_jack: snd_soc_jack = snd_soc_jack { _private: [] };

static mut mt8173_max98090_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headphone".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
];

// SND_SOC_DAPM_SPK("Speaker", NULL),
// SND_SOC_DAPM_MIC("Int Mic", NULL),
// SND_SOC_DAPM_HP("Headphone", NULL),
// SND_SOC_DAPM_MIC("Headset Mic", NULL),
static mt8173_max98090_widgets: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

static mt8173_max98090_routes: [snd_soc_dapm_route; 7] = [
    snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: ptr::null(),
        source: c"SPKL".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: ptr::null(),
        source: c"SPKR".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"DMICL".as_ptr(),
        control: ptr::null(),
        source: c"Int Mic".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Headphone".as_ptr(),
        control: ptr::null(),
        source: c"HPL".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Headphone".as_ptr(),
        control: ptr::null(),
        source: c"HPR".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Headset Mic".as_ptr(),
        control: ptr::null(),
        source: c"MICBIAS".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"IN34".as_ptr(),
        control: ptr::null(),
        source: c"Headset Mic".as_ptr(),
    },
];

// SOC_DAPM_PIN_SWITCH("Speaker"),
// SOC_DAPM_PIN_SWITCH("Int Mic"),
// SOC_DAPM_PIN_SWITCH("Headphone"),
// SOC_DAPM_PIN_SWITCH("Headset Mic"),
static mt8173_max98090_controls: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

unsafe extern "C" fn mt8173_max98090_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);

    snd_soc_dai_set_sysclk(
        codec_dai,
        0,
        params_rate(params).wrapping_mul(256),
        SND_SOC_CLOCK_IN,
    )
}

static mt8173_max98090_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(mt8173_max98090_hw_params),
};

unsafe extern "C" fn mt8173_max98090_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let ret: c_int;
    let card: *mut snd_soc_card = (*runtime).card;
    let component: *mut snd_soc_component = (*snd_soc_rtd_to_codec(runtime, 0)).component;

    /* enable jack detection */
    ret = snd_soc_card_jack_new_pins(
        card,
        c"Headphone".as_ptr(),
        SND_JACK_HEADSET,
        &raw mut mt8173_max98090_jack,
        mt8173_max98090_jack_pins.as_mut_ptr(),
        mt8173_max98090_jack_pins.len() as c_uint,
    );
    if ret != 0 {
        dev_err((*card).dev, c"Can't create a new Jack %d\n".as_ptr(), ret);
        return ret;
    }

    snd_soc_component_set_jack(component, &raw mut mt8173_max98090_jack, ptr::null_mut())
}

// SND_SOC_DAILINK_DEFS(playback,
//     DAILINK_COMP_ARRAY(COMP_CPU("DL1")),
//     DAILINK_COMP_ARRAY(COMP_DUMMY()),
//     DAILINK_COMP_ARRAY(COMP_EMPTY()));
static mut playback_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"DL1".as_ptr(),
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

// SND_SOC_DAILINK_DEFS(capture,
//     DAILINK_COMP_ARRAY(COMP_CPU("VUL")),
//     DAILINK_COMP_ARRAY(COMP_DUMMY()),
//     DAILINK_COMP_ARRAY(COMP_EMPTY()));
static mut capture_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"VUL".as_ptr(),
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

// SND_SOC_DAILINK_DEFS(hifi,
//     DAILINK_COMP_ARRAY(COMP_CPU("I2S")),
//     DAILINK_COMP_ARRAY(COMP_CODEC(NULL, "HiFi")),
//     DAILINK_COMP_ARRAY(COMP_EMPTY()));
static mut hifi_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"I2S".as_ptr(),
    of_node: ptr::null_mut(),
}];
static mut hifi_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut hifi_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    of_node: ptr::null_mut(),
}];

/* Digital audio interface glue - connects codec <---> CPU */
static mut mt8173_max98090_dais: [snd_soc_dai_link; 3] = [
    /* Front End DAI links */
    snd_soc_dai_link {
        name: c"MAX98090 Playback".as_ptr(),
        stream_name: c"MAX98090 Playback".as_ptr(),
        trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
        dynamic: 1,
        playback_only: 1,
        capture_only: 0,
        no_pcm: 0,
        init: None,
        ops: ptr::null(),
        dai_fmt: 0,
        cpus: unsafe { playback_cpus.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { playback_codecs.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { playback_platforms.as_mut_ptr() },
        num_platforms: 1,
    },
    snd_soc_dai_link {
        name: c"MAX98090 Capture".as_ptr(),
        stream_name: c"MAX98090 Capture".as_ptr(),
        trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
        dynamic: 1,
        playback_only: 0,
        capture_only: 1,
        no_pcm: 0,
        init: None,
        ops: ptr::null(),
        dai_fmt: 0,
        cpus: unsafe { capture_cpus.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { capture_codecs.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { capture_platforms.as_mut_ptr() },
        num_platforms: 1,
    },
    /* Back End DAI links */
    snd_soc_dai_link {
        name: c"Codec".as_ptr(),
        stream_name: ptr::null(),
        trigger: [0, 0],
        dynamic: 0,
        playback_only: 0,
        capture_only: 0,
        no_pcm: 1,
        init: Some(mt8173_max98090_init),
        ops: &mt8173_max98090_ops,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        cpus: unsafe { hifi_cpus.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { hifi_codecs.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { hifi_platforms.as_mut_ptr() },
        num_platforms: 1,
    },
];

static mut mt8173_max98090_card: snd_soc_card = snd_soc_card {
    name: c"mt8173-max98090".as_ptr(),
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { mt8173_max98090_dais.as_mut_ptr() },
    num_links: 3,
    controls: mt8173_max98090_controls.as_ptr(),
    num_controls: 4,
    dapm_widgets: mt8173_max98090_widgets.as_ptr(),
    num_dapm_widgets: 4,
    dapm_routes: mt8173_max98090_routes.as_ptr(),
    num_dapm_routes: 7,
    dev: ptr::null_mut(),
};

unsafe extern "C" fn mt8173_max98090_dev_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card = &raw mut mt8173_max98090_card;
    let mut codec_node: *mut device_node;
    let platform_node: *mut device_node;
    let mut ret: c_int;

    platform_node = of_parse_phandle((*pdev).dev.of_node, c"mediatek,platform".as_ptr(), 0);
    if platform_node.is_null() {
        dev_err(
            &raw mut (*pdev).dev,
            c"Property 'platform' missing or invalid\n".as_ptr(),
        );
        return -EINVAL;
    }
    for i in 0..(*card).num_links {
        let dai_link: *mut snd_soc_dai_link = (*card).dai_link.add(i as usize);
        if !(*(*dai_link).platforms).name.is_null() {
            continue;
        }
        (*(*dai_link).platforms).of_node = platform_node;
    }

    codec_node = of_parse_phandle((*pdev).dev.of_node, c"mediatek,audio-codec".as_ptr(), 0);
    if codec_node.is_null() {
        dev_err(
            &raw mut (*pdev).dev,
            c"Property 'audio-codec' missing or invalid\n".as_ptr(),
        );
        ret = -EINVAL;
        of_node_put(platform_node);
        return ret;
    }
    for i in 0..(*card).num_links {
        let dai_link: *mut snd_soc_dai_link = (*card).dai_link.add(i as usize);
        if !(*(*dai_link).codecs).name.is_null() {
            continue;
        }
        (*(*dai_link).codecs).of_node = codec_node;
    }
    (*card).dev = &raw mut (*pdev).dev;

    ret = devm_snd_soc_register_card(&raw mut (*pdev).dev, card);

    of_node_put(codec_node);

    of_node_put(platform_node);
    ret
}

static mt8173_max98090_dt_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"mediatek,mt8173-max98090".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, mt8173_max98090_dt_match);

static mut mt8173_max98090_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"mt8173-max98090".as_ptr(),
        of_match_table: mt8173_max98090_dt_match.as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
    },
    probe: Some(mt8173_max98090_dev_probe),
};

// module_platform_driver(mt8173_max98090_driver);

/* Module information */
// MODULE_DESCRIPTION("MT8173 MAX98090 ALSA SoC machine driver");
// MODULE_AUTHOR("Koro Chen <koro.chen@mediatek.com>");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:mt8173-max98090");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
