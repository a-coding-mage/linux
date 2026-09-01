// SPDX-License-Identifier: GPL-2.0
/*
 * mt8173-rt5650.c  --  MT8173 machine driver with RT5650 codecs
 *
 * Copyright (c) 2016 MediaTek Inc.
 * Author: Koro Chen <koro.chen@mediatek.com>
 */

/* Dependencies in the original C source:
 * linux/module.h, sound/soc.h, sound/jack.h, ../../codecs/rt5645.h
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const MCLK_FOR_CODECS: c_uint = 12288000;
const EINVAL: c_int = 22;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum mt8173_rt5650_mclk {
    MT8173_RT5650_MCLK_EXTERNAL = 0,
    MT8173_RT5650_MCLK_INTERNAL,
}

#[repr(C)]
struct mt8173_rt5650_platform_data {
    pll_from: mt8173_rt5650_mclk,
    /* 0 = external oscillator; 1 = internal source from mt8173 */
}

static mut mt8173_rt5650_priv: mt8173_rt5650_platform_data =
    mt8173_rt5650_platform_data {
        pll_from: mt8173_rt5650_mclk::MT8173_RT5650_MCLK_EXTERNAL,
    };

#[repr(C)]
struct device {
    of_node: *mut device_node,
}

#[repr(C)]
struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai_link_component {
    name: *const c_char,
    dai_name: *const c_char,
    of_node: *mut device_node,
}

#[repr(C)]
struct snd_soc_dai_link {
    name: *const c_char,
    stream_name: *const c_char,
    trigger: [c_uint; 2],
    dynamic: c_uint,
    playback_only: c_uint,
    capture_only: c_uint,
    no_pcm: c_uint,
    init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    dai_fmt: c_uint,
    ops: *const snd_soc_ops,
    ignore_pmdown_time: c_uint,
    cpus: *mut snd_soc_dai_link_component,
    num_cpus: c_uint,
    codecs: *mut snd_soc_dai_link_component,
    num_codecs: c_uint,
    platforms: *mut snd_soc_dai_link_component,
    num_platforms: c_uint,
}

#[repr(C)]
struct snd_soc_card {
    name: *const c_char,
    owner: *mut c_void,
    dai_link: *mut snd_soc_dai_link,
    num_links: c_uint,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
}

#[repr(C)]
struct snd_soc_pcm_runtime_codec {
    component: *mut snd_soc_component,
    name: *const c_char,
}

#[repr(C)]
struct snd_soc_ops {
    hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int,
    >,
}

#[repr(C)]
struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_jack_pin {
    pin: *const c_char,
    mask: c_uint,
}

#[repr(C)]
struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
    pm: *const c_void,
}

#[repr(C)]
struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

unsafe extern "C" {
    static THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: c_void;

    fn snd_soc_substream_to_rtd(
        substream: *mut snd_pcm_substream,
    ) -> *mut snd_soc_pcm_runtime;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_rtd_to_codec(
        runtime: *mut snd_soc_pcm_runtime,
        num: c_int,
    ) -> *mut snd_soc_pcm_runtime_codec;
    fn rt5645_sel_asrc_clk_src(
        component: *mut snd_soc_component,
        filter_mask: c_uint,
        clk_src: c_uint,
    );
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_uint,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn rt5645_set_jack_detect(
        component: *mut snd_soc_component,
        hp_jack: *mut snd_soc_jack,
        mic_jack: *mut snd_soc_jack,
        btn_jack: *mut snd_soc_jack,
    ) -> c_int;
    fn snd_soc_card_jack_new(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_uint,
        jack: *mut snd_soc_jack,
    ) -> c_int;
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_get_child_by_name(
        node: *mut device_node,
        name: *const c_char,
    ) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn snd_soc_of_get_dai_name(
        of_node: *mut device_node,
        dai_name: *mut *const c_char,
        index: c_int,
    ) -> c_int;
    fn device_property_present(dev: *mut device, propname: *const c_char) -> bool;
    fn device_property_read_u32(
        dev: *mut device,
        propname: *const c_char,
        val: *mut mt8173_rt5650_mclk,
    ) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

const SND_JACK_HEADPHONE: c_uint = 0x0001;
const SND_JACK_MICROPHONE: c_uint = 0x0002;
const SND_JACK_BTN_0: c_uint = 0x0004;
const SND_JACK_BTN_1: c_uint = 0x0008;
const SND_JACK_BTN_2: c_uint = 0x0010;
const SND_JACK_BTN_3: c_uint = 0x0020;
const SND_JACK_AVOUT: c_uint = 0x0040;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DPCM_TRIGGER_POST: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const RT5645_DA_STEREO_FILTER: c_uint = 0;
const RT5645_AD_STEREO_FILTER: c_uint = 0;
const RT5645_CLK_SEL_I2S1_ASRC: c_uint = 0;
const RT5645_CLK_SEL_I2S2_ASRC: c_uint = 0;

/* Original C macro initializers translated as Rust macro invocations supplied by future dependencies. */
static mt8173_rt5650_widgets: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_spk!("Ext Spk", ptr::null()),
    snd_soc_dapm_mic!("Int Mic", ptr::null()),
    snd_soc_dapm_hp!("Headphone", ptr::null()),
    snd_soc_dapm_mic!("Headset Mic", ptr::null()),
];

