// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Cryptographic Coprocessor (CCP) AES CMAC crypto API support
 *
 * Copyright (C) 2013,2018 Advanced Micro Devices, Inc.
 *
 * Author: Tom Lendacky <thomas.lendacky@amd.com>
 */

// Linux kernel and local header dependencies are supplied by the surrounding translation.

unsafe fn ccp_aes_cmac_complete(
    async_req: *mut crypto_async_request,
    ret: c_int,
) -> c_int {
    let req = ahash_request_cast(async_req);
    let tfm = crypto_ahash_reqtfm(req);
    let rctx = ahash_request_ctx_dma(req);
    let digest_size = crypto_ahash_digestsize(tfm);

    if ret != 0 {
        return ccp_aes_cmac_complete_free(rctx, ret);
    }

    if (*rctx).hash_rem != 0 {
        /* Save remaining data to buffer */
        let offset = (*rctx).nbytes - (*rctx).hash_rem;
        scatterwalk_map_and_copy((*rctx).buf.as_mut_ptr(), (*rctx).src,
                                 offset, (*rctx).hash_rem, 0);
        (*rctx).buf_count = (*rctx).hash_rem;
    } else {
        (*rctx).buf_count = 0;
    }

    /* Update result area if supplied */
    if !(*req).result.is_null() && (*rctx).final_ != 0 {
        memcpy((*req).result, (*rctx).iv.as_ptr(), digest_size);
    }

    ccp_aes_cmac_complete_free(rctx, ret)
}

unsafe fn ccp_aes_cmac_complete_free(rctx: *mut ccp_aes_cmac_req_ctx, ret: c_int) -> c_int {
    sg_free_table(&mut (*rctx).data_sg);
    ret
}

