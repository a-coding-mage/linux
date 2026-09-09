/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Symmetric key ciphers.
 *
 * Copyright (c) 2007 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Dependencies supplied by the surrounding translation unit.

pub const CRYPTO_ALG_SKCIPHER_REQSIZE_LARGE: u32 = CRYPTO_ALG_OPTIONAL_KEY;

#[repr(C)]
pub struct aead_request;
#[repr(C)]
pub struct rtattr;

#[repr(C)]
pub struct skcipher_instance {
    pub free: Option<unsafe extern "C" fn(inst: *mut skcipher_instance)>,
    pub union_: skcipher_instance_union,
}

#[repr(C)]
pub union skcipher_instance_union {
    pub s: skcipher_instance_s,
    pub alg: skcipher_alg,
}

#[repr(C)]
pub struct skcipher_instance_s {
    pub head: [std::ffi::c_char; 0],
    pub base: crypto_instance,
}

#[repr(C)]
pub struct lskcipher_instance {
    pub free: Option<unsafe extern "C" fn(inst: *mut lskcipher_instance)>,
    pub union_: lskcipher_instance_union,
}

#[repr(C)]
pub union lskcipher_instance_union {
    pub s: lskcipher_instance_s,
    pub alg: lskcipher_alg,
}

#[repr(C)]
pub struct lskcipher_instance_s {
    pub head: [std::ffi::c_char; 0],
    pub base: crypto_instance,
}

#[repr(C)]
pub struct crypto_skcipher_spawn { pub base: crypto_spawn }
#[repr(C)]
pub struct crypto_lskcipher_spawn { pub base: crypto_spawn }

#[repr(C)]
pub struct skcipher_walk {
    pub src: skcipher_walk_src,
    pub dst: skcipher_walk_dst,
    pub nbytes: u32,
    pub total: u32,
    pub page: *mut u8,
    pub buffer: *mut u8,
    pub oiv: *mut u8,
    pub iv: *mut std::ffi::c_void,
    pub ivsize: u32,
    pub flags: std::ffi::c_int,
    pub blocksize: u32,
    pub stride: u32,
    pub alignmask: u32,
}

#[repr(C)]
pub union skcipher_walk_src { pub virt: skcipher_walk_src_virt, pub in_: scatter_walk }
#[repr(C)]
pub struct skcipher_walk_src_virt { pub addr: *const std::ffi::c_void }
#[repr(C)]
pub union skcipher_walk_dst { pub virt: skcipher_walk_dst_virt, pub out: scatter_walk }
#[repr(C)]
pub struct skcipher_walk_dst_virt { pub addr: *mut std::ffi::c_void }

#[inline]
pub unsafe fn skcipher_crypto_instance(inst: *mut skcipher_instance) -> *mut crypto_instance { &mut (*inst).union_.s.base }
#[inline]
pub unsafe fn lskcipher_crypto_instance(inst: *mut lskcipher_instance) -> *mut crypto_instance { &mut (*inst).union_.s.base }
#[inline]
pub unsafe fn skcipher_alg_instance(skcipher: *mut crypto_skcipher) -> *mut skcipher_instance { container_of!(crypto_skcipher_alg(skcipher), skcipher_instance, alg) }
#[inline]
pub unsafe fn lskcipher_alg_instance(lskcipher: *mut crypto_lskcipher) -> *mut lskcipher_instance { container_of!(crypto_lskcipher_alg(lskcipher), lskcipher_instance, alg) }
#[inline]
pub unsafe fn skcipher_instance_ctx(inst: *mut skcipher_instance) -> *mut std::ffi::c_void { crypto_instance_ctx(skcipher_crypto_instance(inst)) }
#[inline]
pub unsafe fn lskcipher_instance_ctx(inst: *mut lskcipher_instance) -> *mut std::ffi::c_void { crypto_instance_ctx(lskcipher_crypto_instance(inst)) }
#[inline]
pub unsafe fn skcipher_request_complete(req: *mut skcipher_request, err: std::ffi::c_int) { crypto_request_complete(&mut (*req).base, err); }

