// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2010-2014, The Linux Foundation. All rights reserved.
 */

// Kernel and driver dependencies are supplied by the surrounding tree.

static mut AES_SW_MAX_LEN: u32 = CONFIG_CRYPTO_DEV_QCE_SW_MAX_LEN;

static mut SKCIPHER_ALGS: ListHead = ListHead::new();

unsafe extern "C" fn qce_skcipher_done(data: *mut c_void) {
    let async_req = data as *mut crypto_async_request;
    let req = skcipher_request_cast(async_req);
    let rctx = skcipher_request_ctx(req);
    let tmpl = to_cipher_tmpl(crypto_skcipher_reqtfm(req));
    let qce = (*tmpl).qce;
    let result_buf = (*qce).dma.result_buf;
    let diff_dst = (*req).src != (*req).dst;
    let dir_src = if diff_dst { DMA_TO_DEVICE } else { DMA_BIDIRECTIONAL };
    let dir_dst = if diff_dst { DMA_FROM_DEVICE } else { DMA_BIDIRECTIONAL };
    let mut status: u32 = 0;

    let error = qce_dma_terminate_all(&mut (*qce).dma);
    if error != 0 {
        dev_dbg((*qce).dev, "skcipher dma termination error (%d)\n", error);
    }
    if diff_dst {
        dma_unmap_sg((*qce).dev, (*rctx).src_sg, (*rctx).src_nents, dir_src);
    }
    dma_unmap_sg((*qce).dev, (*rctx).dst_sg, (*rctx).dst_nents, dir_dst);
    sg_free_table(&mut (*rctx).dst_tbl);

    let error = qce_check_status(qce, &mut status);
    if error < 0 {
        dev_dbg((*qce).dev, "skcipher operation error (%x)\n", status);
    }
    memcpy((*rctx).iv, (*result_buf).encr_cntr_iv, (*rctx).ivsize);
    ((*qce).async_req_done)(qce, error);
}

