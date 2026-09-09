/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * RSA internal helpers
 *
 * Copyright (c) 2015, Intel Corporation
 * Authors: Tadeusz Struk <tadeusz.struk@intel.com>
 */

// Dependencies supplied by the surrounding translation unit:
// linux/types.h and crypto/akcipher.h

/**
 * rsa_key - RSA key structure
 * @n           : RSA modulus raw byte stream
 * @e           : RSA public exponent raw byte stream
 * @d           : RSA private exponent raw byte stream
 * @p           : RSA prime factor p of n raw byte stream
 * @q           : RSA prime factor q of n raw byte stream
 * @dp          : RSA exponent d mod (p - 1) raw byte stream
 * @dq          : RSA exponent d mod (q - 1) raw byte stream
 * @qinv        : RSA CRT coefficient q^(-1) mod p raw byte stream
 * @n_sz        : length in bytes of RSA modulus n
 * @e_sz        : length in bytes of RSA public exponent
 * @d_sz        : length in bytes of RSA private exponent
 * @p_sz        : length in bytes of p field
 * @q_sz        : length in bytes of q field
 * @dp_sz       : length in bytes of dp field
 * @dq_sz       : length in bytes of dq field
 * @qinv_sz     : length in bytes of qinv field
 */
#[repr(C)]
pub struct rsa_key {
    pub n: *const u8,
    pub e: *const u8,
    pub d: *const u8,
    pub p: *const u8,
    pub q: *const u8,
    pub dp: *const u8,
    pub dq: *const u8,
    pub qinv: *const u8,
    pub n_sz: usize,
    pub e_sz: usize,
    pub d_sz: usize,
    pub p_sz: usize,
    pub q_sz: usize,
    pub dp_sz: usize,
    pub dq_sz: usize,
    pub qinv_sz: usize,
}

unsafe extern "C" {
    pub fn rsa_parse_pub_key(
        rsa_key: *mut rsa_key,
        key: *const core::ffi::c_void,
        key_len: core::ffi::c_uint,
    ) -> core::ffi::c_int;

    pub fn rsa_parse_priv_key(
        rsa_key: *mut rsa_key,
        key: *const core::ffi::c_void,
        key_len: core::ffi::c_uint,
    ) -> core::ffi::c_int;
}

pub const RSA_PUB: bool = true;
pub const RSA_PRIV: bool = false;

pub unsafe fn rsa_set_key(
    child: *mut crypto_akcipher,
    key_size: *mut core::ffi::c_uint,
    is_pubkey: bool,
    key: *const core::ffi::c_void,
    keylen: core::ffi::c_uint,
) -> core::ffi::c_int {
    let mut err: core::ffi::c_int;

    *key_size = 0;

    if is_pubkey {
        err = crypto_akcipher_set_pub_key(child, key, keylen);
    } else {
        err = crypto_akcipher_set_priv_key(child, key, keylen);
    }
    if err != 0 {
        return err;
    }

    /* Find out new modulus size from rsa implementation */
    err = crypto_akcipher_maxsize(child);
    if err > PAGE_SIZE {
        return -ENOTSUPP;
    }

    *key_size = err as core::ffi::c_uint;
    0
}

unsafe extern "C" {
    pub static mut rsa_pkcs1pad_tmpl: crypto_template;
    pub static mut rsassa_pkcs1_tmpl: crypto_template;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
