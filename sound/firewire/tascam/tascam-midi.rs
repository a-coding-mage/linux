// SPDX-License-Identifier: GPL-2.0-only
/*
 * tascam-midi.c - a part of driver for TASCAM FireWire series
 *
 * Copyright (c) 2015 Takashi Sakamoto
 */

// Rust translation of the original C implementation. The original included
// "tascam.h"; the referenced kernel/ALSA types, constants, and helpers are
// treated as external dependencies here.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct snd_tscm {
    pub card: *mut snd_card,
    pub spec: *mut snd_tscm_spec,
    pub out_ports: *mut snd_fw_async_midi_port,
    pub tx_midi_substreams: *mut *mut snd_rawmidi_substream,
    pub lock: spinlock_t,
}

#[repr(C)]
pub struct snd_tscm_spec {
    pub midi_playback_ports: c_uint,
    pub midi_capture_ports: c_uint,
}

#[repr(C)]
pub struct snd_card {
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
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
    pub number: c_uint,
    pub name: [c_char; 32],
    pub rmidi: *mut snd_rawmidi,
}

#[repr(C)]
pub struct snd_rawmidi_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
    pub drain: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream)>,
}

#[repr(C)]
pub struct snd_fw_async_midi_port {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

pub const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 0x00000001;
pub const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 0x00000002;
pub const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 0x00000004;
pub const SNDRV_RAWMIDI_STREAM_OUTPUT: usize = 0;
pub const SNDRV_RAWMIDI_STREAM_INPUT: usize = 1;

unsafe extern "C" {
    fn snd_fw_async_midi_port_init(port: *mut snd_fw_async_midi_port);
    fn snd_fw_async_midi_port_finish(port: *mut snd_fw_async_midi_port);
    fn snd_fw_async_midi_port_run(
        port: *mut snd_fw_async_midi_port,
        substream: *mut snd_rawmidi_substream,
    );
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
        stream: usize,
        ops: *const snd_rawmidi_ops,
    );
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
}

pub type c_ulong = core::ffi::c_ulong;

unsafe extern "C" fn midi_capture_open(_substream: *mut snd_rawmidi_substream) -> c_int {
    /* Do nothing. */
    0
}

unsafe extern "C" fn midi_playback_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let tscm = (*(*substream).rmidi).private_data as *mut snd_tscm;

    snd_fw_async_midi_port_init((*tscm).out_ports.add((*substream).number as usize));

    0
}

unsafe extern "C" fn midi_capture_close(_substream: *mut snd_rawmidi_substream) -> c_int {
    /* Do nothing. */
    0
}

unsafe extern "C" fn midi_playback_close(_substream: *mut snd_rawmidi_substream) -> c_int {
    0
}

unsafe extern "C" fn midi_playback_drain(substream: *mut snd_rawmidi_substream) {
    let tscm = (*(*substream).rmidi).private_data as *mut snd_tscm;

    snd_fw_async_midi_port_finish((*tscm).out_ports.add((*substream).number as usize));
}

unsafe extern "C" fn midi_capture_trigger(substrm: *mut snd_rawmidi_substream, up: c_int) {
    let tscm = (*(*substrm).rmidi).private_data as *mut snd_tscm;
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&mut (*tscm).lock, &mut flags);

    if up != 0 {
        *(*tscm).tx_midi_substreams.add((*substrm).number as usize) = substrm;
    } else {
        *(*tscm).tx_midi_substreams.add((*substrm).number as usize) = core::ptr::null_mut();
    }

    spin_unlock_irqrestore(&mut (*tscm).lock, flags);
}

unsafe extern "C" fn midi_playback_trigger(substrm: *mut snd_rawmidi_substream, up: c_int) {
    let tscm = (*(*substrm).rmidi).private_data as *mut snd_tscm;
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&mut (*tscm).lock, &mut flags);

    if up != 0 {
        snd_fw_async_midi_port_run((*tscm).out_ports.add((*substrm).number as usize), substrm);
    }

    spin_unlock_irqrestore(&mut (*tscm).lock, flags);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_tscm_create_midi_devices(tscm: *mut snd_tscm) -> c_int {
    static CAPTURE_OPS: snd_rawmidi_ops = snd_rawmidi_ops {
        open: Some(midi_capture_open),
        close: Some(midi_capture_close),
        trigger: Some(midi_capture_trigger),
        drain: None,
    };
    static PLAYBACK_OPS: snd_rawmidi_ops = snd_rawmidi_ops {
        open: Some(midi_playback_open),
        close: Some(midi_playback_close),
        drain: Some(midi_playback_drain),
        trigger: Some(midi_playback_trigger),
    };
    let mut rmidi: *mut snd_rawmidi = core::ptr::null_mut();
    let mut stream: *mut snd_rawmidi_str;
    let mut subs: *mut snd_rawmidi_substream;
    let err: c_int;

    err = snd_rawmidi_new(
        (*tscm).card,
        (*(*tscm).card).driver.as_ptr(),
        0,
        (*(*tscm).spec).midi_playback_ports,
        (*(*tscm).spec).midi_capture_ports,
        &mut rmidi,
    );
    if err < 0 {
        return err;
    }

    snprintf(
        (*rmidi).name.as_mut_ptr(),
        core::mem::size_of_val(&(*rmidi).name),
        c"%s MIDI".as_ptr(),
        (*(*tscm).card).shortname.as_ptr(),
    );
    (*rmidi).private_data = tscm as *mut c_void;

    (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_INPUT;
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT, &CAPTURE_OPS);
    stream = &mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_INPUT];

    /* Set port names for MIDI input. */
    subs = (*stream).substreams.next as *mut snd_rawmidi_substream;
    while !core::ptr::addr_eq(&mut (*subs).list, &mut (*stream).substreams) {
        /* TODO: support virtual MIDI ports. */
        if (*subs).number < (*(*tscm).spec).midi_capture_ports {
            /* Hardware MIDI ports. */
            scnprintf(
                (*subs).name.as_mut_ptr(),
                core::mem::size_of_val(&(*subs).name),
                c"%s MIDI %d".as_ptr(),
                (*(*tscm).card).shortname.as_ptr(),
                (*subs).number + 1,
            );
        }
        subs = (*subs).list.next as *mut snd_rawmidi_substream;
    }

    (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT;
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT, &PLAYBACK_OPS);
    stream = &mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_OUTPUT];

    /* Set port names for MIDI ourput. */
    subs = (*stream).substreams.next as *mut snd_rawmidi_substream;
    while !core::ptr::addr_eq(&mut (*subs).list, &mut (*stream).substreams) {
        if (*subs).number < (*(*tscm).spec).midi_playback_ports {
            /* Hardware MIDI ports only. */
            scnprintf(
                (*subs).name.as_mut_ptr(),
                core::mem::size_of_val(&(*subs).name),
                c"%s MIDI %d".as_ptr(),
                (*(*tscm).card).shortname.as_ptr(),
                (*subs).number + 1,
            );
        }
        subs = (*subs).list.next as *mut snd_rawmidi_substream;
    }

    (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_DUPLEX;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
