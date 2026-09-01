// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

// C dependencies:
// #include <linux/module.h>
// #include <linux/platform_device.h>
// #include <sound/pcm_params.h>
// #include <sound/soc.h>
// #include <sound/soc-acpi.h>
// #include <sound/soc-dapm.h>
// #include "../utils.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const MAX98373_DEV0_NAME: &[u8] = b"i2c-MX98373:00\0";
const MAX98373_DEV1_NAME: &[u8] = b"i2c-MX98373:01\0";
const MAX98373_CODEC_NAME: &[u8] = b"max98373-aif1\0";

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 1;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 2;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
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
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
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
pub struct snd_soc_component {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_codec_conf {
    pub dlc: snd_soc_dai_link_component,
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int,
    >,
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
    pub be_hw_params_fixup:
        Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    pub nonatomic: c_uint,
    pub no_pcm: c_uint,
    pub ignore_pmdown_time: c_uint,
    pub ops: *const snd_soc_ops,
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
pub struct snd_soc_card {
    pub name: *const c_char,
    pub driver_name: *const c_char,
    pub long_name: *const c_char,
    pub dev: *mut device,
    pub owner: *mut c_void,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_uint,
    pub codec_conf: *mut snd_soc_codec_conf,
    pub num_configs: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub fully_routed: bool,
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

    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_set_format(mask: *mut snd_mask, format: c_int);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, gfp: c_uint) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn dev_name(dev: *mut device) -> *const c_char;
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
}

// Direct Rust placeholders for external C initializer macros:
// COMP_CODEC_CONF(), SOC_DAPM_PIN_SWITCH(), and SND_SOC_DAPM_SPK().
static mut card_codec_conf: [snd_soc_codec_conf; 2] = [
    snd_soc_codec_conf {
        dlc: snd_soc_dai_link_component {
            name: MAX98373_DEV0_NAME.as_ptr() as *const c_char,
            dai_name: ptr::null(),
        },
        name_prefix: b"Right\0".as_ptr() as *const c_char,
    },
    snd_soc_codec_conf {
        dlc: snd_soc_dai_link_component {
            name: MAX98373_DEV1_NAME.as_ptr() as *const c_char,
            dai_name: ptr::null(),
        },
        name_prefix: b"Left\0".as_ptr() as *const c_char,
    },
];

static card_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

static card_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

static card_base_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"Left Spk\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Left BE_OUT\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Right Spk\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Right BE_OUT\0".as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn avs_max98373_be_fixup(
    _runrime: *mut snd_soc_pcm_runtime,
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

    /* set SSP0 to 16 bit */
    snd_mask_none(fmt);
    snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S16_LE);
    0
}

unsafe extern "C" fn avs_max98373_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let runtime: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let mut codec_dai: *mut snd_soc_dai;
    let mut ret: c_int;
    let mut i: c_int;

    // for_each_rtd_codec_dais(runtime, i, codec_dai)
    i = 0;
    while {
        codec_dai = ptr::null_mut();
        false
    } {
        if strcmp((*(*codec_dai).component).name, MAX98373_DEV0_NAME.as_ptr() as *const c_char) == 0
        {
            ret = snd_soc_dai_set_tdm_slot(codec_dai, 0x30, 3, 8, 16);
            if ret < 0 {
                dev_err(
                    (*runtime).dev,
                    b"DEV0 TDM slot err:%d\n\0".as_ptr() as *const c_char,
                    ret,
                );
                return ret;
            }
        }
        if strcmp((*(*codec_dai).component).name, MAX98373_DEV1_NAME.as_ptr() as *const c_char) == 0
        {
            ret = snd_soc_dai_set_tdm_slot(codec_dai, 0xC0, 3, 8, 16);
            if ret < 0 {
                dev_err(
                    (*runtime).dev,
                    b"DEV1 TDM slot err:%d\n\0".as_ptr() as *const c_char,
                    ret,
                );
                return ret;
            }
        }
        i += 1;
    }

    0
}

