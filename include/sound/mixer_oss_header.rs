/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *  OSS MIXER API
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

/* Translated from the CONFIG_SND_MIXER_OSS conditional section. */

pub const SNDRV_OSS_MAX_MIXERS: usize = 32;

pub struct snd_mixer_oss_file;

#[repr(C)]
pub struct snd_mixer_oss_slot {
    pub number: i32,
    /* C unsigned int stereo:1 bit-field; represented by its containing word. */
    pub stereo: u32,
    pub get_volume: Option<unsafe extern "C" fn(
        fmixer: *mut snd_mixer_oss_file,
        chn: *mut snd_mixer_oss_slot,
        left: *mut i32,
        right: *mut i32,
    ) -> i32>,
    pub put_volume: Option<unsafe extern "C" fn(
        fmixer: *mut snd_mixer_oss_file,
        chn: *mut snd_mixer_oss_slot,
        left: i32,
        right: i32,
    ) -> i32>,
    pub get_recsrc: Option<unsafe extern "C" fn(
        fmixer: *mut snd_mixer_oss_file,
        chn: *mut snd_mixer_oss_slot,
        active: *mut i32,
    ) -> i32>,
    pub put_recsrc: Option<unsafe extern "C" fn(
        fmixer: *mut snd_mixer_oss_file,
        chn: *mut snd_mixer_oss_slot,
        active: i32,
    ) -> i32>,
    pub private_value: usize,
    pub private_data: *mut core::ffi::c_void,
    pub private_free: Option<unsafe extern "C" fn(slot: *mut snd_mixer_oss_slot)>,
    pub volume: [i32; 2],
}

#[repr(C)]
pub struct snd_mixer_oss {
    pub card: *mut snd_card,
    pub id: [core::ffi::c_char; 16],
    pub name: [core::ffi::c_char; 32],
    pub slots: [snd_mixer_oss_slot; SNDRV_OSS_MAX_MIXERS], /* OSS mixer slots */
    pub mask_recsrc: u32,                                  /* exclusive recsrc mask */
    pub get_recsrc: Option<unsafe extern "C" fn(
        fmixer: *mut snd_mixer_oss_file,
        active_index: *mut u32,
    ) -> i32>,
    pub put_recsrc: Option<unsafe extern "C" fn(
        fmixer: *mut snd_mixer_oss_file,
        active_index: u32,
    ) -> i32>,
    pub private_data_recsrc: *mut core::ffi::c_void,
    pub private_free_recsrc: Option<unsafe extern "C" fn(mixer: *mut snd_mixer_oss)>,
    pub reg_mutex: mutex,
    pub proc_entry: *mut snd_info_entry,
    pub oss_dev_alloc: i32,
    /* --- */
    pub oss_recsrc: i32,
}

#[repr(C)]
pub struct snd_mixer_oss_file {
    pub card: *mut snd_card,
    pub mixer: *mut snd_mixer_oss,
}

unsafe extern "C" {
    pub fn snd_mixer_oss_ioctl_card(
        card: *mut snd_card,
        cmd: u32,
        arg: usize,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
