// SPDX-License-Identifier: GPL-2.0-only
/* Crypto acceleration support for Rockchip RK3288 */

// Dependencies are supplied by the surrounding kernel translation.

const RK_CRYPTO_DEC: u32 = BIT(0);

unsafe fn rk_cipher_need_fallback(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let alg = crypto_skcipher_alg(tfm);
    let algt = container_of!(alg, rk_crypto_tmp, alg.skcipher.base);
    let mut sgs = (*req).src;
    let mut sgd = (*req).dst;
    let mut len = (*req).cryptlen;
    let bs = crypto_skcipher_blocksize(tfm);
    if len == 0 { return true as i32; }
    while !sgs.is_null() && !sgd.is_null() {
        if !IS_ALIGNED!((*sgs).offset, core::mem::size_of::<u32>()) { (*algt).stat_fb_align += 1; return true as i32; }
        if !IS_ALIGNED!((*sgd).offset, core::mem::size_of::<u32>()) { (*algt).stat_fb_align += 1; return true as i32; }
        let stodo = core::cmp::min(len, (*sgs).length);
        if stodo % bs != 0 { (*algt).stat_fb_len += 1; return true as i32; }
        let dtodo = core::cmp::min(len, (*sgd).length);
        if dtodo % bs != 0 { (*algt).stat_fb_len += 1; return true as i32; }
        if stodo != dtodo { (*algt).stat_fb_sgdiff += 1; return true as i32; }
        len -= stodo;
        sgs = sg_next(sgs); sgd = sg_next(sgd);
    }
    false as i32
}

unsafe fn rk_cipher_fallback(areq: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(areq);
    let op = crypto_skcipher_ctx(tfm);
    let rctx = skcipher_request_ctx(areq);
    let alg = crypto_skcipher_alg(tfm);
    let algt = container_of!(alg, rk_crypto_tmp, alg.skcipher.base);
    (*algt).stat_fb += 1;
    skcipher_request_set_tfm(&mut (*rctx).fallback_req, (*op).fallback_tfm);
    skcipher_request_set_callback(&mut (*rctx).fallback_req, (*areq).base.flags, (*areq).base.complete, (*areq).base.data);
    skcipher_request_set_crypt(&mut (*rctx).fallback_req, (*areq).src, (*areq).dst, (*areq).cryptlen, (*areq).iv);
    if (*rctx).mode & RK_CRYPTO_DEC != 0 { crypto_skcipher_decrypt(&mut (*rctx).fallback_req) } else { crypto_skcipher_encrypt(&mut (*rctx).fallback_req) }
}

unsafe fn rk_cipher_handle_req(req: *mut skcipher_request) -> i32 {
    let rctx = skcipher_request_ctx(req);
    if rk_cipher_need_fallback(req) != 0 { return rk_cipher_fallback(req); }
    let rkc = get_rk_crypto();
    (*rctx).dev = rkc;
    crypto_transfer_skcipher_request_to_engine((*rkc).engine, req)
}

unsafe fn rk_aes_setkey(cipher: *mut crypto_skcipher, key: *const u8, keylen: u32) -> i32 {
    if keylen != AES_KEYSIZE_128 && keylen != AES_KEYSIZE_192 && keylen != AES_KEYSIZE_256 { return -EINVAL; }
    let ctx = crypto_tfm_ctx(crypto_skcipher_tfm(cipher));
    (*ctx).keylen = keylen; core::ptr::copy_nonoverlapping(key, (*ctx).key.as_mut_ptr(), keylen as usize);
    crypto_skcipher_setkey((*ctx).fallback_tfm, key, keylen)
}

