/* SPDX-License-Identifier: GPL-2.0-or-later */
/* AEAD: Authenticated Encryption with Associated Data */

// Types and functions supplied by the corresponding kernel crypto headers are
// intentionally referenced here rather than reimplemented.

pub struct crypto_aead;
pub struct scatterlist;

#[repr(C)]
pub struct aead_request {
    pub base: crypto_async_request,
    pub assoclen: u32,
    pub cryptlen: u32,
    pub iv: *mut u8,
    pub src: *mut scatterlist,
    pub dst: *mut scatterlist,
    pub __ctx: [core::ffi::c_void; 0],
}

#[repr(C)]
pub struct aead_alg {
    pub setkey: Option<unsafe extern "C" fn(*mut crypto_aead, *const u8, u32) -> i32>,
    pub setauthsize: Option<unsafe extern "C" fn(*mut crypto_aead, u32) -> i32>,
    pub encrypt: Option<unsafe extern "C" fn(*mut aead_request) -> i32>,
    pub decrypt: Option<unsafe extern "C" fn(*mut aead_request) -> i32>,
    pub init: Option<unsafe extern "C" fn(*mut crypto_aead) -> i32>,
    pub exit: Option<unsafe extern "C" fn(*mut crypto_aead)>,
    pub ivsize: u32,
    pub maxauthsize: u32,
    pub chunksize: u32,
    pub base: crypto_alg,
}

#[repr(C)]
pub struct crypto_aead_object {
    pub authsize: u32,
    pub reqsize: u32,
    pub base: crypto_tfm,
}

#[repr(C)]
pub struct crypto_sync_aead {
    pub base: crypto_aead_object,
}

pub const MAX_SYNC_AEAD_REQSIZE: usize = 384;

unsafe extern "C" {
    pub fn crypto_alloc_aead(alg_name: *const core::ffi::c_char, type_: u32, mask: u32) -> *mut crypto_aead;
    pub fn crypto_alloc_sync_aead(alg_name: *const core::ffi::c_char, type_: u32, mask: u32) -> *mut crypto_sync_aead;
    pub fn crypto_destroy_tfm(tfm: *mut crypto_aead, base: *mut crypto_tfm);
    pub fn crypto_has_aead(alg_name: *const core::ffi::c_char, type_: u32, mask: u32) -> i32;
    pub fn crypto_aead_setkey(tfm: *mut crypto_aead, key: *const u8, keylen: u32) -> i32;
    pub fn crypto_aead_setauthsize(tfm: *mut crypto_aead, authsize: u32) -> i32;
    pub fn crypto_aead_encrypt(req: *mut aead_request) -> i32;
    pub fn crypto_aead_decrypt(req: *mut aead_request) -> i32;
}

#[inline]
pub unsafe fn __crypto_aead_cast(tfm: *mut crypto_tfm) -> *mut crypto_aead {
    tfm as *mut crypto_aead
}

#[inline]
pub unsafe fn crypto_aead_tfm(tfm: *mut crypto_aead) -> *mut crypto_tfm {
    tfm as *mut crypto_tfm
}

#[inline]
pub unsafe fn crypto_sync_aead_tfm(tfm: *mut crypto_sync_aead) -> *mut crypto_tfm {
    crypto_aead_tfm(&mut (*tfm).base as *mut crypto_aead_object as *mut crypto_aead)
}

#[inline]
pub unsafe fn crypto_free_aead(tfm: *mut crypto_aead) {
    crypto_destroy_tfm(tfm, crypto_aead_tfm(tfm));
}

#[inline]
pub unsafe fn crypto_free_sync_aead(tfm: *mut crypto_sync_aead) {
    crypto_free_aead((&mut (*tfm).base as *mut crypto_aead_object).cast());
}

#[inline]
pub unsafe fn crypto_aead_alg_ivsize(alg: *mut aead_alg) -> u32 { (*alg).ivsize }
#[inline]
pub unsafe fn crypto_aead_ivsize(tfm: *mut crypto_aead) -> u32 { (*((tfm as *mut aead_alg))).ivsize }
#[inline]
pub unsafe fn crypto_sync_aead_ivsize(tfm: *mut crypto_sync_aead) -> u32 { crypto_aead_ivsize((&mut (*tfm).base as *mut crypto_aead_object).cast()) }
#[inline]
pub unsafe fn crypto_aead_authsize(tfm: *mut crypto_aead) -> u32 { (*(tfm as *mut crypto_aead_object)).authsize }
#[inline]
pub unsafe fn crypto_sync_aead_authsize(tfm: *mut crypto_sync_aead) -> u32 { crypto_aead_authsize((&mut (*tfm).base as *mut crypto_aead_object).cast()) }
#[inline]
pub unsafe fn crypto_aead_alg_maxauthsize(alg: *mut aead_alg) -> u32 { (*alg).maxauthsize }
#[inline]
pub unsafe fn crypto_aead_maxauthsize(aead: *mut crypto_aead) -> u32 { crypto_aead_alg_maxauthsize(aead as *mut aead_alg) }
#[inline]
pub unsafe fn crypto_sync_aead_maxauthsize(tfm: *mut crypto_sync_aead) -> u32 { crypto_aead_maxauthsize((&mut (*tfm).base as *mut crypto_aead_object).cast()) }

