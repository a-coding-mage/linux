// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2022-2025 Intel Corporation
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

const AVS_RT5640_MCLK_HZ: c_int = 19200000;
const RT5640_CODEC_DAI: &[u8] = b"rt5640-aif1\0";

type u32 = c_uint;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 0;
const SNDRV_PCM_FORMAT_S32_LE: c_int = 0;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 0;
const SND_JACK_HEADPHONE: c_int = 0;
const SND_JACK_MICROPHONE: c_int = 0;
const SND_JACK_HEADSET: c_int = 0;
const RT5640_PLL1_S_MCLK: c_int = 0;
const RT5640_SCLK_S_PLL1: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const RT5640_DA_STEREO_FILTER: c_int = 0;
const RT5640_AD_STEREO_FILTER: c_int = 0;
const RT5640_DA_MONO_L_FILTER: c_int = 0;
const RT5640_DA_MONO_R_FILTER: c_int = 0;
const RT5640_AD_MONO_L_FILTER: c_int = 0;
const RT5640_AD_MONO_R_FILTER: c_int = 0;
const RT5640_CLK_SEL_ASRC: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;

extern "C" {
    static mut THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_rtd_to_codec(
        runtime: *mut snd_soc_pcm_runtime,
        n: c_int,
    ) -> *mut snd_soc_dai;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn devm_kmemdup(
        dev: *mut device,
        src: *const c_void,
        len: usize,
        gfp: c_uint,
    ) -> *mut c_void;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_int,
    ) -> c_int;
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn snd_soc_dapm_set_idle_bias(dapm: *mut snd_soc_dapm_context, idle_bias: bool);
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_mask_set_format(mask: *mut snd_mask, val: c_int);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn rt5640_sel_asrc_clk_src(
        component: *mut snd_soc_component,
        filter_mask: c_int,
        clk_src: c_int,
    ) -> c_int;
    fn kstrtou32(s: *const c_char, base: c_uint, res: *mut u32) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn dev_name(dev: *const device) -> *const c_char;
    fn snd_soc_card_get_codec_dai(
        card: *mut snd_soc_card,
        dai_name: *const c_char,
    ) -> *mut snd_soc_dai;
    fn dev_get_platdata(dev: *const device) -> *mut c_void;
    fn avs_mach_get_ssp_tdm(
        dev: *mut device,
        mach: *mut snd_soc_acpi_mach,
        ssp_port: *mut c_int,
        tdm_slot: *mut c_int,
    ) -> c_int;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn devm_snd_soc_register_deferrable_card(
        dev: *mut device,
        card: *mut snd_soc_card,
    ) -> c_int;
}

#[repr(C)]
struct module {
    _private: [u8; 0],
}

