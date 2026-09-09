/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Cryptographic API.
 *
 * Copyright (c) 2023 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Declarations supplied by <crypto/internal/skcipher.h> and "internal.h".
// These opaque types correspond to the C declarations used by this header.
#[repr(C)]
pub struct skcipher_request {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_tfm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct skcipher_alg_common {
    _private: [u8; 0],
}

extern "C" {
    pub fn crypto_lskcipher_encrypt_sg(req: *mut skcipher_request) -> ::std::os::raw::c_int;
    pub fn crypto_lskcipher_decrypt_sg(req: *mut skcipher_request) -> ::std::os::raw::c_int;
    pub fn crypto_init_lskcipher_ops_sg(tfm: *mut crypto_tfm) -> ::std::os::raw::c_int;
    pub fn skcipher_prepare_alg_common(
        alg: *mut skcipher_alg_common,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
