// SPDX-License-Identifier: GPL-2.0-only
/*
 * Crypto acceleration support for Rockchip RK3288
 *
 * Copyright (c) 2015, Fuzhou Rockchip Electronics Co., Ltd
 *
 * Author: Zain Wang <zain.wang@rock-chips.com>
 *
 * Some ideas are from marvell/cesa.c and s5p-sss.c driver.
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * IC can not process zero message hash,
 * so we put the fixed hash out when met zero message.
 */

unsafe fn rk_ahash_need_fallback(req: *mut ahash_request) -> bool {
    let mut sg = (*req).src;
    while !sg.is_null() {
        if !is_aligned((*sg).offset, core::mem::size_of::<u32>()) || (*sg).length % 4 != 0 {
            return true;
        }
        sg = sg_next(sg);
    }
    false
}

unsafe fn rk_ahash_digest_fb(areq: *mut ahash_request) -> i32 {
    let rctx = ahash_request_ctx(areq);
    let tfm = crypto_ahash_reqtfm(areq);
    let tfmctx = crypto_ahash_ctx(tfm);
    let alg = crypto_ahash_alg(tfm);
    let algt = container_of(alg, rk_crypto_tmp, alg.hash.base);
    (*algt).stat_fb += 1;
    ahash_request_set_tfm(&mut (*rctx).fallback_req, (*tfmctx).fallback_tfm);
    ahash_request_set_callback(&mut (*rctx).fallback_req,
        (*areq).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP,
        (*areq).base.complete, (*areq).base.data);
    ahash_request_set_crypt(&mut (*rctx).fallback_req, (*areq).src,
        (*areq).result, (*areq).nbytes);
    crypto_ahash_digest(&mut (*rctx).fallback_req)
}

unsafe fn zero_message_process(req: *mut ahash_request) -> i32 {
    let tfm = crypto_ahash_reqtfm(req);
    let digest_size = crypto_ahash_digestsize(tfm);
    match digest_size {
        SHA1_DIGEST_SIZE => memcpy((*req).result, sha1_zero_message_hash, digest_size),
        SHA256_DIGEST_SIZE => memcpy((*req).result, sha256_zero_message_hash, digest_size),
        MD5_DIGEST_SIZE => memcpy((*req).result, md5_zero_message_hash, digest_size),
        _ => return -EINVAL,
    }
    0
}

unsafe fn rk_ahash_reg_init(req: *mut ahash_request, dev: *mut rk_crypto_info) {
    let rctx = ahash_request_ctx(req);
    let mut reg_status = CRYPTO_READ(dev, RK_CRYPTO_CTRL) | RK_CRYPTO_HASH_FLUSH | _SBF(0xffff, 16);
    CRYPTO_WRITE(dev, RK_CRYPTO_CTRL, reg_status);
    reg_status = CRYPTO_READ(dev, RK_CRYPTO_CTRL);
    reg_status &= !RK_CRYPTO_HASH_FLUSH;
    reg_status |= _SBF(0xffff, 16);
    CRYPTO_WRITE(dev, RK_CRYPTO_CTRL, reg_status);
    memset_io((*dev).reg.add(RK_CRYPTO_HASH_DOUT_0), 0, 32);
    CRYPTO_WRITE(dev, RK_CRYPTO_INTENA, RK_CRYPTO_HRDMA_ERR_ENA | RK_CRYPTO_HRDMA_DONE_ENA);
    CRYPTO_WRITE(dev, RK_CRYPTO_INTSTS, RK_CRYPTO_HRDMA_ERR_INT | RK_CRYPTO_HRDMA_DONE_INT);
    CRYPTO_WRITE(dev, RK_CRYPTO_HASH_CTRL, (*rctx).mode | RK_CRYPTO_HASH_SWAP_DO);
    CRYPTO_WRITE(dev, RK_CRYPTO_CONF, RK_CRYPTO_BYTESWAP_HRFIFO | RK_CRYPTO_BYTESWAP_BRFIFO | RK_CRYPTO_BYTESWAP_BTFIFO);
    CRYPTO_WRITE(dev, RK_CRYPTO_HASH_MSG_LEN, (*req).nbytes);
}

unsafe fn rk_ahash_init(req: *mut ahash_request) -> i32 {
    let rctx = ahash_request_ctx(req); let tfm = crypto_ahash_reqtfm(req); let ctx = crypto_ahash_ctx(tfm);
    ahash_request_set_tfm(&mut (*rctx).fallback_req, (*ctx).fallback_tfm);
    ahash_request_set_callback(&mut (*rctx).fallback_req, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP, (*req).base.complete, (*req).base.data);
    crypto_ahash_init(&mut (*rctx).fallback_req)
}

