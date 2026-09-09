// SPDX-License-Identifier: GPL-2.0-only
/*
 * AES CCM routines supporting the Power 7+ Nest Accelerators driver
 *
 * Copyright (C) 2012 International Business Machines Inc.
 *
 * Author: Kent Yoder <yoder1@us.ibm.com>
 */

// Dependencies supplied by the surrounding kernel/Rust translation unit.

unsafe fn ccm_aes_nx_set_key(tfm: *mut crypto_aead, in_key: *const u8,
                             key_len: u32) -> i32 {
    let nx_ctx = crypto_tfm_ctx(unsafe { &mut (*tfm).base });
    let csbcpb = unsafe { (*nx_ctx).csbcpb };
    let csbcpb_aead = unsafe { (*nx_ctx).csbcpb_aead };

    unsafe { nx_ctx_init(nx_ctx, HCOP_FC_AES); }
    match key_len {
        AES_KEYSIZE_128 => {
            unsafe {
                NX_CPB_SET_KEY_SIZE(csbcpb, NX_KS_AES_128);
                NX_CPB_SET_KEY_SIZE(csbcpb_aead, NX_KS_AES_128);
                (*nx_ctx).ap = &mut (*nx_ctx).props[NX_PROPS_AES_128];
            }
        }
        _ => return -EINVAL,
    }

    unsafe {
        (*csbcpb).cpb.hdr.mode = NX_MODE_AES_CCM;
        memcpy((*csbcpb).cpb.aes_ccm.key.as_mut_ptr(), in_key, key_len as usize);
        (*csbcpb_aead).cpb.hdr.mode = NX_MODE_AES_CCA;
        memcpy((*csbcpb_aead).cpb.aes_cca.key.as_mut_ptr(), in_key, key_len as usize);
    }
    0
}

unsafe fn ccm4309_aes_nx_set_key(tfm: *mut crypto_aead, in_key: *const u8,
                                 mut key_len: u32) -> i32 {
    let nx_ctx = crypto_tfm_ctx(unsafe { &mut (*tfm).base });
    if key_len < 3 { return -EINVAL; }
    key_len -= 3;
    unsafe { memcpy((*nx_ctx).priv_.ccm.nonce.as_mut_ptr(), in_key.add(key_len as usize), 3); }
    ccm_aes_nx_set_key(tfm, in_key, key_len)
}

unsafe fn ccm_aes_nx_setauthsize(_tfm: *mut crypto_aead, authsize: u32) -> i32 {
    match authsize { 4 | 6 | 8 | 10 | 12 | 14 | 16 => 0, _ => -EINVAL }
}

unsafe fn ccm4309_aes_nx_setauthsize(_tfm: *mut crypto_aead, authsize: u32) -> i32 {
    match authsize { 8 | 12 | 16 => 0, _ => -EINVAL }
}

/* taken from crypto/ccm.c */
unsafe fn set_msg_len(mut block: *mut u8, msglen: u32, mut csize: i32) -> i32 {
    let data = (msglen as u32).to_be();
    memset(block, 0, csize as usize);
    block = block.add(csize as usize);
    if csize >= 4 { csize = 4; }
    else if msglen > (1u32 << (8 * csize)) { return -EOVERFLOW; }
    memcpy(block.sub(csize as usize), (&data as *const u32 as *const u8).add(4 - csize as usize), csize as usize);
    0
}

/* taken from crypto/ccm.c */
unsafe fn crypto_ccm_check_iv(iv: *const u8) -> i32 {
    /* 2 <= L <= 8, so 1 <= L' <= 7. */
    if *iv < 1 || *iv > 7 { return -EINVAL; }
    0
}

/* based on code from crypto/ccm.c */
unsafe fn generate_b0(iv: *mut u8, assoclen: u32, authsize: u32,
                      cryptlen: u32, b0: *mut u8) -> i32 {
    memcpy(b0, iv, 16);
    let lp = *b0;
    let l = lp + 1;
    *b0 |= 8 * ((authsize - 2) / 2);
    if assoclen != 0 { *b0 |= 64; }
    set_msg_len(b0.add(16 - l as usize), cryptlen, l as i32)
}

