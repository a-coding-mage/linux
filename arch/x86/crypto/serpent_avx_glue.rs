// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Glue Code for AVX assembler versions of Serpent Cipher
 *
 * Copyright (C) 2012 Johannes Goetzfried
 *     <Johannes.Goetzfried@informatik.stud.uni-erlangen.de>
 *
 * Copyright © 2011-2013 Jussi Kivilinna <jussi.kivilinna@iki.fi>
 */

// Linux kernel and crypto dependencies supplied by the surrounding crate.

/* 8-way parallel cipher functions */
extern "C" {
    pub fn serpent_ecb_enc_8way_avx(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn serpent_ecb_dec_8way_avx(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn serpent_cbc_dec_8way_avx(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
}

extern "C" {
    fn __serpent_setkey(ctx: *mut core::ffi::c_void, key: *const u8, keylen: u32) -> i32;
    fn __serpent_encrypt(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    fn __serpent_decrypt(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
}

// The following walk operations are provided by the kernel crypto helpers.
unsafe fn serpent_setkey_skcipher(
    tfm: *mut crypto_skcipher,
    key: *const u8,
    keylen: u32,
) -> i32 {
    __serpent_setkey(crypto_skcipher_ctx(tfm), key, keylen)
}

unsafe fn ecb_encrypt(req: *mut skcipher_request) -> i32 {
    ECB_WALK_START!(req, SERPENT_BLOCK_SIZE, SERPENT_PARALLEL_BLOCKS);
    ECB_BLOCK!(SERPENT_PARALLEL_BLOCKS, serpent_ecb_enc_8way_avx);
    ECB_BLOCK!(1, __serpent_encrypt);
    ECB_WALK_END!()
}

unsafe fn ecb_decrypt(req: *mut skcipher_request) -> i32 {
    ECB_WALK_START!(req, SERPENT_BLOCK_SIZE, SERPENT_PARALLEL_BLOCKS);
    ECB_BLOCK!(SERPENT_PARALLEL_BLOCKS, serpent_ecb_dec_8way_avx);
    ECB_BLOCK!(1, __serpent_decrypt);
    ECB_WALK_END!()
}

unsafe fn cbc_encrypt(req: *mut skcipher_request) -> i32 {
    CBC_WALK_START!(req, SERPENT_BLOCK_SIZE, -1);
    CBC_ENC_BLOCK!(__serpent_encrypt);
    CBC_WALK_END!()
}

unsafe fn cbc_decrypt(req: *mut skcipher_request) -> i32 {
    CBC_WALK_START!(req, SERPENT_BLOCK_SIZE, SERPENT_PARALLEL_BLOCKS);
    CBC_DEC_BLOCK!(SERPENT_PARALLEL_BLOCKS, serpent_cbc_dec_8way_avx);
    CBC_DEC_BLOCK!(1, __serpent_decrypt);
    CBC_WALK_END!()
}

static mut serpent_algs: [skcipher_alg; 2] = [
    skcipher_alg {
        base: crypto_alg {
            cra_name: c"ecb(serpent)".as_ptr(),
            cra_driver_name: c"ecb-serpent-avx".as_ptr(),
            cra_priority: 500,
            cra_blocksize: SERPENT_BLOCK_SIZE,
            cra_ctxsize: core::mem::size_of::<serpent_ctx>(),
            cra_module: THIS_MODULE,
        },
        min_keysize: SERPENT_MIN_KEY_SIZE,
        max_keysize: SERPENT_MAX_KEY_SIZE,
        setkey: Some(serpent_setkey_skcipher),
        encrypt: Some(ecb_encrypt),
        decrypt: Some(ecb_decrypt),
    },
    skcipher_alg {
        base: crypto_alg {
            cra_name: c"cbc(serpent)".as_ptr(),
            cra_driver_name: c"cbc-serpent-avx".as_ptr(),
            cra_priority: 500,
            cra_blocksize: SERPENT_BLOCK_SIZE,
            cra_ctxsize: core::mem::size_of::<serpent_ctx>(),
            cra_module: THIS_MODULE,
        },
        min_keysize: SERPENT_MIN_KEY_SIZE,
        max_keysize: SERPENT_MAX_KEY_SIZE,
        ivsize: SERPENT_BLOCK_SIZE,
        setkey: Some(serpent_setkey_skcipher),
        encrypt: Some(cbc_encrypt),
        decrypt: Some(cbc_decrypt),
    },
];

unsafe fn serpent_init() -> i32 {
    let mut feature_name: *const core::ffi::c_char = core::ptr::null();

    if !cpu_has_xfeatures(XFEATURE_MASK_SSE | XFEATURE_MASK_YMM, &mut feature_name) {
        pr_info!("CPU feature '%s' is not supported.\n", feature_name);
        return -ENODEV;
    }

    crypto_register_skciphers(serpent_algs.as_mut_ptr(), serpent_algs.len())
}

unsafe fn serpent_exit() {
    crypto_unregister_skciphers(serpent_algs.as_mut_ptr(), serpent_algs.len());
}

module_init!(serpent_init);
module_exit!(serpent_exit);

module_description!("Serpent Cipher Algorithm, AVX optimized");
module_license!("GPL");
module_alias_crypto!("serpent");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
