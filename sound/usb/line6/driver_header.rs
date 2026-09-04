// SPDX-License-Identifier: GPL-2.0-only
//
// Line 6 Linux USB driver
//
// Copyright (C) 2004-2010 Markus Grabner (line6@grabner-graz.at)

// Dependencies from C headers (translated):
// #include <linux/usb.h>
// #include <linux/mutex.h>
// #include <linux/kfifo.h>
// #include <sound/core.h>
// #include "midi.h"

// USB 1.1 speed configuration
pub const USB_LOW_INTERVALS_PER_SECOND: u32 = 1000;
pub const USB_LOW_ISO_BUFFERS: u32 = 2;

// USB 2.0+ speed configuration
pub const USB_HIGH_INTERVALS_PER_SECOND: u32 = 8000;
pub const USB_HIGH_ISO_BUFFERS: u32 = 16;

// Fallback USB interval and max packet size values
pub const LINE6_FALLBACK_INTERVAL: u32 = 10;
pub const LINE6_FALLBACK_MAXPACKETSIZE: u32 = 16;

pub const LINE6_TIMEOUT: u32 = 1000;
pub const LINE6_BUFSIZE_LISTEN: u32 = 64;
pub const LINE6_MIDI_MESSAGE_MAXLEN: u32 = 256;

pub const LINE6_RAW_MESSAGES_MAXCOUNT_ORDER: u32 = 7;
// 4k packets are common, BUFSIZE * MAXCOUNT should be bigger...
pub const LINE6_RAW_MESSAGES_MAXCOUNT: u32 = 1 << LINE6_RAW_MESSAGES_MAXCOUNT_ORDER;

// Compile-time check in C: LINE6_BUFSIZE_LISTEN > 65535 would error with "Use dynamic fifo instead"
// This is enforced by preprocessor in C; runtime assertion would be needed in Rust

// Line 6 MIDI control commands
pub const LINE6_PARAM_CHANGE: u8 = 0xb0;
pub const LINE6_PROGRAM_CHANGE: u8 = 0xc0;
pub const LINE6_SYSEX_BEGIN: u8 = 0xf0;
pub const LINE6_SYSEX_END: u8 = 0xf7;
pub const LINE6_RESET: u8 = 0xff;

// MIDI channel for messages initiated by the host
// (and eventually echoed back by the device)
pub const LINE6_CHANNEL_HOST: u8 = 0x00;

// MIDI channel for messages initiated by the device
pub const LINE6_CHANNEL_DEVICE: u8 = 0x02;

pub const LINE6_CHANNEL_UNKNOWN: u8 = 5; // don't know yet what this is good for

pub const LINE6_CHANNEL_MASK: u8 = 0x0f;

extern "C" {
    pub static line6_midi_id: [u8; 3];
}

pub const SYSEX_DATA_OFS: usize = 3 + 3; // sizeof(line6_midi_id) + 3
pub const SYSEX_EXTRA_SIZE: usize = 3 + 4; // sizeof(line6_midi_id) + 4

// Opaque types from kernel headers
#[repr(C)]
pub struct usb_device;

#[repr(C)]
pub struct device;

#[repr(C)]
pub struct snd_card;

#[repr(C)]
pub struct snd_line6_pcm;

#[repr(C)]
pub struct snd_line6_midi;

#[repr(C)]
pub struct urb;

#[repr(C)]
pub struct delayed_work;

#[repr(C)]
pub struct mutex;

#[repr(C)]
pub struct wait_queue_head_t;

#[repr(C)]
pub struct usb_interface;

#[repr(C)]
pub struct usb_device_id;

#[repr(C)]
pub struct pm_message_t;

// Common properties of Line 6 devices.
#[repr(C)]
pub struct line6_properties {
    // Card id string (maximum 16 characters).
    // This can be used to address the device in ALSA programs as
    // "default:CARD=<id>"
    pub id: *const i8,

    // Card short name (maximum 32 characters)
    pub name: *const i8,

    // Bit vector defining this device's capabilities in line6usb driver
    pub capabilities: i32,

    pub altsetting: i32,

    pub ctrl_if: u32,
    pub ep_ctrl_r: u32,
    pub ep_ctrl_w: u32,
    pub ep_audio_r: u32,
    pub ep_audio_w: u32,
}