static avs_max98373_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(avs_max98373_hw_params),
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
        b"SSP%d-Codec%d\0".as_ptr() as *const c_char,
        ssp_port,
        tdm_slot,
    );
    (*dl).cpus = devm_kzalloc(
        dev,
        size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    (*dl).codecs = devm_kcalloc(
        dev,
        2,
        size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if (*dl).name.is_null() || (*dl).cpus.is_null() || (*dl).codecs.is_null() {
        return -ENOMEM;
    }

    (*(*dl).cpus).dai_name = devm_kasprintf(
        dev,
        GFP_KERNEL,
        b"SSP%d Pin%d\0".as_ptr() as *const c_char,
        ssp_port,
        tdm_slot,
    );
    (*(*dl).codecs.add(0)).name =
        devm_kasprintf(dev, GFP_KERNEL, MAX98373_DEV0_NAME.as_ptr() as *const c_char);
    (*(*dl).codecs.add(0)).dai_name =
        devm_kasprintf(dev, GFP_KERNEL, MAX98373_CODEC_NAME.as_ptr() as *const c_char);
    (*(*dl).codecs.add(1)).name =
        devm_kasprintf(dev, GFP_KERNEL, MAX98373_DEV1_NAME.as_ptr() as *const c_char);
    (*(*dl).codecs.add(1)).dai_name =
        devm_kasprintf(dev, GFP_KERNEL, MAX98373_CODEC_NAME.as_ptr() as *const c_char);
    if (*(*dl).cpus).dai_name.is_null()
        || (*(*dl).codecs.add(0)).name.is_null()
        || (*(*dl).codecs.add(0)).dai_name.is_null()
        || (*(*dl).codecs.add(1)).name.is_null()
        || (*(*dl).codecs.add(1)).dai_name.is_null()
    {
        return -ENOMEM;
    }

    (*platform).name = dev_name(dev);
    (*dl).num_cpus = 1;
    (*dl).num_codecs = 2;
    (*dl).platforms = platform;
    (*dl).num_platforms = 1;
    (*dl).id = 0;
    (*dl).dai_fmt = SND_SOC_DAIFMT_DSP_B | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC;
    (*dl).be_hw_params_fixup = Some(avs_max98373_be_fixup);
    (*dl).nonatomic = 1;
    (*dl).no_pcm = 1;
    (*dl).ignore_pmdown_time = 1;
    (*dl).ops = &avs_max98373_ops;

    *dai_link = dl;

    0
}

unsafe extern "C" fn avs_max98373_probe(pdev: *mut platform_device) -> c_int {
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
            b"Failed to create dai link: %d\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    card = devm_kzalloc(dev, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if card.is_null() {
        return -ENOMEM;
    }

    if (*pdata).obsolete_card_names {
        (*card).name = b"avs_max98373\0".as_ptr() as *const c_char;
    } else {
        (*card).driver_name = b"avs_max98373\0".as_ptr() as *const c_char;
        (*card).name = b"AVS I2S MAX98373\0".as_ptr() as *const c_char;
        (*card).long_name = (*card).name;
    }
    (*card).dev = dev;
    (*card).owner = THIS_MODULE;
    (*card).dai_link = dai_link;
    (*card).num_links = 1;
    (*card).codec_conf = card_codec_conf.as_mut_ptr();
    (*card).num_configs = card_codec_conf.len() as c_uint;
    (*card).controls = card_controls.as_ptr();
    (*card).num_controls = card_controls.len() as c_uint;
    (*card).dapm_widgets = card_widgets.as_ptr();
    (*card).num_dapm_widgets = card_widgets.len() as c_uint;
    (*card).dapm_routes = card_base_routes.as_ptr();
    (*card).num_dapm_routes = card_base_routes.len() as c_uint;
    (*card).fully_routed = true;

    devm_snd_soc_register_deferrable_card(dev, card)
}

static avs_max98373_driver_ids: [platform_device_id; 2] = [
    platform_device_id {
        name: [
            b'a' as c_char,
            b'v' as c_char,
            b's' as c_char,
            b'_' as c_char,
            b'm' as c_char,
            b'a' as c_char,
            b'x' as c_char,
            b'9' as c_char,
            b'8' as c_char,
            b'3' as c_char,
            b'7' as c_char,
            b'3' as c_char,
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
// MODULE_DEVICE_TABLE(platform, avs_max98373_driver_ids);

static mut avs_max98373_driver: platform_driver = platform_driver {
    probe: Some(avs_max98373_probe),
    driver: device_driver {
        name: b"avs_max98373\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops },
    },
    id_table: avs_max98373_driver_ids.as_ptr(),
};

// module_platform_driver(avs_max98373_driver)
// MODULE_DESCRIPTION("Intel max98373 machine driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
