// SPDX-License-Identifier: GPL-2.0-only
//
// Line 6 Linux USB driver
//
// Copyright (C) 2004-2010 Markus Grabner (line6@grabner-graz.at)

// Dependencies: sound/pcm.h (snd_pcm_ops), driver.h, pcm.h (snd_line6_pcm)

use std::ffi::c_char;
use std::ffi::c_int;

// Forward declarations of types from other modules
// snd_line6_pcm is defined in pcm.h
// snd_pcm_ops is defined in sound/pcm.h
pub enum snd_line6_pcm {}
pub struct snd_pcm_ops;

extern "C" {
    pub static snd_line6_capture_ops: snd_pcm_ops;

    pub fn line6_capture_copy(
        line6pcm: *mut snd_line6_pcm,
        fbuf: *mut c_char,
        fsize: c_int,
    );

    pub fn line6_capture_check_period(line6pcm: *mut snd_line6_pcm, length: c_int);

    pub fn line6_create_audio_in_urbs(line6pcm: *mut snd_line6_pcm) -> c_int;

    pub fn line6_submit_audio_in_all_urbs(line6pcm: *mut snd_line6_pcm) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
