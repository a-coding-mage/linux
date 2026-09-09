/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * RNG: Random Number Generator algorithms under the crypto API
 *
 * Copyright (c) 2008 Neil Horman <nhorman@tuxdriver.com>
 * Copyright (c) 2015 Herbert Xu <herbert@gondor.apana.org.au>
 */

use core::ffi::c_void;

/* Dependencies supplied by the corresponding crypto headers. */
#[repr(C)]
pub struct crypto_tfm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_rng {
    pub base: crypto_tfm,
}

#[repr(C)]
pub struct rng_alg {
    pub set_ent: Option<unsafe extern "C" fn(*mut crypto_rng, *const u8, u32)>,
}

extern "C" {
    pub fn crypto_tfm_ctx(tfm: *mut crypto_tfm) -> *mut c_void;
    pub fn crypto_rng_alg(tfm: *mut crypto_rng) -> *mut rng_alg;

    pub fn crypto_register_rng(alg: *mut rng_alg) -> i32;
    pub fn crypto_unregister_rng(alg: *mut rng_alg);
    pub fn crypto_register_rngs(algs: *mut rng_alg, count: i32) -> i32;
    pub fn crypto_unregister_rngs(algs: *mut rng_alg, count: i32);

    /* Available when CONFIG_CRYPTO_RNG or CONFIG_CRYPTO_RNG_MODULE is set. */
    #[cfg(any(feature = "CONFIG_CRYPTO_RNG", feature = "CONFIG_CRYPTO_RNG_MODULE"))]
    pub fn crypto_del_default_rng() -> i32;
}

/* When CONFIG_CRYPTO_RNG and CONFIG_CRYPTO_RNG_MODULE are unset. */
#[cfg(not(any(feature = "CONFIG_CRYPTO_RNG", feature = "CONFIG_CRYPTO_RNG_MODULE")))]
#[inline]
pub const fn crypto_del_default_rng() -> i32 {
    0
}

#[inline]
pub unsafe fn crypto_rng_ctx(tfm: *mut crypto_rng) -> *mut c_void {
    crypto_tfm_ctx(&mut (*tfm).base)
}

#[inline]
pub unsafe fn crypto_rng_set_entropy(tfm: *mut crypto_rng, data: *const u8, len: u32) {
    if let Some(set_ent) = (*crypto_rng_alg(tfm)).set_ent {
        set_ent(tfm, data, len);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
