// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 - 2021
 *
 * Richard van Schagen <vschagen@icloud.com>
 * Christian Marangi <ansuelsmth@gmail.com>
 */

// C kernel dependencies are supplied by the surrounding translation unit.

pub unsafe extern "C" fn eip93_skcipher_handle_result(
    async_: *mut crypto_async_request,
    err: i32,
) {
    let ctx = crypto_tfm_ctx((*async_).tfm);
    let eip93 = (*ctx).eip93;
    let req = skcipher_request_cast(async_);
    let rctx = skcipher_request_ctx(req);

    eip93_unmap_dma(eip93, rctx, (*req).src, (*req).dst);
    eip93_handle_result(eip93, rctx, (*req).iv);
    skcipher_request_complete(req, err);
}

unsafe extern "C" fn eip93_skcipher_send_req(async_: *mut crypto_async_request) -> i32 {
    let req = skcipher_request_cast(async_);
    let rctx = skcipher_request_ctx(req);
    let err = check_valid_request(rctx);
    if err != 0 {
        skcipher_request_complete(req, err);
        return err;
    }
    eip93_send_req(async_, (*req).iv, rctx)
}

/* Crypto skcipher API functions */
unsafe extern "C" fn eip93_skcipher_cra_init(tfm: *mut crypto_tfm) -> i32 {
    let ctx = crypto_tfm_ctx(tfm);
    let tmpl = container_of((*tfm).__crt_alg, eip93_alg_template, alg_skcipher_base);
    crypto_skcipher_set_reqsize(__crypto_skcipher_cast(tfm), core::mem::size_of::<eip93_cipher_reqctx>());
    core::ptr::write_bytes(ctx as *mut u8, 0, core::mem::size_of::<eip93_crypto_ctx>());
    (*ctx).eip93 = (*tmpl).eip93;
    (*ctx).type_ = (*tmpl).type_;
    (*ctx).sa_record = kzalloc_obj::<sa_record>();
    if (*ctx).sa_record.is_null() { return -12; }
    0
}

unsafe extern "C" fn eip93_skcipher_cra_exit(tfm: *mut crypto_tfm) {
    let ctx = crypto_tfm_ctx(tfm);
    dma_unmap_single((*(*ctx).eip93).dev, (*ctx).sa_record_base,
        core::mem::size_of::<sa_record>(), DMA_TO_DEVICE);
    kfree((*ctx).sa_record);
}

unsafe extern "C" fn eip93_skcipher_setkey(ctfm: *mut crypto_skcipher, key: *const u8, len: u32) -> i32 {
    let tfm = crypto_skcipher_tfm(ctfm);
    let ctx = crypto_tfm_ctx(tfm);
    let tmpl = container_of((*tfm).__crt_alg, eip93_alg_template, alg_skcipher_base);
    let sa_record = (*ctx).sa_record;
    let mut keylen = len;
    let flags = (*tmpl).flags;
    let mut nonce: u32 = 0;
    if key.is_null() || keylen == 0 { return -22; }
    if IS_RFC3686(flags) {
        if len < CTR_RFC3686_NONCE_SIZE { return -22; }
        keylen = len - CTR_RFC3686_NONCE_SIZE;
        core::ptr::copy_nonoverlapping(key.add(keylen as usize), &mut nonce as *mut u32 as *mut u8, CTR_RFC3686_NONCE_SIZE as usize);
    }
    if flags & EIP93_ALG_DES != 0 {
        (*ctx).blksize = DES_BLOCK_SIZE;
        let ret = verify_skcipher_des_key(ctfm, key); if ret != 0 { return ret; }
    }
    if flags & EIP93_ALG_3DES != 0 {
        (*ctx).blksize = DES3_EDE_BLOCK_SIZE;
        let ret = verify_skcipher_des3_key(ctfm, key); if ret != 0 { return ret; }
    }
    if flags & EIP93_ALG_AES != 0 {
        let mut aes = core::mem::MaybeUninit::<crypto_aes_ctx>::uninit();
        (*ctx).blksize = AES_BLOCK_SIZE;
        let ret = aes_expandkey(aes.as_mut_ptr(), key, keylen); if ret != 0 { return ret; }
    }
    eip93_set_sa_record(sa_record, keylen, flags);
    core::ptr::copy_nonoverlapping(key, (*sa_record).sa_key.as_mut_ptr(), keylen as usize);
    (*ctx).sa_nonce = nonce;
    (*sa_record).sa_nonce = nonce;
    0
}

unsafe extern "C" fn eip93_skcipher_crypt(req: *mut skcipher_request) -> i32 {
    let rctx = skcipher_request_ctx(req);
    let async_ = &mut (*req).base as *mut crypto_async_request;
    let ctx = crypto_tfm_ctx((*req).base.tfm);
    let skcipher = crypto_skcipher_reqtfm(req);
    if (*req).cryptlen == 0 { return 0; }
    if (IS_ECB((*rctx).flags) || IS_CBC((*rctx).flags)) && !IS_ALIGNED((*req).cryptlen, crypto_skcipher_blocksize(skcipher)) { return -22; }
    (*ctx).sa_record_base = dma_map_single((*(*ctx).eip93).dev, (*ctx).sa_record, core::mem::size_of::<sa_record>(), DMA_TO_DEVICE);
    let ret = dma_mapping_error((*(*ctx).eip93).dev, (*ctx).sa_record_base); if ret != 0 { return ret; }
    (*rctx).assoclen = 0; (*rctx).textsize = (*req).cryptlen; (*rctx).authsize = 0;
    (*rctx).sg_src = (*req).src; (*rctx).sg_dst = (*req).dst; (*rctx).ivsize = crypto_skcipher_ivsize(skcipher);
    (*rctx).blksize = (*ctx).blksize; (*rctx).desc_flags = EIP93_DESC_SKCIPHER; (*rctx).sa_record_base = (*ctx).sa_record_base;
    eip93_skcipher_send_req(async_)
}

