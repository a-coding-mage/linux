// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Authenc: Simple AEAD wrapper for IPsec
 *
 * Copyright (c) 2007-2015 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Kernel crypto, scatterlist, module, and rtnetlink dependencies are supplied externally.

#[repr(C)]
pub struct authenc_instance_ctx {
    pub auth: crypto_ahash_spawn,
    pub enc: crypto_skcipher_spawn,
    pub reqoff: c_uint,
}

#[repr(C)]
pub struct crypto_authenc_ctx {
    pub auth: *mut crypto_ahash,
    pub enc: *mut crypto_skcipher,
}

#[repr(C)]
pub struct authenc_request_ctx {
    pub src: [scatterlist; 2],
    pub dst: [scatterlist; 2],
    pub tail: [c_char; 0],
}

unsafe fn authenc_request_complete(req: *mut aead_request, err: c_int) {
    if err != -EINPROGRESS && err != -EBUSY {
        aead_request_complete(req, err);
    }
}

pub unsafe fn crypto_authenc_extractkeys(keys: *mut crypto_authenc_keys, key: *const u8, mut keylen: c_uint) -> c_int {
    let rta = key as *mut rtattr;
    if !RTA_OK(rta, keylen) || (*rta).rta_type != CRYPTO_AUTHENC_KEYA_PARAM {
        return -EINVAL;
    }
    /* RTA_OK() does not align the payload; require the exact aligned parameter size. */
    let param = RTA_DATA(rta) as *mut crypto_authenc_key_param;
    if RTA_PAYLOAD(rta) != core::mem::size_of::<crypto_authenc_key_param>() as c_uint {
        return -EINVAL;
    }
    (*keys).enckeylen = be32_to_cpu((*param).enckeylen);
    key = key.add((*rta).rta_len as usize);
    keylen -= (*rta).rta_len as c_uint;
    if keylen < (*keys).enckeylen { return -EINVAL; }
    (*keys).authkeylen = keylen - (*keys).enckeylen;
    (*keys).authkey = key;
    (*keys).enckey = key.add((*keys).authkeylen as usize);
    0
}

unsafe fn crypto_authenc_setkey(authenc: *mut crypto_aead, key: *const u8, keylen: c_uint) -> c_int {
    let ctx = crypto_aead_ctx(authenc) as *mut crypto_authenc_ctx;
    let auth = (*ctx).auth;
    let enc = (*ctx).enc;
    let mut keys: crypto_authenc_keys = core::mem::zeroed();
    let mut err = -EINVAL;
    if crypto_authenc_extractkeys(&mut keys, key, keylen) != 0 { return err; }
    crypto_ahash_clear_flags(auth, CRYPTO_TFM_REQ_MASK);
    crypto_ahash_set_flags(auth, crypto_aead_get_flags(authenc) & CRYPTO_TFM_REQ_MASK);
    err = crypto_ahash_setkey(auth, keys.authkey, keys.authkeylen);
    if err == 0 {
        crypto_skcipher_clear_flags(enc, CRYPTO_TFM_REQ_MASK);
        crypto_skcipher_set_flags(enc, crypto_aead_get_flags(authenc) & CRYPTO_TFM_REQ_MASK);
        err = crypto_skcipher_setkey(enc, keys.enckey, keys.enckeylen);
    }
    memzero_explicit(&mut keys as *mut _ as *mut c_void, core::mem::size_of_val(&keys));
    err
}

unsafe fn authenc_geniv_ahash_finish(req: *mut aead_request) {
    let authenc = crypto_aead_reqtfm(req);
    let inst = aead_alg_instance(authenc);
    let ictx = aead_instance_ctx(inst) as *mut authenc_instance_ctx;
    let areq_ctx = aead_request_ctx(req) as *mut authenc_request_ctx;
    let ahreq = (*areq_ctx).tail.as_mut_ptr().add((*ictx).reqoff as usize) as *mut ahash_request;
    scatterwalk_map_and_copy((*ahreq).result, (*req).dst, (*req).assoclen + (*req).cryptlen, crypto_aead_authsize(authenc), 1);
}

unsafe fn authenc_geniv_ahash_done(data: *mut c_void, err: c_int) {
    let req = data as *mut aead_request;
    if err == 0 { authenc_geniv_ahash_finish(req); }
    aead_request_complete(req, err);
}

/* Used when the ahash request was invoked in the async callback context of the previous skcipher request. */
unsafe fn authenc_geniv_ahash_done2(data: *mut c_void, err: c_int) {
    let req = data as *mut aead_request;
    if err == 0 { authenc_geniv_ahash_finish(req); }
    authenc_request_complete(req, err);
}