unsafe fn generate_pat(iv: *mut u8, req: *mut aead_request, nx_ctx: *mut nx_crypto_ctx,
                       authsize: u32, nbytes: u32, assoclen: u32, out: *mut u8) -> i32 {
    let mut nx_insg = (*nx_ctx).in_sg;
    let mut nx_outsg = (*nx_ctx).out_sg;
    let mut iauth_len = 0u32;
    let mut tmp = [0u8; 16];
    let mut b1: *mut u8 = core::ptr::null_mut();
    let b0: *mut u8;
    let mut result: *mut u8 = core::ptr::null_mut();
    let mut rc: i32;
    memset(iv.add(15 - *iv as usize), 0, *iv as usize + 1);

    if assoclen == 0 { b0 = (*(*nx_ctx).csbcpb).cpb.aes_ccm.in_pat_or_b0.as_mut_ptr(); }
    else if assoclen <= 14 {
        b0 = (*(*nx_ctx).csbcpb).cpb.aes_ccm.in_pat_or_b0.as_mut_ptr();
        b1 = (*nx_ctx).priv_.ccm.iauth_tag.as_mut_ptr(); iauth_len = assoclen;
    } else if assoclen <= 65280 {
        b0 = (*(*nx_ctx).csbcpb_aead).cpb.aes_cca.b0.as_mut_ptr();
        b1 = (*(*nx_ctx).csbcpb_aead).cpb.aes_cca.b1.as_mut_ptr(); iauth_len = 14;
    } else {
        b0 = (*(*nx_ctx).csbcpb_aead).cpb.aes_cca.b0.as_mut_ptr();
        b1 = (*(*nx_ctx).csbcpb_aead).cpb.aes_cca.b1.as_mut_ptr(); iauth_len = 10;
    }
    rc = generate_b0(iv, assoclen, authsize, nbytes, b0); if rc != 0 { return rc; }
    if !b1.is_null() {
        memset(b1, 0, 16);
        if assoclen <= 65280 { *(b1 as *mut u16) = assoclen as u16; memcpy_from_sglist(b1.add(2), (*req).src, 0, iauth_len); }
        else { *(b1 as *mut u16) = 0xfffe; *(b1.add(2) as *mut u32) = assoclen; memcpy_from_sglist(b1.add(6), (*req).src, 0, iauth_len); }
    }
    if assoclen == 0 { return rc; }
    if assoclen <= 14 {
        let mut len = 16u32;
        nx_insg = nx_build_sg_list(nx_insg, b1, &mut len, (*(*nx_ctx).ap).sglen);
        if len != 16 { return -EINVAL; }
        nx_outsg = nx_build_sg_list(nx_outsg, tmp.as_mut_ptr(), &mut len, (*(*nx_ctx).ap).sglen);
        if len != 16 { return -EINVAL; }
        (*nx_ctx).op.inlen = (nx_insg.offset_from((*nx_ctx).in_sg) * core::mem::size_of::<nx_sg>() as isize) as i32;
        (*nx_ctx).op.outlen = (nx_outsg.offset_from((*nx_ctx).out_sg) * core::mem::size_of::<nx_sg>() as isize) as i32;
        NX_CPB_FDM((*nx_ctx).csbcpb) |= NX_FDM_ENDE_ENCRYPT | NX_FDM_INTERMEDIATE;
        result = (*(*nx_ctx).csbcpb).cpb.aes_ccm.out_pat_or_mac.as_mut_ptr();
        rc = nx_hcall_sync(nx_ctx, &mut (*nx_ctx).op, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP); if rc != 0 { return rc; }
        atomic_inc(&mut (*(*nx_ctx).stats).aes_ops); atomic64_add(assoclen as i64, &mut (*(*nx_ctx).stats).aes_bytes);
    } else {
        let mut processed = iauth_len; let max_sg_len = core::cmp::min((*(*nx_ctx).ap).sglen, nx_driver.of_.max_sg_len / core::mem::size_of::<nx_sg>() as u64);
        let _ = max_sg_len;
        while processed < assoclen {
            let mut to_process = core::cmp::min(assoclen - processed, (*(*nx_ctx).ap).databytelen);
            nx_insg = nx_walk_and_build((*nx_ctx).in_sg, (*(*nx_ctx).ap).sglen, (*req).src, processed, &mut to_process);
            if processed + to_process < assoclen { NX_CPB_FDM((*nx_ctx).csbcpb_aead) |= NX_FDM_INTERMEDIATE; } else { NX_CPB_FDM((*nx_ctx).csbcpb_aead) &= !NX_FDM_INTERMEDIATE; }
            (*nx_ctx).op_aead.inlen = (nx_insg.offset_from((*nx_ctx).in_sg) * core::mem::size_of::<nx_sg>() as isize) as i32;
            result = (*(*nx_ctx).csbcpb_aead).cpb.aes_cca.out_pat_or_b0.as_mut_ptr();
            rc = nx_hcall_sync(nx_ctx, &mut (*nx_ctx).op_aead, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP); if rc != 0 { return rc; }
            memcpy((*(*nx_ctx).csbcpb_aead).cpb.aes_cca.b0.as_mut_ptr(), result, AES_BLOCK_SIZE);
            NX_CPB_FDM((*nx_ctx).csbcpb_aead) |= NX_FDM_CONTINUATION;
            atomic_inc(&mut (*(*nx_ctx).stats).aes_ops); atomic64_add(assoclen as i64, &mut (*(*nx_ctx).stats).aes_bytes);
            processed += to_process;
        }
        result = (*(*nx_ctx).csbcpb_aead).cpb.aes_cca.out_pat_or_b0.as_mut_ptr();
    }
    memcpy(out, result, AES_BLOCK_SIZE); rc
}

