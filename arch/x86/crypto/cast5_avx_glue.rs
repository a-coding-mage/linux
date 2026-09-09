// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Glue Code for the AVX assembler implementation of the Cast5 Cipher
 *
 * Copyright (C) 2012 Johannes Goetzfried
 *     <Johannes.Goetzfried@informatik.stud.uni-erlangen.de>
 */

// C dependencies: <crypto/algapi.h>, <crypto/cast5.h>, <linux/crypto.h>,
// <linux/err.h>, <linux/module.h>, <linux/types.h>, and ecb_cbc_helpers.h.

const CAST5_PARALLEL_BLOCKS: usize = 16;

extern "C" {
    fn cast5_ecb_enc_16way(ctx: *mut cast5_ctx, dst: *mut u8, src: *const u8);
    fn cast5_ecb_dec_16way(ctx: *mut cast5_ctx, dst: *mut u8, src: *const u8);
    fn cast5_cbc_dec_16way(ctx: *mut cast5_ctx, dst: *mut u8, src: *const u8);

    fn cast5_setkey(base: *mut crypto_tfm, key: *const u8, keylen: u32) -> i32;
    fn __cast5_encrypt(ctx: *mut cast5_ctx, dst: *mut u8, src: *const u8);
    fn __cast5_decrypt(ctx: *mut cast5_ctx, dst: *mut u8, src: *const u8);
}

#[repr(C)]
pub struct cast5_ctx {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct crypto_tfm {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct crypto_skcipher {
    pub base: crypto_tfm,
}

#[repr(C)]
pub struct skcipher_request {
    _opaque: [u8; 0],
}

// The following helper macros are supplied by ecb_cbc_helpers.h and remain
// macro invocations so their original walk/block control flow is preserved.
unsafe fn cast5_setkey_skcipher(
    tfm: *mut crypto_skcipher,
    key: *const u8,
    keylen: u32,
) -> i32 {
    cast5_setkey(&mut (*tfm).base, key, keylen)
}

unsafe fn ecb_encrypt(req: *mut skcipher_request) -> i32 {
    ECB_WALK_START!(req, CAST5_BLOCK_SIZE, CAST5_PARALLEL_BLOCKS);
    ECB_BLOCK!(CAST5_PARALLEL_BLOCKS, cast5_ecb_enc_16way);
    ECB_BLOCK!(1, __cast5_encrypt);
    ECB_WALK_END!()
}

unsafe fn ecb_decrypt(req: *mut skcipher_request) -> i32 {
    ECB_WALK_START!(req, CAST5_BLOCK_SIZE, CAST5_PARALLEL_BLOCKS);
    ECB_BLOCK!(CAST5_PARALLEL_BLOCKS, cast5_ecb_dec_16way);
    ECB_BLOCK!(1, __cast5_decrypt);
    ECB_WALK_END!()
}

unsafe fn cbc_encrypt(req: *mut skcipher_request) -> i32 {
    CBC_WALK_START!(req, CAST5_BLOCK_SIZE, -1);
    CBC_ENC_BLOCK!(__cast5_encrypt);
    CBC_WALK_END!()
}

unsafe fn cbc_decrypt(req: *mut skcipher_request) -> i32 {
    CBC_WALK_START!(req, CAST5_BLOCK_SIZE, CAST5_PARALLEL_BLOCKS);
    CBC_DEC_BLOCK!(CAST5_PARALLEL_BLOCKS, cast5_cbc_dec_16way);
    CBC_DEC_BLOCK!(1, __cast5_decrypt);
    CBC_WALK_END!()
}

// Translation of the C skcipher_alg table. Its field layout and constants are
// provided by the kernel crypto dependencies.
static mut cast5_algs: [skcipher_alg; 2] = [
    skcipher_alg {
        base: crypto_alg {
            cra_name: "ecb(cast5)",
            cra_driver_name: "ecb-cast5-avx",
            cra_priority: 200,
            cra_blocksize: CAST5_BLOCK_SIZE,
            cra_ctxsize: core::mem::size_of::<cast5_ctx>(),
            cra_module: THIS_MODULE,
        },
        min_keysize: CAST5_MIN_KEY_SIZE,
        max_keysize: CAST5_MAX_KEY_SIZE,
        setkey: cast5_setkey_skcipher,
        encrypt: ecb_encrypt,
        decrypt: ecb_decrypt,
    },
    skcipher_alg {
        base: crypto_alg {
            cra_name: "cbc(cast5)",
            cra_driver_name: "cbc-cast5-avx",
            cra_priority: 200,
            cra_blocksize: CAST5_BLOCK_SIZE,
            cra_ctxsize: core::mem::size_of::<cast5_ctx>(),
            cra_module: THIS_MODULE,
        },
        min_keysize: CAST5_MIN_KEY_SIZE,
        max_keysize: CAST5_MAX_KEY_SIZE,
        ivsize: CAST5_BLOCK_SIZE,
        setkey: cast5_setkey_skcipher,
        encrypt: cbc_encrypt,
        decrypt: cbc_decrypt,
    },
];

// Build-time kernel definitions and registration helpers are supplied by the
// translated dependency headers.
unsafe fn cast5_init() -> i32 {
    let mut feature_name: *const i8 = core::ptr::null();
    if !cpu_has_xfeatures(XFEATURE_MASK_SSE | XFEATURE_MASK_YMM, &mut feature_name) {
        pr_info!("CPU feature '%s' is not supported.\n", feature_name);
        return -ENODEV;
    }
    crypto_register_skciphers(cast5_algs.as_mut_ptr(), cast5_algs.len())
}

unsafe fn cast5_exit() {
    crypto_unregister_skciphers(cast5_algs.as_mut_ptr(), cast5_algs.len());
}

module_init!(cast5_init);
module_exit!(cast5_exit);

MODULE_DESCRIPTION!("Cast5 Cipher Algorithm, AVX optimized");
MODULE_LICENSE!("GPL");
MODULE_ALIAS_CRYPTO!("cast5");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