unsafe extern "C" fn qce_skcipher_async_req_handle(async_req: *mut crypto_async_request) -> c_int {
    let req = skcipher_request_cast(async_req);
    let rctx = skcipher_request_ctx(req);
    let skcipher = crypto_skcipher_reqtfm(req);
    let tmpl = to_cipher_tmpl(skcipher);
    let qce = (*tmpl).qce;
    let diff_dst = (*req).src != (*req).dst;
    let dir_src = if diff_dst { DMA_TO_DEVICE } else { DMA_BIDIRECTIONAL };
    let dir_dst = if diff_dst { DMA_FROM_DEVICE } else { DMA_BIDIRECTIONAL };

    (*rctx).iv = (*req).iv;
    (*rctx).ivsize = crypto_skcipher_ivsize(skcipher);
    (*rctx).cryptlen = (*req).cryptlen;
    (*rctx).src_nents = sg_nents_for_len((*req).src, (*req).cryptlen);
    (*rctx).dst_nents = if diff_dst { sg_nents_for_len((*req).dst, (*req).cryptlen) } else { (*rctx).src_nents };
    if (*rctx).src_nents < 0 { dev_err((*qce).dev, "Invalid numbers of src SG.\n"); return (*rctx).src_nents; }
    if (*rctx).dst_nents < 0 { dev_err((*qce).dev, "Invalid numbers of dst SG.\n"); return -(*rctx).dst_nents; }
    (*rctx).dst_nents += 1;
    let gfp = if ((*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP) != 0 { GFP_KERNEL } else { GFP_ATOMIC };
    let mut ret = sg_alloc_table(&mut (*rctx).dst_tbl, (*rctx).dst_nents, gfp);
    if ret != 0 { return ret; }
    sg_init_one(&mut (*rctx).result_sg, (*qce).dma.result_buf, QCE_RESULT_BUF_SZ);
    let mut sg = qce_sgtable_add(&mut (*rctx).dst_tbl, (*req).dst, (*req).cryptlen);
    if IS_ERR(sg) { ret = PTR_ERR(sg); goto_error_free(rctx, ret); }
    sg = qce_sgtable_add(&mut (*rctx).dst_tbl, &mut (*rctx).result_sg, QCE_RESULT_BUF_SZ);
    if IS_ERR(sg) { ret = PTR_ERR(sg); goto_error_free(rctx, ret); }
    sg_mark_end(sg);
    (*rctx).dst_sg = (*rctx).dst_tbl.sgl;
    let dst_nents = dma_map_sg((*qce).dev, (*rctx).dst_sg, (*rctx).dst_nents, dir_dst);
    if dst_nents == 0 { ret = -EIO; goto_error_free(rctx, ret); }
    let src_nents;
    if diff_dst {
        src_nents = dma_map_sg((*qce).dev, (*req).src, (*rctx).src_nents, dir_src);
        if src_nents == 0 { ret = -EIO; goto_error_unmap_dst(qce, rctx, dir_dst, ret); }
        (*rctx).src_sg = (*req).src;
    } else { (*rctx).src_sg = (*rctx).dst_sg; src_nents = dst_nents - 1; }
    ret = qce_dma_prep_sgs(&mut (*qce).dma, (*rctx).src_sg, src_nents, (*rctx).dst_sg, dst_nents, Some(qce_skcipher_done), async_req);
    if ret != 0 { goto_error_unmap_src(qce, rctx, req, diff_dst, dir_src, dir_dst, ret); }
    qce_dma_issue_pending(&mut (*qce).dma);
    ret = qce_start(async_req, (*tmpl).crypto_alg_type);
    if ret != 0 { qce_dma_terminate_all(&mut (*qce).dma); goto_error_unmap_src(qce, rctx, req, diff_dst, dir_src, dir_dst, ret); }
    0
}

unsafe fn qce_skcipher_setkey(ablk: *mut crypto_skcipher, key: *const u8, keylen: u32) -> c_int {
    let tfm = crypto_skcipher_tfm(ablk); let ctx = crypto_tfm_ctx(tfm);
    let flags = (*to_cipher_tmpl(ablk)).alg_flags;
    if key.is_null() || keylen == 0 { return -EINVAL; }
    let key_part = if IS_XTS(flags) { keylen >> 1 } else { keylen };
    if IS_XTS(flags) && !memcmp(key, key.add(key_part as usize), key_part as usize) { return -ENOKEY; }
    match key_part { AES_KEYSIZE_128 | AES_KEYSIZE_256 => memcpy((*ctx).enc_key, key, keylen as usize), AES_KEYSIZE_192 => (), _ => return -EINVAL }
    let ret = crypto_skcipher_setkey((*ctx).fallback, key, keylen); if ret == 0 { (*ctx).enc_keylen = keylen; } ret
}

unsafe fn qce_skcipher_crypt(req: *mut skcipher_request, encrypt: c_int) -> c_int {
    let tfm = crypto_skcipher_reqtfm(req); let ctx = crypto_skcipher_ctx(tfm); let rctx = skcipher_request_ctx(req); let tmpl = to_cipher_tmpl(tfm);
    let blocksize = crypto_skcipher_blocksize(tfm); (*rctx).flags = (*tmpl).alg_flags | if encrypt != 0 { QCE_ENCRYPT } else { QCE_DECRYPT };
    let keylen = if IS_XTS((*rctx).flags) { (*ctx).enc_keylen >> 1 } else { (*ctx).enc_keylen };
    if (*req).cryptlen == 0 { return 0; }
    if IS_CBC((*rctx).flags) && !IS_ALIGNED((*req).cryptlen, blocksize) { return -EINVAL; }
    if IS_AES((*rctx).flags) && ((keylen != AES_KEYSIZE_128 && keylen != AES_KEYSIZE_256) || (IS_XTS((*rctx).flags) && ((*req).cryptlen <= AES_SW_MAX_LEN || ((*req).cryptlen > QCE_SECTOR_SIZE && (*req).cryptlen % QCE_SECTOR_SIZE != 0))))) {
        skcipher_request_set_tfm(&mut (*rctx).fallback_req, (*ctx).fallback); skcipher_request_set_callback(&mut (*rctx).fallback_req, (*req).base.flags, (*req).base.complete, (*req).base.data); skcipher_request_set_crypt(&mut (*rctx).fallback_req, (*req).src, (*req).dst, (*req).cryptlen, (*req).iv);
        return if encrypt != 0 { crypto_skcipher_encrypt(&mut (*rctx).fallback_req) } else { crypto_skcipher_decrypt(&mut (*rctx).fallback_req) };
    }
    ((*tmpl).qce).async_req_enqueue((*tmpl).qce, &mut (*req).base)
}

