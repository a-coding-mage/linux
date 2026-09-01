// SPDX-License-Identifier: GPL-2.0-only
/*
 * fireworks_midi.c - a part of driver for Fireworks based devices
 *
 * Copyright (c) 2009-2010 Clemens Ladisch
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

// Rust translation of declarations supplied by "fireworks.h" and other kernel
// headers. Layout is represented only for fields used by this implementation.

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
    pub driver: *mut c_char,
    pub shortname: *mut c_char,
}

#[repr(C)]
pub struct amdtp_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_efw {
    pub card: *mut snd_card,
    pub mutex: mutex,
    pub lock: spinlock_t,
    pub tx_stream: amdtp_stream,
    pub rx_stream: amdtp_stream,
    pub substreams_counter: c_uint,
    pub midi_out_ports: c_uint,
    pub midi_in_ports: c_uint,
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    pub rmidi: *mut snd_rawmidi,
    pub list: list_head,
    pub number: c_int,
    pub name: [c_char; 32],
}

#[repr(C)]
pub struct snd_rawmidi_str {
    pub substreams: list_head,
}

#[repr(C)]
pub struct snd_rawmidi {
    pub private_data: *mut c_void,
    pub name: [c_char; 80],
    pub info_flags: c_uint,
    pub streams: [snd_rawmidi_str; 2],
}

#[repr(C)]
pub struct snd_rawmidi_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
}

pub const SNDRV_RAWMIDI_STREAM_OUTPUT: usize = 0;
pub const SNDRV_RAWMIDI_STREAM_INPUT: usize = 1;
pub const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 0x00000001;
pub const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 0x00000002;
pub const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 0x00000004;

unsafe extern "C" {
    fn snd_efw_stream_lock_try(efw: *mut snd_efw) -> c_int;
    fn snd_efw_stream_lock_release(efw: *mut snd_efw);
    fn snd_efw_stream_reserve_duplex(
        efw: *mut snd_efw,
        rate: c_uint,
        events_per_period: c_uint,
        frames_per_buffer: c_uint,
    ) -> c_int;
    fn snd_efw_stream_start_duplex(efw: *mut snd_efw) -> c_int;
    fn snd_efw_stream_stop_duplex(efw: *mut snd_efw);

    fn amdtp_am824_midi_trigger(
        stream: *mut amdtp_stream,
        port: c_int,
        substrm: *mut snd_rawmidi_substream,
    );

    fn snd_rawmidi_new(
        card: *mut snd_card,
        id: *mut c_char,
        device: c_int,
        output_count: c_uint,
        input_count: c_uint,
        rrawmidi: *mut *mut snd_rawmidi,
    ) -> c_int;
    fn snd_rawmidi_set_ops(
        rmidi: *mut snd_rawmidi,
        stream: c_int,
        ops: *const snd_rawmidi_ops,
    );

    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);

    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
}

#[allow(non_camel_case_types)]
type c_ulong = core::ffi::c_ulong;

#[inline]
unsafe fn container_of_substream_list(ptr: *mut list_head) -> *mut snd_rawmidi_substream {
    (ptr as *mut u8).sub(core::mem::offset_of!(snd_rawmidi_substream, list))
        as *mut snd_rawmidi_substream
}

unsafe extern "C" fn midi_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let efw = (*(*substream).rmidi).private_data as *mut snd_efw;
    let mut err: c_int;

    err = snd_efw_stream_lock_try(efw);
    if err < 0 {
        return err;
    }

    mutex_lock(&mut (*efw).mutex);
    err = snd_efw_stream_reserve_duplex(efw, 0, 0, 0);
    if err >= 0 {
        (*efw).substreams_counter = (*efw).substreams_counter.wrapping_add(1);
        err = snd_efw_stream_start_duplex(efw);
        if err < 0 {
            (*efw).substreams_counter = (*efw).substreams_counter.wrapping_sub(1);
        }
    }
    mutex_unlock(&mut (*efw).mutex);

    if err < 0 {
        snd_efw_stream_lock_release(efw);
    }
    err
}

unsafe extern "C" fn midi_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let efw = (*(*substream).rmidi).private_data as *mut snd_efw;

    mutex_lock(&mut (*efw).mutex);
    (*efw).substreams_counter = (*efw).substreams_counter.wrapping_sub(1);
    snd_efw_stream_stop_duplex(efw);
    mutex_unlock(&mut (*efw).mutex);

    snd_efw_stream_lock_release(efw);
    0
}

unsafe extern "C" fn midi_capture_trigger(substrm: *mut snd_rawmidi_substream, up: c_int) {
    let efw = (*(*substrm).rmidi).private_data as *mut snd_efw;
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&mut (*efw).lock, &mut flags);

    if up != 0 {
        amdtp_am824_midi_trigger(&mut (*efw).tx_stream, (*substrm).number, substrm);
    } else {
        amdtp_am824_midi_trigger(&mut (*efw).tx_stream, (*substrm).number, ptr::null_mut());
    }

    spin_unlock_irqrestore(&mut (*efw).lock, flags);
}

unsafe extern "C" fn midi_playback_trigger(substrm: *mut snd_rawmidi_substream, up: c_int) {
    let efw = (*(*substrm).rmidi).private_data as *mut snd_efw;
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&mut (*efw).lock, &mut flags);

    if up != 0 {
        amdtp_am824_midi_trigger(&mut (*efw).rx_stream, (*substrm).number, substrm);
    } else {
        amdtp_am824_midi_trigger(&mut (*efw).rx_stream, (*substrm).number, ptr::null_mut());
    }

    spin_unlock_irqrestore(&mut (*efw).lock, flags);
}

unsafe fn set_midi_substream_names(efw: *mut snd_efw, str_: *mut snd_rawmidi_str) {
    let mut pos = (*str_).substreams.next;

    while pos != &mut (*str_).substreams {
        let subs = container_of_substream_list(pos);

        scnprintf(
            (*subs).name.as_mut_ptr(),
            core::mem::size_of_val(&(*subs).name),
            c"%s MIDI %d".as_ptr(),
            (*(*efw).card).shortname,
            (*subs).number + 1,
        );

        pos = (*pos).next;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_efw_create_midi_devices(efw: *mut snd_efw) -> c_int {
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
        (*efw).card,
        (*(*efw).card).driver,
        0,
        (*efw).midi_out_ports,
        (*efw).midi_in_ports,
        &mut rmidi,
    );
    if err < 0 {
        return err;
    }

    snprintf(
        (*rmidi).name.as_mut_ptr(),
        core::mem::size_of_val(&(*rmidi).name),
        c"%s MIDI".as_ptr(),
        (*(*efw).card).shortname,
    );
    (*rmidi).private_data = efw as *mut c_void;

    if (*efw).midi_in_ports > 0 {
        (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_INPUT;

        snd_rawmidi_set_ops(
            rmidi,
            SNDRV_RAWMIDI_STREAM_INPUT as c_int,
            &CAPTURE_OPS,
        );

        str_ = &mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_INPUT];

        set_midi_substream_names(efw, str_);
    }

    if (*efw).midi_out_ports > 0 {
        (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT;

        snd_rawmidi_set_ops(
            rmidi,
            SNDRV_RAWMIDI_STREAM_OUTPUT as c_int,
            &PLAYBACK_OPS,
        );

        str_ = &mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_OUTPUT];

        set_midi_substream_names(efw, str_);
    }

    if ((*efw).midi_out_ports > 0) && ((*efw).midi_in_ports > 0) {
        (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_DUPLEX;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
