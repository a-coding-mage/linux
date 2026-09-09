/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Hash algorithms.
 *
 * Copyright (c) 2008 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Dependencies supplied by the surrounding crypto implementation are intentionally
// referenced but not defined in this isolated translation.

pub const CRYPTO_AHASH_ALG_BLOCK_ONLY: u32 = 0x01000000;
pub const CRYPTO_AHASH_ALG_FINAL_NONZERO: u32 = 0x02000000;
pub const CRYPTO_AHASH_ALG_FINUP_MAX: u32 = 0x04000000;
pub const CRYPTO_AHASH_ALG_NO_EXPORT_CORE: u32 = 0x08000000;

#[repr(C)]
pub struct ahash_request;

#[repr(C)]
pub struct scatterlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_hash_walk {
    pub data: *const i8,
    pub offset: u32,
    pub flags: u32,
    pub pg: *mut page,
    pub entrylen: u32,
    pub total: u32,
    pub sg: *mut scatterlist,
}

#[repr(C)]
pub struct ahash_instance {
    pub free: Option<unsafe extern "C" fn(inst: *mut ahash_instance)>,
    pub s: ahash_instance_union,
}

#[repr(C)]
pub union ahash_instance_union {
    pub s: ahash_instance_s,
    pub alg: ahash_alg,
}

#[repr(C)]
pub struct ahash_instance_s {
    pub head: [i8; 0],
    pub base: crypto_instance,
}

#[repr(C)]
pub struct shash_instance {
    pub free: Option<unsafe extern "C" fn(inst: *mut shash_instance)>,
    pub s: shash_instance_union,
}

#[repr(C)]
pub union shash_instance_union {
    pub s: shash_instance_s,
    pub alg: shash_alg,
}

#[repr(C)]
pub struct shash_instance_s {
    pub head: [i8; 0],
    pub base: crypto_instance,
}

#[repr(C)]
pub struct crypto_ahash_spawn {
    pub base: crypto_spawn,
}

#[repr(C)]
pub struct crypto_shash_spawn {
    pub base: crypto_spawn,
}

