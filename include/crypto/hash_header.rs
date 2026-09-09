/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Hash algorithms under the crypto API.  C header dependencies are supplied elsewhere. */

pub const CRYPTO_AHASH_REQ_VIRT: u32 = 0x0000_0001;
pub const CRYPTO_AHASH_REQ_PRIVATE: u32 = CRYPTO_AHASH_REQ_VIRT;

#[repr(C)]
pub struct hash_alg_common {
    pub digestsize: u32,
    pub statesize: u32,
    pub base: crypto_alg,
}

#[repr(C)]
pub union ahash_request_src {
    pub src: *mut scatterlist,
    pub svirt: *const u8,
}

#[repr(C)]
pub struct ahash_request {
    pub base: crypto_async_request,
    pub nbytes: u32,
    pub src: ahash_request_src,
    pub result: *mut u8,
    pub sg_head: [scatterlist; 2],
    pub saved_complete: crypto_completion_t,
    pub saved_data: *mut core::ffi::c_void,
    pub __ctx: [u8; 0],
}

#[repr(C)]
pub struct ahash_alg {
    pub init: Option<unsafe extern "C" fn(*mut ahash_request) -> i32>,
    pub update: Option<unsafe extern "C" fn(*mut ahash_request) -> i32>,
    pub final_: Option<unsafe extern "C" fn(*mut ahash_request) -> i32>,
    pub finup: Option<unsafe extern "C" fn(*mut ahash_request) -> i32>,
    pub digest: Option<unsafe extern "C" fn(*mut ahash_request) -> i32>,
    pub export: Option<unsafe extern "C" fn(*mut ahash_request, *mut core::ffi::c_void) -> i32>,
    pub import: Option<unsafe extern "C" fn(*mut ahash_request, *const core::ffi::c_void) -> i32>,
    pub export_core: Option<unsafe extern "C" fn(*mut ahash_request, *mut core::ffi::c_void) -> i32>,
    pub import_core: Option<unsafe extern "C" fn(*mut ahash_request, *const core::ffi::c_void) -> i32>,
    pub setkey: Option<unsafe extern "C" fn(*mut crypto_ahash, *const u8, u32) -> i32>,
    pub init_tfm: Option<unsafe extern "C" fn(*mut crypto_ahash) -> i32>,
    pub exit_tfm: Option<unsafe extern "C" fn(*mut crypto_ahash)>,
    pub halg: hash_alg_common,
}

#[repr(C)]
pub struct shash_desc {
    pub tfm: *mut crypto_shash,
    pub __ctx: [u8; 0],
}

pub const HASH_MAX_DIGESTSIZE: usize = 64;
pub const fn hash_state_and_block(state: usize, block: usize) -> usize { state + block + 1 }
pub const HASH_MAX_STATESIZE: usize = hash_state_and_block(200, 144);
pub const S390_SHA_CTX_SIZE: usize = 216;
pub const SHA3_224_S390_DESCSIZE: usize = hash_state_and_block(S390_SHA_CTX_SIZE, 144);

#[repr(C)]
pub struct shash_alg {
    pub init: Option<unsafe extern "C" fn(*mut shash_desc) -> i32>,
    pub update: Option<unsafe extern "C" fn(*mut shash_desc, *const u8, u32) -> i32>,
    pub final_: Option<unsafe extern "C" fn(*mut shash_desc, *mut u8) -> i32>,
    pub finup: Option<unsafe extern "C" fn(*mut shash_desc, *const u8, u32, *mut u8) -> i32>,
    pub digest: Option<unsafe extern "C" fn(*mut shash_desc, *const u8, u32, *mut u8) -> i32>,
    pub export: Option<unsafe extern "C" fn(*mut shash_desc, *mut core::ffi::c_void) -> i32>,
    pub import: Option<unsafe extern "C" fn(*mut shash_desc, *const core::ffi::c_void) -> i32>,
    pub export_core: Option<unsafe extern "C" fn(*mut shash_desc, *mut core::ffi::c_void) -> i32>,
    pub import_core: Option<unsafe extern "C" fn(*mut shash_desc, *const core::ffi::c_void) -> i32>,
    pub setkey: Option<unsafe extern "C" fn(*mut crypto_shash, *const u8, u32) -> i32>,
    pub init_tfm: Option<unsafe extern "C" fn(*mut crypto_shash) -> i32>,
    pub exit_tfm: Option<unsafe extern "C" fn(*mut crypto_shash)>,
    pub descsize: u32,
    pub halg: hash_alg_common,
}

