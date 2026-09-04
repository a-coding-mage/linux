// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types)]

use std::ffi::c_char;

const MAXPACK: usize = 50;
const MAXBUFFERMS: usize = 100;
const MAXSTRIDE: usize = 3;

const SSS: usize = (((MAXPACK * MAXBUFFERMS * MAXSTRIDE + 4096) / 4096) * 4096);

#[repr(C)]
pub struct snd_usx2y_hwdep_pcm_shm {
    pub playback: [c_char; SSS],
    pub capture0x8: [c_char; SSS],
    pub capture0xA: [c_char; SSS],
    pub playback_iso_head: i32,
    pub playback_iso_start: i32,
    pub captured_iso: [CapturedIsoElement; 128],
    pub captured_iso_head: i32,
    pub captured_iso_frames: u32,
    pub capture_iso_start: i32,
}

#[repr(C)]
pub struct CapturedIsoElement {
    pub frame: i32,
    pub offset: i32,
    pub length: i32,
}

extern "C" {
    pub fn usx2y_hwdep_pcm_new(card: *mut snd_card) -> i32;
}

#[repr(C)]
pub struct snd_card;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
