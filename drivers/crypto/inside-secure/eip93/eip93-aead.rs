// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 - 2021
 *
 * Richard van Schagen <vschagen@icloud.com>
 * Christian Marangi <ansuelsmth@gmail.com>
 */

// Kernel crypto, DMA, and local EIP93 declarations are supplied by the
// surrounding translation unit.

pub unsafe fn eip93_aead_handle_result(async_: *mut crypto_async_request, err: i32) {
    let ctx = crypto_tfm_ctx((*async_).tfm);
    let eip93 = (*ctx).eip93;
    let req = aead_request_cast(async_);
    let rctx = aead_request_ctx(req);
    eip93_unmap_dma(eip93, rctx, (*req).src, (*req).dst);
    eip93_handle_result(eip93, rctx, (*req).iv);
    aead_request_complete(req, err);
}

unsafe fn eip93_aead_send_req(async_: *mut crypto_async_request) -> i32 {
    let req = aead_request_cast(async_);
    let rctx = aead_request_ctx(req);
    let err = check_valid_request(rctx);
    if err != 0 {
        aead_request_complete(req, err);
        return err;
    }
    eip93_send_req(async_, (*req).iv, rctx)
}

unsafe fn eip93_aead_cra_init(tfm: *mut crypto_tfm) -> i32 {
    let ctx = crypto_tfm_ctx(tfm);
    let tmpl = container_of((*tfm).__crt_alg, eip93_alg_template, alg.aead.base);
    crypto_aead_set_reqsize(__crypto_aead_cast(tfm), core::mem::size_of::<eip93_cipher_reqctx>());
    (*ctx).eip93 = (*tmpl).eip93;
    (*ctx).flags = (*tmpl).flags;
    (*ctx).type_ = (*tmpl).type_;
    (*ctx).set_assoc = true;
    (*ctx).sa_record = kzalloc_obj((*ctx).sa_record);
    if (*ctx).sa_record.is_null() { return -12; }
    0
}

unsafe fn eip93_aead_cra_exit(tfm: *mut crypto_tfm) {
    let ctx = crypto_tfm_ctx(tfm);
    dma_unmap_single((*(*ctx).eip93).dev, (*ctx).sa_record_base,
                     core::mem::size_of_val(&*(*ctx).sa_record), DMA_TO_DEVICE);
    kfree((*ctx).sa_record);
}

unsafe fn eip93_aead_setkey(ctfm: *mut crypto_aead, key: *const u8, len: u32) -> i32 {
    let tfm = crypto_aead_tfm(ctfm);
    let ctx = crypto_tfm_ctx(tfm);
    let mut keys = core::mem::zeroed::<crypto_authenc_keys>();
    let mut aes = core::mem::zeroed::<crypto_aes_ctx>();
    let sa_record = (*ctx).sa_record;
    let mut nonce: u32 = 0;
    if crypto_authenc_extractkeys(&mut keys, key, len) != 0 { return -22; }
    if IS_RFC3686((*ctx).flags) {
        if keys.enckeylen < CTR_RFC3686_NONCE_SIZE { return -22; }
        keys.enckeylen -= CTR_RFC3686_NONCE_SIZE;
        memcpy(&mut nonce as *mut _ as *mut u8, keys.enckey.add(keys.enckeylen as usize), CTR_RFC3686_NONCE_SIZE);
    }
    let ret = match (*ctx).flags & EIP93_ALG_MASK {
        EIP93_ALG_DES => verify_aead_des_key(ctfm, keys.enckey, keys.enckeylen),
        EIP93_ALG_3DES => {
            if keys.enckeylen != DES3_EDE_KEY_SIZE { return -22; }
            verify_aead_des3_key(ctfm, keys.enckey, keys.enckeylen)
        }
        EIP93_ALG_AES => aes_expandkey(&mut aes, keys.enckey, keys.enckeylen),
        _ => 0,
    };
    if ret != 0 { return ret; }
    (*ctx).blksize = crypto_aead_blocksize(ctfm);
    eip93_set_sa_record(sa_record, keys.enckeylen, (*ctx).flags);
    (*sa_record).sa_cmd0_word &= !EIP93_SA_CMD_OPCODE;
    (*sa_record).sa_cmd0_word |= FIELD_PREP(EIP93_SA_CMD_OPCODE, EIP93_SA_CMD_OPCODE_BASIC_OUT_ENC_HASH);
    (*sa_record).sa_cmd0_word &= !EIP93_SA_CMD_DIGEST_LENGTH;
    (*sa_record).sa_cmd0_word |= FIELD_PREP(EIP93_SA_CMD_DIGEST_LENGTH, (*ctx).authsize / core::mem::size_of::<u32>() as u32);
    memcpy((*sa_record).sa_key.as_mut_ptr(), keys.enckey, keys.enckeylen);
    (*ctx).sa_nonce = nonce;
    (*sa_record).sa_nonce = nonce;
    let ret = eip93_hmac_setkey((*ctx).flags, keys.authkey, keys.authkeylen, (*ctx).authsize,
                                (*sa_record).sa_i_digest.as_mut_ptr(), (*sa_record).sa_o_digest.as_mut_ptr(), false);
    (*ctx).set_assoc = true;
    ret
}

