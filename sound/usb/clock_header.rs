// SPDX-License-Identifier: GPL-2.0

extern "C" {
    pub fn snd_usb_init_sample_rate(
        chip: *mut snd_usb_audio,
        fmt: *const audioformat,
        rate: i32,
    ) -> i32;

    pub fn snd_usb_clock_find_source(
        chip: *mut snd_usb_audio,
        fmt: *const audioformat,
        validate: bool,
    ) -> i32;

    pub fn snd_usb_set_sample_rate_v2v3(
        chip: *mut snd_usb_audio,
        fmt: *const audioformat,
        clock: i32,
        rate: i32,
    ) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
