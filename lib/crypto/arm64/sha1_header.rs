/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SHA-1 optimized for ARM64
 *
 * Copyright 2025 Google LLC
 */

// C dependencies: <asm/simd.h> and <linux/cpufeature.h>.

use core::ffi::c_void;

// These types and functions are supplied by the surrounding translation unit.
#[repr(C)]
pub struct sha1_block_state {
    _opaque: [u8; 0],
}

extern "C" {
    static have_ce: c_void;

    fn sha1_ce_transform(
        state: *mut sha1_block_state,
        data: *const u8,
        nblocks: usize,
    );

    fn sha1_blocks_generic(
        state: *mut sha1_block_state,
        data: *const u8,
        nblocks: usize,
    );

    fn static_branch_likely(key: *const c_void) -> bool;
    fn may_use_simd() -> bool;
    fn cpu_have_named_feature(feature: u32) -> bool;
    fn static_branch_enable(key: *mut c_void);
}

// `SHA1` is supplied by the ARM64 CPU feature definitions.
const SHA1: u32 = 0; // build-time feature value supplied externally

static mut HAVE_CE: bool = false;

unsafe fn sha1_blocks(
    state: *mut sha1_block_state,
    data: *const u8,
    nblocks: usize,
) {
    if static_branch_likely(&have_ce as *const c_void) && may_use_simd() {
        // Corresponds to scoped_ksimd() around the SIMD transform call.
        sha1_ce_transform(state, data, nblocks);
    } else {
        sha1_blocks_generic(state, data, nblocks);
    }
}

// #define sha1_mod_init_arch sha1_mod_init_arch
unsafe fn sha1_mod_init_arch() {
    if cpu_have_named_feature(SHA1) {
        static_branch_enable(&HAVE_CE as *const bool as *mut c_void);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