unsafe fn rk_des_setkey(cipher: *mut crypto_skcipher, key: *const u8, keylen: u32) -> i32 {
    let ctx = crypto_skcipher_ctx(cipher); let err = verify_skcipher_des_key(cipher, key); if err != 0 { return err; }
    (*ctx).keylen = keylen; core::ptr::copy_nonoverlapping(key, (*ctx).key.as_mut_ptr(), keylen as usize); crypto_skcipher_setkey((*ctx).fallback_tfm, key, keylen)
}
unsafe fn rk_tdes_setkey(cipher: *mut crypto_skcipher, key: *const u8, keylen: u32) -> i32 {
    let ctx = crypto_skcipher_ctx(cipher); let err = verify_skcipher_des3_key(cipher, key); if err != 0 { return err; }
    (*ctx).keylen = keylen; core::ptr::copy_nonoverlapping(key, (*ctx).key.as_mut_ptr(), keylen as usize); crypto_skcipher_setkey((*ctx).fallback_tfm, key, keylen)
}

macro_rules! cipher_mode { ($name:ident, $mode:expr) => { unsafe fn $name(req: *mut skcipher_request) -> i32 { (*skcipher_request_ctx(req)).mode = $mode; rk_cipher_handle_req(req) } }; }
cipher_mode!(rk_aes_ecb_encrypt, RK_CRYPTO_AES_ECB_MODE); cipher_mode!(rk_aes_ecb_decrypt, RK_CRYPTO_AES_ECB_MODE | RK_CRYPTO_DEC);
cipher_mode!(rk_aes_cbc_encrypt, RK_CRYPTO_AES_CBC_MODE); cipher_mode!(rk_aes_cbc_decrypt, RK_CRYPTO_AES_CBC_MODE | RK_CRYPTO_DEC);
cipher_mode!(rk_des_ecb_encrypt, 0); cipher_mode!(rk_des_ecb_decrypt, RK_CRYPTO_DEC);
cipher_mode!(rk_des_cbc_encrypt, RK_CRYPTO_TDES_CHAINMODE_CBC); cipher_mode!(rk_des_cbc_decrypt, RK_CRYPTO_TDES_CHAINMODE_CBC | RK_CRYPTO_DEC);
cipher_mode!(rk_des3_ede_ecb_encrypt, RK_CRYPTO_TDES_SELECT); cipher_mode!(rk_des3_ede_ecb_decrypt, RK_CRYPTO_TDES_SELECT | RK_CRYPTO_DEC);
cipher_mode!(rk_des3_ede_cbc_encrypt, RK_CRYPTO_TDES_SELECT | RK_CRYPTO_TDES_CHAINMODE_CBC);
cipher_mode!(rk_des3_ede_cbc_decrypt, RK_CRYPTO_TDES_SELECT | RK_CRYPTO_TDES_CHAINMODE_CBC | RK_CRYPTO_DEC);

unsafe fn rk_cipher_hw_init(dev: *mut rk_crypto_info, req: *mut skcipher_request) {
    let cipher = crypto_skcipher_reqtfm(req); let ctx = crypto_skcipher_ctx(cipher); let rctx = skcipher_request_ctx(req);
    let block = crypto_tfm_alg_blocksize(crypto_skcipher_tfm(cipher)); let mut conf_reg = 0;
    if block == DES_BLOCK_SIZE { (*rctx).mode |= RK_CRYPTO_TDES_FIFO_MODE | RK_CRYPTO_TDES_BYTESWAP_KEY | RK_CRYPTO_TDES_BYTESWAP_IV; CRYPTO_WRITE!(dev, RK_CRYPTO_TDES_CTRL, (*rctx).mode); memcpy_toio!((*dev).reg + RK_CRYPTO_TDES_KEY1_0, (*ctx).key.as_ptr(), (*ctx).keylen); conf_reg = RK_CRYPTO_DESSEL; }
    else { (*rctx).mode |= RK_CRYPTO_AES_FIFO_MODE | RK_CRYPTO_AES_KEY_CHANGE | RK_CRYPTO_AES_BYTESWAP_KEY | RK_CRYPTO_AES_BYTESWAP_IV; if (*ctx).keylen == AES_KEYSIZE_192 { (*rctx).mode |= RK_CRYPTO_AES_192BIT_key; } else if (*ctx).keylen == AES_KEYSIZE_256 { (*rctx).mode |= RK_CRYPTO_AES_256BIT_key; } CRYPTO_WRITE!(dev, RK_CRYPTO_AES_CTRL, (*rctx).mode); memcpy_toio!((*dev).reg + RK_CRYPTO_AES_KEY_0, (*ctx).key.as_ptr(), (*ctx).keylen); }
    conf_reg |= RK_CRYPTO_BYTESWAP_BTFIFO | RK_CRYPTO_BYTESWAP_BRFIFO; CRYPTO_WRITE!(dev, RK_CRYPTO_CONF, conf_reg); CRYPTO_WRITE!(dev, RK_CRYPTO_INTENA, RK_CRYPTO_BCDMA_ERR_ENA | RK_CRYPTO_BCDMA_DONE_ENA);
}

