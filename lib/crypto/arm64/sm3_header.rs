/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SM3 optimized for ARM64
 *
 * Copyright 2026 Google LLC
 */

// C dependencies: <asm/simd.h> and <linux/cpufeature.h>.

// DEFINE_STATIC_KEY_FALSE(have_neon) and DEFINE_STATIC_KEY_FALSE(have_ce).
// These flags represent the corresponding Linux static branches.
static mut HAVE_NEON: bool = false;
static mut HAVE_CE: bool = false;

extern "C" {
    pub fn sm3_neon_transform(
        state: *mut sm3_block_state,
        data: *const u8,
        nblocks: usize,
    );
    pub fn sm3_ce_transform(
        state: *mut sm3_block_state,
        data: *const u8,
        nblocks: usize,
    );
    fn sm3_blocks_generic(
        state: *mut sm3_block_state,
        data: *const u8,
        nblocks: usize,
    );
    fn may_use_simd() -> bool;
    fn cpu_have_named_feature(feature: i32) -> bool;
}

// `sm3_block_state`, `ASIMD`, and `SM3` are supplied by the surrounding
// implementation and kernel feature definitions.

unsafe fn sm3_blocks(
    state: *mut sm3_block_state,
    data: *const u8,
    nblocks: usize,
) {
    if HAVE_NEON && may_use_simd() {
        // scoped_ksimd() establishes the SIMD-use critical section in C.
        if HAVE_CE {
            sm3_ce_transform(state, data, nblocks);
        } else {
            sm3_neon_transform(state, data, nblocks);
        }
    } else {
        sm3_blocks_generic(state, data, nblocks);
    }
}

// #define sm3_mod_init_arch sm3_mod_init_arch
unsafe fn sm3_mod_init_arch() {
    if cpu_have_named_feature(ASIMD) {
        HAVE_NEON = true;
        if cpu_have_named_feature(SM3) {
            HAVE_CE = true;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
