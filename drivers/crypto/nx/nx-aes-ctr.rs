// SPDX-License-Identifier: GPL-2.0-only
/*
 * AES CTR routines supporting the Power 7+ Nest Accelerators driver
 *
 * Copyright (C) 2011-2012 International Business Machines Inc.
 *
 * Author: Kent Yoder <yoder1@us.ibm.com>
 */

// Dependencies supplied by the kernel crypto and NX headers are external to
// this translation unit.

unsafe fn ctr_aes_nx_set_key(
    tfm: *mut crypto_skcipher,
    in_key: *const u8,
    key_len: u32,
) -> i32 {
    let nx_ctx = crypto_skcipher_ctx(tfm);
    let csbcpb = (*nx_ctx).csbcpb;

    nx_ctx_init(nx_ctx, HCOP_FC_AES);

    match key_len {
        AES_KEYSIZE_128 => {
            NX_CPB_SET_KEY_SIZE(csbcpb, NX_KS_AES_128);
            (*nx_ctx).ap = &mut (*nx_ctx).props[NX_PROPS_AES_128];
        }
        AES_KEYSIZE_192 => {
            NX_CPB_SET_KEY_SIZE(csbcpb, NX_KS_AES_192);
            (*nx_ctx).ap = &mut (*nx_ctx).props[NX_PROPS_AES_192];
        }
        AES_KEYSIZE_256 => {
            NX_CPB_SET_KEY_SIZE(csbcpb, NX_KS_AES_256);
            (*nx_ctx).ap = &mut (*nx_ctx).props[NX_PROPS_AES_256];
        }
        _ => return -EINVAL,
    }

    (*csbcpb).cpb.hdr.mode = NX_MODE_AES_CTR;
    memcpy(
        (*csbcpb).cpb.aes_ctr.key.as_mut_ptr(),
        in_key,
        key_len as usize,
    );

    0
}

unsafe fn ctr3686_aes_nx_set_key(
    tfm: *mut crypto_skcipher,
    in_key: *const u8,
    mut key_len: u32,
) -> i32 {
    let nx_ctx = crypto_skcipher_ctx(tfm);

    if key_len < CTR_RFC3686_NONCE_SIZE {
        return -EINVAL;
    }

    memcpy(
        (*nx_ctx).priv_.ctr.nonce.as_mut_ptr(),
        in_key.add((key_len - CTR_RFC3686_NONCE_SIZE) as usize),
        CTR_RFC3686_NONCE_SIZE as usize,
    );

    key_len -= CTR_RFC3686_NONCE_SIZE;

    ctr_aes_nx_set_key(tfm, in_key, key_len)
}

unsafe fn ctr_aes_nx_crypt(req: *mut skcipher_request, iv: *mut u8) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let nx_ctx = crypto_skcipher_ctx(tfm);
    let csbcpb = (*nx_ctx).csbcpb;
    let mut irq_flags: unsigned_long = 0;
    let mut processed: u32 = 0;
    let mut to_process: u32;
    let mut rc: i32;

    spin_lock_irqsave(&mut (*nx_ctx).lock, &mut irq_flags);

    loop {
        to_process = (*req).cryptlen - processed;

        rc = nx_build_sg_lists(
            nx_ctx,
            iv,
            (*req).dst,
            (*req).src,
            &mut to_process,
            processed,
            (*csbcpb).cpb.aes_ctr.iv.as_mut_ptr(),
        );
        if rc != 0 {
            break;
        }

        if (*nx_ctx).op.inlen == 0 || (*nx_ctx).op.outlen == 0 {
            rc = -EINVAL;
            break;
        }

        rc = nx_hcall_sync(
            nx_ctx,
            &mut (*nx_ctx).op,
            ((*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP) != 0,
        );
        if rc != 0 {
            break;
        }

        memcpy(iv, (*csbcpb).cpb.aes_cbc.cv.as_ptr(), AES_BLOCK_SIZE as usize);

        atomic_inc(&mut (*(*nx_ctx).stats).aes_ops);
        atomic64_add(
            be32_to_cpu((*csbcpb).csb.processed_byte_count),
            &mut (*(*nx_ctx).stats).aes_bytes,
        );

        processed += to_process;
        if processed >= (*req).cryptlen {
            break;
        }
    }

    spin_unlock_irqrestore(&mut (*nx_ctx).lock, irq_flags);
    rc
}

unsafe fn ctr3686_aes_nx_crypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let nx_ctx = crypto_skcipher_ctx(tfm);
    let mut iv = [0u8; 16];

    memcpy(
        iv.as_mut_ptr(),
        (*nx_ctx).priv_.ctr.nonce.as_ptr(),
        CTR_RFC3686_NONCE_SIZE as usize,
    );
    memcpy(
        iv.as_mut_ptr().add(CTR_RFC3686_NONCE_SIZE as usize),
        (*req).iv,
        CTR_RFC3686_IV_SIZE as usize,
    );
    iv[12] = 0;
    iv[13] = 0;
    iv[14] = 0;
    iv[15] = 1;

    ctr_aes_nx_crypt(req, iv.as_mut_ptr())
}

static mut nx_ctr3686_aes_alg: skcipher_alg = skcipher_alg {
    base: crypto_alg {
        cra_name: c"rfc3686(ctr(aes))",
        cra_driver_name: c"rfc3686-ctr-aes-nx",
        cra_priority: 300,
        cra_blocksize: 1,
        cra_ctxsize: core::mem::size_of::<nx_crypto_ctx>(),
        cra_module: THIS_MODULE,
    },
    init: Some(nx_crypto_ctx_aes_ctr_init),
    exit: Some(nx_crypto_ctx_skcipher_exit),
    min_keysize: AES_MIN_KEY_SIZE + CTR_RFC3686_NONCE_SIZE,
    max_keysize: AES_MAX_KEY_SIZE + CTR_RFC3686_NONCE_SIZE,
    ivsize: CTR_RFC3686_IV_SIZE,
    setkey: Some(ctr3686_aes_nx_set_key),
    encrypt: Some(ctr3686_aes_nx_crypt),
    decrypt: Some(ctr3686_aes_nx_crypt),
    chunksize: AES_BLOCK_SIZE,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
