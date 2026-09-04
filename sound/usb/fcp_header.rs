// SPDX-License-Identifier: GPL-2.0

extern "C" {
    pub fn snd_fcp_init(mixer: *mut usb_mixer_interface) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
