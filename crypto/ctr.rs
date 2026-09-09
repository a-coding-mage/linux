// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * CTR: Counter mode
 *
 * (C) Copyright IBM Corp. 2007 - Joy Latten <latten@us.ibm.com>
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
struct CryptoRfc3686Ctx {
    child: *mut crypto_skcipher,
    nonce: [u8; CTR_RFC3686_NONCE_SIZE],
}

#[repr(C)]
struct CryptoRfc3686ReqCtx {
    iv: [u8; CTR_RFC3686_BLOCK_SIZE],
    subreq: skcipher_request,
}

unsafe fn crypto_ctr_crypt_final(walk: *mut skcipher_walk, tfm: *mut crypto_cipher) {
    let bsize = crypto_cipher_blocksize(tfm);
    let alignmask = crypto_cipher_alignmask(tfm);
    let ctrblk = (*walk).iv;
    let mut tmp = [0u8; MAX_CIPHER_BLOCKSIZE + MAX_CIPHER_ALIGNMASK];
    let keystream = ptr_align(tmp.as_mut_ptr(), alignmask + 1);
    let src = (*walk).src.virt.addr;
    let dst = (*walk).dst.virt.addr;
    let nbytes = (*walk).nbytes;

    crypto_cipher_encrypt_one(tfm, keystream, ctrblk);
    crypto_xor_cpy(dst, keystream, src, nbytes);
    crypto_inc(ctrblk, bsize);
}

unsafe fn crypto_ctr_crypt_segment(walk: *mut skcipher_walk, tfm: *mut crypto_cipher) -> c_int {
    let fn_: unsafe extern "C" fn(*mut crypto_tfm, *mut u8, *const u8) =
        (*crypto_cipher_alg(tfm)).cia_encrypt;
    let bsize = crypto_cipher_blocksize(tfm);
    let mut ctrblk = (*walk).iv;
    let mut src = (*walk).src.virt.addr;
    let mut dst = (*walk).dst.virt.addr;
    let mut nbytes = (*walk).nbytes;

    loop {
        fn_(crypto_cipher_tfm(tfm), dst, ctrblk);
        crypto_xor(dst, src, bsize);
        crypto_inc(ctrblk, bsize);
        src = src.add(bsize as usize);
        dst = dst.add(bsize as usize);
        nbytes -= bsize;
        if nbytes < bsize { break; }
    }
    nbytes as c_int
}

unsafe fn crypto_ctr_crypt_inplace(walk: *mut skcipher_walk, tfm: *mut crypto_cipher) -> c_int {
    let fn_: unsafe extern "C" fn(*mut crypto_tfm, *mut u8, *const u8) =
        (*crypto_cipher_alg(tfm)).cia_encrypt;
    let bsize = crypto_cipher_blocksize(tfm);
    let alignmask = crypto_cipher_alignmask(tfm);
    let mut nbytes = (*walk).nbytes;
    let mut dst = (*walk).dst.virt.addr;
    let ctrblk = (*walk).iv;
    let mut tmp = [0u8; MAX_CIPHER_BLOCKSIZE + MAX_CIPHER_ALIGNMASK];
    let keystream = ptr_align(tmp.as_mut_ptr(), alignmask + 1);

    loop {
        fn_(crypto_cipher_tfm(tfm), keystream, ctrblk);
        crypto_xor(dst, keystream, bsize);
        crypto_inc(ctrblk, bsize);
        dst = dst.add(bsize as usize);
        nbytes -= bsize;
        if nbytes < bsize { break; }
    }
    nbytes as c_int
}

unsafe fn crypto_ctr_crypt(req: *mut skcipher_request) -> c_int {
    let tfm = crypto_skcipher_reqtfm(req);
    let cipher = skcipher_cipher_simple(tfm);
    let bsize = crypto_cipher_blocksize(cipher);
    let mut walk: skcipher_walk = core::mem::zeroed();
    let mut err = skcipher_walk_virt(&mut walk, req, false);

    while walk.nbytes >= bsize {
        let nbytes = if walk.src.virt.addr == walk.dst.virt.addr {
            crypto_ctr_crypt_inplace(&mut walk, cipher)
        } else {
            crypto_ctr_crypt_segment(&mut walk, cipher)
        };
        err = skcipher_walk_done(&mut walk, nbytes as _);
    }
    if walk.nbytes != 0 {
        crypto_ctr_crypt_final(&mut walk, cipher);
        err = skcipher_walk_done(&mut walk, 0);
    }
    err
}

