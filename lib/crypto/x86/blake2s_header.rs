/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than implemented in this header translation.

extern "C" {
    fn blake2s_compress_ssse3(
        ctx: *mut blake2s_ctx,
        data: *const u8,
        nblocks: usize,
        inc: u32,
    );
    fn blake2s_compress_avx512(
        ctx: *mut blake2s_ctx,
        data: *const u8,
        nblocks: usize,
        inc: u32,
    );
    fn blake2s_compress_generic(
        ctx: *mut blake2s_ctx,
        data: *const u8,
        nblocks: usize,
        inc: u32,
    );
    fn may_use_simd() -> bool;
    fn kernel_fpu_begin();
    fn kernel_fpu_end();
    fn boot_cpu_has(feature: u32) -> bool;
    fn cpu_has_xfeatures(mask: u64, feature: *mut core::ffi::c_void) -> bool;
    fn static_branch_enable(key: *mut bool);
}

#[repr(C)]
pub struct blake2s_ctx {
    _private: [u8; 0],
}

static mut blake2s_use_ssse3: bool = false;
static mut blake2s_use_avx512: bool = false;

const SZ_4K: usize = 4096;

unsafe fn blake2s_compress(
    ctx: *mut blake2s_ctx,
    mut data: *const u8,
    mut nblocks: usize,
    inc: u32,
) {
    // SIMD disables preemption, so relax after processing each page.
    // BUILD_BUG_ON(SZ_4K / BLAKE2S_BLOCK_SIZE < 8);

    if !blake2s_use_ssse3 || !may_use_simd() {
        blake2s_compress_generic(ctx, data, nblocks, inc);
        return;
    }

    loop {
        let blocks = core::cmp::min(nblocks, SZ_4K / BLAKE2S_BLOCK_SIZE);

        kernel_fpu_begin();
        if blake2s_use_avx512 {
            blake2s_compress_avx512(ctx, data, blocks, inc);
        } else {
            blake2s_compress_ssse3(ctx, data, blocks, inc);
        }
        kernel_fpu_end();

        data = data.add(blocks * BLAKE2S_BLOCK_SIZE);
        nblocks -= blocks;
        if nblocks == 0 {
            break;
        }
    }
}

// #define blake2s_mod_init_arch blake2s_mod_init_arch
unsafe fn blake2s_mod_init_arch() {
    if boot_cpu_has(X86_FEATURE_SSSE3) {
        static_branch_enable(&raw mut blake2s_use_ssse3);
    }

    if boot_cpu_has(X86_FEATURE_AVX)
        && boot_cpu_has(X86_FEATURE_AVX2)
        && boot_cpu_has(X86_FEATURE_AVX512F)
        && boot_cpu_has(X86_FEATURE_AVX512VL)
        && cpu_has_xfeatures(
            XFEATURE_MASK_SSE | XFEATURE_MASK_YMM | XFEATURE_MASK_AVX512,
            core::ptr::null_mut(),
        )
    {
        static_branch_enable(&raw mut blake2s_use_avx512);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
