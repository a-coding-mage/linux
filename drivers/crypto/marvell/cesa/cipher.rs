// SPDX-License-Identifier: GPL-2.0-only
/*
 * Cipher algorithms supported by the CESA: DES, 3DES and AES.
 *
 * Author: Boris Brezillon <boris.brezillon@free-electrons.com>
 * Author: Arnaud Ebalard <arno@natisbad.org>
 *
 * This work is based on an initial version written by
 * Sebastian Andrzej Siewior < sebastian at breakpoint dot cc >
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
struct mv_cesa_des_ctx { base: mv_cesa_ctx, key: [u8; DES_KEY_SIZE as usize] }
#[repr(C)]
struct mv_cesa_des3_ctx { base: mv_cesa_ctx, key: [u8; DES3_EDE_KEY_SIZE as usize] }
#[repr(C)]
struct mv_cesa_aes_ctx { base: mv_cesa_ctx, aes: crypto_aes_ctx }
#[repr(C)]
struct mv_cesa_skcipher_dma_iter { base: mv_cesa_dma_iter, src: mv_cesa_sg_dma_iter, dst: mv_cesa_sg_dma_iter }

#[inline]
unsafe fn mv_cesa_skcipher_req_iter_init(iter: *mut mv_cesa_skcipher_dma_iter, req: *mut skcipher_request) {
    mv_cesa_req_dma_iter_init(&mut (*iter).base, (*req).cryptlen);
    mv_cesa_sg_dma_iter_init(&mut (*iter).src, (*req).src, DMA_TO_DEVICE);
    mv_cesa_sg_dma_iter_init(&mut (*iter).dst, (*req).dst, DMA_FROM_DEVICE);
}

#[inline]
unsafe fn mv_cesa_skcipher_req_iter_next_op(iter: *mut mv_cesa_skcipher_dma_iter) -> bool {
    (*iter).src.op_offset = 0;
    (*iter).dst.op_offset = 0;
    mv_cesa_req_dma_iter_next_op(&mut (*iter).base)
}

#[inline]
unsafe fn mv_cesa_skcipher_dma_cleanup(req: *mut skcipher_request) {
    let creq = skcipher_request_ctx(req);
    if (*req).dst != (*req).src {
        dma_unmap_sg((*cesa_dev).dev, (*req).dst, (*creq).dst_nents, DMA_FROM_DEVICE);
        dma_unmap_sg((*cesa_dev).dev, (*req).src, (*creq).src_nents, DMA_TO_DEVICE);
    } else {
        dma_unmap_sg((*cesa_dev).dev, (*req).src, (*creq).src_nents, DMA_BIDIRECTIONAL);
    }
    mv_cesa_dma_cleanup(&mut (*creq).base);
}

#[inline]
unsafe fn mv_cesa_skcipher_cleanup(req: *mut skcipher_request) {
    let creq = skcipher_request_ctx(req);
    let engine = (*creq).base.engine;
    if mv_cesa_req_get_type(&(*creq).base) == CESA_DMA_REQ { mv_cesa_skcipher_dma_cleanup(req); }
    atomic_sub((*req).cryptlen, &mut (*engine).load);
}

unsafe fn mv_cesa_skcipher_std_step(req: *mut skcipher_request) {
    let creq = skcipher_request_ctx(req);
    let sreq = &mut (*creq).std;
    let engine = (*creq).base.engine;
    let mut len = min_t((*req).cryptlen - sreq.offset, CESA_SA_SRAM_PAYLOAD_SIZE);
    mv_cesa_adjust_op(engine, &mut sreq.op);
    if (*engine).pool { memcpy((*engine).sram_pool, sreq as *const _, core::mem::size_of_val(&sreq.op)); }
    else { memcpy_toio((*engine).sram, sreq as *const _, core::mem::size_of_val(&sreq.op)); }
    len = mv_cesa_sg_copy_to_sram(engine, (*req).src, (*creq).src_nents, CESA_SA_DATA_SRAM_OFFSET, len, sreq.offset);
    sreq.size = len;
    mv_cesa_set_crypt_op_len(&mut sreq.op, len);
    /* FIXME: only update enc_len field */
    if !sreq.skip_ctx {
        if (*engine).pool { memcpy((*engine).sram_pool, &sreq.op as *const _, core::mem::size_of_val(&sreq.op)); }
        else { memcpy_toio((*engine).sram, &sreq.op as *const _, core::mem::size_of_val(&sreq.op)); }
        sreq.skip_ctx = true;
    } else if (*engine).pool { memcpy((*engine).sram_pool, &sreq.op as *const _, core::mem::size_of_val(&sreq.op.desc)); }
    else { memcpy_toio((*engine).sram, &sreq.op as *const _, core::mem::size_of_val(&sreq.op.desc)); }
    mv_cesa_set_int_mask(engine, CESA_SA_INT_ACCEL0_DONE);
    writel_relaxed(CESA_SA_CFG_PARA_DIS, (*engine).regs.add(CESA_SA_CFG));
    WARN_ON(readl((*engine).regs.add(CESA_SA_CMD)) & CESA_SA_CMD_EN_CESA_SA_ACCL0);
    writel(CESA_SA_CMD_EN_CESA_SA_ACCL0, (*engine).regs.add(CESA_SA_CMD));
}

