// SPDX-License-Identifier: GPL-2.0-or-later
/* ChaCha20-Poly1305 AEAD, RFC7539 */

// Kernel crypto types, constants, and helper functions are supplied by the
// surrounding kernel bindings.
use crate::*;

#[repr(C)]
pub struct ChachapolyInstanceCtx { pub chacha: crypto_skcipher_spawn, pub saltlen: c_uint }
#[repr(C)]
pub struct ChachapolyCtx { pub chacha: *mut crypto_skcipher, pub saltlen: c_uint, pub salt: [u8; 0] }
#[repr(C)]
pub struct ChachaReq { pub iv: [u8; CHACHA_IV_SIZE], pub src: [scatterlist; 1], pub req: skcipher_request }
#[repr(C)]
pub struct ChachapolyReqCtx {
    pub src: [scatterlist; 2], pub dst: [scatterlist; 2],
    pub key: [u8; POLY1305_KEY_SIZE], pub tag: [u8; POLY1305_DIGEST_SIZE],
    pub cryptlen: c_uint, pub assoclen: c_uint, pub flags: u32,
    pub u: ChachaReq,
}

#[inline]
unsafe fn async_done_continue(req: *mut aead_request, mut err: c_int,
                              cont: unsafe fn(*mut aead_request) -> c_int) {
    if err == 0 {
        let rctx = aead_request_ctx::<ChachapolyReqCtx>(req);
        (*rctx).flags &= !CRYPTO_TFM_REQ_MAY_SLEEP;
        err = cont(req);
    }
    if err != -EINPROGRESS && err != -EBUSY { aead_request_complete(req, err); }
}

unsafe fn chacha_iv(iv: *mut u8, req: *mut aead_request, icb: u32) {
    let ctx = crypto_aead_ctx::<ChachapolyCtx>(crypto_aead_reqtfm(req));
    let leicb = cpu_to_le32(icb);
    memcpy(iv as *mut _, &leicb as *const _ as *const _, core::mem::size_of_val(&leicb));
    memcpy(iv.add(core::mem::size_of_val(&leicb)) as *mut _, (*ctx).salt.as_ptr() as *const _, (*ctx).saltlen as usize);
    memcpy(iv.add(4 + (*ctx).saltlen as usize) as *mut _, (*req).iv.as_ptr() as *const _, CHACHA_IV_SIZE - 4 - (*ctx).saltlen as usize);
}

unsafe fn poly_verify_tag(req: *mut aead_request) -> c_int {
    let rctx = aead_request_ctx::<ChachapolyReqCtx>(req);
    let mut tag = [0u8; POLY1305_DIGEST_SIZE];
    scatterwalk_map_and_copy(tag.as_mut_ptr() as *mut _, (*req).src, (*req).assoclen + (*rctx).cryptlen, tag.len(), 0);
    if crypto_memneq(tag.as_ptr() as *const _, (*rctx).tag.as_ptr() as *const _, tag.len()) != 0 { -EBADMSG } else { 0 }
}
unsafe fn chacha_decrypt_done(data: *mut c_void, err: c_int) { async_done_continue(data as *mut aead_request, err, poly_verify_tag); }

unsafe fn chacha_decrypt(req: *mut aead_request) -> c_int {
    let ctx = crypto_aead_ctx::<ChachapolyCtx>(crypto_aead_reqtfm(req));
    let rctx = aead_request_ctx::<ChachapolyReqCtx>(req);
    if (*rctx).cryptlen != 0 {
        chacha_iv((*rctx).u.iv.as_mut_ptr(), req, 1);
        let src = scatterwalk_ffwd((*rctx).src.as_mut_ptr(), (*req).src, (*req).assoclen);
        let dst = if (*req).src == (*req).dst { src } else { scatterwalk_ffwd((*rctx).dst.as_mut_ptr(), (*req).dst, (*req).assoclen) };
        skcipher_request_set_callback(&mut (*rctx).u.req, (*rctx).flags, Some(chacha_decrypt_done), req as *mut _);
        skcipher_request_set_tfm(&mut (*rctx).u.req, (*ctx).chacha);
        skcipher_request_set_crypt(&mut (*rctx).u.req, src, dst, (*rctx).cryptlen, (*rctx).u.iv.as_mut_ptr());
        let err = crypto_skcipher_decrypt(&mut (*rctx).u.req);
        if err != 0 { return err; }
    }
    poly_verify_tag(req)
}

