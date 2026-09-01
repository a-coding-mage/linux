// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//

// C dependencies:
// #include <linux/module.h>
// #include <sound/sof.h>
// #include "sof-audio.h"
// #include "sof-priv.h"

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_int = 0;

#[repr(C)]
pub struct device {
    pub parent: *mut device,
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *mut c_char,
    pub stream_name: *mut c_char,
    pub cpus: *mut snd_soc_dai_link_component,
    pub codecs: *mut snd_soc_dai_link_component,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_cpus: c_int,
    pub num_codecs: c_int,
    pub num_platforms: c_int,
    pub id: c_int,
    pub no_pcm: c_int,
    pub playback_only: bool,
    pub capture_only: bool,
    pub be_hw_params_fixup: Option<unsafe extern "C" fn() -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub mach_params: snd_soc_acpi_mach_params,
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
    pub num_dai_drivers: u32,
    pub dai_drivers: *mut snd_soc_dai_driver,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const c_void,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static mut snd_soc_dummy_dlc: snd_soc_dai_link_component;
    static snd_soc_pm_ops: c_void;

    fn devm_kcalloc(
        dev: *mut device,
        n: usize,
        size: usize,
        flags: c_int,
    ) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, flags: c_int, fmt: *const c_char, ...) -> *mut c_char;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn snd_soc_card_set_topology_name(card: *mut snd_soc_card, name: *const c_char);
    fn sof_pcm_dai_link_fixup() -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

static mut sof_nocodec_card: snd_soc_card = snd_soc_card {
    name: c"nocodec".as_ptr(), /* the sof- prefix is added by the core */
    owner: unsafe { THIS_MODULE },
    dev: ptr::null_mut(),
    dai_link: ptr::null_mut(),
    num_links: 0,
};

unsafe extern "C" fn sof_nocodec_bes_setup(
    dev: *mut device,
    drv: *mut snd_soc_dai_driver,
    links: *mut snd_soc_dai_link,
    link_num: c_int,
) -> c_int {
    let card: *mut snd_soc_card = &raw mut sof_nocodec_card;
    let mut dlc: *mut snd_soc_dai_link_component;
    let mut i: c_int;

    if drv.is_null() || links.is_null() {
        return -EINVAL;
    }

    /* set up BE dai_links */
    i = 0;
    while i < link_num {
        dlc = devm_kcalloc(dev, 2, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL)
            as *mut snd_soc_dai_link_component;
        if dlc.is_null() {
            return -ENOMEM;
        }

        (*links.offset(i as isize)).name =
            devm_kasprintf(dev, GFP_KERNEL, c"NoCodec-%d".as_ptr(), i);
        if (*links.offset(i as isize)).name.is_null() {
            return -ENOMEM;
        }

        (*links.offset(i as isize)).stream_name = (*links.offset(i as isize)).name;

        (*links.offset(i as isize)).cpus = dlc.offset(0);
        (*links.offset(i as isize)).codecs = &raw mut snd_soc_dummy_dlc;
        (*links.offset(i as isize)).platforms = dlc.offset(1);

        (*links.offset(i as isize)).num_cpus = 1;
        (*links.offset(i as isize)).num_codecs = 1;
        (*links.offset(i as isize)).num_platforms = 1;

        (*links.offset(i as isize)).id = i;
        (*links.offset(i as isize)).no_pcm = 1;
        (*(*links.offset(i as isize)).cpus).dai_name = (*drv.offset(i as isize)).name;
        (*(*links.offset(i as isize)).platforms).name = dev_name((*dev).parent);

        (*links.offset(i as isize)).playback_only =
            (*drv.offset(i as isize)).playback.channels_min != 0
                && (*drv.offset(i as isize)).capture.channels_min == 0;
        (*links.offset(i as isize)).capture_only =
            (*drv.offset(i as isize)).playback.channels_min == 0
                && (*drv.offset(i as isize)).capture.channels_min != 0;

        (*links.offset(i as isize)).be_hw_params_fixup = Some(sof_pcm_dai_link_fixup);

        i += 1;
    }

    (*card).dai_link = links;
    (*card).num_links = link_num;

    0
}

unsafe extern "C" fn sof_nocodec_setup(
    dev: *mut device,
    mach: *mut snd_soc_acpi_mach,
) -> c_int {
    let num_dai_drivers: u32 = (*mach).mach_params.num_dai_drivers;
    let dai_drivers: *mut snd_soc_dai_driver = (*mach).mach_params.dai_drivers;
    let links: *mut snd_soc_dai_link;

    /* create dummy BE dai_links */
    links = devm_kcalloc(
        dev,
        num_dai_drivers as usize,
        size_of::<snd_soc_dai_link>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link;
    if links.is_null() {
        return -ENOMEM;
    }

    sof_nocodec_bes_setup(dev, dai_drivers, links, num_dai_drivers as c_int)
}

unsafe extern "C" fn sof_nocodec_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card = &raw mut sof_nocodec_card;
    let mach: *mut snd_soc_acpi_mach;
    let ret: c_int;

    (*card).dev = &raw mut (*pdev).dev;
    mach = (*pdev).dev.platform_data as *mut snd_soc_acpi_mach;

    snd_soc_card_set_topology_name(card, c"sof".as_ptr());

    ret = sof_nocodec_setup((*card).dev, mach);
    if ret < 0 {
        return ret;
    }

    devm_snd_soc_register_card(&raw mut (*pdev).dev, card)
}

static mut sof_nocodec_audio: platform_driver = platform_driver {
    probe: Some(sof_nocodec_probe),
    driver: device_driver {
        name: c"sof-nocodec".as_ptr(),
        pm: unsafe { &raw const snd_soc_pm_ops },
    },
};

// module_platform_driver(sof_nocodec_audio)
// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("ASoC sof nocodec");
// MODULE_AUTHOR("Liam Girdwood");
// MODULE_ALIAS("platform:sof-nocodec");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
