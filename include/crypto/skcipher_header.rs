/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Symmetric key ciphers. */

/* Dependencies supplied by the surrounding kernel translation. */

pub const CRYPTO_LSKCIPHER_FLAG_CONT: u32 = 0x00000001;
pub const CRYPTO_LSKCIPHER_FLAG_FINAL: u32 = 0x00000002;
pub const CRYPTO_SKCIPHER_REQ_CONT: u32 = 0x00000001;
pub const CRYPTO_SKCIPHER_REQ_NOTFINAL: u32 = 0x00000002;
pub const MAX_SYNC_SKCIPHER_REQSIZE: usize = 384;

#[repr(C)]
pub struct scatterlist { _private: [u8; 0] }

#[repr(C)]
pub struct skcipher_request {
    pub cryptlen: ::kernel::ffi::c_uint,
    pub iv: *mut u8,
    pub src: *mut scatterlist,
    pub dst: *mut scatterlist,
    pub base: crypto_async_request,
    pub __ctx: [*mut ::kernel::ffi::c_void; 0],
}

#[repr(C)]
pub struct crypto_skcipher {
    pub reqsize: ::kernel::ffi::c_uint,
    pub base: crypto_tfm,
}

#[repr(C)]
pub struct crypto_sync_skcipher { pub base: crypto_skcipher }

#[repr(C)]
pub struct crypto_lskcipher { pub base: crypto_tfm }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skcipher_alg_common {
    pub min_keysize: ::kernel::ffi::c_uint,
    pub max_keysize: ::kernel::ffi::c_uint,
    pub ivsize: ::kernel::ffi::c_uint,
    pub chunksize: ::kernel::ffi::c_uint,
    pub statesize: ::kernel::ffi::c_uint,
    pub base: crypto_alg,
}

#[repr(C)]
pub union skcipher_alg_common_union {
    pub common: skcipher_alg_common,
    pub co: skcipher_alg_common,
}

#[repr(C)]
pub struct skcipher_alg {
    pub setkey: Option<unsafe extern "C" fn(*mut crypto_skcipher, *const u8, ::kernel::ffi::c_uint) -> i32>,
    pub encrypt: Option<unsafe extern "C" fn(*mut skcipher_request) -> i32>,
    pub decrypt: Option<unsafe extern "C" fn(*mut skcipher_request) -> i32>,
    pub export: Option<unsafe extern "C" fn(*mut skcipher_request, *mut ::kernel::ffi::c_void) -> i32>,
    pub import: Option<unsafe extern "C" fn(*mut skcipher_request, *const ::kernel::ffi::c_void) -> i32>,
    pub init: Option<unsafe extern "C" fn(*mut crypto_skcipher) -> i32>,
    pub exit: Option<unsafe extern "C" fn(*mut crypto_skcipher)>,
    pub walksize: ::kernel::ffi::c_uint,
    pub co: skcipher_alg_common_union,
}

#[repr(C)]
pub struct lskcipher_alg {
    pub setkey: Option<unsafe extern "C" fn(*mut crypto_lskcipher, *const u8, ::kernel::ffi::c_uint) -> i32>,
    pub encrypt: Option<unsafe extern "C" fn(*mut crypto_lskcipher, *const u8, *mut u8, ::kernel::ffi::c_uint, *mut u8, u32) -> i32>,
    pub decrypt: Option<unsafe extern "C" fn(*mut crypto_lskcipher, *const u8, *mut u8, ::kernel::ffi::c_uint, *mut u8, u32) -> i32>,
    pub init: Option<unsafe extern "C" fn(*mut crypto_lskcipher) -> i32>,
    pub exit: Option<unsafe extern "C" fn(*mut crypto_lskcipher)>,
    pub co: skcipher_alg_common,
}

