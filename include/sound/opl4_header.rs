/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 * Global definitions for the OPL4 driver
 * Copyright (c) 2003 by Clemens Ladisch <clemens@ladisch.de>
 */

/* Dependency supplied by the surrounding translation unit: sound/opl3.h. */

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_opl3 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_opl4 {
    _private: [u8; 0],
}

extern "C" {
    pub fn snd_opl4_create(
        card: *mut snd_card,
        fm_port: ::core::ffi::c_ulong,
        pcm_port: ::core::ffi::c_ulong,
        seq_device: ::core::ffi::c_int,
        opl3: *mut *mut snd_opl3,
        opl4: *mut *mut snd_opl4,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
