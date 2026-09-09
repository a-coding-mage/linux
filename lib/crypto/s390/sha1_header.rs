/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SHA-1 optimized using the CP Assist for Cryptographic Functions (CPACF)
 *
 * Copyright 2025 Google LLC
 */

// External symbols supplied by the kernel and related headers.
#[repr(C)]
pub struct Sha1BlockState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct StaticKey {
    _private: [u8; 0],
}

extern "C" {
    fn static_branch_likely(key: *const StaticKey) -> bool;
    fn cpacf_kimd(function_code: u32, state: *mut Sha1BlockState, data: *const u8, len: usize);
    fn sha1_blocks_generic(state: *mut Sha1BlockState, data: *const u8, nblocks: usize);
    fn cpu_have_feature(feature: u32) -> bool;
    fn cpacf_query_func(function: u32, function_code: u32) -> bool;
    fn static_branch_enable(key: *mut StaticKey);
}

extern "C" {
    static mut have_cpacf_sha1: StaticKey;
}

// Values supplied by <asm/cpacf.h> and the SHA-1 implementation headers.
extern "C" {
    static CPACF_KIMD_SHA_1: u32;
    static CPACF_KIMD: u32;
    static S390_CPU_FEATURE_MSA: u32;
    static SHA1_BLOCK_SIZE: usize;
}

// __ro_after_init DEFINE_STATIC_KEY_FALSE(have_cpacf_sha1)

unsafe fn sha1_blocks(state: *mut Sha1BlockState, data: *const u8, nblocks: usize) {
    if static_branch_likely(&have_cpacf_sha1 as *const StaticKey) {
        cpacf_kimd(
            CPACF_KIMD_SHA_1,
            state,
            data,
            nblocks.wrapping_mul(SHA1_BLOCK_SIZE),
        );
    } else {
        sha1_blocks_generic(state, data, nblocks);
    }
}

// #define sha1_mod_init_arch sha1_mod_init_arch
unsafe fn sha1_mod_init_arch() {
    if cpu_have_feature(S390_CPU_FEATURE_MSA)
        && cpacf_query_func(CPACF_KIMD, CPACF_KIMD_SHA_1)
    {
        static_branch_enable(&mut have_cpacf_sha1 as *mut StaticKey);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
