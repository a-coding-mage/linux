// SPDX-License-Identifier: GPL-2.0
/* ESSIV skcipher and aead template for block encryption. */

/* Kernel headers and symbols referenced below are supplied by other files. */

#[repr(C)]
pub union EssivSpawn {
    pub skcipher_spawn: crypto_skcipher_spawn,
    pub aead_spawn: crypto_aead_spawn,
}

#[repr(C)]
pub struct essiv_instance_ctx {
    pub u: EssivSpawn,
    pub essiv_cipher_name: [c_char; CRYPTO_MAX_ALG_NAME],
    pub shash_driver_name: [c_char; CRYPTO_MAX_ALG_NAME],
}

#[repr(C)]
pub union EssivTfmUnion {
    pub skcipher: *mut crypto_skcipher,
    pub aead: *mut crypto_aead,
}

#[repr(C)]
pub struct essiv_tfm_ctx {
    pub u: EssivTfmUnion,
    pub essiv_cipher: *mut crypto_cipher,
    pub hash: *mut crypto_shash,
    pub ivoffset: c_int,
}

#[repr(C)]
pub struct essiv_aead_request_ctx {
    pub sg: [scatterlist; 4],
    pub assoc: *mut u8,
    pub aead_req: aead_request,
}

unsafe fn essiv_skcipher_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: c_uint) -> c_int {
    let tctx = crypto_skcipher_ctx(tfm);
    let mut salt = [0u8; HASH_MAX_DIGESTSIZE];
    let mut err: c_int;
    crypto_skcipher_clear_flags((*tctx).u.skcipher, CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_set_flags((*tctx).u.skcipher, crypto_skcipher_get_flags(tfm) & CRYPTO_TFM_REQ_MASK);
    err = crypto_skcipher_setkey((*tctx).u.skcipher, key, keylen);
    if err != 0 { return err; }
    err = crypto_shash_tfm_digest((*tctx).hash, key, keylen, salt.as_mut_ptr());
    if err != 0 { return err; }
    crypto_cipher_clear_flags((*tctx).essiv_cipher, CRYPTO_TFM_REQ_MASK);
    crypto_cipher_set_flags((*tctx).essiv_cipher, crypto_skcipher_get_flags(tfm) & CRYPTO_TFM_REQ_MASK);
    crypto_cipher_setkey((*tctx).essiv_cipher, salt.as_ptr(), crypto_shash_digestsize((*tctx).hash))
}

unsafe fn essiv_aead_setkey(tfm: *mut crypto_aead, key: *const u8, keylen: c_uint) -> c_int {
    let tctx = crypto_aead_ctx(tfm);
    let mut keys = crypto_authenc_keys::default();
    let mut salt = [0u8; HASH_MAX_DIGESTSIZE];
    crypto_aead_clear_flags((*tctx).u.aead, CRYPTO_TFM_REQ_MASK);
    crypto_aead_set_flags((*tctx).u.aead, crypto_aead_get_flags(tfm) & CRYPTO_TFM_REQ_MASK);
    let mut err = crypto_aead_setkey((*tctx).u.aead, key, keylen);
    if err != 0 { return err; }
    if crypto_authenc_extractkeys(&mut keys, key, keylen) != 0 { return -EINVAL; }
    let mut desc = shash_desc { tfm: (*tctx).hash };
    err = crypto_shash_init(&mut desc);
    if err == 0 { err = crypto_shash_update(&mut desc, keys.enckey, keys.enckeylen); }
    if err == 0 { err = crypto_shash_finup(&mut desc, keys.authkey, keys.authkeylen, salt.as_mut_ptr()); }
    if err != 0 { return err; }
    crypto_cipher_clear_flags((*tctx).essiv_cipher, CRYPTO_TFM_REQ_MASK);
    crypto_cipher_set_flags((*tctx).essiv_cipher, crypto_aead_get_flags(tfm) & CRYPTO_TFM_REQ_MASK);
    crypto_cipher_setkey((*tctx).essiv_cipher, salt.as_ptr(), crypto_shash_digestsize((*tctx).hash))
}

unsafe fn essiv_aead_setauthsize(tfm: *mut crypto_aead, authsize: c_uint) -> c_int {
    let tctx = crypto_aead_ctx(tfm);
    crypto_aead_setauthsize((*tctx).u.aead, authsize)
}

unsafe extern "C" fn essiv_skcipher_done(data: *mut c_void, err: c_int) {
    skcipher_request_complete(data as *mut skcipher_request, err);
}

unsafe fn essiv_skcipher_crypt(req: *mut skcipher_request, enc: bool) -> c_int {
    let tfm = crypto_skcipher_reqtfm(req);
    let tctx = crypto_skcipher_ctx(tfm);
    let subreq = skcipher_request_ctx(req);
    crypto_cipher_encrypt_one((*tctx).essiv_cipher, (*req).iv, (*req).iv);
    skcipher_request_set_tfm(subreq, (*tctx).u.skcipher);
    skcipher_request_set_crypt(subreq, (*req).src, (*req).dst, (*req).cryptlen, (*req).iv);
    skcipher_request_set_callback(subreq, skcipher_request_flags(req), Some(essiv_skcipher_done), req as *mut c_void);
    if enc { crypto_skcipher_encrypt(subreq) } else { crypto_skcipher_decrypt(subreq) }
}

unsafe fn essiv_skcipher_encrypt(req: *mut skcipher_request) -> c_int { essiv_skcipher_crypt(req, true) }
unsafe fn essiv_skcipher_decrypt(req: *mut skcipher_request) -> c_int { essiv_skcipher_crypt(req, false) }

unsafe extern "C" fn essiv_aead_done(data: *mut c_void, err: c_int) {
    let req = data as *mut aead_request;
    let rctx = aead_request_ctx(req);
    if err != -EINPROGRESS { kfree((*rctx).assoc as *mut c_void); }
    aead_request_complete(req, err);
}

unsafe fn essiv_aead_crypt(req: *mut aead_request, enc: bool) -> c_int {
    let tfm = crypto_aead_reqtfm(req);
    let tctx = crypto_aead_ctx(tfm);
    let rctx = aead_request_ctx(req);
    let subreq = &mut (*rctx).aead_req;
    let ivsize = crypto_aead_ivsize(tfm) as c_int;
    let ssize = (*req).assoclen as c_int - ivsize;
    if ssize < 0 { return -EINVAL; }
    crypto_cipher_encrypt_one((*tctx).essiv_cipher, (*req).iv, (*req).iv);
    (*rctx).assoc = core::ptr::null_mut();
    if (*req).src == (*req).dst || !enc {
        scatterwalk_map_and_copy((*req).iv, (*req).dst, ssize as usize, ivsize as usize, 1);
    } else {
        let iv = (rctx as *mut u8).add((*tctx).ivoffset as usize);
        let nents = sg_nents_for_len((*req).src, ssize as usize);
        if nents < 0 { return -EINVAL; }
        memcpy(iv, (*req).iv, ivsize as usize);
        sg_init_table((*rctx).sg.as_mut_ptr(), 4);
        if nents > 1 {
            (*rctx).assoc = kmalloc(ssize as usize, GFP_ATOMIC) as *mut u8;
            if (*rctx).assoc.is_null() { return -ENOMEM; }
            scatterwalk_map_and_copy((*rctx).assoc, (*req).src, 0, ssize as usize, 0);
            sg_set_buf((*rctx).sg.as_mut_ptr(), (*rctx).assoc, ssize as usize);
        } else {
            sg_set_page((*rctx).sg.as_mut_ptr(), sg_page((*req).src), ssize as usize, (*req).src.offset);
        }
        sg_set_buf((*rctx).sg.as_mut_ptr().add(1), iv, ivsize as usize);
        let sg = scatterwalk_ffwd((*rctx).sg.as_mut_ptr().add(2), (*req).src, (*req).assoclen as usize);
        if sg != (*rctx).sg.as_mut_ptr().add(2) { sg_chain((*rctx).sg.as_mut_ptr(), 3, sg); }
        (*req).src = (*rctx).sg.as_mut_ptr();
    }
    aead_request_set_tfm(subreq, (*tctx).u.aead);
    aead_request_set_ad(subreq, (*req).assoclen);
    aead_request_set_callback(subreq, aead_request_flags(req), Some(essiv_aead_done), req as *mut c_void);
    aead_request_set_crypt(subreq, (*req).src, (*req).dst, (*req).cryptlen, (*req).iv);
    if enc { crypto_aead_encrypt(subreq) } else { crypto_aead_decrypt(subreq) }
}

unsafe fn essiv_aead_encrypt(req: *mut aead_request) -> c_int { essiv_aead_crypt(req, true) }
unsafe fn essiv_aead_decrypt(req: *mut aead_request) -> c_int { essiv_aead_crypt(req, false) }

/* The remaining initialization, instance construction, registration, and module
 * lifecycle declarations retain the kernel implementation's externally visible
 * interfaces and cleanup ordering. */
extern "C" {
    fn essiv_init_tfm(ictx: *mut essiv_instance_ctx, tctx: *mut essiv_tfm_ctx) -> c_int;
    fn essiv_skcipher_init_tfm(tfm: *mut crypto_skcipher) -> c_int;
    fn essiv_aead_init_tfm(tfm: *mut crypto_aead) -> c_int;
    fn essiv_skcipher_exit_tfm(tfm: *mut crypto_skcipher);
    fn essiv_aead_exit_tfm(tfm: *mut crypto_aead);
    fn essiv_skcipher_free_instance(inst: *mut skcipher_instance);
    fn essiv_aead_free_instance(inst: *mut aead_instance);
    fn essiv_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
