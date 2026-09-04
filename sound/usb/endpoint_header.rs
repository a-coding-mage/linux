// SPDX-License-Identifier: GPL-2.0

pub const SND_USB_ENDPOINT_TYPE_DATA: i32 = 0;
pub const SND_USB_ENDPOINT_TYPE_SYNC: i32 = 1;

#[repr(C)]
pub struct snd_usb_endpoint {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_usb_audio {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct audioformat {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_usb_substream {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct urb {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_urb_ctx {
    _opaque: [u8; 0],
}

extern "C" {
    pub fn snd_usb_get_endpoint(chip: *mut snd_usb_audio, ep_num: i32) -> *mut snd_usb_endpoint;

    pub fn snd_usb_add_endpoint(chip: *mut snd_usb_audio, ep_num: i32, type_: i32) -> i32;

    pub fn snd_usb_endpoint_open(
        chip: *mut snd_usb_audio,
        fp: *const audioformat,
        params: *const snd_pcm_hw_params,
        is_sync_ep: bool,
        fixed_rate: bool,
    ) -> *mut snd_usb_endpoint;

    pub fn snd_usb_endpoint_close(chip: *mut snd_usb_audio, ep: *mut snd_usb_endpoint);

    pub fn snd_usb_endpoint_set_params(chip: *mut snd_usb_audio, ep: *mut snd_usb_endpoint) -> i32;

    pub fn snd_usb_endpoint_prepare(chip: *mut snd_usb_audio, ep: *mut snd_usb_endpoint) -> i32;

    pub fn snd_usb_endpoint_get_clock_rate(chip: *mut snd_usb_audio, clock: i32) -> i32;

    pub fn snd_usb_endpoint_compatible(
        chip: *mut snd_usb_audio,
        ep: *mut snd_usb_endpoint,
        fp: *const audioformat,
        params: *const snd_pcm_hw_params,
    ) -> bool;

    pub fn snd_usb_endpoint_set_sync(
        chip: *mut snd_usb_audio,
        data_ep: *mut snd_usb_endpoint,
        sync_ep: *mut snd_usb_endpoint,
    );

    pub fn snd_usb_endpoint_set_callback(
        ep: *mut snd_usb_endpoint,
        prepare: Option<unsafe extern "C" fn(*mut snd_usb_substream, *mut urb, bool) -> i32>,
        retire: Option<unsafe extern "C" fn(*mut snd_usb_substream, *mut urb)>,
        data_subs: *mut snd_usb_substream,
    );

    pub fn snd_usb_endpoint_start(ep: *mut snd_usb_endpoint) -> i32;

    pub fn snd_usb_endpoint_stop(ep: *mut snd_usb_endpoint, keep_pending: bool);

    pub fn snd_usb_endpoint_sync_pending_stop(ep: *mut snd_usb_endpoint);

    pub fn snd_usb_endpoint_suspend(ep: *mut snd_usb_endpoint);

    pub fn snd_usb_endpoint_release(ep: *mut snd_usb_endpoint);

    pub fn snd_usb_endpoint_free_all(chip: *mut snd_usb_audio);

    pub fn snd_usb_endpoint_implicit_feedback_sink(ep: *mut snd_usb_endpoint) -> i32;

    pub fn snd_usb_endpoint_next_packet_size(
        ep: *mut snd_usb_endpoint,
        ctx: *mut snd_urb_ctx,
        idx: i32,
        avail: u32,
    ) -> i32;

    pub fn snd_usb_queue_pending_output_urbs(ep: *mut snd_usb_endpoint, in_stream_lock: bool)
        -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
