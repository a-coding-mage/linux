// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2023 Intel Corporation
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

const RT5514_CODEC_DAI: &[u8] = b"rt5514-aif1\0";

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_uint = 0;
const SNDRV_PCM_HW_PARAM_FORMAT: c_uint = 0;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 0;
const RT5514_SCLK_S_MCLK: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAIFMT_DSP_B: c_uint = 0;
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
pub struct snd_soc_dapm_context {
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
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub reg: c_int,
    pub shift: c_uchar,
    pub mask: c_uint,
    pub on_val: c_uint,
    pub off_val: c_uint,
    pub power: c_uchar,
    pub invert: c_uchar,
    pub ignore_suspend: c_uchar,
    pub event_flags: c_ushort,
    pub event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut c_void, c_int) -> c_int>,
    pub kcontrol_news: *const c_void,
    pub num_kcontrols: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: *const c_void,
    pub shutdown: *const c_void,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
    pub id: c_int,
    pub dai_fmt: c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub be_hw_params_fixup:
        Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    pub nonatomic: c_uint,
    pub no_pcm: c_uint,
    pub capture_only: c_uint,
    pub ops: *const snd_soc_ops,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub long_name: *const c_char,
    pub driver_name: *const c_char,
    pub dev: *mut device,
    pub owner: *mut c_void,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
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
    pub driver_data: usize,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const c_void,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
    pub id_table: *const platform_device_id,
}

type c_uchar = u8;
type c_ushort = u16;

static DMIC: &[u8] = b"DMIC\0";
static DMIC1L: &[u8] = b"DMIC1L\0";
static DMIC1R: &[u8] = b"DMIC1R\0";
static DMIC2L: &[u8] = b"DMIC2L\0";
static DMIC2R: &[u8] = b"DMIC2R\0";
static SET_TDM_SLOT_ERR: &[u8] = b"set TDM slot err:%d\n\0";
static SET_SYSCLK_ERR: &[u8] = b"set sysclk err: %d\n\0";
static SSP_CODEC_FMT: &[u8] = b"SSP%d-Codec%d\0";
static SSP_PIN_FMT: &[u8] = b"SSP%d Pin%d\0";
static I2C_RT5514: &[u8] = b"i2c-10EC5514:00\0";
static AVS_RT5514: &[u8] = b"avs_rt5514\0";
static AVS_I2S_ALC5514: &[u8] = b"AVS I2S ALC5514\0";
static FAILED_CREATE_DAI_LINK: &[u8] = b"Failed to create dai link: %d\0";
static DMIC_IGNORE_SUSPEND_FAILED: &[u8] = b"DMIC - Ignore suspend failed = %d\n\0";
static MODULE_DESCRIPTION_STR: &[u8] = b"Intel rt5514 machine driver\0";
static MODULE_LICENSE_STR: &[u8] = b"GPL\0";

/* SND_SOC_DAPM_MIC("DMIC", NULL) */
static card_widgets: [snd_soc_dapm_widget; 1] = [snd_soc_dapm_widget {
    id: 0,
    name: DMIC.as_ptr() as *const c_char,
    reg: 0,
    shift: 0,
    mask: 0,
    on_val: 0,
    off_val: 0,
    power: 0,
    invert: 0,
    ignore_suspend: 0,
    event_flags: 0,
    event: None,
    kcontrol_news: ptr::null(),
    num_kcontrols: 0,
}];

static card_base_routes: [snd_soc_dapm_route; 4] = [
    /* DMIC */
    snd_soc_dapm_route {
        sink: DMIC1L.as_ptr() as *const c_char,
        control: ptr::null(),
        source: DMIC.as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: DMIC1R.as_ptr() as *const c_char,
        control: ptr::null(),
        source: DMIC.as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: DMIC2L.as_ptr() as *const c_char,
        control: ptr::null(),
        source: DMIC.as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: DMIC2R.as_ptr() as *const c_char,
        control: ptr::null(),
        source: DMIC.as_ptr() as *const c_char,
    },
];

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: c_void;

    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_ignore_suspend(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_uint) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_uint) -> *mut snd_mask;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_set_format(mask: *mut snd_mask, format: c_int);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *const c_char;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn avs_mach_get_ssp_tdm(
        dev: *mut device,
        mach: *mut snd_soc_acpi_mach,
        ssp_port: *mut c_int,
        tdm_slot: *mut c_int,
    ) -> c_int;
    fn devm_snd_soc_register_deferrable_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

unsafe extern "C" fn avs_rt5514_codec_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm((*runtime).card);
    let ret: c_int = snd_soc_dapm_ignore_suspend(dapm, DMIC.as_ptr() as *const c_char);

    if ret != 0 {
        dev_err(
            (*runtime).dev,
            DMIC_IGNORE_SUSPEND_FAILED.as_ptr() as *const c_char,
            ret,
        );
    }

    ret
}

unsafe extern "C" fn avs_rt5514_be_fixup(
    _runtime: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rate: *mut snd_interval = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let channels: *mut snd_interval = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let fmt: *mut snd_mask = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);

    (*rate).max = 48000;
    (*rate).min = (*rate).max;
    (*channels).max = 4;
    (*channels).min = (*channels).max;

    snd_mask_none(fmt);
    snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S16_LE);

    0
}

