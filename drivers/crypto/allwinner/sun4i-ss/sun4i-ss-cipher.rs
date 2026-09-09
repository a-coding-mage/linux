// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sun4i-ss-cipher.c - hardware cryptographic accelerator for Allwinner A20 SoC
 *
 * Copyright (C) 2013-2015 Corentin LABBE <clabbe.montjoie@gmail.com>
 *
 * This file add support for AES cipher with 128,192,256 bits
 * keysize in CBC and ECB mode.
 * Add support also for DES and 3DES in CBC and ECB mode.
 *
 * You could find the datasheet in Documentation/arch/arm/sunxi.rst
 */

unsafe fn sun4i_ss_opti_poll(areq: *mut skcipher_request) -> c_int {
    let tfm = crypto_skcipher_reqtfm(areq);
    let op = crypto_skcipher_ctx(tfm);
    let ss = (*op).ss;
    let ivsize = crypto_skcipher_ivsize(tfm);
    let ctx = skcipher_request_ctx(areq);
    let mode = (*ctx).mode;
    // when activating SS, the default FIFO space is SS_RX_DEFAULT(32)
    let mut rx_cnt: u32 = SS_RX_DEFAULT;
    let mut tx_cnt: u32 = 0;
    let mut spaces: u32;
    let mut v: u32;
    let mut err: c_int = 0;
    let mut i: c_uint;
    let mut ileft = (*areq).cryptlen;
    let mut oleft = (*areq).cryptlen;
    let mut todo: c_uint;
    let mut pi: c_ulong = 0;
    let mut po: c_ulong = 0; // progress for in and out
    let mut miter_err: bool;
    let mut mi: sg_mapping_iter;
    let mut mo: sg_mapping_iter;
    let mut oi: c_uint;
    let mut oo: c_uint;
    let mut flags: c_ulong;
    let alg = crypto_skcipher_alg(tfm);
    let mut _algt: *mut sun4i_ss_alg_template;

    if (*areq).cryptlen == 0 { return 0; }
    if (*areq).src.is_null() || (*areq).dst.is_null() {
        dev_err_ratelimited((*ss).dev, "ERROR: Some SGs are NULL\n");
        return -EINVAL;
    }
    if !(*areq).iv.is_null() && ivsize > 0 && mode & SS_DECRYPTION != 0 {
        scatterwalk_map_and_copy((*ctx).backup_iv.as_mut_ptr(), (*areq).src,
            (*areq).cryptlen - ivsize, ivsize, 0);
    }
    // CONFIG_CRYPTO_DEV_SUN4I_SS_DEBUG conditionally updates algt statistics.
    spin_lock_irqsave(&mut (*ss).slock, &mut flags);
    i = 0;
    while i < (*op).keylen / 4 {
        writesl((*ss).base.add(SS_KEY0 as usize + i as usize * 4), (*op).key.as_ptr().add(i as usize), 1);
        i += 1;
    }
    if !(*areq).iv.is_null() {
        i = 0;
        while i < 4 && i < ivsize / 4 {
            v = *( (*areq).iv.add(i as usize * 4) as *const u32 );
            writesl((*ss).base.add(SS_IV0 as usize + i as usize * 4), &v, 1);
            i += 1;
        }
    }
    writel(mode, (*ss).base.add(SS_CTL as usize));
    ileft /= 4; oleft /= 4; oi = 0; oo = 0;
    loop {
        if ileft != 0 {
            sg_miter_start(&mut mi, (*areq).src, sg_nents((*areq).src), SG_MITER_FROM_SG | SG_MITER_ATOMIC);
            if pi != 0 { sg_miter_skip(&mut mi, pi); }
            miter_err = sg_miter_next(&mut mi);
            if !miter_err || mi.addr.is_null() { err = -EINVAL; goto_release_ss(ss, &mut flags); return err; }
            todo = core::cmp::min(rx_cnt, ileft);
            todo = core::cmp::min(todo, ((mi.length - oi) / 4) as c_uint);
            if todo != 0 { ileft -= todo; writesl((*ss).base.add(SS_RXFIFO as usize), mi.addr.add(oi as usize), todo as usize); oi += todo * 4; }
            if oi == mi.length { pi += mi.length as c_ulong; oi = 0; }
            sg_miter_stop(&mut mi);
        }
        spaces = readl((*ss).base.add(SS_FCSR as usize));
        rx_cnt = SS_RXFIFO_SPACES(spaces); tx_cnt = SS_TXFIFO_SPACES(spaces);
        sg_miter_start(&mut mo, (*areq).dst, sg_nents((*areq).dst), SG_MITER_TO_SG | SG_MITER_ATOMIC);
        if po != 0 { sg_miter_skip(&mut mo, po); }
        miter_err = sg_miter_next(&mut mo);
        if !miter_err || mo.addr.is_null() { err = -EINVAL; goto_release_ss(ss, &mut flags); return err; }
        todo = core::cmp::min(tx_cnt, oleft); todo = core::cmp::min(todo, ((mo.length - oo) / 4) as c_uint);
        if todo != 0 { oleft -= todo; readsl((*ss).base.add(SS_TXFIFO as usize), mo.addr.add(oo as usize), todo as usize); oo += todo * 4; }
        if oo == mo.length { oo = 0; po += mo.length as c_ulong; }
        sg_miter_stop(&mut mo);
        if oleft == 0 { break; }
    }
    if !(*areq).iv.is_null() {
        if mode & SS_DECRYPTION != 0 { memcpy((*areq).iv, (*ctx).backup_iv.as_ptr(), ivsize); memzero_explicit((*ctx).backup_iv.as_mut_ptr(), ivsize); }
        else { scatterwalk_map_and_copy((*areq).iv, (*areq).dst, (*areq).cryptlen - ivsize, ivsize, 0); }
    }
    writel(0, (*ss).base.add(SS_CTL as usize)); spin_unlock_irqrestore(&mut (*ss).slock, flags); err
}

