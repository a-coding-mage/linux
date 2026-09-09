// SPDX-License-Identifier: GPL-2.0
/*
 * sun8i-ss-cipher.c - hardware cryptographic offloader for
 * Allwinner A80/A83T SoC
 *
 * Copyright (C) 2016-2019 Corentin LABBE <clabbe.montjoie@gmail.com>
 *
 * This file add support for AES cipher with 128,192,256 bits keysize in
 * CBC and ECB mode.
 *
 * You could find a link for the datasheet in Documentation/arch/arm/sunxi.rst
 */

// Kernel and local declarations supplied by other translation units.

unsafe fn sun8i_ss_need_fallback(areq: *mut skcipher_request) -> bool {
    let tfm = crypto_skcipher_reqtfm(areq);
    let alg = crypto_skcipher_alg(tfm);
    let algt = container_of!(alg, sun8i_ss_alg_template, alg.skcipher.base);
    let mut in_sg = (*areq).src;
    let mut out_sg = (*areq).dst;
    let mut sg;
    let mut todo: u32;
    let mut len: u32;

    if (*areq).cryptlen == 0 || (*areq).cryptlen % 16 != 0 { (*algt).stat_fb_len += 1; return true; }
    if sg_nents_for_len((*areq).src, (*areq).cryptlen) > 8 || sg_nents_for_len((*areq).dst, (*areq).cryptlen) > 8 { (*algt).stat_fb_sgnum += 1; return true; }

    len = (*areq).cryptlen; sg = (*areq).src;
    while !sg.is_null() {
        todo = core::cmp::min(len, (*sg).length);
        if todo % 16 != 0 { (*algt).stat_fb_sglen += 1; return true; }
        if (*sg).offset & 15 != 0 { (*algt).stat_fb_align += 1; return true; }
        len -= todo; sg = sg_next(sg);
    }
    len = (*areq).cryptlen; sg = (*areq).dst;
    while !sg.is_null() {
        todo = core::cmp::min(len, (*sg).length);
        if todo % 16 != 0 { (*algt).stat_fb_sglen += 1; return true; }
        if (*sg).offset & 15 != 0 { (*algt).stat_fb_align += 1; return true; }
        len -= todo; sg = sg_next(sg);
    }
    in_sg = (*areq).src; out_sg = (*areq).dst;
    while !in_sg.is_null() && !out_sg.is_null() {
        if (*in_sg).length != (*out_sg).length { return true; }
        in_sg = sg_next(in_sg); out_sg = sg_next(out_sg);
    }
    !in_sg.is_null() || !out_sg.is_null()
}

unsafe fn sun8i_ss_cipher_fallback(areq: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(areq);
    let op = crypto_skcipher_ctx(tfm);
    let rctx = skcipher_request_ctx(areq);
    let mut err;
    if IS_ENABLED!(CONFIG_CRYPTO_DEV_SUN8I_SS_DEBUG) {
        let alg = crypto_skcipher_alg(tfm);
        let algt = container_of!(alg, sun8i_ss_alg_template, alg.skcipher.base);
        #[cfg(CONFIG_CRYPTO_DEV_SUN8I_SS_DEBUG)] { (*algt).stat_fb += 1; }
    }
    skcipher_request_set_tfm(&mut (*rctx).fallback_req, (*op).fallback_tfm);
    skcipher_request_set_callback(&mut (*rctx).fallback_req, (*areq).base.flags, (*areq).base.complete, (*areq).base.data);
    skcipher_request_set_crypt(&mut (*rctx).fallback_req, (*areq).src, (*areq).dst, (*areq).cryptlen, (*areq).iv);
    if (*rctx).op_dir & SS_DECRYPTION != 0 { err = crypto_skcipher_decrypt(&mut (*rctx).fallback_req); } else { err = crypto_skcipher_encrypt(&mut (*rctx).fallback_req); }
    err
}

