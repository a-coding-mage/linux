// SPDX-License-Identifier: GPL-2.0
/*
 * <linux/usb/audio.h> -- USB Audio definitions.
 *
 * Copyright (C) 2006 Thumtronics Pty Ltd.
 * Developed for Thumtronics by Grey Innovation
 * Ben Williamson <ben.williamson@greyinnovation.com>
 *
 * This file holds USB constants and structures defined
 * by the USB Device Class Definition for Audio Devices.
 * Comments below reference relevant sections of that document:
 *
 * http://www.usb.org/developers/devclass_docs/audio10.pdf
 *
 * Types and defines in this file are either specific to version 1.0 of
 * this standard or common for newer versions.
 */

// Dependency intent from the original header: <uapi/linux/usb/audio.h>.

#[repr(C)]
pub struct usb_audio_control {
    pub list: list_head,
    pub name: *const core::ffi::c_char,
    pub type_: u8,
    pub data: [i32; 5],
    pub set: Option<unsafe extern "C" fn(
        con: *mut usb_audio_control,
        cmd: u8,
        value: i32,
    ) -> i32>,
    pub get: Option<unsafe extern "C" fn(con: *mut usb_audio_control, cmd: u8) -> i32>,
}

#[repr(C)]
pub struct usb_audio_control_selector {
    pub list: list_head,
    pub control: list_head,
    pub id: u8,
    pub name: *const core::ffi::c_char,
    pub type_: u8,
    pub desc: *mut usb_descriptor_header,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
