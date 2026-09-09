// SPDX-License-Identifier: GPL-2.0-only
/*
 * AES XCBC routines supporting the Power 7+ Nest Accelerators driver
 *
 * Copyright (C) 2011-2012 International Business Machines Inc.
 *
 * Author: Kent Yoder <yoder1@us.ibm.com>
 */

// Dependencies supplied by the surrounding kernel/Rust bindings are intentionally external.

#[repr(C)]
struct XcbcState {
    state: [u8; AES_BLOCK_SIZE],
}

unsafe fn nx_xcbc_set_key(
    desc: *mut crypto_shash,
    in_key: *const u8,
    key_len: c_uint,
) -> c_int {
    let nx_ctx = crypto_shash_ctx(desc);
    let csbcpb = (*nx_ctx).csbcpb;

    match key_len {
        AES_KEYSIZE_128 => {
            (*nx_ctx).ap = &mut (*nx_ctx).props[NX_PROPS_AES_128];
        }
        _ => return -EINVAL,
    }

    memcpy((*csbcpb).cpb.aes_xcbc.key.as_mut_ptr(), in_key, key_len as usize);
    0
}

/*
 * Based on RFC 3566, for a zero-length message:
 *
 * n = 1
 * K1 = E(K, 0x01010101010101010101010101010101)
 * K3 = E(K, 0x03030303030303030303030303030303)
 * E[0] = 0x00000000000000000000000000000000
 * M[1] = 0x80000000000000000000000000000000 (0 length message with padding)
 * E[1] = (K1, M[1] ^ E[0] ^ K3)
 * Tag = M[1]
 */
unsafe fn nx_xcbc_empty(desc: *mut shash_desc, out: *mut u8) -> c_int {
    let nx_ctx = crypto_shash_ctx((*desc).tfm);
    let csbcpb = (*nx_ctx).csbcpb;
    let mut in_sg: *mut nx_sg;
    let mut out_sg: *mut nx_sg;
    let mut keys = [[0u8; AES_BLOCK_SIZE]; 2];
    let mut key = [0u8; 32];
    let mut rc: c_int = 0;
    let mut len: c_int;

    (*csbcpb).cpb.hdr.mode = NX_MODE_AES_ECB;
    memcpy(key.as_mut_ptr(), (*csbcpb).cpb.aes_xcbc.key.as_ptr(), AES_BLOCK_SIZE);
    memcpy((*csbcpb).cpb.aes_ecb.key.as_mut_ptr(), key.as_ptr(), AES_BLOCK_SIZE);
    NX_CPB_FDM(csbcpb) |= NX_FDM_ENDE_ENCRYPT;

    memset(keys[0].as_mut_ptr(), 0x01, keys[0].len());
    memset(keys[1].as_mut_ptr(), 0x03, keys[1].len());

    len = core::mem::size_of_val(&keys) as c_int;
    in_sg = nx_build_sg_list((*nx_ctx).in_sg, keys.as_mut_ptr() as *mut u8, &mut len,
                             (*nx_ctx).ap.sglen);
    if len != core::mem::size_of_val(&keys) as c_int { return -EINVAL; }
    out_sg = nx_build_sg_list((*nx_ctx).out_sg, keys.as_mut_ptr() as *mut u8, &mut len,
                              (*nx_ctx).ap.sglen);
    if len != core::mem::size_of_val(&keys) as c_int { return -EINVAL; }

    (*nx_ctx).op.inlen = ((*nx_ctx).in_sg.offset_from(in_sg) as usize) * core::mem::size_of::<nx_sg>();
    (*nx_ctx).op.outlen = ((*nx_ctx).out_sg.offset_from(out_sg) as usize) * core::mem::size_of::<nx_sg>();
    rc = nx_hcall_sync(nx_ctx, &mut (*nx_ctx).op, 0);
    if rc != 0 { goto_out!(out); }
    atomic_inc(&mut (*(*nx_ctx).stats).aes_ops);

    keys[1][0] ^= 0x80;
    len = AES_BLOCK_SIZE as c_int;
    memcpy((*csbcpb).cpb.aes_ecb.key.as_mut_ptr(), keys[0].as_ptr(), AES_BLOCK_SIZE);
    in_sg = nx_build_sg_list((*nx_ctx).in_sg, keys[1].as_mut_ptr(), &mut len, (*nx_ctx).ap.sglen);
    if len != AES_BLOCK_SIZE as c_int { return -EINVAL; }
    len = AES_BLOCK_SIZE as c_int;
    out_sg = nx_build_sg_list((*nx_ctx).out_sg, out, &mut len, (*nx_ctx).ap.sglen);
    if len != AES_BLOCK_SIZE as c_int { return -EINVAL; }
    (*nx_ctx).op.inlen = ((*nx_ctx).in_sg.offset_from(in_sg) as usize) * core::mem::size_of::<nx_sg>();
    (*nx_ctx).op.outlen = ((*nx_ctx).out_sg.offset_from(out_sg) as usize) * core::mem::size_of::<nx_sg>();
    rc = nx_hcall_sync(nx_ctx, &mut (*nx_ctx).op, 0);
    if rc != 0 { goto_out!(out); }
    atomic_inc(&mut (*(*nx_ctx).stats).aes_ops);

out:
    (*csbcpb).cpb.hdr.mode = NX_MODE_AES_XCBC_MAC;
    memcpy((*csbcpb).cpb.aes_xcbc.key.as_mut_ptr(), key.as_ptr(), AES_BLOCK_SIZE);
    NX_CPB_FDM(csbcpb) &= !NX_FDM_ENDE_ENCRYPT;
    rc
}

