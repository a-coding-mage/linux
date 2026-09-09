// SPDX-License-Identifier: GPL-2.0
/*
 * sun8i-ce-hash.c - hardware cryptographic offloader for
 * Allwinner H3/A64/H5/H2+/H6/R40 SoC
 *
 * Copyright (C) 2015-2020 Corentin Labbe <clabbe@baylibre.com>
 *
 * This file add support for MD5 and SHA1/SHA224/SHA256/SHA384/SHA512.
 *
 * You could find the datasheet in Documentation/arch/arm/sunxi.rst
 */

// C header dependencies are supplied by the surrounding kernel translation.

unsafe fn sun8i_ce_hash_stat_fb_inc(tfm: *mut crypto_ahash) {
    if IS_ENABLED(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG) {
        let alg = crypto_ahash_alg(tfm);
        let algt = container_of!(alg, sun8i_ce_alg_template, alg.hash.base);
        (*algt).stat_fb += 1;
    }
}

pub unsafe fn sun8i_ce_hash_init_tfm(tfm: *mut crypto_ahash) -> c_int {
    let op = crypto_ahash_ctx(tfm);
    let alg = crypto_ahash_alg(tfm);
    let algt = container_of!(alg, sun8i_ce_alg_template, alg.hash.base);
    (*op).ce = (*algt).ce;
    (*op).fallback_tfm = crypto_alloc_ahash(crypto_ahash_alg_name(tfm), 0, CRYPTO_ALG_NEED_FALLBACK);
    if IS_ERR((*op).fallback_tfm) {
        dev_err((*(*algt).ce).dev, "Fallback driver could no be loaded\n");
        return PTR_ERR((*op).fallback_tfm);
    }
    crypto_ahash_set_statesize(tfm, crypto_ahash_statesize((*op).fallback_tfm));
    crypto_ahash_set_reqsize(tfm, size_of::<sun8i_ce_hash_reqctx>() + crypto_ahash_reqsize((*op).fallback_tfm) + CRYPTO_DMA_PADDING);
    if IS_ENABLED(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG) {
        memcpy((*algt).fbname.as_mut_ptr(), crypto_ahash_driver_name((*op).fallback_tfm), CRYPTO_MAX_ALG_NAME);
    }
    let err = pm_runtime_resume_and_get((*(*op).ce).dev);
    if err < 0 { crypto_free_ahash((*op).fallback_tfm); return err; }
    0
}

pub unsafe fn sun8i_ce_hash_exit_tfm(tfm: *mut crypto_ahash) {
    let ctx = crypto_ahash_ctx(tfm);
    crypto_free_ahash((*ctx).fallback_tfm);
    pm_runtime_put_sync_suspend((*(*ctx).ce).dev);
}

pub unsafe fn sun8i_ce_hash_init(areq: *mut ahash_request) -> c_int {
    let rctx = ahash_request_ctx_dma(areq);
    let tfm = crypto_ahash_reqtfm(areq);
    let ctx = crypto_ahash_ctx(tfm);
    memset(rctx, 0, size_of::<sun8i_ce_hash_reqctx>());
    ahash_request_set_tfm(&mut (*rctx).fallback_req, (*ctx).fallback_tfm);
    ahash_request_set_callback(&mut (*rctx).fallback_req, (*areq).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP, (*areq).base.complete, (*areq).base.data);
    crypto_ahash_init(&mut (*rctx).fallback_req)
}

unsafe fn hash_fallback_req(areq: *mut ahash_request) -> (*mut sun8i_ce_hash_reqctx, *mut crypto_ahash) {
    let rctx = ahash_request_ctx_dma(areq);
    let tfm = crypto_ahash_reqtfm(areq);
    let ctx = crypto_ahash_ctx(tfm);
    ahash_request_set_tfm(&mut (*rctx).fallback_req, (*ctx).fallback_tfm);
    ahash_request_set_callback(&mut (*rctx).fallback_req, (*areq).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP, (*areq).base.complete, (*areq).base.data);
    (rctx, tfm)
}

pub unsafe fn sun8i_ce_hash_export(areq: *mut ahash_request, out: *mut c_void) -> c_int { let (r, _) = hash_fallback_req(areq); crypto_ahash_export(&mut (*r).fallback_req, out) }
pub unsafe fn sun8i_ce_hash_import(areq: *mut ahash_request, input: *const c_void) -> c_int { let (r, _) = hash_fallback_req(areq); crypto_ahash_import(&mut (*r).fallback_req, input) }
pub unsafe fn sun8i_ce_hash_final(areq: *mut ahash_request) -> c_int { let (r, t) = hash_fallback_req(areq); sun8i_ce_hash_stat_fb_inc(t); ahash_request_set_crypt(&mut (*r).fallback_req, ptr::null_mut(), (*areq).result, 0); crypto_ahash_final(&mut (*r).fallback_req) }
pub unsafe fn sun8i_ce_hash_update(areq: *mut ahash_request) -> c_int { let (r, _) = hash_fallback_req(areq); ahash_request_set_crypt(&mut (*r).fallback_req, (*areq).src, ptr::null_mut(), (*areq).nbytes); crypto_ahash_update(&mut (*r).fallback_req) }
pub unsafe fn sun8i_ce_hash_finup(areq: *mut ahash_request) -> c_int { let (r, t) = hash_fallback_req(areq); sun8i_ce_hash_stat_fb_inc(t); ahash_request_set_crypt(&mut (*r).fallback_req, (*areq).src, (*areq).result, (*areq).nbytes); crypto_ahash_finup(&mut (*r).fallback_req) }

