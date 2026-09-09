// SPDX-License-Identifier: GPL-2.0
// Linux and crypto header dependencies are supplied by the surrounding translation.

#[repr(C)]
struct nitrox_cipher {
    name: *const core::ffi::c_char,
    value: flexi_cipher,
}

/* supported cipher list */
static flexi_cipher_table: [nitrox_cipher; 9] = [
    nitrox_cipher { name: c"null".as_ptr(), value: CIPHER_NULL },
    nitrox_cipher { name: c"cbc(des3_ede)".as_ptr(), value: CIPHER_3DES_CBC },
    nitrox_cipher { name: c"ecb(des3_ede)".as_ptr(), value: CIPHER_3DES_ECB },
    nitrox_cipher { name: c"cbc(aes)".as_ptr(), value: CIPHER_AES_CBC },
    nitrox_cipher { name: c"ecb(aes)".as_ptr(), value: CIPHER_AES_ECB },
    nitrox_cipher { name: c"cfb(aes)".as_ptr(), value: CIPHER_AES_CFB },
    nitrox_cipher { name: c"rfc3686(ctr(aes))".as_ptr(), value: CIPHER_AES_CTR },
    nitrox_cipher { name: c"xts(aes)".as_ptr(), value: CIPHER_AES_XTS },
    nitrox_cipher { name: c"cts(cbc(aes))".as_ptr(), value: CIPHER_AES_CBC_CTS },
];

unsafe fn flexi_cipher_type(name: *const core::ffi::c_char) -> flexi_cipher {
    let mut cipher = flexi_cipher_table.as_ptr();
    while !(*cipher).name.is_null() {
        if libc::strcmp((*cipher).name, name) == 0 { break; }
        cipher = cipher.add(1);
    }
    if cipher == flexi_cipher_table.as_ptr().add(flexi_cipher_table.len()) {
        CIPHER_INVALID
    } else { (*cipher).value }
}

unsafe fn free_src_sglist(skreq: *mut skcipher_request) {
    let nkreq = skcipher_request_ctx(skreq);
    kfree((*nkreq).src as *mut core::ffi::c_void);
}

unsafe fn free_dst_sglist(skreq: *mut skcipher_request) {
    let nkreq = skcipher_request_ctx(skreq);
    kfree((*nkreq).dst as *mut core::ffi::c_void);
}

unsafe extern "C" fn nitrox_skcipher_callback(arg: *mut core::ffi::c_void, mut err: i32) {
    let skreq = arg as *mut skcipher_request;
    free_src_sglist(skreq);
    free_dst_sglist(skreq);
    if err != 0 { pr_err_ratelimited(c"request failed status 0x%0x\n".as_ptr(), err); err = -EINVAL; }
    skcipher_request_complete(skreq, err);
}

unsafe extern "C" fn nitrox_cbc_cipher_callback(arg: *mut core::ffi::c_void, err: i32) {
    let skreq = arg as *mut skcipher_request;
    let nkreq = skcipher_request_ctx(skreq);
    let cipher = crypto_skcipher_reqtfm(skreq);
    let ivsize = crypto_skcipher_ivsize(cipher);
    let start = (*skreq).cryptlen - ivsize as usize;
    if err != 0 { nitrox_skcipher_callback(arg, err); return; }
    if (*nkreq).creq.ctrl.s.arg == ENCRYPT {
        scatterwalk_map_and_copy((*skreq).iv, (*skreq).dst, start, ivsize, 0);
    } else if (*skreq).src != (*skreq).dst {
        scatterwalk_map_and_copy((*skreq).iv, (*skreq).src, start, ivsize, 0);
    } else {
        memcpy((*skreq).iv, (*nkreq).iv_out, ivsize);
        kfree((*nkreq).iv_out as *mut core::ffi::c_void);
    }
    nitrox_skcipher_callback(arg, err);
}