unsafe fn ccp_do_cmac_update(req: *mut ahash_request, nbytes: c_uint, final_: c_uint) -> c_int {
    let tfm = crypto_ahash_reqtfm(req);
    let ctx = crypto_ahash_ctx_dma(tfm);
    let rctx = ahash_request_ctx_dma(req);
    let mut sg: *mut scatterlist = core::ptr::null_mut();
    let mut cmac_key_sg: *mut scatterlist = core::ptr::null_mut();
    let block_size = crypto_tfm_alg_blocksize(crypto_ahash_tfm(tfm));
    let mut need_pad: c_uint;
    let sg_count: c_uint;
    let gfp: gfp_t;
    let len: u64;
    let mut ret: c_int;

    if (*ctx).u.aes.key_len == 0 { return -EINVAL; }
    if nbytes != 0 { (*rctx).null_msg = 0; }
    len = (*rctx).buf_count as u64 + nbytes as u64;

    if final_ == 0 && len <= block_size as u64 {
        scatterwalk_map_and_copy((*rctx).buf.as_mut_ptr().add((*rctx).buf_count as usize),
                                 (*req).src, 0, nbytes, 0);
        (*rctx).buf_count += nbytes;
        return 0;
    }

    (*rctx).src = (*req).src;
    (*rctx).nbytes = nbytes;
    (*rctx).final_ = final_;
    (*rctx).hash_rem = if final_ != 0 { 0 } else { len & (block_size as u64 - 1) } as c_uint;
    (*rctx).hash_cnt = len - (*rctx).hash_rem as u64;
    if final_ == 0 && (*rctx).hash_rem == 0 {
        /* CCP can't do zero length final, so keep some data around */
        (*rctx).hash_cnt -= block_size as u64;
        (*rctx).hash_rem = block_size;
    }
    need_pad = if final_ != 0 && ((*rctx).null_msg != 0 || (len & (block_size as u64 - 1)) != 0) { 1 } else { 0 };

    sg_init_one(&mut (*rctx).iv_sg, (*rctx).iv.as_mut_ptr(), core::mem::size_of_val(&(*rctx).iv));
    sg_count = if nbytes != 0 { sg_nents((*req).src) + 2 } else { 2 };
    gfp = if (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP != 0 { GFP_KERNEL } else { GFP_ATOMIC };
    ret = sg_alloc_table(&mut (*rctx).data_sg, sg_count, gfp);
    if ret != 0 { return ret; }

    if (*rctx).buf_count != 0 {
        sg_init_one(&mut (*rctx).buf_sg, (*rctx).buf.as_mut_ptr(), (*rctx).buf_count);
        sg = ccp_crypto_sg_table_add(&mut (*rctx).data_sg, &mut (*rctx).buf_sg);
        if sg.is_null() { ret = -EINVAL; goto_free(&mut (*rctx).data_sg, ret); }
    }
    if nbytes != 0 {
        sg = ccp_crypto_sg_table_add(&mut (*rctx).data_sg, (*req).src);
        if sg.is_null() { ret = -EINVAL; goto_free(&mut (*rctx).data_sg, ret); }
    }
    if need_pad != 0 {
        let pad_length = block_size - (len & (block_size as u64 - 1)) as c_uint;
        (*rctx).hash_cnt += pad_length as u64;
        memset((*rctx).pad.as_mut_ptr(), 0, core::mem::size_of_val(&(*rctx).pad));
        (*rctx).pad[0] = 0x80;
        sg_init_one(&mut (*rctx).pad_sg, (*rctx).pad.as_mut_ptr(), pad_length);
        sg = ccp_crypto_sg_table_add(&mut (*rctx).data_sg, &mut (*rctx).pad_sg);
        if sg.is_null() { ret = -EINVAL; goto_free(&mut (*rctx).data_sg, ret); }
    }
    if !sg.is_null() { sg_mark_end(sg); sg = (*rctx).data_sg.sgl; }
    if final_ != 0 { cmac_key_sg = if need_pad != 0 { &mut (*ctx).u.aes.k2_sg } else { &mut (*ctx).u.aes.k1_sg }; }

    memset(&mut (*rctx).cmd as *mut _ as *mut c_void, 0, core::mem::size_of_val(&(*rctx).cmd));
    INIT_LIST_HEAD(&mut (*rctx).cmd.entry);
    (*rctx).cmd.engine = CCP_ENGINE_AES;
    (*rctx).cmd.u.aes.type_ = (*ctx).u.aes.type_;
    (*rctx).cmd.u.aes.mode = (*ctx).u.aes.mode;
    (*rctx).cmd.u.aes.action = CCP_AES_ACTION_ENCRYPT;
    (*rctx).cmd.u.aes.key = &mut (*ctx).u.aes.key_sg;
    (*rctx).cmd.u.aes.key_len = (*ctx).u.aes.key_len;
    (*rctx).cmd.u.aes.iv = &mut (*rctx).iv_sg;
    (*rctx).cmd.u.aes.iv_len = AES_BLOCK_SIZE;
    (*rctx).cmd.u.aes.src = sg;
    (*rctx).cmd.u.aes.src_len = (*rctx).hash_cnt;
    (*rctx).cmd.u.aes.dst = core::ptr::null_mut();
    (*rctx).cmd.u.aes.cmac_key = cmac_key_sg;
    (*rctx).cmd.u.aes.cmac_key_len = (*ctx).u.aes.kn_len;
    (*rctx).cmd.u.aes.cmac_final = final_;
    ccp_crypto_enqueue_request(&mut (*req).base, &mut (*rctx).cmd)
}

unsafe fn goto_free(table: &mut scatter_table, ret: c_int) -> c_int {
    sg_free_table(table); ret
}

unsafe fn ccp_aes_cmac_init(req: *mut ahash_request) -> c_int {
    let rctx = ahash_request_ctx_dma(req);
    memset(rctx as *mut c_void, 0, core::mem::size_of::<ccp_aes_cmac_req_ctx>());
    (*rctx).null_msg = 1;
    0
}

unsafe fn ccp_aes_cmac_update(req: *mut ahash_request) -> c_int { ccp_do_cmac_update(req, (*req).nbytes, 0) }
unsafe fn ccp_aes_cmac_final(req: *mut ahash_request) -> c_int { ccp_do_cmac_update(req, 0, 1) }
unsafe fn ccp_aes_cmac_finup(req: *mut ahash_request) -> c_int { ccp_do_cmac_update(req, (*req).nbytes, 1) }

unsafe fn ccp_aes_cmac_digest(req: *mut ahash_request) -> c_int {
    let ret = ccp_aes_cmac_init(req);
    if ret != 0 { return ret; }
    ccp_aes_cmac_finup(req)
}

unsafe fn ccp_aes_cmac_export(req: *mut ahash_request, out: *mut c_void) -> c_int {
    let rctx = ahash_request_ctx_dma(req);
    let mut state: ccp_aes_cmac_exp_ctx = core::mem::zeroed();
    /* Don't let anything leak to 'out' */
    state.null_msg = (*rctx).null_msg;
    memcpy(state.iv.as_mut_ptr() as *mut c_void, (*rctx).iv.as_ptr() as *const c_void, core::mem::size_of_val(&state.iv));
    state.buf_count = (*rctx).buf_count;
    memcpy(state.buf.as_mut_ptr() as *mut c_void, (*rctx).buf.as_ptr() as *const c_void, core::mem::size_of_val(&state.buf));
    /* 'out' may not be aligned so memcpy from local variable */
    memcpy(out, &state as *const _ as *const c_void, core::mem::size_of_val(&state));
    0
}

unsafe fn ccp_aes_cmac_import(req: *mut ahash_request, input: *const c_void) -> c_int {
    let rctx = ahash_request_ctx_dma(req);
    let mut state: ccp_aes_cmac_exp_ctx = core::mem::zeroed();
    /* 'in' may not be aligned so memcpy to local variable */
    memcpy(&mut state as *mut _ as *mut c_void, input, core::mem::size_of_val(&state));
    memset(rctx as *mut c_void, 0, core::mem::size_of::<ccp_aes_cmac_req_ctx>());
    (*rctx).null_msg = state.null_msg;
    memcpy((*rctx).iv.as_mut_ptr() as *mut c_void, state.iv.as_ptr() as *const c_void, core::mem::size_of_val(&(*rctx).iv));
    (*rctx).buf_count = state.buf_count;
    memcpy((*rctx).buf.as_mut_ptr() as *mut c_void, state.buf.as_ptr() as *const c_void, core::mem::size_of_val(&(*rctx).buf));
    0
}

// The remaining setkey, transform initialization, and registration declarations are preserved below.
// External kernel types and helpers are intentionally referenced rather than reimplemented.

unsafe fn ccp_aes_cmac_setkey(tfm: *mut crypto_ahash, key: *const u8, key_len: c_uint) -> c_int {
    let ctx = crypto_ahash_ctx_dma(tfm);
    let alg = ccp_crypto_ahash_alg(crypto_ahash_tfm(tfm));
    let mut k0_hi: u64; let mut k0_lo: u64; let mut k1_hi: u64; let mut k1_lo: u64; let mut k2_hi: u64; let mut k2_lo: u64;
    let rb_hi: u64 = 0x00; let rb_lo: u64 = 0x87;
    let mut aes: aes_enckey = core::mem::zeroed();
    let mut gk: *mut u64;
    match key_len { AES_KEYSIZE_128 => (*ctx).u.aes.type_ = CCP_AES_TYPE_128, AES_KEYSIZE_192 => (*ctx).u.aes.type_ = CCP_AES_TYPE_192, AES_KEYSIZE_256 => (*ctx).u.aes.type_ = CCP_AES_TYPE_256, _ => return -EINVAL }
    (*ctx).u.aes.mode = (*alg).mode; (*ctx).u.aes.key_len = 0;
    let mut ret = aes_prepareenckey(&mut aes, key, key_len); if ret != 0 { return ret; }
    memset((*ctx).u.aes.key.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&(*ctx).u.aes.key));
    aes_encrypt(&mut aes, (*ctx).u.aes.key.as_mut_ptr(), (*ctx).u.aes.key.as_mut_ptr());
    memzero_explicit(&mut aes as *mut _ as *mut c_void, core::mem::size_of_val(&aes));
    k0_hi = be64_to_cpu(*( (*ctx).u.aes.key.as_ptr() as *const u64)); k0_lo = be64_to_cpu(*( (*ctx).u.aes.key.as_ptr().add(8) as *const u64));
    k1_hi = (k0_hi << 1) | (k0_lo >> 63); k1_lo = k0_lo << 1; if (*ctx).u.aes.key[0] & 0x80 != 0 { k1_hi ^= rb_hi; k1_lo ^= rb_lo; }
    gk = (*ctx).u.aes.k1.as_mut_ptr() as *mut u64; *gk = cpu_to_be64(k1_hi); *gk.add(1) = cpu_to_be64(k1_lo);
    k2_hi = (k1_hi << 1) | (k1_lo >> 63); k2_lo = k1_lo << 1; if (*ctx).u.aes.k1[0] & 0x80 != 0 { k2_hi ^= rb_hi; k2_lo ^= rb_lo; }
    gk = (*ctx).u.aes.k2.as_mut_ptr() as *mut u64; *gk = cpu_to_be64(k2_hi); *gk.add(1) = cpu_to_be64(k2_lo);
    (*ctx).u.aes.kn_len = core::mem::size_of_val(&(*ctx).u.aes.k1) as c_uint;
    sg_init_one(&mut (*ctx).u.aes.k1_sg, (*ctx).u.aes.k1.as_mut_ptr(), (*ctx).u.aes.kn_len); sg_init_one(&mut (*ctx).u.aes.k2_sg, (*ctx).u.aes.k2.as_mut_ptr(), (*ctx).u.aes.kn_len);
    memset((*ctx).u.aes.key.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&(*ctx).u.aes.key)); memcpy((*ctx).u.aes.key.as_mut_ptr() as *mut c_void, key as *const c_void, key_len); (*ctx).u.aes.key_len = key_len; sg_init_one(&mut (*ctx).u.aes.key_sg, (*ctx).u.aes.key.as_mut_ptr(), key_len);
    ret
}

