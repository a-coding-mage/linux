// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2022-2023 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const RT5663_CODEC_DAI: *const c_char = b"rt5663-aif\0".as_ptr() as *const c_char;

const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SND_JACK_HEADSET: c_int = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_JACK_BTN_0: c_int = 0x4000;
const SND_JACK_BTN_1: c_int = 0x2000;
const SND_JACK_BTN_2: c_int = 0x1000;
const SND_JACK_BTN_3: c_int = 0x0800;
const KEY_PLAYPAUSE: c_int = 164;
const KEY_VOICECOMMAND: c_int = 246;
const KEY_VOLUMEUP: c_int = 115;
const KEY_VOLUMEDOWN: c_int = 114;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 1;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 2;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 6;
const RT5663_DA_STEREO_FILTER: c_uint = 0;
const RT5663_AD_STEREO_FILTER: c_uint = 0;
const RT5663_CLK_SEL_I2S1_ASRC: c_uint = 0;
const RT5663_SCLK_S_MCLK: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;

#[repr(C)]
pub struct snd_soc_jack {
    pub jack: *mut snd_jack,
}

#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rt5663_private {
    jack: snd_soc_jack,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
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
pub struct snd_soc_jack_pin {
    pin: *const c_char,
    mask: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_card {
    name: *const c_char,
    driver_name: *const c_char,
    long_name: *const c_char,
    dev: *mut device,
    owner: *mut module,
    suspend_pre: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    resume_post: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    dai_link: *mut snd_soc_dai_link,
    num_links: c_int,
    controls: *const snd_kcontrol_new,
    num_controls: c_int,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_int,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_int,
    fully_routed: bool,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    name: *const c_char,
    dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    name: *const c_char,
    cpus: *mut snd_soc_dai_link_component,
    codecs: *mut snd_soc_dai_link_component,
    num_cpus: c_int,
    num_codecs: c_int,
    platforms: *mut snd_soc_dai_link_component,
    num_platforms: c_int,
    id: c_int,
    dai_fmt: c_uint,
    init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    exit: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime)>,
    be_hw_params_fixup:
        Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    nonatomic: c_uint,
    no_pcm: c_uint,
    ops: *const snd_soc_ops,
}

#[repr(C)]
pub struct snd_soc_ops {
    hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_interval {
    min: c_uint,
    max: c_uint,
}

#[repr(C)]
pub struct snd_mask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pdata: *mut avs_mach_pdata,
}

#[repr(C)]
pub struct avs_mach_pdata {
    obsolete_card_names: bool,
}

#[repr(C)]
pub struct platform_device_id {
    name: [c_char; 20],
}

#[repr(C)]
pub struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: device_driver,
    id_table: *const platform_device_id,
}

#[repr(C)]
pub struct device_driver {
    name: *const c_char,
    pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn devm_kmemdup_array(
        dev: *mut device,
        src: *const c_void,
        n: usize,
        size: usize,
        flags: c_uint,
    ) -> *mut c_void;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_int,
    ) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_int);
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn hw_param_interval(
        params: *mut snd_pcm_hw_params,
        var: c_int,
    ) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_set_format(mask: *mut snd_mask, format: c_int);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn rt5663_sel_asrc_clk_src(
        component: *mut snd_soc_component,
        filter_mask: c_uint,
        clk_src: c_uint,
    ) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn snd_soc_card_get_codec_dai(
        card: *mut snd_soc_card,
        dai_name: *const c_char,
    ) -> *mut snd_soc_dai;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn avs_mach_get_ssp_tdm(
        dev: *mut device,
        mach: *mut snd_soc_acpi_mach,
        ssp_port: *mut c_int,
        tdm_slot: *mut c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_snd_soc_register_deferrable_card(
        dev: *mut device,
        card: *mut snd_soc_card,
    ) -> c_int;
}

macro_rules! SOC_DAPM_PIN_SWITCH {
    ($name:expr) => {
        snd_kcontrol_new { _private: [] }
    };
}

macro_rules! SND_SOC_DAPM_HP {
    ($name:expr, $event:expr) => {
        snd_soc_dapm_widget { _private: [] }
    };
}

macro_rules! SND_SOC_DAPM_MIC {
    ($name:expr, $event:expr) => {
        snd_soc_dapm_widget { _private: [] }
    };
}

macro_rules! AVS_STRING_FMT {
    ($prefix:expr, $suffix:expr, $ssp_port:expr, $tdm_slot:expr) => {
        b"%s%d%s%d\0".as_ptr() as *const c_char
    };
}

static card_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_PIN_SWITCH!("Headphone Jack"),
    SOC_DAPM_PIN_SWITCH!("Headset Mic"),
];

static card_widgets: [snd_soc_dapm_widget; 2] = [
    SND_SOC_DAPM_HP!("Headphone Jack", ptr::null_mut::<c_void>()),
    SND_SOC_DAPM_MIC!("Headset Mic", ptr::null_mut::<c_void>()),
];