#[repr(C)]
struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_mask {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_dapm_widget {
    _private: [usize; 3],
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_soc_jack_pin {
    pin: *const c_char,
    mask: c_int,
}

#[repr(C)]
struct snd_soc_ops {
    hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
struct snd_soc_dai_link_component {
    name: *const c_char,
    dai_name: *const c_char,
}

#[repr(C)]
struct snd_soc_dai_link {
    name: *const c_char,
    cpus: *mut snd_soc_dai_link_component,
    codecs: *mut snd_soc_dai_link_component,
    num_cpus: c_uint,
    num_codecs: c_uint,
    platforms: *mut snd_soc_dai_link_component,
    num_platforms: c_uint,
    id: c_int,
    dai_fmt: c_uint,
    init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    exit: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime)>,
    be_hw_params_fixup:
        Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    ops: *const snd_soc_ops,
    nonatomic: c_uint,
    no_pcm: c_uint,
}

#[repr(C)]
struct snd_soc_acpi_mach {
    uid: *const c_char,
}

#[repr(C)]
struct snd_soc_card {
    name: *const c_char,
    driver_name: *const c_char,
    long_name: *const c_char,
    dev: *mut device,
    owner: *mut module,
    suspend_pre: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    resume_post: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    dai_link: *mut snd_soc_dai_link,
    num_links: c_int,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_int,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_int,
    fully_routed: bool,
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct platform_device_id {
    name: [c_char; 20],
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: device_driver,
    id_table: *const platform_device_id,
}

macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! snd_soc_dapm_hp {
    ($name:expr, $event:expr) => {
        snd_soc_dapm_widget {
            _private: [c_str!($name) as usize, $event as usize, 0],
        }
    };
}

macro_rules! snd_soc_dapm_mic {
    ($name:expr, $event:expr) => {
        snd_soc_dapm_widget {
            _private: [c_str!($name) as usize, $event as usize, 0],
        }
    };
}

macro_rules! snd_soc_dapm_spk {
    ($name:expr, $event:expr) => {
        snd_soc_dapm_widget {
            _private: [c_str!($name) as usize, $event as usize, 0],
        }
    };
}

static card_widgets: [snd_soc_dapm_widget; 3] = [
    snd_soc_dapm_hp!("Headphone Jack", ptr::null::<c_void>()),
    snd_soc_dapm_mic!("Mic Jack", ptr::null::<c_void>()),
    snd_soc_dapm_spk!("Speaker", ptr::null::<c_void>()),
];

static card_routes: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route {
        sink: c_str!("Headphone Jack"),
        control: ptr::null(),
        source: c_str!("HPOR"),
    },
    snd_soc_dapm_route {
        sink: c_str!("Headphone Jack"),
        control: ptr::null(),
        source: c_str!("HPOL"),
    },
    snd_soc_dapm_route {
        sink: c_str!("IN2P"),
        control: ptr::null(),
        source: c_str!("Mic Jack"),
    },
    snd_soc_dapm_route {
        sink: c_str!("IN2P"),
        control: ptr::null(),
        source: c_str!("MICBIAS1"),
    },
    snd_soc_dapm_route {
        sink: c_str!("Speaker"),
        control: ptr::null(),
        source: c_str!("SPOLP"),
    },
    snd_soc_dapm_route {
        sink: c_str!("Speaker"),
        control: ptr::null(),
        source: c_str!("SPOLN"),
    },
    snd_soc_dapm_route {
        sink: c_str!("Speaker"),
        control: ptr::null(),
        source: c_str!("SPORP"),
    },
    snd_soc_dapm_route {
        sink: c_str!("Speaker"),
        control: ptr::null(),
        source: c_str!("SPORN"),
    },
];

static card_headset_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c_str!("Headphone Jack"),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c_str!("Mic Jack"),
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe extern "C" fn avs_rt5640_codec_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let codec_dai = snd_soc_rtd_to_codec(runtime, 0);
    let card = (*runtime).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut pins: *mut snd_soc_jack_pin;
    let jack: *mut snd_soc_jack;
    let num_pins: c_int;
    let mut ret: c_int;

    jack = snd_soc_card_get_drvdata(card) as *mut snd_soc_jack;
    num_pins = card_headset_pins.len() as c_int;

    pins = devm_kmemdup(
        (*card).dev,
        card_headset_pins.as_ptr() as *const c_void,
        size_of::<snd_soc_jack_pin>() * num_pins as usize,
        GFP_KERNEL,
    ) as *mut snd_soc_jack_pin;
    if pins.is_null() {
        return -ENOMEM;
    }

    ret = snd_soc_card_jack_new_pins(
        card,
        c_str!("Headset Jack"),
        SND_JACK_HEADSET,
        jack,
        pins,
        num_pins,
    );
    if ret != 0 {
        return ret;
    }

    snd_soc_component_set_jack((*codec_dai).component, jack, ptr::null_mut());
    snd_soc_dapm_set_idle_bias(dapm, false);

    0
}

unsafe extern "C" fn avs_rt5640_codec_exit(runtime: *mut snd_soc_pcm_runtime) {
    let codec_dai = snd_soc_rtd_to_codec(runtime, 0);

    snd_soc_component_set_jack((*codec_dai).component, ptr::null_mut(), ptr::null_mut());
}

unsafe extern "C" fn avs_rt5640_be_fixup(
    _runtime: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let fmask = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);

    /* Format 24/32 is MSB-aligned for HDAudio and LSB-aligned for I2S. */
    if params_format(params) == SNDRV_PCM_FORMAT_S32_LE {
        snd_mask_set_format(fmask, SNDRV_PCM_FORMAT_S24_LE);
    }

    0
}

