// SPDX-License-Identifier: GPL-2.0-only
/*
 * AES GCM routines supporting the Power 7+ Nest Accelerators driver
 *
 * Copyright (C) 2012 International Business Machines Inc.
 *
 * Author: Kent Yoder <yoder1@us.ibm.com>
 */

// C dependencies supplied by the surrounding kernel/driver translation.

unsafe fn gcm_aes_nx_set_key(tfm: *mut crypto_aead, in_key: *const u8,
                             mut key_len: c_uint) -> c_int {
    let nx_ctx = crypto_aead_ctx(tfm);
    let csbcpb = (*nx_ctx).csbcpb;
    let csbcpb_aead = (*nx_ctx).csbcpb_aead;

    nx_ctx_init(nx_ctx, HCOP_FC_AES);

    match key_len {
        AES_KEYSIZE_128 => {
            NX_CPB_SET_KEY_SIZE(csbcpb, NX_KS_AES_128);
            NX_CPB_SET_KEY_SIZE(csbcpb_aead, NX_KS_AES_128);
            (*nx_ctx).ap = &mut (*nx_ctx).props[NX_PROPS_AES_128];
        }
        AES_KEYSIZE_192 => {
            NX_CPB_SET_KEY_SIZE(csbcpb, NX_KS_AES_192);
            NX_CPB_SET_KEY_SIZE(csbcpb_aead, NX_KS_AES_192);
            (*nx_ctx).ap = &mut (*nx_ctx).props[NX_PROPS_AES_192];
        }
        AES_KEYSIZE_256 => {
            NX_CPB_SET_KEY_SIZE(csbcpb, NX_KS_AES_256);
            NX_CPB_SET_KEY_SIZE(csbcpb_aead, NX_KS_AES_256);
            (*nx_ctx).ap = &mut (*nx_ctx).props[NX_PROPS_AES_256];
        }
        _ => return -EINVAL,
    }

    (*csbcpb).cpb.hdr.mode = NX_MODE_AES_GCM;
    memcpy((*csbcpb).cpb.aes_gcm.key.as_mut_ptr() as *mut c_void,
           in_key as *const c_void, key_len as usize);
    (*csbcpb_aead).cpb.hdr.mode = NX_MODE_AES_GCA;
    memcpy((*csbcpb_aead).cpb.aes_gca.key.as_mut_ptr() as *mut c_void,
           in_key as *const c_void, key_len as usize);
    0
}

unsafe fn gcm4106_aes_nx_set_key(tfm: *mut crypto_aead, in_key: *const u8,
                                 mut key_len: c_uint) -> c_int {
    let nx_ctx = crypto_aead_ctx(tfm);
    let nonce = (*nx_ctx).priv_.gcm.nonce;
    if key_len < 4 { return -EINVAL; }
    key_len -= 4;
    let rc = gcm_aes_nx_set_key(tfm, in_key, key_len);
    if rc != 0 { return rc; }
    memcpy(nonce as *mut c_void, in_key.add(key_len as usize) as *const c_void, 4);
    rc
}

unsafe fn gcm4106_aes_nx_setauthsize(_tfm: *mut crypto_aead, authsize: c_uint) -> c_int {
    match authsize { 8 | 12 | 16 => 0, _ => -EINVAL }
}

