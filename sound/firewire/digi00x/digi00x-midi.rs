// SPDX-License-Identifier: GPL-2.0-only
/*
 * digi00x-midi.h - a part of driver for Digidesign Digi 002/003 family
 *
 * Copyright (c) 2014-2015 Takashi Sakamoto
 */

// C dependency: #include "digi00x.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

pub type bool_ = bool;

pub const SNDRV_RAWMIDI_STREAM_OUTPUT: c_int = 0;
pub const SNDRV_RAWMIDI_STREAM_INPUT: c_int = 1;
pub const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 0x00000001;
pub const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 0x00000002;
pub const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 0x00000004;

// External constants supplied by digi00x.h.
unsafe extern "C" {
    static DOT_MIDI_OUT_PORTS: c_uint;
    static DOT_MIDI_IN_PORTS: c_uint;
}

#[repr(C)]
pub struct snd_card {
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    pub list: list_head,
    pub number: c_int,
    pub name: [c_char; 32],
    pub rmidi: *mut snd_rawmidi,
}

#[repr(C)]
pub struct snd_rawmidi_str {
    pub substreams: list_head,
}

#[repr(C)]
pub struct snd_rawmidi {
    pub streams: [snd_rawmidi_str; 2],
    pub private_data: *mut c_void,
    pub device: c_int,
    pub name: [c_char; 80],
    pub info_flags: c_uint,
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
pub struct amdtp_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dg00x {
    pub mutex: mutex,
    pub lock: spinlock_t,
    pub tx_stream: amdtp_stream,
    pub rx_stream: amdtp_stream,
    pub substreams_counter: c_uint,
    pub card: *mut snd_card,
    pub is_console: bool,
}

#[repr(C)]
pub struct snd_rawmidi_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
}

unsafe extern "C" {
    fn snd_dg00x_stream_lock_try(dg00x: *mut snd_dg00x) -> c_int;
    fn snd_dg00x_stream_lock_release(dg00x: *mut snd_dg00x);
    fn snd_dg00x_stream_reserve_duplex(
        dg00x: *mut snd_dg00x,
        rate: c_uint,
        frames_per_period: c_uint,
        frames_per_buffer: c_uint,
    ) -> c_int;
    fn snd_dg00x_stream_start_duplex(dg00x: *mut snd_dg00x) -> c_int;
    fn snd_dg00x_stream_stop_duplex(dg00x: *mut snd_dg00x);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_uint);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_uint);
    fn amdtp_dot_midi_trigger(
        s: *mut amdtp_stream,
        port: c_uint,
        substream: *mut snd_rawmidi_substream,
    );
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn snd_rawmidi_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        output_count: c_uint,
        input_count: c_uint,
        rmidi: *mut *mut snd_rawmidi,
    ) -> c_int;
    fn snd_rawmidi_set_ops(
        rmidi: *mut snd_rawmidi,
        stream: c_int,
        ops: *const snd_rawmidi_ops,
    );
}

unsafe extern "C" fn midi_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let dg00x = (*(*substream).rmidi).private_data as *mut snd_dg00x;
    let mut err: c_int;

    err = snd_dg00x_stream_lock_try(dg00x);
    if err < 0 {
        return err;
    }

    mutex_lock(&mut (*dg00x).mutex);
    err = snd_dg00x_stream_reserve_duplex(dg00x, 0, 0, 0);
    if err >= 0 {
        (*dg00x).substreams_counter = (*dg00x).substreams_counter.wrapping_add(1);
        err = snd_dg00x_stream_start_duplex(dg00x);
        if err < 0 {
            (*dg00x).substreams_counter = (*dg00x).substreams_counter.wrapping_sub(1);
        }
    }
    mutex_unlock(&mut (*dg00x).mutex);

    if err < 0 {
        snd_dg00x_stream_lock_release(dg00x);
    }

    err
}

unsafe extern "C" fn midi_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let dg00x = (*(*substream).rmidi).private_data as *mut snd_dg00x;

    mutex_lock(&mut (*dg00x).mutex);
    (*dg00x).substreams_counter = (*dg00x).substreams_counter.wrapping_sub(1);
    snd_dg00x_stream_stop_duplex(dg00x);
    mutex_unlock(&mut (*dg00x).mutex);

    snd_dg00x_stream_lock_release(dg00x);
    0
}

