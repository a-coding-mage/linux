// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2016 Cavium, Inc.

// External kernel and driver types/functions/constants are supplied by the surrounding crate.

#[repr(C)]
struct CptDeviceHandle {
    cdev: [*mut core::ffi::c_void; MAX_DEVICES],
    dev_count: u32,
}

static mut DEV_HANDLE: CptDeviceHandle = CptDeviceHandle {
    cdev: [core::ptr::null_mut(); MAX_DEVICES],
    dev_count: 0,
};

unsafe fn cvm_callback(status: u32, arg: *mut core::ffi::c_void) {
    let req = arg as *mut crypto_async_request;
    crypto_request_complete(req, status == 0);
}

unsafe fn update_input_iv(req_info: *mut cpt_request_info, iv: *mut u8, enc_iv_len: u32, argcnt: *mut u32) {
    (*req_info).in_[*argcnt as usize].vptr = iv as *mut core::ffi::c_void;
    (*req_info).in_[*argcnt as usize].size = enc_iv_len;
    (*req_info).req.dlen += enc_iv_len;
    *argcnt += 1;
}

unsafe fn update_output_iv(req_info: *mut cpt_request_info, iv: *mut u8, enc_iv_len: u32, argcnt: *mut u32) {
    (*req_info).out[*argcnt as usize].vptr = iv as *mut core::ffi::c_void;
    (*req_info).out[*argcnt as usize].size = enc_iv_len;
    (*req_info).rlen += enc_iv_len;
    *argcnt += 1;
}

unsafe fn update_input_data(req_info: *mut cpt_request_info, mut inp_sg: *mut scatterlist, mut nbytes: u32, argcnt: *mut u32) {
    (*req_info).req.dlen += nbytes;
    while nbytes != 0 {
        let len = core::cmp::min(nbytes, (*inp_sg).length);
        let ptr = sg_virt(inp_sg);
        (*req_info).in_[*argcnt as usize].vptr = ptr as *mut core::ffi::c_void;
        (*req_info).in_[*argcnt as usize].size = len;
        nbytes -= len;
        *argcnt += 1;
        inp_sg = inp_sg.add(1);
    }
}

unsafe fn update_output_data(req_info: *mut cpt_request_info, mut outp_sg: *mut scatterlist, mut nbytes: u32, argcnt: *mut u32) {
    (*req_info).rlen += nbytes;
    while nbytes != 0 {
        let len = core::cmp::min(nbytes, (*outp_sg).length);
        let ptr = sg_virt(outp_sg);
        (*req_info).out[*argcnt as usize].vptr = ptr as *mut core::ffi::c_void;
        (*req_info).out[*argcnt as usize].size = len;
        nbytes -= len;
        *argcnt += 1;
        outp_sg = outp_sg.add(1);
    }
}

unsafe fn create_ctx_hdr(req: *mut skcipher_request, enc: u32, argcnt: *mut u32) -> u32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    let rctx = skcipher_request_ctx_dma(req);
    let fctx = &mut (*rctx).fctx;
    let enc_iv_len = crypto_skcipher_ivsize(tfm);
    let req_info = &mut (*rctx).cpt_req;

    req_info.ctrl.s.grp = 0;
    req_info.ctrl.s.dma_mode = DMA_GATHER_SCATTER;
    req_info.ctrl.s.se_req = SE_CORE_REQ;
    req_info.req.opcode.s.major = MAJOR_OP_FC | DMA_MODE_FLAG(DMA_GATHER_SCATTER);
    req_info.req.opcode.s.minor = if enc != 0 { 2 } else { 3 };
    req_info.req.param1 = (*req).cryptlen;
    req_info.req.param2 = 0;
    fctx.enc.enc_ctrl.e.enc_cipher = ctx.cipher_type;
    fctx.enc.enc_ctrl.e.aes_key = ctx.key_type;
    fctx.enc.enc_ctrl.e.iv_source = FROM_DPTR;
    let key_len = if ctx.cipher_type == AES_XTS { ctx.key_len * 2 } else { ctx.key_len };
    core::ptr::copy_nonoverlapping(ctx.enc_key.as_ptr(), fctx.enc.encr_key.as_mut_ptr(), key_len as usize);
    let ctrl_flags = &mut fctx.enc.enc_ctrl.flags as *mut _ as *mut u64;
    *ctrl_flags = cpu_to_be64(fctx.enc.enc_ctrl.flags);
    let offset_control = &mut (*rctx).control_word as *mut _ as *mut u64;
    *offset_control = cpu_to_be64((enc_iv_len as u64) << 16);
    req_info.in_[*argcnt as usize].vptr = offset_control as *mut u8 as *mut core::ffi::c_void;
    req_info.in_[*argcnt as usize].size = CONTROL_WORD_LEN;
    req_info.req.dlen += CONTROL_WORD_LEN;
    *argcnt += 1;
    req_info.in_[*argcnt as usize].vptr = fctx as *mut _ as *mut core::ffi::c_void;
    req_info.in_[*argcnt as usize].size = core::mem::size_of::<fc_context>() as u32;
    req_info.req.dlen += core::mem::size_of::<fc_context>() as u32;
    *argcnt += 1;
    0
}

