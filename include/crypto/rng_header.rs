/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * RNG: Random Number Generator algorithms under the crypto API
 *
 * Copyright (c) 2008 Neil Horman <nhorman@tuxdriver.com>
 * Copyright (c) 2015 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Dependencies supplied by the Linux crypto API and related headers are
// intentionally referenced here but not implemented in this translation.

use core::ffi::c_void;

#[repr(C)]
pub struct crypto_rng {
    pub base: crypto_tfm,
}

#[repr(C)]
pub struct rng_alg {
    pub generate: Option<unsafe extern "C" fn(
        tfm: *mut crypto_rng,
        src: *const u8,
        slen: u32,
        dst: *mut u8,
        dlen: u32,
    ) -> i32>,
    pub seed: Option<unsafe extern "C" fn(
        tfm: *mut crypto_rng,
        seed: *const u8,
        slen: u32,
    ) -> i32>,
    pub set_ent: Option<unsafe extern "C" fn(
        tfm: *mut crypto_rng,
        data: *const u8,
        len: u32,
    )>,
    pub seedsize: u32,
    pub base: crypto_alg,
}

extern "C" {
    pub static fips_enabled: bool;

    pub fn __crypto_stdrng_get_bytes(buf: *mut c_void, len: u32) -> i32;
    pub fn might_sleep();
    pub fn get_random_bytes_wait(buf: *mut c_void, len: u32) -> i32;

    pub fn crypto_alloc_rng(alg_name: *const i8, type_: u32, mask: u32) -> *mut crypto_rng;
    pub fn crypto_destroy_tfm(tfm: *mut crypto_rng, base: *mut crypto_tfm);
    pub fn crypto_rng_reset(tfm: *mut crypto_rng, seed: *const u8, slen: u32) -> i32;
}

// These types are provided by the Linux crypto API header.
#[repr(C)]
pub struct crypto_tfm {
    pub __crt_alg: *mut crypto_alg,
}

#[repr(C)]
pub struct crypto_alg {
    _private: [u8; 0],
}

#[inline]
pub unsafe fn crypto_stdrng_get_bytes(buf: *mut c_void, len: u32) -> i32 {
    might_sleep();
    if fips_enabled {
        __crypto_stdrng_get_bytes(buf, len)
    } else {
        get_random_bytes_wait(buf, len)
    }
}

#[inline]
pub unsafe fn crypto_rng_tfm(tfm: *mut crypto_rng) -> *mut crypto_tfm {
    &mut (*tfm).base
}

#[inline]
pub unsafe fn __crypto_rng_alg(alg: *mut crypto_alg) -> *mut rng_alg {
    // Equivalent to container_of(alg, struct rng_alg, base).
    (alg as *mut u8).sub(core::mem::offset_of!(rng_alg, base)) as *mut rng_alg
}

#[inline]
pub unsafe fn crypto_rng_alg(tfm: *mut crypto_rng) -> *mut rng_alg {
    __crypto_rng_alg((*crypto_rng_tfm(tfm)).__crt_alg)
}

#[inline]
pub unsafe fn crypto_free_rng(tfm: *mut crypto_rng) {
    crypto_destroy_tfm(tfm, crypto_rng_tfm(tfm));
}

#[inline]
pub unsafe fn crypto_rng_generate(
    tfm: *mut crypto_rng,
    src: *const u8,
    slen: u32,
    dst: *mut u8,
    dlen: u32,
) -> i32 {
    ((*crypto_rng_alg(tfm)).generate.unwrap())(tfm, src, slen, dst, dlen)
}

#[inline]
pub unsafe fn crypto_rng_get_bytes(tfm: *mut crypto_rng, rdata: *mut u8, dlen: u32) -> i32 {
    crypto_rng_generate(tfm, core::ptr::null(), 0, rdata, dlen)
}

#[inline]
pub unsafe fn crypto_rng_seedsize(tfm: *mut crypto_rng) -> i32 {
    (*crypto_rng_alg(tfm)).seedsize as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
