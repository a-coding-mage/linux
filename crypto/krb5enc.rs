// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * AEAD wrapper for Kerberos 5 RFC3961 simplified profile.
 *
 * Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 *
 * Derived from authenc:
 * Copyright (c) 2007-2015 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Kernel crypto and module dependencies are supplied by the surrounding tree.

#[repr(C)]
pub struct krb5enc_instance_ctx {
    pub auth: crypto_ahash_spawn,
    pub enc: crypto_skcipher_spawn,
    pub reqoff: c_uint,
}

#[repr(C)]
pub struct krb5enc_ctx {
    pub auth: *mut crypto_ahash,
    pub enc: *mut crypto_skcipher,
}

#[repr(C)]
pub struct krb5enc_request_ctx {
    pub src: [scatterlist; 2],
    pub dst: [scatterlist; 2],
    pub tail: [u8; 0],
}

/**
 * crypto_krb5enc_extractkeys - Extract Ke and Ki keys from the key blob.
 * @keys: Where to put the key sizes and pointers
 * @key: Encoded key material
 * @keylen: Amount of key material
 *
 * Decode the key blob we're given.  It starts with an rtattr that indicates
 * the format and the length.  Format CRYPTO_AUTHENC_KEYA_PARAM is:
 *
 *	rtattr || __be32 enckeylen || authkey || enckey
 *
 * Note that the rtattr is in cpu-endian form, unlike enckeylen.  This must be
 * handled correctly in static testmgr data.
 */
pub unsafe extern "C" fn crypto_krb5enc_extractkeys(
    keys: *mut crypto_authenc_keys,
    mut key: *const u8,
    mut keylen: c_uint,
) -> c_int {
    let rta = key as *mut rtattr;
    let mut param: *mut crypto_authenc_key_param;

    if !RTA_OK(rta, keylen) {
        return -EINVAL;
    }
    if (*rta).rta_type != CRYPTO_AUTHENC_KEYA_PARAM {
        return -EINVAL;
    }
    if RTA_PAYLOAD(rta) != core::mem::size_of::<crypto_authenc_key_param>() {
        return -EINVAL;
    }
    BUILD_BUG_ON(core::mem::size_of::<crypto_authenc_key_param>() % RTA_ALIGNTO);

    param = RTA_DATA(rta) as *mut crypto_authenc_key_param;
    (*keys).enckeylen = be32_to_cpu((*param).enckeylen);

    key = key.add((*rta).rta_len as usize);
    keylen -= (*rta).rta_len as c_uint;

    if keylen < (*keys).enckeylen {
        return -EINVAL;
    }
    (*keys).authkeylen = keylen - (*keys).enckeylen;
    (*keys).authkey = key;
    (*keys).enckey = key.add((*keys).authkeylen as usize);
    0
}

