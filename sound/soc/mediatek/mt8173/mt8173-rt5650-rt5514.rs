// SPDX-License-Identifier: GPL-2.0
/*
 * mt8173-rt5650-rt5514.c  --  MT8173 machine driver with RT5650/5514 codecs
 *
 * Copyright (c) 2016 MediaTek Inc.
 * Author: Koro Chen <koro.chen@mediatek.com>
 */

// C dependencies:
// #include <linux/module.h>
// #include <sound/soc.h>
// #include <sound/jack.h>
// #include "../../codecs/rt5645.h"

const MCLK_FOR_CODECS: u32 = 12288000;

extern "C" {
    static mut THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: i32,
        source: i32,
        freq_in: u32,
        freq_out: u32,
    ) -> i32;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: i32,
        freq: u32,
        dir: i32,
    ) -> i32;
    fn params_rate(params: *mut snd_pcm_hw_params) -> u32;
    fn snd_soc_rtd_to_codec(
        runtime: *mut snd_soc_pcm_runtime,
        num: i32,
    ) -> *mut snd_soc_dai;
    fn rt5645_sel_asrc_clk_src(component: *mut snd_soc_component, filter_mask: u32, clk_src: u32);
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: i32,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: u32,
    ) -> i32;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> i32;
    fn rt5645_set_jack_detect(
        component: *mut snd_soc_component,
        hp_jack: *mut snd_soc_jack,
        mic_jack: *mut snd_soc_jack,
        btn_jack: *mut snd_soc_jack,
    ) -> i32;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: i32,
    ) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> i32;
}

type c_char = i8;

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
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
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
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
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
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
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: i32,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> i32>,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub trigger: [i32; 2],
    pub dynamic: i32,
    pub playback_only: i32,
    pub capture_only: i32,
    pub no_pcm: i32,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> i32>,
    pub dai_fmt: u32,
    pub ops: *const snd_soc_ops,
    pub ignore_pmdown_time: i32,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: u32,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: u32,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: u32,
}

#[repr(C)]
pub struct snd_soc_codec_conf {
    pub dlc: snd_soc_dai_link_component,
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut module,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: u32,
    pub codec_conf: *mut snd_soc_codec_conf,
    pub num_configs: u32,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: u32,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: u32,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: u32,
    pub dev: *mut device,
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
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
}

const SND_JACK_HEADPHONE: i32 = 0x0001;
const SND_JACK_MICROPHONE: i32 = 0x0002;
const SND_JACK_BTN_0: i32 = 0x4000;
const SND_JACK_BTN_1: i32 = 0x2000;
const SND_JACK_BTN_2: i32 = 0x1000;
const SND_JACK_BTN_3: i32 = 0x0800;
const SND_SOC_CLOCK_IN: i32 = 0;
const RT5645_DA_STEREO_FILTER: u32 = 0x1;
const RT5645_AD_STEREO_FILTER: u32 = 0x2;
const RT5645_CLK_SEL_I2S1_ASRC: u32 = 0;
const SND_SOC_DPCM_TRIGGER_POST: i32 = 1;
const SND_SOC_DAIFMT_I2S: u32 = 1;
const SND_SOC_DAIFMT_NB_NF: u32 = 0;
const SND_SOC_DAIFMT_CBC_CFC: u32 = 0;
const EINVAL: i32 = 22;

macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

// SND_SOC_DAPM_* and SOC_DAPM_PIN_SWITCH expand to dependency-defined
// struct initializers in <sound/soc.h>; their exact data layouts are external.
static mt8173_rt5650_rt5514_widgets: [snd_soc_dapm_widget; 4] = [
    SND_SOC_DAPM_SPK!("Speaker", None),
    SND_SOC_DAPM_MIC!("Int Mic", None),
    SND_SOC_DAPM_HP!("Headphone", None),
    SND_SOC_DAPM_MIC!("Headset Mic", None),
];

