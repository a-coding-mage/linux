/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OSS compatible sequencer driver
 * timer handling routines
 *
 * Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
 */

// Depends on declarations from "seq_oss_device.h".

use core::ffi::{c_int, c_uint};

/*
 * timer information definition
 */
#[repr(C)]
pub struct seq_oss_timer {
    pub dp: *mut seq_oss_devinfo,
    pub cur_tick: reltime_t,
    pub realtime: c_int,
    pub running: c_int,
    pub tempo: c_int,
    pub ppq: c_int, /* ALSA queue */
    pub oss_tempo: c_int,
    pub oss_timebase: c_int,
}

unsafe extern "C" {
    pub fn snd_seq_oss_timer_new(dp: *mut seq_oss_devinfo) -> *mut seq_oss_timer;
    pub fn snd_seq_oss_timer_delete(dp: *mut seq_oss_timer);

    pub fn snd_seq_oss_timer_start(timer: *mut seq_oss_timer) -> c_int;
    pub fn snd_seq_oss_timer_stop(timer: *mut seq_oss_timer) -> c_int;
    pub fn snd_seq_oss_timer_continue(timer: *mut seq_oss_timer) -> c_int;
    pub fn snd_seq_oss_timer_tempo(timer: *mut seq_oss_timer, value: c_int) -> c_int;

    pub fn snd_seq_oss_timer_ioctl(
        timer: *mut seq_oss_timer,
        cmd: c_uint,
        arg: *mut c_int, /* __user */
    ) -> c_int;
}

pub use snd_seq_oss_timer_start as snd_seq_oss_timer_reset;

/*
 * get current processed time
 */
#[inline]
pub unsafe fn snd_seq_oss_timer_cur_tick(timer: *mut seq_oss_timer) -> abstime_t {
    unsafe { (*timer).cur_tick }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
