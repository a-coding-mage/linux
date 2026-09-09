/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * arm64-optimized SHA-512 block function
 *
 * Copyright 2025 Google LLC
 */

// C dependencies supplied by the surrounding kernel translation unit:
// <asm/simd.h> and <linux/cpufeature.h>

static mut have_sha512_insns: StaticKey = StaticKey::default();

extern "C" {
    fn sha512_block_data_order(
        state: *mut Sha512BlockState,
        data: *const u8,
        nblocks: usize,
    );
    fn sha512_ce_transform(
        state: *mut Sha512BlockState,
        data: *const u8,
        nblocks: usize,
    );
}

unsafe fn sha512_blocks(
    state: *mut Sha512BlockState,
    data: *const u8,
    nblocks: usize,
) {
    if static_branch_likely(&have_sha512_insns) && likely(may_use_simd()) {
        // C macro: scoped_ksimd()
        sha512_ce_transform(state, data, nblocks);
    } else {
        sha512_block_data_order(state, data, nblocks);
    }
}

// C macro: #define sha512_mod_init_arch sha512_mod_init_arch
unsafe fn sha512_mod_init_arch() {
    if cpu_have_named_feature(SHA512) {
        static_branch_enable(&mut have_sha512_insns);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
