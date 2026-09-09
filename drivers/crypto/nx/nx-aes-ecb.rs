// SPDX-License-Identifier: GPL-2.0-only
/*
 * AES ECB routines supporting the Power 7+ Nest Accelerators driver
 *
 * Copyright (C) 2011-2012 International Business Machines Inc.
 *
 * Author: Kent Yoder <yoder1@us.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn ecb_aes_nx_set_key(
    tfm: *mut crypto_skcipher,
    in_key: *const u8,
    key_len: c_uint,
) -> c_int {
    let nx_ctx: *mut nx_crypto_ctx = crypto_skcipher_ctx(tfm);
    let csbcpb: *mut nx_csbcpb = (*nx_ctx).csbcpb as *mut nx_csbcpb;

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

    (*csbcpb).cpb.hdr.mode = NX_MODE_AES_ECB;
    memcpy((*csbcpb).cpb.aes_ecb.key.as_mut_ptr(), in_key, key_len as usize);

    0
}

unsafe fn ecb_aes_nx_crypt(req: *mut skcipher_request, enc: c_int) -> c_int {
    let tfm: *mut crypto_skcipher = crypto_skcipher_reqtfm(req);
    let nx_ctx: *mut nx_crypto_ctx = crypto_skcipher_ctx(tfm);
    let csbcpb: *mut nx_csbcpb = (*nx_ctx).csbcpb;
    let mut irq_flags: c_ulong = 0;
    let mut processed: c_uint = 0;
    let mut to_process: c_uint;
    let mut rc: c_int;

    spin_lock_irqsave(&mut (*nx_ctx).lock, &mut irq_flags);

    if enc != 0 {
        NX_CPB_FDM(csbcpb) |= NX_FDM_ENDE_ENCRYPT;
    } else {
        NX_CPB_FDM(csbcpb) &= !NX_FDM_ENDE_ENCRYPT;
    }

    loop {
        to_process = (*req).cryptlen - processed;

        rc = nx_build_sg_lists(
            nx_ctx,
            core::ptr::null_mut(),
            (*req).dst,
            (*req).src,
            &mut to_process,
            processed,
            core::ptr::null_mut(),
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
            (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP,
        );
        if rc != 0 {
            break;
        }

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

unsafe fn ecb_aes_nx_encrypt(req: *mut skcipher_request) -> c_int {
    ecb_aes_nx_crypt(req, 1)
}

unsafe fn ecb_aes_nx_decrypt(req: *mut skcipher_request) -> c_int {
    ecb_aes_nx_crypt(req, 0)
}

#[no_mangle]
pub static mut nx_ecb_aes_alg: skcipher_alg = skcipher_alg {
    base: crypto_alg {
        cra_name: "ecb(aes)" as *const _ as *const c_char,
        cra_driver_name: "ecb-aes-nx" as *const _ as *const c_char,
        cra_priority: 300,
        cra_blocksize: AES_BLOCK_SIZE,
        cra_alignmask: 0xf,
        cra_ctxsize: core::mem::size_of::<nx_crypto_ctx>(),
        cra_module: THIS_MODULE,
    },
    init: Some(nx_crypto_ctx_aes_ecb_init),
    exit: Some(nx_crypto_ctx_skcipher_exit),
    min_keysize: AES_MIN_KEY_SIZE,
    max_keysize: AES_MAX_KEY_SIZE,
    setkey: Some(ecb_aes_nx_set_key),
    encrypt: Some(ecb_aes_nx_encrypt),
    decrypt: Some(ecb_aes_nx_decrypt),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
