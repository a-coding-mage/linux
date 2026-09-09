/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Public Key Signature Algorithm
 *
 * Copyright (c) 2023 Herbert Xu <herbert@gondor.apana.org.au>
 */

use core::ffi::c_void;

/* Dependency supplied by linux/crypto.h. */
#[repr(C)]
pub struct crypto_tfm {
    pub __crt_alg: *mut crypto_alg,
}

#[repr(C)]
pub struct crypto_alg {
    _private: [u8; 0],
}

/**
 * struct crypto_sig - user-instantiated objects which encapsulate
 * algorithms and core processing logic
 */
#[repr(C)]
pub struct crypto_sig {
    pub base: crypto_tfm,
}

/** Generic public key signature algorithm. */
#[repr(C)]
pub struct sig_alg {
    pub sign: Option<unsafe extern "C" fn(
        tfm: *mut crypto_sig,
        src: *const c_void,
        slen: u32,
        dst: *mut c_void,
        dlen: u32,
    ) -> i32>,
    pub verify: Option<unsafe extern "C" fn(
        tfm: *mut crypto_sig,
        src: *const c_void,
        slen: u32,
        digest: *const c_void,
        dlen: u32,
    ) -> i32>,
    pub set_pub_key: Option<unsafe extern "C" fn(
        tfm: *mut crypto_sig,
        key: *const c_void,
        keylen: u32,
    ) -> i32>,
    pub set_priv_key: Option<unsafe extern "C" fn(
        tfm: *mut crypto_sig,
        key: *const c_void,
        keylen: u32,
    ) -> i32>,
    pub key_size: Option<unsafe extern "C" fn(tfm: *mut crypto_sig) -> u32>,
    pub digest_size: Option<unsafe extern "C" fn(tfm: *mut crypto_sig) -> u32>,
    pub max_size: Option<unsafe extern "C" fn(tfm: *mut crypto_sig) -> u32>,
    pub init: Option<unsafe extern "C" fn(tfm: *mut crypto_sig) -> i32>,
    pub exit: Option<unsafe extern "C" fn(tfm: *mut crypto_sig)>,
    pub base: crypto_alg,
}

extern "C" {
    pub fn crypto_alloc_sig(alg_name: *const i8, type_: u32, mask: u32) -> *mut crypto_sig;
    pub fn crypto_destroy_tfm( tfm: *mut crypto_sig, base: *mut crypto_tfm);
}

#[inline]
pub unsafe fn crypto_sig_tfm(tfm: *mut crypto_sig) -> *mut crypto_tfm {
    &mut (*tfm).base
}

#[inline]
pub unsafe fn __crypto_sig_tfm(tfm: *mut crypto_tfm) -> *mut crypto_sig {
    container_of!(tfm, crypto_sig, base)
}

#[inline]
pub unsafe fn __crypto_sig_alg(alg: *mut crypto_alg) -> *mut sig_alg {
    container_of!(alg, sig_alg, base)
}

#[inline]
pub unsafe fn crypto_sig_alg(tfm: *mut crypto_sig) -> *mut sig_alg {
    __crypto_sig_alg((*crypto_sig_tfm(tfm)).__crt_alg)
}

#[inline]
pub unsafe fn crypto_free_sig(tfm: *mut crypto_sig) {
    crypto_destroy_tfm(tfm, crypto_sig_tfm(tfm));
}

#[inline]
pub unsafe fn crypto_sig_keysize(tfm: *mut crypto_sig) -> u32 {
    let alg = crypto_sig_alg(tfm);
    ((*alg).key_size.unwrap())(tfm)
}

#[inline]
pub unsafe fn crypto_sig_digestsize(tfm: *mut crypto_sig) -> u32 {
    let alg = crypto_sig_alg(tfm);
    ((*alg).digest_size.unwrap())(tfm)
}

#[inline]
pub unsafe fn crypto_sig_maxsize(tfm: *mut crypto_sig) -> u32 {
    let alg = crypto_sig_alg(tfm);
    ((*alg).max_size.unwrap())(tfm)
}

#[inline]
pub unsafe fn crypto_sig_sign(
    tfm: *mut crypto_sig,
    src: *const c_void,
    slen: u32,
    dst: *mut c_void,
    dlen: u32,
) -> i32 {
    let alg = crypto_sig_alg(tfm);
    ((*alg).sign.unwrap())(tfm, src, slen, dst, dlen)
}

#[inline]
pub unsafe fn crypto_sig_verify(
    tfm: *mut crypto_sig,
    src: *const c_void,
    slen: u32,
    digest: *const c_void,
    dlen: u32,
) -> i32 {
    let alg = crypto_sig_alg(tfm);
    ((*alg).verify.unwrap())(tfm, src, slen, digest, dlen)
}

#[inline]
pub unsafe fn crypto_sig_set_pubkey(
    tfm: *mut crypto_sig,
    key: *const c_void,
    keylen: u32,
) -> i32 {
    let alg = crypto_sig_alg(tfm);
    ((*alg).set_pub_key.unwrap())(tfm, key, keylen)
}

#[inline]
pub unsafe fn crypto_sig_set_privkey(
    tfm: *mut crypto_sig,
    key: *const c_void,
    keylen: u32,
) -> i32 {
    let alg = crypto_sig_alg(tfm);
    ((*alg).set_priv_key.unwrap())(tfm, key, keylen)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
