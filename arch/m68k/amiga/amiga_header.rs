/* SPDX-License-Identifier: GPL-2.0-only */

/* amisound.c */
extern "C" {
    pub fn amiga_init_sound();
    pub fn amiga_mksound(hz: ::core::ffi::c_uint, ticks: ::core::ffi::c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