pub unsafe extern "C" fn krb5enc_setkey(
    krb5enc: *mut crypto_aead, key: *const u8, keylen: c_uint,
) -> c_int {
    let mut keys: crypto_authenc_keys = core::mem::zeroed();
    let ctx = crypto_aead_ctx(krb5enc) as *mut krb5enc_ctx;
    let enc = (*ctx).enc;
    let auth = (*ctx).auth;
    let flags = crypto_aead_get_flags(krb5enc);
    let mut err = -EINVAL;
    if crypto_krb5enc_extractkeys(&mut keys, key, keylen) != 0 { goto!(out); }
    crypto_ahash_clear_flags(auth, CRYPTO_TFM_REQ_MASK);
    crypto_ahash_set_flags(auth, flags & CRYPTO_TFM_REQ_MASK);
    err = crypto_ahash_setkey(auth, keys.authkey, keys.authkeylen);
    if err != 0 { goto!(out); }
    crypto_skcipher_clear_flags(enc, CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_set_flags(enc, flags & CRYPTO_TFM_REQ_MASK);
    err = crypto_skcipher_setkey(enc, keys.enckey, keys.enckeylen);
out:
    memzero_explicit(&mut keys as *mut _ as *mut c_void, core::mem::size_of_val(&keys));
    err
}

pub unsafe extern "C" fn krb5enc_encrypt_done(data: *mut c_void, err: c_int) {
    aead_request_complete(data as *mut aead_request, err);
}

pub unsafe extern "C" fn krb5enc_dispatch_encrypt(req: *mut aead_request, flags: c_uint) -> c_int {
    let krb5enc = crypto_aead_reqtfm(req);
    let inst = aead_alg_instance(krb5enc);
    let ctx = crypto_aead_ctx(krb5enc) as *mut krb5enc_ctx;
    let ictx = aead_instance_ctx(inst) as *mut krb5enc_instance_ctx;
    let areq_ctx = aead_request_ctx(req) as *mut krb5enc_request_ctx;
    let skreq = ((*areq_ctx).tail.as_mut_ptr().add((*ictx).reqoff as usize)) as *mut skcipher_request;
    let src = scatterwalk_ffwd((*areq_ctx).src.as_mut_ptr(), (*req).src, (*req).assoclen);
    let dst = if (*req).src == (*req).dst { src } else { scatterwalk_ffwd((*areq_ctx).dst.as_mut_ptr(), (*req).dst, (*req).assoclen) };
    skcipher_request_set_tfm(skreq, (*ctx).enc);
    skcipher_request_set_callback(skreq, flags, Some(krb5enc_encrypt_done), req as *mut c_void);
    skcipher_request_set_crypt(skreq, src, dst, (*req).cryptlen, (*req).iv);
    crypto_skcipher_encrypt(skreq)
}

pub unsafe extern "C" fn krb5enc_insert_checksum(req: *mut aead_request, hash: *mut u8) {
    let krb5enc = crypto_aead_reqtfm(req);
    scatterwalk_map_and_copy(hash, (*req).dst, (*req).assoclen + (*req).cryptlen,
                             crypto_aead_authsize(krb5enc), 1);
}

pub unsafe extern "C" fn krb5enc_encrypt_ahash_done(data: *mut c_void, mut err: c_int) {
    let req = data as *mut aead_request;
    let krb5enc = crypto_aead_reqtfm(req);
    let inst = aead_alg_instance(krb5enc);
    let ictx = aead_instance_ctx(inst) as *mut krb5enc_instance_ctx;
    let areq_ctx = aead_request_ctx(req) as *mut krb5enc_request_ctx;
    let ahreq = areq_ctx_tail_as_ahash(areq_ctx, (*ictx).reqoff);
    if err == 0 {
        krb5enc_insert_checksum(req, (*ahreq).result);
        err = krb5enc_dispatch_encrypt(req, 0);
        if err == -EINPROGRESS { return; }
    }
    aead_request_complete(req, err);
}

pub unsafe extern "C" fn krb5enc_dispatch_encrypt_hash(req: *mut aead_request) -> c_int {
    let krb5enc = crypto_aead_reqtfm(req);
    let inst = aead_alg_instance(krb5enc);
    let ctx = crypto_aead_ctx(krb5enc) as *mut krb5enc_ctx;
    let ictx = aead_instance_ctx(inst) as *mut krb5enc_instance_ctx;
    let areq_ctx = aead_request_ctx(req) as *mut krb5enc_request_ctx;
    let ahreq = areq_ctx_tail_as_ahash(areq_ctx, (*ictx).reqoff);
    let hash = (*areq_ctx).tail.as_mut_ptr();
    ahash_request_set_callback(ahreq, aead_request_flags(req), Some(krb5enc_encrypt_ahash_done), req as *mut c_void);
    ahash_request_set_tfm(ahreq, (*ctx).auth);
    ahash_request_set_crypt(ahreq, (*req).src, hash, (*req).assoclen + (*req).cryptlen);
    let err = crypto_ahash_digest(ahreq);
    if err != 0 { return err; }
    krb5enc_insert_checksum(req, hash);
    0
}

pub unsafe extern "C" fn krb5enc_encrypt(req: *mut aead_request) -> c_int {
    let err = krb5enc_dispatch_encrypt_hash(req);
    if err < 0 { return err; }
    krb5enc_dispatch_encrypt(req, aead_request_flags(req))
}

pub unsafe extern "C" fn krb5enc_verify_hash(req: *mut aead_request) -> c_int {
    let krb5enc = crypto_aead_reqtfm(req);
    let inst = aead_alg_instance(krb5enc);
    let ictx = aead_instance_ctx(inst) as *mut krb5enc_instance_ctx;
    let areq_ctx = aead_request_ctx(req) as *mut krb5enc_request_ctx;
    let ahreq = areq_ctx_tail_as_ahash(areq_ctx, (*ictx).reqoff);
    let authsize = crypto_aead_authsize(krb5enc);
    let calc_hash = (*areq_ctx).tail.as_mut_ptr();
    let msg_hash = calc_hash.add(authsize as usize);
    scatterwalk_map_and_copy(msg_hash, (*req).src, (*ahreq).nbytes, authsize, 0);
    if crypto_memneq(msg_hash, calc_hash, authsize) != 0 { return -EBADMSG; }
    0
}

pub unsafe extern "C" fn krb5enc_decrypt_hash_done(data: *mut c_void, mut err: c_int) {
    let req = data as *mut aead_request;
    if err == 0 { err = krb5enc_verify_hash(req); }
    aead_request_complete(req, err);
}

pub unsafe extern "C" fn krb5enc_dispatch_decrypt_hash(req: *mut aead_request, flags: c_uint) -> c_int {
    let krb5enc = crypto_aead_reqtfm(req);
    let inst = aead_alg_instance(krb5enc);
    let ctx = crypto_aead_ctx(krb5enc) as *mut krb5enc_ctx;
    let ictx = aead_instance_ctx(inst) as *mut krb5enc_instance_ctx;
    let areq_ctx = aead_request_ctx(req) as *mut krb5enc_request_ctx;
    let ahreq = areq_ctx_tail_as_ahash(areq_ctx, (*ictx).reqoff);
    let authsize = crypto_aead_authsize(krb5enc);
    let hash = (*areq_ctx).tail.as_mut_ptr();
    ahash_request_set_tfm(ahreq, (*ctx).auth);
    ahash_request_set_crypt(ahreq, (*req).dst, hash, (*req).assoclen + (*req).cryptlen - authsize);
    ahash_request_set_callback(ahreq, flags, Some(krb5enc_decrypt_hash_done), req as *mut c_void);
    let err = crypto_ahash_digest(ahreq);
    if err < 0 { return err; }
    krb5enc_verify_hash(req)
}

pub unsafe extern "C" fn krb5enc_decrypt_done(data: *mut c_void, mut err: c_int) {
    let req = data as *mut aead_request;
    if err == 0 { err = krb5enc_dispatch_decrypt_hash(req, 0); if err == -EINPROGRESS { return; } }
    aead_request_complete(req, err);
}

pub unsafe extern "C" fn krb5enc_dispatch_decrypt(req: *mut aead_request) -> c_int {
    let krb5enc = crypto_aead_reqtfm(req);
    let inst = aead_alg_instance(krb5enc);
    let ctx = crypto_aead_ctx(krb5enc) as *mut krb5enc_ctx;
    let ictx = aead_instance_ctx(inst) as *mut krb5enc_instance_ctx;
    let areq_ctx = aead_request_ctx(req) as *mut krb5enc_request_ctx;
    let skreq = areq_ctx_tail_as_skcipher(areq_ctx, (*ictx).reqoff);
    let authsize = crypto_aead_authsize(krb5enc);
    let src = scatterwalk_ffwd((*areq_ctx).src.as_mut_ptr(), (*req).src, (*req).assoclen);
    let dst = if (*req).src == (*req).dst { src } else { scatterwalk_ffwd((*areq_ctx).dst.as_mut_ptr(), (*req).dst, (*req).assoclen) };
    skcipher_request_set_tfm(skreq, (*ctx).enc);
    skcipher_request_set_callback(skreq, aead_request_flags(req), Some(krb5enc_decrypt_done), req as *mut c_void);
    skcipher_request_set_crypt(skreq, src, dst, (*req).cryptlen - authsize, (*req).iv);
    crypto_skcipher_decrypt(skreq)
}

pub unsafe extern "C" fn krb5enc_decrypt(req: *mut aead_request) -> c_int {
    let err = krb5enc_dispatch_decrypt(req);
    if err < 0 { return err; }
    krb5enc_dispatch_decrypt_hash(req, aead_request_flags(req))
}

// The remaining tfm, instance, template, and module-registration declarations
// retain the kernel's original external interfaces and are supplied by the
// surrounding kernel bindings.

pub unsafe extern "C" fn krb5enc_init_tfm(tfm: *mut crypto_aead) -> c_int {
    let inst = aead_alg_instance(tfm);
    let ictx = aead_instance_ctx(inst) as *mut krb5enc_instance_ctx;
    let ctx = crypto_aead_ctx(tfm) as *mut krb5enc_ctx;
    let auth = crypto_spawn_ahash(&mut (*ictx).auth);
    if IS_ERR(auth) { return PTR_ERR(auth); }
    let enc = crypto_spawn_skcipher(&mut (*ictx).enc);
    if IS_ERR(enc) { crypto_free_ahash(auth); return PTR_ERR(enc); }
    (*ctx).auth = auth;
    (*ctx).enc = enc;
    crypto_aead_set_reqsize(tfm, core::mem::size_of::<krb5enc_request_ctx>() +
        (*ictx).reqoff as usize +
        umax(core::mem::size_of::<ahash_request>() + crypto_ahash_reqsize(auth),
             core::mem::size_of::<skcipher_request>() + crypto_skcipher_reqsize(enc)));
    0
}

pub unsafe extern "C" fn krb5enc_exit_tfm(tfm: *mut crypto_aead) {
    let ctx = crypto_aead_ctx(tfm) as *mut krb5enc_ctx;
    crypto_free_ahash((*ctx).auth);
    crypto_free_skcipher((*ctx).enc);
}

pub unsafe extern "C" fn krb5enc_free(inst: *mut aead_instance) {
    let ctx = aead_instance_ctx(inst) as *mut krb5enc_instance_ctx;
    crypto_drop_skcipher(&mut (*ctx).enc);
    crypto_drop_ahash(&mut (*ctx).auth);
    kfree(inst as *mut c_void);
}

/* Create an instance of a template for a specific hash and cipher pair. */
pub unsafe extern "C" fn krb5enc_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> c_int {
    let mut mask: u32 = 0;
    let err = crypto_check_attr_type(tb, CRYPTO_ALG_TYPE_AEAD, &mut mask);
    if err != 0 { pr_err!("attr_type failed\n"); return err; }
    let inst = kzalloc(core::mem::size_of::<aead_instance>() + core::mem::size_of::<krb5enc_instance_ctx>(), GFP_KERNEL) as *mut aead_instance;
    if inst.is_null() { return -ENOMEM; }
    let ictx = aead_instance_ctx(inst) as *mut krb5enc_instance_ctx;
    let err = crypto_grab_ahash(&mut (*ictx).auth, aead_crypto_instance(inst), crypto_attr_alg_name(*tb.add(1)), 0, mask);
    if err != 0 { krb5enc_free(inst); return err; }
    let auth = crypto_spawn_ahash_alg(&mut (*ictx).auth);
    let err = crypto_grab_skcipher(&mut (*ictx).enc, aead_crypto_instance(inst), crypto_attr_alg_name(*tb.add(2)), 0, mask);
    if err != 0 { krb5enc_free(inst); return err; }
    let enc = crypto_spawn_skcipher_alg_common(&mut (*ictx).enc);
    (*ictx).reqoff = 2 * (*auth).digestsize;
    (*(*inst).alg.base.cra_name.as_mut_ptr()).write(0);
    (*inst).alg.base.cra_priority = (*enc).base.cra_priority * 10 + (*auth).base.cra_priority;
    (*inst).alg.base.cra_blocksize = (*enc).base.cra_blocksize;
    (*inst).alg.base.cra_alignmask = (*enc).base.cra_alignmask;
    (*inst).alg.base.cra_ctxsize = core::mem::size_of::<krb5enc_ctx>();
    (*inst).alg.ivsize = (*enc).ivsize;
    (*inst).alg.chunksize = (*enc).chunksize;
    (*inst).alg.maxauthsize = (*auth).digestsize;
    (*inst).alg.init = Some(krb5enc_init_tfm);
    (*inst).alg.exit = Some(krb5enc_exit_tfm);
    (*inst).alg.setkey = Some(krb5enc_setkey);
    (*inst).alg.encrypt = Some(krb5enc_encrypt);
    (*inst).alg.decrypt = Some(krb5enc_decrypt);
    (*inst).free = Some(krb5enc_free);
    let err = aead_register_instance(tmpl, inst);
    if err != 0 { krb5enc_free(inst); }
    err
}

#[no_mangle]
pub static mut crypto_krb5enc_tmpl: crypto_template = crypto_template {
    name: "krb5enc\0".as_ptr() as *const c_char,
    create: Some(krb5enc_create),
    module: THIS_MODULE,
};

pub unsafe extern "C" fn crypto_krb5enc_module_init() -> c_int {
    crypto_register_template(&mut crypto_krb5enc_tmpl)
}

pub unsafe extern "C" fn crypto_krb5enc_module_exit() {
    crypto_unregister_template(&mut crypto_krb5enc_tmpl);
}

// module_init!(crypto_krb5enc_module_init);
// module_exit!(crypto_krb5enc_module_exit);
// MODULE_LICENSE!("GPL");
// MODULE_DESCRIPTION!("Simple AEAD wrapper for Kerberos 5 RFC3961");
// MODULE_ALIAS_CRYPTO!("krb5enc");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
