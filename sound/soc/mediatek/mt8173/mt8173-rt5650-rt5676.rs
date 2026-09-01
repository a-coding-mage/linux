// SPDX-License-Identifier: GPL-2.0
/*
 * mt8173-rt5650-rt5676.c  --  MT8173 machine driver with RT5650/5676 codecs
 *
 * Copyright (c) 2015 MediaTek Inc.
 * Author: Koro Chen <koro.chen@mediatek.com>
 */

use core::ffi::{c_char, c_int, c_uint};
use core::ptr;

const MCLK_FOR_CODECS: c_uint = 12288000;

const SND_JACK_HEADPHONE: c_uint = 0x0001;
const SND_JACK_MICROPHONE: c_uint = 0x0002;
const SND_JACK_BTN_0: c_uint = 0x4000;
const SND_JACK_BTN_1: c_uint = 0x2000;
const SND_JACK_BTN_2: c_uint = 0x1000;
const SND_JACK_BTN_3: c_uint = 0x0800;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DPCM_TRIGGER_POST: c_int = 1;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const EINVAL: c_int = 22;

const RT5645_DA_STEREO_FILTER: c_uint = 0x1;
const RT5645_AD_STEREO_FILTER: c_uint = 0x2;
const RT5645_CLK_SEL_I2S1_ASRC: c_uint = 0;
const RT5677_DA_STEREO_FILTER: c_uint = 0x1;
const RT5677_AD_STEREO1_FILTER: c_uint = 0x2;
const RT5677_AD_STEREO2_FILTER: c_uint = 0x4;
const RT5677_I2S2_SOURCE: c_uint = 0x8;
const RT5677_CLK_SEL_I2S1_ASRC: c_uint = 0;
const RT5677_CLK_SEL_I2S2_ASRC: c_uint = 1;

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
    card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    name: *const c_char,
    dai_name: *const c_char,
    of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    name: *const c_char,
    stream_name: *const c_char,
    trigger: [c_int; 2],
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
pub struct snd_soc_card {
    name: *const c_char,
    owner: *mut module,
    dai_link: *mut snd_soc_dai_link,
    num_links: c_int,
    codec_conf: *mut snd_soc_codec_conf,
    num_configs: c_int,
    controls: *const snd_kcontrol_new,
    num_controls: c_int,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_int,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_int,
    dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_ops {
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pin: *const c_char,
    mask: c_uint,
}

#[repr(C)]
pub struct snd_soc_codec_conf {
    dlc: snd_soc_dai_link_component,
    name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    name: *const c_char,
    kind: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    name: *const c_char,
}

#[repr(C)]
pub struct device {
    of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
pub struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
    pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut THIS_MODULE: module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_dai_set_pll(
        codec_dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn snd_soc_dai_set_sysclk(
        codec_dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_rtd_to_codec(runtime: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai_link_component;
    fn rt5645_sel_asrc_clk_src(component: *mut snd_soc_component, filter_mask: c_uint, clk_src: c_uint);
    fn rt5677_sel_asrc_clk_src(component: *mut snd_soc_component, filter_mask: c_uint, clk_src: c_uint);
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
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn of_parse_phandle(np: *mut device_node, phandle_name: *const c_char, index: c_int) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

static MT8173_RT5650_RT5676_WIDGETS: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget { name: c"Speaker".as_ptr(), kind: 0 },      /* SND_SOC_DAPM_SPK("Speaker", NULL) */
    snd_soc_dapm_widget { name: c"Int Mic".as_ptr(), kind: 0 },      /* SND_SOC_DAPM_MIC("Int Mic", NULL) */
    snd_soc_dapm_widget { name: c"Headphone".as_ptr(), kind: 0 },    /* SND_SOC_DAPM_HP("Headphone", NULL) */
    snd_soc_dapm_widget { name: c"Headset Mic".as_ptr(), kind: 0 },  /* SND_SOC_DAPM_MIC("Headset Mic", NULL) */
];

static MT8173_RT5650_RT5676_ROUTES: [snd_soc_dapm_route; 11] = [
    snd_soc_dapm_route { sink: c"Speaker".as_ptr(), control: ptr::null(), source: c"SPOL".as_ptr() },
    snd_soc_dapm_route { sink: c"Speaker".as_ptr(), control: ptr::null(), source: c"SPOR".as_ptr() },
    snd_soc_dapm_route { sink: c"Speaker".as_ptr(), control: ptr::null(), source: c"Sub AIF2TX".as_ptr() }, /* IF2 ADC to 5650  */
    snd_soc_dapm_route { sink: c"Sub DMIC L1".as_ptr(), control: ptr::null(), source: c"Int Mic".as_ptr() }, /* DMIC from 5676 */
    snd_soc_dapm_route { sink: c"Sub DMIC R1".as_ptr(), control: ptr::null(), source: c"Int Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphone".as_ptr(), control: ptr::null(), source: c"HPOL".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphone".as_ptr(), control: ptr::null(), source: c"HPOR".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphone".as_ptr(), control: ptr::null(), source: c"Sub AIF2TX".as_ptr() }, /* IF2 ADC to 5650  */
    snd_soc_dapm_route { sink: c"IN1P".as_ptr(), control: ptr::null(), source: c"Headset Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"IN1N".as_ptr(), control: ptr::null(), source: c"Headset Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"Sub AIF2RX".as_ptr(), control: ptr::null(), source: c"Headset Mic".as_ptr() }, /* IF2 DAC from 5650  */
];

static MT8173_RT5650_RT5676_CONTROLS: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new { name: c"Speaker".as_ptr() },     /* SOC_DAPM_PIN_SWITCH("Speaker") */
    snd_kcontrol_new { name: c"Int Mic".as_ptr() },     /* SOC_DAPM_PIN_SWITCH("Int Mic") */
    snd_kcontrol_new { name: c"Headphone".as_ptr() },   /* SOC_DAPM_PIN_SWITCH("Headphone") */
    snd_kcontrol_new { name: c"Headset Mic".as_ptr() }, /* SOC_DAPM_PIN_SWITCH("Headset Mic") */
];

static mut MT8173_RT5650_RT5676_JACK_PINS: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headphone".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe extern "C" fn mt8173_rt5650_rt5676_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = unsafe { snd_soc_substream_to_rtd(substream) };
    let mut ret: c_int;

    /* for_each_rtd_codec_dais(rtd, i, codec_dai) */
    unsafe {
        let dai_link = (*rtd).card.as_ref().map_or(ptr::null_mut(), |card| card.dai_link);
        if !dai_link.is_null() {
            let mut i: c_uint = 0;
            while i < (*dai_link).num_codecs {
                let codec_dai = (*dai_link).codecs.add(i as usize) as *mut snd_soc_dai;

                /* pll from mclk 12.288M */
                ret = snd_soc_dai_set_pll(codec_dai, 0, 0, MCLK_FOR_CODECS, params_rate(params) * 512);
                if ret != 0 {
                    return ret;
                }

                /* sysclk from pll */
                ret = snd_soc_dai_set_sysclk(codec_dai, 1, params_rate(params) * 512, SND_SOC_CLOCK_IN);
                if ret != 0 {
                    return ret;
                }

                i += 1;
            }
        }
    }
    0
}

static MT8173_RT5650_RT5676_OPS: snd_soc_ops = snd_soc_ops {
    hw_params: Some(mt8173_rt5650_rt5676_hw_params),
};

static mut MT8173_RT5650_RT5676_JACK: snd_soc_jack = snd_soc_jack { _private: [] };

unsafe extern "C" fn mt8173_rt5650_rt5676_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let card = unsafe { (*runtime).card };
    let component = unsafe { (*(snd_soc_rtd_to_codec(runtime, 0))).of_node as *mut snd_soc_component };
    let component_sub = unsafe { (*(snd_soc_rtd_to_codec(runtime, 1))).of_node as *mut snd_soc_component };
    let ret: c_int;

