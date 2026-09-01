/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright(c) 2015-18 Intel Corporation.
 */

use core::ffi::{c_int, c_uint};

pub const HDAC_ANALOG_DAI_ID: usize = 0;
pub const HDAC_DIGITAL_DAI_ID: usize = 1;
pub const HDAC_ALT_ANALOG_DAI_ID: usize = 2;
pub const HDAC_HDMI_0_DAI_ID: usize = 3;
pub const HDAC_HDMI_1_DAI_ID: usize = 4;
pub const HDAC_HDMI_2_DAI_ID: usize = 5;
pub const HDAC_HDMI_3_DAI_ID: usize = 6;
pub const HDAC_DAI_ID_NUM: usize = 7;

#[repr(C)]
pub struct hdac_hda_pcm {
    pub stream_tag: [c_int; 2],
    pub format_val: [c_uint; 2],
}

#[repr(C)]
pub struct hdac_hda_priv {
    pub codec: *mut hda_codec,
    pub pcm: [hdac_hda_pcm; HDAC_DAI_ID_NUM],
    pub need_display_power: bool,
    pub dev_index: c_int,
}

unsafe extern "C" {
    pub fn snd_soc_hdac_hda_get_ops() -> *mut hdac_ext_bus_ops;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
