/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SHA-256 optimized for ARM64
 *
 * Copyright 2025 Google LLC
 */
// Dependencies supplied by the surrounding kernel translation:
// asm/simd.h and linux/cpufeature.h

// `DEFINE_STATIC_KEY_FALSE` declarations from the C header.
static mut have_neon: StaticKeyFalse = StaticKeyFalse::new();
static mut have_ce: StaticKeyFalse = StaticKeyFalse::new();

extern "C" {
    pub fn sha256_block_data_order(
        state: *mut sha256_block_state,
        data: *const u8,
        nblocks: usize,
    );
    pub fn sha256_block_neon(
        state: *mut sha256_block_state,
        data: *const u8,
        nblocks: usize,
    );
    pub fn sha256_ce_transform(
        state: *mut sha256_block_state,
        data: *const u8,
        nblocks: usize,
    );
}

unsafe fn sha256_blocks(
    state: *mut sha256_block_state,
    data: *const u8,
    nblocks: usize,
) {
    if static_branch_likely(&have_neon) && likely(may_use_simd()) {
        // `scoped_ksimd()` establishes the kernel SIMD scope.
        scoped_ksimd!({
            if static_branch_likely(&have_ce) {
                sha256_ce_transform(state, data, nblocks);
            } else {
                sha256_block_neon(state, data, nblocks);
            }
        });
    } else {
        sha256_block_data_order(state, data, nblocks);
    }
}

// C static assertions:
// static_assert(offsetof(struct __sha256_ctx, state) == 0);
// static_assert(offsetof(struct __sha256_ctx, bytecount) == 32);
// static_assert(offsetof(struct __sha256_ctx, buf) == 40);

extern "C" {
    pub fn sha256_ce_finup2x(
        ctx: *const __sha256_ctx,
        data1: *const u8,
        data2: *const u8,
        len: i32,
        out1: *mut u8,
        out2: *mut u8,
    );
}

// #define sha256_finup_2x_arch sha256_finup_2x_arch
unsafe fn sha256_finup_2x_arch(
    ctx: *const __sha256_ctx,
    data1: *const u8,
    data2: *const u8,
    len: usize,
    out1: *mut u8,
    out2: *mut u8,
) -> bool {
    // The assembly requires len >= SHA256_BLOCK_SIZE && len <= INT_MAX.
    if static_branch_likely(&have_ce)
        && len >= SHA256_BLOCK_SIZE
        && len <= INT_MAX as usize
        && likely(may_use_simd())
    {
        // `scoped_ksimd()` establishes the kernel SIMD scope.
        scoped_ksimd!({
            sha256_ce_finup2x(ctx, data1, data2, len as i32, out1, out2);
        });
        kmsan_unpoison_memory(out1, SHA256_DIGEST_SIZE);
        kmsan_unpoison_memory(out2, SHA256_DIGEST_SIZE);
        return true;
    }
    false
}

unsafe fn sha256_finup_2x_is_optimized_arch() -> bool {
    static_key_enabled(&have_ce)
}

// #define sha256_mod_init_arch sha256_mod_init_arch
unsafe fn sha256_mod_init_arch() {
    if cpu_have_named_feature(ASIMD) {
        static_branch_enable(&mut have_neon);
        if cpu_have_named_feature(SHA2) {
            static_branch_enable(&mut have_ce);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