unsafe fn sun8i_ce_hash_digest_fb(areq: *mut ahash_request) -> c_int { let (r, t) = hash_fallback_req(areq); sun8i_ce_hash_stat_fb_inc(t); ahash_request_set_crypt(&mut (*r).fallback_req, (*areq).src, (*areq).result, (*areq).nbytes); crypto_ahash_digest(&mut (*r).fallback_req) }

unsafe fn sun8i_ce_hash_need_fallback(areq: *mut ahash_request) -> bool {
    let tfm = crypto_ahash_reqtfm(areq); let alg = __crypto_ahash_alg((*tfm).base.__crt_alg); let algt = container_of!(alg, sun8i_ce_alg_template, alg.hash.base); let mut sg = (*areq).src;
    if (*areq).nbytes == 0 { if IS_ENABLED(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG) { (*algt).stat_fb_len0 += 1; } return true; }
    if sg_nents_for_len((*areq).src, (*areq).nbytes) > MAX_SG - 1 { if IS_ENABLED(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG) { (*algt).stat_fb_maxsg += 1; } return true; }
    while !sg.is_null() { if (*sg).length % 4 != 0 { if IS_ENABLED(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG) { (*algt).stat_fb_srclen += 1; } return true; } if !IS_ALIGNED((*sg).offset, size_of::<u32>()) { if IS_ENABLED(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG) { (*algt).stat_fb_srcali += 1; } return true; } sg = sg_next(sg); }
    false
}

pub unsafe fn sun8i_ce_hash_digest(areq: *mut ahash_request) -> c_int {
    if sun8i_ce_hash_need_fallback(areq) { return sun8i_ce_hash_digest_fb(areq); }
    let tfm = crypto_ahash_reqtfm(areq); let ctx = crypto_ahash_ctx(tfm); let rctx = ahash_request_ctx_dma(areq); let ce = (*ctx).ce;
    let e = sun8i_ce_get_engine_number(ce); (*rctx).flow = e; crypto_transfer_hash_request_to_engine((*ce).chanlist[e].engine, areq)
}

unsafe fn hash_pad(buf: *mut __le32, bufsize: c_uint, padi: u64, byte_count: u64, le: bool, bs: c_int) -> u64 {
    let mut j = padi; *buf.add(j as usize) = cpu_to_le32(0x80); j += 1;
    let (mut fill, min_fill) = if bs == 64 { (64 - byte_count % 64, 3 * size_of::<u32>() as u64) } else { (128 - byte_count % 128, 5 * size_of::<u32>() as u64) };
    if fill < min_fill { fill += bs as u64; }
    let k = j; j += (fill - min_fill) / size_of::<u32>() as u64;
    if j * 4 > bufsize as u64 { pr_err!("%s OVERFLOW %llu\n", "hash_pad", j); return 0; }
    for n in k..j { *buf.add(n as usize) = 0; }
    if le { *(*buf.add(j as usize) as *mut __le64) = cpu_to_le64(byte_count << 3); j += 2; }
    else if bs == 64 { *(*buf.add(j as usize) as *mut __be64) = cpu_to_be64(byte_count << 3); j += 2; }
    else { *(*buf.add(j as usize) as *mut __be64) = cpu_to_be64(byte_count >> 61); j += 2; *(*buf.add(j as usize) as *mut __be64) = cpu_to_be64(byte_count << 3); j += 2; }
    if j * 4 > bufsize as u64 { pr_err!("%s OVERFLOW %llu\n", "hash_pad", j); return 0; } j
}

