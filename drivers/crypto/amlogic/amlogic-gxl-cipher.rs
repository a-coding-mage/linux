// SPDX-License-Identifier: GPL-2.0
/*
 * amlogic-cipher.c - hardware cryptographic offloader for Amlogic GXL SoC
 *
 * Copyright (C) 2018-2019 Corentin LABBE <clabbe@baylibre.com>
 *
 * This file add support for AES cipher with 128,192,256 bits keysize in
 * CBC and ECB mode.
 */

// Kernel and amlogic-gxl.h declarations are supplied by the surrounding crate.

unsafe fn get_engine_number(mc: *mut meson_dev) -> i32 {
    atomic_inc_return(&mut (*mc).flow) % MAXFLOW
}

unsafe fn meson_cipher_need_fallback(areq: *mut skcipher_request) -> bool {
    let mut src_sg = (*areq).src;
    let mut dst_sg = (*areq).dst;
    if (*areq).cryptlen == 0 { return true; }
    if sg_nents(src_sg) != sg_nents(dst_sg) { return true; }
    if sg_nents(src_sg) > MAXDESC - 3 || sg_nents(dst_sg) > MAXDESC - 3 { return true; }
    while !src_sg.is_null() && !dst_sg.is_null() {
        if (*src_sg).length % 16 != 0 || (*dst_sg).length % 16 != 0 { return true; }
        if (*src_sg).length != (*dst_sg).length { return true; }
        if !IS_ALIGNED((*src_sg).offset, core::mem::size_of::<u32>()) ||
           !IS_ALIGNED((*dst_sg).offset, core::mem::size_of::<u32>()) { return true; }
        src_sg = sg_next(src_sg); dst_sg = sg_next(dst_sg);
    }
    false
}

unsafe fn meson_cipher_do_fallback(areq: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(areq);
    let op = crypto_skcipher_ctx(tfm);
    let rctx = skcipher_request_ctx(areq);
    #[cfg(CONFIG_CRYPTO_DEV_AMLOGIC_GXL_DEBUG)] {
        let alg = crypto_skcipher_alg(tfm);
        let algt = container_of!(alg, meson_alg_template, alg.skcipher.base);
        (*algt).stat_fb += 1;
    }
    skcipher_request_set_tfm(&mut (*rctx).fallback_req, (*op).fallback_tfm);
    skcipher_request_set_callback(&mut (*rctx).fallback_req, (*areq).base.flags,
                                  (*areq).base.complete, (*areq).base.data);
    skcipher_request_set_crypt(&mut (*rctx).fallback_req, (*areq).src, (*areq).dst,
                               (*areq).cryptlen, (*areq).iv);
    if (*rctx).op_dir == MESON_DECRYPT {
        crypto_skcipher_decrypt(&mut (*rctx).fallback_req)
    } else { crypto_skcipher_encrypt(&mut (*rctx).fallback_req) }
}