// Capability bits
pub const LINE6_CAP_CONTROL: i32 = 1 << 0; // device supports settings parameter via USB
pub const LINE6_CAP_PCM: i32 = 1 << 1; // device supports PCM input/output via USB
pub const LINE6_CAP_HWMON: i32 = 1 << 2; // device supports hardware monitoring
pub const LINE6_CAP_IN_NEEDS_OUT: i32 = 1 << 3; // device requires output data when input is read
pub const LINE6_CAP_CONTROL_MIDI: i32 = 1 << 4; // device uses raw MIDI via USB (data endpoints)
pub const LINE6_CAP_CONTROL_INFO: i32 = 1 << 5; // device provides low-level information
pub const LINE6_CAP_HWMON_CTL: i32 = 1 << 6; // device provides hardware monitoring volume control

// Circular buffer for non-MIDI control messages
#[repr(C)]
pub struct line6_messages {
    pub read_lock: mutex,
    pub wait_queue: wait_queue_head_t,
    // Bitfield: unsigned int active:1; unsigned int nonblock:1;
    pub flags: u32,
    // STRUCT_KFIFO_REC_2(LINE6_BUFSIZE_LISTEN * LINE6_RAW_MESSAGES_MAXCOUNT)
    // from <linux/kfifo.h>; expands to kernel fifo structure
    pub fifo: [u8; 8192], // SIZE = 64 * 128
}

// Common data shared by all Line 6 devices.
// Corresponds to a pair of USB endpoints.
#[repr(C)]
pub struct usb_line6 {
    // USB device
    pub usbdev: *mut usb_device,

    // Properties
    pub properties: *const line6_properties,

    // Interval for data USB packets
    pub interval: i32,
    // ...for isochronous transfers framing
    pub intervals_per_second: i32,

    // Number of isochronous URBs used for frame transfers
    pub iso_buffers: i32,

    // Maximum size of data USB packet
    pub max_packet_size: i32,

    // Device representing the USB interface
    pub ifcdev: *mut device,

    // Line 6 sound card data structure.
    // Each device has at least MIDI or PCM.
    pub card: *mut snd_card,

    // Line 6 PCM device data structure
    pub line6pcm: *mut snd_line6_pcm,

    // Line 6 MIDI device data structure
    pub line6midi: *mut snd_line6_midi,

    // URB for listening to POD data endpoint
    pub urb_listen: *mut urb,

    // Buffer for incoming data from POD data endpoint
    pub buffer_listen: *mut u8,

    // Buffer for message to be processed, generated from MIDI layer
    pub buffer_message: *mut u8,

    // Length of message to be processed, generated from MIDI layer
    pub message_length: i32,

    // Circular buffer for non-MIDI control messages
    pub messages: line6_messages,

    // Work for delayed PCM startup
    pub startup_work: delayed_work,

    // If MIDI is supported, buffer_message contains the pre-processed data;
    // otherwise the data is only in urb_listen (buffer_incoming).
    pub process_message: unsafe extern "C" fn(*mut usb_line6),
    pub disconnect: unsafe extern "C" fn(*mut usb_line6),
    pub startup: unsafe extern "C" fn(*mut usb_line6),
}

extern "C" {
    pub fn line6_alloc_sysex_buffer(
        line6: *mut usb_line6,
        code1: i32,
        code2: i32,
        size: i32,
    ) -> *mut i8;

    pub fn line6_read_data(
        line6: *mut usb_line6,
        address: u32,
        data: *mut core::ffi::c_void,
        datalen: u32,
    ) -> i32;

    pub fn line6_read_serial_number(
        line6: *mut usb_line6,
        serial_number: *mut u32,
    ) -> i32;

    pub fn line6_send_raw_message(
        line6: *mut usb_line6,
        buffer: *const i8,
        size: i32,
    ) -> i32;

    pub fn line6_send_raw_message_async(
        line6: *mut usb_line6,
        buffer: *const i8,
        size: i32,
    ) -> i32;

    pub fn line6_send_sysex_message(
        line6: *mut usb_line6,
        buffer: *const i8,
        size: i32,
    ) -> i32;

    pub fn line6_version_request_async(line6: *mut usb_line6) -> i32;

    pub fn line6_write_data(
        line6: *mut usb_line6,
        address: u32,
        data: *mut core::ffi::c_void,
        datalen: u32,
    ) -> i32;

    pub fn line6_probe(
        interface: *mut usb_interface,
        id: *const usb_device_id,
        driver_name: *const i8,
        properties: *const line6_properties,
        private_init: unsafe extern "C" fn(*mut usb_line6, *const usb_device_id) -> i32,
        data_size: usize,
    ) -> i32;

    pub fn line6_disconnect(interface: *mut usb_interface);

    // #ifdef CONFIG_PM
    // When CONFIG_PM is defined in kernel config:
    pub fn line6_suspend(interface: *mut usb_interface, message: pm_message_t) -> i32;
    pub fn line6_resume(interface: *mut usb_interface) -> i32;
    // #endif
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
