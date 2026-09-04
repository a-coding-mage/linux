// SPDX-License-Identifier: GPL-2.0

use std::ffi::c_char;
use std::os::raw::{c_int, c_uint, c_ulong, c_void};

// Forward declarations of opaque types from other modules
pub struct audioformat;
pub struct snd_usb_endpoint;
pub struct snd_usb_substream;
pub struct snd_usb_audio;
pub struct usb_interface;
pub struct usb_driver;
pub struct snd_usb_audio_quirk;
pub struct usb_device;

extern "C" {
    pub fn snd_usb_create_quirk(
        chip: *mut snd_usb_audio,
        iface: *mut usb_interface,
        driver: *mut usb_driver,
        quirk: *const snd_usb_audio_quirk,
    ) -> c_int;

    pub fn snd_usb_apply_interface_quirk(
        chip: *mut snd_usb_audio,
        iface: c_int,
        altno: c_int,
    ) -> c_int;

    pub fn snd_usb_apply_boot_quirk(
        dev: *mut usb_device,
        intf: *mut usb_interface,
        quirk: *const snd_usb_audio_quirk,
        usb_id: c_uint,
    ) -> c_int;

    pub fn snd_usb_apply_boot_quirk_once(
        dev: *mut usb_device,
        intf: *mut usb_interface,
        quirk: *const snd_usb_audio_quirk,
        usb_id: c_uint,
    ) -> c_int;

    pub fn snd_usb_set_format_quirk(
        subs: *mut snd_usb_substream,
        fmt: *const audioformat,
    );

    pub fn snd_usb_is_big_endian_format(
        chip: *mut snd_usb_audio,
        fp: *const audioformat,
    ) -> c_int;

    pub fn snd_usb_endpoint_start_quirk(ep: *mut snd_usb_endpoint);

    pub fn snd_usb_ctl_msg_quirk(
        dev: *mut usb_device,
        pipe: c_uint,
        request: u8,
        requesttype: u8,
        value: u16,
        index: u16,
        data: *mut c_void,
        size: u16,
    );

    pub fn snd_usb_select_mode_quirk(
        chip: *mut snd_usb_audio,
        fmt: *const audioformat,
    ) -> c_int;

    pub fn snd_usb_interface_dsd_format_quirks(
        chip: *mut snd_usb_audio,
        fp: *mut audioformat,
        sample_bytes: c_uint,
    ) -> u64;

    pub fn snd_usb_audioformat_attributes_quirk(
        chip: *mut snd_usb_audio,
        fp: *mut audioformat,
        stream: c_int,
    );

    pub fn snd_usb_apply_flag_dbg(
        reason: *const c_char,
        chip: *mut snd_usb_audio,
        flag: c_ulong,
    );

    pub fn snd_usb_init_quirk_flags_table(chip: *mut snd_usb_audio);

    pub fn snd_usb_init_quirk_flags_parse_string(
        chip: *mut snd_usb_audio,
        str: *const c_char,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
