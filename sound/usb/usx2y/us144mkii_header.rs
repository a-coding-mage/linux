// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2025 Šerif Rami <ramiserifpersia@gmail.com>

// Dependencies on Linux kernel types:
// linux/kfifo.h, linux/timer.h, linux/usb.h, linux/workqueue.h,
// sound/control.h, sound/core.h, sound/initval.h, sound/pcm.h, sound/rawmidi.h

pub const DRIVER_NAME: &str = "us144mkii";

// --- USB Device Identification ---
pub const USB_VID_TASCAM: u16 = 0x0644;
pub const USB_PID_TASCAM_US144: u16 = 0x800f;
pub const USB_PID_TASCAM_US144MKII: u16 = 0x8020;

// --- USB Endpoints (Alternate Setting 1) ---
pub const EP_PLAYBACK_FEEDBACK: u8 = 0x81;
pub const EP_AUDIO_OUT: u8 = 0x02;
pub const EP_MIDI_IN: u8 = 0x83;
pub const EP_MIDI_OUT: u8 = 0x04;
pub const EP_AUDIO_IN: u8 = 0x86;

// --- USB Control Message Protocol ---
// These assume external USB_DIR_*, USB_TYPE_*, USB_RECIP_* constants are available
pub const RT_H2D_CLASS_EP: u8 = (0x00 | 0x20 | 0x00); // USB_DIR_OUT | USB_TYPE_CLASS | USB_RECIP_ENDPOINT
pub const RT_D2H_CLASS_EP: u8 = (0x80 | 0x20 | 0x00); // USB_DIR_IN | USB_TYPE_CLASS | USB_RECIP_ENDPOINT
pub const RT_H2D_VENDOR_DEV: u8 = (0x00 | 0x40 | 0x00); // USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE
pub const RT_D2H_VENDOR_DEV: u8 = (0x80 | 0x40 | 0x00); // USB_DIR_IN | USB_TYPE_VENDOR | USB_RECIP_DEVICE

#[repr(u8)]
pub enum UacRequest {
    UacSetCur = 0x01,
    UacGetCur = 0x81,
}

#[repr(u16)]
pub enum UacControlSelector {
    UacSamplingFreqControl = 0x0100,
}

#[repr(u8)]
pub enum TascamVendorRequest {
    VendorReqRegisterWrite = 0x41,
    VendorReqDeepSleep = 0x44,
    VendorReqModeControl = 0x49,
}

#[repr(u16)]
pub enum TascamModeValue {
    ModeValHandshakeRead = 0x0000,
    ModeValConfig = 0x0010,
    ModeValStreamStart = 0x0030,
}

pub const HANDSHAKE_SUCCESS_VAL: u8 = 0x12;

#[repr(u16)]
pub enum TascamRegister {
    RegAddrUnknown0D = 0x0d04,
    RegAddrUnknown0E = 0x0e00,
    RegAddrUnknown0F = 0x0f00,
    RegAddrRate44100 = 0x1000,
    RegAddrRate48000 = 0x1002,
    RegAddrRate88200 = 0x1008,
    RegAddrRate96000 = 0x100a,
    RegAddrUnknown11 = 0x110b,
}

pub const REG_VAL_ENABLE: u16 = 0x0101;

// --- URB Configuration ---
pub const NUM_PLAYBACK_URBS: usize = 4;
pub const PLAYBACK_URB_PACKETS: usize = 8;
pub const NUM_FEEDBACK_URBS: usize = 4;
pub const FEEDBACK_URB_PACKETS: usize = 1;
pub const FEEDBACK_PACKET_SIZE: usize = 3;
pub const NUM_CAPTURE_URBS: usize = 8;
pub const CAPTURE_URB_SIZE: usize = 512;
pub const CAPTURE_RING_BUFFER_SIZE: usize = CAPTURE_URB_SIZE * NUM_CAPTURE_URBS * 4;
pub const NUM_MIDI_IN_URBS: usize = 4;
pub const MIDI_IN_BUF_SIZE: usize = 64;
pub const MIDI_IN_FIFO_SIZE: usize = MIDI_IN_BUF_SIZE * NUM_MIDI_IN_URBS;
pub const MIDI_OUT_BUF_SIZE: usize = 64;
pub const NUM_MIDI_OUT_URBS: usize = 4;
pub const USB_CTRL_TIMEOUT_MS: u32 = 1000;
pub const FEEDBACK_SYNC_LOSS_THRESHOLD: u32 = 41;

