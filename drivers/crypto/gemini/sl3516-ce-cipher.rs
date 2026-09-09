// SPDX-License-Identifier: GPL-2.0
/*
 * sl3516-ce-cipher.c - hardware cryptographic offloader for Storlink SL3516 SoC
 *
 * Copyright (C) 2021 Corentin LABBE <clabbe@baylibre.com>
 *
 * This file adds support for AES cipher with 128,192,256 bits keysize in
 * ECB mode.
 */

// Dependencies supplied by the surrounding kernel translation.

/* sl3516_ce_need_fallback - check if a request can be handled by the CE */
unsafe fn sl3516_ce_need_fallback(areq: *mut skcipher_request) -> bool {
    let tfm = crypto_skcipher_reqtfm(areq);
    let op = crypto_skcipher_ctx(tfm);
    let ce = (*op).ce;
    let mut in_sg: *mut scatterlist;
    let mut out_sg: *mut scatterlist;
    let mut sg: *mut scatterlist;

    if (*areq).cryptlen == 0 || (*areq).cryptlen % 16 != 0 {
        (*ce).fallback_mod16 += 1;
        return true;
    }

    if sg_nents((*areq).src) > MAXDESC / 2 {
        (*ce).fallback_sg_count_tx += 1;
        return true;
    }
    if sg_nents((*areq).dst) > MAXDESC {
        (*ce).fallback_sg_count_rx += 1;
        return true;
    }

    sg = (*areq).src;
    while !sg.is_null() {
        if (*sg).length % 16 != 0 {
            (*ce).fallback_mod16 += 1;
            return true;
        }
        if !is_aligned((*sg).offset, 16) {
            (*ce).fallback_align16 += 1;
            return true;
        }
        sg = sg_next(sg);
    }
    sg = (*areq).dst;
    while !sg.is_null() {
        if (*sg).length % 16 != 0 {
            (*ce).fallback_mod16 += 1;
            return true;
        }
        if !is_aligned((*sg).offset, 16) {
            (*ce).fallback_align16 += 1;
            return true;
        }
        sg = sg_next(sg);
    }

    in_sg = (*areq).src;
    out_sg = (*areq).dst;
    while !in_sg.is_null() && !out_sg.is_null() {
        if (*in_sg).length != (*out_sg).length {
            (*ce).fallback_not_same_len += 1;
            return true;
        }
        in_sg = sg_next(in_sg);
        out_sg = sg_next(out_sg);
    }
    if !in_sg.is_null() || !out_sg.is_null() { return true; }
    false
}

unsafe fn sl3516_ce_cipher_fallback(areq: *mut skcipher_request) -> c_int {
    let tfm = crypto_skcipher_reqtfm(areq);
    let op = crypto_skcipher_ctx(tfm);
    let rctx = skcipher_request_ctx(areq);
    let alg = crypto_skcipher_alg(tfm);
    let algt = container_of!(alg, sl3516_ce_alg_template, alg.skcipher.base);
    (*algt).stat_fb += 1;

    skcipher_request_set_tfm(&mut (*rctx).fallback_req, (*op).fallback_tfm);
    skcipher_request_set_callback(&mut (*rctx).fallback_req, (*areq).base.flags,
                                  (*areq).base.complete, (*areq).base.data);
    skcipher_request_set_crypt(&mut (*rctx).fallback_req, (*areq).src, (*areq).dst,
                               (*areq).cryptlen, (*areq).iv);
    if (*rctx).op_dir == CE_DECRYPTION {
        crypto_skcipher_decrypt(&mut (*rctx).fallback_req)
    } else {
        crypto_skcipher_encrypt(&mut (*rctx).fallback_req)
    }
}

