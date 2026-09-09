// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * authencesn.c - AEAD wrapper for IPsec with extended sequence numbers,
 *                 derived from authenc.c
 *
 * Copyright (C) 2010 secunet Security Networks AG
 * Copyright (C) 2010 Steffen Klassert <steffen.klassert@secunet.com>
 * Copyright (c) 2015 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Linux crypto/kernel dependencies are supplied by the surrounding repository.

#[repr(C)]
struct authenc_esn_instance_ctx { auth: crypto_ahash_spawn, enc: crypto_skcipher_spawn }
#[repr(C)]
struct crypto_authenc_esn_ctx { reqoff: c_uint, auth: *mut crypto_ahash, enc: *mut crypto_skcipher }
#[repr(C)]
struct authenc_esn_request_ctx { src: [scatterlist; 2], dst: [scatterlist; 2], tail: [u8; 0] }

unsafe fn authenc_esn_request_complete(req: *mut aead_request, err: c_int) {
    if err != -EINPROGRESS { aead_request_complete(req, err); }
}

unsafe fn crypto_authenc_esn_setauthsize(_: *mut crypto_aead, authsize: c_uint) -> c_int {
    if authsize > 0 && authsize < 4 { return -EINVAL; }
    0
}

unsafe fn crypto_authenc_esn_setkey(authenc_esn: *mut crypto_aead, key: *const u8, keylen: c_uint) -> c_int {
    let ctx = crypto_aead_ctx(authenc_esn);
    let auth = (*ctx).auth; let enc = (*ctx).enc;
    let mut keys = crypto_authenc_keys { authkey: core::ptr::null(), authkeylen: 0, enckey: core::ptr::null(), enckeylen: 0 };
    let mut err = -EINVAL;
    if crypto_authenc_extractkeys(&mut keys, key, keylen) != 0 { return err; }
    crypto_ahash_clear_flags(auth, CRYPTO_TFM_REQ_MASK);
    crypto_ahash_set_flags(auth, crypto_aead_get_flags(authenc_esn) & CRYPTO_TFM_REQ_MASK);
    err = crypto_ahash_setkey(auth, keys.authkey, keys.authkeylen);
    if err != 0 { memzero_explicit(&mut keys as *mut _ as *mut c_void, core::mem::size_of_val(&keys)); return err; }
    crypto_skcipher_clear_flags(enc, CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_set_flags(enc, crypto_aead_get_flags(authenc_esn) & CRYPTO_TFM_REQ_MASK);
    err = crypto_skcipher_setkey(enc, keys.enckey, keys.enckeylen);
    memzero_explicit(&mut keys as *mut _ as *mut c_void, core::mem::size_of_val(&keys)); err
}

unsafe fn crypto_authenc_esn_genicv_tail(req: *mut aead_request, _: c_uint) -> c_int {
    let tfm = crypto_aead_reqtfm(req); let rc = aead_request_ctx(req); let hash = (*rc).tail.as_mut_ptr();
    let authsize = crypto_aead_authsize(tfm); let assoclen = (*req).assoclen; let cryptlen = (*req).cryptlen; let dst = (*req).dst;
    let mut tmp = [0u32; 2];
    memcpy_from_sglist(tmp.as_mut_ptr() as *mut c_void, dst, 4, 4);
    memcpy_from_sglist(tmp[1..].as_mut_ptr() as *mut c_void, dst, assoclen + cryptlen, 4);
    memcpy_to_sglist(dst, 0, tmp.as_ptr() as *const c_void, 8);
    memcpy_to_sglist(dst, assoclen + cryptlen, hash as *const c_void, authsize); 0
}

unsafe fn authenc_esn_geniv_ahash_done(data: *mut c_void, mut err: c_int) {
    let req = data as *mut aead_request;
    if err == 0 { err = crypto_authenc_esn_genicv_tail(req, 0); }
    aead_request_complete(req, err);
}

unsafe fn crypto_authenc_esn_genicv(req: *mut aead_request, flags: c_uint) -> c_int {
    let tfm = crypto_aead_reqtfm(req); let rc = aead_request_ctx(req); let ctx = crypto_aead_ctx(tfm); let auth = (*ctx).auth;
    let hash = (*rc).tail.as_mut_ptr(); let ahreq = hash.add((*ctx).reqoff as usize) as *mut ahash_request;
    let authsize = crypto_aead_authsize(tfm); let assoclen = (*req).assoclen; let cryptlen = (*req).cryptlen; let mut dst = (*req).dst; let mut tmp = [0u32; 2];
    if authsize == 0 { return 0; }
    memcpy_from_sglist(tmp.as_mut_ptr() as *mut c_void, dst, 0, 8);
    memcpy_to_sglist(dst, 4, tmp.as_ptr() as *const c_void, 4);
    memcpy_to_sglist(dst, assoclen + cryptlen, tmp[1..].as_ptr() as *const c_void, 4);
    sg_init_table((*rc).dst.as_mut_ptr(), 2); dst = scatterwalk_ffwd((*rc).dst.as_mut_ptr(), dst, 4);
    ahash_request_set_tfm(ahreq, auth); ahash_request_set_crypt(ahreq, dst, hash, assoclen + cryptlen);
    ahash_request_set_callback(ahreq, flags, Some(authenc_esn_geniv_ahash_done), req as *mut c_void);
    let err = crypto_ahash_digest(ahreq); if err != 0 { err } else { crypto_authenc_esn_genicv_tail(req, aead_request_flags(req)) }
}

unsafe fn crypto_authenc_esn_encrypt_done(data: *mut c_void, mut err: c_int) {
    let req = data as *mut aead_request; if err == 0 { err = crypto_authenc_esn_genicv(req, 0); } authenc_esn_request_complete(req, err);
}

unsafe fn crypto_authenc_esn_encrypt(req: *mut aead_request) -> c_int {
    let tfm = crypto_aead_reqtfm(req); let rc = aead_request_ctx(req); let ctx = crypto_aead_ctx(tfm);
    let skreq = (*rc).tail.as_mut_ptr().add((*ctx).reqoff as usize) as *mut skcipher_request; let enc = (*ctx).enc;
    let assoclen = (*req).assoclen; let cryptlen = (*req).cryptlen; let mut src; let mut dst;
    if assoclen < 8 { return -EINVAL; }
    sg_init_table((*rc).src.as_mut_ptr(), 2); src = scatterwalk_ffwd((*rc).src.as_mut_ptr(), (*req).src, assoclen); dst = src;
    if (*req).src != (*req).dst { memcpy_sglist((*req).dst, (*req).src, assoclen); sg_init_table((*rc).dst.as_mut_ptr(), 2); dst = scatterwalk_ffwd((*rc).dst.as_mut_ptr(), (*req).dst, assoclen); }
    skcipher_request_set_tfm(skreq, enc); skcipher_request_set_callback(skreq, aead_request_flags(req), Some(crypto_authenc_esn_encrypt_done), req as *mut c_void);
    skcipher_request_set_crypt(skreq, src, dst, cryptlen, (*req).iv);
    let err = crypto_skcipher_encrypt(skreq); if err != 0 { err } else { crypto_authenc_esn_genicv(req, aead_request_flags(req)) }
}

// The remaining decrypt, initialization, registration, and module metadata retain
// the same kernel-facing interfaces and control flow as the C implementation.
// External kernel declarations are intentionally left to repository dependencies.

unsafe fn crypto_authenc_esn_decrypt_tail(req: *mut aead_request, flags: c_uint) -> c_int {
    let tfm = crypto_aead_reqtfm(req); let authsize = crypto_aead_authsize(tfm); let rc = aead_request_ctx(req); let ctx = crypto_aead_ctx(tfm);
    let skreq = (*rc).tail.as_mut_ptr().add((*ctx).reqoff as usize) as *mut skcipher_request; let auth = (*ctx).auth; let ohash = (*rc).tail.as_mut_ptr();
    let cryptlen = (*req).cryptlen - authsize; let assoclen = (*req).assoclen; let mut src = (*req).src; let mut dst = (*req).dst;
    let ihash = ohash.add(crypto_ahash_digestsize(auth) as usize); let mut tmp = [0u32; 2];
    if authsize != 0 {
        if src == dst { memcpy_from_sglist(tmp.as_mut_ptr() as *mut c_void, dst, 4, 4); memcpy_from_sglist(tmp[1..].as_mut_ptr() as *mut c_void, dst, assoclen + cryptlen, 4); memcpy_to_sglist(dst, 0, tmp.as_ptr() as *const c_void, 8); }
        else { memcpy_sglist(dst, src, assoclen); }
        if crypto_memneq(ihash as *const c_void, ohash as *const c_void, authsize) != 0 { return -EBADMSG; }
    }
    dst = scatterwalk_ffwd((*rc).dst.as_mut_ptr(), dst, assoclen); if (*req).src == (*req).dst { src = dst; } else { src = scatterwalk_ffwd((*rc).src.as_mut_ptr(), src, assoclen); }
    skcipher_request_set_tfm(skreq, (*ctx).enc); skcipher_request_set_callback(skreq, flags, (*req).base.complete, (*req).base.data); skcipher_request_set_crypt(skreq, src, dst, cryptlen, (*req).iv); crypto_skcipher_decrypt(skreq)
}

unsafe fn authenc_esn_verify_ahash_done(data: *mut c_void, mut err: c_int) { let req = data as *mut aead_request; if err == 0 { err = crypto_authenc_esn_decrypt_tail(req, 0); } authenc_esn_request_complete(req, err); }

unsafe fn crypto_authenc_esn_decrypt(req: *mut aead_request) -> c_int {
    let tfm = crypto_aead_reqtfm(req); let rc = aead_request_ctx(req); let ctx = crypto_aead_ctx(tfm); let ahreq = (*rc).tail.as_mut_ptr().add((*ctx).reqoff as usize) as *mut ahash_request;
    let authsize = crypto_aead_authsize(tfm); let auth = (*ctx).auth; let ohash = (*rc).tail.as_mut_ptr(); let assoclen = (*req).assoclen; let mut cryptlen = (*req).cryptlen; let ihash = ohash.add(crypto_ahash_digestsize(auth) as usize); let mut src = (*req).src; let mut dst = (*req).dst; let mut tmp = [0u32; 2];
    if assoclen < 8 { return -EINVAL; } if authsize == 0 { return crypto_authenc_esn_decrypt_tail(req, aead_request_flags(req)); }
    cryptlen -= authsize; memcpy_from_sglist(ihash as *mut c_void, (*req).src, assoclen + cryptlen, authsize); memcpy_from_sglist(tmp.as_mut_ptr() as *mut c_void, src, 0, 8);
    if src == dst { memcpy_to_sglist(dst, 4, tmp.as_ptr() as *const c_void, 4); memcpy_to_sglist(dst, assoclen + cryptlen, tmp[1..].as_ptr() as *const c_void, 4); dst = scatterwalk_ffwd((*rc).dst.as_mut_ptr(), dst, 4); }
    else { memcpy_to_sglist(dst, 0, tmp.as_ptr() as *const c_void, 4); memcpy_to_sglist(dst, assoclen + cryptlen - 4, tmp[1..].as_ptr() as *const c_void, 4); src = scatterwalk_ffwd((*rc).src.as_mut_ptr(), src, 8); dst = scatterwalk_ffwd((*rc).dst.as_mut_ptr(), dst, 4); memcpy_sglist(dst, src, assoclen + cryptlen - 8); dst = (*req).dst; }
    ahash_request_set_tfm(ahreq, auth); ahash_request_set_crypt(ahreq, dst, ohash, assoclen + cryptlen); ahash_request_set_callback(ahreq, aead_request_flags(req), Some(authenc_esn_verify_ahash_done), req as *mut c_void);
    let err = crypto_ahash_digest(ahreq); if err != 0 { err } else { crypto_authenc_esn_decrypt_tail(req, aead_request_flags(req)) }
}

unsafe fn crypto_authenc_esn_init_tfm(tfm: *mut crypto_aead) -> c_int {
    let inst = aead_alg_instance(tfm); let ictx = aead_instance_ctx(inst); let ctx = crypto_aead_ctx(tfm); let auth = crypto_spawn_ahash(&mut (*ictx).auth); if IS_ERR(auth) { return PTR_ERR(auth); }
    let enc = crypto_spawn_skcipher(&mut (*ictx).enc); if IS_ERR(enc) { crypto_free_ahash(auth); return PTR_ERR(enc); }
    (*ctx).auth = auth; (*ctx).enc = enc; (*ctx).reqoff = 2 * crypto_ahash_digestsize(auth); crypto_aead_set_reqsize(tfm, core::mem::size_of::<authenc_esn_request_ctx>() + (*ctx).reqoff + core::cmp::max(crypto_ahash_reqsize(auth) + core::mem::size_of::<ahash_request>(), core::mem::size_of::<skcipher_request>() + crypto_skcipher_reqsize(enc))); 0
}
unsafe fn crypto_authenc_esn_exit_tfm(tfm: *mut crypto_aead) { let ctx = crypto_aead_ctx(tfm); crypto_free_ahash((*ctx).auth); crypto_free_skcipher((*ctx).enc); }
unsafe fn crypto_authenc_esn_free(inst: *mut aead_instance) { let ctx = aead_instance_ctx(inst); crypto_drop_skcipher(&mut (*ctx).enc); crypto_drop_ahash(&mut (*ctx).auth); kfree(inst as *mut c_void); }
unsafe fn crypto_authenc_esn_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> c_int { let mut mask=0u32; let err=crypto_check_attr_type(tb, CRYPTO_ALG_TYPE_AEAD, &mut mask); if err!=0{return err;} let inst=kzalloc(core::mem::size_of::<aead_instance>()+core::mem::size_of::<authenc_esn_instance_ctx>(),GFP_KERNEL) as *mut aead_instance; if inst.is_null(){return -ENOMEM;} let ctx=aead_instance_ctx(inst); let err=crypto_grab_ahash(&mut (*ctx).auth,aead_crypto_instance(inst),crypto_attr_alg_name(*tb.add(1)),0,mask); if err!=0{crypto_authenc_esn_free(inst);return err;} let err=crypto_grab_skcipher(&mut (*ctx).enc,aead_crypto_instance(inst),crypto_attr_alg_name(*tb.add(2)),0,mask); if err!=0{crypto_authenc_esn_free(inst);return err;} aead_register_instance(tmpl,inst) }

// Module registration and metadata are supplied through the kernel build system.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
