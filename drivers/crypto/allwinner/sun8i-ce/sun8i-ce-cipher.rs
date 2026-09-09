// SPDX-License-Identifier: GPL-2.0
/*
 * sun8i-ce-cipher.c - hardware cryptographic offloader for
 * Allwinner H3/A64/H5/H2+/H6/R40 SoC
 *
 * Copyright (C) 2016-2019 Corentin LABBE <clabbe.montjoie@gmail.com>
 *
 * This file add support for AES cipher with 128,192,256 bits keysize in
 * CBC and ECB mode.
 *
 * You could find a link for the datasheet in Documentation/arch/arm/sunxi.rst
 */

// Kernel and local header dependencies are supplied by the surrounding crate.

unsafe fn sun8i_ce_cipher_need_fallback(areq: *mut skcipher_request) -> bool {
    let tfm = crypto_skcipher_reqtfm(areq);
    let mut sg: *mut scatterlist;
    let alg = crypto_skcipher_alg(tfm);
    let algt = container_of!(alg, sun8i_ce_alg_template, alg.skcipher.base);
    let mut todo: u32;
    let mut len: u32;

    if sg_nents_for_len((*areq).src, (*areq).cryptlen) > MAX_SG || sg_nents_for_len((*areq).dst, (*areq).cryptlen) > MAX_SG {
        if IS_ENABLED!(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG) { (*algt).stat_fb_maxsg += 1; }
        return true;
    }
    if (*areq).cryptlen < crypto_skcipher_ivsize(tfm) {
        if IS_ENABLED!(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG) { (*algt).stat_fb_leniv += 1; }
        return true;
    }
    if (*areq).cryptlen == 0 {
        if IS_ENABLED!(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG) { (*algt).stat_fb_len0 += 1; }
        return true;
    }
    if (*areq).cryptlen % 16 != 0 {
        if IS_ENABLED!(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG) { (*algt).stat_fb_mod16 += 1; }
        return true;
    }
    len = (*areq).cryptlen;
    sg = (*areq).src;
    while !sg.is_null() {
        if !IS_ALIGNED!((*sg).offset, core::mem::size_of::<u32>()) {
            if IS_ENABLED!(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG) { (*algt).stat_fb_srcali += 1; }
            return true;
        }
        todo = core::cmp::min(len, (*sg).length);
        if todo % 4 != 0 {
            if IS_ENABLED!(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG) { (*algt).stat_fb_srclen += 1; }
            return true;
        }
        len -= todo;
        sg = sg_next(sg);
    }
    len = (*areq).cryptlen;
    sg = (*areq).dst;
    while !sg.is_null() {
        if !IS_ALIGNED!((*sg).offset, core::mem::size_of::<u32>()) {
            if IS_ENABLED!(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG) { (*algt).stat_fb_dstali += 1; }
            return true;
        }
        todo = core::cmp::min(len, (*sg).length);
        if todo % 4 != 0 {
            if IS_ENABLED!(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG) { (*algt).stat_fb_dstlen += 1; }
            return true;
        }
        len -= todo;
        sg = sg_next(sg);
    }
    false
}

unsafe fn sun8i_ce_cipher_fallback(areq: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(areq);
    let op = crypto_skcipher_ctx(tfm);
    let rctx = skcipher_request_ctx(areq);
    if IS_ENABLED!(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG) {
        let alg = crypto_skcipher_alg(tfm);
        let algt = container_of!(alg, sun8i_ce_alg_template, alg.skcipher.base);
        (*algt).stat_fb += 1;
    }
    skcipher_request_set_tfm(&mut (*rctx).fallback_req, (*op).fallback_tfm);
    skcipher_request_set_callback(&mut (*rctx).fallback_req, (*areq).base.flags, (*areq).base.complete, (*areq).base.data);
    skcipher_request_set_crypt(&mut (*rctx).fallback_req, (*areq).src, (*areq).dst, (*areq).cryptlen, (*areq).iv);
    if (*rctx).op_dir & CE_DECRYPTION != 0 { crypto_skcipher_decrypt(&mut (*rctx).fallback_req) } else { crypto_skcipher_encrypt(&mut (*rctx).fallback_req) }
}

