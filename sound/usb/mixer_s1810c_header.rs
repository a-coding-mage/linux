// SPDX-License-Identifier: GPL-2.0
//
// Presonus Studio 1810c driver for ALSA
// Copyright (C) 2019 Nick Kossifidis <mickflemm@gmail.com>

extern "C" {
    pub fn snd_sc1810_init_mixer(mixer: *mut usb_mixer_interface) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
