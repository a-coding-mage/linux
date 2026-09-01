// SPDX-License-Identifier: GPL-2.0-only
// This file incorporates work covered by the following copyright notice:
// Copyright (c) 2024 Intel Corporation.

/*
 *  soc_sdw_rt_mf_sdca
 *  - Helpers to handle RT Multifunction Codec from generic machine driver
 */

// C dependencies:
// linux/device.h, linux/errno.h, linux/soundwire/sdw.h,
// linux/soundwire/sdw_type.h, sound/control.h, sound/soc.h,
// sound/soc-acpi.h, sound/soc-dapm.h, sound/soc_sdw_utils.h

use core::ffi::{c_char, c_int};
use core::ptr;

pub const EINVAL: c_int = 22;

const CODEC_NAME_SIZE: usize = 6;

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

unsafe impl Sync for snd_soc_dapm_route {}

#[repr(C)]
pub struct codec_route_map {
    pub codec_name: *const c_char,
    pub route_map: *const snd_soc_dapm_route,
    pub route_size: usize,
}

unsafe impl Sync for codec_route_map {}

unsafe extern "C" {
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: usize,
    ) -> c_int;
}

/* dapm routes for RT-SPK will be registered dynamically */
static RT712_SPK_MAP: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: ptr::null(),
        source: c"rt712 SPOL".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: ptr::null(),
        source: c"rt712 SPOR".as_ptr(),
    },
];

static RT721_SPK_MAP: [snd_soc_dapm_route; 1] = [snd_soc_dapm_route {
    sink: c"Speaker".as_ptr(),
    control: ptr::null(),
    source: c"rt721 SPK".as_ptr(),
}];

static RT722_SPK_MAP: [snd_soc_dapm_route; 1] = [snd_soc_dapm_route {
    sink: c"Speaker".as_ptr(),
    control: ptr::null(),
    source: c"rt722 SPK".as_ptr(),
}];

/* Codec route maps array */
static CODEC_ROUTES: [codec_route_map; 3] = [
    codec_route_map {
        codec_name: c"rt712".as_ptr(),
        route_map: RT712_SPK_MAP.as_ptr(),
        route_size: RT712_SPK_MAP.len(),
    },
    codec_route_map {
        codec_name: c"rt721".as_ptr(),
        route_map: RT721_SPK_MAP.as_ptr(),
        route_size: RT721_SPK_MAP.len(),
    },
    codec_route_map {
        codec_name: c"rt722".as_ptr(),
        route_map: RT722_SPK_MAP.as_ptr(),
        route_size: RT722_SPK_MAP.len(),
    },
];

unsafe fn get_codec_route_map(codec_name: *const c_char) -> *const codec_route_map {
    for i in 0..CODEC_ROUTES.len() {
        if strcmp(CODEC_ROUTES[i].codec_name, codec_name) == 0 {
            return &CODEC_ROUTES[i];
        }
    }
    ptr::null()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_rt_mf_sdca_spk_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    dai: *mut snd_soc_dai,
) -> c_int {
    let card: *mut snd_soc_card = (*rtd).card;
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);
    let mut codec_name: [c_char; CODEC_NAME_SIZE] = [0; CODEC_NAME_SIZE];
    let ret: c_int;

    /* acquire codec name */
    snprintf(
        codec_name.as_mut_ptr(),
        CODEC_NAME_SIZE,
        c"%s".as_ptr(),
        (*dai).name,
    );

    /* acquire corresponding route map and size */
    let route_map: *const codec_route_map = get_codec_route_map(codec_name.as_ptr());

    if route_map.is_null() {
        dev_err(
            (*rtd).dev,
            c"failed to get codec name and route map\n".as_ptr(),
        );
        return -EINVAL;
    }

    /* Add routes */
    ret = snd_soc_dapm_add_routes(dapm, (*route_map).route_map, (*route_map).route_size);
    if ret != 0 {
        dev_err(
            (*rtd).dev,
            c"failed to add rt sdca spk map: %d\n".as_ptr(),
            ret,
        );
    }

    ret
}

// EXPORT_SYMBOL_NS(asoc_sdw_rt_mf_sdca_spk_rtd_init, "SND_SOC_SDW_UTILS");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