static mt8173_rt5650_rt5514_routes: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route {
        sink: c_str!("Speaker"),
        control: core::ptr::null(),
        source: c_str!("SPOL"),
    },
    snd_soc_dapm_route {
        sink: c_str!("Speaker"),
        control: core::ptr::null(),
        source: c_str!("SPOR"),
    },
    snd_soc_dapm_route {
        sink: c_str!("Sub DMIC1L"),
        control: core::ptr::null(),
        source: c_str!("Int Mic"),
    },
    snd_soc_dapm_route {
        sink: c_str!("Sub DMIC1R"),
        control: core::ptr::null(),
        source: c_str!("Int Mic"),
    },
    snd_soc_dapm_route {
        sink: c_str!("Headphone"),
        control: core::ptr::null(),
        source: c_str!("HPOL"),
    },
    snd_soc_dapm_route {
        sink: c_str!("Headphone"),
        control: core::ptr::null(),
        source: c_str!("HPOR"),
    },
    snd_soc_dapm_route {
        sink: c_str!("IN1P"),
        control: core::ptr::null(),
        source: c_str!("Headset Mic"),
    },
    snd_soc_dapm_route {
        sink: c_str!("IN1N"),
        control: core::ptr::null(),
        source: c_str!("Headset Mic"),
    },
];

static mt8173_rt5650_rt5514_controls: [snd_kcontrol_new; 4] = [
    SOC_DAPM_PIN_SWITCH!("Speaker"),
    SOC_DAPM_PIN_SWITCH!("Int Mic"),
    SOC_DAPM_PIN_SWITCH!("Headphone"),
    SOC_DAPM_PIN_SWITCH!("Headset Mic"),
];

static mut mt8173_rt5650_rt5514_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c_str!("Headphone"),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c_str!("Headset Mic"),
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe extern "C" fn mt8173_rt5650_rt5514_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> i32 {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let mut codec_dai: *mut snd_soc_dai;
    let mut i: i32;
    let mut ret: i32;

    // for_each_rtd_codec_dais(rtd, i, codec_dai)
    i = 0;
    while for_each_rtd_codec_dais!(rtd, i, codec_dai) {
        /* pll from mclk 12.288M */
        ret = snd_soc_dai_set_pll(
            codec_dai,
            0,
            0,
            MCLK_FOR_CODECS,
            params_rate(params).wrapping_mul(512),
        );
        if ret != 0 {
            return ret;
        }

        /* sysclk from pll */
        ret = snd_soc_dai_set_sysclk(
            codec_dai,
            1,
            params_rate(params).wrapping_mul(512),
            SND_SOC_CLOCK_IN,
        );
        if ret != 0 {
            return ret;
        }
    }
    0
}

static mt8173_rt5650_rt5514_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(mt8173_rt5650_rt5514_hw_params),
};

static mut mt8173_rt5650_rt5514_jack: snd_soc_jack = snd_soc_jack { _private: [] };

unsafe extern "C" fn mt8173_rt5650_rt5514_init(runtime: *mut snd_soc_pcm_runtime) -> i32 {
    let card: *mut snd_soc_card = (*runtime).card;
    let component: *mut snd_soc_component = (*snd_soc_rtd_to_codec(runtime, 0)).component;
    let ret: i32;

    rt5645_sel_asrc_clk_src(
        component,
        RT5645_DA_STEREO_FILTER | RT5645_AD_STEREO_FILTER,
        RT5645_CLK_SEL_I2S1_ASRC,
    );

    /* enable jack detection */
    ret = snd_soc_card_jack_new_pins(
        card,
        c_str!("Headset Jack"),
        SND_JACK_HEADPHONE
            | SND_JACK_MICROPHONE
            | SND_JACK_BTN_0
            | SND_JACK_BTN_1
            | SND_JACK_BTN_2
            | SND_JACK_BTN_3,
        &raw mut mt8173_rt5650_rt5514_jack,
        mt8173_rt5650_rt5514_jack_pins.as_mut_ptr(),
        mt8173_rt5650_rt5514_jack_pins.len() as u32,
    );
    if ret != 0 {
        dev_err((*card).dev, c_str!("Can't new Headset Jack %d\n"), ret);
        return ret;
    }

    rt5645_set_jack_detect(
        component,
        &raw mut mt8173_rt5650_rt5514_jack,
        &raw mut mt8173_rt5650_rt5514_jack,
        &raw mut mt8173_rt5650_rt5514_jack,
    )
}

const DAI_LINK_PLAYBACK: usize = 0;
const DAI_LINK_CAPTURE: usize = 1;
const DAI_LINK_CODEC_I2S: usize = 2;

// SND_SOC_DAILINK_DEFS(playback, ...)
static mut playback_cpus: [snd_soc_dai_link_component; 1] =
    [COMP_CPU!("DL1")];
static mut playback_codecs: [snd_soc_dai_link_component; 1] =
    [COMP_DUMMY!()];
