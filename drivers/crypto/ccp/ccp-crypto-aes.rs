// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Cryptographic Coprocessor (CCP) AES crypto API support
 *
 * Copyright (C) 2013-2019 Advanced Micro Devices, Inc.
 *
 * Author: Tom Lendacky <thomas.lendacky@amd.com>
 */

// C dependencies supplied by the surrounding kernel translation.

unsafe fn ccp_aes_complete(async_req: *mut crypto_async_request, ret: i32) -> i32 {
    let req = skcipher_request_cast(async_req);
    let ctx = crypto_skcipher_ctx_dma(crypto_skcipher_reqtfm(req));
    let rctx = skcipher_request_ctx_dma(req);

    if ret != 0 {
        return ret;
    }

    if (*ctx).u.aes.mode != CCP_AES_MODE_ECB {
        let ivsize = crypto_skcipher_ivsize(crypto_skcipher_reqtfm(req));
        memcpy((*req).iv, (*rctx).iv.as_ptr(), ivsize);
    }

    0
}

unsafe fn ccp_aes_setkey(tfm: *mut crypto_skcipher, key: *const u8, key_len: u32) -> i32 {
    let alg = ccp_crypto_skcipher_alg(tfm);
    let ctx = crypto_skcipher_ctx_dma(tfm);

    match key_len {
        AES_KEYSIZE_128 => (*ctx).u.aes.type_ = CCP_AES_TYPE_128,
        AES_KEYSIZE_192 => (*ctx).u.aes.type_ = CCP_AES_TYPE_192,
        AES_KEYSIZE_256 => (*ctx).u.aes.type_ = CCP_AES_TYPE_256,
        _ => return -EINVAL,
    }
    (*ctx).u.aes.mode = (*alg).mode;
    (*ctx).u.aes.key_len = key_len;

    memcpy((*ctx).u.aes.key.as_mut_ptr(), key, key_len as usize);
    sg_init_one(&mut (*ctx).u.aes.key_sg, (*ctx).u.aes.key.as_mut_ptr(), key_len);

    0
}

unsafe fn ccp_aes_crypt(req: *mut skcipher_request, encrypt: bool) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx_dma(tfm);
    let rctx = skcipher_request_ctx_dma(req);
    let mut iv_sg: *mut scatterlist = core::ptr::null_mut();
    let mut iv_len: u32 = 0;

    if (*ctx).u.aes.key_len == 0 { return -EINVAL; }
    if ((*ctx).u.aes.mode == CCP_AES_MODE_ECB || (*ctx).u.aes.mode == CCP_AES_MODE_CBC)
        && ((*req).cryptlen & (AES_BLOCK_SIZE - 1)) != 0 { return -EINVAL; }

    if (*ctx).u.aes.mode != CCP_AES_MODE_ECB {
        if (*req).iv.is_null() { return -EINVAL; }
        memcpy((*rctx).iv.as_mut_ptr(), (*req).iv, AES_BLOCK_SIZE as usize);
        iv_sg = &mut (*rctx).iv_sg;
        iv_len = AES_BLOCK_SIZE;
        sg_init_one(iv_sg, (*rctx).iv.as_mut_ptr(), iv_len);
    }

    memset(&mut (*rctx).cmd as *mut _ as *mut u8, 0, core::mem::size_of::<ccp_cmd>());
    INIT_LIST_HEAD(&mut (*rctx).cmd.entry);
    (*rctx).cmd.engine = CCP_ENGINE_AES;
    (*rctx).cmd.u.aes.type_ = (*ctx).u.aes.type_;
    (*rctx).cmd.u.aes.mode = (*ctx).u.aes.mode;
    (*rctx).cmd.u.aes.action = if encrypt { CCP_AES_ACTION_ENCRYPT } else { CCP_AES_ACTION_DECRYPT };
    (*rctx).cmd.u.aes.key = &mut (*ctx).u.aes.key_sg;
    (*rctx).cmd.u.aes.key_len = (*ctx).u.aes.key_len;
    (*rctx).cmd.u.aes.iv = iv_sg;
    (*rctx).cmd.u.aes.iv_len = iv_len;
    (*rctx).cmd.u.aes.src = (*req).src;
    (*rctx).cmd.u.aes.src_len = (*req).cryptlen;
    (*rctx).cmd.u.aes.dst = (*req).dst;

    ccp_crypto_enqueue_request(&mut (*req).base, &mut (*rctx).cmd)
}

