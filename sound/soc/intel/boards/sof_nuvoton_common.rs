// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file defines data structures and functions used in Machine
 * Driver for Intel platforms with Nuvoton Codecs.
 *
 * Copyright 2023 Intel Corporation.
 */

use core::ffi::{c_char, c_int, c_void};

/*
 * C includes translated as external dependencies:
 * <linux/module.h>
 * <sound/sof.h>
 * "sof_nuvoton_common.h"
 */

#[repr(C)]
pub struct snd_kcontrol_new {
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

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_int,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
}

unsafe extern "C" {
    static NAU8318_DEV0_NAME: *const c_char;
    static NAU8318_CODEC_DAI: *const c_char;

    fn SOC_DAPM_PIN_SWITCH(pin: *const c_char) -> snd_kcontrol_new;
    fn SND_SOC_DAPM_SPK(id: *const c_char, event: *const c_void) -> snd_soc_dapm_widget;

    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widget: *const snd_soc_dapm_widget,
        num: usize,
    ) -> c_int;
    fn snd_soc_add_card_controls(
        card: *mut snd_soc_card,
        controls: *const snd_kcontrol_new,
        num_controls: usize,
    ) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: usize,
    ) -> c_int;
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
}

/*
 * Nuvoton NAU8318
 */
static NAU8318_KCONTROLS: [snd_kcontrol_new; 1] = unsafe { [SOC_DAPM_PIN_SWITCH(c"Spk".as_ptr())] };

static NAU8318_WIDGETS: [snd_soc_dapm_widget; 1] =
    unsafe { [SND_SOC_DAPM_SPK(c"Spk".as_ptr(), core::ptr::null())] };

static NAU8318_ROUTES: [snd_soc_dapm_route; 1] = [snd_soc_dapm_route {
    sink: c"Spk".as_ptr(),
    control: core::ptr::null(),
    source: c"Speaker".as_ptr(),
}];

static mut NAU8318_COMPONENTS: [snd_soc_dai_link_component; 1] = unsafe {
    [snd_soc_dai_link_component {
        name: NAU8318_DEV0_NAME,
        dai_name: NAU8318_CODEC_DAI,
    }]
};

unsafe extern "C" fn nau8318_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card: *mut snd_soc_card = unsafe { (*rtd).card };
    let dapm: *mut snd_soc_dapm_context = unsafe { snd_soc_card_to_dapm(card) };
    let mut ret: c_int;

    ret = unsafe {
        snd_soc_dapm_new_controls(
            dapm,
            NAU8318_WIDGETS.as_ptr(),
            NAU8318_WIDGETS.len(),
        )
    };
    if ret != 0 {
        unsafe {
            dev_err(
                (*rtd).dev,
                c"fail to add nau8318 widgets, ret %d\n".as_ptr(),
                ret,
            );
        }
        return ret;
    }

    ret = unsafe {
        snd_soc_add_card_controls(
            card,
            NAU8318_KCONTROLS.as_ptr(),
            NAU8318_KCONTROLS.len(),
        )
    };
    if ret != 0 {
        unsafe {
            dev_err(
                (*rtd).dev,
                c"fail to add nau8318 kcontrols, ret %d\n".as_ptr(),
                ret,
            );
        }
        return ret;
    }

    ret = unsafe {
        snd_soc_dapm_add_routes(
            dapm,
            NAU8318_ROUTES.as_ptr(),
            NAU8318_ROUTES.len(),
        )
    };

    if ret != 0 {
        unsafe {
            dev_err(
                (*rtd).dev,
                c"fail to add nau8318 routes, ret %d\n".as_ptr(),
                ret,
            );
        }
        return ret;
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn nau8318_set_dai_link(link: *mut snd_soc_dai_link) {
    unsafe {
        (*link).codecs = core::ptr::addr_of_mut!(NAU8318_COMPONENTS) as *mut snd_soc_dai_link_component;
        (*link).num_codecs = NAU8318_COMPONENTS.len() as c_int;
        (*link).init = Some(nau8318_init);
    }
}

/*
 * EXPORT_SYMBOL_NS(nau8318_set_dai_link, "SND_SOC_INTEL_SOF_NUVOTON_COMMON");
 *
 * MODULE_DESCRIPTION("ASoC Intel SOF Nuvoton helpers");
 * MODULE_LICENSE("GPL");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