unsafe fn eip93_aead_setauthsize(ctfm: *mut crypto_aead, authsize: u32) -> i32 {
    let ctx = crypto_tfm_ctx(crypto_aead_tfm(ctfm));
    (*ctx).authsize = authsize;
    (*(*ctx).sa_record).sa_cmd0_word &= !EIP93_SA_CMD_DIGEST_LENGTH;
    (*(*ctx).sa_record).sa_cmd0_word |= FIELD_PREP(EIP93_SA_CMD_DIGEST_LENGTH, authsize / core::mem::size_of::<u32>() as u32);
    0
}

unsafe fn eip93_aead_setassoc(ctx: *mut eip93_crypto_ctx, req: *mut aead_request) {
    let sa = (*ctx).sa_record;
    (*sa).sa_cmd1_word &= !EIP93_SA_CMD_HASH_CRYPT_OFFSET;
    (*sa).sa_cmd1_word |= FIELD_PREP(EIP93_SA_CMD_HASH_CRYPT_OFFSET, (*req).assoclen / core::mem::size_of::<u32>() as u32);
    (*ctx).assoclen = (*req).assoclen;
}

unsafe fn eip93_aead_crypt(req: *mut aead_request) -> i32 {
    let rctx = aead_request_ctx(req);
    let async_ = &mut (*req).base;
    let ctx = crypto_tfm_ctx((*req).base.tfm);
    let aead = crypto_aead_reqtfm(req);
    (*ctx).sa_record_base = dma_map_single((*(*ctx).eip93).dev, (*ctx).sa_record,
                                           core::mem::size_of_val(&*(*ctx).sa_record), DMA_TO_DEVICE);
    let ret = dma_mapping_error((*(*ctx).eip93).dev, (*ctx).sa_record_base);
    if ret != 0 { return ret; }
    (*rctx).textsize = (*req).cryptlen;
    (*rctx).blksize = (*ctx).blksize;
    (*rctx).assoclen = (*req).assoclen;
    (*rctx).authsize = (*ctx).authsize;
    (*rctx).sg_src = (*req).src;
    (*rctx).sg_dst = (*req).dst;
    (*rctx).ivsize = crypto_aead_ivsize(aead);
    (*rctx).desc_flags = EIP93_DESC_AEAD;
    (*rctx).sa_record_base = (*ctx).sa_record_base;
    if IS_DECRYPT((*rctx).flags) { (*rctx).textsize -= (*rctx).authsize; }
    eip93_aead_send_req(async_)
}

unsafe fn eip93_aead_encrypt(req: *mut aead_request) -> i32 {
    let ctx = crypto_tfm_ctx((*req).base.tfm);
    let rctx = aead_request_ctx(req);
    (*rctx).flags = (*ctx).flags | EIP93_ENCRYPT;
    if (*ctx).set_assoc { eip93_aead_setassoc(ctx, req); (*ctx).set_assoc = false; }
    if (*req).assoclen != (*ctx).assoclen { dev_err((*(*ctx).eip93).dev, "Request AAD length error\n"); return -22; }
    eip93_aead_crypt(req)
}

unsafe fn eip93_aead_decrypt(req: *mut aead_request) -> i32 {
    let ctx = crypto_tfm_ctx((*req).base.tfm);
    let rctx = aead_request_ctx(req);
    (*(*ctx).sa_record).sa_cmd0_word |= EIP93_SA_CMD_DIRECTION_IN;
    (*(*ctx).sa_record).sa_cmd1_word &= !(EIP93_SA_CMD_COPY_PAD | EIP93_SA_CMD_COPY_DIGEST);
    (*rctx).flags = (*ctx).flags | EIP93_DECRYPT;
    if (*ctx).set_assoc { eip93_aead_setassoc(ctx, req); (*ctx).set_assoc = false; }
    if (*req).assoclen != (*ctx).assoclen { dev_err((*(*ctx).eip93).dev, "Request AAD length error\n"); return -22; }
    eip93_aead_crypt(req)
}