unsafe fn rk_ahash_update(req: *mut ahash_request) -> i32 {
    let rctx = ahash_request_ctx(req); let tfm = crypto_ahash_reqtfm(req); let ctx = crypto_ahash_ctx(tfm);
    ahash_request_set_tfm(&mut (*rctx).fallback_req, (*ctx).fallback_tfm);
    ahash_request_set_callback(&mut (*rctx).fallback_req, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP, (*req).base.complete, (*req).base.data);
    ahash_request_set_crypt(&mut (*rctx).fallback_req, (*req).src, core::ptr::null_mut(), (*req).nbytes);
    crypto_ahash_update(&mut (*rctx).fallback_req)
}

unsafe fn rk_ahash_final(req: *mut ahash_request) -> i32 {
    let rctx = ahash_request_ctx(req); let tfm = crypto_ahash_reqtfm(req); let ctx = crypto_ahash_ctx(tfm);
    ahash_request_set_tfm(&mut (*rctx).fallback_req, (*ctx).fallback_tfm);
    ahash_request_set_callback(&mut (*rctx).fallback_req, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP, (*req).base.complete, (*req).base.data);
    ahash_request_set_crypt(&mut (*rctx).fallback_req, core::ptr::null_mut(), (*req).result, 0);
    crypto_ahash_final(&mut (*rctx).fallback_req)
}

unsafe fn rk_ahash_finup(req: *mut ahash_request) -> i32 {
    let rctx = ahash_request_ctx(req); let tfm = crypto_ahash_reqtfm(req); let ctx = crypto_ahash_ctx(tfm);
    ahash_request_set_tfm(&mut (*rctx).fallback_req, (*ctx).fallback_tfm);
    ahash_request_set_callback(&mut (*rctx).fallback_req, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP, (*req).base.complete, (*req).base.data);
    ahash_request_set_crypt(&mut (*rctx).fallback_req, (*req).src, (*req).result, (*req).nbytes);
    crypto_ahash_finup(&mut (*rctx).fallback_req)
}

unsafe fn rk_ahash_import(req: *mut ahash_request, input: *const core::ffi::c_void) -> i32 {
    let rctx = ahash_request_ctx(req); let tfm = crypto_ahash_reqtfm(req); let ctx = crypto_ahash_ctx(tfm);
    ahash_request_set_tfm(&mut (*rctx).fallback_req, (*ctx).fallback_tfm);
    ahash_request_set_callback(&mut (*rctx).fallback_req, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP, (*req).base.complete, (*req).base.data);
    crypto_ahash_import(&mut (*rctx).fallback_req, input)
}

unsafe fn rk_ahash_export(req: *mut ahash_request, output: *mut core::ffi::c_void) -> i32 {
    let rctx = ahash_request_ctx(req); let tfm = crypto_ahash_reqtfm(req); let ctx = crypto_ahash_ctx(tfm);
    ahash_request_set_tfm(&mut (*rctx).fallback_req, (*ctx).fallback_tfm);
    ahash_request_set_callback(&mut (*rctx).fallback_req, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP, (*req).base.complete, (*req).base.data);
    crypto_ahash_export(&mut (*rctx).fallback_req, output)
}

unsafe fn rk_ahash_digest(req: *mut ahash_request) -> i32 {
    if rk_ahash_need_fallback(req) { return rk_ahash_digest_fb(req); }
    if (*req).nbytes == 0 { return zero_message_process(req); }
    let rctx = ahash_request_ctx(req); let dev = get_rk_crypto();
    (*rctx).dev = dev;
    crypto_transfer_hash_request_to_engine((*dev).engine, req)
}

unsafe fn crypto_ahash_dma_start(dev: *mut rk_crypto_info, sg: *mut scatterlist) {
    CRYPTO_WRITE(dev, RK_CRYPTO_HRDMAS, sg_dma_address(sg));
    CRYPTO_WRITE(dev, RK_CRYPTO_HRDMAL, sg_dma_len(sg) / 4);
    CRYPTO_WRITE(dev, RK_CRYPTO_CTRL, RK_CRYPTO_HASH_START | (RK_CRYPTO_HASH_START << 16));
}

unsafe fn rk_hash_prepare(_engine: *mut crypto_engine, breq: *mut core::ffi::c_void) -> i32 {
    let areq = container_of(breq, ahash_request, base); let rctx = ahash_request_ctx(areq); let rkc = (*rctx).dev;
    let ret = dma_map_sg((*rkc).dev, (*areq).src, sg_nents((*areq).src), DMA_TO_DEVICE);
    if ret <= 0 { return -EINVAL; } (*rctx).nrsg = ret; 0
}

unsafe fn rk_hash_unprepare(_engine: *mut crypto_engine, breq: *mut core::ffi::c_void) {
    let areq = container_of(breq, ahash_request, base); let rctx = ahash_request_ctx(areq); let rkc = (*rctx).dev;
    dma_unmap_sg((*rkc).dev, (*areq).src, sg_nents((*areq).src), DMA_TO_DEVICE);
}