unsafe fn nx_gca(nx_ctx: *mut nx_crypto_ctx, req: *mut aead_request,
                 out: *mut u8, assoclen: c_uint) -> c_int {
    let csbcpb_aead = (*nx_ctx).csbcpb_aead;
    let mut nx_sg = (*nx_ctx).in_sg;
    let nbytes = assoclen;
    let mut processed = 0;
    let mut to_process;
    let mut max_sg_len;
    if nbytes <= AES_BLOCK_SIZE { memcpy_from_sglist(out, (*req).src, 0, nbytes); return 0; }
    NX_CPB_FDM(csbcpb_aead) &= !NX_FDM_CONTINUATION;
    max_sg_len = min_t(nx_driver.of.max_sg_len / size_of::<nx_sg>(), (*nx_ctx).ap.sglen);
    max_sg_len = min_t(max_sg_len, (*nx_ctx).ap.databytelen / NX_PAGE_SIZE);
    let mut rc;
    while processed < nbytes {
        to_process = min_t(nbytes - processed, (*nx_ctx).ap.databytelen);
        to_process = min_t(to_process, NX_PAGE_SIZE * (max_sg_len - 1));
        nx_sg = nx_walk_and_build((*nx_ctx).in_sg, max_sg_len, (*req).src, processed, &mut to_process);
        if to_process + processed < nbytes { NX_CPB_FDM(csbcpb_aead) |= NX_FDM_INTERMEDIATE; }
        else { NX_CPB_FDM(csbcpb_aead) &= !NX_FDM_INTERMEDIATE; }
        (*nx_ctx).op_aead.inlen = ((*nx_ctx).in_sg.offset_from(nx_sg) as usize * size_of::<nx_sg>()) as _;
        rc = nx_hcall_sync(nx_ctx, &mut (*nx_ctx).op_aead, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP);
        if rc != 0 { return rc; }
        memcpy((*csbcpb_aead).cpb.aes_gca.in_pat.as_mut_ptr() as *mut c_void,
               (*csbcpb_aead).cpb.aes_gca.out_pat.as_ptr() as *const c_void, AES_BLOCK_SIZE);
        NX_CPB_FDM(csbcpb_aead) |= NX_FDM_CONTINUATION;
        atomic_inc(&mut (*nx_ctx).stats.aes_ops);
        atomic64_add(assoclen, &mut (*nx_ctx).stats.aes_bytes);
        processed += to_process;
    }
    memcpy(out as *mut c_void, (*csbcpb_aead).cpb.aes_gca.out_pat.as_ptr() as *const c_void, AES_BLOCK_SIZE);
    rc
}

unsafe fn gmac(req: *mut aead_request, iv: *const u8, assoclen: c_uint) -> c_int {
    let nx_ctx = crypto_aead_ctx(crypto_aead_reqtfm(req));
    let csbcpb = (*nx_ctx).csbcpb;
    let mut nx_sg;
    let nbytes = assoclen;
    let mut processed = 0;
    let mut to_process;
    let mut max_sg_len;
    let mut rc = 0;
    (*csbcpb).cpb.hdr.mode = NX_MODE_AES_GMAC;
    NX_CPB_FDM(csbcpb) &= !NX_FDM_CONTINUATION;
    max_sg_len = min_t(nx_driver.of.max_sg_len / size_of::<nx_sg>(), (*nx_ctx).ap.sglen);
    max_sg_len = min_t(max_sg_len, (*nx_ctx).ap.databytelen / NX_PAGE_SIZE);
    memcpy((*csbcpb).cpb.aes_gcm.iv_or_cnt.as_mut_ptr() as *mut c_void, iv as *const c_void, AES_BLOCK_SIZE);
    while processed < nbytes {
        to_process = min_t(nbytes - processed, (*nx_ctx).ap.databytelen);
        to_process = min_t(to_process, NX_PAGE_SIZE * (max_sg_len - 1));
        nx_sg = nx_walk_and_build((*nx_ctx).in_sg, max_sg_len, (*req).src, processed, &mut to_process);
        if to_process + processed < nbytes { NX_CPB_FDM(csbcpb) |= NX_FDM_INTERMEDIATE; }
        else { NX_CPB_FDM(csbcpb) &= !NX_FDM_INTERMEDIATE; }
        (*nx_ctx).op.inlen = ((*nx_ctx).in_sg.offset_from(nx_sg) as usize * size_of::<nx_sg>()) as _;
        (*csbcpb).cpb.aes_gcm.bit_length_data = 0;
        (*csbcpb).cpb.aes_gcm.bit_length_aad = 8 * nbytes;
        rc = nx_hcall_sync(nx_ctx, &mut (*nx_ctx).op, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP);
        if rc != 0 { break; }
        memcpy((*csbcpb).cpb.aes_gcm.in_pat_or_aad.as_mut_ptr() as *mut c_void, (*csbcpb).cpb.aes_gcm.out_pat_or_mac.as_ptr() as *const c_void, AES_BLOCK_SIZE);
        memcpy((*csbcpb).cpb.aes_gcm.in_s0.as_mut_ptr() as *mut c_void, (*csbcpb).cpb.aes_gcm.out_s0.as_ptr() as *const c_void, AES_BLOCK_SIZE);
        NX_CPB_FDM(csbcpb) |= NX_FDM_CONTINUATION;
        atomic_inc(&mut (*nx_ctx).stats.aes_ops);
        atomic64_add(assoclen, &mut (*nx_ctx).stats.aes_bytes);
        processed += to_process;
    }
    (*csbcpb).cpb.hdr.mode = NX_MODE_AES_GCM;
    rc
}

