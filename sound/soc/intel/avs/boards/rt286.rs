// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

// C dependencies:
// <linux/module.h>
// <sound/jack.h>
// <sound/pcm.h>
// <sound/pcm_params.h>
// <sound/soc.h>
// <sound/soc-acpi.h>
// "../../../codecs/rt286.h"
// "../utils.h"

const RT286_CODEC_DAI: *const c_char = b"rt286-aif1\0".as_ptr() as *const c_char;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const SND_JACK_HEADPHONE: c_uint = 0x0001;
const SND_JACK_MICROPHONE: c_uint = 0x0002;
const SND_JACK_BTN_0: c_uint = 0x4000;
const SND_JACK_HEADSET: c_uint = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_uint = 1;
const SNDRV_PCM_HW_PARAM_FORMAT: c_uint = 2;
const SNDRV_PCM_FORMAT_S24_LE: c_uint = 6;
const RT286_SCLK_S_PLL: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
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
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_uint,
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
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
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub cpus: *mut snd_soc_dai_link_component,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
    pub id: c_int,
    pub dai_fmt: c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime)>,
    pub be_hw_params_fixup:
        Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    pub ops: *const snd_soc_ops,
    pub nonatomic: c_uint,
    pub no_pcm: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub driver_name: *const c_char,
    pub long_name: *const c_char,
    pub dev: *mut device,
    pub owner: *mut c_void,
    pub suspend_pre: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub resume_post: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub fully_routed: bool,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub pdata: *mut avs_mach_pdata,
}

#[repr(C)]
pub struct avs_mach_pdata {
    pub obsolete_card_names: bool,
}

#[repr(C)]
pub struct platform_device_id {
    pub name: [c_char; 20],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
    pub id_table: *const platform_device_id,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
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
        type_: c_uint,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn snd_soc_rtd_to_codec(runtime: *mut snd_soc_pcm_runtime, num: c_uint) -> *mut snd_soc_dai;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_uint) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_uint) -> *mut snd_mask;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_set_format(mask: *mut snd_mask, format: c_uint);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn snd_soc_card_get_codec_dai(card: *mut snd_soc_card, dai_name: *const c_char)
        -> *mut snd_soc_dai;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn avs_mach_get_ssp_tdm(
        dev: *mut device,
        mach: *mut snd_soc_acpi_mach,
        ssp_port: *mut c_int,
        tdm_slot: *mut c_int,
    ) -> c_int;
    fn devm_snd_soc_register_deferrable_card(
        dev: *mut device,
        card: *mut snd_soc_card,
    ) -> c_int;
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

macro_rules! soc_dapm_pin_switch {
    ($name:expr) => {
        snd_kcontrol_new { _private: [] }
    };
}

macro_rules! snd_soc_dapm_hp {
    ($name:expr, $event:expr) => {
        snd_soc_dapm_widget { _private: [] }
    };
}

macro_rules! snd_soc_dapm_mic {
    ($name:expr, $event:expr) => {
        snd_soc_dapm_widget { _private: [] }
    };
}

macro_rules! snd_soc_dapm_spk {
    ($name:expr, $event:expr) => {
        snd_soc_dapm_widget { _private: [] }
    };
}

macro_rules! avs_string_fmt {
    ($prefix:expr, $suffix:expr) => {
        b"%s%d%s%d\0".as_ptr() as *const c_char
    };
}

static card_controls: [snd_kcontrol_new; 3] = [
    soc_dapm_pin_switch!("Headphone Jack"),
    soc_dapm_pin_switch!("Mic Jack"),
    soc_dapm_pin_switch!("Speaker"),
];

static card_widgets: [snd_soc_dapm_widget; 3] = [
    snd_soc_dapm_hp!("Headphone Jack", ptr::null_mut::<c_void>()),
    snd_soc_dapm_mic!("Mic Jack", ptr::null_mut::<c_void>()),
    snd_soc_dapm_spk!("Speaker", ptr::null_mut::<c_void>()),
];

static card_base_routes: [snd_soc_dapm_route; 4] = [
    /* HP jack connectors - unknown if we have jack detect */
    snd_soc_dapm_route {
        sink: b"Headphone Jack\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPO Pin\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"MIC1\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Mic Jack\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Speaker\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"SPOR\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Speaker\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"SPOL\0".as_ptr() as *const c_char,
    },
];

