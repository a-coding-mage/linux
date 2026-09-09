/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Crypto engine API
 *
 * Copyright (c) 2016 Baolin Wang <baolin.wang@linaro.org>
 */

// Dependencies supplied by the corresponding crypto and Linux type headers.
#[repr(C)]
pub struct crypto_engine;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct aead_alg;
#[repr(C)]
pub struct ahash_alg;
#[repr(C)]
pub struct akcipher_alg;
#[repr(C)]
pub struct kpp_alg;
#[repr(C)]
pub struct skcipher_alg;
#[repr(C)]
pub struct aead_request;
#[repr(C)]
pub struct akcipher_request;
#[repr(C)]
pub struct ahash_request;
#[repr(C)]
pub struct kpp_request;
#[repr(C)]
pub struct skcipher_request;

/*
 * struct crypto_engine_op - crypto hardware engine operations
 * @do_one_request: do encryption for current request
 */
#[repr(C)]
pub struct crypto_engine_op {
    pub do_one_request: Option<unsafe extern "C" fn(engine: *mut crypto_engine, areq: *mut core::ffi::c_void) -> i32>,
}

#[repr(C)]
pub struct aead_engine_alg {
    pub base: aead_alg,
    pub op: crypto_engine_op,
}

#[repr(C)]
pub struct ahash_engine_alg {
    pub base: ahash_alg,
    pub op: crypto_engine_op,
}

#[repr(C)]
pub struct akcipher_engine_alg {
    pub base: akcipher_alg,
    pub op: crypto_engine_op,
}

#[repr(C)]
pub struct kpp_engine_alg {
    pub base: kpp_alg,
    pub op: crypto_engine_op,
}

#[repr(C)]
pub struct skcipher_engine_alg {
    pub base: skcipher_alg,
    pub op: crypto_engine_op,
}

extern "C" {
    pub fn crypto_transfer_aead_request_to_engine(engine: *mut crypto_engine, req: *mut aead_request) -> i32;
    pub fn crypto_transfer_akcipher_request_to_engine(engine: *mut crypto_engine, req: *mut akcipher_request) -> i32;
    pub fn crypto_transfer_hash_request_to_engine(engine: *mut crypto_engine, req: *mut ahash_request) -> i32;
    pub fn crypto_transfer_kpp_request_to_engine(engine: *mut crypto_engine, req: *mut kpp_request) -> i32;
    pub fn crypto_transfer_skcipher_request_to_engine(engine: *mut crypto_engine, req: *mut skcipher_request) -> i32;
    pub fn crypto_finalize_aead_request(engine: *mut crypto_engine, req: *mut aead_request, err: i32);
    pub fn crypto_finalize_akcipher_request(engine: *mut crypto_engine, req: *mut akcipher_request, err: i32);
    pub fn crypto_finalize_hash_request(engine: *mut crypto_engine, req: *mut ahash_request, err: i32);
    pub fn crypto_finalize_kpp_request(engine: *mut crypto_engine, req: *mut kpp_request, err: i32);
    pub fn crypto_finalize_skcipher_request(engine: *mut crypto_engine, req: *mut skcipher_request, err: i32);
    pub fn crypto_engine_start(engine: *mut crypto_engine) -> i32;
    pub fn crypto_engine_stop(engine: *mut crypto_engine) -> i32;
    pub fn crypto_engine_alloc_init(dev: *mut device, rt: bool) -> *mut crypto_engine;
    pub fn crypto_engine_alloc_init_and_set(dev: *mut device, retry_support: bool, rt: bool, qlen: i32) -> *mut crypto_engine;
    pub fn crypto_engine_exit(engine: *mut crypto_engine);

    pub fn crypto_engine_register_aead(alg: *mut aead_engine_alg) -> i32;
    pub fn crypto_engine_unregister_aead(alg: *mut aead_engine_alg);
    pub fn crypto_engine_register_aeads(algs: *mut aead_engine_alg, count: i32) -> i32;
    pub fn crypto_engine_unregister_aeads(algs: *mut aead_engine_alg, count: i32);

    pub fn crypto_engine_register_ahash(alg: *mut ahash_engine_alg) -> i32;
    pub fn crypto_engine_unregister_ahash(alg: *mut ahash_engine_alg);
    pub fn crypto_engine_register_ahashes(algs: *mut ahash_engine_alg, count: i32) -> i32;
    pub fn crypto_engine_unregister_ahashes(algs: *mut ahash_engine_alg, count: i32);

    pub fn crypto_engine_register_akcipher(alg: *mut akcipher_engine_alg) -> i32;
    pub fn crypto_engine_unregister_akcipher(alg: *mut akcipher_engine_alg);

    pub fn crypto_engine_register_kpp(alg: *mut kpp_engine_alg) -> i32;
    pub fn crypto_engine_unregister_kpp(alg: *mut kpp_engine_alg);

    pub fn crypto_engine_register_skcipher(alg: *mut skcipher_engine_alg) -> i32;
    pub fn crypto_engine_unregister_skcipher(alg: *mut skcipher_engine_alg);
    pub fn crypto_engine_register_skciphers(algs: *mut skcipher_engine_alg, count: i32) -> i32;
    pub fn crypto_engine_unregister_skciphers(algs: *mut skcipher_engine_alg, count: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
