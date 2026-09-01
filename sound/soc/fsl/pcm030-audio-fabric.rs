// SPDX-License-Identifier: GPL-2.0
//
// Phytec pcm030 driver for the PSC of the Freescale MPC52xx
// configured as AC97 interface
//
// Copyright 2008 Jon Smirl, Digispeaker
// Author: Jon Smirl <jonsmirl@gmail.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

const DRV_NAME: &[u8] = b"pcm030-audio-fabric\0";

const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const GFP_KERNEL: gfp_t = 0;

type gfp_t = u32;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_int,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_int,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_int,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut module,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dev: *mut device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: platform_driver_inner,
}

#[repr(C)]
pub struct pcm030_audio_data {
    pub card: *mut snd_soc_card,
    pub codec_device: *mut platform_device,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;

    fn of_machine_is_compatible(compatible: *const c_char) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut c_void;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn request_module(name: *const c_char) -> c_int;
    fn platform_device_alloc(name: *const c_char, id: c_int) -> *mut platform_device;
    fn platform_device_add(pdev: *mut platform_device) -> c_int;
    fn platform_device_put(pdev: *mut platform_device);
    fn platform_device_unregister(pdev: *mut platform_device);
    fn snd_soc_register_card(card: *mut snd_soc_card) -> c_int;
    fn snd_soc_unregister_card(card: *mut snd_soc_card);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
}

// SND_SOC_DAILINK_DEFS(analog,
//     DAILINK_COMP_ARRAY(COMP_CPU("mpc5200-psc-ac97.0")),
//     DAILINK_COMP_ARRAY(COMP_CODEC("wm9712-codec", "wm9712-hifi")),
//     DAILINK_COMP_ARRAY(COMP_EMPTY()));
static mut analog_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"mpc5200-psc-ac97.0\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut analog_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"wm9712-codec\0".as_ptr() as *const c_char,
    dai_name: b"wm9712-hifi\0".as_ptr() as *const c_char,
    of_node: ptr::null_mut(),
}];
static mut analog_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

// SND_SOC_DAILINK_DEFS(iec958,
//     DAILINK_COMP_ARRAY(COMP_CPU("mpc5200-psc-ac97.1")),
//     DAILINK_COMP_ARRAY(COMP_CODEC("wm9712-codec", "wm9712-aux")),
//     DAILINK_COMP_ARRAY(COMP_EMPTY()));
static mut iec958_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"mpc5200-psc-ac97.1\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut iec958_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"wm9712-codec\0".as_ptr() as *const c_char,
    dai_name: b"wm9712-aux\0".as_ptr() as *const c_char,
    of_node: ptr::null_mut(),
}];
static mut iec958_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut pcm030_fabric_dai: [snd_soc_dai_link; 2] = [
    snd_soc_dai_link {
        name: b"AC97.0\0".as_ptr() as *const c_char,
        stream_name: b"AC97 Analog\0".as_ptr() as *const c_char,
        cpus: ptr::addr_of_mut!(analog_cpus) as *mut snd_soc_dai_link_component,
        num_cpus: 1,
        codecs: ptr::addr_of_mut!(analog_codecs) as *mut snd_soc_dai_link_component,
        num_codecs: 1,
        platforms: ptr::addr_of_mut!(analog_platforms) as *mut snd_soc_dai_link_component,
        num_platforms: 1,
    },
    snd_soc_dai_link {
        name: b"AC97.1\0".as_ptr() as *const c_char,
        stream_name: b"AC97 IEC958\0".as_ptr() as *const c_char,
        cpus: ptr::addr_of_mut!(iec958_cpus) as *mut snd_soc_dai_link_component,
        num_cpus: 1,
        codecs: ptr::addr_of_mut!(iec958_codecs) as *mut snd_soc_dai_link_component,
        num_codecs: 1,
        platforms: ptr::addr_of_mut!(iec958_platforms) as *mut snd_soc_dai_link_component,
        num_platforms: 1,
    },
];

