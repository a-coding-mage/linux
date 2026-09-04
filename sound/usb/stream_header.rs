// SPDX-License-Identifier: GPL-2.0

use std::ffi::c_int;

// Opaque type declarations for C structures
#[repr(C)]
pub struct snd_usb_audio {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct audioformat {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_usb_power_domain {
    _opaque: [u8; 0],
}

// External C function declarations
extern "C" {
    pub fn snd_usb_parse_audio_interface(
        chip: *mut snd_usb_audio,
        iface_no: c_int,
    ) -> c_int;

    pub fn snd_usb_add_audio_stream(
        chip: *mut snd_usb_audio,
        stream: c_int,
        fp: *mut audioformat,
        pdptr: *mut *mut snd_usb_power_domain,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