unsafe fn mv_cesa_skcipher_std_process(req: *mut skcipher_request, _status: u32) -> i32 {
    let creq = skcipher_request_ctx(req);
    let sreq = &mut (*creq).std;
    let len = mv_cesa_sg_copy_from_sram((*creq).base.engine, (*req).dst, (*creq).dst_nents, CESA_SA_DATA_SRAM_OFFSET, sreq.size, sreq.offset);
    sreq.offset += len;
    if sreq.offset < (*req).cryptlen { return -EINPROGRESS; }
    0
}

unsafe fn mv_cesa_skcipher_process(req: *mut crypto_async_request, status: u32) -> i32 {
    let skreq = skcipher_request_cast(req);
    let creq = skcipher_request_ctx(skreq);
    if mv_cesa_req_get_type(&(*creq).base) == CESA_STD_REQ { mv_cesa_skcipher_std_process(skreq, status) } else { mv_cesa_dma_process(&mut (*creq).base, status) }
}

unsafe fn mv_cesa_skcipher_step(req: *mut crypto_async_request) {
    let skreq = skcipher_request_cast(req);
    let creq = skcipher_request_ctx(skreq);
    if mv_cesa_req_get_type(&(*creq).base) == CESA_DMA_REQ { mv_cesa_dma_step(&mut (*creq).base); } else { mv_cesa_skcipher_std_step(skreq); }
}

#[inline] unsafe fn mv_cesa_skcipher_dma_prepare(req: *mut skcipher_request) { let creq = skcipher_request_ctx(req); mv_cesa_dma_prepare(&mut (*creq).base, (*creq).base.engine); }
#[inline] unsafe fn mv_cesa_skcipher_std_prepare(req: *mut skcipher_request) { let creq = skcipher_request_ctx(req); (*creq).std.size = 0; (*creq).std.offset = 0; }
#[inline] unsafe fn mv_cesa_skcipher_prepare(req: *mut crypto_async_request, engine: *mut mv_cesa_engine) { let skreq = skcipher_request_cast(req); let creq = skcipher_request_ctx(skreq); (*creq).base.engine = engine; if mv_cesa_req_get_type(&(*creq).base) == CESA_DMA_REQ { mv_cesa_skcipher_dma_prepare(skreq); } else { mv_cesa_skcipher_std_prepare(skreq); } }
#[inline] unsafe fn mv_cesa_skcipher_req_cleanup(req: *mut crypto_async_request) { mv_cesa_skcipher_cleanup(skcipher_request_cast(req)); }

unsafe fn mv_cesa_skcipher_complete(req: *mut crypto_async_request) {
    let skreq = skcipher_request_cast(req); let creq = skcipher_request_ctx(skreq); let engine = (*creq).base.engine;
    let ivsize = crypto_skcipher_ivsize(crypto_skcipher_reqtfm(skreq));
    if mv_cesa_req_get_type(&(*creq).base) == CESA_DMA_REQ { memcpy((*skreq).iv, (*(*creq).base.chain.last).op.as_ref().unwrap().ctx.skcipher.iv.as_ptr(), ivsize); }
    else if (*engine).pool { memcpy((*skreq).iv, (*engine).sram_pool.add(CESA_SA_CRYPT_IV_SRAM_OFFSET), ivsize); }
    else { memcpy_fromio((*skreq).iv, (*engine).sram.add(CESA_SA_CRYPT_IV_SRAM_OFFSET), ivsize); }
}

static mut mv_cesa_skcipher_req_ops: mv_cesa_req_ops = mv_cesa_req_ops { step: Some(mv_cesa_skcipher_step), process: Some(mv_cesa_skcipher_process), cleanup: Some(mv_cesa_skcipher_req_cleanup), complete: Some(mv_cesa_skcipher_complete) };

