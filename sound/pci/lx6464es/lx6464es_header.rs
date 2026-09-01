/* SPDX-License-Identifier: GPL-2.0-or-later */
/* -*- linux-c -*- *
 *
 * ALSA driver for the digigram lx6464es interface
 *
 * Copyright (c) 2009 Tim Blechmann <tim@klingt.org>
 */

/* Original C dependencies:
 * <linux/spinlock.h>
 * <linux/atomic.h>
 * <sound/core.h>
 * <sound/pcm.h>
 * "lx_core.h"
 */

pub const LXP: &[u8] = b"LX6464ES: \0";

pub const ES_cmd_free: i32 = 0; /* no command executing */
pub const ES_cmd_processing: i32 = 1; /* execution of a read/write command */
pub const ES_read_pending: i32 = 2; /* a asynchron read command is pending */
pub const ES_read_finishing: i32 = 3; /* a read command has finished waiting (set by
                                       * Interrupt or CancelIrp) */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum lx_stream_status {
    LX_STREAM_STATUS_FREE,
    /* 	LX_STREAM_STATUS_OPEN, */
    LX_STREAM_STATUS_SCHEDULE_RUN,
    /* 	LX_STREAM_STATUS_STARTED, */
    LX_STREAM_STATUS_RUNNING,
    LX_STREAM_STATUS_SCHEDULE_STOP,
    /* 	LX_STREAM_STATUS_STOPPED, */
    /* 	LX_STREAM_STATUS_PAUSED */
}

#[repr(C)]
pub struct lx_stream {
    pub stream: *mut snd_pcm_substream,
    pub frame_pos: snd_pcm_uframes_t,
    pub status: lx_stream_status, /* free, open, running, draining
                                   * pause */
    /* C bitfield: unsigned int is_capture:1; */
    pub is_capture: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct lx6464es {
    pub card: *mut snd_card,
    pub pci: *mut pci_dev,
    pub irq: ::core::ffi::c_int,

    pub mac_address: [u8; 6],

    pub lock: mutex, /* interrupt lock */
    pub setup_mutex: mutex, /* mutex used in hw_params, open
                             * and close */

    /* ports */
    pub port_plx: ::core::ffi::c_ulong, /* io port (size=256) */
    pub port_plx_remapped: *mut ::core::ffi::c_void, /* remapped plx port */
    pub port_dsp_bar: *mut ::core::ffi::c_void, /* memory port (32-bit,
                                                 * non-prefetchable,
                                                 * size=8K) */

    /* messaging */
    pub msg_lock: mutex, /* message lock */
    pub rmh: lx_rmh,
    pub irqsrc: u32,

    /* configuration */
    /* C bitfield: uint freq_ratio : 2; */
    pub freq_ratio: ::core::ffi::c_uint,
    /* C bitfield: uint playback_mute : 1; */
    pub playback_mute: ::core::ffi::c_uint,
    pub hardware_running: [::core::ffi::c_uint; 2],
    pub board_sample_rate: u32, /* sample rate read from
                                 * board */
    pub pcm_granularity: u16, /* board blocksize */

    /* dma */
    pub capture_dma_buf: snd_dma_buffer,
    pub playback_dma_buf: snd_dma_buffer,

    /* pcm */
    pub pcm: *mut snd_pcm,

    /* streams */
    pub capture_stream: lx_stream,
    pub playback_stream: lx_stream,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