extern "C" {
    pub fn crypto_grab_skcipher(spawn: *mut crypto_skcipher_spawn, inst: *mut crypto_instance, name: *const std::ffi::c_char, type_: u32, mask: u32) -> std::ffi::c_int;
    pub fn crypto_grab_lskcipher(spawn: *mut crypto_lskcipher_spawn, inst: *mut crypto_instance, name: *const std::ffi::c_char, type_: u32, mask: u32) -> std::ffi::c_int;
    pub fn crypto_register_skcipher(alg: *mut skcipher_alg) -> std::ffi::c_int;
    pub fn crypto_unregister_skcipher(alg: *mut skcipher_alg);
    pub fn crypto_register_skciphers(algs: *mut skcipher_alg, count: std::ffi::c_int) -> std::ffi::c_int;
    pub fn crypto_unregister_skciphers(algs: *mut skcipher_alg, count: std::ffi::c_int);
    pub fn skcipher_register_instance(tmpl: *mut crypto_template, inst: *mut skcipher_instance) -> std::ffi::c_int;
    pub fn crypto_register_lskcipher(alg: *mut lskcipher_alg) -> std::ffi::c_int;
    pub fn crypto_unregister_lskcipher(alg: *mut lskcipher_alg);
    pub fn crypto_register_lskciphers(algs: *mut lskcipher_alg, count: std::ffi::c_int) -> std::ffi::c_int;
    pub fn crypto_unregister_lskciphers(algs: *mut lskcipher_alg, count: std::ffi::c_int);
    pub fn lskcipher_register_instance(tmpl: *mut crypto_template, inst: *mut lskcipher_instance) -> std::ffi::c_int;
    pub fn skcipher_walk_done(walk: *mut skcipher_walk, res: std::ffi::c_int) -> std::ffi::c_int;
    pub fn skcipher_walk_virt(walk: *mut skcipher_walk, req: *mut skcipher_request, atomic: bool) -> std::ffi::c_int;
    pub fn skcipher_walk_aead_encrypt(walk: *mut skcipher_walk, req: *mut aead_request, atomic: bool) -> std::ffi::c_int;
    pub fn skcipher_walk_aead_decrypt(walk: *mut skcipher_walk, req: *mut aead_request, atomic: bool) -> std::ffi::c_int;
    pub fn skcipher_alloc_instance_simple(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> *mut skcipher_instance;
    pub fn lskcipher_alloc_instance_simple(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> *mut lskcipher_instance;
}

#[inline] pub unsafe fn crypto_drop_skcipher(s: *mut crypto_skcipher_spawn) { crypto_drop_spawn(&mut (*s).base); }
#[inline] pub unsafe fn crypto_drop_lskcipher(s: *mut crypto_lskcipher_spawn) { crypto_drop_spawn(&mut (*s).base); }
#[inline] pub unsafe fn crypto_lskcipher_spawn_alg(s: *mut crypto_lskcipher_spawn) -> *mut lskcipher_alg { container_of!((*s).base.alg, lskcipher_alg, co.base) }
#[inline] pub unsafe fn crypto_spawn_skcipher_alg_common(s: *mut crypto_skcipher_spawn) -> *mut skcipher_alg_common { container_of!((*s).base.alg, skcipher_alg_common, base) }
#[inline] pub unsafe fn crypto_spawn_lskcipher_alg(s: *mut crypto_lskcipher_spawn) -> *mut lskcipher_alg { crypto_lskcipher_spawn_alg(s) }
#[inline] pub unsafe fn crypto_spawn_skcipher(s: *mut crypto_skcipher_spawn) -> *mut crypto_skcipher { crypto_spawn_tfm2(&mut (*s).base) }
#[inline] pub unsafe fn crypto_spawn_lskcipher(s: *mut crypto_lskcipher_spawn) -> *mut crypto_lskcipher { crypto_spawn_tfm2(&mut (*s).base) }
#[inline] pub unsafe fn crypto_skcipher_set_reqsize(s: *mut crypto_skcipher, reqsize: u32) { (*s).reqsize = reqsize; }
#[inline] pub unsafe fn crypto_skcipher_set_reqsize_dma(s: *mut crypto_skcipher, mut reqsize: u32) { reqsize += crypto_dma_align() & !(crypto_tfm_ctx_alignment() - 1); (*s).reqsize = reqsize; }
#[inline] pub unsafe fn skcipher_walk_abort(walk: *mut skcipher_walk) { skcipher_walk_done(walk, -ECANCELED); }
#[inline] pub unsafe fn crypto_skcipher_ctx(tfm: *mut crypto_skcipher) -> *mut std::ffi::c_void { crypto_tfm_ctx(&mut (*tfm).base) }
#[inline] pub unsafe fn crypto_lskcipher_ctx(tfm: *mut crypto_lskcipher) -> *mut std::ffi::c_void { crypto_tfm_ctx(&mut (*tfm).base) }
#[inline] pub unsafe fn crypto_skcipher_ctx_dma(tfm: *mut crypto_skcipher) -> *mut std::ffi::c_void { crypto_tfm_ctx_dma(&mut (*tfm).base) }
#[inline] pub unsafe fn crypto_skcipher_tested(tfm: *mut crypto_skcipher) -> bool { let tfm_base = crypto_skcipher_tfm(tfm); (*tfm_base).__crt_alg.as_ref().unwrap().cra_flags & CRYPTO_ALG_TESTED != 0 }
#[inline] pub unsafe fn skcipher_request_ctx(req: *mut skcipher_request) -> *mut std::ffi::c_void { (*req).__ctx }
#[inline] pub unsafe fn skcipher_request_ctx_dma(req: *mut skcipher_request) -> *mut std::ffi::c_void { let mut align = crypto_dma_align(); if align <= crypto_tfm_ctx_alignment() { align = 1; } PTR_ALIGN(skcipher_request_ctx(req), align) }
#[inline] pub unsafe fn skcipher_request_flags(req: *mut skcipher_request) -> u32 { (*req).base.flags }

#[repr(C)] pub struct skcipher_ctx_simple { pub cipher: *mut crypto_cipher }
#[inline] pub unsafe fn skcipher_cipher_simple(tfm: *mut crypto_skcipher) -> *mut crypto_cipher { (*(crypto_skcipher_ctx(tfm) as *mut skcipher_ctx_simple)).cipher }
#[inline] pub unsafe fn skcipher_ialg_simple(inst: *mut skcipher_instance) -> *mut crypto_alg { let spawn = skcipher_instance_ctx(inst) as *mut crypto_cipher_spawn; crypto_spawn_cipher_alg(spawn) }
#[inline] pub unsafe fn lskcipher_cipher_simple(tfm: *mut crypto_lskcipher) -> *mut crypto_lskcipher { *(crypto_lskcipher_ctx(tfm) as *mut *mut crypto_lskcipher) }
#[inline] pub unsafe fn lskcipher_ialg_simple(inst: *mut lskcipher_instance) -> *mut lskcipher_alg { let spawn = lskcipher_instance_ctx(inst) as *mut crypto_lskcipher_spawn; crypto_lskcipher_spawn_alg(spawn) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