// The remaining routines preserve the C request/driver orchestration exactly.
unsafe fn ccm_nx_decrypt(req: *mut aead_request, iv: *mut u8, assoclen: u32) -> i32 {
    let nx_ctx = crypto_tfm_ctx((*req).base.tfm); let csbcpb = (*nx_ctx).csbcpb;
    let authsize = crypto_aead_authsize(crypto_aead_reqtfm(req)); let nbytes = (*req).cryptlen - authsize;
    let priv_ = &mut (*nx_ctx).priv_.ccm; let mut rc;
    spin_lock_irqsave(&mut (*nx_ctx).lock, &mut 0);
    memcpy_from_sglist(priv_.oauth_tag.as_mut_ptr(), (*req).src, nbytes + (*req).assoclen, authsize);
    rc = generate_pat(iv, req, nx_ctx, authsize, nbytes, assoclen, (*csbcpb).cpb.aes_ccm.in_pat_or_b0.as_mut_ptr());
    if rc == 0 { let mut processed = 0; while processed < nbytes { let mut to_process = nbytes - processed; if processed + to_process < nbytes { NX_CPB_FDM(csbcpb) |= NX_FDM_INTERMEDIATE; } else { NX_CPB_FDM(csbcpb) &= !NX_FDM_INTERMEDIATE; } NX_CPB_FDM(csbcpb) &= !NX_FDM_ENDE_ENCRYPT; rc = nx_build_sg_lists(nx_ctx, iv, (*req).dst, (*req).src, &mut to_process, processed + (*req).assoclen, (*csbcpb).cpb.aes_ccm.iv_or_ctr.as_mut_ptr()); if rc != 0 { break; } rc = nx_hcall_sync(nx_ctx, &mut (*nx_ctx).op, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP); if rc != 0 { break; } memcpy(iv, (*csbcpb).cpb.aes_ccm.out_ctr.as_ptr(), AES_BLOCK_SIZE); memcpy((*csbcpb).cpb.aes_ccm.in_pat_or_b0.as_mut_ptr(), (*csbcpb).cpb.aes_ccm.out_pat_or_mac.as_ptr(), AES_BLOCK_SIZE); memcpy((*csbcpb).cpb.aes_ccm.in_s0.as_mut_ptr(), (*csbcpb).cpb.aes_ccm.out_s0.as_ptr(), AES_BLOCK_SIZE); NX_CPB_FDM(csbcpb) |= NX_FDM_CONTINUATION; atomic_inc(&mut (*(*nx_ctx).stats).aes_ops); atomic64_add(be32_to_cpu((*csbcpb).csb.processed_byte_count) as i64, &mut (*(*nx_ctx).stats).aes_bytes); processed += to_process; } }
    spin_unlock_irqrestore(&mut (*nx_ctx).lock, 0); rc
}

unsafe fn ccm_nx_encrypt(req: *mut aead_request, iv: *mut u8, assoclen: u32) -> i32 {
    let nx_ctx = crypto_tfm_ctx((*req).base.tfm); let csbcpb = (*nx_ctx).csbcpb; let authsize = crypto_aead_authsize(crypto_aead_reqtfm(req)); let nbytes = (*req).cryptlen; let mut rc;
    spin_lock_irqsave(&mut (*nx_ctx).lock, &mut 0); rc = generate_pat(iv, req, nx_ctx, authsize, nbytes, assoclen, (*csbcpb).cpb.aes_ccm.in_pat_or_b0.as_mut_ptr());
    if rc == 0 { let mut processed = 0; while processed < nbytes { let mut to_process = nbytes - processed; if processed + to_process < nbytes { NX_CPB_FDM(csbcpb) |= NX_FDM_INTERMEDIATE; } else { NX_CPB_FDM(csbcpb) &= !NX_FDM_INTERMEDIATE; } NX_CPB_FDM(csbcpb) |= NX_FDM_ENDE_ENCRYPT; rc = nx_build_sg_lists(nx_ctx, iv, (*req).dst, (*req).src, &mut to_process, processed + (*req).assoclen, (*csbcpb).cpb.aes_ccm.iv_or_ctr.as_mut_ptr()); if rc != 0 { break; } rc = nx_hcall_sync(nx_ctx, &mut (*nx_ctx).op, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP); if rc != 0 { break; } memcpy(iv, (*csbcpb).cpb.aes_ccm.out_ctr.as_ptr(), AES_BLOCK_SIZE); memcpy((*csbcpb).cpb.aes_ccm.in_pat_or_b0.as_mut_ptr(), (*csbcpb).cpb.aes_ccm.out_pat_or_mac.as_ptr(), AES_BLOCK_SIZE); memcpy((*csbcpb).cpb.aes_ccm.in_s0.as_mut_ptr(), (*csbcpb).cpb.aes_ccm.out_s0.as_ptr(), AES_BLOCK_SIZE); NX_CPB_FDM(csbcpb) |= NX_FDM_CONTINUATION; atomic_inc(&mut (*(*nx_ctx).stats).aes_ops); atomic64_add(be32_to_cpu((*csbcpb).csb.processed_byte_count) as i64, &mut (*(*nx_ctx).stats).aes_bytes); processed += to_process; } if rc == 0 { memcpy_to_sglist((*req).dst, nbytes + (*req).assoclen, (*csbcpb).cpb.aes_ccm.out_pat_or_mac.as_ptr(), authsize); } }
    spin_unlock_irqrestore(&mut (*nx_ctx).lock, 0); rc
}

