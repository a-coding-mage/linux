/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Public Key Encryption
 *
 * Copyright (c) 2015, Intel Corporation
 * Authors: Tadeusz Struk <tadeusz.struk@intel.com>
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct akcipher_request {
    pub base: crypto_async_request,
    pub src: *mut scatterlist,
    pub dst: *mut scatterlist,
    pub src_len: ::core::ffi::c_uint,
    pub dst_len: ::core::ffi::c_uint,
    pub __ctx: [::core::ffi::c_void; 0],
}

#[repr(C)]
pub struct crypto_akcipher {
    pub reqsize: ::core::ffi::c_uint,
    pub base: crypto_tfm,
}

#[repr(C)]
pub struct akcipher_alg {
    pub encrypt: Option<unsafe extern "C" fn(*mut akcipher_request) -> ::core::ffi::c_int>,
    pub decrypt: Option<unsafe extern "C" fn(*mut akcipher_request) -> ::core::ffi::c_int>,
    pub set_pub_key: Option<unsafe extern "C" fn(
        *mut crypto_akcipher,
        *const ::core::ffi::c_void,
        ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int>,
    pub set_priv_key: Option<unsafe extern "C" fn(
        *mut crypto_akcipher,
        *const ::core::ffi::c_void,
        ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int>,
    pub max_size: Option<unsafe extern "C" fn(*mut crypto_akcipher) -> ::core::ffi::c_uint>,
    pub init: Option<unsafe extern "C" fn(*mut crypto_akcipher) -> ::core::ffi::c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut crypto_akcipher)>,
    pub base: crypto_alg,
}

unsafe extern "C" {
    pub fn crypto_alloc_akcipher(
        alg_name: *const ::core::ffi::c_char,
        type_: u32,
        mask: u32,
    ) -> *mut crypto_akcipher;
    pub fn crypto_akcipher_sync_encrypt(
        tfm: *mut crypto_akcipher,
        src: *const ::core::ffi::c_void,
        slen: ::core::ffi::c_uint,
        dst: *mut ::core::ffi::c_void,
        dlen: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn crypto_akcipher_sync_decrypt(
        tfm: *mut crypto_akcipher,
        src: *const ::core::ffi::c_void,
        slen: ::core::ffi::c_uint,
        dst: *mut ::core::ffi::c_void,
        dlen: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn crypto_akcipher_tfm(tfm: *mut crypto_akcipher) -> *mut crypto_tfm {
    unsafe { &mut (*tfm).base }
}

#[inline]
pub unsafe fn __crypto_akcipher_alg(alg: *mut crypto_alg) -> *mut akcipher_alg {
    unsafe { container_of(alg, akcipher_alg, base) }
}

#[inline]
pub unsafe fn __crypto_akcipher_tfm(tfm: *mut crypto_tfm) -> *mut crypto_akcipher {
    unsafe { container_of(tfm, crypto_akcipher, base) }
}

#[inline]
pub unsafe fn crypto_akcipher_alg(tfm: *mut crypto_akcipher) -> *mut akcipher_alg {
    unsafe { __crypto_akcipher_alg((*crypto_akcipher_tfm(tfm)).__crt_alg) }
}

#[inline]
pub unsafe fn crypto_akcipher_reqsize(tfm: *mut crypto_akcipher) -> ::core::ffi::c_uint {
    unsafe { (*tfm).reqsize }
}

#[inline]
pub unsafe fn akcipher_request_set_tfm(req: *mut akcipher_request, tfm: *mut crypto_akcipher) {
    unsafe { (*req).base.tfm = crypto_akcipher_tfm(tfm); }
}

#[inline]
pub unsafe fn crypto_akcipher_reqtfm(req: *mut akcipher_request) -> *mut crypto_akcipher {
    unsafe { __crypto_akcipher_tfm((*req).base.tfm) }
}

#[inline]
pub unsafe fn crypto_free_akcipher(tfm: *mut crypto_akcipher) {
    unsafe { crypto_destroy_tfm(tfm, crypto_akcipher_tfm(tfm)); }
}

#[inline]
pub unsafe fn akcipher_request_alloc(
    tfm: *mut crypto_akcipher,
    gfp: gfp_t,
) -> *mut akcipher_request {
    let req = unsafe { kmalloc(core::mem::size_of::<akcipher_request>() + crypto_akcipher_reqsize(tfm) as usize, gfp) as *mut akcipher_request };
    if unsafe { likely(!req.is_null()) } {
        unsafe { akcipher_request_set_tfm(req, tfm); }
    }
    req
}

#[inline]
pub unsafe fn akcipher_request_free(req: *mut akcipher_request) {
    unsafe { kfree_sensitive(req as *mut ::core::ffi::c_void); }
}

#[inline]
pub unsafe fn akcipher_request_set_callback(
    req: *mut akcipher_request,
    flgs: u32,
    cmpl: crypto_completion_t,
    data: *mut ::core::ffi::c_void,
) {
    unsafe {
        (*req).base.complete = cmpl;
        (*req).base.data = data;
        (*req).base.flags = flgs;
    }
}

#[inline]
pub unsafe fn akcipher_request_set_crypt(
    req: *mut akcipher_request,
    src: *mut scatterlist,
    dst: *mut scatterlist,
    src_len: ::core::ffi::c_uint,
    dst_len: ::core::ffi::c_uint,
) {
    unsafe {
        (*req).src = src;
        (*req).dst = dst;
        (*req).src_len = src_len;
        (*req).dst_len = dst_len;
    }
}

#[inline]
pub unsafe fn crypto_akcipher_maxsize(tfm: *mut crypto_akcipher) -> ::core::ffi::c_uint {
    unsafe { ((*crypto_akcipher_alg(tfm)).max_size.unwrap())(tfm) }
}

#[inline]
pub unsafe fn crypto_akcipher_encrypt(req: *mut akcipher_request) -> ::core::ffi::c_int {
    unsafe { ((*crypto_akcipher_alg(crypto_akcipher_reqtfm(req))).encrypt.unwrap())(req) }
}

#[inline]
pub unsafe fn crypto_akcipher_decrypt(req: *mut akcipher_request) -> ::core::ffi::c_int {
    unsafe { ((*crypto_akcipher_alg(crypto_akcipher_reqtfm(req))).decrypt.unwrap())(req) }
}

#[inline]
pub unsafe fn crypto_akcipher_set_pub_key(
    tfm: *mut crypto_akcipher,
    key: *const ::core::ffi::c_void,
    keylen: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    unsafe { ((*crypto_akcipher_alg(tfm)).set_pub_key.unwrap())(tfm, key, keylen) }
}

#[inline]
pub unsafe fn crypto_akcipher_set_priv_key(
    tfm: *mut crypto_akcipher,
    key: *const ::core::ffi::c_void,
    keylen: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    unsafe { ((*crypto_akcipher_alg(tfm)).set_priv_key.unwrap())(tfm, key, keylen) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