#[repr(C)]
pub struct crypto_ahash { pub using_shash: bool, pub statesize: u32, pub reqsize: u32, pub base: crypto_tfm }
#[repr(C)]
pub struct crypto_shash { pub base: crypto_tfm }

pub const fn crypto_hash_statesize(coresize: usize, blocksize: usize) -> usize { coresize + blocksize + 1 }

extern "C" {
    pub fn crypto_alloc_ahash(alg_name: *const i8, type_: u32, mask: u32) -> *mut crypto_ahash;
    pub fn crypto_has_ahash(alg_name: *const i8, type_: u32, mask: u32) -> i32;
    pub fn crypto_ahash_setkey(tfm: *mut crypto_ahash, key: *const u8, keylen: u32) -> i32;
    pub fn crypto_ahash_finup(req: *mut ahash_request) -> i32;
    pub fn crypto_ahash_digest(req: *mut ahash_request) -> i32;
    pub fn crypto_ahash_export(req: *mut ahash_request, out: *mut core::ffi::c_void) -> i32;
    pub fn crypto_ahash_import(req: *mut ahash_request, input: *const core::ffi::c_void) -> i32;
    pub fn crypto_ahash_init(req: *mut ahash_request) -> i32;
    pub fn crypto_ahash_update(req: *mut ahash_request) -> i32;
    pub fn ahash_request_free(req: *mut ahash_request);
    pub fn crypto_alloc_shash(alg_name: *const i8, type_: u32, mask: u32) -> *mut crypto_shash;
    pub fn crypto_has_shash(alg_name: *const i8, type_: u32, mask: u32) -> i32;
    pub fn crypto_shash_setkey(tfm: *mut crypto_shash, key: *const u8, keylen: u32) -> i32;
    pub fn crypto_shash_digest(desc: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32;
    pub fn crypto_shash_tfm_digest(tfm: *mut crypto_shash, data: *const u8, len: u32, out: *mut u8) -> i32;
    pub fn crypto_hash_digest(tfm: *mut crypto_ahash, data: *const u8, len: u32, out: *mut u8) -> i32;
    pub fn crypto_shash_export(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32;
    pub fn crypto_shash_import(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32;
    pub fn crypto_shash_init(desc: *mut shash_desc) -> i32;
    pub fn crypto_shash_finup(desc: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32;
}

/* The remaining inline API is represented with direct Rust equivalents. */
pub unsafe fn crypto_ahash_final(req: *mut ahash_request) -> i32 { (*req).nbytes = 0; crypto_ahash_finup(req) }
pub unsafe fn crypto_shash_update(desc: *mut shash_desc, data: *const u8, len: u32) -> i32 { crypto_shash_finup(desc, data, len, core::ptr::null_mut()) }
pub unsafe fn crypto_shash_final(desc: *mut shash_desc, out: *mut u8) -> i32 { crypto_shash_finup(desc, core::ptr::null(), 0, out) }

pub unsafe fn ahash_request_set_crypt(req: *mut ahash_request, src: *mut scatterlist, result: *mut u8, nbytes: u32) {
    (*req).src.src = src; (*req).nbytes = nbytes; (*req).result = result;
    (*req).base.flags &= !CRYPTO_AHASH_REQ_VIRT;
}
pub unsafe fn ahash_request_set_virt(req: *mut ahash_request, src: *const u8, result: *mut u8, nbytes: u32) {
    (*req).src.svirt = src; (*req).nbytes = nbytes; (*req).result = result;
    (*req).base.flags |= CRYPTO_AHASH_REQ_VIRT;
}
pub unsafe fn ahash_request_set_tfm(req: *mut ahash_request, tfm: *mut crypto_ahash) {
    crypto_request_set_tfm(&mut (*req).base, &mut (*tfm).base);
}
pub unsafe fn ahash_request_ctx(req: *mut ahash_request) -> *mut core::ffi::c_void { (*req).__ctx.as_mut_ptr() as *mut _ }
pub unsafe fn shash_desc_ctx(desc: *mut shash_desc) -> *mut core::ffi::c_void { (*desc).__ctx.as_mut_ptr() as *mut _ }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
