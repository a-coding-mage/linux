// SPDX-License-Identifier: GPL-2.0-only
/*
 * Rockchip machine ASoC driver for boards using a MAX90809 CODEC.
 *
 * Copyright (c) 2014, ROCKCHIP CORPORATION.  All rights reserved.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

const DRV_NAME: &[u8] = b"rockchip-snd-max98090\0";

const EINVAL: c_int = 22;
const SND_JACK_HEADPHONE: c_int = 0;
const SND_JACK_MICROPHONE: c_int = 0;
const SND_JACK_LINEOUT: c_int = 0;
const SND_JACK_HEADSET: c_int = 0;
const SND_JACK_BTN_0: c_int = 0;
const SND_JACK_BTN_1: c_int = 0;
const SND_JACK_BTN_2: c_int = 0;
const SND_JACK_BTN_3: c_int = 0;
const SND_SOC_CLOCK_OUT: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SNDRV_PCM_HW_PARAM_PERIOD_SIZE: c_int = 0;
const SND_SOC_DAIFMT_I2S: u32 = 0;
const SND_SOC_DAIFMT_NB_NF: u32 = 0;
const SND_SOC_DAIFMT_CBC_CFC: u32 = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai {
    dev: *mut device,
    component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    name: *const c_char,
    stream_name: *const c_char,
    init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    ops: *const snd_soc_ops,
    dai_fmt: u32,
    cpus: *mut snd_soc_dai_link_component,
    num_cpus: c_int,
    codecs: *mut snd_soc_dai_link_component,
    num_codecs: c_int,
    platforms: *mut snd_soc_dai_link_component,
    num_platforms: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
    dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct snd_soc_jack {
    card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pin: *const c_char,
    mask: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_ops {
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_aux_dev {
    dlc: snd_soc_dai_link_component,
    init: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_card {
    name: *const c_char,
    owner: *mut module,
    dai_link: *mut snd_soc_dai_link,
    num_links: c_int,
    aux_dev: *mut snd_soc_aux_dev,
    num_aux_devs: c_int,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_int,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_int,
    controls: *const snd_kcontrol_new,
    num_controls: c_int,
    dev: *mut device,
}

#[repr(C)]
pub struct platform_device {
    dev: platform_device_dev,
}

#[repr(C)]
pub struct platform_device_dev {
    of_node: *mut device_node,
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
pub struct platform_driver_driver {
    name: *const c_char,
    pm: *const dev_pm_ops,
    of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: platform_driver_driver,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_force_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_jack_notifier_register(jack: *mut snd_soc_jack, nb: *mut notifier_block) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_int, dir: c_int) -> c_int;
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_pcm_hw_constraint_minmax(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        min: c_ulong,
        max: c_ulong,
    ) -> c_int;
    fn snd_soc_card_jack_new(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
    ) -> c_int;
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_int,
    ) -> c_int;
    fn ts3a227e_enable_jack_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack) -> c_int;
    fn of_parse_phandle(np: *mut device_node, phandle_name: *const c_char, index: c_int) -> *mut device_node;
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut platform_device_dev, card: *mut snd_soc_card) -> c_int;
}

static mut headset_jack: snd_soc_jack = snd_soc_jack { card: ptr::null_mut() };

/* Headset jack detection DAPM pins */
static mut headset_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headphone".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
];

/* RK_MAX98090_WIDGETS:
 * SND_SOC_DAPM_HP("Headphone", NULL),
 * SND_SOC_DAPM_MIC("Headset Mic", NULL),
 * SND_SOC_DAPM_MIC("Int Mic", NULL),
 * SND_SOC_DAPM_SPK("Speaker", NULL)
 */

/* RK_HDMI_WIDGETS:
 * SND_SOC_DAPM_LINE("HDMI", NULL)
 */

static rk_max98090_dapm_widgets: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

static rk_hdmi_dapm_widgets: [snd_soc_dapm_widget; 1] = [
    snd_soc_dapm_widget { _private: [] },
];

static rk_max98090_hdmi_dapm_widgets: [snd_soc_dapm_widget; 5] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

