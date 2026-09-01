/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Driver for Digigram miXart soundcards
 *
 * main header file
 *
 * Copyright (c) 2003 by Digigram <alsa@digigram.com>
 */

// Original C dependencies:
// #include <linux/interrupt.h>
// #include <linux/mutex.h>
// #include <sound/pcm.h>

use core::ffi::c_void;

use crate::{
    atomic_t, pci_dev, resource, snd_card, snd_dma_buffer, snd_hwdep, snd_pcm,
    snd_pcm_substream, wait_queue_head_t, mutex,
};

pub const MIXART_DRIVER_VERSION: u32 = 0x000100; /* 0.1.0 */

/*
 */

#[repr(C)]
pub struct mixart_uid {
    pub object_id: u32,
    pub desc: u32,
}

#[repr(C)]
pub struct mem_area {
    pub phys: libc::c_ulong,
    pub virt: *mut c_void, /* __iomem */
    pub res: *mut resource,
}

#[repr(C)]
pub struct mixart_route {
    pub connected: u8,
    pub phase_inv: u8,
    pub volume: libc::c_int,
}

/* firmware status codes  */
pub const MIXART_MOTHERBOARD_XLX_INDEX: usize = 0;
pub const MIXART_MOTHERBOARD_ELF_INDEX: usize = 1;
pub const MIXART_AESEBUBOARD_XLX_INDEX: usize = 2;
pub const MIXART_HARDW_FILES_MAX_INDEX: usize = 3; /* xilinx, elf, AESEBU xilinx */

pub const MIXART_MAX_CARDS: usize = 4;
pub const MSG_FIFO_SIZE: usize = 16;

pub const MIXART_MAX_PHYS_CONNECTORS: usize = MIXART_MAX_CARDS * 2 * 2; /* 4 * stereo * (analog+digital) */

#[repr(C)]
pub struct mixart_mgr {
    pub num_cards: libc::c_uint,
    pub chip: [*mut snd_mixart; MIXART_MAX_CARDS],

    pub pci: *mut pci_dev,

    pub irq: libc::c_int,

    /* memory-maps */
    pub mem: [mem_area; 2],

    /* one and only blocking message or notification may be pending  */
    pub pending_event: u32,
    pub msg_sleep: wait_queue_head_t,

    /* messages fifo */
    pub msg_fifo: [u32; MSG_FIFO_SIZE],
    pub msg_fifo_readptr: libc::c_int,
    pub msg_fifo_writeptr: libc::c_int,
    pub msg_processed: atomic_t,       /* number of messages to be processed in irq thread */

    pub lock: mutex,              /* interrupt lock */
    pub msg_lock: mutex,          /* mailbox lock */

    pub setup_mutex: mutex, /* mutex used in hw_params, open and close */

    /* hardware interface */
    pub dsp_loaded: libc::c_uint,      /* bit flags of loaded dsp indices */
    pub board_type: libc::c_uint,      /* read from embedded once elf file is loaded, 250 = miXart8, 251 = with AES, 252 = with Cobranet */

    pub flowinfo: snd_dma_buffer,
    pub bufferinfo: snd_dma_buffer,

    pub uid_console_manager: mixart_uid,
    pub sample_rate: libc::c_int,
    pub ref_count_rate: libc::c_int,

    pub mixer_mutex: mutex, /* mutex for mixer */
}

pub const MIXART_STREAM_STATUS_FREE: libc::c_int = 0;
pub const MIXART_STREAM_STATUS_OPEN: libc::c_int = 1;
pub const MIXART_STREAM_STATUS_RUNNING: libc::c_int = 2;
pub const MIXART_STREAM_STATUS_DRAINING: libc::c_int = 3;
pub const MIXART_STREAM_STATUS_PAUSE: libc::c_int = 4;

pub const MIXART_PLAYBACK_STREAMS: usize = 4;
pub const MIXART_CAPTURE_STREAMS: usize = 1;

pub const MIXART_PCM_ANALOG: usize = 0;
pub const MIXART_PCM_DIGITAL: usize = 1;
pub const MIXART_PCM_TOTAL: usize = 2;

pub const MIXART_MAX_STREAM_PER_CARD: usize =
    MIXART_PCM_TOTAL * (MIXART_PLAYBACK_STREAMS + MIXART_CAPTURE_STREAMS);

pub const MIXART_NOTIFY_CARD_MASK: u32 = 0xF000;
pub const MIXART_NOTIFY_CARD_OFFSET: u32 = 12;
pub const MIXART_NOTIFY_PCM_MASK: u32 = 0x0F00;
pub const MIXART_NOTIFY_PCM_OFFSET: u32 = 8;
pub const MIXART_NOTIFY_CAPT_MASK: u32 = 0x0080;
pub const MIXART_NOTIFY_SUBS_MASK: u32 = 0x007F;

