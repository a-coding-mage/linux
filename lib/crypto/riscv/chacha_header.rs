/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ChaCha stream cipher (RISC-V optimized)
 *
 * Copyright (C) 2023 SiFive, Inc.
 * Author: Jerry Shih <jerry.shih@sifive.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct chacha_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct static_key_false {
    _private: [u8; 0],
}

pub const CHACHA_BLOCK_SIZE: usize = 64;

// __ro_after_init DEFINE_STATIC_KEY_FALSE(use_zvkb);
static mut use_zvkb: static_key_false = static_key_false { _private: [] };

extern "C" {
    pub fn chacha_zvkb(
        state: *mut chacha_state,
        input: *const u8,
        output: *mut u8,
        nblocks: usize,
        nrounds: i32,
    );

    pub fn chacha_crypt_generic(
        state: *mut chacha_state,
        dst: *mut u8,
        src: *const u8,
        bytes: u32,
        nrounds: i32,
    );

    pub fn static_branch_likely(key: *const static_key_false) -> bool;
    pub fn crypto_simd_usable() -> bool;
    pub fn kernel_vector_begin();
    pub fn kernel_vector_end();
    pub fn riscv_isa_extension_available(cpu: *const core::ffi::c_void, extension: i32) -> bool;
    pub fn riscv_vector_vlen() -> i32;
    pub fn static_branch_enable(key: *mut static_key_false);
    pub fn memcpy(dst: *mut u8, src: *const u8, count: usize) -> *mut core::ffi::c_void;
}

// hchacha_block_arch hchacha_block_generic /* not implemented yet */
pub use hchacha_block_generic as hchacha_block_arch;

extern "C" {
    pub fn hchacha_block_generic();
}

pub unsafe fn chacha_crypt_arch(
    state: *mut chacha_state,
    mut dst: *mut u8,
    mut src: *const u8,
    bytes: u32,
    nrounds: i32,
) {
    let mut block_buffer = [0u8; CHACHA_BLOCK_SIZE];
    let full_blocks = bytes / CHACHA_BLOCK_SIZE as u32;
    let tail_bytes = bytes % CHACHA_BLOCK_SIZE as u32;

    if !static_branch_likely(&raw const use_zvkb) || !crypto_simd_usable() {
        return chacha_crypt_generic(state, dst, src, bytes, nrounds);
    }

    kernel_vector_begin();
    if full_blocks != 0 {
        chacha_zvkb(state, src, dst, full_blocks as usize, nrounds);
        src = src.add(full_blocks as usize * CHACHA_BLOCK_SIZE);
        dst = dst.add(full_blocks as usize * CHACHA_BLOCK_SIZE);
    }
    if tail_bytes != 0 {
        memcpy(block_buffer.as_mut_ptr(), src, tail_bytes as usize);
        chacha_zvkb(state, block_buffer.as_ptr(), block_buffer.as_mut_ptr(), 1, nrounds);
        memcpy(dst, block_buffer.as_ptr(), tail_bytes as usize);
    }
    kernel_vector_end();
}

// chacha_mod_init_arch chacha_mod_init_arch
pub unsafe fn chacha_mod_init_arch() {
    // Build-time ISA extension constant supplied by the surrounding translation unit.
    const ZVKB: i32 = 0;
    if riscv_isa_extension_available(core::ptr::null(), ZVKB) && riscv_vector_vlen() >= 128 {
        static_branch_enable(&raw mut use_zvkb);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