unsafe fn ccp_aes_encrypt(req: *mut skcipher_request) -> i32 { ccp_aes_crypt(req, true) }
unsafe fn ccp_aes_decrypt(req: *mut skcipher_request) -> i32 { ccp_aes_crypt(req, false) }

unsafe fn ccp_aes_init_tfm(tfm: *mut crypto_skcipher) -> i32 {
    let ctx = crypto_skcipher_ctx_dma(tfm);
    (*ctx).complete = Some(ccp_aes_complete);
    (*ctx).u.aes.key_len = 0;
    crypto_skcipher_set_reqsize(tfm, core::mem::size_of::<ccp_aes_req_ctx>());
    0
}

unsafe fn ccp_aes_rfc3686_complete(async_req: *mut crypto_async_request, ret: i32) -> i32 {
    let req = skcipher_request_cast(async_req);
    let rctx = skcipher_request_ctx_dma(req);
    (*req).iv = (*rctx).rfc3686_info;
    ccp_aes_complete(async_req, ret)
}

unsafe fn ccp_aes_rfc3686_setkey(tfm: *mut crypto_skcipher, key: *const u8, mut key_len: u32) -> i32 {
    let ctx = crypto_skcipher_ctx_dma(tfm);
    if key_len < CTR_RFC3686_NONCE_SIZE { return -EINVAL; }
    key_len -= CTR_RFC3686_NONCE_SIZE;
    memcpy((*ctx).u.aes.nonce.as_mut_ptr(), key.add(key_len as usize), CTR_RFC3686_NONCE_SIZE as usize);
    ccp_aes_setkey(tfm, key, key_len)
}

unsafe fn ccp_aes_rfc3686_crypt(req: *mut skcipher_request, encrypt: bool) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx_dma(tfm);
    let rctx = skcipher_request_ctx_dma(req);
    let mut iv = (*rctx).rfc3686_iv.as_mut_ptr();
    memcpy(iv, (*ctx).u.aes.nonce.as_ptr(), CTR_RFC3686_NONCE_SIZE as usize);
    iv = iv.add(CTR_RFC3686_NONCE_SIZE as usize);
    memcpy(iv, (*req).iv, CTR_RFC3686_IV_SIZE as usize);
    iv = iv.add(CTR_RFC3686_IV_SIZE as usize);
    *iv.cast::<u32>() = cpu_to_be32(1);
    (*rctx).rfc3686_info = (*req).iv;
    (*req).iv = (*rctx).rfc3686_iv.as_mut_ptr();
    ccp_aes_crypt(req, encrypt)
}

unsafe fn ccp_aes_rfc3686_encrypt(req: *mut skcipher_request) -> i32 { ccp_aes_rfc3686_crypt(req, true) }
unsafe fn ccp_aes_rfc3686_decrypt(req: *mut skcipher_request) -> i32 { ccp_aes_rfc3686_crypt(req, false) }

unsafe fn ccp_aes_rfc3686_init_tfm(tfm: *mut crypto_skcipher) -> i32 {
    let ctx = crypto_skcipher_ctx_dma(tfm);
    (*ctx).complete = Some(ccp_aes_rfc3686_complete);
    (*ctx).u.aes.key_len = 0;
    crypto_skcipher_set_reqsize_dma(tfm, core::mem::size_of::<ccp_aes_req_ctx>());
    0
}

// The following algorithm descriptors preserve the C static initialization and
// rely on the corresponding kernel types and constants supplied externally.
static mut CCP_AES_DEFAULTS: skcipher_alg = skcipher_alg {
    setkey: Some(ccp_aes_setkey), encrypt: Some(ccp_aes_encrypt), decrypt: Some(ccp_aes_decrypt),
    min_keysize: AES_MIN_KEY_SIZE, max_keysize: AES_MAX_KEY_SIZE, init: Some(ccp_aes_init_tfm),
    ..unsafe { core::mem::zeroed() }
};