static card_routes: [snd_soc_dapm_route; 4] = [
    /* HP jack connectors */
    snd_soc_dapm_route {
        sink: b"Headphone Jack\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPOL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Headphone Jack\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPOR\0".as_ptr() as *const c_char,
    },
    /* Mic jacks */
    snd_soc_dapm_route {
        sink: b"IN1P\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Headset Mic\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"IN1N\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Headset Mic\0".as_ptr() as *const c_char,
    },
];

static card_headset_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: b"Headphone Jack\0".as_ptr() as *const c_char,
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: b"Headset Mic\0".as_ptr() as *const c_char,
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe extern "C" fn avs_rt5663_codec_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*runtime).card;
    let priv_ = snd_soc_card_get_drvdata(card) as *mut rt5663_private;
    let pins: *mut snd_soc_jack_pin;
    let jack: *mut snd_soc_jack;
    let num_pins: c_int;
    let ret: c_int;

    jack = &mut (*priv_).jack;
    num_pins = card_headset_pins.len() as c_int;

    pins = devm_kmemdup_array(
        (*card).dev,
        card_headset_pins.as_ptr() as *const c_void,
        num_pins as usize,
        size_of::<snd_soc_jack_pin>(),
        GFP_KERNEL,
    ) as *mut snd_soc_jack_pin;
    if pins.is_null() {
        return -ENOMEM;
    }

    ret = snd_soc_card_jack_new_pins(
        card,
        b"Headset Jack\0".as_ptr() as *const c_char,
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        jack,
        pins,
        num_pins,
    );
    if ret != 0 {
        return ret;
    }

    snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);

    snd_soc_component_set_jack(
        (*snd_soc_rtd_to_codec(runtime, 0)).component,
        jack,
        ptr::null_mut(),
    );

    0
}

unsafe extern "C" fn avs_rt5663_codec_exit(runtime: *mut snd_soc_pcm_runtime) {
    snd_soc_component_set_jack(
        (*snd_soc_rtd_to_codec(runtime, 0)).component,
        ptr::null_mut(),
        ptr::null_mut(),
    );
}

unsafe extern "C" fn avs_rt5663_be_fixup(
    _runtime: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rate: *mut snd_interval;
    let channels: *mut snd_interval;
    let fmt: *mut snd_mask;

    rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    fmt = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);

    /* The ADSP will convert the FE rate to 48k, stereo */
    (*rate).max = 48000;
    (*rate).min = (*rate).max;
    (*channels).max = 2;
    (*channels).min = (*channels).max;

    /* set SSPN to 24 bit */
    snd_mask_none(fmt);
    snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S24_LE);

    0
}

unsafe extern "C" fn avs_rt5663_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let ret: c_int;

    /* use ASRC for internal clocks, as PLL rate isn't multiple of BCLK */
    rt5663_sel_asrc_clk_src(
        (*codec_dai).component,
        RT5663_DA_STEREO_FILTER | RT5663_AD_STEREO_FILTER,
        RT5663_CLK_SEL_I2S1_ASRC,
    );

    ret = snd_soc_dai_set_sysclk(codec_dai, RT5663_SCLK_S_MCLK, 24576000, SND_SOC_CLOCK_IN);

    ret
}

static avs_rt5663_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(avs_rt5663_hw_params),
};