unsafe fn crypto_authenc_genicv(req: *mut aead_request, mask: c_uint) -> c_int {
    let authenc = crypto_aead_reqtfm(req);
    let inst = aead_alg_instance(authenc);
    let ctx = crypto_aead_ctx(authenc) as *mut crypto_authenc_ctx;
    let ictx = aead_instance_ctx(inst) as *mut authenc_instance_ctx;
    let areq_ctx = aead_request_ctx(req) as *mut authenc_request_ctx;
    let ahreq = (*areq_ctx).tail.as_mut_ptr().add((*ictx).reqoff as usize) as *mut ahash_request;
    let hash = (*areq_ctx).tail.as_mut_ptr() as *mut u8;
    let flags = aead_request_flags(req) & !mask;
    ahash_request_set_tfm(ahreq, (*ctx).auth);
    ahash_request_set_crypt(ahreq, (*req).dst, hash, (*req).assoclen + (*req).cryptlen);
    ahash_request_set_callback(ahreq, flags, if mask != 0 { Some(authenc_geniv_ahash_done2) } else { Some(authenc_geniv_ahash_done) }, req as *mut c_void);
    let err = crypto_ahash_digest(ahreq);
    if err != 0 { return err; }
    scatterwalk_map_and_copy(hash, (*req).dst, (*req).assoclen + (*req).cryptlen, crypto_aead_authsize(authenc), 1);
    0
}

unsafe fn crypto_authenc_encrypt_done(data: *mut c_void, mut err: c_int) {
    let req = data as *mut aead_request;
    if err != 0 { aead_request_complete(req, err); return; }
    err = crypto_authenc_genicv(req, CRYPTO_TFM_REQ_MAY_SLEEP);
    authenc_request_complete(req, err);
}

unsafe fn crypto_authenc_encrypt(req: *mut aead_request) -> c_int {
    let authenc = crypto_aead_reqtfm(req);
    let inst = aead_alg_instance(authenc);
    let ctx = crypto_aead_ctx(authenc) as *mut crypto_authenc_ctx;
    let ictx = aead_instance_ctx(inst) as *mut authenc_instance_ctx;
    let areq_ctx = aead_request_ctx(req) as *mut authenc_request_ctx;
    let skreq = (*areq_ctx).tail.as_mut_ptr().add((*ictx).reqoff as usize) as *mut skcipher_request;
    let src = scatterwalk_ffwd((*areq_ctx).src.as_mut_ptr(), (*req).src, (*req).assoclen);
    let mut dst = src;
    if (*req).src != (*req).dst {
        memcpy_sglist((*req).dst, (*req).src, (*req).assoclen);
        dst = scatterwalk_ffwd((*areq_ctx).dst.as_mut_ptr(), (*req).dst, (*req).assoclen);
    }
    skcipher_request_set_tfm(skreq, (*ctx).enc);
    skcipher_request_set_callback(skreq, aead_request_flags(req), Some(crypto_authenc_encrypt_done), req as *mut c_void);
    skcipher_request_set_crypt(skreq, src, dst, (*req).cryptlen, (*req).iv);
    let err = crypto_skcipher_encrypt(skreq);
    if err != 0 { return err; }
    crypto_authenc_genicv(req, 0)
}

unsafe fn authenc_decrypt_tail_done(data: *mut c_void, err: c_int) { authenc_request_complete(data as *mut aead_request, err); }

unsafe fn crypto_authenc_decrypt_tail(req: *mut aead_request, mask: c_uint) -> c_int {
    let authenc = crypto_aead_reqtfm(req);
    let inst = aead_alg_instance(authenc);
    let ctx = crypto_aead_ctx(authenc) as *mut crypto_authenc_ctx;
    let ictx = aead_instance_ctx(inst) as *mut authenc_instance_ctx;
    let areq_ctx = aead_request_ctx(req) as *mut authenc_request_ctx;
    let ahreq = (*areq_ctx).tail.as_mut_ptr().add((*ictx).reqoff as usize) as *mut ahash_request;
    let skreq = ahreq as *mut skcipher_request;
    let authsize = crypto_aead_authsize(authenc);
    let ihash = (*ahreq).result.add(authsize as usize);
    scatterwalk_map_and_copy(ihash, (*req).src, (*ahreq).nbytes, authsize, 0);
    if crypto_memneq(ihash, (*ahreq).result, authsize) != 0 { return -EBADMSG; }
    let src = scatterwalk_ffwd((*areq_ctx).src.as_mut_ptr(), (*req).src, (*req).assoclen);
    let dst = if (*req).src != (*req).dst { scatterwalk_ffwd((*areq_ctx).dst.as_mut_ptr(), (*req).dst, (*req).assoclen) } else { src };
    skcipher_request_set_tfm(skreq, (*ctx).enc);
    skcipher_request_set_callback(skreq, aead_request_flags(req) & !mask, if mask != 0 { Some(authenc_decrypt_tail_done) } else { (*req).base.complete }, if mask != 0 { req as *mut c_void } else { (*req).base.data });
    skcipher_request_set_crypt(skreq, src, dst, (*req).cryptlen - authsize, (*req).iv);
    crypto_skcipher_decrypt(skreq)
}

unsafe fn authenc_verify_ahash_done(data: *mut c_void, mut err: c_int) {
    let req = data as *mut aead_request;
    if err != 0 { aead_request_complete(req, err); return; }
    err = crypto_authenc_decrypt_tail(req, CRYPTO_TFM_REQ_MAY_SLEEP);
    authenc_request_complete(req, err);
}