static mut CCP_AES_RFC3686_DEFAULTS: skcipher_alg = skcipher_alg {
    setkey: Some(ccp_aes_rfc3686_setkey), encrypt: Some(ccp_aes_rfc3686_encrypt), decrypt: Some(ccp_aes_rfc3686_decrypt),
    min_keysize: AES_MIN_KEY_SIZE + CTR_RFC3686_NONCE_SIZE, max_keysize: AES_MAX_KEY_SIZE + CTR_RFC3686_NONCE_SIZE,
    init: Some(ccp_aes_rfc3686_init_tfm), ..unsafe { core::mem::zeroed() }
};

#[repr(C)]
struct ccp_aes_def {
    mode: ccp_aes_mode,
    version: u32,
    name: *const core::ffi::c_char,
    driver_name: *const core::ffi::c_char,
    blocksize: u32,
    ivsize: u32,
    alg_defaults: *const skcipher_alg,
}

static mut AES_ALGS: [ccp_aes_def; 4] = [
    ccp_aes_def { mode: CCP_AES_MODE_ECB, version: CCP_VERSION(3, 0), name: c"ecb(aes)".as_ptr(), driver_name: c"ecb-aes-ccp".as_ptr(), blocksize: AES_BLOCK_SIZE, ivsize: 0, alg_defaults: &CCP_AES_DEFAULTS },
    ccp_aes_def { mode: CCP_AES_MODE_CBC, version: CCP_VERSION(3, 0), name: c"cbc(aes)".as_ptr(), driver_name: c"cbc-aes-ccp".as_ptr(), blocksize: AES_BLOCK_SIZE, ivsize: AES_BLOCK_SIZE, alg_defaults: &CCP_AES_DEFAULTS },
    ccp_aes_def { mode: CCP_AES_MODE_CTR, version: CCP_VERSION(3, 0), name: c"ctr(aes)".as_ptr(), driver_name: c"ctr-aes-ccp".as_ptr(), blocksize: 1, ivsize: AES_BLOCK_SIZE, alg_defaults: &CCP_AES_DEFAULTS },
    ccp_aes_def { mode: CCP_AES_MODE_CTR, version: CCP_VERSION(3, 0), name: c"rfc3686(ctr(aes))".as_ptr(), driver_name: c"rfc3686-ctr-aes-ccp".as_ptr(), blocksize: 1, ivsize: CTR_RFC3686_IV_SIZE, alg_defaults: &CCP_AES_RFC3686_DEFAULTS },
];

unsafe fn ccp_register_aes_alg(head: *mut list_head, def_: *const ccp_aes_def) -> i32 {
    let ccp_alg = kzalloc_obj::<ccp_crypto_skcipher_alg>();
    if ccp_alg.is_null() { return -ENOMEM; }
    INIT_LIST_HEAD(&mut (*ccp_alg).entry);
    (*ccp_alg).mode = (*def_).mode;
    let alg = &mut (*ccp_alg).alg;
    *alg = *(*def_).alg_defaults;
    strscpy((*alg).base.cra_name.as_mut_ptr(), (*def_).name);
    strscpy((*alg).base.cra_driver_name.as_mut_ptr(), (*def_).driver_name);
    (*alg).base.cra_blocksize = (*def_).blocksize;
    (*alg).ivsize = (*def_).ivsize;
    let ret = crypto_register_skcipher(alg);
    if ret != 0 { pr_err((*alg).base.cra_name.as_ptr(), ret); kfree(ccp_alg); return ret; }
    list_add(&mut (*ccp_alg).entry, head);
    0
}

pub unsafe fn ccp_register_aes_algs(head: *mut list_head) -> i32 {
    let ccpversion = ccp_version();
    for i in 0..AES_ALGS.len() {
        if AES_ALGS[i].version > ccpversion { continue; }
        let ret = ccp_register_aes_alg(head, &AES_ALGS[i]);
        if ret != 0 { return ret; }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
