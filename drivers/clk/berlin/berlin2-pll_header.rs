/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2014 Marvell Technology Group Ltd.
 *
 * Alexandre Belloni <alexandre.belloni@free-electrons.com>
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 */

// Dependency intent from the C header: `u8`, `__iomem`, and the C ABI types
// are supplied by the surrounding kernel translation.

#[repr(C)]
pub struct berlin2_pll_map {
    pub vcodiv: [u8; 16],
    pub mult: u8,
    pub fbdiv_shift: u8,
    pub rfdiv_shift: u8,
    pub divsel_shift: u8,
}

pub unsafe extern "C" fn berlin2_pll_register(
    map: *const berlin2_pll_map,
    base: *mut core::ffi::c_void,
    name: *const core::ffi::c_char,
    parent_name: *const core::ffi::c_char,
    flags: core::ffi::c_ulong,
) -> core::ffi::c_int;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