/* RK_MAX98090_AUDIO_MAP */
static rk_max98090_audio_map: [snd_soc_dapm_route; 7] = [
    snd_soc_dapm_route { sink: c"IN34".as_ptr(), control: ptr::null(), source: c"Headset Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"Headset Mic".as_ptr(), control: ptr::null(), source: c"MICBIAS".as_ptr() },
    snd_soc_dapm_route { sink: c"DMICL".as_ptr(), control: ptr::null(), source: c"Int Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphone".as_ptr(), control: ptr::null(), source: c"HPL".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphone".as_ptr(), control: ptr::null(), source: c"HPR".as_ptr() },
    snd_soc_dapm_route { sink: c"Speaker".as_ptr(), control: ptr::null(), source: c"SPKL".as_ptr() },
    snd_soc_dapm_route { sink: c"Speaker".as_ptr(), control: ptr::null(), source: c"SPKR".as_ptr() },
];

/* RK_HDMI_AUDIO_MAP */
static rk_hdmi_audio_map: [snd_soc_dapm_route; 1] = [
    snd_soc_dapm_route { sink: c"HDMI".as_ptr(), control: ptr::null(), source: c"TX".as_ptr() },
];

static rk_max98090_hdmi_audio_map: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route { sink: c"IN34".as_ptr(), control: ptr::null(), source: c"Headset Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"Headset Mic".as_ptr(), control: ptr::null(), source: c"MICBIAS".as_ptr() },
    snd_soc_dapm_route { sink: c"DMICL".as_ptr(), control: ptr::null(), source: c"Int Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphone".as_ptr(), control: ptr::null(), source: c"HPL".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphone".as_ptr(), control: ptr::null(), source: c"HPR".as_ptr() },
    snd_soc_dapm_route { sink: c"Speaker".as_ptr(), control: ptr::null(), source: c"SPKL".as_ptr() },
    snd_soc_dapm_route { sink: c"Speaker".as_ptr(), control: ptr::null(), source: c"SPKR".as_ptr() },
    snd_soc_dapm_route { sink: c"HDMI".as_ptr(), control: ptr::null(), source: c"TX".as_ptr() },
];

/* RK_MAX98090_CONTROLS:
 * SOC_DAPM_PIN_SWITCH("Headphone"),
 * SOC_DAPM_PIN_SWITCH("Headset Mic"),
 * SOC_DAPM_PIN_SWITCH("Int Mic"),
 * SOC_DAPM_PIN_SWITCH("Speaker")
 */

/* RK_HDMI_CONTROLS:
 * SOC_DAPM_PIN_SWITCH("HDMI")
 */

static rk_max98090_controls: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

static rk_hdmi_controls: [snd_kcontrol_new; 1] = [
    snd_kcontrol_new { _private: [] },
];

static rk_max98090_hdmi_controls: [snd_kcontrol_new; 5] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

unsafe extern "C" fn rk_jack_event(
    _nb: *mut notifier_block,
    event: c_ulong,
    data: *mut c_void,
) -> c_int {
    let jack = data as *mut snd_soc_jack;
    let dapm = snd_soc_card_to_dapm((*jack).card);

    if event & SND_JACK_MICROPHONE as c_ulong != 0 {
        snd_soc_dapm_force_enable_pin(dapm, c"MICBIAS".as_ptr());
        snd_soc_dapm_force_enable_pin(dapm, c"SHDN".as_ptr());
    } else {
        snd_soc_dapm_disable_pin(dapm, c"MICBIAS".as_ptr());
        snd_soc_dapm_disable_pin(dapm, c"SHDN".as_ptr());
    }

    snd_soc_dapm_sync(dapm);

    0
}

static mut rk_jack_nb: notifier_block = notifier_block {
    notifier_call: Some(rk_jack_event),
};

unsafe extern "C" fn rk_init(_runtime: *mut snd_soc_pcm_runtime) -> c_int {
    /*
     * The jack has already been created in the rk_98090_headset_init()
     * function.
     */
    snd_soc_jack_notifier_register(&raw mut headset_jack, &raw mut rk_jack_nb);

    0
}

unsafe extern "C" fn rk_aif1_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let mut ret: c_int = 0;
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mclk: c_int;

    match params_rate(params) {
        8000 | 16000 | 24000 | 32000 | 48000 | 64000 | 96000 => {
            mclk = 12288000;
        }
        11025 | 22050 | 44100 | 88200 => {
            mclk = 11289600;
        }
        _ => {
            return -EINVAL;
        }
    }

    ret = snd_soc_dai_set_sysclk(cpu_dai, 0, mclk, SND_SOC_CLOCK_OUT);
    if ret != 0 {
        dev_err((*cpu_dai).dev, c"Can't set cpu dai clock %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(codec_dai, 0, mclk, SND_SOC_CLOCK_IN);

    /* HDMI codec dai does not need to set sysclk. */
    if strcmp((*(*rtd).dai_link).name, c"HDMI".as_ptr()) == 0 {
        return 0;
    }

    if ret != 0 {
        dev_err((*codec_dai).dev, c"Can't set codec dai clock %d\n".as_ptr(), ret);
        return ret;
    }

    ret
}

unsafe extern "C" fn rk_aif1_startup(substream: *mut snd_pcm_substream) -> c_int {
    /*
     * Set period size to 240 because pl330 has issue
     * dealing with larger period in stress testing.
     */
    snd_pcm_hw_constraint_minmax(
        (*substream).runtime,
        SNDRV_PCM_HW_PARAM_PERIOD_SIZE,
        240,
        240,
    )
}

static rk_aif1_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(rk_aif1_hw_params),
    startup: Some(rk_aif1_startup),
};

/* SND_SOC_DAILINK_DEFS(analog,
 *      DAILINK_COMP_ARRAY(COMP_EMPTY()),
 *      DAILINK_COMP_ARRAY(COMP_CODEC(NULL, "HiFi")),
 *      DAILINK_COMP_ARRAY(COMP_EMPTY()));
 */
static mut analog_cpus: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: ptr::null_mut() }];
static mut analog_codecs: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: ptr::null_mut() }];
static mut analog_platforms: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: ptr::null_mut() }];