static mt8173_rt5650_routes: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route {
        sink: c"Ext Spk".as_ptr(),
        control: ptr::null(),
        source: c"SPOL".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Ext Spk".as_ptr(),
        control: ptr::null(),
        source: c"SPOR".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"DMIC L1".as_ptr(),
        control: ptr::null(),
        source: c"Int Mic".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"DMIC R1".as_ptr(),
        control: ptr::null(),
        source: c"Int Mic".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Headphone".as_ptr(),
        control: ptr::null(),
        source: c"HPOL".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Headphone".as_ptr(),
        control: ptr::null(),
        source: c"HPOR".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"IN1P".as_ptr(),
        control: ptr::null(),
        source: c"Headset Mic".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"IN1N".as_ptr(),
        control: ptr::null(),
        source: c"Headset Mic".as_ptr(),
    },
];

static mt8173_rt5650_controls: [snd_kcontrol_new; 4] = [
    soc_dapm_pin_switch!("Ext Spk"),
    soc_dapm_pin_switch!("Int Mic"),
    soc_dapm_pin_switch!("Headphone"),
    soc_dapm_pin_switch!("Headset Mic"),
];

static mut mt8173_rt5650_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headphone".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe extern "C" fn mt8173_rt5650_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let mclk_clock: c_uint;
    let mut codec_dai: *mut snd_soc_dai;
    let mut i: c_int = 0;
    let mut ret: c_int;

    match mt8173_rt5650_priv.pll_from {
        mt8173_rt5650_mclk::MT8173_RT5650_MCLK_EXTERNAL => {
            /* mclk = 12.288M */
            mclk_clock = MCLK_FOR_CODECS;
        }
        mt8173_rt5650_mclk::MT8173_RT5650_MCLK_INTERNAL => {
            /* mclk = sampling rate*256 */
            mclk_clock = params_rate(params).wrapping_mul(256);
        }
    }

    for_each_rtd_codec_dais!(rtd, i, codec_dai, {
        /* pll from mclk */
        ret = snd_soc_dai_set_pll(
            codec_dai,
            0,
            0,
            mclk_clock,
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
    });
    0
}

static mt8173_rt5650_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(mt8173_rt5650_hw_params),
};

static mut mt8173_rt5650_jack: snd_soc_jack = snd_soc_jack { _private: [] };
static mut mt8173_rt5650_hdmi_jack: snd_soc_jack = snd_soc_jack { _private: [] };

