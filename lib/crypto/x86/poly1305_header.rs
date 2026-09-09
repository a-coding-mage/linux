/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 */

#[repr(C)]
pub struct poly1305_arch_internal {
    pub base: poly1305_arch_base,
    pub r: [u64; 2],
    pub pad: u64,
    pub rn: [poly1305_rn; 9],
}

#[repr(C)]
pub union poly1305_arch_base {
    pub fields: poly1305_arch_fields,
    pub hs: [u64; 3],
}

#[repr(C)]
pub struct poly1305_arch_fields {
    pub h: [u32; 5],
    pub is_base2_26: u32,
}

#[repr(C)]
pub struct poly1305_rn {
    pub r2: u32,
    pub r1: u32,
    pub r4: u32,
    pub r3: u32,
}

/* The AVX code uses base 2^26, while the scalar code uses base 2^64. */
pub unsafe fn convert_to_base2_64(ctx: *mut core::ffi::c_void) {
    let state = &mut *(ctx as *mut poly1305_arch_internal);
    let mut cy: u32;

    if state.base.fields.is_base2_26 == 0 {
        return;
    }

    let h = &mut state.base.fields.h;
    cy = h[0] >> 26; h[0] &= 0x3ffffff; h[1] = h[1].wrapping_add(cy);
    cy = h[1] >> 26; h[1] &= 0x3ffffff; h[2] = h[2].wrapping_add(cy);
    cy = h[2] >> 26; h[2] &= 0x3ffffff; h[3] = h[3].wrapping_add(cy);
    cy = h[3] >> 26; h[3] &= 0x3ffffff; h[4] = h[4].wrapping_add(cy);
    state.base.hs[0] = ((h[2] as u64) << 52) | ((h[1] as u64) << 26) | h[0] as u64;
    state.base.hs[1] = ((h[4] as u64) << 40) | ((h[3] as u64) << 14) | (h[2] >> 12) as u64;
    state.base.hs[2] = (h[4] >> 24) as u64;
    /* Unsigned Less Than: branchlessly produces 1 if a < b, else 0. */
    let ult = |a: u64, b: u64| (a ^ ((a ^ b) | (a.wrapping_sub(b) ^ b))) >> 63;
    cy = ((state.base.hs[2] >> 2) + (state.base.hs[2] & !3u64)) as u32;
    state.base.hs[2] &= 3;
    state.base.hs[0] = state.base.hs[0].wrapping_add(cy as u64);
    let carry = ult(state.base.hs[0], cy as u64);
    state.base.hs[1] = state.base.hs[1].wrapping_add(carry);
    state.base.hs[2] = state.base.hs[2].wrapping_add(ult(state.base.hs[1], carry));
    state.base.fields.is_base2_26 = 0;
}

extern "C" {
    pub fn poly1305_init_x86_64(state: *mut poly1305_block_state, raw_key: *const u8);
    pub fn poly1305_blocks_x86_64(ctx: *mut poly1305_arch_internal, inp: *const u8, len: usize, padbit: u32);
    pub fn poly1305_emit_x86_64(ctx: *const poly1305_state, mac: *mut u8, nonce: *const u32);
    pub fn poly1305_emit_avx(ctx: *const poly1305_state, mac: *mut u8, nonce: *const u32);
    pub fn poly1305_blocks_avx(ctx: *mut poly1305_arch_internal, inp: *const u8, len: usize, padbit: u32);
    pub fn poly1305_blocks_avx2(ctx: *mut poly1305_arch_internal, inp: *const u8, len: usize, padbit: u32);
    pub fn poly1305_blocks_avx512(ctx: *mut poly1305_arch_internal, inp: *const u8, len: usize, padbit: u32);
}

extern "C" {
    pub static mut poly1305_use_avx: static_key_false;
    pub static mut poly1305_use_avx2: static_key_false;
    pub static mut poly1305_use_avx512: static_key_false;
}

pub unsafe fn poly1305_block_init(state: *mut poly1305_block_state, raw_key: *const u8) {
    poly1305_init_x86_64(state, raw_key);
}

pub unsafe fn poly1305_blocks(state: *mut poly1305_block_state, mut inp: *const u8, mut len: u32, padbit: u32) {
    let ctx: *mut poly1305_arch_internal = container_of_block_state(state);
    /* SIMD disables preemption, so relax after processing each page. */
    /* BUILD_BUG_ON(SZ_4K < POLY1305_BLOCK_SIZE || SZ_4K % POLY1305_BLOCK_SIZE); */
    if !static_branch_likely(&poly1305_use_avx) ||
       (len < POLY1305_BLOCK_SIZE * 18 && (*ctx).base.fields.is_base2_26 == 0) ||
       !irq_fpu_usable() {
        convert_to_base2_64(ctx as *mut core::ffi::c_void);
        poly1305_blocks_x86_64(ctx, inp, len as usize, padbit);
        return;
    }
    while len != 0 {
        let bytes = core::cmp::min(len, SZ_4K);
        kernel_fpu_begin();
        if static_branch_likely(&poly1305_use_avx512) { poly1305_blocks_avx512(ctx, inp, bytes as usize, padbit); }
        else if static_branch_likely(&poly1305_use_avx2) { poly1305_blocks_avx2(ctx, inp, bytes as usize, padbit); }
        else { poly1305_blocks_avx(ctx, inp, bytes as usize, padbit); }
        kernel_fpu_end();
        len -= bytes;
        inp = inp.add(bytes as usize);
    }
}

pub unsafe fn poly1305_emit(ctx: *const poly1305_state, mac: *mut u8, nonce: *const u32) {
    if !static_branch_likely(&poly1305_use_avx) { poly1305_emit_x86_64(ctx, mac, nonce); }
    else { poly1305_emit_avx(ctx, mac, nonce); }
}

pub unsafe fn poly1305_mod_init_arch() {
    if boot_cpu_has(X86_FEATURE_AVX) && cpu_has_xfeatures(XFEATURE_MASK_SSE | XFEATURE_MASK_YMM, core::ptr::null_mut()) { static_branch_enable(&mut poly1305_use_avx); }
    if boot_cpu_has(X86_FEATURE_AVX) && boot_cpu_has(X86_FEATURE_AVX2) && cpu_has_xfeatures(XFEATURE_MASK_SSE | XFEATURE_MASK_YMM, core::ptr::null_mut()) { static_branch_enable(&mut poly1305_use_avx2); }
    if boot_cpu_has(X86_FEATURE_AVX) && boot_cpu_has(X86_FEATURE_AVX2) && boot_cpu_has(X86_FEATURE_AVX512F) && cpu_has_xfeatures(XFEATURE_MASK_SSE | XFEATURE_MASK_YMM | XFEATURE_MASK_AVX512, core::ptr::null_mut()) && boot_cpu_data.x86_vfm != INTEL_SKYLAKE_X { static_branch_enable(&mut poly1305_use_avx512); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