extern "C" {
    pub fn crypto_hash_walk_done(walk: *mut crypto_hash_walk, err: i32) -> i32;
    pub fn crypto_hash_walk_first(req: *mut ahash_request, walk: *mut crypto_hash_walk) -> i32;
    pub fn crypto_register_ahash(alg: *mut ahash_alg) -> i32;
    pub fn crypto_unregister_ahash(alg: *mut ahash_alg);
    pub fn crypto_register_ahashes(algs: *mut ahash_alg, count: i32) -> i32;
    pub fn crypto_unregister_ahashes(algs: *mut ahash_alg, count: i32);
    pub fn ahash_register_instance(tmpl: *mut crypto_template, inst: *mut ahash_instance) -> i32;
    pub fn ahash_free_singlespawn_instance(inst: *mut ahash_instance);
    pub fn shash_no_setkey(tfm: *mut crypto_shash, key: *const u8, keylen: u32) -> i32;
    pub fn crypto_hash_alg_has_setkey(halg: *mut hash_alg_common) -> bool;
    pub fn crypto_grab_ahash(spawn: *mut crypto_ahash_spawn, inst: *mut crypto_instance,
                             name: *const i8, type_: u32, mask: u32) -> i32;
    pub fn crypto_register_shash(alg: *mut shash_alg) -> i32;
    pub fn crypto_unregister_shash(alg: *mut shash_alg);
    pub fn crypto_register_shashes(algs: *mut shash_alg, count: i32) -> i32;
    pub fn crypto_unregister_shashes(algs: *mut shash_alg, count: i32);
    pub fn shash_register_instance(tmpl: *mut crypto_template, inst: *mut shash_instance) -> i32;
    pub fn shash_free_singlespawn_instance(inst: *mut shash_instance);
    pub fn crypto_grab_shash(spawn: *mut crypto_shash_spawn, inst: *mut crypto_instance,
                             name: *const i8, type_: u32, mask: u32) -> i32;
    pub fn shash_ahash_update(req: *mut ahash_request, desc: *mut shash_desc) -> i32;
    pub fn shash_ahash_finup(req: *mut ahash_request, desc: *mut shash_desc) -> i32;
    pub fn shash_ahash_digest(req: *mut ahash_request, desc: *mut shash_desc) -> i32;
    pub fn crypto_ahash_export_core(req: *mut ahash_request, out: *mut core::ffi::c_void) -> i32;
    pub fn crypto_ahash_import_core(req: *mut ahash_request, input: *const core::ffi::c_void) -> i32;
    pub fn crypto_shash_export_core(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32;
    pub fn crypto_shash_import_core(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32;
}

pub type HASH_REQUEST_ZERO = (); // C macro: memzero_explicit(__name##_req, sizeof(__name##_req))
pub type HASH_FBREQ_ON_STACK = (); // C macro requiring a caller-provided stack request buffer

#[inline]
pub unsafe fn crypto_hash_walk_last(walk: *const crypto_hash_walk) -> bool {
    ((*walk).entrylen | (*walk).total) == 0
}

#[inline]
pub unsafe fn crypto_shash_alg_has_setkey(alg: *mut shash_alg) -> bool {
    (*alg).setkey != Some(shash_no_setkey)
}

#[inline]
pub unsafe fn crypto_shash_alg_needs_key(alg: *mut shash_alg) -> bool {
    crypto_shash_alg_has_setkey(alg) && ((*alg).base.cra_flags & CRYPTO_ALG_OPTIONAL_KEY) == 0
}

#[inline]
pub unsafe fn crypto_hash_alg_needs_key(alg: *mut hash_alg_common) -> bool {
    crypto_hash_alg_has_setkey(alg) && ((*alg).base.cra_flags & CRYPTO_ALG_OPTIONAL_KEY) == 0
}

#[inline]
pub unsafe fn crypto_drop_ahash(spawn: *mut crypto_ahash_spawn) {
    crypto_drop_spawn(&mut (*spawn).base);
}

#[inline]
pub unsafe fn crypto_drop_shash(spawn: *mut crypto_shash_spawn) {
    crypto_drop_spawn(&mut (*spawn).base);
}

#[inline]
pub unsafe fn crypto_shash_coresize(tfm: *mut crypto_shash) -> u32 {
    crypto_shash_statesize(tfm) - crypto_shash_blocksize(tfm) - 1
}

#[inline]
pub unsafe fn crypto_hash_no_export_core(tfm: *mut crypto_ahash) -> bool {
    (crypto_hash_alg_common(tfm).as_ref().unwrap().base.cra_flags
        & CRYPTO_AHASH_ALG_NO_EXPORT_CORE) != 0
}

#[inline]
pub unsafe fn crypto_ahash_ctx(tfm: *mut crypto_ahash) -> *mut core::ffi::c_void {
    crypto_tfm_ctx(crypto_ahash_tfm(tfm))
}

#[inline]
pub unsafe fn crypto_ahash_ctx_dma(tfm: *mut crypto_ahash) -> *mut core::ffi::c_void {
    crypto_tfm_ctx_dma(crypto_ahash_tfm(tfm))
}

#[inline]
pub unsafe fn crypto_ahash_set_statesize(tfm: *mut crypto_ahash, size: u32) {
    (*tfm).statesize = size;
}

#[inline]
pub unsafe fn crypto_ahash_set_reqsize(tfm: *mut crypto_ahash, reqsize: u32) {
    (*tfm).reqsize = reqsize;
}

#[inline]
pub unsafe fn crypto_ahash_tested(tfm: *mut crypto_ahash) -> bool {
    (*crypto_ahash_tfm(tfm)).__crt_alg.as_ref().unwrap().cra_flags & CRYPTO_ALG_TESTED != 0
}

#[inline]
pub unsafe fn crypto_ahash_req_virt(tfm: *mut crypto_ahash) -> bool {
    crypto_tfm_req_virt(&mut (*tfm).base)
}

#[inline]
pub unsafe fn crypto_spawn_ahash(spawn: *mut crypto_ahash_spawn) -> *mut crypto_ahash {
    crypto_spawn_tfm2(&mut (*spawn).base)
}

#[inline]
pub unsafe fn crypto_spawn_shash(spawn: *mut crypto_shash_spawn) -> *mut crypto_shash {
    crypto_spawn_tfm2(&mut (*spawn).base)
}

#[inline]
pub unsafe fn crypto_shash_ctx(tfm: *mut crypto_shash) -> *mut core::ffi::c_void {
    crypto_tfm_ctx(&mut (*tfm).base)
}

#[inline]
pub unsafe fn ahash_request_isvirt(req: *mut ahash_request) -> bool {
    ((*req).base.flags & CRYPTO_AHASH_REQ_VIRT) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
