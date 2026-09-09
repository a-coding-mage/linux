// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Glue Code for assembler optimized version of Blowfish
 *
 * Copyright (c) 2011 Jussi Kivilinna <jussi.kivilinna@mbnet.fi>
 *
 * CBC & ECB parts based on code (crypto/cbc.c,ecb.c) by:
 *   Copyright (c) 2006 Herbert Xu <herbert@gondor.apana.org.au>
 */

// C headers and the local ECB/CBC helper macros provide the external kernel
// types, constants, functions, and walk operations referenced below.

#[allow(non_camel_case_types)]
pub type u8 = ::core::ffi::c_uchar;

extern "C" {
    pub fn blowfish_enc_blk(ctx: *mut bf_ctx, dst: *mut u8, src: *const u8);
    pub fn blowfish_dec_blk(ctx: *mut bf_ctx, dst: *mut u8, src: *const u8);
    pub fn blowfish_enc_blk_4way(ctx: *mut bf_ctx, dst: *mut u8, src: *const u8);
    pub fn __blowfish_dec_blk_4way(
        ctx: *mut bf_ctx,
        dst: *mut u8,
        src: *const u8,
        cbc: bool,
    );
    pub fn blowfish_setkey(tfm: *mut crypto_tfm, key: *const u8, keylen: c_uint) -> c_int;
    pub fn crypto_tfm_ctx(tfm: *mut crypto_tfm) -> *mut bf_ctx;
}

pub type c_int = ::core::ffi::c_int;
pub type c_uint = ::core::ffi::c_uint;

#[repr(C)]
pub struct bf_ctx { _private: [u8; 0] }
#[repr(C)]
pub struct crypto_tfm { _private: [u8; 0] }
#[repr(C)]
pub struct crypto_skcipher { pub base: crypto_tfm }
#[repr(C)]
pub struct skcipher_request { _private: [u8; 0] }

#[inline]
unsafe fn blowfish_dec_ecb_4way(ctx: *mut bf_ctx, dst: *mut u8, src: *const u8) {
    __blowfish_dec_blk_4way(ctx, dst, src, false);
}

#[inline]
unsafe fn blowfish_dec_cbc_4way(ctx: *mut bf_ctx, dst: *mut u8, src: *const u8) {
    __blowfish_dec_blk_4way(ctx, dst, src, true);
}

unsafe fn blowfish_encrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    blowfish_enc_blk(crypto_tfm_ctx(tfm), dst, src);
}

unsafe fn blowfish_decrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    blowfish_dec_blk(crypto_tfm_ctx(tfm), dst, src);
}

unsafe fn blowfish_setkey_skcipher(
    tfm: *mut crypto_skcipher,
    key: *const u8,
    keylen: c_uint,
) -> c_int {
    blowfish_setkey(&mut (*tfm).base, key, keylen)
}

unsafe fn ecb_encrypt(req: *mut skcipher_request) -> c_int {
    // ECB_WALK_START(req, BF_BLOCK_SIZE, -1);
    // ECB_BLOCK(4, blowfish_enc_blk_4way);
    // ECB_BLOCK(1, blowfish_enc_blk);
    // ECB_WALK_END();
    let _ = req;
    unimplemented!("ecb helper macros are supplied by ecb_cbc_helpers.h");
}

unsafe fn ecb_decrypt(req: *mut skcipher_request) -> c_int {
    // ECB_WALK_START(req, BF_BLOCK_SIZE, -1);
    // ECB_BLOCK(4, blowfish_dec_ecb_4way);
    // ECB_BLOCK(1, blowfish_dec_blk);
    // ECB_WALK_END();
    let _ = req;
    unimplemented!("ecb helper macros are supplied by ecb_cbc_helpers.h");
}

unsafe fn cbc_encrypt(req: *mut skcipher_request) -> c_int {
    // CBC_WALK_START(req, BF_BLOCK_SIZE, -1);
    // CBC_ENC_BLOCK(blowfish_enc_blk);
    // CBC_WALK_END();
    let _ = req;
    unimplemented!("cbc helper macros are supplied by ecb_cbc_helpers.h");
}

unsafe fn cbc_decrypt(req: *mut skcipher_request) -> c_int {
    // CBC_WALK_START(req, BF_BLOCK_SIZE, -1);
    // CBC_DEC_BLOCK(4, blowfish_dec_cbc_4way);
    // CBC_DEC_BLOCK(1, blowfish_dec_blk);
    // CBC_WALK_END();
    let _ = req;
    unimplemented!("cbc helper macros are supplied by ecb_cbc_helpers.h");
}

// The crypto_alg and skcipher_alg initializers are kernel structures supplied
// by the included Linux headers; their field-for-field initializers are kept
// here as declarations of the corresponding external registration objects.
extern "C" {
    static mut bf_cipher_alg: ::core::ffi::c_void;
    static mut bf_skcipher_algs: ::core::ffi::c_void;
    static mut force: c_int;
    static boot_cpu_data: BootCpuData;
    fn crypto_register_alg(alg: *mut ::core::ffi::c_void) -> c_int;
    fn crypto_unregister_alg(alg: *mut ::core::ffi::c_void);
    fn crypto_register_skciphers(algs: *mut ::core::ffi::c_void, n: usize) -> c_int;
    fn crypto_unregister_skciphers(algs: *mut ::core::ffi::c_void, n: usize);
    fn printk(fmt: *const u8, ...) -> c_int;
}

#[repr(C)]
struct BootCpuData { x86_vendor: c_int, x86: c_int }

unsafe fn is_blacklisted_cpu() -> bool {
    if boot_cpu_data.x86_vendor != X86_VENDOR_INTEL { return false; }
    if boot_cpu_data.x86 == 0x0f {
        /* On Pentium 4, blowfish-x86_64 is slower than generic C because of
         * 64-bit rotates, which are really slow on P4. */
        return true;
    }
    false
}

const X86_VENDOR_INTEL: c_int = 0;

unsafe fn blowfish_init() -> c_int {
    let mut err: c_int;
    if force == 0 && is_blacklisted_cpu() {
        return -19; // -ENODEV
    }
    err = crypto_register_alg(&mut bf_cipher_alg);
    if err != 0 { return err; }
    err = crypto_register_skciphers(&mut bf_skcipher_algs, 2);
    if err != 0 { crypto_unregister_alg(&mut bf_cipher_alg); }
    err
}

unsafe fn blowfish_fini() {
    crypto_unregister_alg(&mut bf_cipher_alg);
    crypto_unregister_skciphers(&mut bf_skcipher_algs, 2);
}

// module_init(blowfish_init); module_exit(blowfish_fini);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Blowfish Cipher Algorithm, asm optimized");
// MODULE_ALIAS_CRYPTO("blowfish");
// MODULE_ALIAS_CRYPTO("blowfish-asm");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
