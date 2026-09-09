// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Cryptographic Coprocessor (CCP) DES3 crypto API support
 *
 * Copyright (C) 2016,2017 Advanced Micro Devices, Inc.
 *
 * Author: Gary R Hook <ghook@amd.com>
 */

// C dependencies supplied by the surrounding kernel/CCP translation unit.

unsafe fn ccp_des3_complete(async_req: *mut crypto_async_request, ret: i32) -> i32 {
    let req = skcipher_request_cast(async_req);
    let ctx = crypto_skcipher_ctx_dma(crypto_skcipher_reqtfm(req));
    let rctx = skcipher_request_ctx_dma(req);

    if ret != 0 {
        return ret;
    }

    if (*ctx).u.des3.mode != CCP_DES3_MODE_ECB {
        memcpy((*req).iv, (*rctx).iv.as_ptr(), DES3_EDE_BLOCK_SIZE);
    }

    0
}

unsafe fn ccp_des3_setkey(
    tfm: *mut crypto_skcipher,
    key: *const u8,
    key_len: u32,
) -> i32 {
    let alg = ccp_crypto_skcipher_alg(tfm);
    let ctx = crypto_skcipher_ctx_dma(tfm);
    let err = verify_skcipher_des3_key(tfm, key);

    if err != 0 {
        return err;
    }

    /* It's not clear that there is any support for a keysize of 112.
     * If needed, the caller should make K1 == K3
     */
    (*ctx).u.des3.type_ = CCP_DES3_TYPE_168;
    (*ctx).u.des3.mode = (*alg).mode;
    (*ctx).u.des3.key_len = key_len;

    memcpy((*ctx).u.des3.key.as_mut_ptr(), key, key_len as usize);
    sg_init_one(&mut (*ctx).u.des3.key_sg, (*ctx).u.des3.key.as_mut_ptr(), key_len);

    0
}

unsafe fn ccp_des3_crypt(req: *mut skcipher_request, encrypt: bool) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx_dma(tfm);
    let rctx = skcipher_request_ctx_dma(req);
    let mut iv_sg: *mut scatterlist = core::ptr::null_mut();
    let mut iv_len: u32 = 0;

    if (*ctx).u.des3.key_len == 0 {
        return -EINVAL;
    }

    if (((*ctx).u.des3.mode == CCP_DES3_MODE_ECB)
        || ((*ctx).u.des3.mode == CCP_DES3_MODE_CBC))
        && ((*req).cryptlen & (DES3_EDE_BLOCK_SIZE - 1)) != 0
    {
        return -EINVAL;
    }

    if (*ctx).u.des3.mode != CCP_DES3_MODE_ECB {
        if (*req).iv.is_null() {
            return -EINVAL;
        }

        memcpy((*rctx).iv.as_mut_ptr(), (*req).iv, DES3_EDE_BLOCK_SIZE);
        iv_sg = &mut (*rctx).iv_sg;
        iv_len = DES3_EDE_BLOCK_SIZE;
        sg_init_one(iv_sg, (*rctx).iv.as_mut_ptr(), iv_len);
    }

    memset(&mut (*rctx).cmd, 0, core::mem::size_of::<ccp_cmd>());
    INIT_LIST_HEAD(&mut (*rctx).cmd.entry);
    (*rctx).cmd.engine = CCP_ENGINE_DES3;
    (*rctx).cmd.u.des3.type_ = (*ctx).u.des3.type_;
    (*rctx).cmd.u.des3.mode = (*ctx).u.des3.mode;
    (*rctx).cmd.u.des3.action = if encrypt {
        CCP_DES3_ACTION_ENCRYPT
    } else {
        CCP_DES3_ACTION_DECRYPT
    };
    (*rctx).cmd.u.des3.key = &mut (*ctx).u.des3.key_sg;
    (*rctx).cmd.u.des3.key_len = (*ctx).u.des3.key_len;
    (*rctx).cmd.u.des3.iv = iv_sg;
    (*rctx).cmd.u.des3.iv_len = iv_len;
    (*rctx).cmd.u.des3.src = (*req).src;
    (*rctx).cmd.u.des3.src_len = (*req).cryptlen;
    (*rctx).cmd.u.des3.dst = (*req).dst;

    ccp_crypto_enqueue_request(&mut (*req).base, &mut (*rctx).cmd)
}