// --- Audio Format Configuration ---
pub const BYTES_PER_SAMPLE: usize = 3;
pub const NUM_CHANNELS: usize = 4;
pub const BYTES_PER_FRAME: usize = NUM_CHANNELS * BYTES_PER_SAMPLE;
pub const FEEDBACK_ACCUMULATOR_SIZE: usize = 128;

// --- Capture Decoding Defines ---
pub const DECODED_CHANNELS_PER_FRAME: usize = 4;
pub const DECODED_SAMPLE_SIZE: usize = 4;
pub const FRAMES_PER_DECODE_BLOCK: usize = 8;
pub const RAW_BYTES_PER_DECODE_BLOCK: usize = 512;

/// State for dynamic feedback patterns.
///
/// # Fields
/// * `sample_rate_khz` - The current sample rate in kHz.
/// * `base_feedback_value` - The nominal feedback value for the current rate.
/// * `feedback_offset` - An offset to align the feedback value range.
/// * `full_frame_patterns` - A 2D array of pre-calculated packet size patterns.
/// * `current_index` - The current index into the pattern array.
/// * `previous_index` - The previous index, used for state tracking.
/// * `sync_locked` - A flag indicating if the pattern has locked to the stream.
#[repr(C)]
pub struct Us144mkiiFramePatternObserver {
    pub sample_rate_khz: u32,
    pub base_feedback_value: u32,
    pub feedback_offset: i32,
    pub full_frame_patterns: [[u32; 8]; 5],
    pub current_index: u32,
    pub previous_index: u32,
    pub sync_locked: u8,
}