unsafe fn nitrox_skcipher_init(tfm: *mut crypto_skcipher) -> i32 {
    let nctx = crypto_skcipher_ctx(tfm);
    (*nctx).ndev = nitrox_get_first_device();
    if (*nctx).ndev.is_null() { return -ENODEV; }
    let chdr = crypto_alloc_context((*nctx).ndev);
    if chdr.is_null() { nitrox_put_device((*nctx).ndev); return -ENOMEM; }
    (*nctx).callback = Some(nitrox_skcipher_callback);
    (*nctx).chdr = chdr;
    (*nctx).u.ctx_handle = ((*chdr).vaddr as *mut u8).add(core::mem::size_of::<ctx_hdr>()) as usize;
    crypto_skcipher_set_reqsize(tfm, crypto_skcipher_reqsize(tfm) + core::mem::size_of::<nitrox_kcrypt_request>());
    0
}

unsafe fn nitrox_cbc_init(tfm: *mut crypto_skcipher) -> i32 {
    let err = nitrox_skcipher_init(tfm); if err != 0 { return err; }
    (*crypto_skcipher_ctx(tfm)).callback = Some(nitrox_cbc_cipher_callback); 0
}

unsafe fn nitrox_skcipher_exit(tfm: *mut crypto_skcipher) {
    let nctx = crypto_skcipher_ctx(tfm);
    if (*nctx).u.ctx_handle != 0 {
        let fctx = (*nctx).u.fctx;
        memzero_explicit(&mut (*fctx).crypto as *mut _ as *mut core::ffi::c_void, core::mem::size_of::<crypto_keys>());
        memzero_explicit(&mut (*fctx).auth as *mut _ as *mut core::ffi::c_void, core::mem::size_of::<auth_keys>());
        crypto_free_context((*nctx).chdr as *mut core::ffi::c_void);
    }
    nitrox_put_device((*nctx).ndev); (*nctx).u.ctx_handle = 0; (*nctx).ndev = core::ptr::null_mut();
}

// The remaining algorithm callbacks and registration table retain the C implementation's
// externally supplied kernel types and helpers.
unsafe fn nitrox_skcipher_setkey(cipher: *mut crypto_skcipher, aes_keylen: i32, key: *const u8, keylen: usize) -> i32 {
    let tfm = crypto_skcipher_tfm(cipher); let nctx = crypto_tfm_ctx(tfm); let fctx = (*nctx).u.fctx;
    let name = crypto_tfm_alg_name(tfm); let cipher_type = flexi_cipher_type(name);
    if cipher_type == CIPHER_INVALID { pr_err(c"unsupported cipher: %s\n".as_ptr(), name); return -EINVAL; }
    (*fctx).flags.f = 0; (*fctx).flags.w0.cipher_type = cipher_type; (*fctx).flags.w0.aes_keylen = aes_keylen;
    (*fctx).flags.w0.iv_source = IV_FROM_DPTR; (*fctx).flags.f = cpu_to_be64((*fctx).flags.w0 as u64);
    memcpy((*fctx).crypto.u.key.as_mut_ptr() as *mut _, key as *const _, keylen); 0
}

// Direct translations of the simple wrappers and algorithm descriptors.
unsafe fn nitrox_aes_encrypt(r: *mut skcipher_request) -> i32 { nitrox_skcipher_crypt(r, true) }
unsafe fn nitrox_aes_decrypt(r: *mut skcipher_request) -> i32 { nitrox_skcipher_crypt(r, false) }
unsafe fn nitrox_3des_encrypt(r: *mut skcipher_request) -> i32 { nitrox_skcipher_crypt(r, true) }
unsafe fn nitrox_3des_decrypt(r: *mut skcipher_request) -> i32 { nitrox_skcipher_crypt(r, false) }