unsafe extern "C" fn midi_capture_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    let dg00x = (*(*substream).rmidi).private_data as *mut snd_dg00x;
    let port: c_uint;
    let mut flags: c_uint = 0;

    if (*(*substream).rmidi).device == 0 {
        port = (*substream).number as c_uint;
    } else {
        port = 2;
    }

    spin_lock_irqsave(&mut (*dg00x).lock, &mut flags);

    if up != 0 {
        amdtp_dot_midi_trigger(&mut (*dg00x).tx_stream, port, substream);
    } else {
        amdtp_dot_midi_trigger(&mut (*dg00x).tx_stream, port, ptr::null_mut());
    }

    spin_unlock_irqrestore(&mut (*dg00x).lock, flags);
}

unsafe extern "C" fn midi_playback_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    let dg00x = (*(*substream).rmidi).private_data as *mut snd_dg00x;
    let port: c_uint;
    let mut flags: c_uint = 0;

    if (*(*substream).rmidi).device == 0 {
        port = (*substream).number as c_uint;
    } else {
        port = 2;
    }

    spin_lock_irqsave(&mut (*dg00x).lock, &mut flags);

    if up != 0 {
        amdtp_dot_midi_trigger(&mut (*dg00x).rx_stream, port, substream);
    } else {
        amdtp_dot_midi_trigger(&mut (*dg00x).rx_stream, port, ptr::null_mut());
    }

    spin_unlock_irqrestore(&mut (*dg00x).lock, flags);
}

unsafe fn set_substream_names(
    dg00x: *mut snd_dg00x,
    rmidi: *mut snd_rawmidi,
    is_console: bool,
) {
    let mut i: c_int;

    i = 0;
    while i < 2 {
        let str_ = &mut (*rmidi).streams[i as usize] as *mut snd_rawmidi_str;

        /*
         * C source uses:
         * list_for_each_entry(subs, &str->substreams, list) { ... }
         *
         * The concrete iterator depends on Linux list_head/container_of
         * definitions from digi00x.h and related headers.
         */
        let mut pos = (*str_).substreams.next;
        while pos != &mut (*str_).substreams as *mut list_head {
            let subs = pos as *mut snd_rawmidi_substream;

            if !is_console {
                scnprintf(
                    (*subs).name.as_mut_ptr(),
                    core::mem::size_of_val(&(*subs).name),
                    c"%s MIDI %d".as_ptr(),
                    (*(*dg00x).card).shortname.as_ptr(),
                    (*subs).number + 1,
                );
            } else {
                scnprintf(
                    (*subs).name.as_mut_ptr(),
                    core::mem::size_of_val(&(*subs).name),
                    c"%s control".as_ptr(),
                    (*(*dg00x).card).shortname.as_ptr(),
                );
            }

            pos = (*pos).next;
        }

        i += 1;
    }
}

unsafe fn add_substream_pair(
    dg00x: *mut snd_dg00x,
    out_ports: c_uint,
    in_ports: c_uint,
    is_console: bool,
) -> c_int {
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
    let label: *const c_char;
    let mut rmidi: *mut snd_rawmidi = ptr::null_mut();
    let mut err: c_int;

    /* Add physical midi ports. */
    err = snd_rawmidi_new(
        (*dg00x).card,
        (*(*dg00x).card).driver.as_ptr(),
        is_console as c_int,
        out_ports,
        in_ports,
        &mut rmidi,
    );
    if err < 0 {
        return err;
    }
    (*rmidi).private_data = dg00x as *mut c_void;

    if !is_console {
        label = c"%s control".as_ptr();
    } else {
        label = c"%s MIDI".as_ptr();
    }
    snprintf(
        (*rmidi).name.as_mut_ptr(),
        core::mem::size_of_val(&(*rmidi).name),
        label,
        (*(*dg00x).card).shortname.as_ptr(),
    );

    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT, &PLAYBACK_OPS);
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT, &CAPTURE_OPS);

    (*rmidi).info_flags |=
        SNDRV_RAWMIDI_INFO_INPUT | SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_DUPLEX;

    set_substream_names(dg00x, rmidi, is_console);

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_dg00x_create_midi_devices(dg00x: *mut snd_dg00x) -> c_int {
    let mut err: c_int;

    /* Add physical midi ports. */
    err = add_substream_pair(dg00x, DOT_MIDI_OUT_PORTS, DOT_MIDI_IN_PORTS, false);
    if err < 0 {
        return err;
    }

    if (*dg00x).is_console {
        err = add_substream_pair(dg00x, 1, 1, true);
    }

    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
