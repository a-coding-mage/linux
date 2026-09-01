// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2020 Intel Corporation

/*
 *  sof_sdw_hdmi - Helpers to handle HDMI from generic machine driver
 */

// C dependencies:
// linux/acpi.h, linux/device.h, linux/errno.h, linux/kernel.h, linux/list.h,
// linux/soundwire/sdw_intel.h, sound/soc.h, sound/soc-acpi.h, sound/jack.h,
// sof_sdw_common.h, hda_dsp_common.h

pub const EINVAL: i32 = 22;

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    _private: [u8; 0],
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
pub struct asoc_sdw_mc_private {
    pub private: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct intel_mc_ctx {
    pub hdmi: hda_dsp_hdmi_pcm,
}

#[repr(C)]
pub struct hda_dsp_hdmi_pcm {
    pub hdmi_comp: *mut snd_soc_component,
    pub idisp_codec: bool,
}

extern "C" {
    pub fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut asoc_sdw_mc_private;
    pub fn snd_soc_rtd_to_codec(
        rtd: *mut snd_soc_pcm_runtime,
        num: u32,
    ) -> *mut snd_soc_dai;
    pub fn hda_dsp_hdmi_build_controls(
        card: *mut snd_soc_card,
        component: *mut snd_soc_component,
    ) -> i32;
}

pub unsafe extern "C" fn sof_sdw_hdmi_init(rtd: *mut snd_soc_pcm_runtime) -> i32 {
    let ctx: *mut asoc_sdw_mc_private = snd_soc_card_get_drvdata((*rtd).card);
    let intel_ctx: *mut intel_mc_ctx = (*ctx).private as *mut intel_mc_ctx;
    let dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);

    (*intel_ctx).hdmi.hdmi_comp = (*dai).component;

    0
}

pub unsafe extern "C" fn sof_sdw_hdmi_card_late_probe(card: *mut snd_soc_card) -> i32 {
    let ctx: *mut asoc_sdw_mc_private = snd_soc_card_get_drvdata(card);
    let intel_ctx: *mut intel_mc_ctx = (*ctx).private as *mut intel_mc_ctx;

    if !(*intel_ctx).hdmi.idisp_codec {
        return 0;
    }

    if (*intel_ctx).hdmi.hdmi_comp.is_null() {
        return -EINVAL;
    }

    hda_dsp_hdmi_build_controls(card, (*intel_ctx).hdmi.hdmi_comp)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
