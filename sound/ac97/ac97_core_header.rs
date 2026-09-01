/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 Robert Jarzmik <robert.jarzmik@free.fr>
 */

unsafe extern "C" {
    pub fn snd_ac97_bus_scan_one(
        adrv: *mut ac97_controller,
        codec_num: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_uint;
}

#[inline]
pub fn ac97_ids_match(
    id1: ::core::ffi::c_uint,
    id2: ::core::ffi::c_uint,
    mask: ::core::ffi::c_uint,
) -> bool {
    (id1 & mask) == (id2 & mask)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
