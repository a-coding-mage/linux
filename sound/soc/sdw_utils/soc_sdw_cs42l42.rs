// SPDX-License-Identifier: GPL-2.0-only
// This file incorporates work covered by the following copyright notice:
// Copyright (c) 2023 Intel Corporation
// Copyright (c) 2024 Advanced Micro Devices, Inc.
/*
 *  soc_sdw_cs42l42 - Helpers to handle CS42L42 from generic machine driver
 */

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

// Dependencies from Linux/ALSA headers:
// linux/device.h, linux/errno.h, linux/input.h, linux/soundwire/sdw.h,
// linux/soundwire/sdw_type.h, sound/control.h, sound/soc.h, sound/soc-acpi.h,
// sound/soc-dapm.h, sound/jack.h, sound/soc_sdw_utils.h

const GFP_KERNEL: c_int = 0;
const ENOMEM: c_int = 12;

const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SND_JACK_HEADSET: c_int = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_JACK_BTN_0: c_int = 0x4000;
const SND_JACK_BTN_1: c_int = 0x2000;
const SND_JACK_BTN_2: c_int = 0x1000;
const SND_JACK_BTN_3: c_int = 0x0800;

const KEY_PLAYPAUSE: c_int = 164;
const KEY_VOLUMEUP: c_int = 115;
const KEY_VOLUMEDOWN: c_int = 114;
const KEY_VOICECOMMAND: c_int = 0x246;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub components: *mut c_char,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

#[repr(C)]
pub struct snd_soc_jack {
    pub jack: *mut snd_jack,
}

#[repr(C)]
pub struct asoc_sdw_mc_private {
    pub sdw_headset: snd_soc_jack,
}

unsafe extern "C" {
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut asoc_sdw_mc_private;
    fn devm_kasprintf(dev: *mut device, gfp: c_int, fmt: *const c_char, ...) -> *mut c_char;
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

static CS42L42_MAP: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        // HP jack connectors - unknown if we have jack detection
        sink: c"Headphone".as_ptr(),
        control: ptr::null(),
        source: c"cs42l42 HP".as_ptr(),
    },
    snd_soc_dapm_route {
        // other jacks
        sink: c"cs42l42 HS".as_ptr(),
        control: ptr::null(),
        source: c"Headset Mic".as_ptr(),
    },
];

static mut CS42L42_JACK_PINS: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headphone".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_cs42l42_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    dai: *mut snd_soc_dai,
) -> c_int {
    let card: *mut snd_soc_card = unsafe { (*rtd).card };
    let dapm: *mut snd_soc_dapm_context = unsafe { snd_soc_card_to_dapm(card) };
    let ctx: *mut asoc_sdw_mc_private = unsafe { snd_soc_card_get_drvdata(card) };
    let component: *mut snd_soc_component;
    let jack: *mut snd_soc_jack;
    let mut ret: c_int;

    component = unsafe { (*dai).component };
    unsafe {
        (*card).components = devm_kasprintf(
            (*card).dev,
            GFP_KERNEL,
            c"%s hs:cs42l42".as_ptr(),
            (*card).components,
        );
    }
    if unsafe { (*card).components.is_null() } {
        return -ENOMEM;
    }

    ret = unsafe {
        snd_soc_dapm_add_routes(dapm, CS42L42_MAP.as_ptr(), CS42L42_MAP.len() as c_int)
    };

    if ret != 0 {
        unsafe {
            dev_err(
                (*card).dev,
                c"cs42l42 map addition failed: %d\n".as_ptr(),
                ret,
            );
        }
        return ret;
    }

    ret = unsafe {
        snd_soc_card_jack_new_pins(
            (*rtd).card,
            c"Headset Jack".as_ptr(),
            SND_JACK_HEADSET
                | SND_JACK_BTN_0
                | SND_JACK_BTN_1
                | SND_JACK_BTN_2
                | SND_JACK_BTN_3,
            &mut (*ctx).sdw_headset,
            CS42L42_JACK_PINS.as_mut_ptr(),
            CS42L42_JACK_PINS.len() as c_int,
        )
    };
    if ret != 0 {
        unsafe {
            dev_err(
                (*(*rtd).card).dev,
                c"Headset Jack creation failed: %d\n".as_ptr(),
                ret,
            );
        }
        return ret;
    }

    jack = unsafe { &mut (*ctx).sdw_headset };

    unsafe {
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOLUMEUP);
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEDOWN);
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOICECOMMAND);
    }

    ret = unsafe { snd_soc_component_set_jack(component, jack, ptr::null_mut()) };

    if ret != 0 {
        unsafe {
            dev_err(
                (*(*rtd).card).dev,
                c"Headset Jack call-back failed: %d\n".as_ptr(),
                ret,
            );
        }
    }

    ret
}

// EXPORT_SYMBOL_NS(asoc_sdw_cs42l42_rtd_init, "SND_SOC_SDW_UTILS");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
