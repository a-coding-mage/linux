// SPDX-License-Identifier: GPL-2.0-or-later
//
// Linux driver for TerraTec DMX 6Fire USB
//
// Author: Torsten Schenk <torsten.schenk@zoho.com>
// Created: Jan 01, 2011
// Copyright: (C) Torsten Schenk

// Dependency on common.h - defines sfire_chip and other kernel types

// Opaque types from external dependencies
#[repr(C)]
pub struct sfire_chip {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_rawmidi {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct urb {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _opaque: [u8; 0],
}

pub type midi_runtime_in_received_fn = unsafe extern "C" fn(*mut midi_runtime, *mut u8, i32);

#[repr(C)]
pub struct midi_runtime {
    pub chip: *mut sfire_chip,
    pub instance: *mut snd_rawmidi,
    pub r#in: *mut snd_rawmidi_substream,
    pub in_active: u8,
    pub in_lock: spinlock_t,
    pub out_lock: spinlock_t,
    pub out: *mut snd_rawmidi_substream,
    pub out_urb: urb,
    pub out_serial: u8,
    pub out_buffer: *mut u8,
    pub buffer_offset: i32,
    pub in_received: Option<midi_runtime_in_received_fn>,
}

extern "C" {
    pub fn usb6fire_midi_init(chip: *mut sfire_chip) -> i32;
    pub fn usb6fire_midi_abort(chip: *mut sfire_chip);
    pub fn usb6fire_midi_destroy(chip: *mut sfire_chip);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