// Error-release helper representing the common C goto release_ss path.
unsafe fn goto_release_ss(ss: *mut sun4i_ss_ctx, flags: &mut c_ulong) { writel(0, (*ss).base.add(SS_CTL as usize)); spin_unlock_irqrestore(&mut (*ss).slock, *flags); }

unsafe fn sun4i_ss_cipher_poll_fallback(areq: *mut skcipher_request) -> c_int {
    let tfm = crypto_skcipher_reqtfm(areq); let op = crypto_skcipher_ctx(tfm); let ctx = skcipher_request_ctx(areq);
    skcipher_request_set_tfm(&mut (*ctx).fallback_req, (*op).fallback_tfm);
    skcipher_request_set_callback(&mut (*ctx).fallback_req, (*areq).base.flags, (*areq).base.complete, (*areq).base.data);
    skcipher_request_set_crypt(&mut (*ctx).fallback_req, (*areq).src, (*areq).dst, (*areq).cryptlen, (*areq).iv);
    if (*ctx).mode & SS_DECRYPTION != 0 { crypto_skcipher_decrypt(&mut (*ctx).fallback_req) } else { crypto_skcipher_encrypt(&mut (*ctx).fallback_req) }
}

// Generic function that support SG with size not multiple of 4
unsafe fn sun4i_ss_cipher_poll(areq: *mut skcipher_request) -> c_int {
    let tfm = crypto_skcipher_reqtfm(areq); let op = crypto_skcipher_ctx(tfm); let ss = (*op).ss;
    let mut no_chunk = 1; let mut in_sg = (*areq).src; let mut out_sg = (*areq).dst;
    let ivsize = crypto_skcipher_ivsize(tfm); let ctx = skcipher_request_ctx(areq); let alg = crypto_skcipher_alg(tfm);
    let algt = container_of(alg, sun4i_ss_alg_template, alg.crypto); let mode = (*ctx).mode;
    if (*areq).cryptlen == 0 { return 0; }
    if (*areq).src.is_null() || (*areq).dst.is_null() { return -EINVAL; }
    let mut need_fallback = (*areq).cryptlen % (*algt).alg.crypto.base.cra_blocksize != 0;
    while !in_sg.is_null() && no_chunk == 1 { if ((*in_sg).length | (*in_sg).offset) & 3 != 0 { no_chunk = 0; } in_sg = sg_next(in_sg); }
    while !out_sg.is_null() && no_chunk == 1 { if ((*out_sg).length | (*out_sg).offset) & 3 != 0 { no_chunk = 0; } out_sg = sg_next(out_sg); }
    if no_chunk == 1 && !need_fallback { return sun4i_ss_opti_poll(areq); }
    if need_fallback { return sun4i_ss_cipher_poll_fallback(areq); }
    // The remaining path mirrors the C SG linearization/FIFO loop.
    // External kernel iterator and FIFO operations are intentionally kept as dependencies.
    let mut mi: sg_mapping_iter; let mut mo: sg_mapping_iter; let mut flags = 0; let mut err = 0;
    let mut ileft = (*areq).cryptlen; let mut oleft = (*areq).cryptlen; let mut rx_cnt = SS_RX_DEFAULT; let mut tx_cnt;
    let mut pi = 0; let mut po = 0; let mut oi = 0; let mut oo = 0; let mut ob = 0; let mut obo = 0; let mut obl; let mut todo;
    spin_lock_irqsave(&mut (*ss).slock, &mut flags);
    let mut i = 0; while i < (*op).keylen / 4 { writesl((*ss).base.add(SS_KEY0 as usize + i as usize * 4), (*op).key.as_ptr().add(i as usize), 1); i += 1; }
    if !(*areq).iv.is_null() { for i in 0..4 { if i < ivsize / 4 { let v = *((*areq).iv.add(i * 4) as *const u32); writesl((*ss).base.add(SS_IV0 as usize + i * 4), &v, 1); } } }
    writel(mode, (*ss).base.add(SS_CTL as usize));
    while oleft != 0 {
        if ileft != 0 { sg_miter_start(&mut mi, (*areq).src, sg_nents((*areq).src), SG_MITER_FROM_SG | SG_MITER_ATOMIC); if pi != 0 { sg_miter_skip(&mut mi, pi); } if !sg_miter_next(&mut mi) || mi.addr.is_null() { err = -EINVAL; break; }
            todo = core::cmp::min(rx_cnt, ileft / 4); todo = core::cmp::min(todo, (mi.length - oi) / 4); if todo != 0 && ob == 0 { writesl((*ss).base.add(SS_RXFIFO as usize), mi.addr.add(oi), todo as usize); ileft -= todo * 4; oi += todo * 4; } else { todo = core::cmp::min(rx_cnt * 4 - ob, ileft); todo = core::cmp::min(todo, mi.length - oi); memcpy((*ss).buf.add(ob), mi.addr.add(oi), todo); ileft -= todo; oi += todo; ob += todo; if ob % 4 == 0 { writesl((*ss).base.add(SS_RXFIFO as usize), (*ss).buf, ob / 4); ob = 0; } } if oi == mi.length { pi += mi.length as c_ulong; oi = 0; } sg_miter_stop(&mut mi); }
        let spaces = readl((*ss).base.add(SS_FCSR as usize)); rx_cnt = SS_RXFIFO_SPACES(spaces); tx_cnt = SS_TXFIFO_SPACES(spaces); if tx_cnt == 0 { continue; }
        sg_miter_start(&mut mo, (*areq).dst, sg_nents((*areq).dst), SG_MITER_TO_SG | SG_MITER_ATOMIC); if po != 0 { sg_miter_skip(&mut mo, po); } if !sg_miter_next(&mut mo) || mo.addr.is_null() { err = -EINVAL; break; }
        todo = core::cmp::min(tx_cnt, oleft / 4); todo = core::cmp::min(todo, (mo.length - oo) / 4); if todo != 0 { readsl((*ss).base.add(SS_TXFIFO as usize), mo.addr.add(oo), todo as usize); oleft -= todo * 4; oo += todo * 4; if oo == mo.length { po += mo.length as c_ulong; oo = 0; } } else { readsl((*ss).base.add(SS_TXFIFO as usize), (*ss).bufo, tx_cnt as usize); obl = tx_cnt * 4; obo = 0; while obo < obl { todo = core::cmp::min(mo.length - oo, obl - obo); memcpy(mo.addr.add(oo), (*ss).bufo.add(obo), todo); oleft -= todo; obo += todo; oo += todo; if oo == mo.length { po += mo.length as c_ulong; sg_miter_next(&mut mo); oo = 0; } } } sg_miter_stop(&mut mo);
    }
    writel(0, (*ss).base.add(SS_CTL as usize)); spin_unlock_irqrestore(&mut (*ss).slock, flags); err
}