unsafe fn ccp_aes_cmac_cra_init(tfm: *mut crypto_tfm) -> c_int {
    let ctx = crypto_tfm_ctx_dma(tfm); let ahash = __crypto_ahash_cast(tfm); (*ctx).complete = Some(ccp_aes_cmac_complete); (*ctx).u.aes.key_len = 0; crypto_ahash_set_reqsize_dma(ahash, core::mem::size_of::<ccp_aes_cmac_req_ctx>()); 0
}

pub unsafe fn ccp_register_aes_cmac_algs(head: *mut list_head) -> c_int {
    let ccp_alg = kzalloc_obj::<ccp_crypto_ahash_alg>(); if ccp_alg.is_null() { return -ENOMEM; }
    INIT_LIST_HEAD(&mut (*ccp_alg).entry); (*ccp_alg).mode = CCP_AES_MODE_CMAC;
    let alg = &mut (*ccp_alg).alg; alg.init = Some(ccp_aes_cmac_init); alg.update = Some(ccp_aes_cmac_update); alg.final_ = Some(ccp_aes_cmac_final); alg.finup = Some(ccp_aes_cmac_finup); alg.digest = Some(ccp_aes_cmac_digest); alg.export = Some(ccp_aes_cmac_export); alg.import = Some(ccp_aes_cmac_import); alg.setkey = Some(ccp_aes_cmac_setkey);
    let halg = &mut alg.halg; halg.digestsize = AES_BLOCK_SIZE; halg.statesize = core::mem::size_of::<ccp_aes_cmac_exp_ctx>();
    let base = &mut halg.base; snprintf(base.cra_name.as_mut_ptr(), CRYPTO_MAX_ALG_NAME, c"cmac(aes)".as_ptr()); snprintf(base.cra_driver_name.as_mut_ptr(), CRYPTO_MAX_ALG_NAME, c"cmac-aes-ccp".as_ptr()); base.cra_flags = CRYPTO_ALG_ASYNC | CRYPTO_ALG_ALLOCATES_MEMORY | CRYPTO_ALG_KERN_DRIVER_ONLY | CRYPTO_ALG_NEED_FALLBACK; base.cra_blocksize = AES_BLOCK_SIZE; base.cra_ctxsize = core::mem::size_of::<ccp_ctx>() + crypto_dma_padding(); base.cra_priority = CCP_CRA_PRIORITY; base.cra_init = Some(ccp_aes_cmac_cra_init); base.cra_module = THIS_MODULE;
    let ret = crypto_register_ahash(alg); if ret != 0 { pr_err(base.cra_name.as_ptr(), ret); kfree(ccp_alg as *mut c_void); return ret; }
    list_add(&mut (*ccp_alg).entry, head); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