unsafe fn sun8i_ce_cipher_prepare(areq: *mut skcipher_request, cet: *mut ce_task) -> i32 {
    let tfm = crypto_skcipher_reqtfm(areq);
    let op = crypto_skcipher_ctx(tfm);
    let ce = (*op).ce;
    let rctx = skcipher_request_ctx(areq);
    let alg = crypto_skcipher_alg(tfm);
    let algt = container_of!(alg, sun8i_ce_alg_template, alg.skcipher.base);
    let mut sg: *mut scatterlist;
    let (mut todo, mut len, mut offset, mut ivsize): (u32,u32,u32,u32);
    let (mut common, mut sym): (u32,u32);
    let mut nr_sgs = 0;
    let mut nr_sgd = 0;
    let mut err = 0;
    let ns = sg_nents_for_len((*areq).src, (*areq).cryptlen);
    let nd = sg_nents_for_len((*areq).dst, (*areq).cryptlen);

    if IS_ENABLED!(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG) { (*algt).stat_req += 1; }
    core::ptr::write_bytes(cet as *mut u8, 0, core::mem::size_of::<ce_task>());
    (*cet).t_id = cpu_to_le32!((*rctx).flow);
    common = (*ce).variant.alg_cipher[(*algt).ce_algo_id];
    common |= (*rctx).op_dir | CE_COMM_INT;
    (*cet).t_common_ctl = cpu_to_le32!(common);
    (*cet).t_dlen = cpu_to_le32!(if (*ce).variant.cipher_t_dlen_in_bytes { (*areq).cryptlen } else { (*areq).cryptlen / 4 });
    sym = (*ce).variant.op_mode[(*algt).ce_blockmode];
    match (*op).keylen { 16 => sym |= CE_AES_128BITS, 24 => sym |= CE_AES_192BITS, 32 => sym |= CE_AES_256BITS, _ => {} }
    (*cet).t_sym_ctl = cpu_to_le32!(sym);
    (*cet).t_asym_ctl = 0;
    (*rctx).addr_key = dma_map_single((*ce).dev, (*op).key, (*op).keylen, DMA_TO_DEVICE);
    if dma_mapping_error((*ce).dev, (*rctx).addr_key) { dev_err!((*ce).dev, "Cannot DMA MAP KEY\n"); return -EFAULT; }
    (*cet).t_key = desc_addr_val_le32(ce, (*rctx).addr_key);
    ivsize = crypto_skcipher_ivsize(tfm);
    if !(*areq).iv.is_null() && ivsize > 0 {
        if (*rctx).op_dir & CE_DECRYPTION != 0 { offset = (*areq).cryptlen - ivsize; scatterwalk_map_and_copy((*rctx).backup_iv.as_mut_ptr(), (*areq).src, offset, ivsize, 0); }
        core::ptr::copy_nonoverlapping((*areq).iv, (*rctx).bounce_iv.as_mut_ptr(), ivsize as usize);
        (*rctx).addr_iv = dma_map_single((*ce).dev, (*rctx).bounce_iv.as_mut_ptr(), ivsize, DMA_TO_DEVICE);
        if dma_mapping_error((*ce).dev, (*rctx).addr_iv) { dev_err!((*ce).dev, "Cannot DMA MAP IV\n"); dma_unmap_single((*ce).dev, (*rctx).addr_key, (*op).keylen, DMA_TO_DEVICE); return -ENOMEM; }
        (*cet).t_iv = desc_addr_val_le32(ce, (*rctx).addr_iv);
    }
    if (*areq).src == (*areq).dst { nr_sgs = dma_map_sg((*ce).dev, (*areq).src, ns, DMA_BIDIRECTIONAL); if nr_sgs <= 0 || nr_sgs > MAX_SG { err = -EINVAL; } nr_sgd = nr_sgs; } else { nr_sgs = dma_map_sg((*ce).dev, (*areq).src, ns, DMA_TO_DEVICE); if nr_sgs <= 0 || nr_sgs > MAX_SG { err = -EINVAL; } else { nr_sgd = dma_map_sg((*ce).dev, (*areq).dst, nd, DMA_FROM_DEVICE); if nr_sgd <= 0 || nr_sgd > MAX_SG { err = -EINVAL; } } }
    if err != 0 { if nr_sgs > 0 { dma_unmap_sg((*ce).dev, (*areq).src, ns, DMA_TO_DEVICE); } if nr_sgd > 0 && (*areq).src != (*areq).dst { dma_unmap_sg((*ce).dev, (*areq).dst, nd, DMA_FROM_DEVICE); } if !(*areq).iv.is_null() && ivsize > 0 && !dma_mapping_error((*ce).dev, (*rctx).addr_iv) { dma_unmap_single((*ce).dev, (*rctx).addr_iv, ivsize, DMA_TO_DEVICE); } dma_unmap_single((*ce).dev, (*rctx).addr_key, (*op).keylen, DMA_TO_DEVICE); return err; }
    len = (*areq).cryptlen;
    for_each_sg!((*areq).src, sg, nr_sgs, i) { (*cet).t_src[i].addr = desc_addr_val_le32(ce, sg_dma_address(sg)); todo = core::cmp::min(len, sg_dma_len(sg)); (*cet).t_src[i].len = cpu_to_le32!(todo / 4); len -= todo; }
    if len > 0 { err = -EINVAL; }
    len = (*areq).cryptlen;
    for_each_sg!((*areq).dst, sg, nr_sgd, i) { (*cet).t_dst[i].addr = desc_addr_val_le32(ce, sg_dma_address(sg)); todo = core::cmp::min(len, sg_dma_len(sg)); (*cet).t_dst[i].len = cpu_to_le32!(todo / 4); len -= todo; }
    if len > 0 { err = -EINVAL; }
    if err != 0 { if (*areq).src == (*areq).dst { dma_unmap_sg((*ce).dev, (*areq).src, ns, DMA_BIDIRECTIONAL); } else { dma_unmap_sg((*ce).dev, (*areq).src, ns, DMA_TO_DEVICE); dma_unmap_sg((*ce).dev, (*areq).dst, nd, DMA_FROM_DEVICE); } if !(*areq).iv.is_null() && ivsize > 0 && !dma_mapping_error((*ce).dev, (*rctx).addr_iv) { dma_unmap_single((*ce).dev, (*rctx).addr_iv, ivsize, DMA_TO_DEVICE); } dma_unmap_single((*ce).dev, (*rctx).addr_key, (*op).keylen, DMA_TO_DEVICE); return err; }
    (*rctx).nr_sgs = ns; (*rctx).nr_sgd = nd; 0
}

