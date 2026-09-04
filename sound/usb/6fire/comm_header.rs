// Linux driver for TerraTec DMX 6Fire USB
//
// Author: Torsten Schenk <torsten.schenk@zoho.com>
// Created: Jan 01, 2011
// Copyright: (C) Torsten Schenk
// SPDX-License-Identifier: GPL-2.0-or-later

// Requires definitions from common.h

pub const COMM_RECEIVER_BUFSIZE: usize = 64;

#[repr(C)]
pub struct comm_runtime {
    pub chip: *mut sfire_chip,
    pub receiver: urb,
    pub receiver_buffer: *mut u8,
    pub serial: u8,
    pub init_urb: Option<unsafe extern "C" fn(*mut comm_runtime, *mut urb, *mut u8, *mut (), unsafe extern "C" fn(*mut urb)) -> ()>,
    pub write8: Option<unsafe extern "C" fn(*mut comm_runtime, u8, u8, u8) -> i32>,
    pub write16: Option<unsafe extern "C" fn(*mut comm_runtime, u8, u8, u8, u8) -> i32>,
}

extern "C" {
    pub fn usb6fire_comm_init(chip: *mut sfire_chip) -> i32;
    pub fn usb6fire_comm_abort(chip: *mut sfire_chip);
    pub fn usb6fire_comm_destroy(chip: *mut sfire_chip);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