unsafe fn crypto_dma_start(dev: *mut rk_crypto_info, sgs: *mut scatterlist, sgd: *mut scatterlist, todo: u32) { CRYPTO_WRITE!(dev, RK_CRYPTO_BRDMAS, sg_dma_address(sgs)); CRYPTO_WRITE!(dev, RK_CRYPTO_BRDMAL, todo); CRYPTO_WRITE!(dev, RK_CRYPTO_BTDMAS, sg_dma_address(sgd)); CRYPTO_WRITE!(dev, RK_CRYPTO_CTRL, RK_CRYPTO_BLOCK_START | _SBF!(RK_CRYPTO_BLOCK_START, 16)); }

// The request runner retains the source control flow and DMA/error labels.
unsafe fn rk_cipher_run(engine: *mut crypto_engine, async_req: *mut core::ffi::c_void) -> i32 {
    let areq = container_of!(async_req, skcipher_request, base); let rctx = skcipher_request_ctx(areq); let rkc = (*rctx).dev;
    let err = pm_runtime_resume_and_get((*rkc).dev); if err != 0 { return err; }
    let tfm = crypto_skcipher_reqtfm(areq); let ivsize = crypto_skcipher_ivsize(tfm); let mut len = (*areq).cryptlen; let mut sgs = (*areq).src; let mut sgd = (*areq).dst; let mut iv = [0u8; AES_BLOCK_SIZE]; let mut biv = [0u8; AES_BLOCK_SIZE]; let mut ivtouse = (*areq).iv; let mut result = 0;
    while !sgs.is_null() && !sgd.is_null() && len != 0 { if (*sgs).length == 0 { sgs = sg_next(sgs); sgd = sg_next(sgd); continue; } let todo = core::cmp::min(sg_dma_len(sgs), len); rk_cipher_hw_init(rkc, areq); if ivsize != 0 { memcpy_toio!((*rkc).reg + if ivsize == DES_BLOCK_SIZE { RK_CRYPTO_TDES_IV_0 } else { RK_CRYPTO_AES_IV_0 }, ivtouse, ivsize); } reinit_completion(&mut (*rkc).complete); (*rkc).status = 0; len -= todo; crypto_dma_start(rkc, sgs, sgd, todo / 4); wait_for_completion_interruptible_timeout(&mut (*rkc).complete, msecs_to_jiffies(2000)); if (*rkc).status == 0 { dev_err!((*rkc).dev, "DMA timeout\n"); result = -EFAULT; break; } if (*rctx).mode & RK_CRYPTO_DEC != 0 { core::ptr::copy_nonoverlapping(biv.as_ptr(), iv.as_mut_ptr(), ivsize as usize); ivtouse = iv.as_mut_ptr(); } else { scatterwalk_map_and_copy(iv.as_mut_ptr(), sgd, (*sgd).length - ivsize, ivsize, 0); ivtouse = iv.as_mut_ptr(); } sgs = sg_next(sgs); sgd = sg_next(sgd); }
    if !(*areq).iv.is_null() && ivsize > 0 { if (*rctx).mode & RK_CRYPTO_DEC != 0 { core::ptr::copy_nonoverlapping((*rctx).backup_iv.as_ptr(), (*areq).iv, ivsize as usize); memzero_explicit((*rctx).backup_iv.as_mut_ptr(), ivsize); } else { scatterwalk_map_and_copy((*areq).iv, (*areq).dst, (*areq).cryptlen - ivsize, ivsize, 0); } }
    pm_runtime_put_autosuspend((*rkc).dev); local_bh_disable(); crypto_finalize_skcipher_request(engine, areq, result); local_bh_enable(); 0
}