extern "C" {
    pub fn crypto_alloc_skcipher(alg_name: *const ::kernel::ffi::c_char, type_: u32, mask: u32) -> *mut crypto_skcipher;
    pub fn crypto_alloc_sync_skcipher(alg_name: *const ::kernel::ffi::c_char, type_: u32, mask: u32) -> *mut crypto_sync_skcipher;
    pub fn crypto_alloc_lskcipher(alg_name: *const ::kernel::ffi::c_char, type_: u32, mask: u32) -> *mut crypto_lskcipher;
    pub fn crypto_has_skcipher(alg_name: *const ::kernel::ffi::c_char, type_: u32, mask: u32) -> i32;
    pub fn crypto_skcipher_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: ::kernel::ffi::c_uint) -> i32;
    pub fn crypto_lskcipher_setkey(tfm: *mut crypto_lskcipher, key: *const u8, keylen: ::kernel::ffi::c_uint) -> i32;
    pub fn crypto_skcipher_encrypt(req: *mut skcipher_request) -> i32;
    pub fn crypto_skcipher_decrypt(req: *mut skcipher_request) -> i32;
    pub fn crypto_skcipher_export(req: *mut skcipher_request, out: *mut ::kernel::ffi::c_void) -> i32;
    pub fn crypto_skcipher_import(req: *mut skcipher_request, input: *const ::kernel::ffi::c_void) -> i32;
    pub fn crypto_lskcipher_encrypt(tfm: *mut crypto_lskcipher, src: *const u8, dst: *mut u8, len: ::kernel::ffi::c_uint, siv: *mut u8) -> i32;
    pub fn crypto_lskcipher_decrypt(tfm: *mut crypto_lskcipher, src: *const u8, dst: *mut u8, len: ::kernel::ffi::c_uint, siv: *mut u8) -> i32;
}

/* The following helpers retain the header's pointer/container semantics. */
#[inline]
pub unsafe fn crypto_skcipher_tfm(tfm: *mut crypto_skcipher) -> *mut crypto_tfm { &mut (*tfm).base }
#[inline]
pub unsafe fn crypto_lskcipher_tfm(tfm: *mut crypto_lskcipher) -> *mut crypto_tfm { &mut (*tfm).base }
#[inline]
pub unsafe fn crypto_sync_skcipher_tfm(tfm: *mut crypto_sync_skcipher) -> *mut crypto_tfm { crypto_skcipher_tfm(&mut (*tfm).base) }

#[inline]
pub unsafe fn crypto_free_skcipher(tfm: *mut crypto_skcipher) { crypto_destroy_tfm(tfm as *mut _, crypto_skcipher_tfm(tfm)); }
#[inline]
pub unsafe fn crypto_free_sync_skcipher(tfm: *mut crypto_sync_skcipher) { crypto_free_skcipher(&mut (*tfm).base); }
#[inline]
pub unsafe fn crypto_free_lskcipher(tfm: *mut crypto_lskcipher) { crypto_destroy_tfm(tfm as *mut _, crypto_lskcipher_tfm(tfm)); }

#[inline]
pub unsafe fn crypto_skcipher_reqsize(tfm: *mut crypto_skcipher) -> usize { (*tfm).reqsize as usize }
#[inline]
pub unsafe fn skcipher_request_set_tfm(req: *mut skcipher_request, tfm: *mut crypto_skcipher) { (*req).base.tfm = crypto_skcipher_tfm(tfm); }
#[inline]
pub unsafe fn skcipher_request_set_sync_tfm(req: *mut skcipher_request, tfm: *mut crypto_sync_skcipher) { skcipher_request_set_tfm(req, &mut (*tfm).base); }
#[inline]
pub unsafe fn skcipher_request_set_callback(req: *mut skcipher_request, flags: u32, compl: crypto_completion_t, data: *mut ::kernel::ffi::c_void) { (*req).base.complete = compl; (*req).base.data = data; (*req).base.flags = flags; }
#[inline]
pub unsafe fn skcipher_request_set_crypt(req: *mut skcipher_request, src: *mut scatterlist, dst: *mut scatterlist, cryptlen: ::kernel::ffi::c_uint, iv: *mut ::kernel::ffi::c_void) { (*req).src = src; (*req).dst = dst; (*req).cryptlen = cryptlen; (*req).iv = iv as *mut u8; }

#[inline]
pub unsafe fn crypto_sync_skcipher_setkey(tfm: *mut crypto_sync_skcipher, key: *const u8, keylen: ::kernel::ffi::c_uint) -> i32 { crypto_skcipher_setkey(&mut (*tfm).base, key, keylen) }