unsafe fn rk_hash_run(engine: *mut crypto_engine, breq: *mut core::ffi::c_void) -> i32 {
    let areq = container_of(breq, ahash_request, base); let tfm = crypto_ahash_reqtfm(areq); let rctx = ahash_request_ctx(areq);
    let alg = crypto_ahash_alg(tfm); let algt = container_of(alg, rk_crypto_tmp, alg.hash.base); let mut sg = (*areq).src; let rkc = (*rctx).dev;
    let mut err = pm_runtime_resume_and_get((*rkc).dev); if err != 0 { return err; }
    err = rk_hash_prepare(engine, breq); if err != 0 { pm_runtime_put_autosuspend((*rkc).dev); return finalize_hash(engine, breq, err); }
    (*rctx).mode = 0; (*algt).stat_req += 1; (*rkc).nreq += 1;
    (*rctx).mode = match crypto_ahash_digestsize(tfm) { SHA1_DIGEST_SIZE => RK_CRYPTO_HASH_SHA1, SHA256_DIGEST_SIZE => RK_CRYPTO_HASH_SHA256, MD5_DIGEST_SIZE => RK_CRYPTO_HASH_MD5, _ => { err = -EINVAL; 0 } };
    if err != 0 { rk_hash_unprepare(engine, breq); pm_runtime_put_autosuspend((*rkc).dev); return finalize_hash(engine, breq, err); }
    rk_ahash_reg_init(areq, rkc);
    while !sg.is_null() {
        reinit_completion(&mut (*rkc).complete); (*rkc).status = 0; crypto_ahash_dma_start(rkc, sg);
        wait_for_completion_interruptible_timeout(&mut (*rkc).complete, msecs_to_jiffies(2000));
        if (*rkc).status == 0 { dev_err((*rkc).dev, "DMA timeout\0"); err = -EFAULT; break; }
        sg = sg_next(sg);
    }
    let mut v: u32 = 0;
    if err == 0 { err = readl_poll_timeout((*rkc).reg.add(RK_CRYPTO_HASH_STS), &mut v, v == 0, 10, 1000); }
    if err == 0 { for i in 0..(crypto_ahash_digestsize(tfm) / 4) { v = readl((*rkc).reg.add(RK_CRYPTO_HASH_DOUT_0 + i * 4)); put_unaligned_le32(v, (*areq).result.add(i * 4)); } }
    pm_runtime_put_autosuspend((*rkc).dev); rk_hash_unprepare(engine, breq); finalize_hash(engine, breq, err); 0
}

unsafe fn rk_hash_init_tfm(tfm: *mut crypto_ahash) -> i32 {
    let tctx = crypto_ahash_ctx(tfm);
    let alg_name = crypto_ahash_alg_name(tfm);
    let alg = crypto_ahash_alg(tfm);
    let algt = container_of(alg, rk_crypto_tmp, alg.hash.base);
    (*tctx).fallback_tfm = crypto_alloc_ahash(alg_name, 0, CRYPTO_ALG_NEED_FALLBACK);
    if is_err((*tctx).fallback_tfm) {
        dev_err((*algt).dev.dev, "Could not load fallback driver.\n");
        return ptr_err((*tctx).fallback_tfm);
    }
    crypto_ahash_set_reqsize(tfm, core::mem::size_of::<rk_ahash_rctx>() + crypto_ahash_reqsize((*tctx).fallback_tfm));
    0
}

unsafe fn rk_hash_exit_tfm(tfm: *mut crypto_ahash) {
    let tctx = crypto_ahash_ctx(tfm);
    crypto_free_ahash((*tctx).fallback_tfm);
}

// Algorithm registration objects retain the C layout and callbacks.
pub static mut rk_ahash_sha1: rk_crypto_tmp = rk_crypto_tmp::sha1(rk_ahash_init, rk_ahash_update, rk_ahash_final, rk_ahash_finup, rk_ahash_export, rk_ahash_import, rk_ahash_digest, rk_hash_init_tfm, rk_hash_exit_tfm, rk_hash_run);
pub static mut rk_ahash_sha256: rk_crypto_tmp = rk_crypto_tmp::sha256(rk_ahash_init, rk_ahash_update, rk_ahash_final, rk_ahash_finup, rk_ahash_export, rk_ahash_import, rk_ahash_digest, rk_hash_init_tfm, rk_hash_exit_tfm, rk_hash_run);
pub static mut rk_ahash_md5: rk_crypto_tmp = rk_crypto_tmp::md5(rk_ahash_init, rk_ahash_update, rk_ahash_final, rk_ahash_finup, rk_ahash_export, rk_ahash_import, rk_ahash_digest, rk_hash_init_tfm, rk_hash_exit_tfm, rk_hash_run);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
