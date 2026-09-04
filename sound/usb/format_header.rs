// SPDX-License-Identifier: GPL-2.0

// External types from other modules
#[repr(C)]
pub struct snd_usb_audio;

#[repr(C)]
pub struct audioformat;

#[repr(C)]
pub struct uac_format_type_i_continuous_descriptor;

#[repr(C)]
pub struct uac3_as_header_descriptor;

extern "C" {
    pub fn snd_usb_parse_audio_format(
        chip: *mut snd_usb_audio,
        fp: *mut audioformat,
        format: u64,
        fmt: *mut uac_format_type_i_continuous_descriptor,
        stream: i32,
    ) -> i32;

    pub fn snd_usb_parse_audio_format_v3(
        chip: *mut snd_usb_audio,
        fp: *mut audioformat,
        r#as: *mut uac3_as_header_descriptor,
        stream: i32,
    ) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