unsafe fn mv_cesa_skcipher_cra_exit(tfm: *mut crypto_tfm) { let ctx = crypto_tfm_ctx(tfm); memzero_explicit(ctx, (*(*tfm).__crt_alg).cra_ctxsize); }
unsafe fn mv_cesa_skcipher_cra_init(tfm: *mut crypto_tfm) -> i32 { let ctx = crypto_tfm_ctx(tfm); (*ctx).ops = &mut mv_cesa_skcipher_req_ops; crypto_skcipher_set_reqsize(__crypto_skcipher_cast(tfm), core::mem::size_of::<mv_cesa_skcipher_req>()); 0 }

unsafe fn mv_cesa_aes_setkey(cipher: *mut crypto_skcipher, key: *const u8, len: u32) -> i32 {
    let ctx = crypto_tfm_ctx(crypto_skcipher_tfm(cipher)); let ret = aes_expandkey(&mut (*ctx).aes, key, len); if ret != 0 { return ret; }
    let remaining = ((*ctx).aes.key_length - 16) / 4; let offset = (*ctx).aes.key_length + 24 - remaining;
    for i in 0..remaining { (*ctx).aes.key_dec[4 + i] = (*ctx).aes.key_enc[offset + i]; } 0
}
unsafe fn mv_cesa_des_setkey(cipher: *mut crypto_skcipher, key: *const u8, _len: u32) -> i32 { let ctx = crypto_skcipher_ctx(cipher); let err = verify_skcipher_des_key(cipher, key); if err != 0 { return err; } memcpy((*ctx).key.as_mut_ptr(), key, DES_KEY_SIZE); 0 }
unsafe fn mv_cesa_des3_ede_setkey(cipher: *mut crypto_skcipher, key: *const u8, _len: u32) -> i32 { let ctx = crypto_skcipher_ctx(cipher); let err = verify_skcipher_des3_key(cipher, key); if err != 0 { return err; } memcpy((*ctx).key.as_mut_ptr(), key, DES3_EDE_KEY_SIZE); 0 }