unsafe fn create_input_list(req: *mut skcipher_request, enc: u32, enc_iv_len: u32) -> u32 {
    let rctx = skcipher_request_ctx_dma(req);
    let req_info = &mut (*rctx).cpt_req;
    let mut argcnt = 0;
    create_ctx_hdr(req, enc, &mut argcnt);
    update_input_iv(req_info, (*req).iv, enc_iv_len, &mut argcnt);
    update_input_data(req_info, (*req).src, (*req).cryptlen, &mut argcnt);
    req_info.incnt = argcnt;
    0
}

unsafe fn store_cb_info(req: *mut skcipher_request, req_info: *mut cpt_request_info) {
    (*req_info).callback = Some(cvm_callback);
    (*req_info).callback_arg = &mut (*req).base as *mut _ as *mut core::ffi::c_void;
}

unsafe fn create_output_list(req: *mut skcipher_request, enc_iv_len: u32) {
    let rctx = skcipher_request_ctx_dma(req);
    let req_info = &mut (*rctx).cpt_req;
    let mut argcnt = 0;
    update_output_iv(req_info, (*req).iv, enc_iv_len, &mut argcnt);
    update_output_data(req_info, (*req).dst, (*req).cryptlen, &mut argcnt);
    req_info.outcnt = argcnt;
}

unsafe fn cvm_enc_dec(req: *mut skcipher_request, enc: u32) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let rctx = skcipher_request_ctx_dma(req);
    let enc_iv_len = crypto_skcipher_ivsize(tfm);
    let req_info = &mut (*rctx).cpt_req;
    core::ptr::write_bytes(req_info as *mut _, 0, 1);
    req_info.may_sleep = ((*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP) != 0;
    core::ptr::write_bytes(&mut (*rctx).fctx as *mut _, 0, 1);
    create_input_list(req, enc, enc_iv_len);
    create_output_list(req, enc_iv_len);
    store_cb_info(req, req_info);
    let cdev = DEV_HANDLE.cdev[smp_processor_id() as usize];
    let status = cptvf_do_request(cdev, req_info);
    if status != 0 { status } else { -EINPROGRESS }
}

unsafe fn cvm_encrypt(req: *mut skcipher_request) -> i32 { cvm_enc_dec(req, 1) }
unsafe fn cvm_decrypt(req: *mut skcipher_request) -> i32 { cvm_enc_dec(req, 0) }