/// Main driver data structure for the TASCAM US-144MKII.
///
/// # Fields
/// * `dev` - Pointer to the USB device.
/// * `iface0` - Pointer to USB interface 0 (audio).
/// * `iface1` - Pointer to USB interface 1 (MIDI).
/// * `card` - Pointer to the ALSA sound card instance.
/// * `pcm` - Pointer to the ALSA PCM device.
/// * `rmidi` - Pointer to the ALSA rawmidi device.
/// * `playback_substream` - Pointer to the active playback PCM substream.
/// * `playback_urbs` - Array of URBs for playback.
/// * `playback_urb_alloc_size` - Size of allocated buffer for each playback URB.
/// * `feedback_urbs` - Array of URBs for feedback.
/// * `feedback_urb_alloc_size` - Size of allocated buffer for each feedback URB.
/// * `playback_active` - Atomic flag indicating if playback is active.
/// * `playback_frames_consumed` - Total frames consumed by playback.
/// * `driver_playback_pos` - Current position in the ALSA playback buffer (frames).
/// * `last_period_pos` - Last reported period position for playback.
/// * `capture_substream` - Pointer to the active capture PCM substream.
/// * `capture_urbs` - Array of URBs for capture.
/// * `capture_urb_alloc_size` - Size of allocated buffer for each capture URB.
/// * `capture_active` - Atomic flag indicating if capture is active.
/// * `driver_capture_pos` - Current position in the ALSA capture buffer (frames).
/// * `capture_frames_processed` - Total frames processed for capture.
/// * `last_capture_period_pos` - Last reported period position for capture.
/// * `capture_ring_buffer` - Ring buffer for raw capture data from USB.
/// * `capture_ring_buffer_read_ptr` - Read pointer for the capture ring buffer.
/// * `capture_ring_buffer_write_ptr` - Write pointer for the capture ring buffer.
/// * `capture_decode_raw_block` - Buffer for a raw 512-byte capture block.
/// * `capture_decode_dst_block` - Buffer for decoded 32-bit capture samples.
/// * `capture_routing_buffer` - Intermediate buffer for capture routing.
/// * `capture_work` - Work struct for deferred capture processing.
/// * `stop_work` - Work struct for deferred stream stopping.
/// * `stop_pcm_work` - Work struct for stopping PCM due to a fatal error (e.g. xrun).
/// * `midi_in_substream` - Pointer to the active MIDI input substream.
/// * `midi_out_substream` - Pointer to the active MIDI output substream.
/// * `midi_in_urbs` - Array of URBs for MIDI input.
/// * `midi_out_urbs` - Array of URBs for MIDI output.
/// * `midi_in_active` - Atomic flag indicating if MIDI input is active.
/// * `midi_out_active` - Atomic flag indicating if MIDI output is active.
/// * `midi_in_fifo` - FIFO for raw MIDI input data.
/// * `midi_in_work` - Work struct for deferred MIDI input processing.
/// * `midi_out_work` - Work struct for deferred MIDI output processing.
/// * `midi_in_lock` - Spinlock for MIDI input FIFO.
/// * `midi_out_lock` - Spinlock for MIDI output.
/// * `midi_out_urbs_in_flight` - Bitmap of MIDI output URBs currently in flight.
/// * `midi_running_status` - Stores the last MIDI status byte for running status.
/// * `error_timer` - Timer for MIDI error retry logic.
/// * `lock` - Main spinlock for protecting shared driver state.
/// * `active_urbs` - Atomic counter for active URBs.
/// * `current_rate` - Currently configured sample rate of the device.
/// * `line_out_source` - Source for Line Outputs (0: Playback 1-2, 1: Playback 3-4).
/// * `digital_out_source` - Source for Digital Outputs (0: Playback 1-2, 1: Playback 3-4).
/// * `capture_12_source` - Source for Capture channels 1-2 (0: Analog In, 1: Digital In).
/// * `capture_34_source` - Source for Capture channels 3-4 (0: Analog In, 1: Digital In).
/// * `feedback_accumulator_pattern` - Stores the calculated frames per packet for feedback.
/// * `feedback_pattern_out_idx` - Read index for feedback_accumulator_pattern.
/// * `feedback_pattern_in_idx` - Write index for feedback_accumulator_pattern.
/// * `feedback_synced` - Flag indicating if feedback is synced.
/// * `feedback_consecutive_errors` - Counter for consecutive feedback errors.
/// * `feedback_urb_skip_count` - Number of feedback URBs to skip initially for stabilization.
/// * `fpo` - Holds the state for the dynamic feedback pattern generation.
/// * `playback_anchor` - USB anchor for playback URBs.
/// * `capture_anchor` - USB anchor for capture URBs.
/// * `feedback_anchor` - USB anchor for feedback URBs.
/// * `midi_in_anchor` - USB anchor for MIDI input URBs.
/// * `midi_out_anchor` - USB anchor for MIDI output URBs.
#[repr(C)]
pub struct TascamCard {
    // --- Core device pointers ---
    pub dev: *mut core::ffi::c_void,
    pub iface0: *mut core::ffi::c_void,
    pub iface1: *mut core::ffi::c_void,
    pub card: *mut core::ffi::c_void,
    pub pcm: *mut core::ffi::c_void,
    pub rmidi: *mut core::ffi::c_void,

    // --- PCM Substreams ---
    pub playback_substream: *mut core::ffi::c_void,
    pub capture_substream: *mut core::ffi::c_void,

    // --- URBs and Anchors ---
    pub playback_urbs: [*mut core::ffi::c_void; NUM_PLAYBACK_URBS],
    pub playback_urb_alloc_size: usize,
    pub feedback_urbs: [*mut core::ffi::c_void; NUM_FEEDBACK_URBS],
    pub feedback_urb_alloc_size: usize,
    pub capture_urbs: [*mut core::ffi::c_void; NUM_CAPTURE_URBS],
    pub capture_urb_alloc_size: usize,
    pub midi_in_urbs: [*mut core::ffi::c_void; NUM_MIDI_IN_URBS],
    pub midi_out_urbs: [*mut core::ffi::c_void; NUM_MIDI_OUT_URBS],
    pub playback_anchor: core::ffi::c_void,
    pub capture_anchor: core::ffi::c_void,
    pub feedback_anchor: core::ffi::c_void,
    pub midi_in_anchor: core::ffi::c_void,
    pub midi_out_anchor: core::ffi::c_void,

    // --- Stream State ---
    pub lock: core::ffi::c_void,
    pub playback_active: i32,
    pub capture_active: i32,
    pub active_urbs: i32,
    pub current_rate: i32,

    // --- Playback State ---
    pub playback_frames_consumed: u64,
    pub driver_playback_pos: u32,
    pub last_period_pos: u64,