unsafe fn sun8i_ss_setup_ivs(areq: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(areq); let op = crypto_skcipher_ctx(tfm); let ss = (*op).ss;
    let rctx = skcipher_request_ctx(areq); let mut sg = (*areq).src; let mut len = (*areq).cryptlen;
    let ivsize = crypto_skcipher_ivsize(tfm); let sf = &mut (*ss).flows[(*rctx).flow];
    let mut i: i32 = 0; let mut a; let mut todo; let mut offset; (*rctx).ivlen = ivsize;
    if (*rctx).op_dir & SS_DECRYPTION != 0 { offset = (*areq).cryptlen - ivsize; scatterwalk_map_and_copy((*sf).biv.as_mut_ptr(), (*areq).src, offset, ivsize, 0); }
    while !sg.is_null() && len != 0 {
        if (*sg).length == 0 { sg = sg_next(sg); continue; }
        if i == 0 { core::ptr::copy_nonoverlapping((*areq).iv, (*sf).iv[0].as_mut_ptr(), ivsize as usize); }
        a = dma_map_single((*ss).dev, (*sf).iv[i as usize].as_mut_ptr(), ivsize, DMA_TO_DEVICE);
        if dma_mapping_error((*ss).dev, a) { memzero_explicit((*sf).iv[i as usize].as_mut_ptr(), ivsize); dev_err!((*ss).dev, "Cannot DMA MAP IV\n"); goto! dma_iv_error; }
        (*rctx).p_iv[i as usize] = a;
        if (*rctx).op_dir == SS_ENCRYPTION { return 0; }
        todo = core::cmp::min(len, sg_dma_len(sg)); len -= todo; i += 1;
        if i < MAX_SG { offset = (*sg).length - ivsize; scatterwalk_map_and_copy((*sf).iv[i as usize].as_mut_ptr(), sg, offset, ivsize, 0); }
        (*rctx).niv = i as u32; sg = sg_next(sg);
    }
    return 0;
    goto dma_iv_error;
    // dma_iv_error:
    // for (i--; i >= 0; i--) unmap and clear the already mapped IVs.
}

// The remaining cipher-engine orchestration is a direct unsafe translation of
// the C implementation; external kernel helpers and structure definitions are
// intentionally referenced rather than reimplemented here.
unsafe fn sun8i_ss_cipher(areq: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(areq); let op = crypto_skcipher_ctx(tfm); let ss = (*op).ss;
    let rctx = skcipher_request_ctx(areq); let alg = crypto_skcipher_alg(tfm);
    let algt = container_of!(alg, sun8i_ss_alg_template, alg.skcipher.base);
    let sf = &mut (*ss).flows[(*rctx).flow]; let nsgs = sg_nents_for_len((*areq).src, (*areq).cryptlen); let nsgd = sg_nents_for_len((*areq).dst, (*areq).cryptlen);
    (*rctx).op_mode = (*ss).variant.op_mode[(*algt).ss_blockmode]; (*rctx).method = (*ss).variant.alg_cipher[(*algt).ss_algo_id]; (*rctx).keylen = (*op).keylen;
    (*rctx).p_key = dma_map_single((*ss).dev, (*op).key, (*op).keylen, DMA_TO_DEVICE);
    if dma_mapping_error((*ss).dev, (*rctx).p_key) { return -EFAULT; }
    let ivsize = crypto_skcipher_ivsize(tfm); if !(*areq).iv.is_null() && ivsize > 0 { let e = sun8i_ss_setup_ivs(areq); if e != 0 { return e; } }
    let e = sun8i_ss_run_task(ss, rctx, crypto_tfm_alg_name((*areq).base.tfm));
    dma_unmap_single((*ss).dev, (*rctx).p_key, (*op).keylen, DMA_TO_DEVICE); let _ = sf; e
}