unsafe fn poly_hash(req: *mut aead_request) -> c_int {
    let rctx = aead_request_ctx::<ChachapolyReqCtx>(req);
    if (*req).src != (*req).dst { memcpy_sglist((*req).dst, (*req).src, (*req).assoclen); }
    if (*rctx).cryptlen == (*req).cryptlen { (*req).src = (*req).dst; }
    // Poly1305 scatter-walk and length encoding are delegated to the kernel API.
    let mut desc = core::mem::MaybeUninit::<poly1305_desc_ctx>::zeroed();
    poly1305_init(desc.as_mut_ptr(), (*rctx).key.as_ptr());
    poly1305_hash_aead(desc.as_mut_ptr(), (*req).src, (*rctx).assoclen, (*rctx).cryptlen, (*rctx).tag.as_mut_ptr());
    if (*rctx).cryptlen != (*req).cryptlen { chacha_decrypt(req) } else { 0 }
}
unsafe fn poly_genkey_done(data: *mut c_void, err: c_int) { async_done_continue(data as *mut aead_request, err, poly_hash); }
unsafe fn poly_genkey(req: *mut aead_request) -> c_int {
    let tfm = crypto_aead_reqtfm(req); let ctx = crypto_aead_ctx::<ChachapolyCtx>(tfm); let rctx = aead_request_ctx::<ChachapolyReqCtx>(req);
    (*rctx).assoclen = (*req).assoclen; if crypto_aead_ivsize(tfm) == 8 { if (*rctx).assoclen < 8 { return -EINVAL; } (*rctx).assoclen -= 8; }
    memset((*rctx).key.as_mut_ptr() as *mut _, 0, (*rctx).key.len()); sg_init_one((*rctx).u.src.as_mut_ptr(), (*rctx).key.as_mut_ptr() as *mut _, (*rctx).key.len());
    chacha_iv((*rctx).u.iv.as_mut_ptr(), req, 0); skcipher_request_set_callback(&mut (*rctx).u.req, (*rctx).flags, Some(poly_genkey_done), req as *mut _); skcipher_request_set_tfm(&mut (*rctx).u.req, (*ctx).chacha); skcipher_request_set_crypt(&mut (*rctx).u.req, (*rctx).u.src.as_mut_ptr(), (*rctx).u.src.as_mut_ptr(), POLY1305_KEY_SIZE, (*rctx).u.iv.as_mut_ptr());
    let err = crypto_skcipher_decrypt(&mut (*rctx).u.req); if err != 0 { err } else { poly_hash(req) }
}
unsafe fn chacha_encrypt_done(data: *mut c_void, err: c_int) { async_done_continue(data as *mut aead_request, err, poly_genkey); }

unsafe fn chacha_encrypt(req: *mut aead_request) -> c_int {
    let ctx = crypto_aead_ctx::<ChachapolyCtx>(crypto_aead_reqtfm(req)); let rctx = aead_request_ctx::<ChachapolyReqCtx>(req);
    if (*req).cryptlen != 0 { chacha_iv((*rctx).u.iv.as_mut_ptr(), req, 1); let src = scatterwalk_ffwd((*rctx).src.as_mut_ptr(), (*req).src, (*req).assoclen); let dst = if (*req).src == (*req).dst { src } else { scatterwalk_ffwd((*rctx).dst.as_mut_ptr(), (*req).dst, (*req).assoclen) }; skcipher_request_set_callback(&mut (*rctx).u.req, (*rctx).flags, Some(chacha_encrypt_done), req as *mut _); skcipher_request_set_tfm(&mut (*rctx).u.req, (*ctx).chacha); skcipher_request_set_crypt(&mut (*rctx).u.req, src, dst, (*req).cryptlen, (*rctx).u.iv.as_mut_ptr()); let err = crypto_skcipher_encrypt(&mut (*rctx).u.req); if err != 0 { return err; } }
    poly_genkey(req)
}
unsafe fn chachapoly_encrypt(req: *mut aead_request) -> c_int { let r = aead_request_ctx::<ChachapolyReqCtx>(req); (*r).cryptlen = (*req).cryptlen; (*r).flags = aead_request_flags(req); chacha_encrypt(req) }
unsafe fn chachapoly_decrypt(req: *mut aead_request) -> c_int { let r = aead_request_ctx::<ChachapolyReqCtx>(req); if (*req).cryptlen < POLY1305_DIGEST_SIZE { return -EINVAL; } (*r).cryptlen = (*req).cryptlen - POLY1305_DIGEST_SIZE; (*r).flags = aead_request_flags(req); poly_genkey(req) }