unsafe extern "C" fn avs_rt5514_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mut ret: c_int;

    ret = snd_soc_dai_set_tdm_slot(codec_dai, 0xF, 0, 8, 16);
    if ret < 0 {
        dev_err((*rtd).dev, SET_TDM_SLOT_ERR.as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(codec_dai, RT5514_SCLK_S_MCLK, 24576000, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*rtd).dev, SET_SYSCLK_ERR.as_ptr() as *const c_char, ret);
    }

    ret
}

static avs_rt5514_ops: snd_soc_ops = snd_soc_ops {
    startup: ptr::null(),
    shutdown: ptr::null(),
    hw_params: Some(avs_rt5514_hw_params),
};

unsafe extern "C" fn avs_create_dai_link(
    dev: *mut device,
    ssp_port: c_int,
    tdm_slot: c_int,
    dai_link: *mut *mut snd_soc_dai_link,
) -> c_int {
    let platform: *mut snd_soc_dai_link_component =
        devm_kzalloc(dev, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL)
            as *mut snd_soc_dai_link_component;
    let dl: *mut snd_soc_dai_link =
        devm_kzalloc(dev, size_of::<snd_soc_dai_link>(), GFP_KERNEL) as *mut snd_soc_dai_link;

    if dl.is_null() || platform.is_null() {
        return -ENOMEM;
    }

    (*dl).name = devm_kasprintf(
        dev,
        GFP_KERNEL,
        SSP_CODEC_FMT.as_ptr() as *const c_char,
        ssp_port,
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
        SSP_PIN_FMT.as_ptr() as *const c_char,
        ssp_port,
        tdm_slot,
    );
    (*(*dl).codecs).name =
        devm_kasprintf(dev, GFP_KERNEL, I2C_RT5514.as_ptr() as *const c_char);
    (*(*dl).codecs).dai_name =
        devm_kasprintf(dev, GFP_KERNEL, RT5514_CODEC_DAI.as_ptr() as *const c_char);
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
    (*dl).dai_fmt = SND_SOC_DAIFMT_DSP_B | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC;
    (*dl).init = Some(avs_rt5514_codec_init);
    (*dl).be_hw_params_fixup = Some(avs_rt5514_be_fixup);
    (*dl).nonatomic = 1;
    (*dl).no_pcm = 1;
    (*dl).capture_only = 1;
    (*dl).ops = &avs_rt5514_ops;

    *dai_link = dl;

    0
}

unsafe extern "C" fn avs_rt5514_probe(pdev: *mut platform_device) -> c_int {
    let mut dai_link: *mut snd_soc_dai_link = ptr::null_mut();
    let mach: *mut snd_soc_acpi_mach;
    let pdata: *mut avs_mach_pdata;
    let card: *mut snd_soc_card;
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
            FAILED_CREATE_DAI_LINK.as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    card = devm_kzalloc(dev, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if card.is_null() {
        return -ENOMEM;
    }

    if (*pdata).obsolete_card_names {
        (*card).name = AVS_RT5514.as_ptr() as *const c_char;
    } else {
        (*card).driver_name = AVS_RT5514.as_ptr() as *const c_char;
        (*card).name = AVS_I2S_ALC5514.as_ptr() as *const c_char;
        (*card).long_name = (*card).name;
    }
    (*card).dev = dev;
    (*card).owner = THIS_MODULE;
    (*card).dai_link = dai_link;
    (*card).num_links = 1;
    (*card).dapm_widgets = card_widgets.as_ptr();
    (*card).num_dapm_widgets = card_widgets.len() as c_int;
    (*card).dapm_routes = card_base_routes.as_ptr();
    (*card).num_dapm_routes = card_base_routes.len() as c_int;
    (*card).fully_routed = true;

    devm_snd_soc_register_deferrable_card(dev, card)
}

static avs_rt5514_driver_ids: [platform_device_id; 2] = [
    platform_device_id {
        name: [
            b'a' as c_char,
            b'v' as c_char,
            b's' as c_char,
            b'_' as c_char,
            b'r' as c_char,
            b't' as c_char,
            b'5' as c_char,
            b'5' as c_char,
            b'1' as c_char,
            b'4' as c_char,
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
        driver_data: 0,
    },
    platform_device_id {
        name: [0; 20],
        driver_data: 0,
    },
];

/* MODULE_DEVICE_TABLE(platform, avs_rt5514_driver_ids); */

static mut avs_rt5514_driver: platform_driver = platform_driver {
    probe: Some(avs_rt5514_probe),
    driver: device_driver {
        name: AVS_RT5514.as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops as *const c_void },
    },
    id_table: avs_rt5514_driver_ids.as_ptr(),
};

/* module_platform_driver(avs_rt5514_driver); */

/* MODULE_DESCRIPTION("Intel rt5514 machine driver"); */
/* MODULE_LICENSE("GPL"); */
const _MODULE_DESCRIPTION: *const c_char = MODULE_DESCRIPTION_STR.as_ptr() as *const c_char;
const _MODULE_LICENSE: *const c_char = MODULE_LICENSE_STR.as_ptr() as *const c_char;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