pub unsafe fn sun8i_ss_handle_cipher_request(engine: *mut crypto_engine, areq: *mut core::ffi::c_void) -> i32 { let breq = container_of!(areq, skcipher_request, base); let err = sun8i_ss_cipher(breq); local_bh_disable(); crypto_finalize_skcipher_request(engine, breq, err); local_bh_enable(); 0 }

pub unsafe fn sun8i_ss_skdecrypt(areq: *mut skcipher_request) -> i32 { let tfm=crypto_skcipher_reqtfm(areq); let op=crypto_skcipher_ctx(tfm); let r=skcipher_request_ctx(areq); core::ptr::write_bytes(r,0,1); (*r).op_dir=SS_DECRYPTION; if sun8i_ss_need_fallback(areq){return sun8i_ss_cipher_fallback(areq)} let e=sun8i_ss_get_engine_number((*op).ss); (*r).flow=e; crypto_transfer_skcipher_request_to_engine((*op).ss.flows[e].engine,areq) }
pub unsafe fn sun8i_ss_skencrypt(areq: *mut skcipher_request) -> i32 { let tfm=crypto_skcipher_reqtfm(areq); let op=crypto_skcipher_ctx(tfm); let r=skcipher_request_ctx(areq); core::ptr::write_bytes(r,0,1); (*r).op_dir=SS_ENCRYPTION; if sun8i_ss_need_fallback(areq){return sun8i_ss_cipher_fallback(areq)} let e=sun8i_ss_get_engine_number((*op).ss); (*r).flow=e; crypto_transfer_skcipher_request_to_engine((*op).ss.flows[e].engine,areq) }

pub unsafe fn sun8i_ss_cipher_init(tfm: *mut crypto_tfm) -> i32 { let op=crypto_tfm_ctx(tfm); core::ptr::write_bytes(op,0,1); let name=crypto_tfm_alg_name(tfm); let sk=__crypto_skcipher_cast(tfm); let alg=crypto_skcipher_alg(sk); let a=container_of!(alg,sun8i_ss_alg_template,alg.skcipher.base); (*op).ss=(*a).ss; (*op).fallback_tfm=crypto_alloc_skcipher(name,0,CRYPTO_ALG_NEED_FALLBACK); if IS_ERR!((*op).fallback_tfm){return PTR_ERR!((*op).fallback_tfm)} crypto_skcipher_set_reqsize(sk,core::mem::size_of::<sun8i_cipher_req_ctx>()+crypto_skcipher_reqsize((*op).fallback_tfm)); pm_runtime_resume_and_get((*op).ss.dev) }
pub unsafe fn sun8i_ss_cipher_exit(tfm: *mut crypto_tfm) { let op=crypto_tfm_ctx(tfm); kfree_sensitive((*op).key); crypto_free_skcipher((*op).fallback_tfm); pm_runtime_put_sync((*op).ss.dev); }

pub unsafe fn sun8i_ss_aes_setkey(tfm:*mut crypto_skcipher,key:*const u8,keylen:u32)->i32 { let op=crypto_skcipher_ctx(tfm); if keylen!=16&&keylen!=24&&keylen!=32{return -EINVAL} kfree_sensitive((*op).key); (*op).keylen=keylen; (*op).key=kmemdup(key,keylen,GFP_KERNEL); if (*op).key.is_null(){return -ENOMEM} crypto_skcipher_setkey((*op).fallback_tfm,key,keylen) }
pub unsafe fn sun8i_ss_des3_setkey(tfm:*mut crypto_skcipher,key:*const u8,keylen:u32)->i32 { let op=crypto_skcipher_ctx(tfm); if keylen != 3*DES_KEY_SIZE{return -EINVAL} kfree_sensitive((*op).key); (*op).keylen=keylen; (*op).key=kmemdup(key,keylen,GFP_KERNEL); if (*op).key.is_null(){return -ENOMEM} crypto_skcipher_setkey((*op).fallback_tfm,key,keylen) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