static mut playback_platforms: [snd_soc_dai_link_component; 1] =
    [COMP_EMPTY!()];

// SND_SOC_DAILINK_DEFS(capture, ...)
static mut capture_cpus: [snd_soc_dai_link_component; 1] =
    [COMP_CPU!("VUL")];
static mut capture_codecs: [snd_soc_dai_link_component; 1] =
    [COMP_DUMMY!()];
static mut capture_platforms: [snd_soc_dai_link_component; 1] =
    [COMP_EMPTY!()];

// SND_SOC_DAILINK_DEFS(codec, ...)
static mut codec_cpus: [snd_soc_dai_link_component; 1] =
    [COMP_CPU!("I2S")];
static mut codec_codecs: [snd_soc_dai_link_component; 2] = [
    COMP_CODEC!(core::ptr::null(), "rt5645-aif1"),
    COMP_CODEC!(core::ptr::null(), "rt5514-aif1"),
];
static mut codec_platforms: [snd_soc_dai_link_component; 1] =
    [COMP_EMPTY!()];

/* Digital audio interface glue - connects codec <---> CPU */
static mut mt8173_rt5650_rt5514_dais: [snd_soc_dai_link; 3] = [
    /* Front End DAI links */
    snd_soc_dai_link {
        name: c_str!("rt5650_rt5514 Playback"),
        stream_name: c_str!("rt5650_rt5514 Playback"),
        trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
        dynamic: 1,
        playback_only: 1,
        capture_only: 0,
        no_pcm: 0,
        init: None,
        dai_fmt: 0,
        ops: core::ptr::null(),
        ignore_pmdown_time: 0,
        cpus: unsafe { playback_cpus.as_mut_ptr() },
        num_cpus: unsafe { playback_cpus.len() as u32 },
        codecs: unsafe { playback_codecs.as_mut_ptr() },
        num_codecs: unsafe { playback_codecs.len() as u32 },
        platforms: unsafe { playback_platforms.as_mut_ptr() },
        num_platforms: unsafe { playback_platforms.len() as u32 },
    },
    snd_soc_dai_link {
        name: c_str!("rt5650_rt5514 Capture"),
        stream_name: c_str!("rt5650_rt5514 Capture"),
        trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
        dynamic: 1,
        playback_only: 0,
        capture_only: 1,
        no_pcm: 0,
        init: None,
        dai_fmt: 0,
        ops: core::ptr::null(),
        ignore_pmdown_time: 0,
        cpus: unsafe { capture_cpus.as_mut_ptr() },
        num_cpus: unsafe { capture_cpus.len() as u32 },
        codecs: unsafe { capture_codecs.as_mut_ptr() },
        num_codecs: unsafe { capture_codecs.len() as u32 },
        platforms: unsafe { capture_platforms.as_mut_ptr() },
        num_platforms: unsafe { capture_platforms.len() as u32 },
    },
    /* Back End DAI links */
    snd_soc_dai_link {
        name: c_str!("Codec"),
        stream_name: core::ptr::null(),
        trigger: [0, 0],
        dynamic: 0,
        playback_only: 0,
        capture_only: 0,
        no_pcm: 1,
        init: Some(mt8173_rt5650_rt5514_init),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        ops: &mt8173_rt5650_rt5514_ops,
        ignore_pmdown_time: 1,
        cpus: unsafe { codec_cpus.as_mut_ptr() },
        num_cpus: unsafe { codec_cpus.len() as u32 },
        codecs: unsafe { codec_codecs.as_mut_ptr() },
        num_codecs: unsafe { codec_codecs.len() as u32 },
        platforms: unsafe { codec_platforms.as_mut_ptr() },
        num_platforms: unsafe { codec_platforms.len() as u32 },
    },
];

static mut mt8173_rt5650_rt5514_codec_conf: [snd_soc_codec_conf; 1] = [snd_soc_codec_conf {
    dlc: snd_soc_dai_link_component {
        name: core::ptr::null(),
        dai_name: core::ptr::null(),
        of_node: core::ptr::null_mut(),
    },
    name_prefix: c_str!("Sub"),
}];