// The remaining routines retain the C driver's request/context layout and external helper calls.
// Their declarations are translated below without introducing implementations for dependencies.

unsafe fn gcm_empty(req: *mut aead_request, iv: *const u8, enc: c_int) -> c_int {
    let nx_ctx = crypto_aead_ctx(crypto_aead_reqtfm(req));
    let csbcpb = (*nx_ctx).csbcpb;
    let mut out = [0u8; AES_BLOCK_SIZE as usize];
    let mut len = AES_BLOCK_SIZE;
    (*csbcpb).cpb.hdr.mode = NX_MODE_AES_ECB;
    memcpy((*csbcpb).cpb.aes_ecb.key.as_mut_ptr() as *mut c_void, (*csbcpb).cpb.aes_gcm.key.as_ptr() as *const c_void, size_of_val(&(*csbcpb).cpb.aes_ecb.key));
    if enc != 0 { NX_CPB_FDM(csbcpb) |= NX_FDM_ENDE_ENCRYPT; } else { NX_CPB_FDM(csbcpb) &= !NX_FDM_ENDE_ENCRYPT; }
    let in_sg = nx_build_sg_list((*nx_ctx).in_sg, iv as *mut u8, &mut len, (*nx_ctx).ap.sglen);
    if len != AES_BLOCK_SIZE { return -EINVAL; }
    len = out.len() as _;
    let out_sg = nx_build_sg_list((*nx_ctx).out_sg, out.as_mut_ptr(), &mut len, (*nx_ctx).ap.sglen);
    if len != out.len() as _ { return -EINVAL; }
    (*nx_ctx).op.inlen = ((*nx_ctx).in_sg.offset_from(in_sg) as usize * size_of::<nx_sg>()) as _;
    (*nx_ctx).op.outlen = ((*nx_ctx).out_sg.offset_from(out_sg) as usize * size_of::<nx_sg>()) as _;
    let rc = nx_hcall_sync(nx_ctx, &mut (*nx_ctx).op, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP);
    if rc == 0 { atomic_inc(&mut (*nx_ctx).stats.aes_ops); memcpy((*csbcpb).cpb.aes_gcm.out_pat_or_mac.as_mut_ptr() as *mut c_void, out.as_ptr() as *const c_void, crypto_aead_authsize(crypto_aead_reqtfm(req))); }
    (*csbcpb).cpb.hdr.mode = NX_MODE_AES_GCM;
    memset((*csbcpb).cpb.aes_ecb.key.as_mut_ptr() as *mut c_void, 0, size_of_val(&(*csbcpb).cpb.aes_ecb.key));
    rc
}

