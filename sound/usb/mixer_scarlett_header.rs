// SPDX-License-Identifier: GPL-2.0

extern "C" {
    pub fn snd_scarlett_controls_create(mixer: *mut usb_mixer_interface) -> c_int;
    pub fn snd_forte_controls_create(mixer: *mut usb_mixer_interface) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
