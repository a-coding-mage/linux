// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OSS compatible sequencer driver
 *
 * synth device information
 *
 * Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
 */

// C header dependencies:
// #include "seq_oss_device.h"
// #include <sound/seq_oss_legacy.h>
// #include <sound/seq_device.h>

use core::ffi::{c_char, c_int, c_ulong, c_uchar, c_uint, c_void};

// Opaque external types supplied by included headers.
#[repr(C)]
pub struct snd_seq_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_oss_devinfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_oss_synthinfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_seq_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct synth_info {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn snd_seq_oss_synth_init();
    pub fn snd_seq_oss_synth_probe(dev: *mut snd_seq_device) -> c_int;
    pub fn snd_seq_oss_synth_remove(dev: *mut snd_seq_device);
    pub fn snd_seq_oss_synth_setup(dp: *mut seq_oss_devinfo);
    pub fn snd_seq_oss_synth_setup_midi(dp: *mut seq_oss_devinfo);
    pub fn snd_seq_oss_synth_cleanup(dp: *mut seq_oss_devinfo);

    pub fn snd_seq_oss_synth_reset(dp: *mut seq_oss_devinfo, dev: c_int);
    pub fn snd_seq_oss_synth_load_patch(
        dp: *mut seq_oss_devinfo,
        dev: c_int,
        fmt: c_int,
        buf: *const c_char,
        p: c_int,
        c: c_int,
    ) -> c_int;
    pub fn snd_seq_oss_synth_info(
        dp: *mut seq_oss_devinfo,
        dev: c_int,
    ) -> *mut seq_oss_synthinfo;
    pub fn snd_seq_oss_synth_sysex(
        dp: *mut seq_oss_devinfo,
        dev: c_int,
        buf: *mut c_uchar,
        ev: *mut snd_seq_event,
    ) -> c_int;
    pub fn snd_seq_oss_synth_addr(
        dp: *mut seq_oss_devinfo,
        dev: c_int,
        ev: *mut snd_seq_event,
    ) -> c_int;
    pub fn snd_seq_oss_synth_ioctl(
        dp: *mut seq_oss_devinfo,
        dev: c_int,
        cmd: c_uint,
        addr: c_ulong,
    ) -> c_int;
    pub fn snd_seq_oss_synth_raw_event(
        dp: *mut seq_oss_devinfo,
        dev: c_int,
        data: *mut c_uchar,
        ev: *mut snd_seq_event,
    ) -> c_int;

    pub fn snd_seq_oss_synth_make_info(
        dp: *mut seq_oss_devinfo,
        dev: c_int,
        inf: *mut synth_info,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
