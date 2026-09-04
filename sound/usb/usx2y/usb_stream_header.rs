// SPDX-License-Identifier: GPL-2.0
// Translated from: usb/usx2y/usb_stream.h
// Depends on: uapi/sound/usb_stream.h and kernel types (usb_stream, usb_device, urb, wait_queue_head_t)

pub const USB_STREAM_NURBS: usize = 4;
pub const USB_STREAM_URBDEPTH: usize = 4;

// Forward declarations for external kernel types
pub struct usb_stream;
pub struct usb_device;
pub struct urb;
pub struct wait_queue_head_t;

/// USB stream kernel context structure
#[repr(C)]
pub struct usb_stream_kernel {
    pub s: *mut usb_stream,
    pub dev: *mut usb_device,
    pub write_page: *mut core::ffi::c_void,
    pub n_o_ps: u32,
    pub inurb: [*mut urb; 4],
    pub idle_inurb: *mut urb,
    pub completed_inurb: *mut urb,
    pub outurb: [*mut urb; 4],
    pub idle_outurb: *mut urb,
    pub completed_outurb: *mut urb,
    pub i_urb: *mut urb,
    pub iso_frame_balance: i32,
    pub sleep: wait_queue_head_t,
    pub out_phase: u32,
    pub out_phase_peeked: u32,
    pub freqn: u32,
}

extern "C" {
    pub fn usb_stream_new(
        sk: *mut usb_stream_kernel,
        dev: *mut usb_device,
        in_endpoint: u32,
        out_endpoint: u32,
        sample_rate: u32,
        use_packsize: u32,
        period_frames: u32,
        frame_size: u32,
    ) -> *mut usb_stream;

    pub fn usb_stream_free(sk: *mut usb_stream_kernel);

    pub fn usb_stream_start(sk: *mut usb_stream_kernel) -> i32;

    pub fn usb_stream_stop(sk: *mut usb_stream_kernel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