    unsafe {
        rt5645_sel_asrc_clk_src(
            component,
            RT5645_DA_STEREO_FILTER | RT5645_AD_STEREO_FILTER,
            RT5645_CLK_SEL_I2S1_ASRC,
        );
        rt5677_sel_asrc_clk_src(
            component_sub,
            RT5677_DA_STEREO_FILTER | RT5677_AD_STEREO1_FILTER,
            RT5677_CLK_SEL_I2S1_ASRC,
        );
        rt5677_sel_asrc_clk_src(
            component_sub,
            RT5677_AD_STEREO2_FILTER | RT5677_I2S2_SOURCE,
            RT5677_CLK_SEL_I2S2_ASRC,
        );

        /* enable jack detection */
        ret = snd_soc_card_jack_new_pins(
            card,
            c"Headset Jack".as_ptr(),
            SND_JACK_HEADPHONE | SND_JACK_MICROPHONE | SND_JACK_BTN_0 | SND_JACK_BTN_1 |
                SND_JACK_BTN_2 | SND_JACK_BTN_3,
            &raw mut MT8173_RT5650_RT5676_JACK,
            MT8173_RT5650_RT5676_JACK_PINS.as_mut_ptr(),
            MT8173_RT5650_RT5676_JACK_PINS.len() as c_uint,
        );
        if ret != 0 {
            dev_err((*card).dev, c"Can't new Headset Jack %d\n".as_ptr(), ret);
            return ret;
        }

        rt5645_set_jack_detect(
            component,
            &raw mut MT8173_RT5650_RT5676_JACK,
            &raw mut MT8173_RT5650_RT5676_JACK,
            &raw mut MT8173_RT5650_RT5676_JACK,
        )
    }
}

const DAI_LINK_PLAYBACK: usize = 0;
const DAI_LINK_CAPTURE: usize = 1;
const DAI_LINK_HDMI: usize = 2;
const DAI_LINK_CODEC_I2S: usize = 3;
const DAI_LINK_HDMI_I2S: usize = 4;
const DAI_LINK_INTERCODEC: usize = 5;

static mut PLAYBACK_CPUS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null(), dai_name: c"DL1".as_ptr(), of_node: ptr::null_mut() }];
static mut PLAYBACK_CODECS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null(), dai_name: ptr::null(), of_node: ptr::null_mut() }];
static mut PLAYBACK_PLATFORMS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null(), dai_name: ptr::null(), of_node: ptr::null_mut() }];
static mut CAPTURE_CPUS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null(), dai_name: c"VUL".as_ptr(), of_node: ptr::null_mut() }];
static mut CAPTURE_CODECS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null(), dai_name: ptr::null(), of_node: ptr::null_mut() }];
static mut CAPTURE_PLATFORMS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null(), dai_name: ptr::null(), of_node: ptr::null_mut() }];
static mut HDMI_PCM_CPUS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null(), dai_name: c"HDMI".as_ptr(), of_node: ptr::null_mut() }];
static mut HDMI_PCM_CODECS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null(), dai_name: ptr::null(), of_node: ptr::null_mut() }];
static mut HDMI_PCM_PLATFORMS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null(), dai_name: ptr::null(), of_node: ptr::null_mut() }];
static mut CODEC_CPUS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null(), dai_name: c"I2S".as_ptr(), of_node: ptr::null_mut() }];
static mut CODEC_CODECS: [snd_soc_dai_link_component; 2] = [
    snd_soc_dai_link_component { name: ptr::null(), dai_name: c"rt5645-aif1".as_ptr(), of_node: ptr::null_mut() },
    snd_soc_dai_link_component { name: ptr::null(), dai_name: c"rt5677-aif1".as_ptr(), of_node: ptr::null_mut() },
];
static mut CODEC_PLATFORMS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null(), dai_name: ptr::null(), of_node: ptr::null_mut() }];
static mut HDMI_BE_CPUS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null(), dai_name: c"HDMIO".as_ptr(), of_node: ptr::null_mut() }];
static mut HDMI_BE_CODECS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null(), dai_name: c"i2s-hifi".as_ptr(), of_node: ptr::null_mut() }];
static mut HDMI_BE_PLATFORMS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null(), dai_name: ptr::null(), of_node: ptr::null_mut() }];
static mut INTERCODEC_CPUS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null(), dai_name: ptr::null(), of_node: ptr::null_mut() }];
static mut INTERCODEC_CODECS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null(), dai_name: c"rt5677-aif2".as_ptr(), of_node: ptr::null_mut() }];
static mut INTERCODEC_PLATFORMS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null(), dai_name: ptr::null(), of_node: ptr::null_mut() }];