unsafe fn ccp_des3_encrypt(req: *mut skcipher_request) -> i32 {
    ccp_des3_crypt(req, true)
}

unsafe fn ccp_des3_decrypt(req: *mut skcipher_request) -> i32 {
    ccp_des3_crypt(req, false)
}

unsafe fn ccp_des3_init_tfm(tfm: *mut crypto_skcipher) -> i32 {
    let ctx = crypto_skcipher_ctx_dma(tfm);

    (*ctx).complete = Some(ccp_des3_complete);
    (*ctx).u.des3.key_len = 0;

    crypto_skcipher_set_reqsize_dma(tfm, core::mem::size_of::<ccp_des3_req_ctx>());

    0
}

static mut CCP_DES3_DEFAULTS: skcipher_alg = skcipher_alg {
    setkey: Some(ccp_des3_setkey),
    encrypt: Some(ccp_des3_encrypt),
    decrypt: Some(ccp_des3_decrypt),
    min_keysize: DES3_EDE_KEY_SIZE,
    max_keysize: DES3_EDE_KEY_SIZE,
    init: Some(ccp_des3_init_tfm),
    base: crypto_alg {
        cra_flags: CRYPTO_ALG_ASYNC | CRYPTO_ALG_ALLOCATES_MEMORY |
            CRYPTO_ALG_KERN_DRIVER_ONLY | CRYPTO_ALG_NEED_FALLBACK,
        cra_blocksize: DES3_EDE_BLOCK_SIZE,
        cra_ctxsize: core::mem::size_of::<ccp_ctx>() + CRYPTO_DMA_PADDING,
        cra_priority: CCP_CRA_PRIORITY,
        cra_module: THIS_MODULE,
        ..unsafe { core::mem::zeroed() }
    },
    ..unsafe { core::mem::zeroed() }
};

#[repr(C)]
struct ccp_des3_def {
    mode: ccp_des3_mode,
    version: u32,
    name: *const i8,
    driver_name: *const i8,
    blocksize: u32,
    ivsize: u32,
    alg_defaults: *const skcipher_alg,
}

static DES3_ALGS: [ccp_des3_def; 2] = [
    ccp_des3_def { mode: CCP_DES3_MODE_ECB, version: CCP_VERSION(5, 0), name: c"ecb(des3_ede)".as_ptr(), driver_name: c"ecb-des3-ccp".as_ptr(), blocksize: DES3_EDE_BLOCK_SIZE, ivsize: 0, alg_defaults: unsafe { &CCP_DES3_DEFAULTS } },
    ccp_des3_def { mode: CCP_DES3_MODE_CBC, version: CCP_VERSION(5, 0), name: c"cbc(des3_ede)".as_ptr(), driver_name: c"cbc-des3-ccp".as_ptr(), blocksize: DES3_EDE_BLOCK_SIZE, ivsize: DES3_EDE_BLOCK_SIZE, alg_defaults: unsafe { &CCP_DES3_DEFAULTS } },
];

unsafe fn ccp_register_des3_alg(head: *mut list_head, def: *const ccp_des3_def) -> i32 {
    let ccp_alg = kzalloc_obj::<ccp_crypto_skcipher_alg>();
    if ccp_alg.is_null() { return -ENOMEM; }
    INIT_LIST_HEAD(&mut (*ccp_alg).entry);
    (*ccp_alg).mode = (*def).mode;
    let alg = &mut (*ccp_alg).alg;
    *alg = core::ptr::read((*def).alg_defaults);
    strscpy((*alg).base.cra_name.as_mut_ptr(), (*def).name);
    strscpy((*alg).base.cra_driver_name.as_mut_ptr(), (*def).driver_name);
    (*alg).base.cra_blocksize = (*def).blocksize;
    (*alg).ivsize = (*def).ivsize;
    let ret = crypto_register_skcipher(alg);
    if ret != 0 { pr_err((*alg).base.cra_name.as_ptr(), ret); kfree(ccp_alg as *mut core::ffi::c_void); return ret; }
    list_add(&mut (*ccp_alg).entry, head);
    0
}

unsafe fn ccp_register_des3_algs(head: *mut list_head) -> i32 {
    let ccpversion = ccp_version();
    for def in DES3_ALGS.iter() {
        if def.version > ccpversion { continue; }
        let ret = ccp_register_des3_alg(head, def);
        if ret != 0 { return ret; }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
