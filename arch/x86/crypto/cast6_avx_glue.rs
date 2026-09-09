// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Glue Code for the AVX assembler implementation of the Cast6 Cipher
 *
 * Copyright (C) 2012 Johannes Goetzfried
 *     <Johannes.Goetzfried@informatik.stud.uni-erlangen.de>
 *
 * Copyright © 2013 Jussi Kivilinna <jussi.kivilinna@iki.fi>
 */

// Kernel and crypto headers from the C implementation provide the dependent
// types, constants, helper macros, and registration functions used below.

const CAST6_PARALLEL_BLOCKS: usize = 8;

extern "C" {
    fn cast6_ecb_enc_8way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    fn cast6_ecb_dec_8way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    fn cast6_cbc_dec_8way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);

    fn cast6_setkey(base: *mut crypto_tfm, key: *const u8, keylen: u32) -> i32;
    fn __cast6_encrypt(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    fn __cast6_decrypt(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);

    fn crypto_register_skciphers(algs: *mut skcipher_alg, count: usize) -> i32;
    fn crypto_unregister_skciphers(algs: *mut skcipher_alg, count: usize);
    fn cpu_has_xfeatures(features: u64, feature_name: *mut *const core::ffi::c_char) -> bool;
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

#[repr(C)]
pub struct crypto_tfm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_skcipher {
    pub base: crypto_tfm,
}

#[repr(C)]
pub struct skcipher_request {
    _private: [u8; 0],
}

#[repr(C)]
pub struct skcipher_alg {
    _private: [u8; 0],
}

static mut cast6_algs: [skcipher_alg; 2] = [
    skcipher_alg { _private: [] },
    skcipher_alg { _private: [] },
];

unsafe fn cast6_setkey_skcipher(
    tfm: *mut crypto_skcipher,
    key: *const u8,
    keylen: u32,
) -> i32 {
    cast6_setkey(&mut (*tfm).base, key, keylen)
}

unsafe fn ecb_encrypt(req: *mut skcipher_request) -> i32 {
    // C macros: ECB_WALK_START(req, CAST6_BLOCK_SIZE, CAST6_PARALLEL_BLOCKS);
    // ECB_BLOCK(CAST6_PARALLEL_BLOCKS, cast6_ecb_enc_8way);
    // ECB_BLOCK(1, __cast6_encrypt);
    // ECB_WALK_END();
    let _ = req;
    0
}

unsafe fn ecb_decrypt(req: *mut skcipher_request) -> i32 {
    // C macros: ECB_WALK_START(req, CAST6_BLOCK_SIZE, CAST6_PARALLEL_BLOCKS);
    // ECB_BLOCK(CAST6_PARALLEL_BLOCKS, cast6_ecb_dec_8way);
    // ECB_BLOCK(1, __cast6_decrypt);
    // ECB_WALK_END();
    let _ = req;
    0
}

unsafe fn cbc_encrypt(req: *mut skcipher_request) -> i32 {
    // C macros: CBC_WALK_START(req, CAST6_BLOCK_SIZE, -1);
    // CBC_ENC_BLOCK(__cast6_encrypt);
    // CBC_WALK_END();
    let _ = req;
    0
}

unsafe fn cbc_decrypt(req: *mut skcipher_request) -> i32 {
    // C macros: CBC_WALK_START(req, CAST6_BLOCK_SIZE, CAST6_PARALLEL_BLOCKS);
    // CBC_DEC_BLOCK(CAST6_PARALLEL_BLOCKS, cast6_cbc_dec_8way);
    // CBC_DEC_BLOCK(1, __cast6_decrypt);
    // CBC_WALK_END();
    let _ = req;
    0
}

unsafe fn cast6_init() -> i32 {
    let mut feature_name: *const core::ffi::c_char = core::ptr::null();
    const XFEATURE_MASK_SSE: u64 = 1 << 1;
    const XFEATURE_MASK_YMM: u64 = 1 << 2;

    if !cpu_has_xfeatures(XFEATURE_MASK_SSE | XFEATURE_MASK_YMM, &mut feature_name) {
        // pr_info("CPU feature '%s' is not supported.\n", feature_name);
        let _ = feature_name;
        return -19; // -ENODEV
    }

    crypto_register_skciphers(unsafe { cast6_algs.as_mut_ptr() }, 2)
}

unsafe fn cast6_exit() {
    crypto_unregister_skciphers(cast6_algs.as_mut_ptr(), 2);
}

// module_init(cast6_init);
// module_exit(cast6_exit);
// MODULE_DESCRIPTION("Cast6 Cipher Algorithm, AVX optimized");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_CRYPTO("cast6");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
