/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license. When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2021 Advanced Micro Devices, Inc. All rights reserved.
 *
 * Author: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
 */

use core::ffi::c_void;
use std::os::raw::{c_int, c_uint};

/*
 * C header dependencies:
 * <sound/core.h>, <sound/jack.h>, <sound/pcm_params.h>,
 * <sound/soc-dapm.h>, <linux/input.h>, <linux/module.h>,
 * <sound/soc.h>, and "acp_common.h".
 */

pub const TDM_CHANNELS: c_int = 8;

/* List of DMI quirks - check acp-mach-common.c for usage. */
pub const QUIRK_TDM_MODE_ENABLE: c_int = 1;
pub const QUIRK_REMAP_DMIC_BT: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum be_id {
    HEADSET_BE_ID = 0,
    AMP_BE_ID = 1,
    DMIC_BE_ID = 2,
    BT_BE_ID = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cpu_endpoints {
    NONE = 0,
    I2S_HS = 1,
    I2S_SP = 2,
    I2S_BT = 3,
    DMIC = 4,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum codec_endpoints {
    DUMMY = 0,
    RT5682 = 1,
    RT1019 = 2,
    MAX98360A = 3,
    RT5682S = 4,
    NAU8825 = 5,
    NAU8821 = 6,
    MAX98388 = 7,
    ES83XX = 8,
}

#[repr(C)]
pub struct acp_mach_ops {
    pub probe: Option<unsafe extern "C" fn(card: *mut snd_soc_card) -> c_int>,
    pub configure_link: Option<
        unsafe extern "C" fn(
            card: *mut snd_soc_card,
            dai_link: *mut snd_soc_dai_link,
        ) -> c_int,
    >,
    pub configure_widgets: Option<unsafe extern "C" fn(card: *mut snd_soc_card) -> c_int>,
    pub suspend_pre: Option<unsafe extern "C" fn(card: *mut snd_soc_card) -> c_int>,
    pub resume_post: Option<unsafe extern "C" fn(card: *mut snd_soc_card) -> c_int>,
}

#[repr(C)]
pub struct acp_card_drvdata {
    pub hs_cpu_id: c_uint,
    pub amp_cpu_id: c_uint,
    pub bt_cpu_id: c_uint,
    pub dmic_cpu_id: c_uint,
    pub hs_codec_id: c_uint,
    pub amp_codec_id: c_uint,
    pub bt_codec_id: c_uint,
    pub dmic_codec_id: c_uint,
    pub dai_fmt: c_uint,
    pub acp_rev: c_uint,
    pub wclk: *mut clk,
    pub bclk: *mut clk,
    pub ops: acp_mach_ops,
    pub acpi_mach: *mut snd_soc_acpi_mach,
    pub mach_priv: *mut c_void,
    pub soc_mclk: bool,
    pub tdm_mode: bool,
}

unsafe extern "C" {
    pub fn acp_sofdsp_dai_links_create(card: *mut snd_soc_card) -> c_int;
    pub fn acp_legacy_dai_links_create(card: *mut snd_soc_card) -> c_int;
    pub static acp_quirk_table: [dmi_system_id; 0usize];
}

#[inline]
pub unsafe fn acp_get_drvdata(card: *mut snd_soc_card) -> *mut acp_card_drvdata {
    unsafe { (*card).drvdata as *mut acp_card_drvdata }
}

#[inline]
pub unsafe fn acp_ops_probe(card: *mut snd_soc_card) -> c_int {
    let mut ret: c_int = 1;
    let priv_: *mut acp_card_drvdata = unsafe { acp_get_drvdata(card) };

    if let Some(probe) = unsafe { (*priv_).ops.probe } {
        ret = unsafe { probe(card) };
    }
    ret
}

#[inline]
pub unsafe fn acp_ops_configure_link(
    card: *mut snd_soc_card,
    dai_link: *mut snd_soc_dai_link,
) -> c_int {
    let mut ret: c_int = 1;
    let priv_: *mut acp_card_drvdata = unsafe { acp_get_drvdata(card) };

    if let Some(configure_link) = unsafe { (*priv_).ops.configure_link } {
        ret = unsafe { configure_link(card, dai_link) };
    }
    ret
}

#[inline]
pub unsafe fn acp_ops_configure_widgets(card: *mut snd_soc_card) -> c_int {
    let mut ret: c_int = 1;
    let priv_: *mut acp_card_drvdata = unsafe { acp_get_drvdata(card) };

    if let Some(configure_widgets) = unsafe { (*priv_).ops.configure_widgets } {
        ret = unsafe { configure_widgets(card) };
    }
    ret
}

#[inline]
pub unsafe fn acp_ops_suspend_pre(card: *mut snd_soc_card) -> c_int {
    let mut ret: c_int = 1;
    let priv_: *mut acp_card_drvdata = unsafe { acp_get_drvdata(card) };

    if let Some(suspend_pre) = unsafe { (*priv_).ops.suspend_pre } {
        ret = unsafe { suspend_pre(card) };
    }
    ret
}

#[inline]
pub unsafe fn acp_ops_resume_post(card: *mut snd_soc_card) -> c_int {
    let mut ret: c_int = 1;
    let priv_: *mut acp_card_drvdata = unsafe { acp_get_drvdata(card) };

    if let Some(resume_post) = unsafe { (*priv_).ops.resume_post } {
        ret = unsafe { resume_post(card) };
    }
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