unsafe fn nx_crypto_ctx_aes_xcbc_init2(tfm: *mut crypto_shash) -> c_int {
    let nx_ctx = crypto_shash_ctx(tfm);
    let csbcpb = (*nx_ctx).csbcpb;
    let err = nx_crypto_ctx_aes_xcbc_init(tfm);
    if err != 0 { return err; }
    nx_ctx_init(nx_ctx, HCOP_FC_AES);
    NX_CPB_SET_KEY_SIZE(csbcpb, NX_KS_AES_128);
    (*csbcpb).cpb.hdr.mode = NX_MODE_AES_XCBC_MAC;
    0
}

unsafe fn nx_xcbc_init(desc: *mut shash_desc) -> c_int {
    let sctx = shash_desc_ctx(desc) as *mut XcbcState;
    memset(sctx as *mut u8, 0, core::mem::size_of::<XcbcState>());
    0
}

unsafe fn nx_xcbc_update(desc: *mut shash_desc, data: *const u8, len: c_uint) -> c_int {
    let nx_ctx = crypto_shash_ctx((*desc).tfm);
    let sctx = shash_desc_ctx(desc) as *mut XcbcState;
    let csbcpb = (*nx_ctx).csbcpb;
    let mut in_sg: *mut nx_sg;
    let mut out_sg: *mut nx_sg;
    let mut max_sg_len: c_uint;
    let mut irq_flags: c_ulong = 0;
    let mut to_process: u32;
    let mut total = len;
    let mut rc: c_int = 0;
    let mut data_len: c_int;

    spin_lock_irqsave(&mut (*nx_ctx).lock, &mut irq_flags);
    memcpy((*csbcpb).cpb.aes_xcbc.out_cv_mac.as_mut_ptr(), (*sctx).state.as_ptr(), AES_BLOCK_SIZE);
    NX_CPB_FDM(csbcpb) |= NX_FDM_INTERMEDIATE | NX_FDM_CONTINUATION;
    in_sg = (*nx_ctx).in_sg;
    max_sg_len = core::cmp::min(nx_driver.of_.max_sg_len / core::mem::size_of::<nx_sg>(), (*nx_ctx).ap.sglen);
    max_sg_len = core::cmp::min(max_sg_len, (*nx_ctx).ap.databytelen / NX_PAGE_SIZE);
    data_len = AES_BLOCK_SIZE as c_int;
    out_sg = nx_build_sg_list((*nx_ctx).out_sg, (*sctx).state.as_mut_ptr(), &mut data_len, (*nx_ctx).ap.sglen);
    if data_len != AES_BLOCK_SIZE as c_int { rc = -EINVAL; goto_out_unlock!(out); }
    (*nx_ctx).op.outlen = ((*nx_ctx).out_sg.offset_from(out_sg) as usize) * core::mem::size_of::<nx_sg>();
    loop {
        to_process = total & !(AES_BLOCK_SIZE as u32 - 1);
        in_sg = nx_build_sg_list(in_sg, data, &mut to_process as *mut u32 as *mut c_int, max_sg_len);
        (*nx_ctx).op.inlen = ((*nx_ctx).in_sg.offset_from(in_sg) as usize) * core::mem::size_of::<nx_sg>();
        memcpy((*csbcpb).cpb.aes_xcbc.cv.as_mut_ptr(), (*csbcpb).cpb.aes_xcbc.out_cv_mac.as_ptr(), AES_BLOCK_SIZE);
        if (*nx_ctx).op.inlen == 0 || (*nx_ctx).op.outlen == 0 { rc = -EINVAL; goto_out_unlock!(out); }
        rc = nx_hcall_sync(nx_ctx, &mut (*nx_ctx).op, 0);
        if rc != 0 { goto_out_unlock!(out); }
        atomic_inc(&mut (*(*nx_ctx).stats).aes_ops);
        total -= to_process;
        data = data.add(to_process as usize);
        in_sg = (*nx_ctx).in_sg;
        if total < AES_BLOCK_SIZE as u32 { break; }
    }
    rc = total as c_int;
    memcpy((*sctx).state.as_mut_ptr(), (*csbcpb).cpb.aes_xcbc.out_cv_mac.as_ptr(), AES_BLOCK_SIZE);
out:
    spin_unlock_irqrestore(&mut (*nx_ctx).lock, irq_flags);
    rc
}