unsafe extern "C" fn mt8173_rt5650_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*runtime).card;
    let component = (*snd_soc_rtd_to_codec(runtime, 0)).component;
    let codec_capture_dai = (*snd_soc_rtd_to_codec(runtime, 1)).name;
    let ret: c_int;

    rt5645_sel_asrc_clk_src(
        component,
        RT5645_DA_STEREO_FILTER,
        RT5645_CLK_SEL_I2S1_ASRC,
    );

    if strcmp(codec_capture_dai, c"rt5645-aif1".as_ptr()) == 0 {
        rt5645_sel_asrc_clk_src(
            component,
            RT5645_AD_STEREO_FILTER,
            RT5645_CLK_SEL_I2S1_ASRC,
        );
    } else if strcmp(codec_capture_dai, c"rt5645-aif2".as_ptr()) == 0 {
        rt5645_sel_asrc_clk_src(
            component,
            RT5645_AD_STEREO_FILTER,
            RT5645_CLK_SEL_I2S2_ASRC,
        );
    } else {
        dev_warn(
            (*card).dev,
            c"Only one dai codec found in DTS, enabled rt5645 AD filter\n".as_ptr(),
        );
        rt5645_sel_asrc_clk_src(
            component,
            RT5645_AD_STEREO_FILTER,
            RT5645_CLK_SEL_I2S1_ASRC,
        );
    }

    /* enable jack detection */
    ret = snd_soc_card_jack_new_pins(
        card,
        c"Headset Jack".as_ptr(),
        SND_JACK_HEADPHONE
            | SND_JACK_MICROPHONE
            | SND_JACK_BTN_0
            | SND_JACK_BTN_1
            | SND_JACK_BTN_2
            | SND_JACK_BTN_3,
        &raw mut mt8173_rt5650_jack,
        mt8173_rt5650_jack_pins.as_mut_ptr(),
        mt8173_rt5650_jack_pins.len() as c_uint,
    );
    if ret != 0 {
        dev_err((*card).dev, c"Can't new Headset Jack %d\n".as_ptr(), ret);
        return ret;
    }

    rt5645_set_jack_detect(
        component,
        &raw mut mt8173_rt5650_jack,
        &raw mut mt8173_rt5650_jack,
        &raw mut mt8173_rt5650_jack,
    )
}

unsafe extern "C" fn mt8173_rt5650_hdmi_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let mut ret: c_int;

    ret = snd_soc_card_jack_new(
        (*rtd).card,
        c"HDMI Jack".as_ptr(),
        SND_JACK_AVOUT,
        &raw mut mt8173_rt5650_hdmi_jack,
    );
    if ret != 0 {
        return ret;
    }

    snd_soc_component_set_jack(
        (*snd_soc_rtd_to_codec(rtd, 0)).component,
        &raw mut mt8173_rt5650_hdmi_jack,
        ptr::null_mut(),
    )
}

const DAI_LINK_PLAYBACK: usize = 0;
const DAI_LINK_CAPTURE: usize = 1;
const DAI_LINK_HDMI: usize = 2;
const DAI_LINK_CODEC_I2S: usize = 3;
const DAI_LINK_HDMI_I2S: usize = 4;

static mut playback_cpus: [snd_soc_dai_link_component; 1] =
    dailink_comp_array!(comp_cpu!("DL1"));
static mut playback_codecs: [snd_soc_dai_link_component; 1] =
    dailink_comp_array!(comp_dummy!());
static mut playback_platforms: [snd_soc_dai_link_component; 1] =
    dailink_comp_array!(comp_empty!());

static mut capture_cpus: [snd_soc_dai_link_component; 1] =
    dailink_comp_array!(comp_cpu!("VUL"));
static mut capture_codecs: [snd_soc_dai_link_component; 1] =
    dailink_comp_array!(comp_dummy!());
static mut capture_platforms: [snd_soc_dai_link_component; 1] =
    dailink_comp_array!(comp_empty!());

static mut hdmi_pcm_cpus: [snd_soc_dai_link_component; 1] =
    dailink_comp_array!(comp_cpu!("HDMI"));
static mut hdmi_pcm_codecs: [snd_soc_dai_link_component; 1] =
    dailink_comp_array!(comp_dummy!());
static mut hdmi_pcm_platforms: [snd_soc_dai_link_component; 1] =
    dailink_comp_array!(comp_empty!());

static mut codec_cpus: [snd_soc_dai_link_component; 1] =
    dailink_comp_array!(comp_cpu!("I2S"));
static mut codec_codecs: [snd_soc_dai_link_component; 2] = dailink_comp_array!(
    comp_codec!(ptr::null_mut(), "rt5645-aif1"), /* Playback */
    comp_codec!(ptr::null_mut(), "rt5645-aif1")  /* Capture */
);
static mut codec_platforms: [snd_soc_dai_link_component; 1] =
    dailink_comp_array!(comp_empty!());

static mut hdmi_be_cpus: [snd_soc_dai_link_component; 1] =
    dailink_comp_array!(comp_cpu!("HDMIO"));
static mut hdmi_be_codecs: [snd_soc_dai_link_component; 1] =
    dailink_comp_array!(comp_codec!(ptr::null_mut(), "i2s-hifi"));
static mut hdmi_be_platforms: [snd_soc_dai_link_component; 1] =
    dailink_comp_array!(comp_empty!());