extern "C" {
    pub fn crypto_skcipher_driver_name(tfm: *mut crypto_skcipher) -> *const ::kernel::ffi::c_char;
    pub fn crypto_lskcipher_driver_name(tfm: *mut crypto_lskcipher) -> *const ::kernel::ffi::c_char;
    pub fn crypto_skcipher_ivsize(tfm: *mut crypto_skcipher) -> ::kernel::ffi::c_uint;
    pub fn crypto_sync_skcipher_ivsize(tfm: *mut crypto_sync_skcipher) -> ::kernel::ffi::c_uint;
    pub fn crypto_lskcipher_ivsize(tfm: *mut crypto_lskcipher) -> ::kernel::ffi::c_uint;
    pub fn crypto_skcipher_blocksize(tfm: *mut crypto_skcipher) -> ::kernel::ffi::c_uint;
    pub fn crypto_sync_skcipher_blocksize(tfm: *mut crypto_sync_skcipher) -> ::kernel::ffi::c_uint;
    pub fn crypto_lskcipher_blocksize(tfm: *mut crypto_lskcipher) -> ::kernel::ffi::c_uint;
    pub fn crypto_skcipher_chunksize(tfm: *mut crypto_skcipher) -> ::kernel::ffi::c_uint;
    pub fn crypto_lskcipher_chunksize(tfm: *mut crypto_lskcipher) -> ::kernel::ffi::c_uint;
    pub fn crypto_skcipher_statesize(tfm: *mut crypto_skcipher) -> ::kernel::ffi::c_uint;
    pub fn crypto_lskcipher_statesize(tfm: *mut crypto_lskcipher) -> ::kernel::ffi::c_uint;
    pub fn crypto_skcipher_alignmask(tfm: *mut crypto_skcipher) -> ::kernel::ffi::c_uint;
    pub fn crypto_lskcipher_alignmask(tfm: *mut crypto_lskcipher) -> ::kernel::ffi::c_uint;
    pub fn crypto_skcipher_get_flags(tfm: *mut crypto_skcipher) -> u32;
    pub fn crypto_skcipher_set_flags(tfm: *mut crypto_skcipher, flags: u32);
    pub fn crypto_skcipher_clear_flags(tfm: *mut crypto_skcipher, flags: u32);
    pub fn crypto_lskcipher_get_flags(tfm: *mut crypto_lskcipher) -> u32;
    pub fn crypto_lskcipher_set_flags(tfm: *mut crypto_lskcipher, flags: u32);
    pub fn crypto_lskcipher_clear_flags(tfm: *mut crypto_lskcipher, flags: u32);
    pub fn crypto_skcipher_min_keysize(tfm: *mut crypto_skcipher) -> ::kernel::ffi::c_uint;
    pub fn crypto_skcipher_max_keysize(tfm: *mut crypto_skcipher) -> ::kernel::ffi::c_uint;
    pub fn crypto_lskcipher_min_keysize(tfm: *mut crypto_lskcipher) -> ::kernel::ffi::c_uint;
    pub fn crypto_lskcipher_max_keysize(tfm: *mut crypto_lskcipher) -> ::kernel::ffi::c_uint;
    pub fn crypto_skcipher_reqtfm(req: *mut skcipher_request) -> *mut crypto_skcipher;
    pub fn crypto_sync_skcipher_reqtfm(req: *mut skcipher_request) -> *mut crypto_sync_skcipher;
    pub fn skcipher_request_cast(req: *mut crypto_async_request) -> *mut skcipher_request;
    pub fn skcipher_request_alloc_noprof(tfm: *mut crypto_skcipher, gfp: gfp_t) -> *mut skcipher_request;
    pub fn skcipher_request_free(req: *mut skcipher_request);
    pub fn skcipher_request_zero(req: *mut skcipher_request);
}

/* External types and helpers are supplied by the translated dependency headers. */
extern "C" {
    pub fn crypto_destroy_tfm(obj: *mut ::kernel::ffi::c_void, tfm: *mut crypto_tfm);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
