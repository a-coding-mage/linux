// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
pub struct snd_usb_audio {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct audioformat {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct usb_host_interface {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _opaque: [u8; 0],
}

extern "C" {
    pub fn snd_usb_parse_implicit_fb_quirk(
        chip: *mut snd_usb_audio,
        fmt: *mut audioformat,
        alts: *mut usb_host_interface,
    ) -> i32;

    pub fn snd_usb_find_implicit_fb_sync_format(
        chip: *mut snd_usb_audio,
        target: *const audioformat,
        params: *const snd_pcm_hw_params,
        stream: i32,
        fixed_rate: *mut bool,
    ) -> *const audioformat;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
