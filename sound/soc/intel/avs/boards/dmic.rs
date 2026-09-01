// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

type c_uint = u32;
type bool_ = bool;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
    pub id: c_int,
    pub nonatomic: c_uint,
    pub no_pcm: c_uint,
    pub capture_only: c_uint,
    pub ignore_suspend: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub sname: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub driver_name: *const c_char,
    pub long_name: *const c_char,
    pub dev: *mut device,
    pub owner: *mut module,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub fully_routed: bool_,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub pdata: *mut avs_mach_pdata,
}

#[repr(C)]
pub struct avs_mach_pdata {
    pub codec_name: *const c_char,
    pub obsolete_card_names: bool_,
}

#[repr(C)]
pub struct platform_device_id {
    pub name: [c_char; 20],
    pub driver_data: usize,
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
    static mut THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn devm_kcalloc(
        dev: *mut device,
        n: usize,
        size: usize,
        flags: c_uint,
    ) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kstrdup(dev: *mut device, s: *const c_char, flags: c_uint) -> *mut c_char;
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn dev_name(dev: *const device) -> *const c_char;
    fn dev_get_platdata(dev: *const device) -> *mut c_void;
    fn devm_snd_soc_register_deferrable_card(
        dev: *mut device,
        card: *mut snd_soc_card,
    ) -> c_int;
}

// SND_SOC_DAILINK_DEF(dmic_pin, DAILINK_COMP_ARRAY(COMP_CPU("DMIC Pin")));
static mut dmic_pin_component: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"DMIC Pin\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
}];
static mut dmic_pin: *mut snd_soc_dai_link_component = unsafe { dmic_pin_component.as_mut_ptr() };

// SND_SOC_DAILINK_DEF(dmic_wov_pin, DAILINK_COMP_ARRAY(COMP_CPU("DMIC WoV Pin")));
static mut dmic_wov_pin_component: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component {
        name: b"DMIC WoV Pin\0".as_ptr() as *const c_char,
        dai_name: ptr::null(),
    }];
static mut dmic_wov_pin: *mut snd_soc_dai_link_component =
    unsafe { dmic_wov_pin_component.as_mut_ptr() };

static card_widgets: [snd_soc_dapm_widget; 1] = [snd_soc_dapm_widget {
    id: 0,
    name: b"SoC DMIC\0".as_ptr() as *const c_char,
    sname: ptr::null(),
}];

static card_routes: [snd_soc_dapm_route; 1] = [snd_soc_dapm_route {
    sink: b"DMic\0".as_ptr() as *const c_char,
    control: ptr::null(),
    source: b"SoC DMIC\0".as_ptr() as *const c_char,
}];

unsafe extern "C" fn avs_create_dai_links(
    dev: *mut device,
    codec_name: *const c_char,
    links: *mut *mut snd_soc_dai_link,
    num_links: *mut c_int,
) -> c_int {
    let platform: *mut snd_soc_dai_link_component;
    let dl: *mut snd_soc_dai_link;
    let num_dl: c_int = 2;

    dl = devm_kcalloc(
        dev,
        num_dl as usize,
        size_of::<snd_soc_dai_link>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link;
    platform = devm_kzalloc(dev, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL)
        as *mut snd_soc_dai_link_component;
    if dl.is_null() || platform.is_null() {
        return -ENOMEM;
    }

    (*dl).codecs = devm_kzalloc(
        dev,
        size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if (*dl).codecs.is_null() {
        return -ENOMEM;
    }

    (*(*dl).codecs).name = devm_kstrdup(dev, codec_name, GFP_KERNEL);
    (*(*dl).codecs).dai_name =
        devm_kasprintf(dev, GFP_KERNEL, b"dmic-hifi\0".as_ptr() as *const c_char);
    if (*(*dl).codecs).name.is_null() || (*(*dl).codecs).dai_name.is_null() {
        return -ENOMEM;
    }

    (*platform).name = dev_name(dev);
    (*dl.add(0)).num_cpus = 1;
    (*dl.add(0)).num_codecs = 1;
    (*dl.add(0)).platforms = platform;
    (*dl.add(0)).num_platforms = 1;
    (*dl.add(0)).nonatomic = 1;
    (*dl.add(0)).no_pcm = 1;
    (*dl.add(0)).capture_only = 1;
    ptr::copy_nonoverlapping(dl.add(0), dl.add(1), 1);

    (*dl.add(0)).name = b"DMIC\0".as_ptr() as *const c_char;
    (*dl.add(0)).cpus = dmic_pin;
    (*dl.add(0)).id = 0;
    (*dl.add(1)).name = b"DMIC WoV\0".as_ptr() as *const c_char;
    (*dl.add(1)).cpus = dmic_wov_pin;
    (*dl.add(1)).id = 1;
    (*dl.add(1)).ignore_suspend = 1;

    *links = dl;
    *num_links = num_dl;
    0
}

unsafe extern "C" fn avs_dmic_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let mach: *mut snd_soc_acpi_mach;
    let pdata: *mut avs_mach_pdata;
    let card: *mut snd_soc_card;
    let ret: c_int;

    mach = dev_get_platdata(dev) as *mut snd_soc_acpi_mach;
    pdata = (*mach).pdata;

    card = devm_kzalloc(dev, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if card.is_null() {
        return -ENOMEM;
    }

    ret = avs_create_dai_links(
        dev,
        (*pdata).codec_name,
        &mut (*card).dai_link,
        &mut (*card).num_links,
    );
    if ret != 0 {
        return ret;
    }

    if (*pdata).obsolete_card_names {
        (*card).name = b"avs_dmic\0".as_ptr() as *const c_char;
    } else {
        (*card).driver_name = b"avs_dmic\0".as_ptr() as *const c_char;
        (*card).name = b"AVS DMIC\0".as_ptr() as *const c_char;
        (*card).long_name = (*card).name;
    }
    (*card).dev = dev;
    (*card).owner = THIS_MODULE;
    (*card).dapm_widgets = card_widgets.as_ptr();
    (*card).num_dapm_widgets = card_widgets.len() as c_int;
    (*card).dapm_routes = card_routes.as_ptr();
    (*card).num_dapm_routes = card_routes.len() as c_int;
    (*card).fully_routed = true;

    devm_snd_soc_register_deferrable_card(dev, card)
}

static avs_dmic_driver_ids: [platform_device_id; 2] = [
    platform_device_id {
        name: [
            b'a' as c_char,
            b'v' as c_char,
            b's' as c_char,
            b'_' as c_char,
            b'd' as c_char,
            b'm' as c_char,
            b'i' as c_char,
            b'c' as c_char,
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
            0,
        ],
        driver_data: 0,
    },
    platform_device_id {
        name: [0; 20],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(platform, avs_dmic_driver_ids);

static mut avs_dmic_driver: platform_driver = platform_driver {
    probe: Some(avs_dmic_probe),
    driver: device_driver {
        name: b"avs_dmic\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops as *const dev_pm_ops },
    },
    id_table: avs_dmic_driver_ids.as_ptr(),
};

// module_platform_driver(avs_dmic_driver);
// MODULE_DESCRIPTION("Intel DMIC machine driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
