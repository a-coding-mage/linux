// SPDX-License-Identifier: GPL-2.0-only
/*
 * AES CBC routines supporting the Power 7+ Nest Accelerators driver
 *
 * Copyright (C) 2011-2012 International Business Machines Inc.
 *
 * Author: Kent Yoder <yoder1@us.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn cbc_aes_nx_set_key(
    tfm: *mut crypto_skcipher,
    in_key: *const u8,
    key_len: c_uint,
) -> c_int {
    let nx_ctx: *mut nx_crypto_ctx = crypto_skcipher_ctx(tfm);
    let csbcpb: *mut nx_csbcpb = (*nx_ctx).csbcpb;

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

    (*csbcpb).cpb.hdr.mode = NX_MODE_AES_CBC;
    memcpy(
        (*csbcpb).cpb.aes_cbc.key.as_mut_ptr() as *mut c_void,
        in_key as *const c_void,
        key_len as usize,
    );

    0
}

unsafe fn cbc_aes_nx_crypt(req: *mut skcipher_request, enc: c_int) -> c_int {
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
            (*req).iv,
            (*req).dst,
            (*req).src,
            &mut to_process,
            processed,
            (*csbcpb).cpb.aes_cbc.iv.as_mut_ptr(),
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

        memcpy(
            (*req).iv as *mut c_void,
            (*csbcpb).cpb.aes_cbc.cv.as_ptr() as *const c_void,
            AES_BLOCK_SIZE as usize,
        );
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

unsafe fn cbc_aes_nx_encrypt(req: *mut skcipher_request) -> c_int {
    cbc_aes_nx_crypt(req, 1)
}

unsafe fn cbc_aes_nx_decrypt(req: *mut skcipher_request) -> c_int {
    cbc_aes_nx_crypt(req, 0)
}

#[no_mangle]
pub static mut nx_cbc_aes_alg: skcipher_alg = unsafe { core::mem::zeroed() };

// Field initialization corresponding to the C designated initializer above:
// base.cra_name = "cbc(aes)", base.cra_driver_name = "cbc-aes-nx",
// base.cra_priority = 300, base.cra_blocksize = AES_BLOCK_SIZE,
// base.cra_ctxsize = size_of::<nx_crypto_ctx>(), base.cra_alignmask = 0xf,
// base.cra_module = THIS_MODULE, init = nx_crypto_ctx_aes_cbc_init,
// exit = nx_crypto_ctx_skcipher_exit, min_keysize = AES_MIN_KEY_SIZE,
// max_keysize = AES_MAX_KEY_SIZE, ivsize = AES_BLOCK_SIZE,
// setkey = cbc_aes_nx_set_key, encrypt = cbc_aes_nx_encrypt,
// decrypt = cbc_aes_nx_decrypt.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
