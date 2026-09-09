/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018 Linaro Ltd <ard.biesheuvel@linaro.org>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation.
 */

// Kernel SIMD and CPU-feature declarations are supplied by the surrounding
// translation unit.

extern "C" {
    pub static mut have_sha3: StaticKeyFalse;

    pub fn sha3_ce_transform(
        state: *mut sha3_state,
        data: *const u8,
        nblocks: usize,
        block_size: usize,
    );

    fn sha3_absorb_blocks_generic(
        state: *mut sha3_state,
        data: *const u8,
        nblocks: usize,
        block_size: usize,
    );

    fn sha3_keccakf_generic(state: *mut sha3_state);
    fn static_branch_likely(key: *const StaticKeyFalse) -> bool;
    fn may_use_simd() -> bool;
    fn cpu_have_named_feature(feature: CpuFeature) -> bool;
    fn static_branch_enable(key: *mut StaticKeyFalse);
}

// Opaque types and kernel-specific operations are provided by other files.
pub enum sha3_state {}
#[repr(C)]
pub struct StaticKeyFalse {
    _private: [u8; 0],
}
#[repr(C)]
pub struct CpuFeature {
    _private: [u8; 0],
}

// SHA3 is the CPU feature name used by cpu_have_named_feature().
pub const SHA3: CpuFeature = CpuFeature { _private: [] };

// The value is supplied by the SHA-3 definitions in the surrounding code.
extern "C" {
    pub static SHA3_512_BLOCK_SIZE: usize;
}

pub unsafe fn sha3_absorb_blocks(
    state: *mut sha3_state,
    data: *const u8,
    nblocks: usize,
    block_size: usize,
) {
    if static_branch_likely(&have_sha3) && may_use_simd() {
        // scoped_ksimd() guards the SIMD call in the kernel implementation.
        sha3_ce_transform(state, data, nblocks, block_size);
    } else {
        sha3_absorb_blocks_generic(state, data, nblocks, block_size);
    }
}

pub unsafe fn sha3_keccakf(state: *mut sha3_state) {
    if static_branch_likely(&have_sha3) && may_use_simd() {
        /*
         * Passing zeroes into sha3_ce_transform() gives the plain
         * Keccak-f permutation, which is what we want here.  Any
         * supported block size may be used.  Use SHA3_512_BLOCK_SIZE
         * since it's the shortest.
         */
        static ZEROES: [u8; 0] = [];

        // scoped_ksimd() guards the SIMD call in the kernel implementation.
        sha3_ce_transform(state, ZEROES.as_ptr(), 1, SHA3_512_BLOCK_SIZE);
    } else {
        sha3_keccakf_generic(state);
    }
}

// #define sha3_mod_init_arch sha3_mod_init_arch
pub unsafe fn sha3_mod_init_arch() {
    if cpu_have_named_feature(SHA3) {
        static_branch_enable(&mut have_sha3);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
