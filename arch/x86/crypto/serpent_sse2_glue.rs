// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Glue Code for SSE2 assembler versions of Serpent Cipher
 *
 * Copyright (c) 2011 Jussi Kivilinna <jussi.kivilinna@mbnet.fi>
 *
 * Glue code based on aesni-intel_glue.c by:
 *  Copyright (C) 2008, Intel Corp.
 *    Author: Huang Ying <ying.huang@intel.com>
 *
 * CBC & ECB parts based on code (crypto/cbc.c,ecb.c) by:
 *   Copyright (c) 2006 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Kernel, crypto, Serpent, SSE2, and ECB/CBC helper declarations are supplied
// by the surrounding translation unit.

extern "C" {
    fn __serpent_setkey(ctx: *mut serpent_ctx, key: *const u8, keylen: c_uint) -> c_int;
    fn serpent_dec_blk_xway(ctx: *const c_void, dst: *mut u8, src: *const u8);
    fn serpent_enc_blk_xway(ctx: *const c_void, dst: *mut u8, src: *const u8);
    fn __serpent_encrypt(ctx: *const c_void, dst: *mut u8, src: *const u8);
    fn __serpent_decrypt(ctx: *const c_void, dst: *mut u8, src: *const u8);
    fn crypto_skcipher_ctx(tfm: *mut crypto_skcipher) -> *mut serpent_ctx;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn crypto_xor(dst: *mut u8, src: *const u8, n: usize);
    fn crypto_register_skciphers(alg: *mut skcipher_alg, n: usize) -> c_int;
    fn crypto_unregister_skciphers(alg: *mut skcipher_alg, n: usize);
    fn boot_cpu_has(feature: c_int) -> bool;
    fn printk(level: c_uint, message: *const u8) -> c_int;
}

use core::ffi::{c_int, c_uint, c_void};

#[repr(C)]
pub struct serpent_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_skcipher {
    _private: [u8; 0],
}

#[repr(C)]
pub struct skcipher_request {
    _private: [u8; 0],
}

#[repr(C)]
pub struct skcipher_alg {
    _private: [u8; 0],
}

const SERPENT_PARALLEL_BLOCKS: usize = 4; // supplied by crypto/serpent.h
const SERPENT_BLOCK_SIZE: usize = 16;
const SERPENT_MIN_KEY_SIZE: usize = 16;
const SERPENT_MAX_KEY_SIZE: usize = 32;
const X86_FEATURE_XMM2: c_int = 26; // supplied by the kernel x86 headers

unsafe fn serpent_setkey_skcipher(
    tfm: *mut crypto_skcipher,
    key: *const u8,
    keylen: c_uint,
) -> c_int {
    __serpent_setkey(crypto_skcipher_ctx(tfm), key, keylen)
}

unsafe fn serpent_decrypt_cbc_xway(ctx: *const c_void, dst: *mut u8, src: *const u8) {
    let mut buf = [[0u8; SERPENT_BLOCK_SIZE]; SERPENT_PARALLEL_BLOCKS - 1];
    let mut s = src;

    if dst == src {
        s = memcpy(
            buf.as_mut_ptr() as *mut c_void,
            src as *const c_void,
            core::mem::size_of_val(&buf),
        ) as *const u8;
    }
    serpent_dec_blk_xway(ctx, dst, src);
    crypto_xor(
        dst.add(SERPENT_BLOCK_SIZE),
        s,
        core::mem::size_of_val(&buf),
    );
}

unsafe fn ecb_encrypt(req: *mut skcipher_request) -> c_int {
    ECB_WALK_START!(req, SERPENT_BLOCK_SIZE, SERPENT_PARALLEL_BLOCKS);
    ECB_BLOCK!(SERPENT_PARALLEL_BLOCKS, serpent_enc_blk_xway);
    ECB_BLOCK!(1, __serpent_encrypt);
    ECB_WALK_END!();
}

unsafe fn ecb_decrypt(req: *mut skcipher_request) -> c_int {
    ECB_WALK_START!(req, SERPENT_BLOCK_SIZE, SERPENT_PARALLEL_BLOCKS);
    ECB_BLOCK!(SERPENT_PARALLEL_BLOCKS, serpent_dec_blk_xway);
    ECB_BLOCK!(1, __serpent_decrypt);
    ECB_WALK_END!();
}

unsafe fn cbc_encrypt(req: *mut skcipher_request) -> c_int {
    CBC_WALK_START!(req, SERPENT_BLOCK_SIZE, -1);
    CBC_ENC_BLOCK!(__serpent_encrypt);
    CBC_WALK_END!();
}

unsafe fn cbc_decrypt(req: *mut skcipher_request) -> c_int {
    CBC_WALK_START!(req, SERPENT_BLOCK_SIZE, SERPENT_PARALLEL_BLOCKS);
    CBC_DEC_BLOCK!(SERPENT_PARALLEL_BLOCKS, serpent_decrypt_cbc_xway);
    CBC_DEC_BLOCK!(1, __serpent_decrypt);
    CBC_WALK_END!();
}

// The C designated initializers below map to the externally supplied
// repr(C) skcipher_alg layout.
static mut serpent_algs: [skcipher_alg; 2] = [
    skcipher_alg { _private: [] },
    skcipher_alg { _private: [] },
];

unsafe fn serpent_sse2_init() -> c_int {
    if !boot_cpu_has(X86_FEATURE_XMM2) {
        printk(0, b"SSE2 instructions are not detected.\0".as_ptr());
        return -19; // -ENODEV
    }

    crypto_register_skciphers(serpent_algs.as_mut_ptr(), serpent_algs.len())
}

unsafe fn serpent_sse2_exit() {
    crypto_unregister_skciphers(serpent_algs.as_mut_ptr(), serpent_algs.len());
}

// module_init(serpent_sse2_init);
// module_exit(serpent_sse2_exit);
// MODULE_DESCRIPTION("Serpent Cipher Algorithm, SSE2 optimized");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_CRYPTO("serpent");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