unsafe fn sun8i_ce_cipher_unprepare(areq: *mut skcipher_request, cet: *mut ce_task) {
    let tfm = crypto_skcipher_reqtfm(areq); let op = crypto_skcipher_ctx(tfm); let ce = (*op).ce; let rctx = skcipher_request_ctx(areq);
    let ivsize = crypto_skcipher_ivsize(tfm);
    if (*areq).src == (*areq).dst { dma_unmap_sg((*ce).dev, (*areq).src, (*rctx).nr_sgs, DMA_BIDIRECTIONAL); } else { if (*rctx).nr_sgs > 0 { dma_unmap_sg((*ce).dev, (*areq).src, (*rctx).nr_sgs, DMA_TO_DEVICE); } dma_unmap_sg((*ce).dev, (*areq).dst, (*rctx).nr_sgd, DMA_FROM_DEVICE); }
    if !(*areq).iv.is_null() && ivsize > 0 { if (*cet).t_iv != 0 { dma_unmap_single((*ce).dev, (*rctx).addr_iv, ivsize, DMA_TO_DEVICE); } let offset = (*areq).cryptlen - ivsize; if (*rctx).op_dir & CE_DECRYPTION != 0 { core::ptr::copy_nonoverlapping((*rctx).backup_iv.as_ptr(), (*areq).iv, ivsize as usize); memzero_explicit((*rctx).backup_iv.as_mut_ptr(), ivsize); } else { scatterwalk_map_and_copy((*areq).iv, (*areq).dst, offset, ivsize, 0); } memzero_explicit((*rctx).bounce_iv.as_mut_ptr(), ivsize); }
    dma_unmap_single((*ce).dev, (*rctx).addr_key, (*op).keylen, DMA_TO_DEVICE);
}

pub unsafe fn sun8i_ce_cipher_do_one(engine: *mut crypto_engine, areq: *mut core::ffi::c_void) -> i32 { let req = skcipher_request_cast(areq); let rctx = skcipher_request_ctx(req); let tfm = crypto_skcipher_reqtfm(req); let ctx = crypto_skcipher_ctx(tfm); let ce = (*ctx).ce; let chan = &mut (*ce).chanlist[(*rctx).flow as usize]; let err = sun8i_ce_cipher_prepare(req, chan.tl); if err != 0 { return err; } let err = sun8i_ce_run_task(ce, (*rctx).flow, crypto_tfm_alg_name((*req).base.tfm)); sun8i_ce_cipher_unprepare(req, chan.tl); local_bh_disable(); crypto_finalize_skcipher_request(engine, req, err); local_bh_enable(); 0 }