// The remaining registration and module plumbing is represented by the kernel
// bindings; the two templates retain the original names and IV sizes.
pub static RFC7539_IVSIZE: u32 = 12;
pub static RFC7539ESP_IVSIZE: u32 = 8;

unsafe fn chachapoly_setauthsize(_tfm: *mut crypto_aead, authsize: c_uint) -> c_int {
    if authsize != POLY1305_DIGEST_SIZE { -EINVAL } else { 0 }
}
unsafe fn chachapoly_setkey(aead: *mut crypto_aead, key: *const u8, keylen: c_uint) -> c_int {
    let ctx = crypto_aead_ctx::<ChachapolyCtx>(aead);
    if keylen != (*ctx).saltlen + CHACHA_KEY_SIZE { return -EINVAL; }
    let n = keylen - (*ctx).saltlen;
    memcpy((*ctx).salt.as_mut_ptr() as *mut _, key.add(n as usize) as *const _, (*ctx).saltlen as usize);
    crypto_skcipher_clear_flags((*ctx).chacha, CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_set_flags((*ctx).chacha, crypto_aead_get_flags(aead) & CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_setkey((*ctx).chacha, key, n)
}

unsafe fn chachapoly_init(tfm: *mut crypto_aead) -> c_int {
    let inst = aead_alg_instance(tfm);
    let ictx = aead_instance_ctx::<ChachapolyInstanceCtx>(inst);
    let ctx = crypto_aead_ctx::<ChachapolyCtx>(tfm);
    let chacha = crypto_spawn_skcipher(&mut (*ictx).chacha);
    if IS_ERR(chacha) { return PTR_ERR(chacha); }
    (*ctx).chacha = chacha; (*ctx).saltlen = (*ictx).saltlen; 0
}
unsafe fn chachapoly_exit(tfm: *mut crypto_aead) { let ctx = crypto_aead_ctx::<ChachapolyCtx>(tfm); crypto_free_skcipher((*ctx).chacha); }
unsafe fn chachapoly_free(inst: *mut aead_instance) { let ctx = aead_instance_ctx::<ChachapolyInstanceCtx>(inst); crypto_drop_skcipher(&mut (*ctx).chacha); kfree(inst as *mut _); }
unsafe fn chachapoly_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr, name: *const c_char, ivsize: c_uint) -> c_int {
    if ivsize > CHACHAPOLY_IV_SIZE { return -EINVAL; }
    let mut mask = 0; let err = crypto_check_attr_type(tb, CRYPTO_ALG_TYPE_AEAD, &mut mask); if err != 0 { return err; }
    let inst = kzalloc(core::mem::size_of::<aead_instance>() + core::mem::size_of::<ChachapolyInstanceCtx>(), GFP_KERNEL) as *mut aead_instance;
    if inst.is_null() { return -ENOMEM; }
    let ctx = aead_instance_ctx::<ChachapolyInstanceCtx>(inst); (*ctx).saltlen = CHACHAPOLY_IV_SIZE - ivsize;
    let err = crypto_grab_skcipher(&mut (*ctx).chacha, aead_crypto_instance(inst), crypto_attr_alg_name(*tb.add(1)), 0, mask);
    if err != 0 { chachapoly_free(inst); return err; }
    aead_register_instance(tmpl, inst)
}
unsafe fn rfc7539_create(t: *mut crypto_template, tb: *mut *mut rtattr) -> c_int { chachapoly_create(t, tb, b"rfc7539\0".as_ptr() as *const c_char, 12) }
unsafe fn rfc7539esp_create(t: *mut crypto_template, tb: *mut *mut rtattr) -> c_int { chachapoly_create(t, tb, b"rfc7539esp\0".as_ptr() as *const c_char, 8) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
