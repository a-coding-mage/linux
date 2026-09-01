// SPDX-License-Identifier: GPL-2.0-or-later
//
// Linux driver for TerraTec DMX 6Fire USB
//
// Author: Torsten Schenk <torsten.schenk@zoho.com>
// Created: Jan 01, 2011
// Copyright: (C) Torsten Schenk

// External dependencies from common.h and other headers
#[repr(C)]
pub struct usb_device;
#[repr(C)]
pub struct snd_card;
#[repr(C)]
pub struct midi_runtime;
#[repr(C)]
pub struct pcm_runtime;
#[repr(C)]
pub struct control_runtime;
#[repr(C)]
pub struct comm_runtime;

#[repr(C)]
pub struct sfire_chip {
    pub dev: *mut usb_device,
    pub card: *mut snd_card,
    pub intf_count: i32,
    pub regidx: i32, // index in module parameter arrays
    pub shutdown: bool,
    pub midi: *mut midi_runtime,
    pub pcm: *mut pcm_runtime,
    pub control: *mut control_runtime,
    pub comm: *mut comm_runtime,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