unsafe extern "C" fn avs_rt5640_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(runtime, 0);
    let mut ret: c_int;

    ret = snd_soc_dai_set_pll(
        codec_dai,
        0,
        RT5640_PLL1_S_MCLK,
        AVS_RT5640_MCLK_HZ as c_uint,
        params_rate(params).wrapping_mul(512),
    );
    if ret < 0 {
        dev_err((*runtime).dev, c_str!("Set codec PLL failed: %d\n"), ret);
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(
        codec_dai,
        RT5640_SCLK_S_PLL1,
        params_rate(params).wrapping_mul(512),
        SND_SOC_CLOCK_IN,
    );
    if ret < 0 {
        dev_err((*runtime).dev, c_str!("Set codec SCLK failed: %d\n"), ret);
        return ret;
    }

    ret = rt5640_sel_asrc_clk_src(
        (*codec_dai).component,
        RT5640_DA_STEREO_FILTER
            | RT5640_AD_STEREO_FILTER
            | RT5640_DA_MONO_L_FILTER
            | RT5640_DA_MONO_R_FILTER
            | RT5640_AD_MONO_L_FILTER
            | RT5640_AD_MONO_R_FILTER,
        RT5640_CLK_SEL_ASRC,
    );
    if ret != 0 {
        dev_err((*runtime).dev, c_str!("Set codec ASRC failed: %d\n"), ret);
    }

    ret
}

static avs_rt5640_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(avs_rt5640_hw_params),
};

unsafe extern "C" fn avs_create_dai_link(
    dev: *mut device,
    ssp_port: c_int,
    tdm_slot: c_int,
    mach: *mut snd_soc_acpi_mach,
    dai_link: *mut *mut snd_soc_dai_link,
) -> c_int {
    let mut platform: *mut snd_soc_dai_link_component;
    let dl: *mut snd_soc_dai_link;
    let mut uid: u32 = 0;
    let mut ret: c_int;

    if !(*mach).uid.is_null() {
        ret = kstrtou32((*mach).uid, 0, &mut uid);
        if ret != 0 {
            return ret;
        }
        uid = uid.wrapping_sub(1); /* 0-based indexing. */
    }

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
        c_str!("%s%d%s%d"),
        c_str!("SSP"),
        ssp_port,
        c_str!("-Codec"),
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
        c_str!("%s%d%s%d"),
        c_str!("SSP"),
        ssp_port,
        c_str!(" Pin"),
        tdm_slot,
    );
    (*(*dl).codecs).name = devm_kasprintf(dev, GFP_KERNEL, c_str!("i2c-10EC5640:0%d"), uid);
    (*(*dl).codecs).dai_name = devm_kasprintf(dev, GFP_KERNEL, RT5640_CODEC_DAI.as_ptr() as *const c_char);
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
    (*dl).init = Some(avs_rt5640_codec_init);
    (*dl).exit = Some(avs_rt5640_codec_exit);
    (*dl).be_hw_params_fixup = Some(avs_rt5640_be_fixup);
    (*dl).ops = &avs_rt5640_ops;
    (*dl).nonatomic = 1;
    (*dl).no_pcm = 1;

    *dai_link = dl;

    0
}