unsafe fn gcm_aes_nx_crypt(req: *mut aead_request, enc: c_int, assoclen: c_uint) -> c_int {
    let nx_ctx = crypto_aead_ctx(crypto_aead_reqtfm(req));
    let rctx = aead_request_ctx(req);
    let csbcpb = (*nx_ctx).csbcpb;
    let mut nbytes = (*req).cryptlen;
    let mut processed = 0;
    let mut rc = -EINVAL;
    let mut irq_flags = 0;
    spin_lock_irqsave(&mut (*nx_ctx).lock, &mut irq_flags);
    *( (*rctx).iv.as_mut_ptr().add(NX_GCM_CTR_OFFSET) as *mut u32) = 1;
    if nbytes == 0 {
        rc = if assoclen == 0 { gcm_empty(req, (*rctx).iv.as_ptr(), enc) } else { gmac(req, (*rctx).iv.as_ptr(), assoclen) };
        if rc != 0 { spin_unlock_irqrestore(&mut (*nx_ctx).lock, irq_flags); return rc; }
    } else {
        (*csbcpb).cpb.aes_gcm.bit_length_aad = assoclen * 8;
        if assoclen != 0 { rc = nx_gca(nx_ctx, req, (*csbcpb).cpb.aes_gcm.in_pat_or_aad.as_mut_ptr(), assoclen); if rc != 0 { spin_unlock_irqrestore(&mut (*nx_ctx).lock, irq_flags); return rc; } }
        NX_CPB_FDM(csbcpb) &= !NX_FDM_CONTINUATION;
        if enc != 0 { NX_CPB_FDM(csbcpb) |= NX_FDM_ENDE_ENCRYPT; } else { NX_CPB_FDM(csbcpb) &= !NX_FDM_ENDE_ENCRYPT; nbytes -= crypto_aead_authsize(crypto_aead_reqtfm(req)); }
        while processed < nbytes {
            let mut to_process = nbytes - processed;
            (*csbcpb).cpb.aes_gcm.bit_length_data = nbytes * 8;
            rc = nx_build_sg_lists(nx_ctx, (*rctx).iv.as_mut_ptr(), (*req).dst, (*req).src, &mut to_process, processed + (*req).assoclen, (*csbcpb).cpb.aes_gcm.iv_or_cnt.as_mut_ptr());
            if rc != 0 { break; }
            if to_process + processed < nbytes { NX_CPB_FDM(csbcpb) |= NX_FDM_INTERMEDIATE; } else { NX_CPB_FDM(csbcpb) &= !NX_FDM_INTERMEDIATE; }
            rc = nx_hcall_sync(nx_ctx, &mut (*nx_ctx).op, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP);
            if rc != 0 { break; }
            memcpy((*rctx).iv.as_mut_ptr() as *mut c_void, (*csbcpb).cpb.aes_gcm.out_cnt.as_ptr() as *const c_void, AES_BLOCK_SIZE);
            memcpy((*csbcpb).cpb.aes_gcm.in_pat_or_aad.as_mut_ptr() as *mut c_void, (*csbcpb).cpb.aes_gcm.out_pat_or_mac.as_ptr() as *const c_void, AES_BLOCK_SIZE);
            memcpy((*csbcpb).cpb.aes_gcm.in_s0.as_mut_ptr() as *mut c_void, (*csbcpb).cpb.aes_gcm.out_s0.as_ptr() as *const c_void, AES_BLOCK_SIZE);
            NX_CPB_FDM(csbcpb) |= NX_FDM_CONTINUATION;
            atomic_inc(&mut (*nx_ctx).stats.aes_ops);
            atomic64_add(be32_to_cpu((*csbcpb).csb.processed_byte_count), &mut (*nx_ctx).stats.aes_bytes);
            processed += to_process;
        }
    }
    if rc == 0 {
        if enc != 0 { memcpy_to_sglist((*req).dst, (*req).assoclen + nbytes, (*csbcpb).cpb.aes_gcm.out_pat_or_mac.as_ptr(), crypto_aead_authsize(crypto_aead_reqtfm(req))); }
        else { let itag = (*nx_ctx).priv_.gcm.iauth_tag; let len = crypto_aead_authsize(crypto_aead_reqtfm(req)); memcpy_from_sglist(itag, (*req).src, (*req).assoclen + nbytes, len); rc = if crypto_memneq(itag, (*csbcpb).cpb.aes_gcm.out_pat_or_mac.as_ptr(), len) != 0 { -EBADMSG } else { 0 }; }
    }
    spin_unlock_irqrestore(&mut (*nx_ctx).lock, irq_flags); rc
}

