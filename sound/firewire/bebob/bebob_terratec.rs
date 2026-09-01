// SPDX-License-Identifier: GPL-2.0-only
/*
 * bebob_terratec.c - a part of driver for BeBoB based devices
 *
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

// Rust translation of definitions depending on "./bebob.h".

use core::ptr;

extern "C" {
    fn avc_audio_get_selector(
        unit: *mut core::ffi::c_void,
        subunit_id: u32,
        fb_id: u32,
        value: *mut u32,
    ) -> i32;
    fn snd_bebob_stream_get_rate(bebob: *mut snd_bebob, rate: *mut u32) -> i32;
    fn snd_bebob_stream_set_rate(bebob: *mut snd_bebob, rate: u32) -> i32;
}

#[repr(C)]
pub struct snd_bebob {
    pub unit: *mut core::ffi::c_void,
}

pub type snd_bebob_clock_type = u32;

pub const SND_BEBOB_CLOCK_TYPE_INTERNAL: snd_bebob_clock_type = 0;
pub const SND_BEBOB_CLOCK_TYPE_EXTERNAL: snd_bebob_clock_type = 1;

#[repr(C)]
pub struct snd_bebob_rate_spec {
    pub get: Option<unsafe extern "C" fn(*mut snd_bebob, *mut u32) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut snd_bebob, u32) -> i32>,
}

#[repr(C)]
pub struct snd_bebob_clock_spec {
    pub num: usize,
    pub types: *const snd_bebob_clock_type,
    pub get: Option<unsafe extern "C" fn(*mut snd_bebob, *mut u32) -> i32>,
}

#[repr(C)]
pub struct snd_bebob_spec {
    pub clock: *const snd_bebob_clock_spec,
    pub rate: *const snd_bebob_rate_spec,
    pub meter: *const core::ffi::c_void,
}

static phase88_rack_clk_src_types: [snd_bebob_clock_type; 3] = [
    SND_BEBOB_CLOCK_TYPE_INTERNAL,
    SND_BEBOB_CLOCK_TYPE_EXTERNAL, /* S/PDIF */
    SND_BEBOB_CLOCK_TYPE_EXTERNAL, /* Word Clock */
];

unsafe extern "C" fn phase88_rack_clk_src_get(bebob: *mut snd_bebob, id: *mut u32) -> i32 {
    let mut enable_ext: u32 = 0;
    let mut enable_word: u32 = 0;
    let mut err: i32;

    err = avc_audio_get_selector((*bebob).unit, 0, 9, &mut enable_ext);
    if err < 0 {
        return err;
    }
    err = avc_audio_get_selector((*bebob).unit, 0, 8, &mut enable_word);
    if err < 0 {
        return err;
    }

    if enable_ext == 0 {
        *id = 0;
    } else if enable_word == 0 {
        *id = 1;
    } else {
        *id = 2;
    }

    err
}

static phase_series_rate_spec: snd_bebob_rate_spec = snd_bebob_rate_spec {
    get: Some(snd_bebob_stream_get_rate),
    set: Some(snd_bebob_stream_set_rate),
};

/* PHASE 88 Rack FW */
static phase88_rack_clk: snd_bebob_clock_spec = snd_bebob_clock_spec {
    num: phase88_rack_clk_src_types.len(),
    types: phase88_rack_clk_src_types.as_ptr(),
    get: Some(phase88_rack_clk_src_get),
};

pub static phase88_rack_spec: snd_bebob_spec = snd_bebob_spec {
    clock: &phase88_rack_clk,
    rate: &phase_series_rate_spec,
    meter: ptr::null(),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
