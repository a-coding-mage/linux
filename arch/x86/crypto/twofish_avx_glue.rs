// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Glue Code for AVX assembler version of Twofish Cipher
 *
 * Copyright (C) 2012 Johannes Goetzfried
 *     <Johannes.Goetzfried@informatik.stud.uni-erlangen.de>
 *
 * Copyright © 2013 Jussi Kivilinna <jussi.kivilinna@iki.fi>
 */

// Linux and crypto definitions are supplied by the surrounding kernel crate.

const TWOFISH_PARALLEL_BLOCKS: usize = 8;

extern "C" {
    fn twofish_ecb_enc_8way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    fn twofish_ecb_dec_8way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    fn twofish_cbc_dec_8way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
}

unsafe fn twofish_setkey_skcipher(
    tfm: *mut crypto_skcipher,
    key: *const u8,
    keylen: u32,
) -> i32 {
    twofish_setkey(&mut (*tfm).base, key, keylen)
}

#[inline]
unsafe fn twofish_enc_blk_3way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8) {
    __twofish_enc_blk_3way(ctx, dst, src, false);
}

unsafe fn ecb_encrypt(req: *mut skcipher_request) -> i32 {
    ECB_WALK_START!(req, TF_BLOCK_SIZE, TWOFISH_PARALLEL_BLOCKS);
    ECB_BLOCK!(TWOFISH_PARALLEL_BLOCKS, twofish_ecb_enc_8way);
    ECB_BLOCK!(3, twofish_enc_blk_3way);
    ECB_BLOCK!(1, twofish_enc_blk);
    ECB_WALK_END!();
}

unsafe fn ecb_decrypt(req: *mut skcipher_request) -> i32 {
    ECB_WALK_START!(req, TF_BLOCK_SIZE, TWOFISH_PARALLEL_BLOCKS);
    ECB_BLOCK!(TWOFISH_PARALLEL_BLOCKS, twofish_ecb_dec_8way);
    ECB_BLOCK!(3, twofish_dec_blk_3way);
    ECB_BLOCK!(1, twofish_dec_blk);
    ECB_WALK_END!();
}

unsafe fn cbc_encrypt(req: *mut skcipher_request) -> i32 {
    CBC_WALK_START!(req, TF_BLOCK_SIZE, -1);
    CBC_ENC_BLOCK!(twofish_enc_blk);
    CBC_WALK_END!();
}

unsafe fn cbc_decrypt(req: *mut skcipher_request) -> i32 {
    CBC_WALK_START!(req, TF_BLOCK_SIZE, TWOFISH_PARALLEL_BLOCKS);
    CBC_DEC_BLOCK!(TWOFISH_PARALLEL_BLOCKS, twofish_cbc_dec_8way);
    CBC_DEC_BLOCK!(3, twofish_dec_blk_cbc_3way);
    CBC_DEC_BLOCK!(1, twofish_dec_blk);
    CBC_WALK_END!();
}

// The following table is the direct translation of the C skcipher_alg array;
// its kernel-provided type and field definitions are supplied by dependencies.
static mut twofish_algs: [skcipher_alg; 2] = [
    skcipher_alg {
        base: crypto_alg {
            cra_name: b"ecb(twofish)\0".as_ptr() as *const i8,
            cra_driver_name: b"ecb-twofish-avx\0".as_ptr() as *const i8,
            cra_priority: 400,
            cra_blocksize: TF_BLOCK_SIZE,
            cra_ctxsize: core::mem::size_of::<twofish_ctx>(),
            cra_module: THIS_MODULE,
        },
        min_keysize: TF_MIN_KEY_SIZE,
        max_keysize: TF_MAX_KEY_SIZE,
        setkey: Some(twofish_setkey_skcipher),
        encrypt: Some(ecb_encrypt),
        decrypt: Some(ecb_decrypt),
    },
    skcipher_alg {
        base: crypto_alg {
            cra_name: b"cbc(twofish)\0".as_ptr() as *const i8,
            cra_driver_name: b"cbc-twofish-avx\0".as_ptr() as *const i8,
            cra_priority: 400,
            cra_blocksize: TF_BLOCK_SIZE,
            cra_ctxsize: core::mem::size_of::<twofish_ctx>(),
            cra_module: THIS_MODULE,
        },
        min_keysize: TF_MIN_KEY_SIZE,
        max_keysize: TF_MAX_KEY_SIZE,
        ivsize: TF_BLOCK_SIZE,
        setkey: Some(twofish_setkey_skcipher),
        encrypt: Some(cbc_encrypt),
        decrypt: Some(cbc_decrypt),
    },
];

unsafe fn twofish_init() -> i32 {
    let mut feature_name: *const i8 = core::ptr::null();
    if !cpu_has_xfeatures(XFEATURE_MASK_SSE | XFEATURE_MASK_YMM, &mut feature_name) {
        pr_info!("CPU feature '%s' is not supported.\n", feature_name);
        return -ENODEV;
    }
    crypto_register_skciphers(twofish_algs.as_mut_ptr(), twofish_algs.len())
}

unsafe fn twofish_exit() {
    crypto_unregister_skciphers(twofish_algs.as_mut_ptr(), twofish_algs.len());
}

module_init!(twofish_init);
module_exit!(twofish_exit);

MODULE_DESCRIPTION!("Twofish Cipher Algorithm, AVX optimized");
MODULE_LICENSE!("GPL");
MODULE_ALIAS_CRYPTO!("twofish");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
