// SPDX-License-Identifier: GPL-2.0-or-later
//
// Linux driver for TerraTec DMX 6Fire USB
//
// Author: Torsten Schenk <torsten.schenk@zoho.com>
// Created: Jan 01, 2011
// Copyright: (C) Torsten Schenk

// Requires: <sound/pcm.h>, <linux/mutex.h>, "common.h"

pub const PCM_N_URBS: usize = 16;
pub const PCM_N_PACKETS_PER_URB: usize = 8;
pub const PCM_MAX_PACKET_SIZE: usize = 604;

#[repr(C)]
pub struct pcm_urb {
    pub chip: *mut sfire_chip,
    // BEGIN DO NOT SEPARATE
    pub instance: urb,
    pub packets: [usb_iso_packet_descriptor; PCM_N_PACKETS_PER_URB],
    // END DO NOT SEPARATE
    pub buffer: *mut u8,
    pub peer: *mut pcm_urb,
}

#[repr(C)]
pub struct pcm_substream {
    pub lock: spinlock_t,
    pub instance: *mut snd_pcm_substream,
    pub active: bool,
    pub dma_off: snd_pcm_uframes_t,
    pub period_off: snd_pcm_uframes_t,
}

#[repr(C)]
pub struct pcm_runtime {
    pub chip: *mut sfire_chip,
    pub instance: *mut snd_pcm,
    pub playback: pcm_substream,
    pub capture: pcm_substream,
    pub panic: bool,
    pub in_urbs: [pcm_urb; PCM_N_URBS],
    pub out_urbs: [pcm_urb; PCM_N_URBS],
    pub in_packet_size: i32,
    pub out_packet_size: i32,
    pub in_n_analog: i32,
    pub out_n_analog: i32,
    pub stream_mutex: mutex,
    pub stream_state: u8,
    pub rate: u8,
    pub stream_wait_queue: wait_queue_head_t,
    pub stream_wait_cond: bool,
}

extern "C" {
    pub fn usb6fire_pcm_init(chip: *mut sfire_chip) -> i32;
    pub fn usb6fire_pcm_abort(chip: *mut sfire_chip);
    pub fn usb6fire_pcm_destroy(chip: *mut sfire_chip);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
