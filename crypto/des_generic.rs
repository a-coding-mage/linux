// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Cryptographic API.
 *
 * DES & Triple DES EDE Cipher Algorithms.
 *
 * Copyright (c) 2005 Dag Arne Osvik <da@osvik.no>
 */

// Dependencies supplied by the surrounding kernel crypto implementation.

extern "C" {
    fn des_expand_key(dctx: *mut des_ctx, key: *const u8, keylen: c_uint) -> c_int;
    fn des_encrypt(dctx: *const des_ctx, dst: *mut u8, src: *const u8);
    fn des_decrypt(dctx: *const des_ctx, dst: *mut u8, src: *const u8);
    fn des3_ede_expand_key(dctx: *mut des3_ede_ctx, key: *const u8, keylen: c_uint) -> c_int;
    fn des3_ede_encrypt(dctx: *const des3_ede_ctx, dst: *mut u8, src: *const u8);
    fn des3_ede_decrypt(dctx: *const des3_ede_ctx, dst: *mut u8, src: *const u8);
    fn crypto_tfm_get_flags(tfm: *mut crypto_tfm) -> c_uint;
    fn crypto_register_algs(algs: *mut crypto_alg, count: usize) -> c_int;
    fn crypto_unregister_algs(algs: *mut crypto_alg, count: usize);
}

use core::ffi::{c_int, c_uint};

#[repr(C)]
pub struct crypto_tfm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct des_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct des3_ede_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_alg {
    pub cra_name: *const u8,
    pub cra_driver_name: *const u8,
    pub cra_priority: c_int,
    pub cra_flags: c_uint,
    pub cra_blocksize: c_uint,
    pub cra_ctxsize: usize,
    pub cra_module: *mut core::ffi::c_void,
    pub cra_u: crypto_alg_u,
}

#[repr(C)]
pub union crypto_alg_u {
    pub cipher: crypto_cipher,
}

#[repr(C)]
pub struct crypto_cipher {
    pub cia_min_keysize: c_uint,
    pub cia_max_keysize: c_uint,
    pub cia_setkey: Option<unsafe extern "C" fn(*mut crypto_tfm, *const u8, c_uint) -> c_int>,
    pub cia_encrypt: Option<unsafe extern "C" fn(*mut crypto_tfm, *mut u8, *const u8)>,
    pub cia_decrypt: Option<unsafe extern "C" fn(*mut crypto_tfm, *mut u8, *const u8)>,
}

const ENOKEY: c_int = 126;
const EINVAL: c_int = 22;
const CRYPTO_TFM_REQ_FORBID_WEAK_KEYS: c_uint = 1 << 2;
const CRYPTO_ALG_TYPE_CIPHER: c_uint = 1;
const DES_BLOCK_SIZE: c_uint = 8;
const DES_KEY_SIZE: c_uint = 8;
const DES3_EDE_BLOCK_SIZE: c_uint = 8;
const DES3_EDE_KEY_SIZE: c_uint = 24;

unsafe fn crypto_tfm_ctx<T>(tfm: *mut crypto_tfm) -> *mut T {
    tfm as *mut T
}

unsafe extern "C" fn des_setkey(tfm: *mut crypto_tfm, key: *const u8, keylen: c_uint) -> c_int {
    let dctx = crypto_tfm_ctx::<des_ctx>(tfm);
    let mut err = des_expand_key(dctx, key, keylen);
    if err == -ENOKEY {
        if crypto_tfm_get_flags(tfm) & CRYPTO_TFM_REQ_FORBID_WEAK_KEYS != 0 {
            err = -EINVAL;
        } else {
            err = 0;
        }
    }
    if err != 0 {
        core::ptr::write_bytes(dctx as *mut u8, 0, core::mem::size_of::<des_ctx>());
    }
    err
}

unsafe extern "C" fn crypto_des_encrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    des_encrypt(crypto_tfm_ctx::<des_ctx>(tfm), dst, src);
}

unsafe extern "C" fn crypto_des_decrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    des_decrypt(crypto_tfm_ctx::<des_ctx>(tfm), dst, src);
}

unsafe extern "C" fn des3_ede_setkey(tfm: *mut crypto_tfm, key: *const u8, keylen: c_uint) -> c_int {
    let dctx = crypto_tfm_ctx::<des3_ede_ctx>(tfm);
    let mut err = des3_ede_expand_key(dctx, key, keylen);
    if err == -ENOKEY {
        if crypto_tfm_get_flags(tfm) & CRYPTO_TFM_REQ_FORBID_WEAK_KEYS != 0 { err = -EINVAL; } else { err = 0; }
    }
    if err != 0 { core::ptr::write_bytes(dctx as *mut u8, 0, core::mem::size_of::<des3_ede_ctx>()); }
    err
}

unsafe extern "C" fn crypto_des3_ede_encrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    des3_ede_encrypt(crypto_tfm_ctx::<des3_ede_ctx>(tfm), dst, src);
}

unsafe extern "C" fn crypto_des3_ede_decrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    des3_ede_decrypt(crypto_tfm_ctx::<des3_ede_ctx>(tfm), dst, src);
}

static mut des_algs: [crypto_alg; 2] = [
    crypto_alg {
        cra_name: b"des\0".as_ptr(),
        cra_driver_name: b"des-generic\0".as_ptr(),
        cra_priority: 100,
        cra_flags: CRYPTO_ALG_TYPE_CIPHER,
        cra_blocksize: DES_BLOCK_SIZE,
        cra_ctxsize: core::mem::size_of::<des_ctx>(),
        cra_module: core::ptr::null_mut(),
        cra_u: crypto_alg_u { cipher: crypto_cipher {
            cia_min_keysize: DES_KEY_SIZE,
            cia_max_keysize: DES_KEY_SIZE,
            cia_setkey: Some(des_setkey),
            cia_encrypt: Some(crypto_des_encrypt),
            cia_decrypt: Some(crypto_des_decrypt),
        }},
    },
    crypto_alg {
        cra_name: b"des3_ede\0".as_ptr(),
        cra_driver_name: b"des3_ede-generic\0".as_ptr(),
        cra_priority: 100,
        cra_flags: CRYPTO_ALG_TYPE_CIPHER,
        cra_blocksize: DES3_EDE_BLOCK_SIZE,
        cra_ctxsize: core::mem::size_of::<des3_ede_ctx>(),
        cra_module: core::ptr::null_mut(),
        cra_u: crypto_alg_u { cipher: crypto_cipher {
            cia_min_keysize: DES3_EDE_KEY_SIZE,
            cia_max_keysize: DES3_EDE_KEY_SIZE,
            cia_setkey: Some(des3_ede_setkey),
            cia_encrypt: Some(crypto_des3_ede_encrypt),
            cia_decrypt: Some(crypto_des3_ede_decrypt),
        }},
    },
];

unsafe extern "C" fn des_generic_mod_init() -> c_int {
    crypto_register_algs(des_algs.as_mut_ptr(), des_algs.len())
}

unsafe extern "C" fn des_generic_mod_fini() {
    crypto_unregister_algs(des_algs.as_mut_ptr(), des_algs.len());
}

// module_init(des_generic_mod_init);
// module_exit(des_generic_mod_fini);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("DES & Triple DES EDE Cipher Algorithms");
// MODULE_AUTHOR("Dag Arne Osvik <da@osvik.no>");
// MODULE_ALIAS_CRYPTO("des");
// MODULE_ALIAS_CRYPTO("des-generic");
// MODULE_ALIAS_CRYPTO("des3_ede");
// MODULE_ALIAS_CRYPTO("des3_ede-generic");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
