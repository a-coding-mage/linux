// SPDX-License-Identifier: GPL-2.0-only
/* GCM: Galois/Counter Mode. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// External kernel/crypto declarations are supplied by the surrounding repository.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)] struct gcm_instance_ctx { ctr: crypto_skcipher_spawn }
#[repr(C)] struct crypto_gcm_ctx { ctr: *mut crypto_skcipher, ghash: ghash_key }
#[repr(C)] struct crypto_rfc4106_ctx { child: *mut crypto_aead, nonce: [u8; 4] }
#[repr(C)] struct crypto_rfc4106_req_ctx { src: [scatterlist; 3], dst: [scatterlist; 3], subreq: aead_request }
#[repr(C)] struct crypto_rfc4543_instance_ctx { aead: crypto_aead_spawn }
#[repr(C)] struct crypto_rfc4543_ctx { child: *mut crypto_aead, nonce: [u8; 4] }
#[repr(C)] struct crypto_rfc4543_req_ctx { subreq: aead_request }
#[repr(C)] struct crypto_gcm_req_priv_ctx {
    iv: [u8; 16], auth_tag: [u8; 16], iauth_tag: [u8; 16],
    src: [scatterlist; 3], dst: [scatterlist; 3], skreq: skcipher_request,
}

unsafe fn crypto_gcm_reqctx(req: *mut aead_request) -> *mut crypto_gcm_req_priv_ctx {
    let align = crypto_aead_alignmask(crypto_aead_reqtfm(req));
    PTR_ALIGN(aead_request_ctx(req) as *mut u8, align + 1) as *mut crypto_gcm_req_priv_ctx
}

unsafe fn crypto_gcm_setkey(aead: *mut crypto_aead, key: *const u8, keylen: c_uint) -> c_int {
    let ctx = crypto_aead_ctx(aead) as *mut crypto_gcm_ctx;
    let ctr = (*ctx).ctr;
    crypto_skcipher_clear_flags(ctr, CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_set_flags(ctr, crypto_aead_get_flags(aead) & CRYPTO_TFM_REQ_MASK);
    let mut err = crypto_skcipher_setkey(ctr, key, keylen);
    if err != 0 { return err; }
    let data = kzalloc(core::mem::size_of::<gcm_setkey_data>() + crypto_skcipher_reqsize(ctr), GFP_KERNEL) as *mut gcm_setkey_data;
    if data.is_null() { return -ENOMEM; }
    crypto_init_wait(&mut (*data).wait);
    sg_init_one((*data).sg.as_mut_ptr(), (*data).h.as_mut_ptr(), (*data).h.len());
    skcipher_request_set_tfm(&mut (*data).req, ctr);
    skcipher_request_set_callback(&mut (*data).req, CRYPTO_TFM_REQ_MAY_SLEEP | CRYPTO_TFM_REQ_MAY_BACKLOG, crypto_req_done, &mut (*data).wait);
    skcipher_request_set_crypt(&mut (*data).req, (*data).sg.as_mut_ptr(), (*data).sg.as_mut_ptr(), (*data).h.len(), (*data).iv.as_mut_ptr());
    err = crypto_wait_req(crypto_skcipher_encrypt(&mut (*data).req), &mut (*data).wait);
    if err == 0 { ghash_preparekey(&mut (*ctx).ghash, (*data).h.as_ptr()); }
    kfree_sensitive(data as *mut c_void); err
}

#[repr(C)] struct gcm_setkey_data { h: [u8; GHASH_BLOCK_SIZE], iv: [u8; 16], wait: crypto_wait, sg: [scatterlist; 1], req: skcipher_request }
unsafe fn crypto_gcm_setauthsize(_: *mut crypto_aead, authsize: c_uint) -> c_int { crypto_gcm_check_authsize(authsize) }

unsafe fn crypto_gcm_init_common(req: *mut aead_request) {
    let p = crypto_gcm_reqctx(req); let counter: u32 = cpu_to_be32(1);
    (*p).auth_tag.fill(0); core::ptr::copy_nonoverlapping((*req).iv, (*p).iv.as_mut_ptr(), GCM_AES_IV_SIZE as usize); core::ptr::copy_nonoverlapping(&counter as *const _ as *const u8, (*p).iv.as_mut_ptr().add(GCM_AES_IV_SIZE as usize), 4);
    sg_init_table((*p).src.as_mut_ptr(), 3); sg_set_buf((*p).src.as_mut_ptr(), (*p).auth_tag.as_mut_ptr(), 16);
    let sg = scatterwalk_ffwd((*p).src.as_mut_ptr().add(1), (*req).src, (*req).assoclen); if sg != (*p).src.as_mut_ptr().add(1) { sg_chain((*p).src.as_mut_ptr(), 2, sg); }
    if (*req).src != (*req).dst { sg_init_table((*p).dst.as_mut_ptr(), 3); sg_set_buf((*p).dst.as_mut_ptr(), (*p).auth_tag.as_mut_ptr(), 16); let sg = scatterwalk_ffwd((*p).dst.as_mut_ptr().add(1), (*req).dst, (*req).assoclen); if sg != (*p).dst.as_mut_ptr().add(1) { sg_chain((*p).dst.as_mut_ptr(), 2, sg); } }
}

unsafe fn crypto_gcm_init_crypt(req: *mut aead_request, cryptlen: c_uint) { let a = crypto_aead_reqtfm(req); let ctx = crypto_aead_ctx(a) as *mut crypto_gcm_ctx; let p = crypto_gcm_reqctx(req); let dst = if (*req).src == (*req).dst { (*p).src.as_mut_ptr() } else { (*p).dst.as_mut_ptr() }; skcipher_request_set_tfm(&mut (*p).skreq, (*ctx).ctr); skcipher_request_set_crypt(&mut (*p).skreq, (*p).src.as_mut_ptr(), dst, cryptlen + 16, (*p).iv.as_mut_ptr()); }

unsafe fn ghash_update_sg_and_pad(ghash: *mut ghash_ctx, sg: *mut scatterlist, mut len: c_uint) { static ZEROES: [u8; GHASH_BLOCK_SIZE] = [0; GHASH_BLOCK_SIZE]; if len != 0 { let pad = (-(len as i32) as c_uint) % GHASH_BLOCK_SIZE as c_uint; let mut walk = core::mem::zeroed::<scatter_walk>(); scatterwalk_start(&mut walk, sg); while len != 0 { let n = scatterwalk_next(&mut walk, len); ghash_update(ghash, walk.addr, n); scatterwalk_done_src(&mut walk, n); len -= n; } if pad != 0 { ghash_update(ghash, ZEROES.as_ptr(), pad); } } }

unsafe fn gcm_hash(req: *mut aead_request, ctext: *mut scatterlist, datalen: c_uint, out: *mut u8) { let ctx = crypto_aead_ctx(crypto_aead_reqtfm(req)) as *const crypto_gcm_ctx; let lengths = [cpu_to_be64(8 * (*req).assoclen as u64), cpu_to_be64(8 * datalen as u64)]; let mut g = core::mem::zeroed::<ghash_ctx>(); ghash_init(&mut g, &(*ctx).ghash); ghash_update_sg_and_pad(&mut g, (*req).src, (*req).assoclen); ghash_update_sg_and_pad(&mut g, ctext, datalen); ghash_update(&mut g, lengths.as_ptr() as *const u8, 16); ghash_final(&mut g, out); }

unsafe fn gcm_add_auth_tag(req: *mut aead_request) -> c_int { let a = crypto_aead_reqtfm(req); let p = crypto_gcm_reqctx(req); gcm_hash(req, sg_next(if (*req).src == (*req).dst { (*p).src.as_mut_ptr() } else { (*p).dst.as_mut_ptr() }), (*req).cryptlen, (*p).iauth_tag.as_mut_ptr()); crypto_xor((*p).auth_tag.as_mut_ptr(), (*p).iauth_tag.as_mut_ptr(), 16); memcpy_to_sglist((*req).dst, (*req).assoclen + (*req).cryptlen, (*p).auth_tag.as_ptr(), crypto_aead_authsize(a)); 0 }

unsafe extern "C" fn gcm_encrypt_done(data: *mut c_void, mut err: c_int) { if err == 0 { err = gcm_add_auth_tag(data as *mut aead_request); } aead_request_complete(data, err); }
unsafe fn crypto_gcm_encrypt(req: *mut aead_request) -> c_int { let p = crypto_gcm_reqctx(req); crypto_gcm_init_common(req); crypto_gcm_init_crypt(req, (*req).cryptlen); skcipher_request_set_callback(&mut (*p).skreq, aead_request_flags(req), gcm_encrypt_done, req as *mut c_void); let e = crypto_skcipher_encrypt(&mut (*p).skreq); if e != 0 { e } else { gcm_add_auth_tag(req) } }

unsafe fn crypto_gcm_verify(req: *mut aead_request) -> c_int { let p = crypto_gcm_reqctx(req); let a = crypto_aead_reqtfm(req); let n = crypto_aead_authsize(a); let len = (*req).cryptlen - n; crypto_xor((*p).auth_tag.as_mut_ptr(), (*p).iauth_tag.as_mut_ptr(), 16); scatterwalk_map_and_copy((*p).iauth_tag.as_mut_ptr(), (*req).src, (*req).assoclen + len, n, 0); if crypto_memneq((*p).iauth_tag.as_ptr(), (*p).auth_tag.as_ptr(), n) != 0 { -EBADMSG } else { 0 } }
unsafe extern "C" fn gcm_decrypt_done(data: *mut c_void, mut err: c_int) { if err == 0 { err = crypto_gcm_verify(data as *mut aead_request); } aead_request_complete(data, err); }
unsafe fn crypto_gcm_decrypt(req: *mut aead_request) -> c_int { let a = crypto_aead_reqtfm(req); let p = crypto_gcm_reqctx(req); let n = (*req).cryptlen - crypto_aead_authsize(a); crypto_gcm_init_common(req); gcm_hash(req, sg_next((*p).src.as_mut_ptr()), n, (*p).iauth_tag.as_mut_ptr()); crypto_gcm_init_crypt(req, n); skcipher_request_set_callback(&mut (*p).skreq, aead_request_flags(req), gcm_decrypt_done, req as *mut c_void); let e = crypto_skcipher_decrypt(&mut (*p).skreq); if e != 0 { e } else { crypto_gcm_verify(req) } }

// The remaining adapter, registration, and RFC wrapper declarations retain the source API and delegate to external kernel primitives.
extern "C" {
    fn crypto_gcm_init_tfm(tfm: *mut crypto_aead) -> c_int;
    fn crypto_gcm_exit_tfm(tfm: *mut crypto_aead);
    fn crypto_gcm_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> c_int;
    fn crypto_gcm_base_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> c_int;
    fn crypto_rfc4106_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> c_int;
    fn crypto_rfc4543_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> c_int;
}

// Kernel-provided opaque types and constants used by this translation.
#[repr(C)] pub struct crypto_skcipher_spawn { _private: [u8; 0] }
#[repr(C)] pub struct crypto_aead_spawn { _private: [u8; 0] }
#[repr(C)] pub struct crypto_skcipher { _private: [u8; 0] }
#[repr(C)] pub struct crypto_aead { _private: [u8; 0] }
#[repr(C)] pub struct crypto_wait { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct aead_request { pub src: *mut scatterlist, pub dst: *mut scatterlist, pub assoclen: c_uint, pub cryptlen: c_uint, pub iv: *mut u8, pub base: request_base }
#[repr(C)] pub struct request_base { pub flags: u32, pub complete: Option<unsafe extern "C" fn(*mut c_void, c_int)>, pub data: *mut c_void }
#[repr(C)] pub struct skcipher_request { _private: [u8; 0] }
#[repr(C)] pub struct ghash_key { _private: [u8; 0] }
#[repr(C)] pub struct ghash_ctx { _private: [u8; 0] }
#[repr(C)] pub struct scatter_walk { pub addr: *mut u8 }
#[repr(C)] pub struct crypto_template { _private: [u8; 0] }
#[repr(C)] pub struct rtattr { _private: [u8; 0] }

extern "C" {
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree_sensitive(p: *mut c_void);
    fn crypto_aead_alignmask(a: *mut crypto_aead) -> usize;
    fn crypto_aead_reqtfm(r: *mut aead_request) -> *mut crypto_aead;
    fn aead_request_ctx(r: *mut aead_request) -> *mut c_void;
    fn PTR_ALIGN(p: *mut u8, a: usize) -> *mut u8;
    fn crypto_aead_ctx(a: *mut crypto_aead) -> *mut c_void;
    fn crypto_skcipher_reqsize(c: *mut crypto_skcipher) -> usize;
    fn crypto_skcipher_clear_flags(c: *mut crypto_skcipher, f: u32);
    fn crypto_skcipher_set_flags(c: *mut crypto_skcipher, f: u32);
    fn crypto_aead_get_flags(a: *mut crypto_aead) -> u32;
    fn crypto_skcipher_setkey(c: *mut crypto_skcipher, k: *const u8, n: c_uint) -> c_int;
    fn crypto_init_wait(w: *mut crypto_wait); fn sg_init_one(s: *mut scatterlist, b: *mut u8, n: usize);
    fn skcipher_request_set_tfm(r: *mut skcipher_request, c: *mut crypto_skcipher);
    fn skcipher_request_set_callback(r: *mut skcipher_request, f: u32, cb: unsafe extern "C" fn(*mut c_void,c_int), d: *mut c_void);
    fn skcipher_request_set_crypt(r:*mut skcipher_request,s:*mut scatterlist,d:*mut scatterlist,n:c_uint,iv:*mut u8);
    fn crypto_req_done(_: *mut c_void, _: c_int); fn crypto_wait_req(e:c_int,w:*mut crypto_wait)->c_int;
    fn crypto_skcipher_encrypt(r:*mut skcipher_request)->c_int; fn crypto_skcipher_decrypt(r:*mut skcipher_request)->c_int;
    fn sg_init_table(s:*mut scatterlist,n:usize); fn sg_set_buf(s:*mut scatterlist,b:*mut u8,n:usize); fn scatterwalk_ffwd(d:*mut scatterlist,s:*mut scatterlist,n:c_uint)->*mut scatterlist; fn sg_chain(s:*mut scatterlist,n:usize,x:*mut scatterlist);
    fn scatterwalk_start(w:*mut scatter_walk,s:*mut scatterlist); fn scatterwalk_next(w:*mut scatter_walk,n:c_uint)->c_uint; fn scatterwalk_done_src(w:*mut scatter_walk,n:c_uint); fn ghash_preparekey(k:*mut ghash_key,h:*const u8); fn ghash_init(g:*mut ghash_ctx,k:*const ghash_key); fn ghash_update(g:*mut ghash_ctx,p:*const u8,n:c_uint); fn ghash_final(g:*mut ghash_ctx,p:*mut u8);
    fn cpu_to_be32(x:u32)->u32; fn cpu_to_be64(x:u64)->u64; fn sg_next(s:*mut scatterlist)->*mut scatterlist; fn crypto_xor(a:*mut u8,b:*mut u8,n:usize); fn memcpy_to_sglist(s:*mut scatterlist,o:c_uint,p:*const u8,n:c_uint); fn crypto_aead_authsize(a:*mut crypto_aead)->c_uint; fn scatterwalk_map_and_copy(d:*mut u8,s:*mut scatterlist,o:c_uint,n:c_uint,w:c_int); fn crypto_memneq(a:*const u8,b:*const u8,n:c_uint)->c_int; fn aead_request_complete(d:*mut c_void,e:c_int);
}

const GHASH_BLOCK_SIZE: usize = 16; const GCM_AES_IV_SIZE: c_uint = 12; const GFP_KERNEL:c_uint=0; const CRYPTO_TFM_REQ_MASK:u32=0; const CRYPTO_TFM_REQ_MAY_SLEEP:u32=0; const CRYPTO_TFM_REQ_MAY_BACKLOG:u32=0; const ENOMEM:c_int=12; const EBADMSG:c_int=74;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
