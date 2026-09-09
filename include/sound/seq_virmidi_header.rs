/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *  Virtual Raw MIDI client on Sequencer
 *  Copyright (c) 2000 by Takashi Iwai <tiwai@suse.de>,
 *                        Jaroslav Kysela <perex@perex.cz>
 */

/* External types are supplied by the corresponding sound subsystem headers. */

/*
 * device file instance:
 * This instance is created at each time the midi device file is
 * opened.  Each instance has its own input buffer and MIDI parser
 * (buffer), and is associated with the device instance.
 */
#[repr(C)]
pub struct snd_virmidi {
    pub list: list_head,
    pub seq_mode: ::core::ffi::c_int,
    pub client: ::core::ffi::c_int,
    pub port: ::core::ffi::c_int,
    pub trigger: bool,
    pub parser: *mut snd_midi_event,
    pub event: snd_seq_event,
    pub rdev: *mut snd_virmidi_dev,
    pub substream: *mut snd_rawmidi_substream,
    pub output_work: work_struct,
}

pub const SNDRV_VIRMIDI_SUBSCRIBE: ::core::ffi::c_uint = 1 << 0;
pub const SNDRV_VIRMIDI_USE: ::core::ffi::c_uint = 1 << 1;

/*
 * device record:
 * Each virtual midi device has one device instance.  It contains
 * common information and the linked-list of opened files,
 */
#[repr(C)]
pub struct snd_virmidi_dev {
    pub card: *mut snd_card,             /* associated card */
    pub rmidi: *mut snd_rawmidi,         /* rawmidi device */
    pub seq_mode: ::core::ffi::c_int,    /* SNDRV_VIRMIDI_XXX */
    pub device: ::core::ffi::c_int,      /* sequencer device */
    pub client: ::core::ffi::c_int,      /* created/attached client */
    pub port: ::core::ffi::c_int,        /* created/attached port */
    pub flags: ::core::ffi::c_uint,      /* SNDRV_VIRMIDI_* */
    pub filelist_sem: rw_semaphore,
    pub filelist: list_head,
}

/* sequencer mode:
 * ATTACH = input/output events from midi device are routed to the
 *          attached sequencer port.  sequencer port is not created
 *          by virmidi itself.
 *          the input to rawmidi must be processed by passing the
 *          incoming events via snd_virmidi_receive()
 * DISPATCH = input/output events are routed to subscribers.
 *            sequencer port is created in virmidi.
 */
pub const SNDRV_VIRMIDI_SEQ_NONE: ::core::ffi::c_int = 0;
pub const SNDRV_VIRMIDI_SEQ_ATTACH: ::core::ffi::c_int = 1;
pub const SNDRV_VIRMIDI_SEQ_DISPATCH: ::core::ffi::c_int = 2;

unsafe extern "C" {
    pub fn snd_virmidi_new(
        card: *mut snd_card,
        device: ::core::ffi::c_int,
        rrmidi: *mut *mut snd_rawmidi,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
