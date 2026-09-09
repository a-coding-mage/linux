// SPDX-License-Identifier: GPL-2.0

/*
 * SM4 Cipher Algorithm.
 *
 * Copyright (C) 2018 ARM Limited or its affiliates.
 * All rights reserved.
 */

// Dependencies supplied by the surrounding kernel/Rust environment.

extern "C" {
    fn crypto_tfm_ctx(tfm: *mut crypto_tfm) -> *mut sm4_ctx;
    fn sm4_expandkey(ctx: *mut sm4_ctx, in_key: *const u8, key_len: u32) -> i32;
    fn sm4_crypt_block(rkey: *const u32, out: *mut u8, input: *const u8);
    fn crypto_register_alg(alg: *mut crypto_alg) -> i32;
    fn crypto_unregister_alg(alg: *mut crypto_alg);
}

#[repr(C)]
pub struct crypto_tfm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sm4_ctx {
    pub rkey_enc: [u32; 32],
    pub rkey_dec: [u32; 32],
}

#[repr(C)]
pub struct crypto_alg_cipher {
    pub cia_min_keysize: u32,
    pub cia_max_keysize: u32,
    pub cia_setkey: Option<unsafe extern "C" fn(*mut crypto_tfm, *const u8, u32) -> i32>,
    pub cia_encrypt: Option<unsafe extern "C" fn(*mut crypto_tfm, *mut u8, *const u8)>,
    pub cia_decrypt: Option<unsafe extern "C" fn(*mut crypto_tfm, *mut u8, *const u8)>,
}

#[repr(C)]
pub union crypto_alg_u {
    pub cipher: crypto_alg_cipher,
}

#[repr(C)]
pub struct crypto_alg {
    pub cra_name: *const u8,
    pub cra_driver_name: *const u8,
    pub cra_priority: i32,
    pub cra_flags: u32,
    pub cra_blocksize: u32,
    pub cra_ctxsize: usize,
    pub cra_module: *mut core::ffi::c_void,
    pub cra_u: crypto_alg_u,
}

const SM4_BLOCK_SIZE: u32 = 16;
const SM4_KEY_SIZE: u32 = 16;
const CRYPTO_ALG_TYPE_CIPHER: u32 = 0x0000_0001;

static mut sm4_alg: crypto_alg = crypto_alg {
    cra_name: b"sm4\0".as_ptr(),
    cra_driver_name: b"sm4-generic\0".as_ptr(),
    cra_priority: 100,
    cra_flags: CRYPTO_ALG_TYPE_CIPHER,
    cra_blocksize: SM4_BLOCK_SIZE,
    cra_ctxsize: core::mem::size_of::<sm4_ctx>(),
    cra_module: core::ptr::null_mut(),
    cra_u: crypto_alg_u {
        cipher: crypto_alg_cipher {
            cia_min_keysize: SM4_KEY_SIZE,
            cia_max_keysize: SM4_KEY_SIZE,
            cia_setkey: Some(sm4_setkey),
            cia_encrypt: Some(sm4_encrypt),
            cia_decrypt: Some(sm4_decrypt),
        },
    },
};

/**
 * sm4_setkey - Set the SM4 key.
 * @tfm: The %crypto_tfm that is used in the context.
 * @in_key: The input key.
 * @key_len: The size of the key.
 *
 * This function uses sm4_expandkey() to expand the key.
 * &sm4_ctx _must_ be the private data embedded in @tfm which is
 * retrieved with crypto_tfm_ctx().
 *
 * Return: 0 on success; -EINVAL on failure (only happens for bad key lengths)
 */
unsafe extern "C" fn sm4_setkey(
    tfm: *mut crypto_tfm,
    in_key: *const u8,
    key_len: u32,
) -> i32 {
    let ctx = crypto_tfm_ctx(tfm);

    sm4_expandkey(ctx, in_key, key_len)
}

/* encrypt a block of text */

unsafe extern "C" fn sm4_encrypt(tfm: *mut crypto_tfm, out: *mut u8, input: *const u8) {
    let ctx = crypto_tfm_ctx(tfm);

    sm4_crypt_block((*ctx).rkey_enc.as_ptr(), out, input);
}

/* decrypt a block of text */

unsafe extern "C" fn sm4_decrypt(tfm: *mut crypto_tfm, out: *mut u8, input: *const u8) {
    let ctx = crypto_tfm_ctx(tfm);

    sm4_crypt_block((*ctx).rkey_dec.as_ptr(), out, input);
}

unsafe extern "C" fn sm4_init() -> i32 {
    crypto_register_alg(&raw mut sm4_alg)
}

unsafe extern "C" fn sm4_fini() {
    crypto_unregister_alg(&raw mut sm4_alg);
}

// module_init(sm4_init);
// module_exit(sm4_fini);
// MODULE_DESCRIPTION("SM4 Cipher Algorithm");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS_CRYPTO("sm4");
// MODULE_ALIAS_CRYPTO("sm4-generic");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
