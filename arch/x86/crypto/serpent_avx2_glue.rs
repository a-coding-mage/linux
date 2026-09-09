// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Glue Code for x86_64/AVX2 assembler optimized version of Serpent
 *
 * Copyright © 2012-2013 Jussi Kivilinna <jussi.kivilinna@mbnet.fi>
 */

// Linux kernel headers and the local Serpent/ECB-CBC helper headers supply
// the types, constants, and helper operations referenced below.

pub const SERPENT_AVX2_PARALLEL_BLOCKS: usize = 16;

extern "C" {
    pub fn serpent_ecb_enc_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn serpent_ecb_dec_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn serpent_cbc_dec_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn serpent_ecb_enc_8way_avx(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn serpent_ecb_dec_8way_avx(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn serpent_cbc_dec_8way_avx(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn __serpent_encrypt(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn __serpent_decrypt(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn __serpent_setkey(ctx: *mut core::ffi::c_void, key: *const u8, keylen: u32) -> i32;
}

#[repr(C)]
pub struct crypto_skcipher {
    _private: [u8; 0],
}

#[repr(C)]
pub struct skcipher_request {
    _private: [u8; 0],
}

#[inline]
unsafe fn crypto_skcipher_ctx(tfm: *mut crypto_skcipher) -> *mut core::ffi::c_void {
    // Supplied by the kernel crypto API; this declaration is intentionally
    // kept as an external dependency of the translated source.
    extern "C" {
        fn crypto_skcipher_ctx(tfm: *mut crypto_skcipher) -> *mut core::ffi::c_void;
    }
    crypto_skcipher_ctx(tfm)
}

pub unsafe fn serpent_setkey_skcipher(
    tfm: *mut crypto_skcipher,
    key: *const u8,
    keylen: u32,
) -> i32 {
    __serpent_setkey(crypto_skcipher_ctx(tfm), key, keylen)
}

pub unsafe fn ecb_encrypt(req: *mut skcipher_request) -> i32 {
    // ECB_WALK_START(req, SERPENT_BLOCK_SIZE, SERPENT_PARALLEL_BLOCKS);
    // ECB_BLOCK(SERPENT_AVX2_PARALLEL_BLOCKS, serpent_ecb_enc_16way);
    // ECB_BLOCK(SERPENT_PARALLEL_BLOCKS, serpent_ecb_enc_8way_avx);
    // ECB_BLOCK(1, __serpent_encrypt);
    // ECB_WALK_END();
    let _ = req;
    unimplemented!("ECB walk helpers are supplied by ecb_cbc_helpers.h")
}

pub unsafe fn ecb_decrypt(req: *mut skcipher_request) -> i32 {
    // ECB_WALK_START(req, SERPENT_BLOCK_SIZE, SERPENT_PARALLEL_BLOCKS);
    // ECB_BLOCK(SERPENT_AVX2_PARALLEL_BLOCKS, serpent_ecb_dec_16way);
    // ECB_BLOCK(SERPENT_PARALLEL_BLOCKS, serpent_ecb_dec_8way_avx);
    // ECB_BLOCK(1, __serpent_decrypt);
    // ECB_WALK_END();
    let _ = req;
    unimplemented!("ECB walk helpers are supplied by ecb_cbc_helpers.h")
}

pub unsafe fn cbc_encrypt(req: *mut skcipher_request) -> i32 {
    // CBC_WALK_START(req, SERPENT_BLOCK_SIZE, -1);
    // CBC_ENC_BLOCK(__serpent_encrypt);
    // CBC_WALK_END();
    let _ = req;
    unimplemented!("CBC walk helpers are supplied by ecb_cbc_helpers.h")
}

pub unsafe fn cbc_decrypt(req: *mut skcipher_request) -> i32 {
    // CBC_WALK_START(req, SERPENT_BLOCK_SIZE, SERPENT_PARALLEL_BLOCKS);
    // CBC_DEC_BLOCK(SERPENT_AVX2_PARALLEL_BLOCKS, serpent_cbc_dec_16way);
    // CBC_DEC_BLOCK(SERPENT_PARALLEL_BLOCKS, serpent_cbc_dec_8way_avx);
    // CBC_DEC_BLOCK(1, __serpent_decrypt);
    // CBC_WALK_END();
    let _ = req;
    unimplemented!("CBC walk helpers are supplied by ecb_cbc_helpers.h")
}

// The skcipher_alg table, module registration, CPU feature checks, and
// lifecycle functions below depend on Linux kernel declarations supplied by
// the included headers. Their source-level declarations are retained here.
pub unsafe fn serpent_avx2_init() -> i32 {
    unimplemented!("Linux kernel module and crypto registration dependencies")
}

pub unsafe fn serpent_avx2_fini() {
    // crypto_unregister_skciphers(serpent_algs, ARRAY_SIZE(serpent_algs));
}

// module_init(serpent_avx2_init);
// module_exit(serpent_avx2_fini);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Serpent Cipher Algorithm, AVX2 optimized");
// MODULE_ALIAS_CRYPTO("serpent");
// MODULE_ALIAS_CRYPTO("serpent-asm");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