unsafe fn cvm_xts_setkey(cipher: *mut crypto_skcipher, key: *const u8, keylen: u32) -> i32 {
    let ctx = crypto_skcipher_ctx(cipher);
    let err = xts_verify_key(cipher, key, keylen);
    if err != 0 { return err; }
    (*ctx).key_len = keylen;
    core::ptr::copy_nonoverlapping(key, (*ctx).enc_key.as_mut_ptr(), (keylen / 2) as usize);
    core::ptr::copy_nonoverlapping(key.add((keylen / 2) as usize), (*ctx).enc_key.as_mut_ptr().add(KEY2_OFFSET as usize), (keylen / 2) as usize);
    (*ctx).cipher_type = AES_XTS;
    match (*ctx).key_len { 32 => (*ctx).key_type = AES_128_BIT, 64 => (*ctx).key_type = AES_256_BIT, _ => return -EINVAL }
    0
}

unsafe fn cvm_validate_keylen(ctx: *mut cvm_enc_ctx, keylen: u32) -> i32 {
    if keylen != 16 && keylen != 24 && keylen != 32 { return -EINVAL; }
    (*ctx).key_len = keylen;
    (*ctx).key_type = match keylen { 16 => AES_128_BIT, 24 => AES_192_BIT, 32 => AES_256_BIT, _ => return -EINVAL };
    if (*ctx).cipher_type == DES3_CBC { (*ctx).key_type = 0; }
    0
}

unsafe fn cvm_setkey(cipher: *mut crypto_skcipher, key: *const u8, keylen: u32, cipher_type: u8) -> i32 {
    let ctx = crypto_skcipher_ctx(cipher);
    (*ctx).cipher_type = cipher_type;
    if cvm_validate_keylen(ctx, keylen) == 0 {
        core::ptr::copy_nonoverlapping(key, (*ctx).enc_key.as_mut_ptr(), keylen as usize); 0
    } else { -EINVAL }
}

unsafe fn cvm_cbc_aes_setkey(c: *mut crypto_skcipher, k: *const u8, l: u32) -> i32 { cvm_setkey(c, k, l, AES_CBC) }
unsafe fn cvm_ecb_aes_setkey(c: *mut crypto_skcipher, k: *const u8, l: u32) -> i32 { cvm_setkey(c, k, l, AES_ECB) }
unsafe fn cvm_cbc_des3_setkey(c: *mut crypto_skcipher, k: *const u8, l: u32) -> i32 { let e = verify_skcipher_des3_key(c, k); if e != 0 { e } else { cvm_setkey(c, k, l, DES3_CBC) } }
unsafe fn cvm_ecb_des3_setkey(c: *mut crypto_skcipher, k: *const u8, l: u32) -> i32 { let e = verify_skcipher_des3_key(c, k); if e != 0 { e } else { cvm_setkey(c, k, l, DES3_ECB) } }

unsafe fn cvm_enc_dec_init(tfm: *mut crypto_skcipher) -> i32 {
    crypto_skcipher_set_reqsize_dma(tfm, core::mem::size_of::<cvm_req_ctx>());
    0
}

