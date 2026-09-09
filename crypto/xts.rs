// SPDX-License-Identifier: GPL-2.0-or-later
/* XTS: as defined in IEEE1619/D16
 * http://grouper.ieee.org/groups/1619/email/pdf00086.pdf
 *
 * Copyright (c) 2007 Rik Snel <rsnel@cube.dyndns.org>
 * Based on ecb.c
 * Copyright (c) 2006 Herbert Xu <herbert@gondor.apana.org.au>
 */

// External Linux crypto/scatterwalk APIs and types are supplied by other files.

#[repr(C)]
struct XtsTfmCtx {
    child: *mut crypto_skcipher,
    tweak: *mut crypto_cipher,
}

#[repr(C)]
struct XtsInstanceCtx {
    spawn: crypto_skcipher_spawn,
    tweak_spawn: crypto_cipher_spawn,
}

#[repr(C)]
struct XtsRequestCtx {
    t: le128,
    tail: *mut scatterlist,
    sg: [scatterlist; 2],
    subreq: skcipher_request,
}

unsafe fn xts_setkey(parent: *mut crypto_skcipher, key: *const u8, mut keylen: c_uint) -> c_int {
    let ctx = crypto_skcipher_ctx(parent) as *mut XtsTfmCtx;
    let err = xts_verify_key(parent, key, keylen);
    if err != 0 { return err; }
    keylen /= 2;

    let tweak = (*ctx).tweak;
    crypto_cipher_clear_flags(tweak, CRYPTO_TFM_REQ_MASK);
    crypto_cipher_set_flags(tweak, crypto_skcipher_get_flags(parent) & CRYPTO_TFM_REQ_MASK);
    let err = crypto_cipher_setkey(tweak, key.add(keylen as usize), keylen);
    if err != 0 { return err; }

    let child = (*ctx).child;
    crypto_skcipher_clear_flags(child, CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_set_flags(child, crypto_skcipher_get_flags(parent) & CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_setkey(child, key, keylen)
}

unsafe fn xts_xor_tweak(req_: *mut skcipher_request, second_pass: bool, enc: bool) -> c_int {
    let rctx = skcipher_request_ctx(req_) as *mut XtsRequestCtx;
    let tfm = crypto_skcipher_reqtfm(req_);
    let cts = ((*req_).cryptlen % XTS_BLOCK_SIZE) != 0;
    let bs = XTS_BLOCK_SIZE as usize;
    let mut w: skcipher_walk = core::mem::zeroed();
    let mut t = (*rctx).t;
    let mut req = req_;
    if second_pass {
        req = &mut (*rctx).subreq;
        skcipher_request_set_tfm(req, tfm);
    }
    let mut err = skcipher_walk_virt(&mut w, req, false);
    while w.nbytes != 0 {
        let mut avail = w.nbytes;
        let mut wsrc = w.src.virt.addr as *const le128;
        let mut wdst = w.dst.virt.addr as *mut le128;
        loop {
            if cts && w.total - w.nbytes + avail < 2 * XTS_BLOCK_SIZE {
                if !enc {
                    if second_pass { (*rctx).t = t; }
                    gf128mul_x_ble(&mut t, &t);
                }
                le128_xor(wdst, &t, wsrc);
                if enc && second_pass { gf128mul_x_ble(&mut (*rctx).t, &t); }
                skcipher_walk_done(&mut w, avail - bs);
                return 0;
            }
            le128_xor(wdst, &t, wsrc);
            wdst = wdst.add(1); wsrc = wsrc.add(1);
            gf128mul_x_ble(&mut t, &t);
            avail -= bs;
            if avail < bs { break; }
        }
        err = skcipher_walk_done(&mut w, avail);
    }
    err
}

unsafe fn xts_xor_tweak_pre(req: *mut skcipher_request, enc: bool) -> c_int { xts_xor_tweak(req, false, enc) }
unsafe fn xts_xor_tweak_post(req: *mut skcipher_request, enc: bool) -> c_int { xts_xor_tweak(req, true, enc) }

unsafe extern "C" fn xts_cts_done(data: *mut c_void, err: c_int) {
    let req = data as *mut skcipher_request;
    if err == 0 {
        let rctx = skcipher_request_ctx(req) as *mut XtsRequestCtx;
        let mut b: le128 = core::mem::zeroed();
        scatterwalk_map_and_copy(&mut b, (*rctx).tail, 0, XTS_BLOCK_SIZE, 0);
        le128_xor(&mut b, &(*rctx).t, &b);
        scatterwalk_map_and_copy(&b, (*rctx).tail, 0, XTS_BLOCK_SIZE, 1);
    }
    skcipher_request_complete(req, err);
}

unsafe fn xts_cts_final(req: *mut skcipher_request, crypt: unsafe fn(*mut skcipher_request) -> c_int) -> c_int {
    let ctx = crypto_skcipher_ctx(crypto_skcipher_reqtfm(req)) as *const XtsTfmCtx;
    let offset = (*req).cryptlen & !(XTS_BLOCK_SIZE - 1);
    let rctx = skcipher_request_ctx(req) as *mut XtsRequestCtx;
    let subreq = &mut (*rctx).subreq;
    let tail = (*req).cryptlen % XTS_BLOCK_SIZE;
    let mut b: [le128; 2] = core::mem::zeroed();
    (*rctx).tail = scatterwalk_ffwd((*rctx).sg.as_mut_ptr(), (*req).dst, offset - XTS_BLOCK_SIZE);
    scatterwalk_map_and_copy(b.as_mut_ptr(), (*rctx).tail, 0, XTS_BLOCK_SIZE, 0);
    b[1] = b[0];
    scatterwalk_map_and_copy(b.as_mut_ptr(), (*req).src, offset, tail, 0);
    le128_xor(b.as_mut_ptr(), &(*rctx).t, b.as_ptr());
    scatterwalk_map_and_copy(b.as_ptr(), (*rctx).tail, 0, XTS_BLOCK_SIZE + tail, 1);
    skcipher_request_set_tfm(subreq, (*ctx).child);
    skcipher_request_set_callback(subreq, (*req).base.flags, Some(xts_cts_done), req as *mut c_void);
    skcipher_request_set_crypt(subreq, (*rctx).tail, (*rctx).tail, XTS_BLOCK_SIZE, core::ptr::null_mut());
    let err = crypt(subreq);
    if err != 0 { return err; }
    scatterwalk_map_and_copy(b.as_mut_ptr(), (*rctx).tail, 0, XTS_BLOCK_SIZE, 0);
    le128_xor(b.as_mut_ptr(), &(*rctx).t, b.as_ptr());
    scatterwalk_map_and_copy(b.as_ptr(), (*rctx).tail, 0, XTS_BLOCK_SIZE, 1);
    0
}

unsafe extern "C" fn xts_encrypt_done(data: *mut c_void, mut err: c_int) {
    let req = data as *mut skcipher_request;
    if err == 0 {
        let rctx = skcipher_request_ctx(req) as *mut XtsRequestCtx;
        (*rctx).subreq.base.flags &= CRYPTO_TFM_REQ_MAY_BACKLOG;
        err = xts_xor_tweak_post(req, true);
        if err == 0 && (*req).cryptlen % XTS_BLOCK_SIZE != 0 {
            err = xts_cts_final(req, crypto_skcipher_encrypt);
            if err == -EINPROGRESS || err == -EBUSY { return; }
        }
    }
    skcipher_request_complete(req, err);
}

unsafe extern "C" fn xts_decrypt_done(data: *mut c_void, mut err: c_int) {
    let req = data as *mut skcipher_request;
    if err == 0 {
        let rctx = skcipher_request_ctx(req) as *mut XtsRequestCtx;
        (*rctx).subreq.base.flags &= CRYPTO_TFM_REQ_MAY_BACKLOG;
        err = xts_xor_tweak_post(req, false);
        if err == 0 && (*req).cryptlen % XTS_BLOCK_SIZE != 0 {
            err = xts_cts_final(req, crypto_skcipher_decrypt);
            if err == -EINPROGRESS || err == -EBUSY { return; }
        }
    }
    skcipher_request_complete(req, err);
}

// Remaining registration glue follows the kernel API declarations supplied externally.
// The function bodies retain the source control flow and external symbol usage.
unsafe fn xts_init_crypt(req: *mut skcipher_request, compl: crypto_completion_t) -> c_int {
    let ctx = crypto_skcipher_ctx(crypto_skcipher_reqtfm(req)) as *const XtsTfmCtx;
    let rctx = skcipher_request_ctx(req) as *mut XtsRequestCtx;
    if (*req).cryptlen < XTS_BLOCK_SIZE { return -EINVAL; }
    skcipher_request_set_tfm(&mut (*rctx).subreq, (*ctx).child);
    skcipher_request_set_callback(&mut (*rctx).subreq, (*req).base.flags, compl, req as *mut c_void);
    skcipher_request_set_crypt(&mut (*rctx).subreq, (*req).dst, (*req).dst, (*req).cryptlen & !(XTS_BLOCK_SIZE - 1), core::ptr::null_mut());
    crypto_cipher_encrypt_one((*ctx).tweak, &mut (*rctx).t as *mut le128 as *mut u8, (*req).iv);
    0
}

unsafe fn xts_encrypt(req: *mut skcipher_request) -> c_int {
    let rctx = skcipher_request_ctx(req) as *mut XtsRequestCtx;
    let subreq = &mut (*rctx).subreq;
    let mut err = xts_init_crypt(req, Some(xts_encrypt_done));
    if err == 0 { err = xts_xor_tweak_pre(req, true); }
    if err == 0 { err = crypto_skcipher_encrypt(subreq); }
    if err == 0 { err = xts_xor_tweak_post(req, true); }
    if err != 0 || (*req).cryptlen % XTS_BLOCK_SIZE == 0 { return err; }
    xts_cts_final(req, crypto_skcipher_encrypt)
}

unsafe fn xts_decrypt(req: *mut skcipher_request) -> c_int {
    let rctx = skcipher_request_ctx(req) as *mut XtsRequestCtx;
    let subreq = &mut (*rctx).subreq;
    let mut err = xts_init_crypt(req, Some(xts_decrypt_done));
    if err == 0 { err = xts_xor_tweak_pre(req, false); }
    if err == 0 { err = crypto_skcipher_decrypt(subreq); }
    if err == 0 { err = xts_xor_tweak_post(req, false); }
    if err != 0 || (*req).cryptlen % XTS_BLOCK_SIZE == 0 { return err; }
    xts_cts_final(req, crypto_skcipher_decrypt)
}

unsafe fn xts_init_tfm(tfm: *mut crypto_skcipher) -> c_int {
    let inst = skcipher_alg_instance(tfm);
    let ictx = skcipher_instance_ctx(inst) as *mut XtsInstanceCtx;
    let ctx = crypto_skcipher_ctx(tfm) as *mut XtsTfmCtx;
    (*ctx).child = crypto_spawn_skcipher(&mut (*ictx).spawn);
    if IS_ERR((*ctx).child) { return PTR_ERR((*ctx).child); }
    (*ctx).tweak = crypto_spawn_cipher(&mut (*ictx).tweak_spawn);
    if IS_ERR((*ctx).tweak) { crypto_free_skcipher((*ctx).child); return PTR_ERR((*ctx).tweak); }
    crypto_skcipher_set_reqsize(tfm, crypto_skcipher_reqsize((*ctx).child) + core::mem::size_of::<XtsRequestCtx>());
    0
}

unsafe fn xts_exit_tfm(tfm: *mut crypto_skcipher) {
    let ctx = crypto_skcipher_ctx(tfm) as *mut XtsTfmCtx;
    crypto_free_skcipher((*ctx).child);
    crypto_free_cipher((*ctx).tweak);
}

unsafe fn xts_free_instance(inst: *mut skcipher_instance) {
    let ctx = skcipher_instance_ctx(inst) as *mut XtsInstanceCtx;
    crypto_drop_skcipher(&mut (*ctx).spawn);
    crypto_drop_cipher(&mut (*ctx).tweak_spawn);
    kfree(inst as *mut c_void);
}

unsafe fn xts_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> c_int {
    let mut mask: u32 = 0;
    let err = crypto_check_attr_type(tb, CRYPTO_ALG_TYPE_SKCIPHER, &mut mask);
    if err != 0 { return err; }
    let cipher_name = crypto_attr_alg_name(*tb.add(1));
    if IS_ERR(cipher_name) { return PTR_ERR(cipher_name); }
    let inst = kzalloc(core::mem::size_of::<skcipher_instance>() + core::mem::size_of::<XtsInstanceCtx>(), GFP_KERNEL) as *mut skcipher_instance;
    if inst.is_null() { return -ENOMEM; }
    let ctx = skcipher_instance_ctx(inst) as *mut XtsInstanceCtx;
    let mut err = crypto_grab_skcipher(&mut (*ctx).spawn, skcipher_crypto_instance(inst), cipher_name, 0, mask);
    if err == -ENOENT && memcmp(cipher_name, b"ecb(\0".as_ptr(), 4) != 0 {
        err = -ENAMETOOLONG;
        let mut name = [0i8; CRYPTO_MAX_ALG_NAME as usize];
        if snprintf(name.as_mut_ptr(), CRYPTO_MAX_ALG_NAME, b"ecb(%s)\0".as_ptr() as *const i8, cipher_name) >= CRYPTO_MAX_ALG_NAME { xts_free_instance(inst); return err; }
        err = crypto_grab_skcipher(&mut (*ctx).spawn, skcipher_crypto_instance(inst), name.as_ptr(), 0, mask);
    }
    if err != 0 { xts_free_instance(inst); return err; }
    let alg = crypto_spawn_skcipher_alg_common(&mut (*ctx).spawn);
    if (*alg).base.cra_blocksize != XTS_BLOCK_SIZE || (*alg).ivsize != 0 { xts_free_instance(inst); return -EINVAL; }
    err = crypto_inst_setname(skcipher_crypto_instance(inst), b"xts\0".as_ptr(), &(*alg).base);
    if err != 0 { xts_free_instance(inst); return err; }
    let mut name = [0i8; CRYPTO_MAX_ALG_NAME as usize];
    if memcmp((*alg).base.cra_name, b"ecb(\0".as_ptr(), 4) != 0 { xts_free_instance(inst); return -EINVAL; }
    let len = strscpy(name.as_mut_ptr(), (*alg).base.cra_name.add(4));
    if len < 2 || name[(len - 1) as usize] != b')' as i8 { xts_free_instance(inst); return -EINVAL; }
    name[(len - 1) as usize] = 0;
    if snprintf((*inst).alg.base.cra_name.as_mut_ptr(), CRYPTO_MAX_ALG_NAME, b"xts(%s)\0".as_ptr() as *const i8, name.as_ptr()) >= CRYPTO_MAX_ALG_NAME { xts_free_instance(inst); return -ENAMETOOLONG; }
    err = crypto_grab_cipher(&mut (*ctx).tweak_spawn, skcipher_crypto_instance(inst), name.as_ptr(), 0, mask);
    if err != 0 { xts_free_instance(inst); return err; }
    (*inst).alg.base.cra_priority = (*alg).base.cra_priority;
    (*inst).alg.base.cra_blocksize = XTS_BLOCK_SIZE;
    (*inst).alg.ivsize = XTS_BLOCK_SIZE;
    (*inst).alg.min_keysize = (*alg).min_keysize * 2;
    (*inst).alg.max_keysize = (*alg).max_keysize * 2;
    (*inst).alg.base.cra_ctxsize = core::mem::size_of::<XtsTfmCtx>();
    (*inst).alg.init = Some(xts_init_tfm); (*inst).alg.exit = Some(xts_exit_tfm);
    (*inst).alg.setkey = Some(xts_setkey); (*inst).alg.encrypt = Some(xts_encrypt); (*inst).alg.decrypt = Some(xts_decrypt);
    (*inst).free = Some(xts_free_instance);
    err = skcipher_register_instance(tmpl, inst);
    if err != 0 { xts_free_instance(inst); }
    err
}

static mut xts_tmpl: crypto_template = crypto_template { name: b"xts\0".as_ptr(), create: Some(xts_create), module: THIS_MODULE };
unsafe fn xts_module_init() -> c_int { crypto_register_template(&mut xts_tmpl) }
unsafe fn xts_module_exit() { crypto_unregister_template(&mut xts_tmpl); }

// module_init(xts_module_init); module_exit(xts_module_exit);
// MODULE_LICENSE("GPL"); MODULE_DESCRIPTION("XTS block cipher mode");
// MODULE_ALIAS_CRYPTO("xts"); MODULE_IMPORT_NS("CRYPTO_INTERNAL"); MODULE_SOFTDEP("pre: ecb");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
