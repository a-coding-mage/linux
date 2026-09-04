// SPDX-License-Identifier: GPL-2.0-only
//
// Line 6 Linux USB driver
//
// Copyright (C) 2004-2010 Markus Grabner (line6@grabner-graz.at)

// Requires: sound/rawmidi.h
// Requires: midibuf.h

pub const MIDI_BUFFER_SIZE: usize = 1024;

#[repr(C)]
pub struct snd_line6_midi {
    // Pointer back to the Line 6 driver data structure
    pub line6: *mut usb_line6,

    // MIDI substream for receiving (or NULL if not active)
    pub substream_receive: *mut snd_rawmidi_substream,

    // MIDI substream for transmitting (or NULL if not active)
    pub substream_transmit: *mut snd_rawmidi_substream,

    // Number of currently active MIDI send URBs
    pub num_active_send_urbs: i32,

    // Spin lock to protect MIDI buffer handling
    pub lock: spinlock_t,

    // Wait queue for MIDI transmission
    pub send_wait: wait_queue_head_t,

    // Buffer for incoming MIDI stream
    pub midibuf_in: midi_buffer,

    // Buffer for outgoing MIDI stream
    pub midibuf_out: midi_buffer,
}

extern "C" {
    pub fn line6_init_midi(line6: *mut usb_line6) -> i32;
    pub fn line6_midi_receive(line6: *mut usb_line6, data: *mut u8, length: i32);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