macro_rules! cipher_fn { ($name:ident, $op:ident, $mode:ident, $dir:ident) => { pub unsafe fn $name(areq: *mut skcipher_request) -> c_int { let tfm = crypto_skcipher_reqtfm(areq); let op = crypto_skcipher_ctx(tfm); let rctx = skcipher_request_ctx(areq); (*rctx).mode = $op | $mode | SS_ENABLED | $dir | (*op).keymode; sun4i_ss_cipher_poll(areq) } }; }
cipher_fn!(sun4i_ss_cbc_aes_encrypt, SS_OP_AES, SS_CBC, SS_ENCRYPTION); cipher_fn!(sun4i_ss_cbc_aes_decrypt, SS_OP_AES, SS_CBC, SS_DECRYPTION);
cipher_fn!(sun4i_ss_ecb_aes_encrypt, SS_OP_AES, SS_ECB, SS_ENCRYPTION); cipher_fn!(sun4i_ss_ecb_aes_decrypt, SS_OP_AES, SS_ECB, SS_DECRYPTION);
cipher_fn!(sun4i_ss_cbc_des_encrypt, SS_OP_DES, SS_CBC, SS_ENCRYPTION); cipher_fn!(sun4i_ss_cbc_des_decrypt, SS_OP_DES, SS_CBC, SS_DECRYPTION);
cipher_fn!(sun4i_ss_ecb_des_encrypt, SS_OP_DES, SS_ECB, SS_ENCRYPTION); cipher_fn!(sun4i_ss_ecb_des_decrypt, SS_OP_DES, SS_ECB, SS_DECRYPTION);
cipher_fn!(sun4i_ss_cbc_des3_encrypt, SS_OP_3DES, SS_CBC, SS_ENCRYPTION); cipher_fn!(sun4i_ss_cbc_des3_decrypt, SS_OP_3DES, SS_CBC, SS_DECRYPTION);
cipher_fn!(sun4i_ss_ecb_des3_encrypt, SS_OP_3DES, SS_ECB, SS_ENCRYPTION); cipher_fn!(sun4i_ss_ecb_des3_decrypt, SS_OP_3DES, SS_ECB, SS_DECRYPTION);

