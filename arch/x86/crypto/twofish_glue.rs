/*
 * Glue Code for assembler optimized version of TWOFISH
 *
 * Originally Twofish for GPG
 * By Matthew Skala <mskala@ansuz.sooke.bc.ca>, July 26, 1998
 * 256-bit key length added March 20, 1999
 * Some modifications to reduce the text size by Werner Koch, April, 1998
 * Ported to the kerneli patch by Marc Mutz <Marc@Mutz.com>
 * Ported to CryptoAPI by Colin Slater <hoho@tacomeat.net>
 *
 * The original author has disclaimed all copyright interest in this
 * code and thus put it in the public domain. The subsequent authors
 * have put this under the GNU General Public License.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 */

// Kernel headers supplying these types, constants, macros, and functions are
// external dependencies of this translation.

#[allow(non_camel_case_types)]
pub type u8 = ::core::ffi::c_uchar;

#[repr(C)]
pub struct twofish_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_tfm {
    _private: [u8; 0],
}

extern "C" {
    pub fn twofish_enc_blk(ctx: *mut twofish_ctx, dst: *mut u8, src: *const u8);
    pub fn twofish_dec_blk(ctx: *mut twofish_ctx, dst: *mut u8, src: *const u8);
    pub fn twofish_setkey();
    pub fn crypto_tfm_ctx(tfm: *mut crypto_tfm) -> *mut twofish_ctx;
    pub fn crypto_register_alg(alg: *mut crypto_alg) -> ::core::ffi::c_int;
    pub fn crypto_unregister_alg(alg: *mut crypto_alg);
}

#[repr(C)]
pub struct crypto_cipher {
    pub cia_min_keysize: usize,
    pub cia_max_keysize: usize,
    pub cia_setkey: Option<unsafe extern "C" fn()>,
    pub cia_encrypt: Option<unsafe extern "C" fn(*mut crypto_tfm, *mut u8, *const u8)>,
    pub cia_decrypt: Option<unsafe extern "C" fn(*mut crypto_tfm, *mut u8, *const u8)>,
}

#[repr(C)]
pub union crypto_alg_union {
    pub cipher: crypto_cipher,
}

#[repr(C)]
pub struct crypto_alg {
    pub cra_name: *const u8,
    pub cra_driver_name: *const u8,
    pub cra_priority: ::core::ffi::c_int,
    pub cra_flags: u32,
    pub cra_blocksize: usize,
    pub cra_ctxsize: usize,
    pub cra_module: *mut ::core::ffi::c_void,
    pub cra_u: crypto_alg_union,
}

const TF_BLOCK_SIZE: usize = 16;
const TF_MIN_KEY_SIZE: usize = 16;
const TF_MAX_KEY_SIZE: usize = 32;

unsafe extern "C" fn twofish_encrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    twofish_enc_blk(crypto_tfm_ctx(tfm), dst, src);
}

unsafe extern "C" fn twofish_decrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    twofish_dec_blk(crypto_tfm_ctx(tfm), dst, src);
}

static mut alg: crypto_alg = crypto_alg {
    cra_name: b"twofish\0".as_ptr(),
    cra_driver_name: b"twofish-asm\0".as_ptr(),
    cra_priority: 200,
    cra_flags: CRYPTO_ALG_TYPE_CIPHER,
    cra_blocksize: TF_BLOCK_SIZE,
    cra_ctxsize: core::mem::size_of::<twofish_ctx>(),
    cra_module: core::ptr::null_mut(),
    cra_u: crypto_alg_union {
        cipher: crypto_cipher {
            cia_min_keysize: TF_MIN_KEY_SIZE,
            cia_max_keysize: TF_MAX_KEY_SIZE,
            cia_setkey: Some(twofish_setkey),
            cia_encrypt: Some(twofish_encrypt),
            cia_decrypt: Some(twofish_decrypt),
        },
    },
};

unsafe extern "C" fn twofish_glue_init() -> ::core::ffi::c_int {
    crypto_register_alg(&mut alg)
}

unsafe extern "C" fn twofish_glue_fini() {
    crypto_unregister_alg(&mut alg);
}

// Equivalent kernel module registration and metadata declarations.
#[used]
#[no_mangle]
pub static __twofish_glue_init: unsafe extern "C" fn() -> ::core::ffi::c_int = twofish_glue_init;
#[used]
#[no_mangle]
pub static __twofish_glue_fini: unsafe extern "C" fn() = twofish_glue_fini;
#[allow(dead_code)]
const MODULE_LICENSE: &str = "GPL";
#[allow(dead_code)]
const MODULE_DESCRIPTION: &str = "Twofish Cipher Algorithm, asm optimized";
#[allow(dead_code)]
const MODULE_ALIAS: [&str; 2] = ["twofish", "twofish-asm"];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