unsafe extern "C" fn avs_card_suspend_pre(card: *mut snd_soc_card) -> c_int {
    let codec_dai = snd_soc_card_get_codec_dai(card, RT5640_CODEC_DAI.as_ptr() as *const c_char);

    snd_soc_component_set_jack((*codec_dai).component, ptr::null_mut(), ptr::null_mut())
}

unsafe extern "C" fn avs_card_resume_post(card: *mut snd_soc_card) -> c_int {
    let codec_dai = snd_soc_card_get_codec_dai(card, RT5640_CODEC_DAI.as_ptr() as *const c_char);
    let jack = snd_soc_card_get_drvdata(card) as *mut snd_soc_jack;

    snd_soc_component_set_jack((*codec_dai).component, jack, ptr::null_mut())
}

unsafe extern "C" fn avs_rt5640_probe(pdev: *mut platform_device) -> c_int {
    let mut dai_link: *mut snd_soc_dai_link = ptr::null_mut();
    let dev = &mut (*pdev).dev as *mut device;
    let mach: *mut snd_soc_acpi_mach;
    let card: *mut snd_soc_card;
    let jack: *mut snd_soc_jack;
    let mut ssp_port: c_int = 0;
    let mut tdm_slot: c_int = 0;
    let mut ret: c_int;

    mach = dev_get_platdata(dev) as *mut snd_soc_acpi_mach;

    ret = avs_mach_get_ssp_tdm(dev, mach, &mut ssp_port, &mut tdm_slot);
    if ret != 0 {
        return ret;
    }

    ret = avs_create_dai_link(dev, ssp_port, tdm_slot, mach, &mut dai_link);
    if ret != 0 {
        dev_err(dev, c_str!("Failed to create dai link: %d"), ret);
        return ret;
    }

    jack = devm_kzalloc(dev, size_of::<snd_soc_jack>(), GFP_KERNEL) as *mut snd_soc_jack;
    card = devm_kzalloc(dev, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if jack.is_null() || card.is_null() {
        return -ENOMEM;
    }

    if !(*mach).uid.is_null() {
        (*card).name = devm_kasprintf(
            dev,
            GFP_KERNEL,
            c_str!("AVS I2S ALC5640.%s"),
            (*mach).uid,
        );
        if (*card).name.is_null() {
            return -ENOMEM;
        }
    } else {
        (*card).name = c_str!("AVS I2S ALC5640");
    }
    (*card).driver_name = c_str!("avs_rt5640");
    (*card).long_name = (*card).name;
    (*card).dev = dev;
    (*card).owner = THIS_MODULE;
    (*card).suspend_pre = Some(avs_card_suspend_pre);
    (*card).resume_post = Some(avs_card_resume_post);
    (*card).dai_link = dai_link;
    (*card).num_links = 1;
    (*card).dapm_widgets = card_widgets.as_ptr();
    (*card).num_dapm_widgets = card_widgets.len() as c_int;
    (*card).dapm_routes = card_routes.as_ptr();
    (*card).num_dapm_routes = card_routes.len() as c_int;
    (*card).fully_routed = true;
    snd_soc_card_set_drvdata(card, jack as *mut c_void);

    devm_snd_soc_register_deferrable_card(dev, card)
}

static avs_rt5640_driver_ids: [platform_device_id; 2] = [
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
            b'4' as c_char,
            b'0' as c_char,
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
/* MODULE_DEVICE_TABLE(platform, avs_rt5640_driver_ids); */

static mut avs_rt5640_driver: platform_driver = platform_driver {
    probe: Some(avs_rt5640_probe),
    driver: device_driver {
        name: c_str!("avs_rt5640"),
        pm: unsafe { &snd_soc_pm_ops },
    },
    id_table: avs_rt5640_driver_ids.as_ptr(),
};

/* module_platform_driver(avs_rt5640_driver); */

/* MODULE_DESCRIPTION("Intel rt5640 machine driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