unsafe fn crypto_authenc_decrypt(req: *mut aead_request) -> c_int {
    let authenc = crypto_aead_reqtfm(req);
    let authsize = crypto_aead_authsize(authenc);
    let inst = aead_alg_instance(authenc);
    let ctx = crypto_aead_ctx(authenc) as *mut crypto_authenc_ctx;
    let ictx = aead_instance_ctx(inst) as *mut authenc_instance_ctx;
    let areq_ctx = aead_request_ctx(req) as *mut authenc_request_ctx;
    let ahreq = (*areq_ctx).tail.as_mut_ptr().add((*ictx).reqoff as usize) as *mut ahash_request;
    let hash = (*areq_ctx).tail.as_mut_ptr() as *mut u8;
    ahash_request_set_tfm(ahreq, (*ctx).auth);
    ahash_request_set_crypt(ahreq, (*req).src, hash, (*req).assoclen + (*req).cryptlen - authsize);
    ahash_request_set_callback(ahreq, aead_request_flags(req), Some(authenc_verify_ahash_done), req as *mut c_void);
    let err = crypto_ahash_digest(ahreq);
    if err != 0 { return err; }
    crypto_authenc_decrypt_tail(req, 0)
}

unsafe fn crypto_authenc_init_tfm(tfm: *mut crypto_aead) -> c_int {
    let inst = aead_alg_instance(tfm);
    let ictx = aead_instance_ctx(inst) as *mut authenc_instance_ctx;
    let ctx = crypto_aead_ctx(tfm) as *mut crypto_authenc_ctx;
    let auth = crypto_spawn_ahash(&mut (*ictx).auth);
    if IS_ERR(auth) { return PTR_ERR(auth); }
    let enc = crypto_spawn_skcipher(&mut (*ictx).enc);
    if IS_ERR(enc) { crypto_free_ahash(auth); return PTR_ERR(enc); }
    (*ctx).auth = auth; (*ctx).enc = enc;
    crypto_aead_set_reqsize(tfm, core::mem::size_of::<authenc_request_ctx>() + (*ictx).reqoff as usize + core::cmp::max(crypto_ahash_reqsize(auth) + core::mem::size_of::<ahash_request>(), core::mem::size_of::<skcipher_request>() + crypto_skcipher_reqsize(enc)));
    0
}
unsafe fn crypto_authenc_exit_tfm(tfm: *mut crypto_aead) {
    let ctx = crypto_aead_ctx(tfm) as *mut crypto_authenc_ctx;
    crypto_free_ahash((*ctx).auth); crypto_free_skcipher((*ctx).enc);
}
unsafe fn crypto_authenc_free(inst: *mut aead_instance) {
    let ctx = aead_instance_ctx(inst) as *mut authenc_instance_ctx;
    crypto_drop_skcipher(&mut (*ctx).enc); crypto_drop_ahash(&mut (*ctx).auth); kfree(inst as *mut c_void);
}
unsafe fn crypto_authenc_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> c_int {
    let mut mask = 0u32;
    let err = crypto_check_attr_type(tb, CRYPTO_ALG_TYPE_AEAD, &mut mask); if err != 0 { return err; }
    let inst = kzalloc(core::mem::size_of::<aead_instance>() + core::mem::size_of::<authenc_instance_ctx>(), GFP_KERNEL) as *mut aead_instance;
    if inst.is_null() { return -ENOMEM; }
    let ctx = aead_instance_ctx(inst) as *mut authenc_instance_ctx;
    let err = crypto_grab_ahash(&mut (*ctx).auth, aead_crypto_instance(inst), crypto_attr_alg_name(*tb.add(1)), 0, mask);
    if err != 0 { crypto_authenc_free(inst); return err; }
    let auth = crypto_spawn_ahash_alg(&mut (*ctx).auth);
    let err = crypto_grab_skcipher(&mut (*ctx).enc, aead_crypto_instance(inst), crypto_attr_alg_name(*tb.add(2)), 0, mask);
    if err != 0 { crypto_authenc_free(inst); return err; }
    let enc = crypto_spawn_skcipher_alg_common(&mut (*ctx).enc);
    (*ctx).reqoff = 2 * (*auth).digestsize;
    (*inst).alg.ivsize = (*enc).ivsize; (*inst).alg.chunksize = (*enc).chunksize; (*inst).alg.maxauthsize = (*auth).digestsize;
    (*inst).alg.init = Some(crypto_authenc_init_tfm); (*inst).alg.exit = Some(crypto_authenc_exit_tfm);
    (*inst).alg.setkey = Some(crypto_authenc_setkey); (*inst).alg.encrypt = Some(crypto_authenc_encrypt); (*inst).alg.decrypt = Some(crypto_authenc_decrypt);
    (*inst).free = Some(crypto_authenc_free);
    let err = aead_register_instance(tmpl, inst); if err != 0 { crypto_authenc_free(inst); } err
}

// module_init(crypto_authenc_module_init); module_exit(crypto_authenc_module_exit);
// MODULE_LICENSE("GPL"); MODULE_DESCRIPTION("Simple AEAD wrapper for IPsec"); MODULE_ALIAS_CRYPTO("authenc");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
