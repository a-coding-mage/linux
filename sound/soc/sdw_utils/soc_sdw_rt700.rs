// SPDX-License-Identifier: GPL-2.0-only
// This file incorporates work covered by the following copyright notice:
// Copyright (c) 2020 Intel Corporation
// Copyright (c) 2024 Advanced Micro Devices, Inc.

/*
 *  soc_sdw_rt700 - Helpers to handle RT700 from generic machine driver
 */

/* C dependencies:
 * linux/device.h, linux/errno.h, linux/input.h, sound/control.h,
 * sound/soc.h, sound/soc-acpi.h, sound/soc-dapm.h, sound/jack.h,
 * sound/soc_sdw_utils.h
 */

use core::ffi::{c_char, c_int, c_void};
use core::ptr;
use crate::*;

unsafe extern "C" {
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, gfp: gfp_t, fmt: *const c_char, ...) -> *mut c_char;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
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
}

const ENOMEM: c_int = 12;

static RT700_MAP: [snd_soc_dapm_route; 3] = [
    /* Headphones */
    snd_soc_dapm_route {
        sink: b"Headphones\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"rt700 HP\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Speaker\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"rt700 SPK\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"rt700 MIC2\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"AMIC\0".as_ptr() as *const c_char,
    },
];

static mut RT700_JACK_PINS: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: b"Headphones\0".as_ptr() as *const c_char,
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: b"AMIC\0".as_ptr() as *const c_char,
        mask: SND_JACK_MICROPHONE,
    },
];

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_rt700_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    dai: *mut snd_soc_dai,
) -> c_int {
    let card: *mut snd_soc_card = unsafe { (*rtd).card };
    let dapm: *mut snd_soc_dapm_context = unsafe { snd_soc_card_to_dapm(card) };
    let ctx: *mut asoc_sdw_mc_private =
        unsafe { snd_soc_card_get_drvdata(card) as *mut asoc_sdw_mc_private };
    let component: *mut snd_soc_component;
    let jack: *mut snd_soc_jack;
    let mut ret: c_int;

    component = unsafe { (*dai).component };
    unsafe {
        (*card).components = devm_kasprintf(
            (*card).dev,
            GFP_KERNEL,
            b"%s hs:rt700\0".as_ptr() as *const c_char,
            (*card).components,
        );
    }
    if unsafe { (*card).components.is_null() } {
        return -ENOMEM;
    }

    ret = unsafe {
        snd_soc_dapm_add_routes(
            dapm,
            RT700_MAP.as_ptr(),
            RT700_MAP.len() as c_int,
        )
    };

    if ret != 0 {
        unsafe {
            dev_err(
                (*card).dev,
                b"rt700 map addition failed: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
        }
        return ret;
    }

    ret = unsafe {
        snd_soc_card_jack_new_pins(
            (*rtd).card,
            b"Headset Jack\0".as_ptr() as *const c_char,
            SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
            &mut (*ctx).sdw_headset,
            RT700_JACK_PINS.as_mut_ptr(),
            RT700_JACK_PINS.len() as c_int,
        )
    };
    if ret != 0 {
        unsafe {
            dev_err(
                (*(*rtd).card).dev,
                b"Headset Jack creation failed: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
        }
        return ret;
    }

    jack = unsafe { &mut (*ctx).sdw_headset };

    unsafe {
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);
    }

    ret = unsafe { snd_soc_component_set_jack(component, jack, ptr::null_mut()) };
    if ret != 0 {
        unsafe {
            dev_err(
                (*(*rtd).card).dev,
                b"Headset Jack call-back failed: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
        }
    }

    ret
}

/* EXPORT_SYMBOL_NS(asoc_sdw_rt700_rtd_init, "SND_SOC_SDW_UTILS"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