unsafe fn gcm_aes_nx_encrypt(req: *mut aead_request) -> c_int { let rctx = aead_request_ctx(req); memcpy((*rctx).iv.as_mut_ptr() as *mut c_void, (*req).iv as *const c_void, GCM_AES_IV_SIZE); gcm_aes_nx_crypt(req, 1, (*req).assoclen) }
unsafe fn gcm_aes_nx_decrypt(req: *mut aead_request) -> c_int { let rctx = aead_request_ctx(req); memcpy((*rctx).iv.as_mut_ptr() as *mut c_void, (*req).iv as *const c_void, GCM_AES_IV_SIZE); gcm_aes_nx_crypt(req, 0, (*req).assoclen) }
unsafe fn gcm4106_aes_nx_encrypt(req: *mut aead_request) -> c_int { let nx = crypto_aead_ctx(crypto_aead_reqtfm(req)); let r = aead_request_ctx(req); memcpy((*r).iv.as_mut_ptr() as *mut c_void, (*nx).priv_.gcm.nonce as *const c_void, NX_GCM4106_NONCE_LEN); memcpy((*r).iv.as_mut_ptr().add(NX_GCM4106_NONCE_LEN) as *mut c_void, (*req).iv as *const c_void, 8); if (*req).assoclen < 8 { -EINVAL } else { gcm_aes_nx_crypt(req, 1, (*req).assoclen - 8) } }
unsafe fn gcm4106_aes_nx_decrypt(req: *mut aead_request) -> c_int { let nx = crypto_aead_ctx(crypto_aead_reqtfm(req)); let r = aead_request_ctx(req); memcpy((*r).iv.as_mut_ptr() as *mut c_void, (*nx).priv_.gcm.nonce as *const c_void, NX_GCM4106_NONCE_LEN); memcpy((*r).iv.as_mut_ptr().add(NX_GCM4106_NONCE_LEN) as *mut c_void, (*req).iv as *const c_void, 8); if (*req).assoclen < 8 { -EINVAL } else { gcm_aes_nx_crypt(req, 0, (*req).assoclen - 8) } }

#[no_mangle]
pub static mut nx_gcm_aes_alg: aead_alg = aead_alg { base: crypto_alg { cra_name: "gcm(aes)", cra_driver_name: "gcm-aes-nx", cra_priority: 300, cra_blocksize: 1, cra_ctxsize: size_of::<nx_crypto_ctx>(), cra_module: THIS_MODULE }, init: nx_crypto_ctx_aes_gcm_init, exit: nx_crypto_ctx_aead_exit, ivsize: GCM_AES_IV_SIZE, maxauthsize: AES_BLOCK_SIZE, setkey: gcm_aes_nx_set_key, encrypt: gcm_aes_nx_encrypt, decrypt: gcm_aes_nx_decrypt, setauthsize: None };
#[no_mangle]
pub static mut nx_gcm4106_aes_alg: aead_alg = aead_alg { base: crypto_alg { cra_name: "rfc4106(gcm(aes))", cra_driver_name: "rfc4106-gcm-aes-nx", cra_priority: 300, cra_blocksize: 1, cra_ctxsize: size_of::<nx_crypto_ctx>(), cra_module: THIS_MODULE }, init: nx_crypto_ctx_aes_gcm_init, exit: nx_crypto_ctx_aead_exit, ivsize: GCM_RFC4106_IV_SIZE, maxauthsize: AES_BLOCK_SIZE, setkey: gcm4106_aes_nx_set_key, setauthsize: gcm4106_aes_nx_setauthsize, encrypt: gcm4106_aes_nx_encrypt, decrypt: gcm4106_aes_nx_decrypt };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