/* Digital audio interface glue - connects codec <---> CPU */
static mut MT8173_RT5650_RT5676_DAIS: [snd_soc_dai_link; 6] = [
    /* Front End DAI links */
    snd_soc_dai_link {
        name: c"rt5650_rt5676 Playback".as_ptr(),
        stream_name: c"rt5650_rt5676 Playback".as_ptr(),
        trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
        dynamic: 1,
        playback_only: 1,
        capture_only: 0,
        no_pcm: 0,
        init: None,
        dai_fmt: 0,
        ops: ptr::null(),
        ignore_pmdown_time: 0,
        cpus: unsafe { PLAYBACK_CPUS.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { PLAYBACK_CODECS.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { PLAYBACK_PLATFORMS.as_mut_ptr() },
        num_platforms: 1,
    },
    snd_soc_dai_link {
        name: c"rt5650_rt5676 Capture".as_ptr(),
        stream_name: c"rt5650_rt5676 Capture".as_ptr(),
        trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
        dynamic: 1,
        playback_only: 0,
        capture_only: 1,
        no_pcm: 0,
        init: None,
        dai_fmt: 0,
        ops: ptr::null(),
        ignore_pmdown_time: 0,
        cpus: unsafe { CAPTURE_CPUS.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { CAPTURE_CODECS.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { CAPTURE_PLATFORMS.as_mut_ptr() },
        num_platforms: 1,
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
        cpus: unsafe { HDMI_PCM_CPUS.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { HDMI_PCM_CODECS.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { HDMI_PCM_PLATFORMS.as_mut_ptr() },
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
        init: Some(mt8173_rt5650_rt5676_init),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        ops: &MT8173_RT5650_RT5676_OPS,
        ignore_pmdown_time: 1,
        cpus: unsafe { CODEC_CPUS.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { CODEC_CODECS.as_mut_ptr() },
        num_codecs: 2,
        platforms: unsafe { CODEC_PLATFORMS.as_mut_ptr() },
        num_platforms: 1,
    },
    snd_soc_dai_link {
        name: c"HDMI BE".as_ptr(),
        stream_name: ptr::null(),
        trigger: [0, 0],
        dynamic: 0,
        playback_only: 1,
        capture_only: 0,
        no_pcm: 1,
        init: None,
        dai_fmt: 0,
        ops: ptr::null(),
        ignore_pmdown_time: 0,
        cpus: unsafe { HDMI_BE_CPUS.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { HDMI_BE_CODECS.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { HDMI_BE_PLATFORMS.as_mut_ptr() },
        num_platforms: 1,
    },
    /* rt5676 <-> rt5650 intercodec link: Sets rt5676 I2S2 as master */
    snd_soc_dai_link {
        name: c"rt5650_rt5676 intercodec".as_ptr(),
        stream_name: c"rt5650_rt5676 intercodec".as_ptr(),
        trigger: [0, 0],
        dynamic: 0,
        playback_only: 0,
        capture_only: 0,
        no_pcm: 1,
        init: None,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        ops: ptr::null(),
        ignore_pmdown_time: 0,
        cpus: unsafe { INTERCODEC_CPUS.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { INTERCODEC_CODECS.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { INTERCODEC_PLATFORMS.as_mut_ptr() },
        num_platforms: 1,
    },
];

static mut MT8173_RT5650_RT5676_CODEC_CONF: [snd_soc_codec_conf; 1] = [
    snd_soc_codec_conf {
        dlc: snd_soc_dai_link_component { name: ptr::null(), dai_name: ptr::null(), of_node: ptr::null_mut() },
        name_prefix: c"Sub".as_ptr(),
    },
];

static mut MT8173_RT5650_RT5676_CARD: snd_soc_card = snd_soc_card {
    name: c"mtk-rt5650-rt5676".as_ptr(),
    owner: unsafe { &raw mut THIS_MODULE },
    dai_link: unsafe { MT8173_RT5650_RT5676_DAIS.as_mut_ptr() },
    num_links: 6,
    codec_conf: unsafe { MT8173_RT5650_RT5676_CODEC_CONF.as_mut_ptr() },
    num_configs: 1,
    controls: MT8173_RT5650_RT5676_CONTROLS.as_ptr(),
    num_controls: 4,
    dapm_widgets: MT8173_RT5650_RT5676_WIDGETS.as_ptr(),
    num_dapm_widgets: 4,
    dapm_routes: MT8173_RT5650_RT5676_ROUTES.as_ptr(),
    num_dapm_routes: 11,
    dev: ptr::null_mut(),
};

unsafe extern "C" fn mt8173_rt5650_rt5676_dev_probe(pdev: *mut platform_device) -> c_int {
    let card = unsafe { &raw mut MT8173_RT5650_RT5676_CARD };
    let platform_node: *mut device_node;
    let mut ret: c_int;

    unsafe {
        platform_node = of_parse_phandle((*pdev).dev.of_node, c"mediatek,platform".as_ptr(), 0);
        if platform_node.is_null() {
            dev_err(&raw mut (*pdev).dev, c"Property 'platform' missing or invalid\n".as_ptr());
            return -EINVAL;
        }

        /* for_each_card_prelinks(card, i, dai_link) */
        let mut i: c_int = 0;
        while i < (*card).num_links {
            let dai_link = (*card).dai_link.add(i as usize);
            if !(*(*dai_link).platforms).name.is_null() {
                i += 1;
                continue;
            }
            (*(*dai_link).platforms).of_node = platform_node;
            i += 1;
        }

        (*MT8173_RT5650_RT5676_DAIS[DAI_LINK_CODEC_I2S].codecs.add(0)).of_node =
            of_parse_phandle((*pdev).dev.of_node, c"mediatek,audio-codec".as_ptr(), 0);
        if (*MT8173_RT5650_RT5676_DAIS[DAI_LINK_CODEC_I2S].codecs.add(0)).of_node.is_null() {
            dev_err(&raw mut (*pdev).dev, c"Property 'audio-codec' missing or invalid\n".as_ptr());
            ret = -EINVAL;
            of_node_put(platform_node);
            return ret;
        }
        (*MT8173_RT5650_RT5676_DAIS[DAI_LINK_CODEC_I2S].codecs.add(1)).of_node =
            of_parse_phandle((*pdev).dev.of_node, c"mediatek,audio-codec".as_ptr(), 1);
        if (*MT8173_RT5650_RT5676_DAIS[DAI_LINK_CODEC_I2S].codecs.add(1)).of_node.is_null() {
            dev_err(&raw mut (*pdev).dev, c"Property 'audio-codec' missing or invalid\n".as_ptr());
            ret = -EINVAL;
            of_node_put(platform_node);
            return ret;
        }
        MT8173_RT5650_RT5676_CODEC_CONF[0].dlc.of_node =
            (*MT8173_RT5650_RT5676_DAIS[DAI_LINK_CODEC_I2S].codecs.add(1)).of_node;

        (*MT8173_RT5650_RT5676_DAIS[DAI_LINK_INTERCODEC].codecs).of_node =
            (*MT8173_RT5650_RT5676_DAIS[DAI_LINK_CODEC_I2S].codecs.add(1)).of_node;

        (*MT8173_RT5650_RT5676_DAIS[DAI_LINK_HDMI_I2S].codecs).of_node =
            of_parse_phandle((*pdev).dev.of_node, c"mediatek,audio-codec".as_ptr(), 2);
        if (*MT8173_RT5650_RT5676_DAIS[DAI_LINK_HDMI_I2S].codecs).of_node.is_null() {
            dev_err(&raw mut (*pdev).dev, c"Property 'audio-codec' missing or invalid\n".as_ptr());
            ret = -EINVAL;
            of_node_put(platform_node);
            return ret;
        }

        (*card).dev = &raw mut (*pdev).dev;

        ret = devm_snd_soc_register_card(&raw mut (*pdev).dev, card);

        of_node_put(platform_node);
        ret
    }
}

static MT8173_RT5650_RT5676_DT_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: c"mediatek,mt8173-rt5650-rt5676".as_ptr() },
    of_device_id { compatible: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, mt8173_rt5650_rt5676_dt_match); */

static mut MT8173_RT5650_RT5676_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: c"mtk-rt5650-rt5676".as_ptr(),
        of_match_table: MT8173_RT5650_RT5676_DT_MATCH.as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
    },
    probe: Some(mt8173_rt5650_rt5676_dev_probe),
};

/* module_platform_driver(mt8173_rt5650_rt5676_driver); */

/* Module information */
const MODULE_DESCRIPTION: &str = "MT8173 RT5650 and RT5676 SoC machine driver";
const MODULE_AUTHOR: &str = "Koro Chen <koro.chen@mediatek.com>";
const MODULE_LICENSE: &str = "GPL v2";
const MODULE_ALIAS: &str = "platform:mtk-rt5650-rt5676";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
