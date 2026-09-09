// SPDX-License-Identifier: GPL-2.0-only
/* AMD Cryptographic Coprocessor (CCP) SHA crypto API support */

// Kernel and CCP dependencies are supplied by the surrounding translation unit.

unsafe fn ccp_sha_complete(async_req: *mut crypto_async_request, ret: c_int) -> c_int {
    let req = ahash_request_cast(async_req);
    let tfm = crypto_ahash_reqtfm(req);
    let rctx = ahash_request_ctx_dma(req);
    let digest_size = crypto_ahash_digestsize(tfm);

    if ret != 0 { goto_e_free!(); }
    if (*rctx).hash_rem != 0 {
        let offset = (*rctx).nbytes - (*rctx).hash_rem;
        scatterwalk_map_and_copy((*rctx).buf.as_mut_ptr(), (*rctx).src,
                                 offset, (*rctx).hash_rem, 0);
        (*rctx).buf_count = (*rctx).hash_rem;
    } else { (*rctx).buf_count = 0; }
    if !(*req).result.is_null() && (*rctx).final_ != 0 {
        memcpy((*req).result, (*rctx).ctx.as_ptr() as *const c_void, digest_size);
    }
e_free:
    sg_free_table(&mut (*rctx).data_sg);
    ret
}

unsafe fn ccp_do_sha_update(req: *mut ahash_request, nbytes: c_uint, final_: c_uint) -> c_int {
    let tfm = crypto_ahash_reqtfm(req);
    let ctx = crypto_ahash_ctx_dma(tfm);
    let rctx = ahash_request_ctx_dma(req);
    let block_size = crypto_tfm_alg_blocksize(crypto_ahash_tfm(tfm));
    let len: u64 = (*rctx).buf_count as u64 + nbytes as u64;
    if final_ == 0 && len <= block_size as u64 {
        scatterwalk_map_and_copy((*rctx).buf.as_mut_ptr().add((*rctx).buf_count as usize),
                                 (*req).src, 0, nbytes, 0);
        (*rctx).buf_count += nbytes;
        return 0;
    }
    (*rctx).src = (*req).src; (*rctx).nbytes = nbytes; (*rctx).final_ = final_;
    (*rctx).hash_rem = if final_ != 0 { 0 } else { (len & (block_size as u64 - 1)) as c_uint };
    (*rctx).hash_cnt = len - (*rctx).hash_rem as u64;
    if final_ == 0 && (*rctx).hash_rem == 0 {
        (*rctx).hash_cnt -= block_size as u64; (*rctx).hash_rem = block_size;
    }
    sg_init_one(&mut (*rctx).ctx_sg, (*rctx).ctx.as_mut_ptr() as *mut c_void, size_of_val(&(*rctx).ctx));
    let mut sg: *mut scatterlist = ptr::null_mut();
    if (*rctx).buf_count != 0 && nbytes != 0 {
        let gfp = if (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP != 0 { GFP_KERNEL } else { GFP_ATOMIC };
        let sg_count = sg_nents((*req).src) + 1;
        let mut ret = sg_alloc_table(&mut (*rctx).data_sg, sg_count, gfp);
        if ret != 0 { return ret; }
        sg_init_one(&mut (*rctx).buf_sg, (*rctx).buf.as_mut_ptr() as *mut c_void, (*rctx).buf_count);
        sg = ccp_crypto_sg_table_add(&mut (*rctx).data_sg, &mut (*rctx).buf_sg);
        if sg.is_null() { ret = -EINVAL; goto_e_free!(); }
        sg = ccp_crypto_sg_table_add(&mut (*rctx).data_sg, (*req).src);
        if sg.is_null() { ret = -EINVAL; goto_e_free!(); }
        sg_mark_end(sg); sg = (*rctx).data_sg.sgl;
    } else if (*rctx).buf_count != 0 {
        sg_init_one(&mut (*rctx).buf_sg, (*rctx).buf.as_mut_ptr() as *mut c_void, (*rctx).buf_count);
        sg = &mut (*rctx).buf_sg;
    } else if nbytes != 0 { sg = (*req).src; }
    (*rctx).msg_bits += (*rctx).hash_cnt << 3;
    memset(&mut (*rctx).cmd as *mut _ as *mut c_void, 0, size_of_val(&(*rctx).cmd));
    INIT_LIST_HEAD(&mut (*rctx).cmd.entry); (*rctx).cmd.engine = CCP_ENGINE_SHA;
    (*rctx).cmd.u.sha.type_ = (*rctx).type; (*rctx).cmd.u.sha.ctx = &mut (*rctx).ctx_sg;
    (*rctx).cmd.u.sha.ctx_len = match (*rctx).type { CCP_SHA_TYPE_1 => SHA1_DIGEST_SIZE, CCP_SHA_TYPE_224 => SHA224_DIGEST_SIZE, CCP_SHA_TYPE_256 => SHA256_DIGEST_SIZE, CCP_SHA_TYPE_384 => SHA384_DIGEST_SIZE, CCP_SHA_TYPE_512 => SHA512_DIGEST_SIZE, _ => 0 };
    (*rctx).cmd.u.sha.src = sg; (*rctx).cmd.u.sha.src_len = (*rctx).hash_cnt;
    (*rctx).cmd.u.sha.opad = if (*ctx).u.sha.key_len != 0 { &mut (*ctx).u.sha.opad_sg } else { ptr::null_mut() };
    (*rctx).cmd.u.sha.opad_len = if (*ctx).u.sha.key_len != 0 { (*ctx).u.sha.opad_count } else { 0 };
    (*rctx).cmd.u.sha.first = (*rctx).first; (*rctx).cmd.u.sha.final_ = (*rctx).final_; (*rctx).cmd.u.sha.msg_bits = (*rctx).msg_bits;
    (*rctx).first = 0;
    ccp_crypto_enqueue_request(&mut (*req).base, &mut (*rctx).cmd)
e_free:
    sg_free_table(&mut (*rctx).data_sg); ret
}

unsafe fn ccp_sha_init(req: *mut ahash_request) -> c_int {
    let tfm = crypto_ahash_reqtfm(req); let ctx = crypto_ahash_ctx_dma(tfm); let rctx = ahash_request_ctx_dma(req);
    let alg = ccp_crypto_ahash_alg(crypto_ahash_tfm(tfm)); let block_size = crypto_tfm_alg_blocksize(crypto_ahash_tfm(tfm));
    memset(rctx as *mut c_void, 0, size_of::<ccp_sha_req_ctx>()); (*rctx).type_ = (*alg).type_; (*rctx).first = 1;
    if (*ctx).u.sha.key_len != 0 { memcpy((*rctx).buf.as_mut_ptr() as *mut c_void, (*ctx).u.sha.ipad.as_ptr() as *const c_void, block_size); (*rctx).buf_count = block_size as c_uint; }
    0
}
unsafe fn ccp_sha_update(req: *mut ahash_request) -> c_int { ccp_do_sha_update(req, (*req).nbytes, 0) }
unsafe fn ccp_sha_final(req: *mut ahash_request) -> c_int { ccp_do_sha_update(req, 0, 1) }
unsafe fn ccp_sha_finup(req: *mut ahash_request) -> c_int { ccp_do_sha_update(req, (*req).nbytes, 1) }
unsafe fn ccp_sha_digest(req: *mut ahash_request) -> c_int { let ret = ccp_sha_init(req); if ret != 0 { ret } else { ccp_sha_finup(req) } }

unsafe fn ccp_sha_export(req: *mut ahash_request, out: *mut c_void) -> c_int {
    let rctx = ahash_request_ctx_dma(req); let mut state: ccp_sha_exp_ctx = zeroed();
    state.type_ = (*rctx).type_; state.msg_bits = (*rctx).msg_bits; state.first = (*rctx).first;
    memcpy(state.ctx.as_mut_ptr() as *mut c_void, (*rctx).ctx.as_ptr() as *const c_void, size_of_val(&state.ctx));
    state.buf_count = (*rctx).buf_count; memcpy(state.buf.as_mut_ptr() as *mut c_void, (*rctx).buf.as_ptr() as *const c_void, size_of_val(&state.buf));
    memcpy(out, &state as *const _ as *const c_void, size_of_val(&state)); 0
}
unsafe fn ccp_sha_import(req: *mut ahash_request, input: *const c_void) -> c_int {
    let rctx = ahash_request_ctx_dma(req); let mut state: ccp_sha_exp_ctx = zeroed();
    memcpy(&mut state as *mut _ as *mut c_void, input, size_of_val(&state)); memset(rctx as *mut c_void, 0, size_of::<ccp_sha_req_ctx>());
    (*rctx).type_ = state.type_; (*rctx).msg_bits = state.msg_bits; (*rctx).first = state.first; (*rctx).buf_count = state.buf_count;
    memcpy((*rctx).ctx.as_mut_ptr() as *mut c_void, state.ctx.as_ptr() as *const c_void, size_of_val(&state.ctx));
    memcpy((*rctx).buf.as_mut_ptr() as *mut c_void, state.buf.as_ptr() as *const c_void, size_of_val(&state.buf)); 0
}
unsafe fn ccp_sha_setkey(tfm: *mut crypto_ahash, key: *const u8, mut key_len: c_uint) -> c_int {
    let ctx = crypto_ahash_ctx_dma(tfm); let shash = (*ctx).u.sha.hmac_tfm; let block_size = crypto_shash_blocksize(shash); let digest_size = crypto_shash_digestsize(shash);
    (*ctx).u.sha.key_len = 0; memset((*ctx).u.sha.key.as_mut_ptr() as *mut c_void, 0, size_of_val(&(*ctx).u.sha.key));
    if key_len > block_size { if crypto_shash_tfm_digest(shash, key, key_len, (*ctx).u.sha.key.as_mut_ptr()) != 0 { return -EINVAL; } key_len = digest_size; } else { memcpy((*ctx).u.sha.key.as_mut_ptr() as *mut c_void, key as *const c_void, key_len as usize); }
    for i in 0..block_size as usize { (*ctx).u.sha.ipad[i] = (*ctx).u.sha.key[i] ^ HMAC_IPAD_VALUE; (*ctx).u.sha.opad[i] = (*ctx).u.sha.key[i] ^ HMAC_OPAD_VALUE; }
    sg_init_one(&mut (*ctx).u.sha.opad_sg, (*ctx).u.sha.opad.as_mut_ptr() as *mut c_void, block_size); (*ctx).u.sha.opad_count = block_size; (*ctx).u.sha.key_len = key_len; 0
}
unsafe fn ccp_sha_cra_init(tfm: *mut crypto_tfm) -> c_int { let ahash = __crypto_ahash_cast(tfm); let ctx = crypto_ahash_ctx_dma(ahash); (*ctx).complete = Some(ccp_sha_complete); (*ctx).u.sha.key_len = 0; crypto_ahash_set_reqsize_dma(ahash, size_of::<ccp_sha_req_ctx>()); 0 }
unsafe fn ccp_sha_cra_exit(_tfm: *mut crypto_tfm) {}
unsafe fn ccp_hmac_sha_cra_init(tfm: *mut crypto_tfm) -> c_int { let ctx = crypto_tfm_ctx_dma(tfm); let alg = ccp_crypto_ahash_alg(tfm); let h = crypto_alloc_shash((*alg).child_alg, 0, 0); if IS_ERR(h) { return PTR_ERR(h); } (*ctx).u.sha.hmac_tfm = h; ccp_sha_cra_init(tfm) }
unsafe fn ccp_hmac_sha_cra_exit(tfm: *mut crypto_tfm) { let ctx = crypto_tfm_ctx_dma(tfm); if !(*ctx).u.sha.hmac_tfm.is_null() { crypto_free_shash((*ctx).u.sha.hmac_tfm); } ccp_sha_cra_exit(tfm) }

// Registration declarations and algorithm definitions depend on the surrounding CCP headers.
#[allow(dead_code)]
static mut sha_algs: [ccp_sha_def; 5] = [
    ccp_sha_def { version: CCP_VERSION(3,0), name: b"sha1\0".as_ptr() as *const c_char, drv_name: b"sha1-ccp\0".as_ptr() as *const c_char, type_: CCP_SHA_TYPE_1, digest_size: SHA1_DIGEST_SIZE, block_size: SHA1_BLOCK_SIZE },
    ccp_sha_def { version: CCP_VERSION(3,0), name: b"sha224\0".as_ptr() as *const c_char, drv_name: b"sha224-ccp\0".as_ptr() as *const c_char, type_: CCP_SHA_TYPE_224, digest_size: SHA224_DIGEST_SIZE, block_size: SHA224_BLOCK_SIZE },
    ccp_sha_def { version: CCP_VERSION(3,0), name: b"sha256\0".as_ptr() as *const c_char, drv_name: b"sha256-ccp\0".as_ptr() as *const c_char, type_: CCP_SHA_TYPE_256, digest_size: SHA256_DIGEST_SIZE, block_size: SHA256_BLOCK_SIZE },
    ccp_sha_def { version: CCP_VERSION(5,0), name: b"sha384\0".as_ptr() as *const c_char, drv_name: b"sha384-ccp\0".as_ptr() as *const c_char, type_: CCP_SHA_TYPE_384, digest_size: SHA384_DIGEST_SIZE, block_size: SHA384_BLOCK_SIZE },
    ccp_sha_def { version: CCP_VERSION(5,0), name: b"sha512\0".as_ptr() as *const c_char, drv_name: b"sha512-ccp\0".as_ptr() as *const c_char, type_: CCP_SHA_TYPE_512, digest_size: SHA512_DIGEST_SIZE, block_size: SHA512_BLOCK_SIZE },
];

unsafe fn ccp_register_hmac_alg(head: *mut list_head, def: *const ccp_sha_def, base_alg: *const ccp_crypto_ahash_alg) -> c_int {
    let ccp_alg = kzalloc_obj::<ccp_crypto_ahash_alg>(); if ccp_alg.is_null() { return -ENOMEM; }
    *ccp_alg = *base_alg; INIT_LIST_HEAD(&mut (*ccp_alg).entry); strscpy((*ccp_alg).child_alg.as_mut_ptr(), (*def).name);
    (*ccp_alg).alg.setkey = Some(ccp_sha_setkey); let base = &mut (*ccp_alg).alg.halg.base;
    snprintf(base.cra_name.as_mut_ptr(), CRYPTO_MAX_ALG_NAME, b"hmac(%s)\0".as_ptr() as *const c_char, (*def).name);
    snprintf(base.cra_driver_name.as_mut_ptr(), CRYPTO_MAX_ALG_NAME, b"hmac-%s\0".as_ptr() as *const c_char, (*def).drv_name);
    base.cra_init = Some(ccp_hmac_sha_cra_init); base.cra_exit = Some(ccp_hmac_sha_cra_exit);
    let ret = crypto_register_ahash(&mut (*ccp_alg).alg); if ret != 0 { kfree(ccp_alg as *mut c_void); return ret; }
    list_add(&mut (*ccp_alg).entry, head); ret
}
unsafe fn ccp_register_sha_alg(head: *mut list_head, def: *const ccp_sha_def) -> c_int {
    let ccp_alg = kzalloc_obj::<ccp_crypto_ahash_alg>(); if ccp_alg.is_null() { return -ENOMEM; }
    INIT_LIST_HEAD(&mut (*ccp_alg).entry); (*ccp_alg).type_ = (*def).type_;
    let alg = &mut (*ccp_alg).alg; alg.init = Some(ccp_sha_init); alg.update = Some(ccp_sha_update); alg.final_ = Some(ccp_sha_final); alg.finup = Some(ccp_sha_finup); alg.digest = Some(ccp_sha_digest); alg.export = Some(ccp_sha_export); alg.import = Some(ccp_sha_import);
    alg.halg.digestsize = (*def).digest_size; alg.halg.statesize = size_of::<ccp_sha_exp_ctx>(); let base = &mut alg.halg.base;
    strscpy(base.cra_name.as_mut_ptr(), (*def).name); strscpy(base.cra_driver_name.as_mut_ptr(), (*def).drv_name); base.cra_blocksize = (*def).block_size; base.cra_ctxsize = size_of::<ccp_ctx>() + crypto_dma_padding(); base.cra_init = Some(ccp_sha_cra_init); base.cra_exit = Some(ccp_sha_cra_exit);
    let ret = crypto_register_ahash(alg); if ret != 0 { kfree(ccp_alg as *mut c_void); return ret; } list_add(&mut (*ccp_alg).entry, head); ccp_register_hmac_alg(head, def, ccp_alg)
}
pub unsafe fn ccp_register_sha_algs(head: *mut list_head) -> c_int {
    let version = ccp_version(); for i in 0..sha_algs.len() { if sha_algs[i].version <= version { let ret = ccp_register_sha_alg(head, &sha_algs[i]); if ret != 0 { return ret; } } } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
