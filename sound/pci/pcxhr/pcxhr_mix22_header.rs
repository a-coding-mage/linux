/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Driver for Digigram pcxhr compatible soundcards
 *
 * low level interface with interrupt ans message handling
 *
 * Copyright (c) 2004 by Digigram <alsa@digigram.com>
 */

use core::ffi::{c_int, c_uchar, c_uint};

/* Opaque C declarations supplied by the surrounding driver sources. */
#[repr(C)]
pub struct pcxhr_mgr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcxhr {
    _private: [u8; 0],
}

/* enum pcxhr_clock_type is supplied by another header in the original driver. */
pub type pcxhr_clock_type = c_uint;

unsafe extern "C" {
    pub fn hr222_sub_init(mgr: *mut pcxhr_mgr) -> c_int;
    pub fn hr222_sub_set_clock(
        mgr: *mut pcxhr_mgr,
        rate: c_uint,
        changed: *mut c_int,
    ) -> c_int;
    pub fn hr222_get_external_clock(
        mgr: *mut pcxhr_mgr,
        clock_type: pcxhr_clock_type,
        sample_rate: *mut c_int,
    ) -> c_int;

    pub fn hr222_read_gpio(mgr: *mut pcxhr_mgr, is_gpi: c_int, value: *mut c_int) -> c_int;
    pub fn hr222_write_gpo(mgr: *mut pcxhr_mgr, value: c_int) -> c_int;
    pub fn hr222_manage_timecode(mgr: *mut pcxhr_mgr, enable: c_int) -> c_int;
}

pub const HR222_LINE_PLAYBACK_LEVEL_MIN: c_int = 0; /* -25.5 dB */
pub const HR222_LINE_PLAYBACK_ZERO_LEVEL: c_int = 51; /* 0.0 dB */
pub const HR222_LINE_PLAYBACK_LEVEL_MAX: c_int = 99; /* +24.0 dB */

pub const HR222_LINE_CAPTURE_LEVEL_MIN: c_int = 0; /* -111.5 dB */
pub const HR222_LINE_CAPTURE_ZERO_LEVEL: c_int = 223; /* 0.0 dB */
pub const HR222_LINE_CAPTURE_LEVEL_MAX: c_int = 255; /* +16 dB */
pub const HR222_MICRO_CAPTURE_LEVEL_MIN: c_int = 0; /* -98.5 dB */
pub const HR222_MICRO_CAPTURE_LEVEL_MAX: c_int = 210; /* +6.5 dB */

unsafe extern "C" {
    pub fn hr222_update_analog_audio_level(
        chip: *mut snd_pcxhr,
        is_capture: c_int,
        channel: c_int,
    ) -> c_int;
    pub fn hr222_set_audio_source(chip: *mut snd_pcxhr) -> c_int;
    pub fn hr222_iec958_capture_byte(
        chip: *mut snd_pcxhr,
        aes_idx: c_int,
        aes_bits: *mut c_uchar,
    ) -> c_int;
    pub fn hr222_iec958_update_byte(
        chip: *mut snd_pcxhr,
        aes_idx: c_int,
        aes_bits: c_uchar,
    ) -> c_int;

    pub fn hr222_add_mic_controls(chip: *mut snd_pcxhr) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