unsafe fn nx_xcbc_finup(desc: *mut shash_desc, src: *const u8, nbytes: c_uint, out: *mut u8) -> c_int {
    let nx_ctx = crypto_shash_ctx((*desc).tfm);
    let sctx = shash_desc_ctx(desc) as *mut XcbcState;
    let csbcpb = (*nx_ctx).csbcpb;
    let mut in_sg: *mut nx_sg;
    let mut out_sg: *mut nx_sg;
    let mut irq_flags: c_ulong = 0;
    let mut rc: c_int = 0;
    let mut len: c_int;
    spin_lock_irqsave(&mut (*nx_ctx).lock, &mut irq_flags);
    if nbytes != 0 {
        memcpy((*csbcpb).cpb.aes_xcbc.cv.as_mut_ptr(), (*sctx).state.as_ptr(), AES_BLOCK_SIZE);
    } else {
        rc = nx_xcbc_empty(desc, out);
        goto_out_unlock!(out);
    }
    NX_CPB_FDM(csbcpb) &= !NX_FDM_INTERMEDIATE;
    len = nbytes as c_int;
    in_sg = nx_build_sg_list((*nx_ctx).in_sg, src as *mut u8, &mut len, (*nx_ctx).ap.sglen);
    if len != nbytes as c_int { rc = -EINVAL; goto_out_unlock!(out); }
    len = AES_BLOCK_SIZE as c_int;
    out_sg = nx_build_sg_list((*nx_ctx).out_sg, out, &mut len, (*nx_ctx).ap.sglen);
    if len != AES_BLOCK_SIZE as c_int { rc = -EINVAL; goto_out_unlock!(out); }
    (*nx_ctx).op.inlen = ((*nx_ctx).in_sg.offset_from(in_sg) as usize) * core::mem::size_of::<nx_sg>();
    (*nx_ctx).op.outlen = ((*nx_ctx).out_sg.offset_from(out_sg) as usize) * core::mem::size_of::<nx_sg>();
    if (*nx_ctx).op.outlen == 0 { rc = -EINVAL; goto_out_unlock!(out); }
    rc = nx_hcall_sync(nx_ctx, &mut (*nx_ctx).op, 0);
    if rc != 0 { goto_out_unlock!(out); }
    atomic_inc(&mut (*(*nx_ctx).stats).aes_ops);
    memcpy(out, (*csbcpb).cpb.aes_xcbc.out_cv_mac.as_ptr(), AES_BLOCK_SIZE);
out:
    spin_unlock_irqrestore(&mut (*nx_ctx).lock, irq_flags);
    rc
}

// The following descriptor mirrors the C shash algorithm registration.
static mut nx_shash_aes_xcbc_alg: shash_alg = shash_alg {
    digestsize: AES_BLOCK_SIZE,
    init: Some(nx_xcbc_init), update: Some(nx_xcbc_update), finup: Some(nx_xcbc_finup),
    setkey: Some(nx_xcbc_set_key), descsize: core::mem::size_of::<XcbcState>(),
    init_tfm: Some(nx_crypto_ctx_aes_xcbc_init2), exit_tfm: Some(nx_crypto_ctx_shash_exit),
    base: crypto_alg {
        cra_name: c"xcbc(aes)", cra_driver_name: c"xcbc-aes-nx", cra_priority: 300,
        cra_flags: CRYPTO_AHASH_ALG_BLOCK_ONLY | CRYPTO_AHASH_ALG_FINAL_NONZERO,
        cra_blocksize: AES_BLOCK_SIZE, cra_module: THIS_MODULE,
        cra_ctxsize: core::mem::size_of::<nx_crypto_ctx>(),
    },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
