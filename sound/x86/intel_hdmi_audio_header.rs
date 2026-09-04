// SPDX-License-Identifier: MIT
//
// Copyright (C) 2016 Intel Corporation
//  Authors: Sailaja Bandarupalli <sailaja.bandarupalli@intel.com>
//          Ramesh Babu K V <ramesh.babu@intel.com>
//          Vaibhav Agarwal <vaibhav.agarwal@intel.com>
//          Jerome Anand <jerome.anand@intel.com>

// Requires: intel_hdmi_lpe_audio.h (external kernel header)

use std::ffi::c_void;

pub const MAX_PB_STREAMS: usize = 1;
pub const MAX_CAP_STREAMS: usize = 0;
pub const BYTES_PER_WORD: u32 = 0x4;
pub const INTEL_HAD: &str = "HdmiLpeAudio";

//
// CEA speaker placement:
//
// FL  FLC   FC   FRC   FR
//
//                         LFE
//
// RL  RLC   RC   RRC   RR
//
// The Left/Right Surround channel _notions_ LS/RS in SMPTE 320M
// corresponds to CEA RL/RR; The SMPTE channel _assignment_ C/LFE is
// swapped to CEA LFE/FC.
//

#[repr(C)]
pub enum cea_speaker_placement {
    FL = (1 << 0),        // Front Left
    FC = (1 << 1),        // Front Center
    FR = (1 << 2),        // Front Right
    FLC = (1 << 3),       // Front Left Center
    FRC = (1 << 4),       // Front Right Center
    RL = (1 << 5),        // Rear Left
    RC = (1 << 6),        // Rear Center
    RR = (1 << 7),        // Rear Right
    RLC = (1 << 8),       // Rear Left Center
    RRC = (1 << 9),       // Rear Right Center
    LFE = (1 << 10),      // Low Frequency Effect
}

#[repr(C)]
pub struct cea_channel_speaker_allocation {
    pub ca_index: i32,
    pub speakers: [i32; 8],

    // derived values, just for convenience
    pub channels: i32,
    pub spk_mask: i32,
}

#[repr(C)]
pub struct channel_map_table {
    pub map: u8,              // ALSA API channel map position
    pub cea_slot: u8,         // CEA slot value
    pub spk_mask: i32,        // speaker position bit mask
}

#[repr(C)]
pub struct pcm_stream_info {
    pub substream: *mut snd_pcm_substream,
    pub substream_refcount: i32,
}

//
// struct snd_intelhad - intelhad driver structure
//
// @card_ctx: ptr to hold card details
// @connected: the monitor connection status
// @stream_info: stream information
// @eld: holds ELD info
// @curr_buf: pointer to hold current active ring buf
// @valid_buf_cnt: ring buffer count for stream
// @had_spinlock: driver lock
// @aes_bits: IEC958 status bits
// @buff_done: id of current buffer done intr
// @dev: platform device handle
// @chmap: holds channel map info
//

#[repr(C)]
pub struct snd_intelhad {
    pub card_ctx: *mut snd_intelhad_card,
    pub connected: bool,
    pub stream_info: pcm_stream_info,
    pub eld: [u8; HDMI_MAX_ELD_BYTES],
    pub dp_output: bool,
    pub aes_bits: u32,
    pub had_spinlock: spinlock_t,
    pub dev: *mut device,
    pub chmap: *mut snd_pcm_chmap,
    pub tmds_clock_speed: i32,
    pub link_rate: i32,
    pub port: i32,    // fixed
    pub pipe: i32,    // can change dynamically

    // ring buffer (BD) position index
    pub bd_head: u32,
    // PCM buffer position indices
    pub pcmbuf_head: u32,     // being processed
    pub pcmbuf_filled: u32,   // to be filled

    pub num_bds: u32,         // number of BDs
    pub period_bytes: u32,    // PCM period size in bytes

    // internal stuff
    pub aud_config: aud_cfg,  // AUD_CONFIG reg value cache
    pub hdmi_audio_wq: work_struct,
    pub mutex: mutex,         // for protecting chmap and eld
    pub jack: *mut snd_jack,
}

#[repr(C)]
pub struct snd_intelhad_card {
    pub card: *mut snd_card,
    pub dev: *mut device,

    // internal stuff
    pub irq: i32,
    pub mmio_start: *mut c_void,
    pub num_pipes: i32,
    pub num_ports: i32,
    pub pcm_ctx: [snd_intelhad; 3],  // one for each port
}

// External opaque types from intel_hdmi_lpe_audio.h and kernel headers
pub struct snd_pcm_substream;
pub struct spinlock_t;
pub struct device;
pub struct snd_pcm_chmap;
pub struct work_struct;
pub struct mutex;
pub struct snd_jack;
pub struct snd_card;
pub struct aud_cfg;

// External constant from intel_hdmi_lpe_audio.h
// Must be available at compile time from the external header
#[allow(non_upper_case_globals)]
pub const HDMI_MAX_ELD_BYTES: usize = 256;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