macro_rules! eip93_aead_template {
    ($name:ident, $flags:expr, $iv:expr, $max:expr, $n:expr, $d:expr, $bs:expr) => {
        pub static mut $name: eip93_alg_template = eip93_alg_template {
            type_: EIP93_ALG_TYPE_AEAD, flags: EIP93_HASH_HMAC | $flags,
            alg: eip93_alg { aead: eip93_aead_alg { setkey: eip93_aead_setkey, encrypt: eip93_aead_encrypt, decrypt: eip93_aead_decrypt, ivsize: $iv, setauthsize: eip93_aead_setauthsize, maxauthsize: $max, base: eip93_alg_base { cra_name: $n, cra_driver_name: $d, cra_priority: EIP93_CRA_PRIORITY, cra_flags: CRYPTO_ALG_ASYNC | CRYPTO_ALG_KERN_DRIVER_ONLY | CRYPTO_ALG_ALLOCATES_MEMORY, cra_blocksize: $bs, cra_ctxsize: core::mem::size_of::<eip93_crypto_ctx>(), cra_alignmask: 0, cra_init: eip93_aead_cra_init, cra_exit: eip93_aead_cra_exit, cra_module: THIS_MODULE } } }
        };
    }
}

eip93_aead_template!(eip93_alg_authenc_hmac_md5_cbc_aes, EIP93_HASH_MD5 | EIP93_MODE_CBC | EIP93_ALG_AES, 16, MD5_DIGEST_SIZE, "authenc(hmac(md5),cbc(aes))", "authenc(hmac(md5-eip93), cbc(aes-eip93))", AES_BLOCK_SIZE);
eip93_aead_template!(eip93_alg_authenc_hmac_sha1_cbc_aes, EIP93_HASH_SHA1 | EIP93_MODE_CBC | EIP93_ALG_AES, 16, SHA1_DIGEST_SIZE, "authenc(hmac(sha1),cbc(aes))", "authenc(hmac(sha1-eip93), cbc(aes-eip93))", AES_BLOCK_SIZE);
eip93_aead_template!(eip93_alg_authenc_hmac_sha224_cbc_aes, EIP93_HASH_SHA224 | EIP93_MODE_CBC | EIP93_ALG_AES, 16, SHA224_DIGEST_SIZE, "authenc(hmac(sha224),cbc(aes))", "authenc(hmac(sha224-eip93), cbc(aes-eip93))", AES_BLOCK_SIZE);
eip93_aead_template!(eip93_alg_authenc_hmac_sha256_cbc_aes, EIP93_HASH_SHA256 | EIP93_MODE_CBC | EIP93_ALG_AES, 16, SHA256_DIGEST_SIZE, "authenc(hmac(sha256),cbc(aes))", "authenc(hmac(sha256-eip93), cbc(aes-eip93))", AES_BLOCK_SIZE);
eip93_aead_template!(eip93_alg_authenc_hmac_md5_rfc3686_aes, EIP93_HASH_MD5 | EIP93_MODE_CTR | EIP93_MODE_RFC3686 | EIP93_ALG_AES, CTR_RFC3686_IV_SIZE, MD5_DIGEST_SIZE, "authenc(hmac(md5),rfc3686(ctr(aes)))", "authenc(hmac(md5-eip93),rfc3686(ctr(aes-eip93)))", 1);
eip93_aead_template!(eip93_alg_authenc_hmac_sha1_rfc3686_aes, EIP93_HASH_SHA1 | EIP93_MODE_CTR | EIP93_MODE_RFC3686 | EIP93_ALG_AES, CTR_RFC3686_IV_SIZE, SHA1_DIGEST_SIZE, "authenc(hmac(sha1),rfc3686(ctr(aes)))", "authenc(hmac(sha1-eip93),rfc3686(ctr(aes-eip93)))", 1);
eip93_aead_template!(eip93_alg_authenc_hmac_sha224_rfc3686_aes, EIP93_HASH_SHA224 | EIP93_MODE_CTR | EIP93_MODE_RFC3686 | EIP93_ALG_AES, CTR_RFC3686_IV_SIZE, SHA224_DIGEST_SIZE, "authenc(hmac(sha224),rfc3686(ctr(aes)))", "authenc(hmac(sha224-eip93),rfc3686(ctr(aes-eip93)))", 1);
eip93_aead_template!(eip93_alg_authenc_hmac_sha256_rfc3686_aes, EIP93_HASH_SHA256 | EIP93_MODE_CTR | EIP93_MODE_RFC3686 | EIP93_ALG_AES, CTR_RFC3686_IV_SIZE, SHA256_DIGEST_SIZE, "authenc(hmac(sha256),rfc3686(ctr(aes)))", "authenc(hmac(sha256-eip93),rfc3686(ctr(aes-eip93)))", 1);
eip93_aead_template!(eip93_alg_authenc_hmac_md5_cbc_des, EIP93_HASH_MD5 | EIP93_MODE_CBC | EIP93_ALG_DES, DES_BLOCK_SIZE, MD5_DIGEST_SIZE, "authenc(hmac(md5),cbc(des))", "authenc(hmac(md5-eip93), cbc(des-eip93))", DES_BLOCK_SIZE);
eip93_aead_template!(eip93_alg_authenc_hmac_sha1_cbc_des, EIP93_HASH_SHA1 | EIP93_MODE_CBC | EIP93_ALG_DES, DES_BLOCK_SIZE, SHA1_DIGEST_SIZE, "authenc(hmac(sha1),cbc(des))", "authenc(hmac(sha1-eip93), cbc(des-eip93))", DES_BLOCK_SIZE);
eip93_aead_template!(eip93_alg_authenc_hmac_sha224_cbc_des, EIP93_HASH_SHA224 | EIP93_MODE_CBC | EIP93_ALG_DES, DES_BLOCK_SIZE, SHA224_DIGEST_SIZE, "authenc(hmac(sha224),cbc(des))", "authenc(hmac(sha224-eip93), cbc(des-eip93))", DES_BLOCK_SIZE);
eip93_aead_template!(eip93_alg_authenc_hmac_sha256_cbc_des, EIP93_HASH_SHA256 | EIP93_MODE_CBC | EIP93_ALG_DES, DES_BLOCK_SIZE, SHA256_DIGEST_SIZE, "authenc(hmac(sha256),cbc(des))", "authenc(hmac(sha256-eip93), cbc(des-eip93))", DES_BLOCK_SIZE);
eip93_aead_template!(eip93_alg_authenc_hmac_md5_cbc_des3_ede, EIP93_HASH_MD5 | EIP93_MODE_CBC | EIP93_ALG_3DES, DES3_EDE_BLOCK_SIZE, MD5_DIGEST_SIZE, "authenc(hmac(md5),cbc(des3_ede))", "authenc(hmac(md5-eip93), cbc(des3_ede-eip93))", DES3_EDE_BLOCK_SIZE);
eip93_aead_template!(eip93_alg_authenc_hmac_sha1_cbc_des3_ede, EIP93_HASH_SHA1 | EIP93_MODE_CBC | EIP93_ALG_3DES, DES3_EDE_BLOCK_SIZE, SHA1_DIGEST_SIZE, "authenc(hmac(sha1),cbc(des3_ede))", "authenc(hmac(sha1-eip93), cbc(des3_ede-eip93))", DES3_EDE_BLOCK_SIZE);
eip93_aead_template!(eip93_alg_authenc_hmac_sha224_cbc_des3_ede, EIP93_HASH_SHA224 | EIP93_MODE_CBC | EIP93_ALG_3DES, DES3_EDE_BLOCK_SIZE, SHA224_DIGEST_SIZE, "authenc(hmac(sha224),cbc(des3_ede))", "authenc(hmac(sha224-eip93), cbc(des3_ede-eip93))", DES3_EDE_BLOCK_SIZE);
eip93_aead_template!(eip93_alg_authenc_hmac_sha256_cbc_des3_ede, EIP93_HASH_SHA256 | EIP93_MODE_CBC | EIP93_ALG_3DES, DES3_EDE_BLOCK_SIZE, SHA256_DIGEST_SIZE, "authenc(hmac(sha256),cbc(des3_ede))", "authenc(hmac(sha256-eip93), cbc(des3_ede-eip93))", DES3_EDE_BLOCK_SIZE);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