static card_headset_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: b"Headphone Jack\0".as_ptr() as *const c_char,
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: b"Mic Jack\0".as_ptr() as *const c_char,
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe extern "C" fn avs_rt286_codec_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*runtime).card;
    let pins: *mut snd_soc_jack_pin;
    let jack: *mut snd_soc_jack;
    let num_pins: c_int;
    let mut ret: c_int;

    jack = snd_soc_card_get_drvdata(card) as *mut snd_soc_jack;
    num_pins = card_headset_pins.len() as c_int;

    pins = devm_kmemdup_array(
        (*card).dev,
        card_headset_pins.as_ptr() as *const c_void,
        num_pins as usize,
        core::mem::size_of::<snd_soc_jack_pin>(),
        GFP_KERNEL,
    ) as *mut snd_soc_jack_pin;
    if pins.is_null() {
        return -ENOMEM;
    }

    ret = snd_soc_card_jack_new_pins(
        card,
        b"Headset Jack\0".as_ptr() as *const c_char,
        SND_JACK_HEADSET | SND_JACK_BTN_0,
        jack,
        pins,
        num_pins as c_uint,
    );
    if ret != 0 {
        return ret;
    }

    return snd_soc_component_set_jack(
        (*snd_soc_rtd_to_codec(runtime, 0)).component,
        jack,
        ptr::null_mut(),
    );
}

unsafe extern "C" fn avs_rt286_codec_exit(rtd: *mut snd_soc_pcm_runtime) {
    snd_soc_component_set_jack(
        (*snd_soc_rtd_to_codec(rtd, 0)).component,
        ptr::null_mut(),
        ptr::null_mut(),
    );
}

unsafe extern "C" fn avs_rt286_be_fixup(
    runtime: *mut snd_soc_pcm_runtime,
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

    /* set SSP0 to 24 bit */
    snd_mask_none(fmt);
    snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S24_LE);

    return 0;
}

unsafe extern "C" fn avs_rt286_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(runtime, 0);
    let ret: c_int;

    ret = snd_soc_dai_set_sysclk(codec_dai, RT286_SCLK_S_PLL, 24000000, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err(
            (*runtime).dev,
            b"Set codec sysclk failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
    }

    return ret;
}

static avs_rt286_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(avs_rt286_hw_params),
};

