// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2014 Carlo Caione <carlo@caione.org>
 */

use core::ffi::c_char;

// Translated from <asm/mach/arch.h>.  The DT_MACHINE_START/MACHINE_END
// macros define the platform machine descriptor in the surrounding kernel.
pub static MESON_COMMON_BOARD_COMPAT: [*const c_char; 5] = [
    b"amlogic,meson6\0".as_ptr() as *const c_char,
    b"amlogic,meson8\0".as_ptr() as *const c_char,
    b"amlogic,meson8b\0".as_ptr() as *const c_char,
    b"amlogic,meson8m2\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(MESON, "Amlogic Meson platform")
//     .dt_compat = meson_common_board_compat,
//     .l2c_aux_val = 0,
//     .l2c_aux_mask = ~0,
// MACHINE_END
pub const MESON_NAME: &[u8] = b"Amlogic Meson platform\0";
pub const MESON_L2C_AUX_VAL: u32 = 0;
pub const MESON_L2C_AUX_MASK: u32 = !0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