unsafe fn meson_cipher(areq: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(areq);
    let op = crypto_skcipher_ctx(tfm);
    let rctx = skcipher_request_ctx(areq);
    let mc = (*op).mc;
    let alg = crypto_skcipher_alg(tfm);
    let algt = container_of!(alg, meson_alg_template, alg.skcipher.base);
    let flow = (*rctx).flow;
    let mut src_sg = (*areq).src; let mut dst_sg = (*areq).dst;
    let mut err = 0; let mut backup_iv: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut bkeyiv: *mut u8 = kzalloc(48, GFP_KERNEL | GFP_DMA);
    if bkeyiv.is_null() { return -ENOMEM; }
    memcpy(bkeyiv, (*op).key, (*op).keylen); let mut keyivlen = (*op).keylen;
    let ivsize = crypto_skcipher_ivsize(tfm);
    if !(*areq).iv.is_null() && ivsize > 0 {
        if ivsize > (*areq).cryptlen { err = -EINVAL; goto theend; }
        memcpy(bkeyiv.add(32), (*areq).iv, ivsize); keyivlen = 48;
        if (*rctx).op_dir == MESON_DECRYPT {
            backup_iv = kzalloc(ivsize, GFP_KERNEL);
            if backup_iv.is_null() { err = -ENOMEM; goto theend; }
            scatterwalk_map_and_copy(backup_iv, (*areq).src, (*areq).cryptlen - ivsize, ivsize, 0);
        }
    }
    if keyivlen == 24 { keyivlen = 32; }
    let phykeyiv = dma_map_single((*mc).dev, bkeyiv, keyivlen, DMA_TO_DEVICE);
    err = dma_mapping_error((*mc).dev, phykeyiv); if err != 0 { goto theend; }
    let mut tloffset = 0; let mut eat = 0; let mut i = 0;
    while keyivlen > eat {
        let desc = &mut (*mc).chanlist[flow as usize].tl[tloffset];
        memset(desc, 0, core::mem::size_of::<meson_desc>());
        let todo = core::cmp::min(keyivlen - eat, 16u32);
        (*desc).t_src = cpu_to_le32(phykeyiv + i * 16); (*desc).t_dst = cpu_to_le32(i * 16);
        (*desc).t_status = cpu_to_le32((MODE_KEY << 20) | DESC_OWN | 16);
        eat += todo; i += 1; tloffset += 1;
    }
    let nr_sgs = dma_map_sg((*mc).dev, (*areq).src, sg_nents((*areq).src),
                            if (*areq).src == (*areq).dst { DMA_BIDIRECTIONAL } else { DMA_TO_DEVICE });
    if nr_sgs == 0 || (*areq).src != (*areq).dst && nr_sgs > MAXDESC - 3 { err = -EINVAL; goto theend; }
    let nr_sgd = if (*areq).src == (*areq).dst { nr_sgs } else {
        let n = dma_map_sg((*mc).dev, (*areq).dst, sg_nents((*areq).dst), DMA_FROM_DEVICE);
        if n == 0 || n > MAXDESC - 3 { err = -EINVAL; goto theend; } n
    };
    let _ = nr_sgd;
    let mut len = (*areq).cryptlen;
    while !src_sg.is_null() {
        let desc = &mut (*mc).chanlist[flow as usize].tl[tloffset]; memset(desc, 0, core::mem::size_of::<meson_desc>());
        let todo = core::cmp::min(len, sg_dma_len(src_sg));
        let mut v = ((*op).keymode << 20) | DESC_OWN | todo | ((*algt).blockmode << 26);
        if (*rctx).op_dir != 0 { v |= DESC_ENCRYPTION; } len -= todo;
        (*desc).t_src = cpu_to_le32(sg_dma_address(src_sg)); (*desc).t_dst = cpu_to_le32(sg_dma_address(dst_sg));
        if sg_next(src_sg).is_null() { v |= DESC_LAST; } (*desc).t_status = cpu_to_le32(v);
        tloffset += 1; src_sg = sg_next(src_sg); dst_sg = sg_next(dst_sg);
    }
    reinit_completion(&mut (*mc).chanlist[flow as usize].complete); (*mc).chanlist[flow as usize].status = 0;
    writel((*mc).chanlist[flow as usize].t_phy | 2, (*mc).base.add(flow as usize * 4));
    wait_for_completion_interruptible_timeout(&mut (*mc).chanlist[flow as usize].complete, msecs_to_jiffies(500));
    if (*mc).chanlist[flow as usize].status == 0 { err = -EINVAL; }
    dma_unmap_single((*mc).dev, phykeyiv, keyivlen, DMA_TO_DEVICE);
    if (*areq).src == (*areq).dst { dma_unmap_sg((*mc).dev, (*areq).src, sg_nents((*areq).src), DMA_BIDIRECTIONAL); }
    else { dma_unmap_sg((*mc).dev, (*areq).src, sg_nents((*areq).src), DMA_TO_DEVICE); dma_unmap_sg((*mc).dev, (*areq).dst, sg_nents((*areq).dst), DMA_FROM_DEVICE); }
    if !(*areq).iv.is_null() && ivsize > 0 { if (*rctx).op_dir == MESON_DECRYPT { memcpy((*areq).iv, backup_iv, ivsize); } else { scatterwalk_map_and_copy((*areq).iv, (*areq).dst, (*areq).cryptlen - ivsize, ivsize, 0); } }
theend:
    kfree_sensitive(bkeyiv); kfree_sensitive(backup_iv); err
}