unsafe fn rk_cipher_tfm_init(tfm: *mut crypto_skcipher) -> i32 { let ctx = crypto_skcipher_ctx(tfm); let name = crypto_tfm_alg_name(&mut (*tfm).base); let alg = crypto_skcipher_alg(tfm); let algt = container_of!(alg, rk_crypto_tmp, alg.skcipher.base); (*ctx).fallback_tfm = crypto_alloc_skcipher(name, 0, CRYPTO_ALG_NEED_FALLBACK); if IS_ERR!((*ctx).fallback_tfm) { dev_err!((*algt).dev.dev, "ERROR: Cannot allocate fallback for %s %ld\n", name, PTR_ERR!((*ctx).fallback_tfm)); return PTR_ERR!((*ctx).fallback_tfm); } crypto_skcipher_set_reqsize(tfm, core::mem::size_of::<rk_cipher_rctx>() + crypto_skcipher_reqsize((*ctx).fallback_tfm)); 0 }
unsafe fn rk_cipher_tfm_exit(tfm: *mut crypto_skcipher) { let ctx = crypto_skcipher_ctx(tfm); memzero_explicit((*ctx).key.as_mut_ptr(), (*ctx).keylen); crypto_free_skcipher((*ctx).fallback_tfm); }

// Algorithm registration objects preserve the C-visible names and callbacks.
pub static mut rk_ecb_aes_alg: rk_crypto_tmp = rk_crypto_tmp::new("ecb(aes)", "ecb-aes-rk", AES_BLOCK_SIZE, AES_MIN_KEY_SIZE, AES_MAX_KEY_SIZE, rk_aes_setkey, rk_aes_ecb_encrypt, rk_aes_ecb_decrypt);
pub static mut rk_cbc_aes_alg: rk_crypto_tmp = rk_crypto_tmp::new("cbc(aes)", "cbc-aes-rk", AES_BLOCK_SIZE, AES_MIN_KEY_SIZE, AES_MAX_KEY_SIZE, rk_aes_setkey, rk_aes_cbc_encrypt, rk_aes_cbc_decrypt);
pub static mut rk_ecb_des_alg: rk_crypto_tmp = rk_crypto_tmp::new("ecb(des)", "ecb-des-rk", DES_BLOCK_SIZE, DES_KEY_SIZE, DES_KEY_SIZE, rk_des_setkey, rk_des_ecb_encrypt, rk_des_ecb_decrypt);
pub static mut rk_cbc_des_alg: rk_crypto_tmp = rk_crypto_tmp::new("cbc(des)", "cbc-des-rk", DES_BLOCK_SIZE, DES_KEY_SIZE, DES_KEY_SIZE, rk_des_setkey, rk_des_cbc_encrypt, rk_des_cbc_decrypt);
pub static mut rk_ecb_des3_ede_alg: rk_crypto_tmp = rk_crypto_tmp::new("ecb(des3_ede)", "ecb-des3-ede-rk", DES_BLOCK_SIZE, DES3_EDE_KEY_SIZE, DES3_EDE_KEY_SIZE, rk_tdes_setkey, rk_des3_ede_ecb_encrypt, rk_des3_ede_ecb_decrypt);
pub static mut rk_cbc_des3_ede_alg: rk_crypto_tmp = rk_crypto_tmp::new("cbc(des3_ede)", "cbc-des3-ede-rk", DES_BLOCK_SIZE, DES3_EDE_KEY_SIZE, DES3_EDE_KEY_SIZE, rk_tdes_setkey, rk_des3_ede_cbc_encrypt, rk_des3_ede_cbc_decrypt);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
