// SPDX-License-Identifier: GPL-2.0-only
// This file incorporates work covered by the following copyright notice:
// Copyright (c) 2020 Intel Corporation
// Copyright (c) 2024 Advanced Micro Devices, Inc.

/*
 *  soc_sdw_dmic - Helpers to handle dmic from generic machine driver
 */

// Dependencies from:
// <sound/soc.h>
// <sound/soc-acpi.h>
// <sound/soc-dapm.h>
// <sound/soc_sdw_utils.h>

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

unsafe extern "C" {
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widget: *const snd_soc_dapm_widget,
        num: c_int,
    ) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...) -> c_int;
}

static dmic_widgets: [snd_soc_dapm_widget; 1] = [
    SND_SOC_DAPM_MIC!(c"SoC DMIC".as_ptr(), core::ptr::null_mut()),
];

static dmic_map: [snd_soc_dapm_route; 1] = [
    /* digital mics */
    snd_soc_dapm_route {
        sink: c"DMic".as_ptr(),
        control: core::ptr::null(),
        source: c"SoC DMIC".as_ptr(),
    },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_dmic_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card: *mut snd_soc_card = unsafe { (*rtd).card };
    let dapm: *mut snd_soc_dapm_context = unsafe { snd_soc_card_to_dapm(card) };
    let mut ret: c_int;

    ret = unsafe {
        snd_soc_dapm_new_controls(
            dapm,
            dmic_widgets.as_ptr(),
            dmic_widgets.len() as c_int,
        )
    };
    if ret != 0 {
        unsafe {
            dev_err(
                (*card).dev,
                c"DMic widget addition failed: %d\n".as_ptr(),
                ret,
            );
        }
        /* Don't need to add routes if widget addition failed */
        return ret;
    }

    ret = unsafe {
        snd_soc_dapm_add_routes(
            dapm,
            dmic_map.as_ptr(),
            dmic_map.len() as c_int,
        )
    };

    if ret != 0 {
        unsafe {
            dev_err(
                (*card).dev,
                c"DMic map addition failed: %d\n".as_ptr(),
                ret,
            );
        }
    }

    ret
}

// EXPORT_SYMBOL_NS(asoc_sdw_dmic_init, "SND_SOC_SDW_UTILS");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