pub unsafe fn meson_handle_cipher_request(engine: *mut crypto_engine, areq: *mut core::ffi::c_void) -> i32 { let breq = container_of!(areq, skcipher_request, base); let err = meson_cipher(breq); local_bh_disable(); crypto_finalize_skcipher_request(engine, breq, err); local_bh_enable(); 0 }

pub unsafe fn meson_skdecrypt(areq: *mut skcipher_request) -> i32 { let tfm = crypto_skcipher_reqtfm(areq); let op = crypto_skcipher_ctx(tfm); let rctx = skcipher_request_ctx(areq); (*rctx).op_dir = MESON_DECRYPT; if meson_cipher_need_fallback(areq) { return meson_cipher_do_fallback(areq); } let e = get_engine_number((*op).mc); (*rctx).flow = e; crypto_transfer_skcipher_request_to_engine((*op).mc.as_ref().unwrap().chanlist[e as usize].engine, areq) }
pub unsafe fn meson_skencrypt(areq: *mut skcipher_request) -> i32 { let tfm = crypto_skcipher_reqtfm(areq); let op = crypto_skcipher_ctx(tfm); let rctx = skcipher_request_ctx(areq); (*rctx).op_dir = MESON_ENCRYPT; if meson_cipher_need_fallback(areq) { return meson_cipher_do_fallback(areq); } let e = get_engine_number((*op).mc); (*rctx).flow = e; crypto_transfer_skcipher_request_to_engine((*op).mc.as_ref().unwrap().chanlist[e as usize].engine, areq) }

pub unsafe fn meson_cipher_init(tfm: *mut crypto_tfm) -> i32 { let op = crypto_tfm_ctx(tfm); memset(op, 0, core::mem::size_of::<meson_cipher_tfm_ctx>()); let sktfm = __crypto_skcipher_cast(tfm); let alg = crypto_skcipher_alg(sktfm); let algt = container_of!(alg, meson_alg_template, alg.skcipher.base); (*op).mc = (*algt).mc; let name = crypto_tfm_alg_name(tfm); (*op).fallback_tfm = crypto_alloc_skcipher(name, 0, CRYPTO_ALG_NEED_FALLBACK); if IS_ERR((*op).fallback_tfm) { return PTR_ERR((*op).fallback_tfm); } crypto_skcipher_set_reqsize(sktfm, core::mem::size_of::<meson_cipher_req_ctx>() + crypto_skcipher_reqsize((*op).fallback_tfm)); 0 }
pub unsafe fn meson_cipher_exit(tfm: *mut crypto_tfm) { let op = crypto_tfm_ctx(tfm); kfree_sensitive((*op).key); crypto_free_skcipher((*op).fallback_tfm); }

pub unsafe fn meson_aes_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: u32) -> i32 { let op = crypto_skcipher_ctx(tfm); (*op).keymode = match keylen { 16 => MODE_AES_128, 24 => MODE_AES_192, 32 => MODE_AES_256, _ => return -EINVAL }; kfree_sensitive((*op).key); (*op).keylen = keylen; (*op).key = kmemdup(key, keylen, GFP_KERNEL | GFP_DMA); if (*op).key.is_null() { return -ENOMEM; } crypto_skcipher_setkey((*op).fallback_tfm, key, keylen) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
