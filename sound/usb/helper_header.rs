// SPDX-License-Identifier: GPL-2.0

use std::os::raw::c_void;

// Opaque external types
pub struct usb_device;
pub struct snd_usb_audio;
pub struct usb_host_interface;

extern "C" {
    pub fn snd_usb_combine_bytes(bytes: *mut u8, size: i32) -> u32;

    pub fn snd_usb_find_desc(
        descstart: *mut c_void,
        desclen: i32,
        after: *mut c_void,
        dtype: u8,
    ) -> *mut c_void;

    pub fn snd_usb_find_csint_desc(
        descstart: *mut c_void,
        desclen: i32,
        after: *mut c_void,
        dsubtype: u8,
    ) -> *mut c_void;

    pub fn snd_usb_ctl_msg(
        dev: *mut usb_device,
        pipe: u32,
        request: u8,
        requesttype: u8,
        value: u16,
        index: u16,
        data: *mut c_void,
        size: u16,
    ) -> i32;

    pub fn snd_usb_parse_datainterval(
        chip: *mut snd_usb_audio,
        alts: *mut usb_host_interface,
    ) -> u8;

    pub fn snd_usb_get_host_interface(
        chip: *mut snd_usb_audio,
        ifnum: i32,
        altsetting: i32,
    ) -> *mut usb_host_interface;

    pub fn snd_usb_add_ctrl_interface_link(
        chip: *mut snd_usb_audio,
        ifnum: i32,
        ctrlif: i32,
    ) -> i32;

    pub fn snd_usb_find_ctrl_interface(
        chip: *mut snd_usb_audio,
        ifnum: i32,
    ) -> *mut usb_host_interface;

    pub fn snd_usb_validate_audio_desc(p: *mut c_void, protocol: i32) -> bool;
    pub fn snd_usb_validate_midi_desc(p: *mut c_void) -> bool;
}

// retrieve usb_interface descriptor from the host interface
// (conditional for compatibility with the older API)
#[macro_export]
macro_rules! get_iface_desc {
    ($iface:expr) => {
        unsafe { &(*$iface).desc }
    };
}

#[macro_export]
macro_rules! get_endpoint {
    ($alt:expr, $ep:expr) => {
        unsafe { &(*$alt).endpoint[$ep].desc }
    };
}

#[macro_export]
macro_rules! get_ep_desc {
    ($ep:expr) => {
        unsafe { &(*$ep).desc }
    };
}

#[macro_export]
macro_rules! get_cfg_desc {
    ($cfg:expr) => {
        unsafe { &(*$cfg).desc }
    };
}

#[macro_export]
macro_rules! snd_usb_get_speed {
    ($dev:expr) => {
        unsafe { (*$dev).speed }
    };
}

#[inline]
pub unsafe fn snd_usb_ctrl_intf(ctrl_intf: *mut usb_host_interface) -> i32 {
    (*ctrl_intf).desc.bInterfaceNumber as i32
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