/* SND_SOC_DAILINK_DEFS(hdmi,
 *      DAILINK_COMP_ARRAY(COMP_EMPTY()),
 *      DAILINK_COMP_ARRAY(COMP_CODEC(NULL, "i2s-hifi")),
 *      DAILINK_COMP_ARRAY(COMP_EMPTY()));
 */
static mut hdmi_cpus: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: ptr::null_mut() }];
static mut hdmi_codecs: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: ptr::null_mut() }];
static mut hdmi_platforms: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: ptr::null_mut() }];

const DAILINK_MAX98090: usize = 0;
const DAILINK_HDMI: usize = 1;

static mut rk_hdmi_jack: snd_soc_jack = snd_soc_jack { card: ptr::null_mut() };

unsafe extern "C" fn rk_hdmi_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*runtime).card;
    let component = (*snd_soc_rtd_to_codec(runtime, 0)).component;
    let mut ret: c_int;

    /* enable jack detection */
    ret = snd_soc_card_jack_new(card, c"HDMI Jack".as_ptr(), SND_JACK_LINEOUT, &raw mut rk_hdmi_jack);
    if ret != 0 {
        dev_err((*card).dev, c"Can't new HDMI Jack %d\n".as_ptr(), ret);
        return ret;
    }

    snd_soc_component_set_jack(component, &raw mut rk_hdmi_jack, ptr::null_mut())
}