    // --- Capture State ---
    pub capture_frames_processed: u64,
    pub driver_capture_pos: u32,
    pub last_capture_period_pos: u64,
    pub capture_ring_buffer: *mut u8,
    pub capture_ring_buffer_read_ptr: usize,
    pub capture_ring_buffer_write_ptr: usize,
    pub capture_decode_raw_block: *mut u8,
    pub capture_decode_dst_block: *mut i32,
    pub capture_routing_buffer: *mut i32,

    // --- MIDI State ---
    pub midi_in_substream: *mut core::ffi::c_void,
    pub midi_out_substream: *mut core::ffi::c_void,
    pub midi_in_active: i32,
    pub midi_out_active: i32,
    pub midi_in_fifo: core::ffi::c_void,
    pub midi_in_lock: core::ffi::c_void,
    pub midi_out_lock: core::ffi::c_void,
    pub midi_out_urbs_in_flight: usize,
    pub midi_running_status: u8,
    pub error_timer: core::ffi::c_void,
    pub midi_out_drain_completion: core::ffi::c_void,

    // --- Feedback Sync State ---
    pub feedback_accumulator_pattern: [u32; FEEDBACK_ACCUMULATOR_SIZE],
    pub feedback_pattern_out_idx: u32,
    pub feedback_pattern_in_idx: u32,
    pub feedback_synced: u8,
    pub feedback_consecutive_errors: u32,
    pub feedback_urb_skip_count: u32,
    pub fpo: Us144mkiiFramePatternObserver,

    // --- Workqueues ---
    pub stop_work: core::ffi::c_void,
    pub stop_pcm_work: core::ffi::c_void,
    pub capture_work: core::ffi::c_void,
    pub midi_in_work: core::ffi::c_void,
    pub midi_out_work: core::ffi::c_void,

    // --- Mixer/Routing State ---
    pub line_out_source: u32,
    pub digital_out_source: u32,
    pub capture_12_source: u32,
    pub capture_34_source: u32,
}

// main.c

/// Free all allocated URBs and associated buffers.
///
/// This function kills, unlinks, and frees all playback, feedback, capture,
/// and MIDI URBs, along with their transfer buffers and the capture
/// ring/decode buffers.
extern "C" {
    pub fn tascam_free_urbs(tascam: *mut TascamCard);
}

/// Allocate all URBs and associated buffers.
///
/// This function allocates and initializes all URBs for playback, feedback,
/// capture, and MIDI, as well as the necessary buffers for data processing.
///
/// # Return
/// 0 on success, or a negative error code on failure.
extern "C" {
    pub fn tascam_alloc_urbs(tascam: *mut TascamCard) -> i32;
}

/// Work handler to stop all active streams.
///
/// This function is scheduled to stop all active URBs (playback, feedback,
/// capture) and reset the active_urbs counter.
extern "C" {
    pub fn tascam_stop_work_handler(work: *mut core::ffi::c_void);
}

// us144mkii_pcm.h - Include dependency
// Note: us144mkii_pcm.h declarations would be included here

// us144mkii_midi.c

/// Completion handler for MIDI IN URBs.
///
/// This function runs in interrupt context. It places the raw data from the
/// USB endpoint into a kfifo and schedules a work item to process it later,
/// ensuring the interrupt handler remains fast.
extern "C" {
    pub fn tascam_midi_in_urb_complete(urb: *mut core::ffi::c_void);
}

/// Completion handler for MIDI OUT bulk URB.
///
/// This function runs in interrupt context. It marks the output URB as no
/// longer in-flight. It then re-schedules the work handler to check for and
/// send any more data waiting in the ALSA buffer. This is a safe, non-blocking
/// way to continue the data transmission chain.
extern "C" {
    pub fn tascam_midi_out_urb_complete(urb: *mut core::ffi::c_void);
}

/// Create and initialize the ALSA rawmidi device.
///
/// # Return
/// 0 on success, or a negative error code on failure.
extern "C" {
    pub fn tascam_create_midi(tascam: *mut TascamCard) -> i32;
}

// us144mkii_controls.c

/// Creates and adds ALSA mixer controls for the device.
///
/// This function registers custom ALSA controls for managing audio routing
/// (line out source, digital out source, capture 1-2 source, capture 3-4 source)
/// and displaying the current sample rate.
///
/// # Return
/// 0 on success, or a negative error code on failure.
extern "C" {
    pub fn tascam_create_controls(tascam: *mut TascamCard) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