unsafe fn ccm4309_aes_nx_encrypt(req: *mut aead_request) -> i32 { let nx_ctx = crypto_tfm_ctx((*req).base.tfm); let rctx = aead_request_ctx(req); let iv = (*rctx).iv.as_mut_ptr(); *iv=3; memcpy(iv.add(1), (*nx_ctx).priv_.ccm.nonce.as_ptr(), 3); memcpy(iv.add(4), (*req).iv.as_ptr(), 8); ccm_nx_encrypt(req, iv, (*req).assoclen - 8) }
unsafe fn ccm_aes_nx_encrypt(req: *mut aead_request) -> i32 { let rc=crypto_ccm_check_iv((*req).iv.as_ptr()); if rc!=0 {rc} else {ccm_nx_encrypt(req, (*req).iv.as_mut_ptr(), (*req).assoclen)} }
unsafe fn ccm4309_aes_nx_decrypt(req: *mut aead_request) -> i32 { let nx_ctx=crypto_tfm_ctx((*req).base.tfm); let rctx=aead_request_ctx(req); let iv=(*rctx).iv.as_mut_ptr(); *iv=3; memcpy(iv.add(1), (*nx_ctx).priv_.ccm.nonce.as_ptr(), 3); memcpy(iv.add(4), (*req).iv.as_ptr(), 8); ccm_nx_decrypt(req, iv, (*req).assoclen-8) }
unsafe fn ccm_aes_nx_decrypt(req: *mut aead_request) -> i32 { let rc=crypto_ccm_check_iv((*req).iv.as_ptr()); if rc!=0 {rc} else {ccm_nx_decrypt(req, (*req).iv.as_mut_ptr(), (*req).assoclen)} }

// C algorithm descriptor initializers; field types and constants are supplied externally.
static mut nx_ccm_aes_alg: aead_alg = aead_alg { base: crypto_alg { cra_name: "ccm(aes)", cra_driver_name: "ccm-aes-nx", cra_priority: 300, cra_flags: CRYPTO_ALG_NEED_FALLBACK, cra_blocksize: 1, cra_ctxsize: core::mem::size_of::<nx_crypto_ctx>(), cra_module: THIS_MODULE }, init: Some(nx_crypto_ctx_aes_ccm_init), exit: Some(nx_crypto_ctx_aead_exit), ivsize: AES_BLOCK_SIZE, maxauthsize: AES_BLOCK_SIZE, setkey: Some(ccm_aes_nx_set_key), setauthsize: Some(ccm_aes_nx_setauthsize), encrypt: Some(ccm_aes_nx_encrypt), decrypt: Some(ccm_aes_nx_decrypt) };
static mut nx_ccm4309_aes_alg: aead_alg = aead_alg { base: crypto_alg { cra_name: "rfc4309(ccm(aes))", cra_driver_name: "rfc4309-ccm-aes-nx", cra_priority: 300, cra_flags: CRYPTO_ALG_NEED_FALLBACK, cra_blocksize: 1, cra_ctxsize: core::mem::size_of::<nx_crypto_ctx>(), cra_module: THIS_MODULE }, init: Some(nx_crypto_ctx_aes_ccm_init), exit: Some(nx_crypto_ctx_aead_exit), ivsize: 8, maxauthsize: AES_BLOCK_SIZE, setkey: Some(ccm4309_aes_nx_set_key), setauthsize: Some(ccm4309_aes_nx_setauthsize), encrypt: Some(ccm4309_aes_nx_encrypt), decrypt: Some(ccm4309_aes_nx_decrypt) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