unsafe extern "C" fn eip93_skcipher_encrypt(req: *mut skcipher_request) -> i32 {
    let tmpl = container_of((*(*req).base.tfm).__crt_alg, eip93_alg_template, alg_skcipher_base);
    let rctx = skcipher_request_ctx(req); (*rctx).flags = (*tmpl).flags | EIP93_ENCRYPT; eip93_skcipher_crypt(req)
}

unsafe extern "C" fn eip93_skcipher_decrypt(req: *mut skcipher_request) -> i32 {
    let ctx = crypto_tfm_ctx((*req).base.tfm);
    let rctx = skcipher_request_ctx(req);
    let tmpl = container_of((*(*req).base.tfm).__crt_alg, eip93_alg_template, alg_skcipher_base);
    (*ctx).sa_record.sa_cmd0_word |= EIP93_SA_CMD_DIRECTION_IN;
    (*rctx).flags = (*tmpl).flags | EIP93_DECRYPT; eip93_skcipher_crypt(req)
}

// Algorithm registration descriptors. Their C-layout fields and constants are provided externally.
pub static mut eip93_alg_ecb_aes: eip93_alg_template = eip93_alg_template::skcipher("ecb(aes)", "ecb(aes-eip93)", EIP93_MODE_ECB | EIP93_ALG_AES, AES_MIN_KEY_SIZE, AES_MAX_KEY_SIZE, 0, AES_BLOCK_SIZE, eip93_skcipher_setkey, eip93_skcipher_encrypt, eip93_skcipher_decrypt);
pub static mut eip93_alg_cbc_aes: eip93_alg_template = eip93_alg_template::skcipher("cbc(aes)", "cbc(aes-eip93)", EIP93_MODE_CBC | EIP93_ALG_AES, AES_MIN_KEY_SIZE, AES_MAX_KEY_SIZE, AES_BLOCK_SIZE, AES_BLOCK_SIZE, eip93_skcipher_setkey, eip93_skcipher_encrypt, eip93_skcipher_decrypt);
pub static mut eip93_alg_ctr_aes: eip93_alg_template = eip93_alg_template::skcipher("ctr(aes)", "ctr(aes-eip93)", EIP93_MODE_CTR | EIP93_ALG_AES, AES_MIN_KEY_SIZE, AES_MAX_KEY_SIZE, AES_BLOCK_SIZE, 1, eip93_skcipher_setkey, eip93_skcipher_encrypt, eip93_skcipher_decrypt);
pub static mut eip93_alg_rfc3686_aes: eip93_alg_template = eip93_alg_template::skcipher("rfc3686(ctr(aes))", "rfc3686(ctr(aes-eip93))", EIP93_MODE_CTR | EIP93_MODE_RFC3686 | EIP93_ALG_AES, AES_MIN_KEY_SIZE + CTR_RFC3686_NONCE_SIZE, AES_MAX_KEY_SIZE + CTR_RFC3686_NONCE_SIZE, CTR_RFC3686_IV_SIZE, 1, eip93_skcipher_setkey, eip93_skcipher_encrypt, eip93_skcipher_decrypt);
pub static mut eip93_alg_ecb_des: eip93_alg_template = eip93_alg_template::skcipher("ecb(des)", "ecb(des-eip93)", EIP93_MODE_ECB | EIP93_ALG_DES, DES_KEY_SIZE, DES_KEY_SIZE, 0, DES_BLOCK_SIZE, eip93_skcipher_setkey, eip93_skcipher_encrypt, eip93_skcipher_decrypt);
pub static mut eip93_alg_cbc_des: eip93_alg_template = eip93_alg_template::skcipher("cbc(des)", "cbc(des-eip93)", EIP93_MODE_CBC | EIP93_ALG_DES, DES_KEY_SIZE, DES_KEY_SIZE, DES_BLOCK_SIZE, DES_BLOCK_SIZE, eip93_skcipher_setkey, eip93_skcipher_encrypt, eip93_skcipher_decrypt);
pub static mut eip93_alg_ecb_des3_ede: eip93_alg_template = eip93_alg_template::skcipher("ecb(des3_ede)", "ecb(des3_ede-eip93)", EIP93_MODE_ECB | EIP93_ALG_3DES, DES3_EDE_KEY_SIZE, DES3_EDE_KEY_SIZE, 0, DES3_EDE_BLOCK_SIZE, eip93_skcipher_setkey, eip93_skcipher_encrypt, eip93_skcipher_decrypt);
pub static mut eip93_alg_cbc_des3_ede: eip93_alg_template = eip93_alg_template::skcipher("cbc(des3_ede)", "cbc(des3_ede-eip93)", EIP93_MODE_CBC | EIP93_ALG_3DES, DES3_EDE_KEY_SIZE, DES3_EDE_KEY_SIZE, DES3_EDE_BLOCK_SIZE, DES3_EDE_BLOCK_SIZE, eip93_skcipher_setkey, eip93_skcipher_encrypt, eip93_skcipher_decrypt);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