#[repr(C)]
pub struct mixart_stream {
    pub substream: *mut snd_pcm_substream,
    pub pipe: *mut mixart_pipe,
    pub pcm_number: libc::c_int,

    pub status: libc::c_int,      /* nothing, running, draining */

    pub abs_period_elapsed: u64,  /* last absolute stream position where period_elapsed was called (multiple of runtime->period_size) */
    pub buf_periods: u32,         /* periods counter in the buffer (< runtime->periods) */
    pub buf_period_frag: u32,     /* defines with buf_periods the exact position in the buffer (< runtime->period_size) */

    pub channels: libc::c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum mixart_pipe_status {
    PIPE_UNDEFINED,
    PIPE_STOPPED,
    PIPE_RUNNING,
    PIPE_CLOCK_SET,
}

#[repr(C)]
pub struct mixart_pipe {
    pub group_uid: mixart_uid,            /* id of the pipe, as returned by embedded */
    pub stream_count: libc::c_int,
    pub uid_left_connector: mixart_uid,   /* UID's for the audio connectors */
    pub uid_right_connector: mixart_uid,
    pub status: mixart_pipe_status,
    pub references: libc::c_int,          /* number of subs openned */
    pub monitoring: libc::c_int,          /* pipe used for monitoring issue */
}

#[repr(C)]
pub struct snd_mixart {
    pub card: *mut snd_card,
    pub mgr: *mut mixart_mgr,
    pub chip_idx: libc::c_int,            /* zero based */
    pub hwdep: *mut snd_hwdep,            /* DSP loader, only for the first card */

    pub pcm: *mut snd_pcm,                /* PCM analog i/o */
    pub pcm_dig: *mut snd_pcm,            /* PCM digital i/o */

    /* allocate stereo pipe for instance */
    pub pipe_in_ana: mixart_pipe,
    pub pipe_out_ana: mixart_pipe,

    /* if AES/EBU daughter board is available, additional pipes possible on pcm_dig */
    pub pipe_in_dig: mixart_pipe,
    pub pipe_out_dig: mixart_pipe,

    pub playback_stream: [[mixart_stream; MIXART_PLAYBACK_STREAMS]; MIXART_PCM_TOTAL], /* 0 = pcm, 1 = pcm_dig */
    pub capture_stream: [mixart_stream; MIXART_PCM_TOTAL],                             /* 0 = pcm, 1 = pcm_dig */

    /* UID's for the physical io's */
    pub uid_out_analog_physio: mixart_uid,
    pub uid_in_analog_physio: mixart_uid,

    pub analog_playback_active: [libc::c_int; 2],      /* Mixer : Master Playback active (!mute) */
    pub analog_playback_volume: [libc::c_int; 2],      /* Mixer : Master Playback Volume */
    pub analog_capture_volume: [libc::c_int; 2],       /* Mixer : Master Capture Volume */
    pub digital_playback_active: [[libc::c_int; 2]; 2 * MIXART_PLAYBACK_STREAMS], /* Mixer : Digital Playback Active [(analog+AES output)*streams][stereo]*/
    pub digital_playback_volume: [[libc::c_int; 2]; 2 * MIXART_PLAYBACK_STREAMS], /* Mixer : Digital Playback Volume [(analog+AES output)*streams][stereo]*/
    pub digital_capture_volume: [[libc::c_int; 2]; 2], /* Mixer : Digital Capture Volume [analog+AES output][stereo] */
    pub monitoring_active: [libc::c_int; 2],           /* Mixer : Monitoring Active */
    pub monitoring_volume: [libc::c_int; 2],           /* Mixer : Monitoring Volume */
}

#[repr(C)]
pub struct mixart_bufferinfo {
    pub buffer_address: u32,
    pub reserved: [u32; 5],
    pub available_length: u32,
    pub buffer_id: u32,
}

#[repr(C)]
pub struct mixart_flowinfo {
    pub bufferinfo_array_phy_address: u32,
    pub reserved: [u32; 11],
    pub bufferinfo_count: u32,
    pub capture: u32,
}

/* exported */
unsafe extern "C" {
    pub fn snd_mixart_create_pcm(chip: *mut snd_mixart) -> libc::c_int;
    pub fn snd_mixart_add_ref_pipe(
        chip: *mut snd_mixart,
        pcm_number: libc::c_int,
        capture: libc::c_int,
        monitoring: libc::c_int,
    ) -> *mut mixart_pipe;
    pub fn snd_mixart_kill_ref_pipe(
        mgr: *mut mixart_mgr,
        pipe: *mut mixart_pipe,
        monitoring: libc::c_int,
    ) -> libc::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
