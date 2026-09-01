/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OSS compatible sequencer driver
 *
 * seq_oss_event.h - OSS event queue record
 *
 * Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
 */

/* C header dependency: "seq_oss_device.h" */

pub const SHORT_EVENT_SIZE: usize = 4;
pub const LONG_EVENT_SIZE: usize = 8;

/* short event (4bytes) */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_short {
    pub code: u8,
    pub parm1: u8,
    pub dev: u8,
    pub parm2: u8,
}

/* short note events (4bytes) */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_note {
    pub code: u8,
    pub chn: u8,
    pub note: u8,
    pub vel: u8,
}

/* long timer events (8bytes) */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_timer {
    pub code: u8,
    pub cmd: u8,
    pub dummy1: u8,
    pub dummy2: u8,
    pub time: u32,
}

/* long extended events (8bytes) */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_extended {
    pub code: u8,
    pub cmd: u8,
    pub dev: u8,
    pub chn: u8,
    pub p1: u8,
    pub p2: u8,
    pub p3: u8,
    pub p4: u8,
}

/* long channel events (8bytes) */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_long {
    pub code: u8,
    pub dev: u8,
    pub cmd: u8,
    pub chn: u8,
    pub p1: u8,
    pub p2: u8,
    pub val: u16,
}

/* channel voice events (8bytes) */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_voice {
    pub code: u8,
    pub dev: u8,
    pub cmd: u8,
    pub chn: u8,
    pub note: u8,
    pub parm: u8,
    pub dummy: u16,
}

/* sysex events (8bytes) */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_sysex {
    pub code: u8,
    pub dev: u8,
    pub buf: [u8; 6],
}

/* event record */
#[repr(C)]
#[derive(Copy, Clone)]
pub union evrec {
    pub s: evrec_short,
    pub n: evrec_note,
    pub l: evrec_long,
    pub v: evrec_voice,
    pub t: evrec_timer,
    pub e: evrec_extended,
    pub x: evrec_sysex,
    pub echo: u32,
    pub c: [u8; LONG_EVENT_SIZE],
}

#[inline]
pub unsafe fn ev_is_long(ev: *const evrec) -> bool {
    unsafe { (*ev).s.code >= 128 }
}

#[inline]
pub unsafe fn ev_length(ev: *const evrec) -> usize {
    if unsafe { (*ev).s.code >= 128 } {
        LONG_EVENT_SIZE
    } else {
        SHORT_EVENT_SIZE
    }
}

unsafe extern "C" {
    pub fn snd_seq_oss_process_event(
        dp: *mut seq_oss_devinfo,
        q: *mut evrec,
        ev: *mut snd_seq_event,
        lockp: *mut *mut snd_use_lock_t,
    ) -> core::ffi::c_int;

    pub fn snd_seq_oss_process_timer_event(
        rec: *mut seq_oss_timer,
        q: *mut evrec,
    ) -> core::ffi::c_int;

    pub fn snd_seq_oss_event_input(
        ev: *mut snd_seq_event,
        direct: core::ffi::c_int,
        private_data: *mut core::ffi::c_void,
        atomic: core::ffi::c_int,
        hop: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub fn snd_use_lock_free(lock: *mut snd_use_lock_t);
}

/* DEFINE_FREE(seq_oss_use_lock, snd_use_lock_t *, if (_T) snd_use_lock_free(_T)) */
#[inline]
pub unsafe fn seq_oss_use_lock(_T: *mut snd_use_lock_t) {
    if !_T.is_null() {
        unsafe { snd_use_lock_free(_T) };
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