// The skcipher algorithm table is represented with the surrounding crate's C-compatible descriptor.
static mut ALGS: [skcipher_alg; 5] = [
    skcipher_alg { base: crypto_alg { cra_flags: CRYPTO_ALG_ASYNC | CRYPTO_ALG_ALLOCATES_MEMORY, cra_blocksize: AES_BLOCK_SIZE, cra_ctxsize: core::mem::size_of::<cvm_enc_ctx>(), cra_alignmask: 7, cra_priority: 4001, cra_name: c"xts(aes)", cra_driver_name: c"cavium-xts-aes", cra_module: THIS_MODULE }, ivsize: AES_BLOCK_SIZE, min_keysize: 2 * AES_MIN_KEY_SIZE, max_keysize: 2 * AES_MAX_KEY_SIZE, setkey: Some(cvm_xts_setkey), encrypt: Some(cvm_encrypt), decrypt: Some(cvm_decrypt), init: Some(cvm_enc_dec_init) },
    skcipher_alg { base: crypto_alg { cra_flags: CRYPTO_ALG_ASYNC | CRYPTO_ALG_ALLOCATES_MEMORY, cra_blocksize: AES_BLOCK_SIZE, cra_ctxsize: core::mem::size_of::<cvm_enc_ctx>(), cra_alignmask: 7, cra_priority: 4001, cra_name: c"cbc(aes)", cra_driver_name: c"cavium-cbc-aes", cra_module: THIS_MODULE }, ivsize: AES_BLOCK_SIZE, min_keysize: AES_MIN_KEY_SIZE, max_keysize: AES_MAX_KEY_SIZE, setkey: Some(cvm_cbc_aes_setkey), encrypt: Some(cvm_encrypt), decrypt: Some(cvm_decrypt), init: Some(cvm_enc_dec_init) },
    skcipher_alg { base: crypto_alg { cra_flags: CRYPTO_ALG_ASYNC | CRYPTO_ALG_ALLOCATES_MEMORY, cra_blocksize: AES_BLOCK_SIZE, cra_ctxsize: core::mem::size_of::<cvm_enc_ctx>(), cra_alignmask: 7, cra_priority: 4001, cra_name: c"ecb(aes)", cra_driver_name: c"cavium-ecb-aes", cra_module: THIS_MODULE }, ivsize: 0, min_keysize: AES_MIN_KEY_SIZE, max_keysize: AES_MAX_KEY_SIZE, setkey: Some(cvm_ecb_aes_setkey), encrypt: Some(cvm_encrypt), decrypt: Some(cvm_decrypt), init: Some(cvm_enc_dec_init) },
    skcipher_alg { base: crypto_alg { cra_flags: CRYPTO_ALG_ASYNC | CRYPTO_ALG_ALLOCATES_MEMORY, cra_blocksize: DES3_EDE_BLOCK_SIZE, cra_ctxsize: core::mem::size_of::<cvm_des3_ctx>(), cra_alignmask: 7, cra_priority: 4001, cra_name: c"cbc(des3_ede)", cra_driver_name: c"cavium-cbc-des3_ede", cra_module: THIS_MODULE }, ivsize: DES_BLOCK_SIZE, min_keysize: DES3_EDE_KEY_SIZE, max_keysize: DES3_EDE_KEY_SIZE, setkey: Some(cvm_cbc_des3_setkey), encrypt: Some(cvm_encrypt), decrypt: Some(cvm_decrypt), init: Some(cvm_enc_dec_init) },
    skcipher_alg { base: crypto_alg { cra_flags: CRYPTO_ALG_ASYNC | CRYPTO_ALG_ALLOCATES_MEMORY, cra_blocksize: DES3_EDE_BLOCK_SIZE, cra_ctxsize: core::mem::size_of::<cvm_des3_ctx>(), cra_alignmask: 7, cra_priority: 4001, cra_name: c"ecb(des3_ede)", cra_driver_name: c"cavium-ecb-des3_ede", cra_module: THIS_MODULE }, ivsize: DES_BLOCK_SIZE, min_keysize: DES3_EDE_KEY_SIZE, max_keysize: DES3_EDE_KEY_SIZE, setkey: Some(cvm_ecb_des3_setkey), encrypt: Some(cvm_encrypt), decrypt: Some(cvm_decrypt), init: Some(cvm_enc_dec_init) },
];

unsafe fn cav_register_algs() -> i32 { crypto_register_skciphers(ALGS.as_mut_ptr(), ALGS.len()) }
unsafe fn cav_unregister_algs() { crypto_unregister_skciphers(ALGS.as_mut_ptr(), ALGS.len()); }

unsafe fn cvm_crypto_init(cptvf: *mut cpt_vf) -> i32 {
    let pdev = (*cptvf).pdev;
    let dev_count = DEV_HANDLE.dev_count;
    DEV_HANDLE.cdev[dev_count as usize] = cptvf as *mut core::ffi::c_void;
    DEV_HANDLE.dev_count += 1;
    if dev_count == 3 && cav_register_algs() != 0 {
        dev_err(&(*pdev).dev, "Error in registering crypto algorithms\n");
        return -EINVAL;
    }
    0
}

unsafe fn cvm_crypto_exit() {
    DEV_HANDLE.dev_count -= 1;
    if DEV_HANDLE.dev_count == 0 { cav_unregister_algs(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
