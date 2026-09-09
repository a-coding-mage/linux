/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   ALSA Driver for the PT2258 volume controller.
 *
 *	Copyright (c) 2006  Jochen Voss <voss@seehuhn.de>
 */

// C header guard: __SOUND_PT2258_H

// Types supplied by the surrounding ALSA implementation.
pub struct snd_card;
pub struct snd_i2c_bus;
pub struct snd_i2c_device;

#[repr(C)]
pub struct snd_pt2258 {
    pub card: *mut snd_card,
    pub i2c_bus: *mut snd_i2c_bus,
    pub i2c_dev: *mut snd_i2c_device,

    pub volume: [u8; 6],
    pub mute: i32,
}

extern "C" {
    pub fn snd_pt2258_reset(pt: *mut snd_pt2258) -> i32;
    pub fn snd_pt2258_build_controls(pt: *mut snd_pt2258) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
