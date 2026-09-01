// SPDX-License-Identifier: GPL-2.0-only
// Based on sof_sdw_cs42l45.c
// This file incorporates work covered by the following copyright notice:
// Copyright (c) 2023 Intel Corporation
// Copyright (c) 2024 Advanced Micro Devices, Inc.

/*
 *  soc_sdw_cs47l47 - Helpers to handle CS47L47 from generic machine driver
 */
// C dependencies:
// #include <linux/device.h>
// #include <linux/errno.h>
// #include <sound/jack.h>
// #include <sound/soc.h>
// #include <sound/soc-card.h>
// #include <sound/soc-component.h>
// #include <sound/soc-dai.h>
// #include <sound/soc_sdw_utils.h>

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SND_JACK_LINEOUT: c_int = 0x2000;
const SND_JACK_MECHANICAL: c_int = 0x0008;
const SND_JACK_HEADSET: c_int = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;

type c_uint = u32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub component: *mut snd_soc_component,
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
pub struct asoc_sdw_mc_private {
    pub sdw_headset: snd_soc_jack,
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

unsafe extern "C" {
    fn devm_kasprintf(dev: *mut device, gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn snd_soc_rtd_to_codec(
        rtd: *mut snd_soc_pcm_runtime,
        n: c_int,
    ) -> *mut snd_soc_dai_link_component;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

static mut soc_jack_pins: [snd_soc_jack_pin; 4] = [
    snd_soc_jack_pin {
        pin: c"cs47l47 OT 43 Headphone".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c"cs47l47 OT 45 Headset".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c"cs47l47 IT 31 Microphone".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
    snd_soc_jack_pin {
        pin: c"cs47l47 IT 33 Headset".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
];

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_cs47l47_hs_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let card: *mut snd_soc_card = unsafe { (*rtd).card };
    let component: *mut snd_soc_component =
        unsafe { (*snd_soc_rtd_to_codec(rtd, 0)).component };
    let ctx: *mut asoc_sdw_mc_private =
        unsafe { snd_soc_card_get_drvdata(card) as *mut asoc_sdw_mc_private };
    let jack: *mut snd_soc_jack = unsafe { &mut (*ctx).sdw_headset };
    let mut ret: c_int;

    unsafe {
        (*card).components = devm_kasprintf(
            (*card).dev,
            GFP_KERNEL,
            c"%s hs:cs47l47".as_ptr(),
            (*card).components,
        );
        if (*card).components.is_null() {
            return -ENOMEM;
        }

        ret = snd_soc_card_jack_new_pins(
            card,
            c"Jack".as_ptr(),
            SND_JACK_MECHANICAL | SND_JACK_HEADSET | SND_JACK_LINEOUT,
            jack,
            soc_jack_pins.as_mut_ptr(),
            soc_jack_pins.len() as c_uint,
        );
        if ret != 0 {
            dev_err((*card).dev, c"Failed to create jack: %d\n".as_ptr(), ret);
            return ret;
        }

        ret = snd_soc_component_set_jack(component, jack, ptr::null_mut());
        if ret != 0 {
            dev_err((*card).dev, c"Failed to register jack: %d\n".as_ptr(), ret);
            return ret;
        }
    }

    0
}

// EXPORT_SYMBOL_NS(asoc_sdw_cs47l47_hs_rtd_init, "SND_SOC_SDW_UTILS");

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_cs47l47_dmic_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let card: *mut snd_soc_card = unsafe { (*rtd).card };

    unsafe {
        (*card).components = devm_kasprintf(
            (*card).dev,
            GFP_KERNEL,
            c"%s mic:cs47l47-dmic".as_ptr(),
            (*card).components,
        );
        if (*card).components.is_null() {
            return -ENOMEM;
        }
    }

    0
}

// EXPORT_SYMBOL_NS(asoc_sdw_cs47l47_dmic_rtd_init, "SND_SOC_SDW_UTILS");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