/* max98090 dai_link */
static mut rk_max98090_dailinks: [snd_soc_dai_link; 1] = [
    snd_soc_dai_link {
        name: c"max98090".as_ptr(),
        stream_name: c"Analog".as_ptr(),
        init: Some(rk_init),
        ops: &rk_aif1_ops,
        /* set max98090 as slave */
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        cpus: unsafe { analog_cpus.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { analog_codecs.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { analog_platforms.as_mut_ptr() },
        num_platforms: 1,
    },
];

/* HDMI codec dai_link */
static mut rk_hdmi_dailinks: [snd_soc_dai_link; 1] = [
    snd_soc_dai_link {
        name: c"HDMI".as_ptr(),
        stream_name: c"HDMI".as_ptr(),
        init: Some(rk_hdmi_init),
        ops: &rk_aif1_ops,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        cpus: unsafe { hdmi_cpus.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { hdmi_codecs.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { hdmi_platforms.as_mut_ptr() },
        num_platforms: 1,
    },
];

/* max98090 and HDMI codec dai_link */
static mut rk_max98090_hdmi_dailinks: [snd_soc_dai_link; 2] = [
    snd_soc_dai_link {
        name: c"max98090".as_ptr(),
        stream_name: c"Analog".as_ptr(),
        init: Some(rk_init),
        ops: &rk_aif1_ops,
        /* set max98090 as slave */
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        cpus: unsafe { analog_cpus.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { analog_codecs.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { analog_platforms.as_mut_ptr() },
        num_platforms: 1,
    },
    snd_soc_dai_link {
        name: c"HDMI".as_ptr(),
        stream_name: c"HDMI".as_ptr(),
        init: Some(rk_hdmi_init),
        ops: &rk_aif1_ops,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        cpus: unsafe { hdmi_cpus.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { hdmi_codecs.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { hdmi_platforms.as_mut_ptr() },
        num_platforms: 1,
    },
];

unsafe extern "C" fn rk_98090_headset_init(component: *mut snd_soc_component) -> c_int {
    let mut ret: c_int;

    /* Enable Headset and 4 Buttons Jack detection */
    ret = snd_soc_card_jack_new_pins(
        (*component).card,
        c"Headset Jack".as_ptr(),
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        &raw mut headset_jack,
        headset_jack_pins.as_mut_ptr(),
        headset_jack_pins.len() as c_int,
    );
    if ret != 0 {
        return ret;
    }

    ret = ts3a227e_enable_jack_detect(component, &raw mut headset_jack);

    ret
}

static mut rk_98090_headset_dev: snd_soc_aux_dev = snd_soc_aux_dev {
    dlc: snd_soc_dai_link_component { of_node: ptr::null_mut() },
    init: Some(rk_98090_headset_init),
};

static mut rockchip_max98090_card: snd_soc_card = snd_soc_card {
    name: c"ROCKCHIP-I2S".as_ptr(),
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { rk_max98090_dailinks.as_mut_ptr() },
    num_links: 1,
    aux_dev: unsafe { &raw mut rk_98090_headset_dev },
    num_aux_devs: 1,
    dapm_widgets: rk_max98090_dapm_widgets.as_ptr(),
    num_dapm_widgets: 4,
    dapm_routes: rk_max98090_audio_map.as_ptr(),
    num_dapm_routes: 7,
    controls: rk_max98090_controls.as_ptr(),
    num_controls: 4,
    dev: ptr::null_mut(),
};

static mut rockchip_hdmi_card: snd_soc_card = snd_soc_card {
    name: c"ROCKCHIP-HDMI".as_ptr(),
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { rk_hdmi_dailinks.as_mut_ptr() },
    num_links: 1,
    aux_dev: ptr::null_mut(),
    num_aux_devs: 0,
    dapm_widgets: rk_hdmi_dapm_widgets.as_ptr(),
    num_dapm_widgets: 1,
    dapm_routes: rk_hdmi_audio_map.as_ptr(),
    num_dapm_routes: 1,
    controls: rk_hdmi_controls.as_ptr(),
    num_controls: 1,
    dev: ptr::null_mut(),
};

static mut rockchip_max98090_hdmi_card: snd_soc_card = snd_soc_card {
    name: c"ROCKCHIP-MAX98090-HDMI".as_ptr(),
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { rk_max98090_hdmi_dailinks.as_mut_ptr() },
    num_links: 2,
    aux_dev: unsafe { &raw mut rk_98090_headset_dev },
    num_aux_devs: 1,
    dapm_widgets: rk_max98090_hdmi_dapm_widgets.as_ptr(),
    num_dapm_widgets: 5,
    dapm_routes: rk_max98090_hdmi_audio_map.as_ptr(),
    num_dapm_routes: 8,
    controls: rk_max98090_hdmi_controls.as_ptr(),
    num_controls: 5,
    dev: ptr::null_mut(),
};

unsafe extern "C" fn rk_parse_headset_from_of(dev: *mut device, np: *mut device_node) -> c_int {
    rk_98090_headset_dev.dlc.of_node =
        of_parse_phandle(np, c"rockchip,headset-codec".as_ptr(), 0);
    if rk_98090_headset_dev.dlc.of_node.is_null() {
        dev_err(
            dev,
            c"Property 'rockchip,headset-codec' missing/invalid\n".as_ptr(),
        );
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn snd_rk_mc_probe(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int = 0;
    let card: *mut snd_soc_card;
    let dev = &raw mut (*pdev).dev as *mut platform_device_dev as *mut device;
    let np = (*pdev).dev.of_node;
    let np_cpu: *mut device_node;
    let np_audio: *mut device_node;
    let np_hdmi: *mut device_node;

    /* Parse DTS for I2S controller. */
    np_cpu = of_parse_phandle(np, c"rockchip,i2s-controller".as_ptr(), 0);

    if np_cpu.is_null() {
        dev_err(
            &raw mut (*pdev).dev as *mut platform_device_dev as *mut device,
            c"Property 'rockchip,i2s-controller missing or invalid\n".as_ptr(),
        );
        return -EINVAL;
    }

    /*
     * Find the card to use based on the presences of audio codec
     * and hdmi codec in device property. Set their of_node accordingly.
     */
    np_audio = of_parse_phandle(np, c"rockchip,audio-codec".as_ptr(), 0);
    np_hdmi = of_parse_phandle(np, c"rockchip,hdmi-codec".as_ptr(), 0);
    if !np_audio.is_null() && !np_hdmi.is_null() {
        card = &raw mut rockchip_max98090_hdmi_card;
        (*(*card).dai_link.add(DAILINK_MAX98090)).codecs.as_mut().unwrap().of_node = np_audio;
        (*(*card).dai_link.add(DAILINK_HDMI)).codecs.as_mut().unwrap().of_node = np_hdmi;
        (*(*card).dai_link.add(DAILINK_MAX98090)).cpus.as_mut().unwrap().of_node = np_cpu;
        (*(*card).dai_link.add(DAILINK_MAX98090)).platforms.as_mut().unwrap().of_node = np_cpu;
        (*(*card).dai_link.add(DAILINK_HDMI)).cpus.as_mut().unwrap().of_node = np_cpu;
        (*(*card).dai_link.add(DAILINK_HDMI)).platforms.as_mut().unwrap().of_node = np_cpu;
    } else if !np_audio.is_null() {
        card = &raw mut rockchip_max98090_card;
        (*(*card).dai_link.add(0)).codecs.as_mut().unwrap().of_node = np_audio;
        (*(*card).dai_link.add(0)).cpus.as_mut().unwrap().of_node = np_cpu;
        (*(*card).dai_link.add(0)).platforms.as_mut().unwrap().of_node = np_cpu;
    } else if !np_hdmi.is_null() {
        card = &raw mut rockchip_hdmi_card;
        (*(*card).dai_link.add(0)).codecs.as_mut().unwrap().of_node = np_hdmi;
        (*(*card).dai_link.add(0)).cpus.as_mut().unwrap().of_node = np_cpu;
        (*(*card).dai_link.add(0)).platforms.as_mut().unwrap().of_node = np_cpu;
    } else {
        dev_err(dev, c"At least one of codecs should be specified\n".as_ptr());
        return -EINVAL;
    }

    (*card).dev = dev;

    /* Parse headset detection codec. */
    if !np_audio.is_null() {
        ret = rk_parse_headset_from_of(dev, np);
        if ret != 0 {
            return ret;
        }
    }

    /* Parse card name. */
    ret = snd_soc_of_parse_card_name(card, c"rockchip,model".as_ptr());
    if ret != 0 {
        return ret;
    }

    /* register the soc card */
    ret = devm_snd_soc_register_card(&raw mut (*pdev).dev, card);
    if ret != 0 {
        dev_err(
            &raw mut (*pdev).dev as *mut platform_device_dev as *mut device,
            c"Soc register card failed %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    ret
}

static rockchip_max98090_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"rockchip,rockchip-audio-max98090".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

/* MODULE_DEVICE_TABLE(of, rockchip_max98090_of_match); */

static snd_rk_mc_driver: platform_driver = platform_driver {
    probe: Some(snd_rk_mc_probe),
    driver: platform_driver_driver {
        name: DRV_NAME.as_ptr() as *const c_char,
        pm: &snd_soc_pm_ops,
        of_match_table: rockchip_max98090_of_match.as_ptr(),
    },
};

/* module_platform_driver(snd_rk_mc_driver); */

/* MODULE_AUTHOR("jianqun <jay.xu@rock-chips.com>"); */
/* MODULE_DESCRIPTION("Rockchip max98090 machine ASoC driver"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("platform:" DRV_NAME); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
