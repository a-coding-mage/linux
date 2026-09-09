/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SHA-256 optimized using the CP Assist for Cryptographic Functions (CPACF)
 *
 * Copyright 2025 Google LLC
 */

// C dependencies: <asm/cpacf.h>, <linux/cpufeature.h>.

extern "C" {
    static mut have_cpacf_sha256: StaticKey;

    fn static_branch_likely(key: *const StaticKey) -> bool;
    fn cpacf_kimd(function: u32, state: *mut sha256_block_state, data: *const u8, len: usize);
    fn sha256_blocks_generic(
        state: *mut sha256_block_state,
        data: *const u8,
        nblocks: usize,
    );
    fn cpu_have_feature(feature: u32) -> bool;
    fn cpacf_query_func(function: u32, subfunction: u32) -> bool;
    fn static_branch_enable(key: *mut StaticKey);
}

// Supplied by the including translation unit/dependencies.
#[repr(C)]
pub struct StaticKey {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sha256_block_state {
    _private: [u8; 0],
}

unsafe fn sha256_blocks(
    state: *mut sha256_block_state,
    data: *const u8,
    nblocks: usize,
) {
    if static_branch_likely(&raw const have_cpacf_sha256) {
        cpacf_kimd(CPACF_KIMD_SHA_256, state, data, nblocks * SHA256_BLOCK_SIZE);
    } else {
        sha256_blocks_generic(state, data, nblocks);
    }
}

// C self-referential macro: #define sha256_mod_init_arch sha256_mod_init_arch
unsafe fn sha256_mod_init_arch() {
    if cpu_have_feature(S390_CPU_FEATURE_MSA)
        && cpacf_query_func(CPACF_KIMD, CPACF_KIMD_SHA_256)
    {
        static_branch_enable(&raw mut have_cpacf_sha256);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