unsafe fn mv_cesa_skcipher_dma_req_init(req: *mut skcipher_request, op_templ: *const mv_cesa_op_ctx) -> i32 {
    let creq = skcipher_request_ctx(req); let flags = if (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP != 0 { GFP_KERNEL } else { GFP_ATOMIC }; let basereq = &mut (*creq).base; let mut iter: mv_cesa_skcipher_dma_iter = core::mem::zeroed(); let mut skip_ctx = false;
    (*basereq).chain.first = core::ptr::null_mut(); (*basereq).chain.last = core::ptr::null_mut();
    if (*req).src != (*req).dst { if dma_map_sg((*cesa_dev).dev, (*req).src, (*creq).src_nents, DMA_TO_DEVICE) == 0 { return -ENOMEM; } if dma_map_sg((*cesa_dev).dev, (*req).dst, (*creq).dst_nents, DMA_FROM_DEVICE) == 0 { dma_unmap_sg((*cesa_dev).dev, (*req).src, (*creq).src_nents, DMA_TO_DEVICE); return -ENOMEM; } }
    else if dma_map_sg((*cesa_dev).dev, (*req).src, (*creq).src_nents, DMA_BIDIRECTIONAL) == 0 { return -ENOMEM; }
    mv_cesa_tdma_desc_iter_init(&mut (*basereq).chain); mv_cesa_skcipher_req_iter_init(&mut iter, req);
    loop { let op = mv_cesa_dma_add_op(&mut (*basereq).chain, op_templ, skip_ctx, flags); if IS_ERR(op) { let ret = PTR_ERR(op); mv_cesa_dma_cleanup(basereq); return ret; } skip_ctx = true; mv_cesa_set_crypt_op_len(op, iter.base.op_len); let mut ret = mv_cesa_dma_add_op_transfers(&mut (*basereq).chain, &mut iter.base, &mut iter.src, flags); if ret != 0 { mv_cesa_dma_cleanup(basereq); return ret; } ret = mv_cesa_dma_add_dummy_launch(&mut (*basereq).chain, flags); if ret != 0 { mv_cesa_dma_cleanup(basereq); return ret; } ret = mv_cesa_dma_add_op_transfers(&mut (*basereq).chain, &mut iter.base, &mut iter.dst, flags); if ret != 0 { mv_cesa_dma_cleanup(basereq); return ret; } if !mv_cesa_skcipher_req_iter_next_op(&mut iter) { break; } }
    let ret = mv_cesa_dma_add_result_op(&mut (*basereq).chain, CESA_SA_CFG_SRAM_OFFSET, CESA_SA_DATA_SRAM_OFFSET, CESA_TDMA_SRC_IN_SRAM, flags); if ret != 0 { mv_cesa_dma_cleanup(basereq); return ret; } (*(*basereq).chain.last).flags |= CESA_TDMA_END_OF_REQ; 0
}

#[inline] unsafe fn mv_cesa_skcipher_std_req_init(req: *mut skcipher_request, tmpl: *const mv_cesa_op_ctx) -> i32 { let creq = skcipher_request_ctx(req); (*creq).std.op = *tmpl; (*creq).std.skip_ctx = false; (*creq).base.chain.first = core::ptr::null_mut(); (*creq).base.chain.last = core::ptr::null_mut(); 0 }
unsafe fn mv_cesa_skcipher_req_init(req: *mut skcipher_request, tmpl: *mut mv_cesa_op_ctx) -> i32 { let creq = skcipher_request_ctx(req); let blksize = crypto_skcipher_blocksize(crypto_skcipher_reqtfm(req)); if (*req).cryptlen % blksize != 0 { return -EINVAL; } (*creq).src_nents = sg_nents_for_len((*req).src, (*req).cryptlen); if (*creq).src_nents < 0 { dev_err((*cesa_dev).dev, "Invalid number of src SG"); return (*creq).src_nents; } (*creq).dst_nents = sg_nents_for_len((*req).dst, (*req).cryptlen); if (*creq).dst_nents < 0 { dev_err((*cesa_dev).dev, "Invalid number of dst SG"); return (*creq).dst_nents; } mv_cesa_update_op_cfg(tmpl, CESA_SA_DESC_CFG_OP_CRYPT_ONLY, CESA_SA_DESC_CFG_OP_MSK); if (*(*cesa_dev).caps).has_tdma { mv_cesa_skcipher_dma_req_init(req, tmpl) } else { mv_cesa_skcipher_std_req_init(req, tmpl) } }
unsafe fn mv_cesa_skcipher_queue_req(req: *mut skcipher_request, tmpl: *mut mv_cesa_op_ctx) -> i32 { if (*req).cryptlen == 0 { return 0; } let ret = mv_cesa_skcipher_req_init(req, tmpl); if ret != 0 { return ret; } let engine = mv_cesa_select_engine((*req).cryptlen); mv_cesa_skcipher_prepare(&mut (*req).base, engine); let ret = mv_cesa_queue_req(&mut (*req).base, &mut (*skcipher_request_ctx(req)).base); if mv_cesa_req_needs_cleanup(&(*req).base, ret) { mv_cesa_skcipher_cleanup(req); } ret }

unsafe fn mv_cesa_des_op(req: *mut skcipher_request, tmpl: *mut mv_cesa_op_ctx) -> i32 { let ctx = crypto_tfm_ctx((*req).base.tfm); mv_cesa_update_op_cfg(tmpl, CESA_SA_DESC_CFG_CRYPTM_DES, CESA_SA_DESC_CFG_CRYPTM_MSK); memcpy((*tmpl).ctx.skcipher.key.as_mut_ptr(), (*ctx).key.as_ptr(), DES_KEY_SIZE); mv_cesa_skcipher_queue_req(req, tmpl) }
unsafe fn mv_cesa_des3_op(req: *mut skcipher_request, tmpl: *mut mv_cesa_op_ctx) -> i32 { let ctx = crypto_tfm_ctx((*req).base.tfm); mv_cesa_update_op_cfg(tmpl, CESA_SA_DESC_CFG_CRYPTM_3DES, CESA_SA_DESC_CFG_CRYPTM_MSK); memcpy((*tmpl).ctx.skcipher.key.as_mut_ptr(), (*ctx).key.as_ptr(), DES3_EDE_KEY_SIZE); mv_cesa_skcipher_queue_req(req, tmpl) }
unsafe fn mv_cesa_aes_op(req: *mut skcipher_request, tmpl: *mut mv_cesa_op_ctx) -> i32 { let ctx = crypto_tfm_ctx((*req).base.tfm); let key = if mv_cesa_get_op_cfg(tmpl) & CESA_SA_DESC_CFG_DIR_DEC != 0 { (*ctx).aes.key_dec.as_ptr() } else { (*ctx).aes.key_enc.as_ptr() }; for i in 0..((*ctx).aes.key_length / core::mem::size_of::<u32>()) { (*tmpl).ctx.skcipher.key[i] = cpu_to_le32(*key.add(i)); } let mut cfg = CESA_SA_DESC_CFG_CRYPTM_AES; if (*ctx).aes.key_length == 24 { cfg |= CESA_SA_DESC_CFG_AES_LEN_192; } else if (*ctx).aes.key_length == 32 { cfg |= CESA_SA_DESC_CFG_AES_LEN_256; } mv_cesa_update_op_cfg(tmpl, cfg, CESA_SA_DESC_CFG_CRYPTM_MSK | CESA_SA_DESC_CFG_AES_LEN_MSK); mv_cesa_skcipher_queue_req(req, tmpl) }

macro_rules! cipher_mode_fn { ($name:ident, $op:ident, $cfg:expr) => { unsafe fn $name(req: *mut skcipher_request) -> i32 { let mut t: mv_cesa_op_ctx = core::mem::zeroed(); mv_cesa_set_op_cfg(&mut t, $cfg); $op(req, &mut t) } }; }
cipher_mode_fn!(mv_cesa_ecb_des_encrypt, mv_cesa_des_op, CESA_SA_DESC_CFG_CRYPTCM_ECB | CESA_SA_DESC_CFG_DIR_ENC);
cipher_mode_fn!(mv_cesa_ecb_des_decrypt, mv_cesa_des_op, CESA_SA_DESC_CFG_CRYPTCM_ECB | CESA_SA_DESC_CFG_DIR_DEC);
cipher_mode_fn!(mv_cesa_ecb_des3_ede_encrypt, mv_cesa_des3_op, CESA_SA_DESC_CFG_CRYPTCM_ECB | CESA_SA_DESC_CFG_3DES_EDE | CESA_SA_DESC_CFG_DIR_ENC);
cipher_mode_fn!(mv_cesa_ecb_des3_ede_decrypt, mv_cesa_des3_op, CESA_SA_DESC_CFG_CRYPTCM_ECB | CESA_SA_DESC_CFG_3DES_EDE | CESA_SA_DESC_CFG_DIR_DEC);
cipher_mode_fn!(mv_cesa_ecb_aes_encrypt, mv_cesa_aes_op, CESA_SA_DESC_CFG_CRYPTCM_ECB | CESA_SA_DESC_CFG_DIR_ENC);
cipher_mode_fn!(mv_cesa_ecb_aes_decrypt, mv_cesa_aes_op, CESA_SA_DESC_CFG_CRYPTCM_ECB | CESA_SA_DESC_CFG_DIR_DEC);

unsafe fn mv_cesa_cbc_des_op(req: *mut skcipher_request, t: *mut mv_cesa_op_ctx) -> i32 { mv_cesa_update_op_cfg(t, CESA_SA_DESC_CFG_CRYPTCM_CBC, CESA_SA_DESC_CFG_CRYPTCM_MSK); memcpy((*t).ctx.skcipher.iv.as_mut_ptr(), (*req).iv, DES_BLOCK_SIZE); mv_cesa_des_op(req,t) }
unsafe fn mv_cesa_cbc_des3_op(req: *mut skcipher_request, t: *mut mv_cesa_op_ctx) -> i32 { memcpy((*t).ctx.skcipher.iv.as_mut_ptr(), (*req).iv, DES3_EDE_BLOCK_SIZE); mv_cesa_des3_op(req,t) }
unsafe fn mv_cesa_cbc_aes_op(req: *mut skcipher_request, t: *mut mv_cesa_op_ctx) -> i32 { mv_cesa_update_op_cfg(t, CESA_SA_DESC_CFG_CRYPTCM_CBC, CESA_SA_DESC_CFG_CRYPTCM_MSK); memcpy((*t).ctx.skcipher.iv.as_mut_ptr(), (*req).iv, AES_BLOCK_SIZE); mv_cesa_aes_op(req,t) }
cipher_mode_fn!(mv_cesa_cbc_des_encrypt, mv_cesa_cbc_des_op, CESA_SA_DESC_CFG_DIR_ENC);
cipher_mode_fn!(mv_cesa_cbc_des_decrypt, mv_cesa_cbc_des_op, CESA_SA_DESC_CFG_DIR_DEC);
cipher_mode_fn!(mv_cesa_cbc_des3_ede_encrypt, mv_cesa_cbc_des3_op, CESA_SA_DESC_CFG_CRYPTCM_CBC | CESA_SA_DESC_CFG_3DES_EDE | CESA_SA_DESC_CFG_DIR_ENC);
cipher_mode_fn!(mv_cesa_cbc_des3_ede_decrypt, mv_cesa_cbc_des3_op, CESA_SA_DESC_CFG_CRYPTCM_CBC | CESA_SA_DESC_CFG_3DES_EDE | CESA_SA_DESC_CFG_DIR_DEC);
cipher_mode_fn!(mv_cesa_cbc_aes_encrypt, mv_cesa_cbc_aes_op, CESA_SA_DESC_CFG_DIR_ENC);
cipher_mode_fn!(mv_cesa_cbc_aes_decrypt, mv_cesa_cbc_aes_op, CESA_SA_DESC_CFG_DIR_DEC);

// The six exported algorithm descriptors preserve the corresponding C names and
// callback/size metadata; their field layout is supplied by the kernel bindings.
macro_rules! alg_desc {
    ($name:ident, $set:ident, $enc:ident, $dec:ident, $min:expr, $max:expr, $iv:expr, $n:expr, $d:expr, $bs:expr, $ctx:ty) => {
        #[no_mangle]
        static mut $name: skcipher_alg = skcipher_alg {
            setkey: Some($set), encrypt: Some($enc), decrypt: Some($dec),
            min_keysize: $min, max_keysize: $max, ivsize: $iv,
            base: crypto_alg { cra_name: $n, cra_driver_name: $d, cra_priority: 300,
                cra_flags: CRYPTO_ALG_KERN_DRIVER_ONLY | CRYPTO_ALG_ASYNC | CRYPTO_ALG_ALLOCATES_MEMORY,
                cra_blocksize: $bs, cra_ctxsize: core::mem::size_of::<$ctx>(), cra_alignmask: 0,
                cra_module: THIS_MODULE, cra_init: Some(mv_cesa_skcipher_cra_init), cra_exit: Some(mv_cesa_skcipher_cra_exit) }
        };
    };
}
alg_desc!(mv_cesa_ecb_des_alg, mv_cesa_des_setkey, mv_cesa_ecb_des_encrypt, mv_cesa_ecb_des_decrypt, DES_KEY_SIZE, DES_KEY_SIZE, 0, "ecb(des)", "mv-ecb-des", DES_BLOCK_SIZE, mv_cesa_des_ctx);
alg_desc!(mv_cesa_cbc_des_alg, mv_cesa_des_setkey, mv_cesa_cbc_des_encrypt, mv_cesa_cbc_des_decrypt, DES_KEY_SIZE, DES_KEY_SIZE, DES_BLOCK_SIZE, "cbc(des)", "mv-cbc-des", DES_BLOCK_SIZE, mv_cesa_des_ctx);
alg_desc!(mv_cesa_ecb_des3_ede_alg, mv_cesa_des3_ede_setkey, mv_cesa_ecb_des3_ede_encrypt, mv_cesa_ecb_des3_ede_decrypt, DES3_EDE_KEY_SIZE, DES3_EDE_KEY_SIZE, 0, "ecb(des3_ede)", "mv-ecb-des3-ede", DES3_EDE_BLOCK_SIZE, mv_cesa_des3_ctx);
alg_desc!(mv_cesa_cbc_des3_ede_alg, mv_cesa_des3_ede_setkey, mv_cesa_cbc_des3_ede_encrypt, mv_cesa_cbc_des3_ede_decrypt, DES3_EDE_KEY_SIZE, DES3_EDE_KEY_SIZE, DES3_EDE_BLOCK_SIZE, "cbc(des3_ede)", "mv-cbc-des3-ede", DES3_EDE_BLOCK_SIZE, mv_cesa_des3_ctx);
alg_desc!(mv_cesa_ecb_aes_alg, mv_cesa_aes_setkey, mv_cesa_ecb_aes_encrypt, mv_cesa_ecb_aes_decrypt, AES_MIN_KEY_SIZE, AES_MAX_KEY_SIZE, 0, "ecb(aes)", "mv-ecb-aes", AES_BLOCK_SIZE, mv_cesa_aes_ctx);
alg_desc!(mv_cesa_cbc_aes_alg, mv_cesa_aes_setkey, mv_cesa_cbc_aes_encrypt, mv_cesa_cbc_aes_decrypt, AES_MIN_KEY_SIZE, AES_MAX_KEY_SIZE, AES_BLOCK_SIZE, "cbc(aes)", "mv-cbc-aes", AES_BLOCK_SIZE, mv_cesa_aes_ctx);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
