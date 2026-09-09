/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SHA-512 optimized using the CP Assist for Cryptographic Functions (CPACF)
 *
 * Copyright 2025 Google LLC
 */

// Dependencies supplied by the surrounding kernel translation unit:
// <asm/cpacf.h>, <linux/cpufeature.h>

use core::ffi::c_void;

// Opaque types supplied by the surrounding translation unit.
pub enum sha512_block_state {}
pub enum static_key_false {}

extern "C" {
    static mut have_cpacf_sha512: static_key_false;

    fn static_branch_likely(key: *const static_key_false) -> bool;
    fn static_branch_enable(key: *mut static_key_false);
    fn cpacf_kimd(function: c_void, state: *mut sha512_block_state, data: *const u8, len: usize);
    fn sha512_blocks_generic(
        state: *mut sha512_block_state,
        data: *const u8,
        nblocks: usize,
    );
    fn cpu_have_feature(feature: c_void) -> bool;
    fn cpacf_query_func(function: c_void, subfunction: c_void) -> bool;
}

// CPACF and SHA-512 constants are supplied by the surrounding translation unit.
extern "C" {
    static CPACF_KIMD_SHA_512: c_void;
    static CPACF_KIMD: c_void;
    static S390_CPU_FEATURE_MSA: c_void;
    static SHA512_BLOCK_SIZE: usize;
}

unsafe fn sha512_blocks(
    state: *mut sha512_block_state,
    data: *const u8,
    nblocks: usize,
) {
    if static_branch_likely(&have_cpacf_sha512 as *const static_key_false) {
        cpacf_kimd(
            CPACF_KIMD_SHA_512,
            state,
            data,
            nblocks.wrapping_mul(SHA512_BLOCK_SIZE),
        );
    } else {
        sha512_blocks_generic(state, data, nblocks);
    }
}

// #define sha512_mod_init_arch sha512_mod_init_arch
unsafe fn sha512_mod_init_arch() {
    if cpu_have_feature(S390_CPU_FEATURE_MSA)
        && cpacf_query_func(CPACF_KIMD, CPACF_KIMD_SHA_512)
    {
        static_branch_enable(&mut have_cpacf_sha512 as *mut static_key_false);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