pub unsafe fn sun8i_ce_skdecrypt(areq: *mut skcipher_request) -> i32 { let tfm = crypto_skcipher_reqtfm(areq); let op = crypto_skcipher_ctx(tfm); let rctx = skcipher_request_ctx(areq); (*rctx).op_dir = CE_DECRYPTION; if sun8i_ce_cipher_need_fallback(areq) { return sun8i_ce_cipher_fallback(areq); } let e = sun8i_ce_get_engine_number((*op).ce); (*rctx).flow = e; crypto_transfer_skcipher_request_to_engine((*op).ce.chanlist[e as usize].engine, areq) }

pub unsafe fn sun8i_ce_skencrypt(areq: *mut skcipher_request) -> i32 { let tfm = crypto_skcipher_reqtfm(areq); let op = crypto_skcipher_ctx(tfm); let rctx = skcipher_request_ctx(areq); (*rctx).op_dir = CE_ENCRYPTION; if sun8i_ce_cipher_need_fallback(areq) { return sun8i_ce_cipher_fallback(areq); } let e = sun8i_ce_get_engine_number((*op).ce); (*rctx).flow = e; crypto_transfer_skcipher_request_to_engine((*op).ce.chanlist[e as usize].engine, areq) }

pub unsafe fn sun8i_ce_cipher_init(tfm: *mut crypto_tfm) -> i32 { let op = crypto_tfm_ctx(tfm); let sktfm = __crypto_skcipher_cast(tfm); let alg = crypto_skcipher_alg(sktfm); let algt = container_of!(alg, sun8i_ce_alg_template, alg.skcipher.base); core::ptr::write_bytes(op as *mut u8, 0, core::mem::size_of::<sun8i_cipher_tfm_ctx>()); (*op).ce = (*algt).ce; let name = crypto_tfm_alg_name(tfm); (*op).fallback_tfm = crypto_alloc_skcipher(name, 0, CRYPTO_ALG_NEED_FALLBACK); if IS_ERR!((*op).fallback_tfm) { return PTR_ERR!((*op).fallback_tfm); } crypto_skcipher_set_reqsize(sktfm, core::mem::size_of::<sun8i_cipher_req_ctx>() + crypto_skcipher_reqsize((*op).fallback_tfm)); let err = pm_runtime_resume_and_get((*op).ce.dev); if err < 0 { crypto_free_skcipher((*op).fallback_tfm); return err; } 0 }

pub unsafe fn sun8i_ce_cipher_exit(tfm: *mut crypto_tfm) { let op = crypto_tfm_ctx(tfm); kfree_sensitive((*op).key); crypto_free_skcipher((*op).fallback_tfm); pm_runtime_put_sync_suspend((*op).ce.dev); }

pub unsafe fn sun8i_ce_aes_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: u32) -> i32 { let op = crypto_skcipher_ctx(tfm); if keylen != 16 && keylen != 24 && keylen != 32 { return -EINVAL; } kfree_sensitive((*op).key); (*op).keylen = keylen; (*op).key = kmemdup(key, keylen, GFP_KERNEL | GFP_DMA); if (*op).key.is_null() { return -ENOMEM; } crypto_skcipher_clear_flags((*op).fallback_tfm, CRYPTO_TFM_REQ_MASK); crypto_skcipher_set_flags((*op).fallback_tfm, (*tfm).base.crt_flags & CRYPTO_TFM_REQ_MASK); crypto_skcipher_setkey((*op).fallback_tfm, key, keylen) }

pub unsafe fn sun8i_ce_des3_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: u32) -> i32 { let op = crypto_skcipher_ctx(tfm); let err = verify_skcipher_des3_key(tfm, key); if err != 0 { return err; } kfree_sensitive((*op).key); (*op).keylen = keylen; (*op).key = kmemdup(key, keylen, GFP_KERNEL | GFP_DMA); if (*op).key.is_null() { return -ENOMEM; } crypto_skcipher_clear_flags((*op).fallback_tfm, CRYPTO_TFM_REQ_MASK); crypto_skcipher_set_flags((*op).fallback_tfm, (*tfm).base.crt_flags & CRYPTO_TFM_REQ_MASK); crypto_skcipher_setkey((*op).fallback_tfm, key, keylen) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
