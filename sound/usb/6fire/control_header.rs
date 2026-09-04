// SPDX-License-Identifier: GPL-2.0-or-later
//
// Linux driver for TerraTec DMX 6Fire USB
//
// Author:    Torsten Schenk <torsten.schenk@zoho.com>
// Created:   Jan 01, 2011
// Copyright: (C) Torsten Schenk

// External types from common.h
pub struct sfire_chip;
pub struct snd_kcontrol;

pub const CONTROL_MAX_ELEMENTS: usize = 32;

pub const CONTROL_RATE_44KHZ: i32 = 0;
pub const CONTROL_RATE_48KHZ: i32 = 1;
pub const CONTROL_RATE_88KHZ: i32 = 2;
pub const CONTROL_RATE_96KHZ: i32 = 3;
pub const CONTROL_RATE_176KHZ: i32 = 4;
pub const CONTROL_RATE_192KHZ: i32 = 5;
pub const CONTROL_N_RATES: i32 = 6;

pub struct control_runtime {
    pub update_streaming: Option<extern "C" fn(*mut control_runtime) -> i32>,
    pub set_rate: Option<extern "C" fn(*mut control_runtime, i32) -> i32>,
    pub set_channels: Option<extern "C" fn(*mut control_runtime, i32, i32, bool, bool) -> i32>,
    pub chip: *mut sfire_chip,
    pub element: [*mut snd_kcontrol; CONTROL_MAX_ELEMENTS],
    pub opt_coax_switch: bool,
    pub line_phono_switch: bool,
    pub digital_thru_switch: bool,
    pub usb_streaming: bool,
    pub output_vol: [u8; 6],
    pub ovol_updated: u8,
    pub output_mute: u8,
    pub input_vol: [i8; 2],
    pub ivol_updated: u8,
}

extern "C" {
    pub fn usb6fire_control_init(chip: *mut sfire_chip) -> i32;
    pub fn usb6fire_control_abort(chip: *mut sfire_chip);
    pub fn usb6fire_control_destroy(chip: *mut sfire_chip);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
