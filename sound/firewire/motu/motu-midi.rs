// SPDX-License-Identifier: GPL-2.0-only
/*
 * motu-midi.h - a part of driver for MOTU FireWire series
 *
 * Copyright (c) 2015-2017 Takashi Sakamoto <o-takashi@sakamocchi.jp>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

// Dependency intent: translated from `#include "motu.h"`.

pub const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 0x00000001;
pub const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 0x00000002;
pub const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 0x00000004;

pub const SNDRV_RAWMIDI_STREAM_OUTPUT: usize = 0;
pub const SNDRV_RAWMIDI_STREAM_INPUT: usize = 1;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
}

#[repr(C)]
pub struct amdtp_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_motu {
    pub card: *mut snd_card,
    pub mutex: mutex,
    pub lock: spinlock_t,
    pub tx_stream: amdtp_stream,
    pub rx_stream: amdtp_stream,
    pub substreams_counter: c_uint,
}

#[repr(C)]
pub struct snd_rawmidi {
    pub name: [c_char; 80],
    pub streams: [snd_rawmidi_str; 2],
    pub private_data: *mut c_void,
    pub info_flags: c_uint,
}

#[repr(C)]
pub struct snd_rawmidi_str {
    pub substreams: list_head,
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    pub list: list_head,
    pub rmidi: *mut snd_rawmidi,
    pub number: c_int,
    pub name: [c_char; 32],
}

#[repr(C)]
pub struct snd_rawmidi_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
}

unsafe extern "C" {
    fn snd_motu_stream_lock_try(motu: *mut snd_motu) -> c_int;
    fn snd_motu_stream_lock_release(motu: *mut snd_motu);
    fn snd_motu_stream_reserve_duplex(
        motu: *mut snd_motu,
        rate: c_uint,
        frames_per_period: c_uint,
        frames_per_buffer: c_uint,
    ) -> c_int;
    fn snd_motu_stream_start_duplex(motu: *mut snd_motu) -> c_int;
    fn snd_motu_stream_stop_duplex(motu: *mut snd_motu);

    fn amdtp_motu_midi_trigger(
        s: *mut amdtp_stream,
        port: c_int,
        substrm: *mut snd_rawmidi_substream,
    );

    fn snd_rawmidi_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        output_count: c_int,
        input_count: c_int,
        rrawmidi: *mut *mut snd_rawmidi,
    ) -> c_int;
    fn snd_rawmidi_set_ops(rmidi: *mut snd_rawmidi, stream: c_int, ops: *const snd_rawmidi_ops);

    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;

    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn spin_lock_irqsave(lock: *mut spinlock_t) -> c_ulong;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
}

pub type c_ulong = core::ffi::c_ulong;

const MIDI_NAME_FMT: &[u8] = b"%s MIDI\0";
const MIDI_SUBSTREAM_NAME_FMT: &[u8] = b"%s MIDI %d\0";

unsafe extern "C" fn midi_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let motu = (*(*substream).rmidi).private_data as *mut snd_motu;
    let mut err: c_int;

    err = snd_motu_stream_lock_try(motu);
    if err < 0 {
        return err;
    }

    mutex_lock(ptr::addr_of_mut!((*motu).mutex));
    err = snd_motu_stream_reserve_duplex(motu, 0, 0, 0);
    if err >= 0 {
        (*motu).substreams_counter = (*motu).substreams_counter.wrapping_add(1);
        err = snd_motu_stream_start_duplex(motu);
        if err < 0 {
            (*motu).substreams_counter = (*motu).substreams_counter.wrapping_sub(1);
        }
    }
    mutex_unlock(ptr::addr_of_mut!((*motu).mutex));

    if err < 0 {
        snd_motu_stream_lock_release(motu);
    }

    err
}

unsafe extern "C" fn midi_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let motu = (*(*substream).rmidi).private_data as *mut snd_motu;

    mutex_lock(ptr::addr_of_mut!((*motu).mutex));
    (*motu).substreams_counter = (*motu).substreams_counter.wrapping_sub(1);
    snd_motu_stream_stop_duplex(motu);
    mutex_unlock(ptr::addr_of_mut!((*motu).mutex));

    snd_motu_stream_lock_release(motu);
    0
}

unsafe extern "C" fn midi_capture_trigger(substrm: *mut snd_rawmidi_substream, up: c_int) {
    let motu = (*(*substrm).rmidi).private_data as *mut snd_motu;

    let flags = spin_lock_irqsave(ptr::addr_of_mut!((*motu).lock));

    if up != 0 {
        amdtp_motu_midi_trigger(ptr::addr_of_mut!((*motu).tx_stream), (*substrm).number, substrm);
    } else {
        amdtp_motu_midi_trigger(
            ptr::addr_of_mut!((*motu).tx_stream),
            (*substrm).number,
            ptr::null_mut(),
        );
    }

    spin_unlock_irqrestore(ptr::addr_of_mut!((*motu).lock), flags);
}

unsafe extern "C" fn midi_playback_trigger(substrm: *mut snd_rawmidi_substream, up: c_int) {
    let motu = (*(*substrm).rmidi).private_data as *mut snd_motu;

    let flags = spin_lock_irqsave(ptr::addr_of_mut!((*motu).lock));

    if up != 0 {
        amdtp_motu_midi_trigger(ptr::addr_of_mut!((*motu).rx_stream), (*substrm).number, substrm);
    } else {
        amdtp_motu_midi_trigger(
            ptr::addr_of_mut!((*motu).rx_stream),
            (*substrm).number,
            ptr::null_mut(),
        );
    }

    spin_unlock_irqrestore(ptr::addr_of_mut!((*motu).lock), flags);
}

unsafe fn set_midi_substream_names(motu: *mut snd_motu, str_: *mut snd_rawmidi_str) {
    let head = ptr::addr_of_mut!((*str_).substreams);
    let mut pos = (*head).next;

    while pos != head {
        let subs = (pos as *mut u8).sub(offset_of!(snd_rawmidi_substream, list))
            as *mut snd_rawmidi_substream;
        pos = (*pos).next;

        scnprintf(
            (*subs).name.as_mut_ptr(),
            size_of::<[c_char; 32]>(),
            MIDI_SUBSTREAM_NAME_FMT.as_ptr() as *const c_char,
            (*(*motu).card).shortname.as_ptr(),
            (*subs).number + 1,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_motu_create_midi_devices(motu: *mut snd_motu) -> c_int {
    static CAPTURE_OPS: snd_rawmidi_ops = snd_rawmidi_ops {
        open: Some(midi_open),
        close: Some(midi_close),
        trigger: Some(midi_capture_trigger),
    };
    static PLAYBACK_OPS: snd_rawmidi_ops = snd_rawmidi_ops {
        open: Some(midi_open),
        close: Some(midi_close),
        trigger: Some(midi_playback_trigger),
    };
    let mut rmidi: *mut snd_rawmidi = ptr::null_mut();
    let mut str_: *mut snd_rawmidi_str;
    let mut err: c_int;

    /* create midi ports */
    err = snd_rawmidi_new(
        (*motu).card,
        (*(*motu).card).driver.as_ptr(),
        0,
        1,
        1,
        ptr::addr_of_mut!(rmidi),
    );
    if err < 0 {
        return err;
    }

    snprintf(
        (*rmidi).name.as_mut_ptr(),
        size_of::<[c_char; 80]>(),
        MIDI_NAME_FMT.as_ptr() as *const c_char,
        (*(*motu).card).shortname.as_ptr(),
    );
    (*rmidi).private_data = motu as *mut c_void;

    (*rmidi).info_flags |=
        SNDRV_RAWMIDI_INFO_INPUT | SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_DUPLEX;

    snd_rawmidi_set_ops(
        rmidi,
        SNDRV_RAWMIDI_STREAM_INPUT as c_int,
        ptr::addr_of!(CAPTURE_OPS),
    );
    str_ = ptr::addr_of_mut!((*rmidi).streams[SNDRV_RAWMIDI_STREAM_INPUT]);
    set_midi_substream_names(motu, str_);

    snd_rawmidi_set_ops(
        rmidi,
        SNDRV_RAWMIDI_STREAM_OUTPUT as c_int,
        ptr::addr_of!(PLAYBACK_OPS),
    );
    str_ = ptr::addr_of_mut!((*rmidi).streams[SNDRV_RAWMIDI_STREAM_OUTPUT]);
    set_midi_substream_names(motu, str_);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
