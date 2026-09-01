// SPDX-License-Identifier: GPL-2.0-only
/*
 * bebob_yamaha.c - a part of driver for BeBoB based devices
 *
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

// Rust translation of implementation depending on declarations from "./bebob.h".

use core::ffi::{c_int, c_uint, c_void};
use core::ptr;

const EIO: c_int = 5;

#[repr(C)]
pub struct snd_bebob {
    pub unit: *mut c_void,
}

#[repr(C)]
pub enum snd_bebob_clock_type {
    SND_BEBOB_CLOCK_TYPE_INTERNAL,
    SND_BEBOB_CLOCK_TYPE_EXTERNAL,
}

#[repr(C)]
pub struct snd_bebob_clock_spec {
    pub num: c_uint,
    pub types: *const snd_bebob_clock_type,
    pub get: Option<unsafe extern "C" fn(*mut snd_bebob, *mut c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_bebob_rate_spec {
    pub get: Option<unsafe extern "C" fn(*mut snd_bebob, *mut c_uint) -> c_int>,
    pub set: Option<unsafe extern "C" fn(*mut snd_bebob, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_bebob_spec {
    pub clock: *const snd_bebob_clock_spec,
    pub rate: *const snd_bebob_rate_spec,
    pub meter: *const c_void,
}

unsafe extern "C" {
    fn avc_audio_get_selector(
        unit: *mut c_void,
        subunit_id: c_uint,
        fb_id: c_uint,
        id: *mut c_uint,
    ) -> c_int;
    fn snd_bebob_stream_get_rate(bebob: *mut snd_bebob, rate: *mut c_uint) -> c_int;
    fn snd_bebob_stream_set_rate(bebob: *mut snd_bebob, rate: c_uint) -> c_int;
}

/*
 * NOTE:
 * Yamaha GO44 is not designed to be used as stand-alone mixer. So any streams
 * must be accompanied. If changing the state, a LED on the device starts to
 * blink and its sync status is false. In this state, the device sounds nothing
 * even if streaming. To start streaming at the current sampling rate is only
 * way to recover this state. GO46 is better for stand-alone mixer.
 *
 * Both of them have a capability to change its sampling rate up to 192.0kHz.
 * At 192.0kHz, the device reports 4 PCM-in, 1 MIDI-in, 6 PCM-out, 1 MIDI-out.
 * But Yamaha's driver reduce 2 PCM-in, 1 MIDI-in, 2 PCM-out, 1 MIDI-out to use
 * 'Extended Stream Format Information Command - Single Request' in 'Additional
 * AVC commands' defined by BridgeCo.
 * This ALSA driver don't do this because a bit tiresome. Then isochronous
 * streaming with many asynchronous transactions brings sounds with noises.
 * Unfortunately current 'ffado-mixer' generated many asynchronous transaction
 * to observe device's state, mainly check cmp connection and signal format. I
 * recommend users to close ffado-mixer at 192.0kHz if mixer is needless.
 *
 * Terratec PHASE 24 FW and PHASE X24 FW are internally the same as
 * Yamaha GO 44 and GO 46. Yamaha and Terratec had cooperated for these models.
 */

static clk_src_types: [snd_bebob_clock_type; 2] = [
    snd_bebob_clock_type::SND_BEBOB_CLOCK_TYPE_INTERNAL,
    snd_bebob_clock_type::SND_BEBOB_CLOCK_TYPE_EXTERNAL, /* S/PDIF */
];

unsafe extern "C" fn clk_src_get(bebob: *mut snd_bebob, id: *mut c_uint) -> c_int {
    let err: c_int;

    err = unsafe { avc_audio_get_selector((*bebob).unit, 0, 4, id) };
    if err < 0 {
        return err;
    }

    if unsafe { *id } >= clk_src_types.len() as c_uint {
        return -EIO;
    }

    0
}

static clock_spec: snd_bebob_clock_spec = snd_bebob_clock_spec {
    num: clk_src_types.len() as c_uint,
    types: clk_src_types.as_ptr(),
    get: Some(clk_src_get),
};

static rate_spec: snd_bebob_rate_spec = snd_bebob_rate_spec {
    get: Some(snd_bebob_stream_get_rate),
    set: Some(snd_bebob_stream_set_rate),
};

pub static yamaha_terratec_spec: snd_bebob_spec = snd_bebob_spec {
    clock: &clock_spec,
    rate: &rate_spec,
    meter: ptr::null(),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