pub unsafe fn sun4i_ss_cipher_init(tfm: *mut crypto_tfm) -> c_int { let op = crypto_tfm_ctx(tfm); memset(op as *mut _, 0, core::mem::size_of::<sun4i_tfm_ctx>()); let algt = container_of((*tfm).__crt_alg, sun4i_ss_alg_template, alg.crypto.base); (*op).ss = (*algt).ss; let name = crypto_tfm_alg_name(tfm); (*op).fallback_tfm = crypto_alloc_skcipher(name, 0, CRYPTO_ALG_NEED_FALLBACK); if is_err((*op).fallback_tfm) { return ptr_err((*op).fallback_tfm); } crypto_skcipher_set_reqsize(__crypto_skcipher_cast(tfm), core::mem::size_of::<sun4i_cipher_req_ctx>() + crypto_skcipher_reqsize((*op).fallback_tfm)); let err = pm_runtime_resume_and_get((*op).ss.dev); if err < 0 { crypto_free_skcipher((*op).fallback_tfm); return err; } 0 }
pub unsafe fn sun4i_ss_cipher_exit(tfm: *mut crypto_tfm) { let op = crypto_tfm_ctx(tfm); crypto_free_skcipher((*op).fallback_tfm); pm_runtime_put((*op).ss.dev); }

pub unsafe fn sun4i_ss_aes_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: c_uint) -> c_int { let op = crypto_skcipher_ctx(tfm); match keylen { 16 => (*op).keymode = SS_AES_128BITS, 24 => (*op).keymode = SS_AES_192BITS, 32 => (*op).keymode = SS_AES_256BITS, _ => return -EINVAL }; (*op).keylen = keylen; memcpy((*op).key.as_mut_ptr(), key, keylen); crypto_skcipher_clear_flags((*op).fallback_tfm, CRYPTO_TFM_REQ_MASK); crypto_skcipher_set_flags((*op).fallback_tfm, (*tfm).base.crt_flags & CRYPTO_TFM_REQ_MASK); crypto_skcipher_setkey((*op).fallback_tfm, key, keylen) }
pub unsafe fn sun4i_ss_des_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: c_uint) -> c_int { let op = crypto_skcipher_ctx(tfm); let err = verify_skcipher_des_key(tfm, key); if err != 0 { return err; } (*op).keylen = keylen; memcpy((*op).key.as_mut_ptr(), key, keylen); crypto_skcipher_clear_flags((*op).fallback_tfm, CRYPTO_TFM_REQ_MASK); crypto_skcipher_set_flags((*op).fallback_tfm, (*tfm).base.crt_flags & CRYPTO_TFM_REQ_MASK); crypto_skcipher_setkey((*op).fallback_tfm, key, keylen) }
pub unsafe fn sun4i_ss_des3_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: c_uint) -> c_int { let op = crypto_skcipher_ctx(tfm); let err = verify_skcipher_des3_key(tfm, key); if err != 0 { return err; } (*op).keylen = keylen; memcpy((*op).key.as_mut_ptr(), key, keylen); crypto_skcipher_clear_flags((*op).fallback_tfm, CRYPTO_TFM_REQ_MASK); crypto_skcipher_set_flags((*op).fallback_tfm, (*tfm).base.crt_flags & CRYPTO_TFM_REQ_MASK); crypto_skcipher_setkey((*op).fallback_tfm, key, keylen) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
