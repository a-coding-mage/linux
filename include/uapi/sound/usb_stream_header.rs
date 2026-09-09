/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright (C) 2007, 2008 Karsten Wiese <fzu@wemgehoertderstaat.de>
 */

pub const USB_STREAM_INTERFACE_VERSION: u32 = 2;

/* _IOW('H', 0x90, struct usb_stream_config) */
pub const SNDRV_USB_STREAM_IOCTL_SET_PARAMS: u32 = 0x4010_4890;

#[repr(C)]
pub struct usb_stream_packet {
    pub offset: u32,
    pub length: u32,
}

#[repr(C)]
pub struct usb_stream_config {
    pub version: u32,
    pub sample_rate: u32,
    pub period_frames: u32,
    pub frame_size: u32,
}

#[repr(C)]
pub struct usb_stream {
    pub cfg: usb_stream_config,
    pub read_size: u32,
    pub write_size: u32,

    pub period_size: i32,

    pub state: u32,

    pub idle_insize: i32,
    pub idle_outsize: i32,
    pub sync_packet: i32,
    pub insize_done: u32,
    pub periods_done: u32,
    pub periods_polled: u32,

    pub outpacket: [usb_stream_packet; 2],
    pub inpackets: u32,
    pub inpacket_head: u32,
    pub inpacket_split: u32,
    pub inpacket_split_at: u32,
    pub next_inpacket_split: u32,
    pub next_inpacket_split_at: u32,
    pub inpacket: [usb_stream_packet; 0],
}

#[repr(C)]
pub enum usb_stream_state {
    usb_stream_invalid,
    usb_stream_stopped,
    usb_stream_sync0,
    usb_stream_sync1,
    usb_stream_ready,
    usb_stream_running,
    usb_stream_xrun,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
