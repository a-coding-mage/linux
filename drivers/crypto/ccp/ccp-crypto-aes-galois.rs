// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Cryptographic Coprocessor (CCP) AES GCM crypto API support
 *
 * Copyright (C) 2016,2017 Advanced Micro Devices, Inc.
 *
 * Author: Gary R Hook <gary.hook@amd.com>
 */

// Linux kernel and CCP dependencies are supplied by the surrounding crate.

unsafe fn ccp_aes_gcm_complete(
    _async_req: *mut crypto_async_request,
    ret: i32,
) -> i32 {
    ret
}

unsafe fn ccp_aes_gcm_setkey(
    tfm: *mut crypto_aead,
    key: *const u8,
    key_len: u32,
) -> i32 {
    let ctx = crypto_aead_ctx_dma(tfm);

    match key_len {
        AES_KEYSIZE_128 => (*ctx).u.aes.type_ = CCP_AES_TYPE_128,
        AES_KEYSIZE_192 => (*ctx).u.aes.type_ = CCP_AES_TYPE_192,
        AES_KEYSIZE_256 => (*ctx).u.aes.type_ = CCP_AES_TYPE_256,
        _ => return -EINVAL,
    }

    (*ctx).u.aes.mode = CCP_AES_MODE_GCM;
    (*ctx).u.aes.key_len = key_len;

    memcpy((*ctx).u.aes.key.as_mut_ptr(), key, key_len as usize);
    sg_init_one(&mut (*ctx).u.aes.key_sg, (*ctx).u.aes.key.as_mut_ptr(), key_len);

    0
}

unsafe fn ccp_aes_gcm_setauthsize(
    _tfm: *mut crypto_aead,
    authsize: u32,
) -> i32 {
    match authsize {
        16 | 15 | 14 | 13 | 12 | 8 | 4 => {}
        _ => return -EINVAL,
    }

    0
}

unsafe fn ccp_aes_gcm_crypt(req: *mut aead_request, encrypt: bool) -> i32 {
    let tfm = crypto_aead_reqtfm(req);
    let ctx = crypto_aead_ctx_dma(tfm);
    let rctx = aead_request_ctx_dma(req);
    let mut iv_sg: *mut scatterlist = core::ptr::null_mut();
    let mut iv_len: u32 = 0;

    if (*ctx).u.aes.key_len == 0 {
        return -EINVAL;
    }

    if (*ctx).u.aes.mode != CCP_AES_MODE_GCM {
        return -EINVAL;
    }

    if (*req).iv.is_null() {
        return -EINVAL;
    }

    /*
     * 5 parts:
     *   plaintext/ciphertext input
     *   AAD
     *   key
     *   IV
     *   Destination+tag buffer
     */

    /* Prepare the IV: 12 bytes + an integer (counter) */
    memcpy((*rctx).iv.as_mut_ptr(), (*req).iv, GCM_AES_IV_SIZE as usize);
    for i in 0..3 {
        (*rctx).iv[i + GCM_AES_IV_SIZE as usize] = 0;
    }
    (*rctx).iv[AES_BLOCK_SIZE as usize - 1] = 1;

    /* Set up a scatterlist for the IV */
    iv_sg = &mut (*rctx).iv_sg;
    iv_len = AES_BLOCK_SIZE;
    sg_init_one(iv_sg, (*rctx).iv.as_mut_ptr(), iv_len);

    /* The AAD + plaintext are concatenated in the src buffer */
    memset(
        &mut (*rctx).cmd as *mut _ as *mut u8,
        0,
        core::mem::size_of_val(&(*rctx).cmd),
    );
    INIT_LIST_HEAD(&mut (*rctx).cmd.entry);
    (*rctx).cmd.engine = CCP_ENGINE_AES;
    (*rctx).cmd.u.aes.authsize = crypto_aead_authsize(tfm);
    (*rctx).cmd.u.aes.type_ = (*ctx).u.aes.type_;
    (*rctx).cmd.u.aes.mode = (*ctx).u.aes.mode;
    (*rctx).cmd.u.aes.action = encrypt;
    (*rctx).cmd.u.aes.key = &mut (*ctx).u.aes.key_sg;
    (*rctx).cmd.u.aes.key_len = (*ctx).u.aes.key_len;
    (*rctx).cmd.u.aes.iv = iv_sg;
    (*rctx).cmd.u.aes.iv_len = iv_len;
    (*rctx).cmd.u.aes.src = (*req).src;
    (*rctx).cmd.u.aes.src_len = (*req).cryptlen;
    (*rctx).cmd.u.aes.aad_len = (*req).assoclen;

    /* The cipher text + the tag are in the dst buffer */
    (*rctx).cmd.u.aes.dst = (*req).dst;

    ccp_crypto_enqueue_request(&mut (*req).base, &mut (*rctx).cmd)
}

