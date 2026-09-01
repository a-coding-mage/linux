/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OSS compatible sequencer driver
 * read fifo queue
 *
 * Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
 */

// C header dependencies:
// #include "seq_oss_device.h"
// #include "seq_oss_event.h"

/*
 * definition of read queue
 */
#[repr(C)]
pub struct seq_oss_readq {
    pub qlen: core::ffi::c_int,
    pub maxlen: core::ffi::c_int,
    pub head: core::ffi::c_int,
    pub tail: core::ffi::c_int,
    pub pre_event_timeout: core::ffi::c_ulong,
    pub input_time: core::ffi::c_ulong,
    pub midi_sleep: wait_queue_head_t,
    pub lock: spinlock_t,
    // Flexible array member in C: union evrec q[] __counted_by(maxlen);
    pub q: [evrec; 0],
}

unsafe extern "C" {
    pub fn snd_seq_oss_readq_new(
        dp: *mut seq_oss_devinfo,
        maxlen: core::ffi::c_int,
    ) -> *mut seq_oss_readq;
    pub fn snd_seq_oss_readq_delete(q: *mut seq_oss_readq);
    pub fn snd_seq_oss_readq_clear(readq: *mut seq_oss_readq);
    pub fn snd_seq_oss_readq_poll(
        readq: *mut seq_oss_readq,
        file: *mut file,
        wait: *mut poll_table,
    ) -> core::ffi::c_uint;
    pub fn snd_seq_oss_readq_puts(
        readq: *mut seq_oss_readq,
        dev: core::ffi::c_int,
        data: *mut core::ffi::c_uchar,
        len: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn snd_seq_oss_readq_sysex(
        q: *mut seq_oss_readq,
        dev: core::ffi::c_int,
        ev: *mut snd_seq_event,
    ) -> core::ffi::c_int;
    pub fn snd_seq_oss_readq_put_event(
        readq: *mut seq_oss_readq,
        ev: *mut evrec,
    ) -> core::ffi::c_int;
    pub fn snd_seq_oss_readq_put_timestamp(
        readq: *mut seq_oss_readq,
        curt: core::ffi::c_ulong,
        seq_mode: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn snd_seq_oss_readq_pick(
        q: *mut seq_oss_readq,
        rec: *mut evrec,
    ) -> core::ffi::c_int;
    pub fn snd_seq_oss_readq_wait(q: *mut seq_oss_readq);
    pub fn snd_seq_oss_readq_free(q: *mut seq_oss_readq);
}

macro_rules! snd_seq_oss_readq_lock {
    ($q:expr, $flags:expr) => {
        spin_lock_irqsave(core::ptr::addr_of_mut!((*$q).lock), $flags)
    };
}

macro_rules! snd_seq_oss_readq_unlock {
    ($q:expr, $flags:expr) => {
        spin_unlock_irqrestore(core::ptr::addr_of_mut!((*$q).lock), $flags)
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
