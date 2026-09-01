/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Digital Beep Input Interface for HD-audio codec
 *
 * Author: Matt Ranostay <matt.ranostay@konsulko.com>
 * Copyright (c) 2008 Embedded Alley Solutions Inc
 */

/* C header dependency: <sound/hda_codec.h> */

pub const HDA_BEEP_MODE_OFF: ::core::ffi::c_int = 0;
pub const HDA_BEEP_MODE_ON: ::core::ffi::c_int = 1;

/* beep information */
#[repr(C)]
pub struct hda_beep {
    pub dev: *mut input_dev,
    pub codec: *mut hda_codec,
    pub phys: [::core::ffi::c_char; 32],
    pub tone: ::core::ffi::c_int,
    pub nid: hda_nid_t,
    /*
     * C bitfields:
     * unsigned int registered:1;
     * unsigned int enabled:1;
     * unsigned int linear_tone:1;          // linear tone for IDT/STAC codec
     * unsigned int playing:1;
     * unsigned int keep_power_at_enable:1; // set by driver
     */
    pub flags: ::core::ffi::c_uint,
    pub beep_work: work_struct, /* scheduled task for beep event */
    pub power_hook: Option<unsafe extern "C" fn(beep: *mut hda_beep, on: bool)>,
}

pub const HDA_BEEP_REGISTERED: ::core::ffi::c_uint = 1 << 0;
pub const HDA_BEEP_ENABLED: ::core::ffi::c_uint = 1 << 1;
pub const HDA_BEEP_LINEAR_TONE: ::core::ffi::c_uint = 1 << 2;
pub const HDA_BEEP_PLAYING: ::core::ffi::c_uint = 1 << 3;
pub const HDA_BEEP_KEEP_POWER_AT_ENABLE: ::core::ffi::c_uint = 1 << 4;

/* CONFIG_SND_HDA_INPUT_BEEP */
#[cfg(CONFIG_SND_HDA_INPUT_BEEP)]
unsafe extern "C" {
    pub fn snd_hda_enable_beep_device(
        codec: *mut hda_codec,
        enable: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn snd_hda_attach_beep_device(
        codec: *mut hda_codec,
        nid: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn snd_hda_detach_beep_device(codec: *mut hda_codec);
}

#[cfg(not(CONFIG_SND_HDA_INPUT_BEEP))]
#[inline]
pub unsafe fn snd_hda_attach_beep_device(
    _codec: *mut hda_codec,
    _nid: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_SND_HDA_INPUT_BEEP))]
#[inline]
pub unsafe fn snd_hda_detach_beep_device(_codec: *mut hda_codec) {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
