// SPDX-License-Identifier: GPL-2.0
// Translated from C header: usbusx2y.h
// Dependencies: ../usbaudio.h, ../midi.h, usbus428ctldefs.h, usx2yhwdeppcm.h

use std::ffi::{c_char, c_int, c_uint, c_void};
use std::sync::atomic::AtomicI32;

pub const NRURBS: usize = 2;

// Default value used for nr of packs per urb.
// 1 to 4 have been tested ok on uhci.
// To use 3 on ohci, you'd need a patch:
// look for "0000425-linux-2.6.9-rc4-mm1_ohci-hcd.patch.gz" on
// "https://bugtrack.alsa-project.org/alsa-bug/bug_view_page.php?bug_id=0000425"
//
// 1, 2 and 4 work out of the box on ohci, if I recall correctly.
// Bigger is safer operation, smaller gives lower latencies.
pub const USX2Y_NRPACKS: usize = 4;

pub const USX2Y_NRPACKS_MAX: usize = 1024;

// If your system works ok with this module's parameter
// nrpacks set to 1, you might as well comment
// this define out, and thereby produce smaller, faster code.
// You'd also set USX2Y_NRPACKS to 1 then.
// Note: USX2Y_NRPACKS_VARIABLE is defined; using dynamic nrpacks

extern "C" {
    pub static nrpacks: c_int;
}

// Runtime-configurable number of packs (when USX2Y_NRPACKS_VARIABLE is set)
#[inline]
pub fn nr_of_packs() -> c_int {
    unsafe { nrpacks }
}

pub const URBS_ASYNC_SEQ: usize = 10;
pub const URB_DATA_LEN_ASYNC_SEQ: usize = 32;

#[repr(C)]
pub struct snd_usx2y_async_seq {
    pub urb: [*mut c_void; URBS_ASYNC_SEQ],
    pub buffer: *mut c_char,
}

#[repr(C)]
pub struct snd_usx2y_urb_seq {
    pub submitted: c_int,
    pub len: c_int,
    pub urb: [*mut c_void; 0],
}

// Include from usx2yhwdeppcm.h is a dependency

#[repr(C)]
pub struct usx2ydev {
    pub dev: *mut c_void,
    pub card_index: c_int,
    pub stride: c_int,
    pub in04_urb: *mut c_void,
    pub in04_buf: *mut c_void,
    pub in04_last: [c_char; 24],
    pub in04_int_calls: c_uint,
    pub us04: *mut snd_usx2y_urb_seq,
    pub in04_wait_queue: c_void,
    pub as04: snd_usx2y_async_seq,
    pub rate: c_uint,
    pub format: c_uint,
    pub chip_status: c_int,
    pub pcm_mutex: c_void,
    pub us428ctls_sharedmem: *mut c_void,
    pub wait_iso_frame: c_int,
    pub us428ctls_wait_queue_head: c_void,
    pub hwdep_pcm_shm: *mut c_void,
    pub subs: [*mut snd_usx2y_substream; 4],
    pub prepare_subs: *mut snd_usx2y_substream,
    pub prepare_wait_queue: c_void,
    pub midi_list: c_void,
    pub pcm_devs: c_int,
}

pub const STATE_STOPPED: c_int = 0;
pub const STATE_STARTING1: c_int = 1;
pub const STATE_STARTING2: c_int = 2;
pub const STATE_STARTING3: c_int = 3;
pub const STATE_PREPARED: c_int = 4;
pub const STATE_PRERUNNING: c_int = 6;
pub const STATE_RUNNING: c_int = 8;

#[repr(C)]
pub struct snd_usx2y_substream {
    pub usx2y: *mut usx2ydev,
    pub pcm_substream: *mut c_void,

    pub endpoint: c_int,
    pub maxpacksize: c_uint,

    pub state: AtomicI32,

    pub hwptr: c_int,
    pub hwptr_done: c_int,
    pub transfer_done: c_int,

    pub urb: [*mut c_void; NRURBS],
    pub completed_urb: *mut c_void,
    pub tmpbuf: *mut c_char,
}

// Macro translation: usx2y(c) -> ((struct usx2ydev *)(c)->private_data)
#[inline]
pub fn usx2y(c: *const c_void) -> *mut usx2ydev {
    unsafe {
        let card = c as *const *mut usx2ydev;
        *card
    }
}

pub const NAME_ALLCAPS: &str = "US-X2Y";

extern "C" {
    pub fn usx2y_audio_create(card: *mut c_void) -> c_int;

    pub fn usx2y_async_seq04_init(usx2y: *mut usx2ydev) -> c_int;
    pub fn usx2y_in04_init(usx2y: *mut usx2ydev) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