unsafe fn nitrox_aes_setkey(c: *mut crypto_skcipher, key: *const u8, len: usize) -> i32 {
    let n = flexi_aes_keylen(len); if n < 0 { return -EINVAL; } nitrox_skcipher_setkey(c, n, key, len)
}
unsafe fn alloc_src_sglist(r: *mut skcipher_request, iv: i32) -> i32 {
    let n = skcipher_request_ctx(r); let ents = sg_nents((*r).src) + 1; let ret = alloc_src_req_buf(n, ents, iv); if ret != 0 { return ret; }
    nitrox_creq_copy_iv((*n).src, (*r).iv, iv); nitrox_creq_set_src_sg(n, ents, iv, (*r).src, (*r).cryptlen); 0
}
unsafe fn alloc_dst_sglist(r: *mut skcipher_request, iv: i32) -> i32 {
    let n = skcipher_request_ctx(r); let ents = sg_nents((*r).dst) + 3; let ret = alloc_dst_req_buf(n, ents); if ret != 0 { return ret; }
    nitrox_creq_set_orh(n); nitrox_creq_set_comp(n); nitrox_creq_set_dst_sg(n, ents, iv, (*r).dst, (*r).cryptlen); 0
}
unsafe fn nitrox_skcipher_crypt(r: *mut skcipher_request, enc: bool) -> i32 {
    let cipher = crypto_skcipher_reqtfm(r); let ctx = crypto_skcipher_ctx(cipher); let n = skcipher_request_ctx(r);
    let iv = crypto_skcipher_ivsize(cipher); let q = &mut (*n).creq;
    q.flags = (*r).base.flags; q.gfp = if q.flags & CRYPTO_TFM_REQ_MAY_SLEEP != 0 { GFP_KERNEL } else { GFP_ATOMIC };
    q.ctrl.value = 0; q.opcode = FLEXI_CRYPTO_ENCRYPT_HMAC; q.ctrl.s.arg = if enc { ENCRYPT } else { DECRYPT };
    q.gph.param0 = cpu_to_be16((*r).cryptlen as u16); q.gph.param1 = 0; q.gph.param2 = cpu_to_be16(iv as u16); q.gph.param3 = 0;
    q.ctx_handle = (*ctx).u.ctx_handle; q.ctrl.s.ctxl = core::mem::size_of::<flexi_crypto_context>();
    let ret = alloc_src_sglist(r, iv); if ret != 0 { return ret; } let ret = alloc_dst_sglist(r, iv); if ret != 0 { free_src_sglist(r); return ret; }
    nitrox_process_se_request((*ctx).ndev, q, (*ctx).callback, r as *mut _)
}
unsafe fn nitrox_cbc_decrypt(r: *mut skcipher_request) -> i32 {
    let n = skcipher_request_ctx(r); let c = crypto_skcipher_reqtfm(r); let iv = crypto_skcipher_ivsize(c); let start = (*r).cryptlen - iv as usize;
    if (*r).src != (*r).dst { return nitrox_skcipher_crypt(r, false); }
    (*n).iv_out = kmalloc(iv, if (*r).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP != 0 { GFP_KERNEL } else { GFP_ATOMIC }); if (*n).iv_out.is_null() { return -ENOMEM; }
    scatterwalk_map_and_copy((*n).iv_out, (*r).src, start, iv, 0); nitrox_skcipher_crypt(r, false)
}
unsafe fn nitrox_3des_setkey(c: *mut crypto_skcipher, key: *const u8, len: usize) -> i32 {
    let ret = verify_skcipher_des3_key(c, key); if ret != 0 { return ret; } nitrox_skcipher_setkey(c, 0, key, len)
}
unsafe fn nitrox_aes_xts_setkey(c: *mut crypto_skcipher, key: *const u8, len: usize) -> i32 {
    let ret = xts_verify_key(c, key, len); if ret != 0 { return ret; } let half = len / 2; let n = flexi_aes_keylen(half); if n < 0 { return -EINVAL; }
    let ctx = crypto_skcipher_ctx(c); memcpy((*ctx).u.fctx.crypto.iv.as_mut_ptr() as *mut _, key.add(half) as *const _, half); nitrox_skcipher_setkey(c, n, key, half)
}
unsafe fn nitrox_aes_ctr_rfc3686_setkey(c: *mut crypto_skcipher, key: *const u8, mut len: usize) -> i32 {
    if len < CTR_RFC3686_NONCE_SIZE { return -EINVAL; } let ctx = crypto_skcipher_ctx(c); memcpy((*ctx).u.fctx.crypto.iv.as_mut_ptr() as *mut _, key.add(len - CTR_RFC3686_NONCE_SIZE) as *const _, CTR_RFC3686_NONCE_SIZE); len -= CTR_RFC3686_NONCE_SIZE;
    let n = flexi_aes_keylen(len); if n < 0 { return -EINVAL; } nitrox_skcipher_setkey(c, n, key, len)
}