unsafe fn sl3516_ce_cipher(areq: *mut skcipher_request) -> c_int {
    let tfm = crypto_skcipher_reqtfm(areq);
    let op = crypto_skcipher_ctx(tfm);
    let ce = (*op).ce;
    let rctx = skcipher_request_ctx(areq);
    let alg = crypto_skcipher_alg(tfm);
    let algt = container_of!(alg, sl3516_ce_alg_template, alg.skcipher.base);
    let mut sg: *mut scatterlist;
    let mut todo: usize;
    let mut len: usize;
    let mut nr_sgs: c_int = 0;
    let mut nr_sgd: c_int = 0;
    let mut err: c_int = 0;
    let mut i: c_int;

    (*algt).stat_req += 1;
    if (*areq).src == (*areq).dst {
        nr_sgs = dma_map_sg((*ce).dev, (*areq).src, sg_nents((*areq).src), DMA_BIDIRECTIONAL);
        if nr_sgs <= 0 || nr_sgs > MAXDESC / 2 { err = -EINVAL; goto!(__theend); }
        nr_sgd = nr_sgs;
    } else {
        nr_sgs = dma_map_sg((*ce).dev, (*areq).src, sg_nents((*areq).src), DMA_TO_DEVICE);
        if nr_sgs <= 0 || nr_sgs > MAXDESC / 2 { err = -EINVAL; goto!(__theend); }
        nr_sgd = dma_map_sg((*ce).dev, (*areq).dst, sg_nents((*areq).dst), DMA_FROM_DEVICE);
        if nr_sgd <= 0 || nr_sgd > MAXDESC { err = -EINVAL; goto!(__theend_sgs); }
    }

    len = (*areq).cryptlen; i = 0; sg = (*areq).src;
    while i < nr_sgs && !sg.is_null() && len != 0 {
        if sg_dma_len(sg) == 0 { sg = sg_next(sg); continue; }
        (*rctx).t_src[i as usize].addr = sg_dma_address(sg);
        todo = core::cmp::min(len, sg_dma_len(sg));
        (*rctx).t_src[i as usize].len = todo; len -= todo; i += 1; sg = sg_next(sg);
    }
    if len > 0 { err = -EINVAL; goto!(__theend_sgs); }

    len = (*areq).cryptlen; i = 0; sg = (*areq).dst;
    while i < nr_sgd && !sg.is_null() && len != 0 {
        if sg_dma_len(sg) == 0 { sg = sg_next(sg); continue; }
        (*rctx).t_dst[i as usize].addr = sg_dma_address(sg);
        todo = core::cmp::min(len, sg_dma_len(sg));
        (*rctx).t_dst[i as usize].len = todo; len -= todo; i += 1; sg = sg_next(sg);
    }
    if len > 0 { err = -EINVAL; goto!(__theend_sgs); }

    match (*algt).mode {
        ECB_AES => {
            (*rctx).pctrllen = core::mem::size_of::<pkt_control_ecb>();
            let ecb = (*ce).pctrl as *mut pkt_control_ecb;
            (*rctx).tqflag = TQ0_TYPE_CTRL | TQ1_CIPHER | TQ4_KEY0 | TQ5_KEY4 | TQ6_KEY6;
            (*ecb).control.op_mode = (*rctx).op_dir;
            (*ecb).control.cipher_algorithm = ECB_AES;
            (*ecb).cipher.header_len = 0;
            (*ecb).cipher.algorithm_len = (*areq).cryptlen;
            cpu_to_be32_array((*ecb).key.as_mut_ptr() as *mut __be32,
                              (*op).key as *mut u32, (*op).keylen / 4);
            (*rctx).h = &mut (*ecb).cipher;
            (*ecb).control.aesnk = (*op).keylen / 4;
        }
        _ => {}
    }
    (*rctx).nr_sgs = nr_sgs; (*rctx).nr_sgd = nr_sgd;
    err = sl3516_ce_run_task(ce, rctx, crypto_tfm_alg_name((*areq).base.tfm));

    __theend_sgs: {
        if (*areq).src == (*areq).dst {
            dma_unmap_sg((*ce).dev, (*areq).src, sg_nents((*areq).src), DMA_BIDIRECTIONAL);
        } else {
            dma_unmap_sg((*ce).dev, (*areq).src, sg_nents((*areq).src), DMA_TO_DEVICE);
            dma_unmap_sg((*ce).dev, (*areq).dst, sg_nents((*areq).dst), DMA_FROM_DEVICE);
        }
    }
    __theend: err
}