// The remaining hardware preparation, unpreparation, and engine-run paths retain
// the C implementation's external kernel structures and DMA operations.
pub unsafe fn sun8i_ce_hash_prepare(areq: *mut ahash_request, cet: *mut ce_task) -> c_int {
    let tfm = crypto_ahash_reqtfm(areq); let alg = __crypto_ahash_alg((*tfm).base.__crt_alg);
    let r = ahash_request_ctx_dma(areq); let algt = container_of!(alg, sun8i_ce_alg_template, alg.hash.base); let ce = (*algt).ce;
    let mut digestsize = crypto_ahash_digestsize(tfm); let bs = crypto_ahash_blocksize(tfm) as u64;
    if digestsize == SHA224_DIGEST_SIZE { digestsize = SHA256_DIGEST_SIZE; } if digestsize == SHA384_DIGEST_SIZE { digestsize = SHA512_DIGEST_SIZE; }
    if IS_ENABLED(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG) { (*algt).stat_req += 1; }
    memset(cet, 0, size_of::<ce_task>()); (*cet).t_id = cpu_to_le32((*r).flow);
    (*cet).t_common_ctl = cpu_to_le32((*(*ce).variant).alg_hash[(*algt).ce_algo_id] | CE_COMM_INT); (*cet).t_sym_ctl = 0; (*cet).t_asym_ctl = 0;
    (*r).nr_sgs = sg_nents_for_len((*areq).src, (*areq).nbytes); let nr = dma_map_sg((*ce).dev, (*areq).src, (*r).nr_sgs, DMA_TO_DEVICE);
    if nr <= 0 || nr > MAX_SG { dma_unmap_sg((*ce).dev, (*areq).src, (*r).nr_sgs, DMA_TO_DEVICE); return -EINVAL; }
    let mut len = (*areq).nbytes; let mut sg = (*areq).src; let mut i = 0;
    while i < nr { (*cet).t_src[i].addr = desc_addr_val_le32(ce, sg_dma_address(sg)); let todo = min(len, sg_dma_len(sg)); (*cet).t_src[i].len = cpu_to_le32(todo / 4); len -= todo; sg = sg_next(sg); i += 1; }
    if len > 0 { dma_unmap_sg((*ce).dev, (*areq).src, (*r).nr_sgs, DMA_TO_DEVICE); return -EINVAL; }
    (*r).result_len = digestsize; (*r).addr_res = dma_map_single((*ce).dev, (*r).result, (*r).result_len, DMA_FROM_DEVICE); if dma_mapping_error((*ce).dev, (*r).addr_res) { dma_unmap_sg((*ce).dev, (*areq).src, (*r).nr_sgs, DMA_TO_DEVICE); return -EINVAL; }
    let j = hash_pad((*r).pad.as_mut_ptr(), (2 * bs) as c_uint, 0, (*areq).nbytes as u64, (*algt).ce_algo_id == CE_ID_HASH_MD5, bs as c_int); if j == 0 { dma_unmap_single((*ce).dev, (*r).addr_res, (*r).result_len, DMA_FROM_DEVICE); dma_unmap_sg((*ce).dev, (*areq).src, (*r).nr_sgs, DMA_TO_DEVICE); return -EINVAL; }
    (*cet).t_dst[0].addr = desc_addr_val_le32(ce, (*r).addr_res); (*cet).t_dst[0].len = cpu_to_le32((*r).result_len / 4); (*r).pad_len = j * 4; (*r).addr_pad = dma_map_single((*ce).dev, (*r).pad, (*r).pad_len, DMA_TO_DEVICE); (*cet).t_src[i].addr = desc_addr_val_le32(ce, (*r).addr_pad); (*cet).t_src[i].len = cpu_to_le32(j as u32);
    if dma_mapping_error((*ce).dev, (*r).addr_pad) { dma_unmap_single((*ce).dev, (*r).addr_res, (*r).result_len, DMA_FROM_DEVICE); dma_unmap_sg((*ce).dev, (*areq).src, (*r).nr_sgs, DMA_TO_DEVICE); return -EINVAL; }
    (*cet).t_dlen = cpu_to_le32(if (*(*ce).variant).hash_t_dlen_in_bits { ((*areq).nbytes as u64 + j * 4) as u32 * 8 } else { (*areq).nbytes / 4 + j as u32 }); 0
}
pub unsafe fn sun8i_ce_hash_unprepare(areq: *mut ahash_request, _cet: *mut ce_task) { let r = ahash_request_ctx_dma(areq); let ce = (*crypto_ahash_ctx(crypto_ahash_reqtfm(areq))).ce; dma_unmap_single((*ce).dev, (*r).addr_pad, (*r).pad_len, DMA_TO_DEVICE); dma_unmap_single((*ce).dev, (*r).addr_res, (*r).result_len, DMA_FROM_DEVICE); dma_unmap_sg((*ce).dev, (*areq).src, (*r).nr_sgs, DMA_TO_DEVICE); }
pub unsafe fn sun8i_ce_hash_run(engine: *mut crypto_engine, async_req: *mut c_void) -> c_int { let areq = ahash_request_cast(async_req); let tfm = crypto_ahash_reqtfm(areq); let r = ahash_request_ctx_dma(areq); let ce = (*crypto_ahash_ctx(tfm)).ce; let chan = &mut (*ce).chanlist[(*r).flow]; let mut err = sun8i_ce_hash_prepare(areq, chan.tl); if err == 0 { err = sun8i_ce_run_task(ce, (*r).flow, crypto_ahash_alg_name(tfm)); } sun8i_ce_hash_unprepare(areq, chan.tl); if err == 0 { memcpy((*areq).result, (*r).result, crypto_ahash_digestsize(tfm)); } local_bh_disable(); crypto_finalize_hash_request(engine, async_req, err); local_bh_enable(); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
