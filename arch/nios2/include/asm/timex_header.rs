/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright Altera Corporation (C) 2014. All rights reserved.
 */

pub type cycles_t = ::core::ffi::c_ulong;

extern "C" {
    pub fn get_cycles() -> cycles_t;
    pub fn random_get_entropy_fallback() -> ::core::ffi::c_ulong;
}

// C macro: #define get_cycles get_cycles

#[inline]
pub unsafe fn random_get_entropy() -> ::core::ffi::c_ulong {
    let cycles = get_cycles();
    if cycles != 0 {
        cycles as ::core::ffi::c_ulong
    } else {
        random_get_entropy_fallback()
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
