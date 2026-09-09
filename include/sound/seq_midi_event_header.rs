/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *  MIDI byte <-> sequencer event coder
 *
 *  Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>,
 *                        Jaroslav Kysela <perex@perex.cz>
 */

/* Dependency supplied by the surrounding kernel translation. */

pub const MAX_MIDI_EVENT_BUF: usize = 256;

/* midi status */
#[repr(C)]
pub struct snd_midi_event {
    pub qlen: core::ffi::c_int,             /* queue length */
    pub read: core::ffi::c_int,             /* chars read */
    pub r#type: core::ffi::c_int,           /* current event type */
    pub lastcmd: u8,                        /* last command (for MIDI state handling) */
    pub nostat: u8,                         /* no state flag */
    pub bufsize: core::ffi::c_int,          /* allocated buffer size */
    pub buf: *mut u8,                       /* input buffer */
    pub lock: spinlock_t,
}

extern "C" {
    pub fn snd_midi_event_new(
        bufsize: core::ffi::c_int,
        rdev: *mut *mut snd_midi_event,
    ) -> core::ffi::c_int;
    pub fn snd_midi_event_free(dev: *mut snd_midi_event);
    pub fn snd_midi_event_reset_encode(dev: *mut snd_midi_event);
    pub fn snd_midi_event_reset_decode(dev: *mut snd_midi_event);
    pub fn snd_midi_event_no_status(dev: *mut snd_midi_event, on: core::ffi::c_int);
    pub fn snd_midi_event_encode_byte(
        dev: *mut snd_midi_event,
        c: u8,
        ev: *mut snd_seq_event,
    ) -> bool;
    /* decode from event to bytes - return number of written bytes if success */
    pub fn snd_midi_event_decode(
        dev: *mut snd_midi_event,
        buf: *mut u8,
        count: core::ffi::c_long,
        ev: *mut snd_seq_event,
    ) -> core::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