/* Digital audio interface glue - connects codec <---> CPU */
static mut mt8173_rt5650_dais: [snd_soc_dai_link; 5] = [
    /* Front End DAI links */
    snd_soc_dai_link {
        name: c"rt5650 Playback".as_ptr(),
        stream_name: c"rt5650 Playback".as_ptr(),
        trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
        dynamic: 1,
        playback_only: 1,
        capture_only: 0,
        no_pcm: 0,
        init: None,
        dai_fmt: 0,
        ops: ptr::null(),
        ignore_pmdown_time: 0,
        cpus: playback_cpus.as_mut_ptr(),
        num_cpus: playback_cpus.len() as c_uint,
        codecs: playback_codecs.as_mut_ptr(),
        num_codecs: playback_codecs.len() as c_uint,
        platforms: playback_platforms.as_mut_ptr(),
        num_platforms: playback_platforms.len() as c_uint,
    },
    snd_soc_dai_link {
        name: c"rt5650 Capture".as_ptr(),
        stream_name: c"rt5650 Capture".as_ptr(),
        trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
        dynamic: 1,
        playback_only: 0,
        capture_only: 1,
        no_pcm: 0,
        init: None,
        dai_fmt: 0,
        ops: ptr::null(),
        ignore_pmdown_time: 0,
        cpus: capture_cpus.as_mut_ptr(),
        num_cpus: capture_cpus.len() as c_uint,
        codecs: capture_codecs.as_mut_ptr(),
        num_codecs: capture_codecs.len() as c_uint,
        platforms: capture_platforms.as_mut_ptr(),
        num_platforms: capture_platforms.len() as c_uint,
    },
    snd_soc_dai_link {
        name: c"HDMI".as_ptr(),
        stream_name: c"HDMI PCM".as_ptr(),
        trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
        dynamic: 1,
        playback_only: 1,
        capture_only: 0,
        no_pcm: 0,
        init: None,
        dai_fmt: 0,
        ops: ptr::null(),
        ignore_pmdown_time: 0,
        cpus: hdmi_pcm_cpus.as_mut_ptr(),
        num_cpus: hdmi_pcm_cpus.len() as c_uint,
        codecs: hdmi_pcm_codecs.as_mut_ptr(),
        num_codecs: hdmi_pcm_codecs.len() as c_uint,
        platforms: hdmi_pcm_platforms.as_mut_ptr(),
        num_platforms: hdmi_pcm_platforms.len() as c_uint,
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
        init: Some(mt8173_rt5650_init),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        ops: &mt8173_rt5650_ops,
        ignore_pmdown_time: 1,
        cpus: codec_cpus.as_mut_ptr(),
        num_cpus: codec_cpus.len() as c_uint,
        codecs: codec_codecs.as_mut_ptr(),
        num_codecs: codec_codecs.len() as c_uint,
        platforms: codec_platforms.as_mut_ptr(),
        num_platforms: codec_platforms.len() as c_uint,
    },
    snd_soc_dai_link {
        name: c"HDMI BE".as_ptr(),
        stream_name: ptr::null(),
        trigger: [0, 0],
        dynamic: 0,
        playback_only: 1,
        capture_only: 0,
        no_pcm: 1,
        init: Some(mt8173_rt5650_hdmi_init),
        dai_fmt: 0,
        ops: ptr::null(),
        ignore_pmdown_time: 0,
        cpus: hdmi_be_cpus.as_mut_ptr(),
        num_cpus: hdmi_be_cpus.len() as c_uint,
        codecs: hdmi_be_codecs.as_mut_ptr(),
        num_codecs: hdmi_be_codecs.len() as c_uint,
        platforms: hdmi_be_platforms.as_mut_ptr(),
        num_platforms: hdmi_be_platforms.len() as c_uint,
    },
];

static mut mt8173_rt5650_card: snd_soc_card = snd_soc_card {
    name: c"mtk-rt5650".as_ptr(),
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { mt8173_rt5650_dais.as_mut_ptr() },
    num_links: 5,
    controls: mt8173_rt5650_controls.as_ptr(),
    num_controls: mt8173_rt5650_controls.len() as c_uint,
    dapm_widgets: mt8173_rt5650_widgets.as_ptr(),
    num_dapm_widgets: mt8173_rt5650_widgets.len() as c_uint,
    dapm_routes: mt8173_rt5650_routes.as_ptr(),
    num_dapm_routes: mt8173_rt5650_routes.len() as c_uint,
    dev: ptr::null_mut(),
};

