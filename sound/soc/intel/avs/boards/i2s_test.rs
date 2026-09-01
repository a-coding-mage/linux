// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

type bool_ = bool;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const GFP_KERNEL: c_int = 0;

#[repr(C)]
pub struct device {
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
    pub codecs: *const snd_soc_dai_link_component,
    pub num_cpus: c_int,
    pub num_codecs: c_int,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_int,
    pub id: c_int,
    pub nonatomic: c_int,
    pub no_pcm: c_int,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub pdata: *mut avs_mach_pdata,
}

#[repr(C)]
pub struct avs_mach_pdata {
    pub obsolete_card_names: bool_,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub driver_name: *const c_char,
    pub long_name: *const c_char,
    pub dev: *mut device,
    pub owner: *mut c_void,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub fully_routed: bool_,
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
    static snd_soc_dummy_dlc: snd_soc_dai_link_component;
    static snd_soc_pm_ops: dev_pm_ops;
    static mut THIS_MODULE: *mut c_void;

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, flags: c_int, fmt: *const c_char, ...) -> *mut c_char;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn avs_mach_singular_ssp(mach: *mut snd_soc_acpi_mach) -> bool_;
    fn avs_mach_ssp_port(mach: *mut snd_soc_acpi_mach) -> c_int;
    fn avs_mach_singular_tdm(mach: *mut snd_soc_acpi_mach, ssp_port: c_int) -> bool_;
    fn avs_mach_ssp_tdm(mach: *mut snd_soc_acpi_mach, ssp_port: c_int) -> c_int;
    fn devm_snd_soc_register_deferrable_card(
        dev: *mut device,
        card: *mut snd_soc_card,
    ) -> c_int;
}

unsafe fn AVS_STRING_FMT(
    prefix: &'static [u8],
    suffix: &'static [u8],
    _ssp_port: c_int,
    _tdm_slot: c_int,
) -> *const c_char {
    // Rust cannot expand the C preprocessor macro locally without its definition.
    // This preserves the call sites' dependency on the external AVS_STRING_FMT format.
    let _ = (prefix, suffix);
    ptr::null()
}

unsafe extern "C" fn avs_create_dai_link(
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
        AVS_STRING_FMT(b"SSP\0", b"-Codec\0", ssp_port, tdm_slot),
    );
    (*dl).cpus = devm_kzalloc(
        dev,
        core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if (*dl).name.is_null() || (*dl).cpus.is_null() {
        return -ENOMEM;
    }

    (*(*dl).cpus).dai_name = devm_kasprintf(
        dev,
        GFP_KERNEL,
        AVS_STRING_FMT(b"SSP\0", b" Pin\0", ssp_port, tdm_slot),
    );
    (*dl).codecs = &snd_soc_dummy_dlc;
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
    (*dl).nonatomic = 1;
    (*dl).no_pcm = 1;

    *dai_link = dl;

    0
}

unsafe extern "C" fn avs_i2s_test_probe(pdev: *mut platform_device) -> c_int {
    let mut dai_link: *mut snd_soc_dai_link = ptr::null_mut();
    let mach: *mut snd_soc_acpi_mach;
    let pdata: *mut avs_mach_pdata;
    let card: *mut snd_soc_card;
    let dev: *mut device = &mut (*pdev).dev;
    let ssp_port: c_int;
    let tdm_slot: c_int;
    let ret: c_int;

    mach = dev_get_platdata(dev) as *mut snd_soc_acpi_mach;
    pdata = (*mach).pdata;

    if !avs_mach_singular_ssp(mach) {
        dev_err(dev, c"Invalid SSP configuration\n".as_ptr());
        return -EINVAL;
    }
    ssp_port = avs_mach_ssp_port(mach);

    if !avs_mach_singular_tdm(mach, ssp_port) {
        dev_err(dev, c"Invalid TDM configuration\n".as_ptr());
        return -EINVAL;
    }
    tdm_slot = avs_mach_ssp_tdm(mach, ssp_port);

    card = devm_kzalloc(dev, core::mem::size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if card.is_null() {
        return -ENOMEM;
    }

    if (*pdata).obsolete_card_names {
        (*card).name = devm_kasprintf(
            dev,
            GFP_KERNEL,
            AVS_STRING_FMT(b"ssp\0", b"-loopback\0", ssp_port, tdm_slot),
        );
    } else {
        (*card).driver_name = c"avs_i2s_test".as_ptr();
        (*card).name = devm_kasprintf(
            dev,
            GFP_KERNEL,
            AVS_STRING_FMT(b"AVS I2S TEST-\0", b"\0", ssp_port, tdm_slot),
        );
        (*card).long_name = (*card).name;
    }
    if (*card).name.is_null() {
        return -ENOMEM;
    }

    ret = avs_create_dai_link(dev, ssp_port, tdm_slot, &mut dai_link);
    if ret != 0 {
        dev_err(dev, c"Failed to create dai link: %d\n".as_ptr(), ret);
        return ret;
    }

    (*card).dev = dev;
    (*card).owner = THIS_MODULE;
    (*card).dai_link = dai_link;
    (*card).num_links = 1;
    (*card).fully_routed = true;

    devm_snd_soc_register_deferrable_card(dev, card)
}

static avs_i2s_test_driver_ids: [platform_device_id; 2] = [
    platform_device_id {
        name: [
            b'a' as c_char,
            b'v' as c_char,
            b's' as c_char,
            b'_' as c_char,
            b'i' as c_char,
            b'2' as c_char,
            b's' as c_char,
            b'_' as c_char,
            b't' as c_char,
            b'e' as c_char,
            b's' as c_char,
            b't' as c_char,
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

// MODULE_DEVICE_TABLE(platform, avs_i2s_test_driver_ids);

static mut avs_i2s_test_driver: platform_driver = platform_driver {
    probe: Some(avs_i2s_test_probe),
    driver: device_driver {
        name: c"avs_i2s_test".as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
    },
    id_table: avs_i2s_test_driver_ids.as_ptr(),
};

// module_platform_driver(avs_i2s_test_driver);
// MODULE_DESCRIPTION("Intel i2s test machine driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