unsafe extern "C" fn avs_create_dai_link(
    dev: *mut device,
    ssp_port: c_int,
    tdm_slot: c_int,
    dai_link: *mut *mut snd_soc_dai_link,
) -> c_int {
    let platform: *mut snd_soc_dai_link_component;
    let dl: *mut snd_soc_dai_link;

    dl = devm_kzalloc(dev, size_of::<snd_soc_dai_link>(), GFP_KERNEL) as *mut snd_soc_dai_link;
    platform = devm_kzalloc(
        dev,
        size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if dl.is_null() || platform.is_null() {
        return -ENOMEM;
    }

    (*dl).name = devm_kasprintf(
        dev,
        GFP_KERNEL,
        AVS_STRING_FMT!("SSP", "-Codec", ssp_port, tdm_slot),
        b"SSP\0".as_ptr() as *const c_char,
        ssp_port,
        b"-Codec\0".as_ptr() as *const c_char,
        tdm_slot,
    );
    (*dl).cpus = devm_kzalloc(
        dev,
        size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    (*dl).codecs = devm_kzalloc(
        dev,
        size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if (*dl).name.is_null() || (*dl).cpus.is_null() || (*dl).codecs.is_null() {
        return -ENOMEM;
    }

    (*(*dl).cpus).dai_name = devm_kasprintf(
        dev,
        GFP_KERNEL,
        AVS_STRING_FMT!("SSP", " Pin", ssp_port, tdm_slot),
        b"SSP\0".as_ptr() as *const c_char,
        ssp_port,
        b" Pin\0".as_ptr() as *const c_char,
        tdm_slot,
    );
    (*(*dl).codecs).name = devm_kasprintf(
        dev,
        GFP_KERNEL,
        b"i2c-10EC5663:00\0".as_ptr() as *const c_char,
    );
    (*(*dl).codecs).dai_name = devm_kasprintf(dev, GFP_KERNEL, RT5663_CODEC_DAI);
    if (*(*dl).cpus).dai_name.is_null()
        || (*(*dl).codecs).name.is_null()
        || (*(*dl).codecs).dai_name.is_null()
    {
        return -ENOMEM;
    }

    (*platform).name = dev_name(dev);
    (*dl).num_cpus = 1;
    (*dl).num_codecs = 1;
    (*dl).platforms = platform;
    (*dl).num_platforms = 1;
    (*dl).id = 0;
    (*dl).dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC;
    (*dl).init = Some(avs_rt5663_codec_init);
    (*dl).exit = Some(avs_rt5663_codec_exit);
    (*dl).be_hw_params_fixup = Some(avs_rt5663_be_fixup);
    (*dl).nonatomic = 1;
    (*dl).no_pcm = 1;
    (*dl).ops = &avs_rt5663_ops;

    *dai_link = dl;

    0
}

unsafe extern "C" fn avs_card_suspend_pre(card: *mut snd_soc_card) -> c_int {
    let codec_dai = snd_soc_card_get_codec_dai(card, RT5663_CODEC_DAI);

    snd_soc_component_set_jack((*codec_dai).component, ptr::null_mut(), ptr::null_mut())
}

unsafe extern "C" fn avs_card_resume_post(card: *mut snd_soc_card) -> c_int {
    let codec_dai = snd_soc_card_get_codec_dai(card, RT5663_CODEC_DAI);
    let jack = snd_soc_card_get_drvdata(card) as *mut snd_soc_jack;

    snd_soc_component_set_jack((*codec_dai).component, jack, ptr::null_mut())
}

unsafe extern "C" fn avs_rt5663_probe(pdev: *mut platform_device) -> c_int {
    let mut dai_link: *mut snd_soc_dai_link = ptr::null_mut();
    let mach: *mut snd_soc_acpi_mach;
    let pdata: *mut avs_mach_pdata;
    let card: *mut snd_soc_card;
    let priv_: *mut rt5663_private;
    let dev: *mut device = &mut (*pdev).dev;
    let mut ssp_port: c_int = 0;
    let mut tdm_slot: c_int = 0;
    let mut ret: c_int;

    mach = dev_get_platdata(dev) as *mut snd_soc_acpi_mach;
    pdata = (*mach).pdata;

    ret = avs_mach_get_ssp_tdm(dev, mach, &mut ssp_port, &mut tdm_slot);
    if ret != 0 {
        return ret;
    }

    ret = avs_create_dai_link(dev, ssp_port, tdm_slot, &mut dai_link);
    if ret != 0 {
        dev_err(
            dev,
            b"Failed to create dai link: %d\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    priv_ = devm_kzalloc(dev, size_of::<rt5663_private>(), GFP_KERNEL) as *mut rt5663_private;
    card = devm_kzalloc(dev, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if priv_.is_null() || card.is_null() {
        return -ENOMEM;
    }

    if (*pdata).obsolete_card_names {
        (*card).name = b"avs_rt5663\0".as_ptr() as *const c_char;
    } else {
        (*card).driver_name = b"avs_rt5663\0".as_ptr() as *const c_char;
        (*card).name = b"AVS I2S ALC5663\0".as_ptr() as *const c_char;
        (*card).long_name = (*card).name;
    }
    (*card).dev = dev;
    (*card).owner = THIS_MODULE;
    (*card).suspend_pre = Some(avs_card_suspend_pre);
    (*card).resume_post = Some(avs_card_resume_post);
    (*card).dai_link = dai_link;
    (*card).num_links = 1;
    (*card).controls = card_controls.as_ptr();
    (*card).num_controls = card_controls.len() as c_int;
    (*card).dapm_widgets = card_widgets.as_ptr();
    (*card).num_dapm_widgets = card_widgets.len() as c_int;
    (*card).dapm_routes = card_routes.as_ptr();
    (*card).num_dapm_routes = card_routes.len() as c_int;
    (*card).fully_routed = true;
    snd_soc_card_set_drvdata(card, priv_ as *mut c_void);

    devm_snd_soc_register_deferrable_card(dev, card)
}

static avs_rt5663_driver_ids: [platform_device_id; 2] = [
    platform_device_id {
        name: [
            b'a' as c_char,
            b'v' as c_char,
            b's' as c_char,
            b'_' as c_char,
            b'r' as c_char,
            b't' as c_char,
            b'5' as c_char,
            b'6' as c_char,
            b'6' as c_char,
            b'3' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    },
    platform_device_id { name: [0; 20] },
];

// MODULE_DEVICE_TABLE(platform, avs_rt5663_driver_ids);

static mut avs_rt5663_driver: platform_driver = platform_driver {
    probe: Some(avs_rt5663_probe),
    driver: device_driver {
        name: b"avs_rt5663\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops },
    },
    id_table: avs_rt5663_driver_ids.as_ptr(),
};

// module_platform_driver(avs_rt5663_driver);
// MODULE_DESCRIPTION("Intel rt5663 machine driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