/* The skcipher_alg array is represented with the kernel's externally supplied
 * descriptor type; field values and callbacks mirror the C registration table. */
#[allow(non_upper_case_globals)]
static mut nitrox_skciphers: [skcipher_alg; 7] = [
    skcipher_alg::new(c"cbc(aes)".as_ptr(), c"n5_cbc(aes)".as_ptr(), AES_BLOCK_SIZE, AES_MIN_KEY_SIZE, AES_MAX_KEY_SIZE, nitrox_aes_setkey, nitrox_aes_encrypt, nitrox_cbc_decrypt, nitrox_cbc_init, nitrox_skcipher_exit),
    skcipher_alg::new(c"ecb(aes)".as_ptr(), c"n5_ecb(aes)".as_ptr(), AES_BLOCK_SIZE, AES_MIN_KEY_SIZE, AES_MAX_KEY_SIZE, nitrox_aes_setkey, nitrox_aes_encrypt, nitrox_aes_decrypt, nitrox_skcipher_init, nitrox_skcipher_exit),
    skcipher_alg::new(c"xts(aes)".as_ptr(), c"n5_xts(aes)".as_ptr(), AES_BLOCK_SIZE, 2 * AES_MIN_KEY_SIZE, 2 * AES_MAX_KEY_SIZE, nitrox_aes_xts_setkey, nitrox_aes_encrypt, nitrox_aes_decrypt, nitrox_skcipher_init, nitrox_skcipher_exit),
    skcipher_alg::new(c"rfc3686(ctr(aes))".as_ptr(), c"n5_rfc3686(ctr(aes))".as_ptr(), 1, AES_MIN_KEY_SIZE + CTR_RFC3686_NONCE_SIZE, AES_MAX_KEY_SIZE + CTR_RFC3686_NONCE_SIZE, nitrox_aes_ctr_rfc3686_setkey, nitrox_aes_encrypt, nitrox_aes_decrypt, nitrox_skcipher_init, nitrox_skcipher_exit),
    skcipher_alg::new(c"cts(cbc(aes))".as_ptr(), c"n5_cts(cbc(aes))".as_ptr(), AES_BLOCK_SIZE, AES_MIN_KEY_SIZE, AES_MAX_KEY_SIZE, nitrox_aes_setkey, nitrox_aes_encrypt, nitrox_aes_decrypt, nitrox_skcipher_init, nitrox_skcipher_exit),
    skcipher_alg::new(c"cbc(des3_ede)".as_ptr(), c"n5_cbc(des3_ede)".as_ptr(), DES3_EDE_BLOCK_SIZE, DES3_EDE_KEY_SIZE, DES3_EDE_KEY_SIZE, nitrox_3des_setkey, nitrox_3des_encrypt, nitrox_cbc_decrypt, nitrox_cbc_init, nitrox_skcipher_exit),
    skcipher_alg::new(c"ecb(des3_ede)".as_ptr(), c"n5_ecb(des3_ede)".as_ptr(), DES3_EDE_BLOCK_SIZE, DES3_EDE_KEY_SIZE, DES3_EDE_KEY_SIZE, nitrox_3des_setkey, nitrox_3des_encrypt, nitrox_3des_decrypt, nitrox_skcipher_init, nitrox_skcipher_exit),
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