unsafe fn ccp_aes_gcm_encrypt(req: *mut aead_request) -> i32 {
    ccp_aes_gcm_crypt(req, CCP_AES_ACTION_ENCRYPT)
}

unsafe fn ccp_aes_gcm_decrypt(req: *mut aead_request) -> i32 {
    ccp_aes_gcm_crypt(req, CCP_AES_ACTION_DECRYPT)
}

unsafe fn ccp_aes_gcm_cra_init(tfm: *mut crypto_aead) -> i32 {
    let ctx = crypto_aead_ctx_dma(tfm);

    (*ctx).complete = Some(ccp_aes_gcm_complete);
    (*ctx).u.aes.key_len = 0;

    crypto_aead_set_reqsize_dma(tfm, core::mem::size_of::<ccp_aes_req_ctx>());

    0
}

unsafe fn ccp_aes_gcm_cra_exit(_tfm: *mut crypto_tfm) {}

static mut CCP_AES_GCM_DEFAULTS: aead_alg = aead_alg {
    setkey: Some(ccp_aes_gcm_setkey),
    setauthsize: Some(ccp_aes_gcm_setauthsize),
    encrypt: Some(ccp_aes_gcm_encrypt),
    decrypt: Some(ccp_aes_gcm_decrypt),
    init: Some(ccp_aes_gcm_cra_init),
    ivsize: GCM_AES_IV_SIZE,
    maxauthsize: AES_BLOCK_SIZE,
    base: crypto_alg {
        cra_flags: CRYPTO_ALG_ASYNC |
            CRYPTO_ALG_ALLOCATES_MEMORY |
            CRYPTO_ALG_KERN_DRIVER_ONLY |
            CRYPTO_ALG_NEED_FALLBACK,
        cra_blocksize: AES_BLOCK_SIZE,
        cra_ctxsize: core::mem::size_of::<ccp_ctx>() + CRYPTO_DMA_PADDING,
        cra_priority: CCP_CRA_PRIORITY,
        cra_exit: Some(ccp_aes_gcm_cra_exit),
        cra_module: THIS_MODULE,
        ..unsafe { core::mem::zeroed() }
    },
};

struct ccp_aes_aead_def {
    mode: ccp_aes_mode,
    version: u32,
    name: *const core::ffi::c_char,
    driver_name: *const core::ffi::c_char,
    blocksize: u32,
    ivsize: u32,
    alg_defaults: *mut aead_alg,
}

static mut AES_AEAD_ALGS: [ccp_aes_aead_def; 1] = [ccp_aes_aead_def {
    mode: CCP_AES_MODE_GHASH,
    version: CCP_VERSION(5, 0),
    name: c"gcm(aes)".as_ptr(),
    driver_name: c"gcm-aes-ccp".as_ptr(),
    blocksize: 1,
    ivsize: AES_BLOCK_SIZE,
    alg_defaults: core::ptr::addr_of_mut!(CCP_AES_GCM_DEFAULTS),
}];

unsafe fn ccp_register_aes_aead(
    head: *mut list_head,
    def: *const ccp_aes_aead_def,
) -> i32 {
    let ccp_aead = kzalloc_obj::<ccp_crypto_aead>();
    if ccp_aead.is_null() {
        return -ENOMEM;
    }

    INIT_LIST_HEAD(&mut (*ccp_aead).entry);
    (*ccp_aead).mode = (*def).mode;

    /* Copy the defaults and override as necessary */
    let alg = &mut (*ccp_aead).alg;
    *alg = *(*def).alg_defaults;
    strscpy(alg.base.cra_name.as_mut_ptr(), (*def).name);
    strscpy(alg.base.cra_driver_name.as_mut_ptr(), (*def).driver_name);
    alg.base.cra_blocksize = (*def).blocksize;

    let ret = crypto_register_aead(alg);
    if ret != 0 {
        pr_err("%s aead algorithm registration error (%d)\n", alg.base.cra_name.as_ptr(), ret);
        kfree(ccp_aead);
        return ret;
    }

    list_add(&mut (*ccp_aead).entry, head);
    0
}

pub unsafe fn ccp_register_aes_aeads(head: *mut list_head) -> i32 {
    let ccpversion = ccp_version();

    for i in 0..AES_AEAD_ALGS.len() {
        if AES_AEAD_ALGS[i].version > ccpversion {
            continue;
        }
        let ret = ccp_register_aes_aead(head, &AES_AEAD_ALGS[i]);
        if ret != 0 {
            return ret;
        }
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
