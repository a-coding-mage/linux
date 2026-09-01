/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OSS compatible sequencer driver
 * write priority queue
 *
 * Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
 */

/* Depends on seq_oss_device.h for seq_oss_devinfo and related kernel types. */
use crate::{abstime_t, seq_oss_devinfo, spinlock_t, wait_queue_head_t};

#[repr(C)]
pub struct seq_oss_writeq {
    pub dp: *mut seq_oss_devinfo,
    pub maxlen: ::core::ffi::c_int,
    pub sync_time: abstime_t,
    pub sync_event_put: ::core::ffi::c_int,
    pub sync_sleep: wait_queue_head_t,
    pub sync_lock: spinlock_t,
}

/*
 * seq_oss_writeq.c
 */
unsafe extern "C" {
    pub fn snd_seq_oss_writeq_new(
        dp: *mut seq_oss_devinfo,
        maxlen: ::core::ffi::c_int,
    ) -> *mut seq_oss_writeq;
    pub fn snd_seq_oss_writeq_delete(q: *mut seq_oss_writeq);
    pub fn snd_seq_oss_writeq_clear(q: *mut seq_oss_writeq);
    pub fn snd_seq_oss_writeq_sync(q: *mut seq_oss_writeq) -> ::core::ffi::c_int;
    pub fn snd_seq_oss_writeq_wakeup(q: *mut seq_oss_writeq, time: abstime_t);
    pub fn snd_seq_oss_writeq_get_free_size(q: *mut seq_oss_writeq) -> ::core::ffi::c_int;
    pub fn snd_seq_oss_writeq_set_output(q: *mut seq_oss_writeq, size: ::core::ffi::c_int);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