unsafe fn avs_create_dai_link(
    dev: *mut device,
    ssp_port: c_int,
    tdm_slot: c_int,
    dai_link: *mut *mut snd_soc_dai_link,
) -> c_int {
    let platform: *mut snd_soc_dai_link_component;
    let dl: *mut snd_soc_dai_link;

    dl = devm_kzalloc(dev, core::mem::size_of::<snd_soc_dai_link>(), GFP_KERNEL)
        as *mut snd_soc_dai_link;
    platform = devm_kzalloc(
        dev,
        core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if dl.is_null() || platform.is_null() {
        return -ENOMEM;
    }

    (*dl).name = devm_kasprintf(
        dev,
        GFP_KERNEL,
        avs_string_fmt!("SSP", "-Codec"),
        b"SSP\0".as_ptr() as *const c_char,
        ssp_port,
        b"-Codec\0".as_ptr() as *const c_char,
        tdm_slot,
    );
    (*dl).cpus = devm_kzalloc(
        dev,
        core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    (*dl).codecs = devm_kzalloc(
        dev,
        core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if (*dl).name.is_null() || (*dl).cpus.is_null() || (*dl).codecs.is_null() {
        return -ENOMEM;
    }

    (*(*dl).cpus).dai_name = devm_kasprintf(
        dev,
        GFP_KERNEL,
        avs_string_fmt!("SSP", " Pin"),
        b"SSP\0".as_ptr() as *const c_char,
        ssp_port,
        b" Pin\0".as_ptr() as *const c_char,
        tdm_slot,
    );
    (*(*dl).codecs).name =
        devm_kasprintf(dev, GFP_KERNEL, b"i2c-INT343A:00\0".as_ptr() as *const c_char);
    (*(*dl).codecs).dai_name = devm_kasprintf(dev, GFP_KERNEL, RT286_CODEC_DAI);
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
    (*dl).init = Some(avs_rt286_codec_init);
    (*dl).exit = Some(avs_rt286_codec_exit);
    (*dl).be_hw_params_fixup = Some(avs_rt286_be_fixup);
    (*dl).ops = &avs_rt286_ops;
    (*dl).nonatomic = 1;
    (*dl).no_pcm = 1;

    *dai_link = dl;

    return 0;
}

unsafe extern "C" fn avs_card_suspend_pre(card: *mut snd_soc_card) -> c_int {
    let codec_dai = snd_soc_card_get_codec_dai(card, RT286_CODEC_DAI);

    return snd_soc_component_set_jack((*codec_dai).component, ptr::null_mut(), ptr::null_mut());
}

unsafe extern "C" fn avs_card_resume_post(card: *mut snd_soc_card) -> c_int {
    let codec_dai = snd_soc_card_get_codec_dai(card, RT286_CODEC_DAI);
    let jack = snd_soc_card_get_drvdata(card) as *mut snd_soc_jack;

    return snd_soc_component_set_jack((*codec_dai).component, jack, ptr::null_mut());
}

unsafe extern "C" fn avs_rt286_probe(pdev: *mut platform_device) -> c_int {
    let mut dai_link: *mut snd_soc_dai_link = ptr::null_mut();
    let mach: *mut snd_soc_acpi_mach;
    let pdata: *mut avs_mach_pdata;
    let card: *mut snd_soc_card;
    let jack: *mut snd_soc_jack;
    let dev = &mut (*pdev).dev as *mut device;
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

    jack = devm_kzalloc(dev, core::mem::size_of::<snd_soc_jack>(), GFP_KERNEL) as *mut snd_soc_jack;
    card = devm_kzalloc(dev, core::mem::size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if jack.is_null() || card.is_null() {
        return -ENOMEM;
    }

    if (*pdata).obsolete_card_names {
        (*card).name = b"avs_rt286\0".as_ptr() as *const c_char;
    } else {
        (*card).driver_name = b"avs_rt286\0".as_ptr() as *const c_char;
        (*card).name = b"AVS I2S ALC286\0".as_ptr() as *const c_char;
        (*card).long_name = (*card).name;
    }
    (*card).dev = dev;
    (*card).owner = THIS_MODULE;
    (*card).suspend_pre = Some(avs_card_suspend_pre);
    (*card).resume_post = Some(avs_card_resume_post);
    (*card).dai_link = dai_link;
    (*card).num_links = 1;
    (*card).controls = card_controls.as_ptr();
    (*card).num_controls = card_controls.len() as c_uint;
    (*card).dapm_widgets = card_widgets.as_ptr();
    (*card).num_dapm_widgets = card_widgets.len() as c_uint;
    (*card).dapm_routes = card_base_routes.as_ptr();
    (*card).num_dapm_routes = card_base_routes.len() as c_uint;
    (*card).fully_routed = true;
    snd_soc_card_set_drvdata(card, jack as *mut c_void);

    return devm_snd_soc_register_deferrable_card(dev, card);
}

static avs_rt286_driver_ids: [platform_device_id; 2] = [
    platform_device_id {
        name: [
            b'a' as c_char,
            b'v' as c_char,
            b's' as c_char,
            b'_' as c_char,
            b'r' as c_char,
            b't' as c_char,
            b'2' as c_char,
            b'8' as c_char,
            b'6' as c_char,
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
            0,
        ],
    },
    platform_device_id { name: [0; 20] },
];
// MODULE_DEVICE_TABLE(platform, avs_rt286_driver_ids);

static mut avs_rt286_driver: platform_driver = platform_driver {
    probe: Some(avs_rt286_probe),
    driver: device_driver {
        name: b"avs_rt286\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops },
    },
    id_table: avs_rt286_driver_ids.as_ptr(),
};

// module_platform_driver(avs_rt286_driver);
unsafe fn avs_rt286_driver_init() -> c_int {
    platform_driver_register(&mut avs_rt286_driver)
}

unsafe fn avs_rt286_driver_exit() {
    platform_driver_unregister(&mut avs_rt286_driver);
}

// MODULE_DESCRIPTION("Intel rt286 machine driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