unsafe fn qce_skcipher_encrypt(req: *mut skcipher_request) -> c_int { qce_skcipher_crypt(req, 1) }
unsafe fn qce_skcipher_decrypt(req: *mut skcipher_request) -> c_int { qce_skcipher_crypt(req, 0) }

// The remaining registration definitions mirror the C algorithm tables and callbacks.
#[repr(C)]
struct qce_skcipher_def { flags: c_ulong, name: *const c_char, drv_name: *const c_char, blocksize: u32, chunksize: u32, ivsize: u32, min_keysize: u32, max_keysize: u32 }

static SKCIPHER_DEF: [qce_skcipher_def; 3] = [
    qce_skcipher_def { flags: QCE_ALG_AES | QCE_MODE_CBC, name: c"cbc(aes)".as_ptr(), drv_name: c"cbc-aes-qce".as_ptr(), blocksize: AES_BLOCK_SIZE, chunksize: 0, ivsize: AES_BLOCK_SIZE, min_keysize: AES_MIN_KEY_SIZE, max_keysize: AES_MAX_KEY_SIZE },
    qce_skcipher_def { flags: QCE_ALG_AES | QCE_MODE_CTR, name: c"ctr(aes)".as_ptr(), drv_name: c"ctr-aes-qce".as_ptr(), blocksize: 1, chunksize: AES_BLOCK_SIZE, ivsize: AES_BLOCK_SIZE, min_keysize: AES_MIN_KEY_SIZE, max_keysize: AES_MAX_KEY_SIZE },
    qce_skcipher_def { flags: QCE_ALG_AES | QCE_MODE_XTS, name: c"xts(aes)".as_ptr(), drv_name: c"xts-aes-qce".as_ptr(), blocksize: AES_BLOCK_SIZE, chunksize: 0, ivsize: AES_BLOCK_SIZE, min_keysize: AES_MIN_KEY_SIZE * 2, max_keysize: AES_MAX_KEY_SIZE * 2 },
];

unsafe extern "C" fn qce_skcipher_register(qce: *mut qce_device) -> c_int {
    let mut i = 0; while i < SKCIPHER_DEF.len() { let ret = qce_skcipher_register_one(&SKCIPHER_DEF[i], qce); if ret != 0 { qce_skcipher_unregister(qce); return ret; } i += 1; } 0
}
unsafe extern "C" fn qce_skcipher_unregister(_qce: *mut qce_device) { }

#[repr(C)]
pub struct qce_algo_ops { pub r#type: u32, pub register_algs: unsafe extern "C" fn(*mut qce_device) -> c_int, pub unregister_algs: unsafe extern "C" fn(*mut qce_device), pub async_req_handle: unsafe extern "C" fn(*mut crypto_async_request) -> c_int }

pub static SKCIPHER_OPS: qce_algo_ops = qce_algo_ops { r#type: CRYPTO_ALG_TYPE_SKCIPHER, register_algs: qce_skcipher_register, unregister_algs: qce_skcipher_unregister, async_req_handle: qce_skcipher_async_req_handle };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