unsafe fn crypto_ctr_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> c_int {
    let inst = skcipher_alloc_instance_simple(tmpl, tb);
    if is_err(inst as _) { return ptr_err(inst as _); }
    let alg = skcipher_ialg_simple(inst);
    let mut err = -EINVAL;
    if (*alg).cra_blocksize < 4 { goto_free!(inst); }
    if (*alg).cra_blocksize % 4 != 0 { goto_free!(inst); }
    (*inst).alg.base.cra_blocksize = 1;
    (*inst).alg.chunksize = (*alg).cra_blocksize;
    (*inst).alg.encrypt = Some(crypto_ctr_crypt);
    (*inst).alg.decrypt = Some(crypto_ctr_crypt);
    err = skcipher_register_instance(tmpl, inst);
    if err != 0 { (*inst).free(inst); }
    return err;
    macro_rules! goto_free { ($x:expr) => {{ ($x).free($x); return err; }} }
}

unsafe fn crypto_rfc3686_setkey(parent: *mut crypto_skcipher, key: *const u8, mut keylen: u32) -> c_int {
    let ctx = crypto_skcipher_ctx(parent) as *mut CryptoRfc3686Ctx;
    let child = (*ctx).child;
    if keylen < CTR_RFC3686_NONCE_SIZE as u32 { return -EINVAL; }
    core::ptr::copy_nonoverlapping(key.add((keylen as usize) - CTR_RFC3686_NONCE_SIZE), (*ctx).nonce.as_mut_ptr(), CTR_RFC3686_NONCE_SIZE);
    keylen -= CTR_RFC3686_NONCE_SIZE as u32;
    crypto_skcipher_clear_flags(child, CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_set_flags(child, crypto_skcipher_get_flags(parent) & CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_setkey(child, key, keylen)
}

unsafe fn crypto_rfc3686_crypt(req: *mut skcipher_request) -> c_int {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm) as *mut CryptoRfc3686Ctx;
    let rctx = ptr_align(skcipher_request_ctx(req) as *mut u8, crypto_skcipher_alignmask(tfm) + 1) as *mut CryptoRfc3686ReqCtx;
    let subreq = &mut (*rctx).subreq;
    let iv = (*rctx).iv.as_mut_ptr();
    core::ptr::copy_nonoverlapping((*ctx).nonce.as_ptr(), iv, CTR_RFC3686_NONCE_SIZE);
    core::ptr::copy_nonoverlapping((*req).iv, iv.add(CTR_RFC3686_NONCE_SIZE), CTR_RFC3686_IV_SIZE);
    *(iv.add(CTR_RFC3686_NONCE_SIZE + CTR_RFC3686_IV_SIZE) as *mut u32) = cpu_to_be32(1);
    skcipher_request_set_tfm(subreq, (*ctx).child);
    skcipher_request_set_callback(subreq, (*req).base.flags, (*req).base.complete, (*req).base.data);
    skcipher_request_set_crypt(subreq, (*req).src, (*req).dst, (*req).cryptlen, iv);
    crypto_skcipher_encrypt(subreq)
}

unsafe fn crypto_rfc3686_init_tfm(tfm: *mut crypto_skcipher) -> c_int {
    let inst = skcipher_alg_instance(tfm);
    let spawn = skcipher_instance_ctx(inst);
    let ctx = crypto_skcipher_ctx(tfm) as *mut CryptoRfc3686Ctx;
    let cipher = crypto_spawn_skcipher(spawn);
    if is_err(cipher as _) { return ptr_err(cipher as _); }
    (*ctx).child = cipher;
    let mut align = crypto_skcipher_alignmask(tfm);
    align &= !(crypto_tfm_ctx_alignment() - 1);
    let reqsize = align + core::mem::size_of::<CryptoRfc3686ReqCtx>() as u64 + crypto_skcipher_reqsize(cipher) as u64;
    crypto_skcipher_set_reqsize(tfm, reqsize as _);
    0
}
unsafe fn crypto_rfc3686_exit_tfm(tfm: *mut crypto_skcipher) { let ctx = crypto_skcipher_ctx(tfm) as *mut CryptoRfc3686Ctx; crypto_free_skcipher((*ctx).child); }
unsafe fn crypto_rfc3686_free(inst: *mut skcipher_instance) { crypto_drop_skcipher(skcipher_instance_ctx(inst)); kfree(inst as _); }

unsafe fn crypto_rfc3686_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> c_int {
    let mut mask = 0u32;
    let mut err = crypto_check_attr_type(tb, CRYPTO_ALG_TYPE_SKCIPHER, &mut mask);
    if err != 0 { return err; }
    let inst = kzalloc(core::mem::size_of::<skcipher_instance>() + core::mem::size_of::<crypto_skcipher_spawn>(), GFP_KERNEL);
    if inst.is_null() { return -ENOMEM; }
    let spawn = skcipher_instance_ctx(inst as *mut skcipher_instance);
    err = crypto_grab_skcipher(spawn, skcipher_crypto_instance(inst as *mut skcipher_instance), crypto_attr_alg_name(*tb.add(1)), 0, mask);
    if err != 0 { crypto_rfc3686_free(inst as *mut skcipher_instance); return err; }
    let alg = crypto_spawn_skcipher_alg_common(spawn);
    if (*alg).ivsize != CTR_RFC3686_BLOCK_SIZE || (*alg).base.cra_blocksize != 1 { crypto_rfc3686_free(inst as *mut skcipher_instance); return -EINVAL; }
    (*inst.cast::<skcipher_instance>()).alg.base.cra_blocksize = 1;
    (*inst.cast::<skcipher_instance>()).alg.ivsize = CTR_RFC3686_IV_SIZE;
    (*inst.cast::<skcipher_instance>()).alg.min_keysize = (*alg).min_keysize + CTR_RFC3686_NONCE_SIZE;
    (*inst.cast::<skcipher_instance>()).alg.max_keysize = (*alg).max_keysize + CTR_RFC3686_NONCE_SIZE;
    (*inst.cast::<skcipher_instance>()).alg.setkey = Some(crypto_rfc3686_setkey);
    (*inst.cast::<skcipher_instance>()).alg.encrypt = Some(crypto_rfc3686_crypt);
    (*inst.cast::<skcipher_instance>()).alg.decrypt = Some(crypto_rfc3686_crypt);
    (*inst.cast::<skcipher_instance>()).alg.base.cra_ctxsize = core::mem::size_of::<CryptoRfc3686Ctx>();
    (*inst.cast::<skcipher_instance>()).alg.init = Some(crypto_rfc3686_init_tfm);
    (*inst.cast::<skcipher_instance>()).alg.exit = Some(crypto_rfc3686_exit_tfm);
    (*inst.cast::<skcipher_instance>()).free = crypto_rfc3686_free;
    err = skcipher_register_instance(tmpl, inst as *mut skcipher_instance);
    if err != 0 { crypto_rfc3686_free(inst as *mut skcipher_instance); }
    err
}

// The remaining template-registration glue is represented by the corresponding kernel declarations.
static mut CRYPTO_CTR_TMPLS: [crypto_template; 2] = [
    crypto_template { name: "ctr", create: Some(crypto_ctr_create), module: THIS_MODULE },
    crypto_template { name: "rfc3686", create: Some(crypto_rfc3686_create), module: THIS_MODULE },
];

unsafe extern "C" fn crypto_ctr_module_init() -> c_int { crypto_register_templates(CRYPTO_CTR_TMPLS.as_mut_ptr(), 2) }
unsafe extern "C" fn crypto_ctr_module_exit() { crypto_unregister_templates(CRYPTO_CTR_TMPLS.as_mut_ptr(), 2); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
