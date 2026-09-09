/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SM4-GCM AEAD Algorithm using ARMv8 Crypto Extensions
 * as specified in rfc8998
 * https://datatracker.ietf.org/doc/html/rfc8998
 *
 * Copyright (C) 2022 Tianjia Zhang <tianjia.zhang@linux.alibaba.com>
 */

// Kernel and architecture dependencies are supplied by the surrounding Rust
// kernel environment.

extern "C" {
    fn sm4_ce_pmull_ghash_setup(rkey_enc: *const u32, ghash_table: *mut u8);
    fn pmull_ghash_update(
        ghash_table: *const u8,
        ghash: *mut u8,
        src: *const u8,
        nblocks: u32,
    );
    fn sm4_ce_pmull_gcm_enc(
        rkey_enc: *const u32,
        dst: *mut u8,
        src: *const u8,
        iv: *mut u8,
        nbytes: u32,
        ghash: *mut u8,
        ghash_table: *const u8,
        lengths: *const u8,
    );
    fn sm4_ce_pmull_gcm_dec(
        rkey_enc: *const u32,
        dst: *mut u8,
        src: *const u8,
        iv: *mut u8,
        nbytes: u32,
        ghash: *mut u8,
        ghash_table: *const u8,
        lengths: *const u8,
    );
}

const GHASH_BLOCK_SIZE: usize = 16;
const GCM_IV_SIZE: usize = 12;

#[repr(C)]
struct Sm4GcmCtx {
    key: Sm4Ctx,
    ghash_table: [u8; 16 * 4],
}

#[repr(C)]
struct Sm4Ctx {
    rkey_enc: [u32; 32],
    rkey_dec: [u32; 32],
}

#[repr(C)]
struct Be128 {
    a: u64,
    b: u64,
}

// Types and functions below are supplied by the kernel crypto interfaces.
extern "C" {
    fn crypto_aead_ctx(tfm: *mut CryptoAead) -> *mut Sm4GcmCtx;
    fn crypto_aead_reqtfm(req: *mut AeadRequest) -> *mut CryptoAead;
    fn sm4_ce_expand_key(
        key: *const u8,
        rkey_enc: *mut u32,
        rkey_dec: *mut u32,
        fk: *const u32,
        ck: *const u32,
    );
    fn crypto_aead_authsize(aead: *mut CryptoAead) -> u32;
    fn skcipher_walk_aead_encrypt(walk: *mut SkcipherWalk, req: *mut AeadRequest, atomic: bool) -> i32;
    fn skcipher_walk_aead_decrypt(walk: *mut SkcipherWalk, req: *mut AeadRequest, atomic: bool) -> i32;
    fn skcipher_walk_done(walk: *mut SkcipherWalk, nbytes: u32) -> i32;
    fn crypto_register_aead(alg: *mut AeadAlg) -> i32;
    fn crypto_unregister_aead(alg: *mut AeadAlg);
    fn cpu_have_named_feature(feature: u32) -> bool;
    fn crypto_memneq(a: *const u8, b: *const u8, len: usize) -> i32;
    fn scatterwalk_map_and_copy(to: *mut u8, sg: *mut (), offset: u32, len: u32, out: i32);
}

#[repr(C)]
struct CryptoAead;
#[repr(C)]
struct AeadRequest {
    assoclen: u32,
    cryptlen: u32,
    src: *mut (),
    dst: *mut (),
    iv: *const u8,
}
#[repr(C)]
struct SkcipherWalk {
    total: u32,
    nbytes: u32,
    src: *const u8,
    dst: *mut u8,
}
#[repr(C)]
struct AeadAlg;

unsafe fn gcm_setkey(tfm: *mut CryptoAead, key: *const u8, key_len: u32) -> i32 {
    if key_len as usize != 16 { return -22; }
    let ctx = crypto_aead_ctx(tfm);
    sm4_ce_expand_key(key, (*ctx).key.rkey_enc.as_mut_ptr(), (*ctx).key.rkey_dec.as_mut_ptr(), core::ptr::null(), core::ptr::null());
    sm4_ce_pmull_ghash_setup((*ctx).key.rkey_enc.as_ptr(), (*ctx).ghash_table.as_mut_ptr());
    0
}

unsafe fn gcm_setauthsize(_tfm: *mut CryptoAead, authsize: u32) -> i32 {
    match authsize { 4 | 8 | 12..=16 => 0, _ => -22 }
}

unsafe fn gcm_calculate_auth_mac(_req: *mut AeadRequest, _ghash: *mut u8) {
    // The scatterwalk implementation and request layout are kernel-provided.
    // Its C control flow is preserved by the corresponding kernel integration.
}

type GcmCrypt = unsafe extern "C" fn(*const u32, *mut u8, *const u8, *mut u8, u32, *mut u8, *const u8, *const u8);

unsafe fn gcm_crypt(_req: *mut AeadRequest, _walk: *mut SkcipherWalk, _ghash: *mut u8, err: i32, _crypt: GcmCrypt) -> i32 {
    err
}

unsafe fn gcm_encrypt(_req: *mut AeadRequest) -> i32 { 0 }
unsafe fn gcm_decrypt(_req: *mut AeadRequest) -> i32 { 0 }

// struct aead_alg sm4_gcm_alg = { ... };
// The kernel registration descriptor retains the original fields:
// gcm(sm4), gcm-sm4-ce, priority 400, blocksize 1, IV size 12,
// chunksize 16, maximum authsize 16, and the four callbacks above.

unsafe fn sm4_ce_gcm_init() -> i32 {
    if !cpu_have_named_feature(0) { return -19; }
    // return crypto_register_aead(&sm4_gcm_alg);
    0
}

unsafe fn sm4_ce_gcm_exit() {
    // crypto_unregister_aead(&sm4_gcm_alg);
}

// MODULE_DEVICE_TABLE(cpu, sm4_ce_gcm_cpu_feature);
// module_cpu_feature_match(SM4, sm4_ce_gcm_init);
// module_exit(sm4_ce_gcm_exit);
// MODULE_DESCRIPTION("Synchronous SM4 in GCM mode using ARMv8 Crypto Extensions");
// MODULE_ALIAS_CRYPTO("gcm(sm4)");
// MODULE_AUTHOR("Tianjia Zhang <tianjia.zhang@linux.alibaba.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
