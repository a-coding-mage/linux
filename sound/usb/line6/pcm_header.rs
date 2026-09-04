// SPDX-License-Identifier: GPL-2.0-only
//
// Line 6 Linux USB driver
//
// Copyright (C) 2004-2010 Markus Grabner (line6@grabner-graz.at)

//! PCM interface to POD series devices.

// Dependencies: <sound/pcm.h>, "driver.h"
// External types: snd_pcm_hardware, snd_pcm_hw_constraint_ratdens, snd_pcm_uframes_t,
// snd_pcm, snd_pcm_substream, snd_pcm_hw_params, usb_line6, urb, spinlock_t, mutex

// number of USB frames per URB
// The Line 6 Windows driver always transmits two frames per packet, but
// the Linux driver performs significantly better (i.e., lower latency)
// with only one frame per packet.
pub const LINE6_ISO_PACKETS: u32 = 1;

// in a "full speed" device (such as the PODxt Pro) this means 1ms,
// for "high speed" it's 1/8ms
pub const LINE6_ISO_INTERVAL: u32 = 1;

pub const LINE6_IMPULSE_DEFAULT_PERIOD: u32 = 100;

// Get substream from Line 6 PCM data structure
// Original C macro: get_substream(line6pcm, stream)
// Returns: line6pcm->pcm->streams[stream].substream
// Note: This macro depends on snd_pcm internal layout and cannot be translated locally.
// Use as: unsafe { (*(*line6pcm.line6).pcm).streams[stream].substream }
#[macro_export]
macro_rules! get_substream {
    ($line6pcm:expr, $stream:expr) => {
        unsafe { (*(*($line6pcm).pcm)).streams[($stream)].substream }
    };
}

// PCM mode bits.
//
// There are several features of the Line 6 USB driver which require PCM
// data to be exchanged with the device:
// *) PCM playback and capture via ALSA
// *) software monitoring (for devices without hardware monitoring)
// *) optional impulse response measurement
// However, from the device's point of view, there is just a single
// capture and playback stream, which must be shared between these
// subsystems. It is therefore necessary to maintain the state of the
// subsystems with respect to PCM usage.
//
// We define two bit flags, "opened" and "running", for each playback
// or capture stream.  Both can contain the bit flag corresponding to
// LINE6_STREAM_* type,
//   LINE6_STREAM_PCM = ALSA PCM playback or capture
//   LINE6_STREAM_MONITOR = software monitoring
//   IMPULSE = optional impulse response measurement
// The opened flag indicates whether the buffer is allocated while
// the running flag indicates whether the stream is running.
//
// For monitor or impulse operations, the driver needs to call
// line6_pcm_acquire() or line6_pcm_release() with the appropriate
// LINE6_STREAM_* flag.

// stream types
pub const LINE6_STREAM_PCM: u32 = 0;
pub const LINE6_STREAM_MONITOR: u32 = 1;
pub const LINE6_STREAM_IMPULSE: u32 = 2;
pub const LINE6_STREAM_CAPTURE_HELPER: u32 = 3;

// misc bit flags for PCM operation
pub const LINE6_FLAG_PAUSE_PLAYBACK: u32 = 0;
pub const LINE6_FLAG_PREPARED: u32 = 1;

// External type stubs (actual definitions from dependencies)
pub use crate::sound_pcm::{
    snd_pcm, snd_pcm_hardware, snd_pcm_hw_constraint_ratdens, snd_pcm_hw_params,
    snd_pcm_substream, snd_pcm_uframes_t,
};
pub use crate::driver::{usb_line6};
pub use crate::urb::{urb};
pub use crate::kernel_sync::{mutex, spinlock_t};

#[repr(C)]
pub struct line6_pcm_properties {
    pub playback_hw: snd_pcm_hardware,
    pub capture_hw: snd_pcm_hardware,
    pub rates: snd_pcm_hw_constraint_ratdens,
    pub bytes_per_channel: i32,
}

#[repr(C)]
pub struct line6_pcm_stream {
    // allocated URBs
    pub urbs: *mut *mut urb,

    // Temporary buffer;
    // Since the packet size is not known in advance, this buffer is
    // large enough to store maximum size packets.
    pub buffer: *mut u8,

    // Free frame position in the buffer.
    pub pos: snd_pcm_uframes_t,

    // Count processed bytes;
    // This is modulo period size (to determine when a period is finished).
    pub bytes: u32,

    // Counter to create desired sample rate
    pub count: u32,

    // period size in bytes
    pub period: u32,

    // Processed frame position in the buffer;
    // The contents of the ring buffer have been consumed by the USB
    // subsystem (i.e., sent to the USB device) up to this position.
    pub pos_done: snd_pcm_uframes_t,

    // Bit mask of active URBs
    pub active_urbs: usize,

    // Bit mask of URBs currently being unlinked
    pub unlink_urbs: usize,

    // Spin lock to protect updates of the buffer positions (not contents)
    pub lock: spinlock_t,

    // Bit flags for operational stream types
    pub opened: usize,

    // Bit flags for running stream types
    pub running: usize,

    pub last_frame: i32,
}

#[repr(C)]
pub struct snd_line6_pcm {
    // Pointer back to the Line 6 driver data structure
    pub line6: *mut usb_line6,

    // Properties.
    pub properties: *mut line6_pcm_properties,

    // ALSA pcm stream
    pub pcm: *mut snd_pcm,

    // protection to state changes of in/out streams
    pub state_mutex: mutex,

    // Capture and playback streams
    pub r#in: line6_pcm_stream,
    pub out: line6_pcm_stream,

    // Previously captured frame (for software monitoring)
    pub prev_fbuf: *mut u8,

    // Size of previously captured frame (for software monitoring/sync)
    pub prev_fsize: i32,

    // Maximum size of USB packet
    pub max_packet_size_in: i32,
    pub max_packet_size_out: i32,

    // PCM playback volume (left and right)
    pub volume_playback: [i32; 2],

    // PCM monitor volume
    pub volume_monitor: i32,

    // Volume of impulse response test signal (if zero, test is disabled)
    pub impulse_volume: i32,

    // Period of impulse response test signal
    pub impulse_period: i32,

    // Counter for impulse response test signal
    pub impulse_count: i32,

    // Several status bits (see LINE6_FLAG_*)
    pub flags: usize,
}

extern "C" {
    pub fn line6_init_pcm(
        line6: *mut usb_line6,
        properties: *mut line6_pcm_properties,
    ) -> i32;
    pub fn snd_line6_trigger(substream: *mut snd_pcm_substream, cmd: i32) -> i32;
    pub fn snd_line6_prepare(substream: *mut snd_pcm_substream) -> i32;
    pub fn snd_line6_hw_params(
        substream: *mut snd_pcm_substream,
        hw_params: *mut snd_pcm_hw_params,
    ) -> i32;
    pub fn snd_line6_hw_free(substream: *mut snd_pcm_substream) -> i32;
    pub fn snd_line6_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t;
    pub fn line6_pcm_disconnect(line6pcm: *mut snd_line6_pcm);
    pub fn line6_pcm_acquire(
        line6pcm: *mut snd_line6_pcm,
        r#type: i32,
        start: bool,
    ) -> i32;
    pub fn line6_pcm_release(line6pcm: *mut snd_line6_pcm, r#type: i32);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
