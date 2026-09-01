// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2022-2025 Qualcomm Innovation Center, Inc. All rights reserved.
 */

// C dependencies:
// #include <dt-bindings/sound/qcom,q6afe.h>
// #include <linux/module.h>
// #include <sound/jack.h>
// #include <sound/soc-usb.h>
// #include "usb_offload_utils.h"

use core::ffi::{c_int, c_void};

pub const EINVAL: c_int = 22;

unsafe extern "C" {
    pub static USB_RX: c_int;
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
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
pub struct snd_soc_dai {
    pub id: c_int,
    pub component: *mut snd_soc_component,
}

unsafe extern "C" {
    pub fn snd_soc_rtd_to_cpu(
        rtd: *mut snd_soc_pcm_runtime,
        n: c_int,
    ) -> *mut snd_soc_dai;
    pub fn snd_soc_rtd_to_codec(
        rtd: *mut snd_soc_pcm_runtime,
        n: c_int,
    ) -> *mut snd_soc_dai;
    pub fn snd_soc_usb_setup_offload_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
    ) -> c_int;
    pub fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qcom_snd_usb_offload_jack_setup(
    rtd: *mut snd_soc_pcm_runtime,
    jack: *mut snd_soc_jack,
    jack_setup: *mut bool,
) -> c_int {
    let cpu_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_cpu(rtd, 0) };
    let codec_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_codec(rtd, 0) };
    let mut ret: c_int = 0;

    if unsafe { (*cpu_dai).id != USB_RX } {
        return -EINVAL;
    }

    if unsafe { !*jack_setup } {
        ret = unsafe { snd_soc_usb_setup_offload_jack((*codec_dai).component, jack) };
        if ret != 0 {
            return ret;
        }
    }

    unsafe {
        *jack_setup = true;
    }

    0
}

// EXPORT_SYMBOL_GPL(qcom_snd_usb_offload_jack_setup);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qcom_snd_usb_offload_jack_remove(
    rtd: *mut snd_soc_pcm_runtime,
    jack_setup: *mut bool,
) -> c_int {
    let cpu_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_cpu(rtd, 0) };
    let codec_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_codec(rtd, 0) };
    let mut ret: c_int = 0;

    if unsafe { (*cpu_dai).id != USB_RX } {
        return -EINVAL;
    }

    if unsafe { *jack_setup } {
        ret = unsafe {
            snd_soc_component_set_jack(
                (*codec_dai).component,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            )
        };
        if ret != 0 {
            return ret;
        }
    }

    unsafe {
        *jack_setup = false;
    }

    0
}

// EXPORT_SYMBOL_GPL(qcom_snd_usb_offload_jack_remove);
// MODULE_DESCRIPTION("ASoC Q6 USB offload controls");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