#[inline]
pub unsafe fn crypto_aead_reqsize(tfm: *mut crypto_aead) -> u32 { (*(tfm as *mut crypto_aead_object)).reqsize }

#[inline]
pub unsafe fn crypto_aead_driver_name(_tfm: *mut crypto_aead) -> *const core::ffi::c_char { core::ptr::null() }
#[inline]
pub unsafe fn crypto_aead_blocksize(_tfm: *mut crypto_aead) -> u32 { 0 }
#[inline]
pub unsafe fn crypto_sync_aead_blocksize(tfm: *mut crypto_sync_aead) -> u32 { crypto_aead_blocksize((&mut (*tfm).base as *mut crypto_aead_object).cast()) }
#[inline]
pub unsafe fn crypto_aead_alignmask(_tfm: *mut crypto_aead) -> u32 { 0 }
#[inline]
pub unsafe fn crypto_aead_get_flags(_tfm: *mut crypto_aead) -> u32 { 0 }
#[inline]
pub unsafe fn crypto_aead_set_flags(_tfm: *mut crypto_aead, _flags: u32) {}
#[inline]
pub unsafe fn crypto_aead_clear_flags(_tfm: *mut crypto_aead, _flags: u32) {}
#[inline]
pub unsafe fn crypto_sync_aead_get_flags(tfm: *mut crypto_sync_aead) -> u32 { crypto_aead_get_flags((&mut (*tfm).base as *mut crypto_aead_object).cast()) }
#[inline]
pub unsafe fn crypto_sync_aead_set_flags(tfm: *mut crypto_sync_aead, flags: u32) { crypto_aead_set_flags((&mut (*tfm).base as *mut crypto_aead_object).cast(), flags); }
#[inline]
pub unsafe fn crypto_sync_aead_clear_flags(tfm: *mut crypto_sync_aead, flags: u32) { crypto_aead_clear_flags((&mut (*tfm).base as *mut crypto_aead_object).cast(), flags); }

#[inline]
pub unsafe fn crypto_sync_aead_setkey(tfm: *mut crypto_sync_aead, key: *const u8, keylen: u32) -> i32 { crypto_aead_setkey((&mut (*tfm).base as *mut crypto_aead_object).cast(), key, keylen) }
#[inline]
pub unsafe fn crypto_sync_aead_setauthsize(tfm: *mut crypto_sync_aead, authsize: u32) -> i32 { crypto_aead_setauthsize((&mut (*tfm).base as *mut crypto_aead_object).cast(), authsize) }

#[inline]
pub unsafe fn crypto_aead_reqtfm(req: *mut aead_request) -> *mut crypto_aead { __crypto_aead_cast((*req).base.tfm) }
#[inline]
pub unsafe fn crypto_sync_aead_reqtfm(req: *mut aead_request) -> *mut crypto_sync_aead { crypto_aead_reqtfm(req) as *mut crypto_sync_aead }

#[inline]
pub unsafe fn aead_request_set_tfm(req: *mut aead_request, tfm: *mut crypto_aead) { (*req).base.tfm = crypto_aead_tfm(tfm); }
#[inline]
pub unsafe fn aead_request_set_sync_tfm(req: *mut aead_request, tfm: *mut crypto_sync_aead) { aead_request_set_tfm(req, (&mut (*tfm).base as *mut crypto_aead_object).cast()); }

#[inline]
pub unsafe fn aead_request_set_crypt(req: *mut aead_request, src: *mut scatterlist, dst: *mut scatterlist, cryptlen: u32, iv: *mut u8) {
    (*req).src = src; (*req).dst = dst; (*req).cryptlen = cryptlen; (*req).iv = iv;
}

#[inline]
pub unsafe fn aead_request_set_ad(req: *mut aead_request, assoclen: u32) { (*req).assoclen = assoclen; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