static mut pcm030_card: snd_soc_card = snd_soc_card {
    name: b"pcm030\0".as_ptr() as *const c_char,
    owner: ptr::null_mut(),
    dai_link: ptr::addr_of_mut!(pcm030_fabric_dai) as *mut snd_soc_dai_link,
    num_links: 2,
    dev: ptr::null_mut(),
};

unsafe extern "C" fn pcm030_fabric_probe(op: *mut platform_device) -> c_int {
    let np: *mut device_node = (*op).dev.of_node;
    let platform_np: *mut device_node;
    let card: *mut snd_soc_card = ptr::addr_of_mut!(pcm030_card);
    let pdata: *mut pcm030_audio_data;
    let mut dai_link: *mut snd_soc_dai_link;
    let mut ret: c_int;
    let mut i: c_int;

    if of_machine_is_compatible(b"phytec,pcm030\0".as_ptr() as *const c_char) == 0 {
        return -ENODEV;
    }

    pdata = devm_kzalloc(
        ptr::addr_of_mut!((*op).dev),
        size_of::<pcm030_audio_data>(),
        GFP_KERNEL,
    ) as *mut pcm030_audio_data;
    if pdata.is_null() {
        return -ENOMEM;
    }

    (*card).dev = ptr::addr_of_mut!((*op).dev);

    (*pdata).card = card;

    platform_np = of_parse_phandle(np, b"asoc-platform\0".as_ptr() as *const c_char, 0);
    if platform_np.is_null() {
        dev_err(
            ptr::addr_of_mut!((*op).dev),
            b"ac97 not registered\n\0".as_ptr() as *const c_char,
        );
        return -ENODEV;
    }

    i = 0;
    while i < (*card).num_links {
        dai_link = (*card).dai_link.add(i as usize);
        (*(*dai_link).platforms).of_node = platform_np;
        i += 1;
    }

    ret = request_module(b"snd-soc-wm9712\0".as_ptr() as *const c_char);
    if ret != 0 {
        dev_err(
            ptr::addr_of_mut!((*op).dev),
            b"request_module returned: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
    }

    (*pdata).codec_device = platform_device_alloc(b"wm9712-codec\0".as_ptr() as *const c_char, -1);
    if (*pdata).codec_device.is_null() {
        dev_err(
            ptr::addr_of_mut!((*op).dev),
            b"platform_device_alloc() failed\n\0".as_ptr() as *const c_char,
        );
    }

    ret = platform_device_add((*pdata).codec_device);
    if ret != 0 {
        dev_err(
            ptr::addr_of_mut!((*op).dev),
            b"platform_device_add() failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        platform_device_put((*pdata).codec_device);
    }

    ret = snd_soc_register_card(card);
    if ret != 0 {
        dev_err(
            ptr::addr_of_mut!((*op).dev),
            b"snd_soc_register_card() failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        platform_device_unregister((*pdata).codec_device);
    }

    platform_set_drvdata(op, pdata as *mut c_void);
    ret
}

unsafe extern "C" fn pcm030_fabric_remove(op: *mut platform_device) {
    let pdata: *mut pcm030_audio_data = platform_get_drvdata(op) as *mut pcm030_audio_data;

    snd_soc_unregister_card((*pdata).card);
    platform_device_unregister((*pdata).codec_device);
}

static pcm030_audio_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"phytec,pcm030-audio-fabric\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, pcm030_audio_match);

static mut pcm030_fabric_driver: platform_driver = platform_driver {
    probe: Some(pcm030_fabric_probe),
    remove: Some(pcm030_fabric_remove),
    driver: platform_driver_inner {
        name: DRV_NAME.as_ptr() as *const c_char,
        of_match_table: pcm030_audio_match.as_ptr(),
    },
};

// module_platform_driver(pcm030_fabric_driver);
// MODULE_AUTHOR("Jon Smirl <jonsmirl@gmail.com>");
// MODULE_DESCRIPTION(DRV_NAME ": mpc5200 pcm030 fabric driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