static mut mt8173_rt5650_rt5514_card: snd_soc_card = unsafe {
    snd_soc_card {
        name: c_str!("mtk-rt5650-rt5514"),
        owner: THIS_MODULE,
        dai_link: mt8173_rt5650_rt5514_dais.as_mut_ptr(),
        num_links: mt8173_rt5650_rt5514_dais.len() as u32,
        codec_conf: mt8173_rt5650_rt5514_codec_conf.as_mut_ptr(),
        num_configs: mt8173_rt5650_rt5514_codec_conf.len() as u32,
        controls: mt8173_rt5650_rt5514_controls.as_ptr(),
        num_controls: mt8173_rt5650_rt5514_controls.len() as u32,
        dapm_widgets: mt8173_rt5650_rt5514_widgets.as_ptr(),
        num_dapm_widgets: mt8173_rt5650_rt5514_widgets.len() as u32,
        dapm_routes: mt8173_rt5650_rt5514_routes.as_ptr(),
        num_dapm_routes: mt8173_rt5650_rt5514_routes.len() as u32,
        dev: core::ptr::null_mut(),
    }
};

unsafe extern "C" fn mt8173_rt5650_rt5514_dev_probe(pdev: *mut platform_device) -> i32 {
    let card: *mut snd_soc_card = &raw mut mt8173_rt5650_rt5514_card;
    let platform_node: *mut device_node;
    let mut dai_link: *mut snd_soc_dai_link;
    let mut i: i32;
    let mut ret: i32;

    platform_node = of_parse_phandle((*pdev).dev.of_node, c_str!("mediatek,platform"), 0);
    if platform_node.is_null() {
        dev_err(
            &raw mut (*pdev).dev,
            c_str!("Property 'platform' missing or invalid\n"),
        );
        return -EINVAL;
    }

    // for_each_card_prelinks(card, i, dai_link)
    i = 0;
    while for_each_card_prelinks!(card, i, dai_link) {
        if !(*(*dai_link).platforms).name.is_null() {
            continue;
        }
        (*(*dai_link).platforms).of_node = platform_node;
    }

    mt8173_rt5650_rt5514_dais[DAI_LINK_CODEC_I2S].codecs.as_mut().unwrap().of_node =
        of_parse_phandle((*pdev).dev.of_node, c_str!("mediatek,audio-codec"), 0);
    if mt8173_rt5650_rt5514_dais[DAI_LINK_CODEC_I2S].codecs.as_ref().unwrap().of_node.is_null() {
        dev_err(
            &raw mut (*pdev).dev,
            c_str!("Property 'audio-codec' missing or invalid\n"),
        );
        ret = -EINVAL;
        of_node_put(platform_node);
        return ret;
    }
    (*mt8173_rt5650_rt5514_dais[DAI_LINK_CODEC_I2S].codecs.add(1)).of_node =
        of_parse_phandle((*pdev).dev.of_node, c_str!("mediatek,audio-codec"), 1);
    if (*mt8173_rt5650_rt5514_dais[DAI_LINK_CODEC_I2S].codecs.add(1))
        .of_node
        .is_null()
    {
        dev_err(
            &raw mut (*pdev).dev,
            c_str!("Property 'audio-codec' missing or invalid\n"),
        );
        ret = -EINVAL;
        of_node_put(platform_node);
        return ret;
    }
    mt8173_rt5650_rt5514_codec_conf[0].dlc.of_node =
        (*mt8173_rt5650_rt5514_dais[DAI_LINK_CODEC_I2S].codecs.add(1)).of_node;

    (*card).dev = &raw mut (*pdev).dev;

    ret = devm_snd_soc_register_card(&raw mut (*pdev).dev, card);

    of_node_put(platform_node);
    ret
}

static mt8173_rt5650_rt5514_dt_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c_str!("mediatek,mt8173-rt5650-rt5514"),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, mt8173_rt5650_rt5514_dt_match);

static mut mt8173_rt5650_rt5514_driver: platform_driver = unsafe {
    platform_driver {
        driver: device_driver {
            name: c_str!("mtk-rt5650-rt5514"),
            of_match_table: mt8173_rt5650_rt5514_dt_match.as_ptr(),
            pm: &snd_soc_pm_ops,
        },
        probe: Some(mt8173_rt5650_rt5514_dev_probe),
    }
};

// module_platform_driver(mt8173_rt5650_rt5514_driver);

/* Module information */
// MODULE_DESCRIPTION("MT8173 RT5650 and RT5514 SoC machine driver");
// MODULE_AUTHOR("Koro Chen <koro.chen@mediatek.com>");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:mtk-rt5650-rt5514");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