unsafe extern "C" fn mt8173_rt5650_dev_probe(pdev: *mut platform_device) -> c_int {
    let card = &raw mut mt8173_rt5650_card;
    let platform_node: *mut device_node;
    let mut np: *mut device_node;
    let mut codec_capture_dai: *const c_char = ptr::null();
    let mut dai_link: *mut snd_soc_dai_link;
    let mut i: c_int = 0;
    let mut ret: c_int;

    platform_node = of_parse_phandle((*pdev).dev.of_node, c"mediatek,platform".as_ptr(), 0);
    if platform_node.is_null() {
        dev_err(
            &raw mut (*pdev).dev,
            c"Property 'platform' missing or invalid\n".as_ptr(),
        );
        return -EINVAL;
    }

    for_each_card_prelinks!(card, i, dai_link, {
        if !(*(*dai_link).platforms).name.is_null() {
            continue;
        }
        (*(*dai_link).platforms).of_node = platform_node;
    });

    mt8173_rt5650_dais[DAI_LINK_CODEC_I2S].codecs.add(0).as_mut().unwrap().of_node =
        of_parse_phandle((*pdev).dev.of_node, c"mediatek,audio-codec".as_ptr(), 0);
    if (*mt8173_rt5650_dais[DAI_LINK_CODEC_I2S].codecs.add(0))
        .of_node
        .is_null()
    {
        dev_err(
            &raw mut (*pdev).dev,
            c"Property 'audio-codec' missing or invalid\n".as_ptr(),
        );
        ret = -EINVAL;
        goto_put_platform_node!(ret, platform_node);
    }
    (*mt8173_rt5650_dais[DAI_LINK_CODEC_I2S].codecs.add(1)).of_node =
        (*mt8173_rt5650_dais[DAI_LINK_CODEC_I2S].codecs.add(0)).of_node;

    np = of_get_child_by_name((*pdev).dev.of_node, c"codec-capture".as_ptr());
    if !np.is_null() {
        ret = snd_soc_of_get_dai_name(np, &mut codec_capture_dai, 0);
        of_node_put(np);
        if ret < 0 {
            dev_err(
                &raw mut (*pdev).dev,
                c"%s codec_capture_dai name fail %d\n".as_ptr(),
                c"mt8173_rt5650_dev_probe".as_ptr(),
                ret,
            );
            goto_put_platform_node!(ret, platform_node);
        }
        (*mt8173_rt5650_dais[DAI_LINK_CODEC_I2S].codecs.add(1)).dai_name =
            codec_capture_dai;
    }

    if device_property_present(&raw mut (*pdev).dev, c"mediatek,mclk".as_ptr()) {
        ret = device_property_read_u32(
            &raw mut (*pdev).dev,
            c"mediatek,mclk".as_ptr(),
            &raw mut mt8173_rt5650_priv.pll_from,
        );
        if ret != 0 {
            dev_err(
                &raw mut (*pdev).dev,
                c"%s device_property_read_u32() fail %d\n".as_ptr(),
                c"mt8173_rt5650_dev_probe".as_ptr(),
                ret,
            );
        }
    }

    (*mt8173_rt5650_dais[DAI_LINK_HDMI_I2S].codecs).of_node =
        of_parse_phandle((*pdev).dev.of_node, c"mediatek,audio-codec".as_ptr(), 1);
    if (*mt8173_rt5650_dais[DAI_LINK_HDMI_I2S].codecs)
        .of_node
        .is_null()
    {
        dev_err(
            &raw mut (*pdev).dev,
            c"Property 'audio-codec' missing or invalid\n".as_ptr(),
        );
        ret = -EINVAL;
        goto_put_platform_node!(ret, platform_node);
    }
    (*card).dev = &raw mut (*pdev).dev;

    ret = devm_snd_soc_register_card(&raw mut (*pdev).dev, card);

    of_node_put(platform_node);
    ret
}

static mt8173_rt5650_dt_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"mediatek,mt8173-rt5650".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
module_device_table!(of, mt8173_rt5650_dt_match);

static mut mt8173_rt5650_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"mtk-rt5650".as_ptr(),
        of_match_table: mt8173_rt5650_dt_match.as_ptr(),
        pm: unsafe { &snd_soc_pm_ops as *const c_void },
    },
    probe: Some(mt8173_rt5650_dev_probe),
};

module_platform_driver!(mt8173_rt5650_driver);

/* Module information */
module_description!("MT8173 RT5650 SoC machine driver");
module_author!("Koro Chen <koro.chen@mediatek.com>");
module_license!("GPL v2");
module_alias!("platform:mtk-rt5650");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