pub unsafe fn sl3516_ce_handle_cipher_request(engine: *mut crypto_engine, areq: *mut c_void) -> c_int {
    let breq = container_of!(areq, skcipher_request, base);
    let err = sl3516_ce_cipher(breq);
    local_bh_disable(); crypto_finalize_skcipher_request(engine, breq, err); local_bh_enable();
    0
}

pub unsafe fn sl3516_ce_skdecrypt(areq: *mut skcipher_request) -> c_int {
    let tfm = crypto_skcipher_reqtfm(areq); let op = crypto_skcipher_ctx(tfm);
    let rctx = skcipher_request_ctx(areq); core::ptr::write_bytes(rctx, 0, 1); (*rctx).op_dir = CE_DECRYPTION;
    if sl3516_ce_need_fallback(areq) { return sl3516_ce_cipher_fallback(areq); }
    crypto_transfer_skcipher_request_to_engine((*(*op).ce).engine, areq)
}

pub unsafe fn sl3516_ce_skencrypt(areq: *mut skcipher_request) -> c_int {
    let tfm = crypto_skcipher_reqtfm(areq); let op = crypto_skcipher_ctx(tfm);
    let rctx = skcipher_request_ctx(areq); core::ptr::write_bytes(rctx, 0, 1); (*rctx).op_dir = CE_ENCRYPTION;
    if sl3516_ce_need_fallback(areq) { return sl3516_ce_cipher_fallback(areq); }
    crypto_transfer_skcipher_request_to_engine((*(*op).ce).engine, areq)
}

pub unsafe fn sl3516_ce_cipher_init(tfm: *mut crypto_tfm) -> c_int {
    let op = crypto_tfm_ctx(tfm); let sktfm = __crypto_skcipher_cast(tfm);
    core::ptr::write_bytes(op, 0, 1);
    let alg = crypto_skcipher_alg(sktfm);
    let algt = container_of!(alg, sl3516_ce_alg_template, alg.skcipher.base);
    (*op).ce = (*algt).ce;
    let name = crypto_tfm_alg_name(tfm);
    (*op).fallback_tfm = crypto_alloc_skcipher(name, 0, CRYPTO_ALG_NEED_FALLBACK);
    if is_err((*op).fallback_tfm) { return ptr_err((*op).fallback_tfm); }
    crypto_skcipher_set_reqsize(sktfm, core::mem::size_of::<sl3516_ce_cipher_req_ctx>() + crypto_skcipher_reqsize((*op).fallback_tfm));
    let err = pm_runtime_get_sync((*(*op).ce).dev);
    if err < 0 { pm_runtime_put_noidle((*(*op).ce).dev); crypto_free_skcipher((*op).fallback_tfm); return err; }
    0
}

pub unsafe fn sl3516_ce_cipher_exit(tfm: *mut crypto_tfm) {
    let op = crypto_tfm_ctx(tfm);
    kfree_sensitive((*op).key); crypto_free_skcipher((*op).fallback_tfm); pm_runtime_put_sync_suspend((*(*op).ce).dev);
}

pub unsafe fn sl3516_ce_aes_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: usize) -> c_int {
    let op = crypto_skcipher_ctx(tfm); let ce = (*op).ce;
    match keylen { 16 | 24 | 32 => {}, _ => return -EINVAL }
    kfree_sensitive((*op).key); (*op).keylen = keylen; (*op).key = kmemdup(key, keylen, GFP_KERNEL | GFP_DMA);
    if (*op).key.is_null() { return -ENOMEM; }
    crypto_skcipher_clear_flags((*op).fallback_tfm, CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_set_flags((*op).fallback_tfm, (*tfm).base.crt_flags & CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_setkey((*op).fallback_tfm, key, keylen)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
