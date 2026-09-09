// SPDX-License-Identifier: GPL-2.0-or-later
/* AMCC SoC PPC4xx Crypto Driver; Linux crypto algorithms. */

unsafe fn set_dynamic_sa_command_0(sa: *mut dynamic_sa_ctl, save_h: u32, save_iv: u32,
    ld_h: u32, ld_iv: u32, hdr_proc: u32, h: u32, c: u32, pad_type: u32,
    op_grp: u32, op: u32, dir: u32) {
    (*sa).sa_command_0.w = 0;
    (*sa).sa_command_0.bf.save_hash_state = save_h;
    (*sa).sa_command_0.bf.save_iv = save_iv;
    (*sa).sa_command_0.bf.load_hash_state = ld_h;
    (*sa).sa_command_0.bf.load_iv = ld_iv;
    (*sa).sa_command_0.bf.hdr_proc = hdr_proc;
    (*sa).sa_command_0.bf.hash_alg = h;
    (*sa).sa_command_0.bf.cipher_alg = c;
    (*sa).sa_command_0.bf.pad_type = pad_type & 3;
    (*sa).sa_command_0.bf.extend_pad = pad_type >> 2;
    (*sa).sa_command_0.bf.op_group = op_grp;
    (*sa).sa_command_0.bf.opcode = op;
    (*sa).sa_command_0.bf.dir = dir;
}

unsafe fn set_dynamic_sa_command_1(sa: *mut dynamic_sa_ctl, cm: u32, hmac_mc: u32,
    cfb: u32, esn: u32, sn_mask: u32, mute: u32, cp_pad: u32,
    cp_pay: u32, cp_hdr: u32) {
    (*sa).sa_command_1.w = 0;
    (*sa).sa_command_1.bf.crypto_mode31 = (cm & 4) >> 2;
    (*sa).sa_command_1.bf.crypto_mode9_8 = cm & 3;
    (*sa).sa_command_1.bf.feedback_mode = cfb;
    (*sa).sa_command_1.bf.sa_rev = 1;
    (*sa).sa_command_1.bf.hmac_muting = hmac_mc;
    (*sa).sa_command_1.bf.extended_seq_num = esn;
    (*sa).sa_command_1.bf.seq_num_mask = sn_mask;
    (*sa).sa_command_1.bf.mutable_bit_proc = mute;
    (*sa).sa_command_1.bf.copy_pad = cp_pad;
    (*sa).sa_command_1.bf.copy_payload = cp_pay;
    (*sa).sa_command_1.bf.copy_hdr = cp_hdr;
}

unsafe fn crypto4xx_crypt(req: *mut skcipher_request, ivlen: c_uint, decrypt: bool,
    check_blocksize: bool) -> c_int {
    let cipher = crypto_skcipher_reqtfm(req); let ctx = crypto_skcipher_ctx(cipher);
    let mut iv: [__le32; AES_IV_SIZE / 4] = [0; AES_IV_SIZE / 4];
    if check_blocksize && !IS_ALIGNED((*req).cryptlen, AES_BLOCK_SIZE) { return -EINVAL; }
    if ivlen != 0 { crypto4xx_memcpy_to_le32(iv.as_mut_ptr(), (*req).iv, ivlen); }
    crypto4xx_build_pd(&mut (*req).base, ctx, (*req).src, (*req).dst, (*req).cryptlen,
        iv.as_mut_ptr(), ivlen, if decrypt { (*ctx).sa_in } else { (*ctx).sa_out },
        (*ctx).sa_len, 0, core::ptr::null_mut())
}

pub unsafe fn crypto4xx_encrypt_noiv_block(req: *mut skcipher_request) -> c_int { crypto4xx_crypt(req, 0, false, true) }
pub unsafe fn crypto4xx_encrypt_iv_stream(req: *mut skcipher_request) -> c_int { crypto4xx_crypt(req, AES_IV_SIZE, false, false) }
pub unsafe fn crypto4xx_decrypt_noiv_block(req: *mut skcipher_request) -> c_int { crypto4xx_crypt(req, 0, true, true) }
pub unsafe fn crypto4xx_decrypt_iv_stream(req: *mut skcipher_request) -> c_int { crypto4xx_crypt(req, AES_IV_SIZE, true, false) }
pub unsafe fn crypto4xx_encrypt_iv_block(req: *mut skcipher_request) -> c_int { crypto4xx_crypt(req, AES_IV_SIZE, false, true) }
pub unsafe fn crypto4xx_decrypt_iv_block(req: *mut skcipher_request) -> c_int { crypto4xx_crypt(req, AES_IV_SIZE, true, true) }

/* AES Functions */
unsafe fn crypto4xx_setkey_aes(cipher: *mut crypto_skcipher, key: *const u8, keylen: c_uint, cm: u8, fb: u8) -> c_int {
    let ctx = crypto_skcipher_ctx(cipher); if keylen != AES_KEYSIZE_256 && keylen != AES_KEYSIZE_192 && keylen != AES_KEYSIZE_128 { return -EINVAL; }
    if (*ctx).sa_in != core::ptr::null_mut() || (*ctx).sa_out != core::ptr::null_mut() { crypto4xx_free_sa(ctx); }
    let rc = crypto4xx_alloc_sa(ctx, SA_AES128_LEN + (keylen - 16) / 4); if rc != 0 { return rc; }
    let mut sa = (*ctx).sa_in;
    set_dynamic_sa_command_0(sa, SA_NOT_SAVE_HASH, if cm == CRYPTO_MODE_ECB { SA_NOT_SAVE_IV } else { SA_SAVE_IV }, SA_NOT_LOAD_HASH, if cm == CRYPTO_MODE_ECB { SA_LOAD_IV_FROM_SA } else { SA_LOAD_IV_FROM_STATE }, SA_NO_HEADER_PROC, SA_HASH_ALG_NULL, SA_CIPHER_ALG_AES, SA_PAD_TYPE_ZERO, SA_OP_GROUP_BASIC, SA_OPCODE_DECRYPT, DIR_INBOUND);
    set_dynamic_sa_command_1(sa, cm as u32, SA_HASH_MODE_HASH, fb as u32, SA_EXTENDED_SN_OFF, SA_SEQ_MASK_OFF, SA_MC_ENABLE, SA_NOT_COPY_PAD, SA_NOT_COPY_PAYLOAD, SA_NOT_COPY_HDR);
    crypto4xx_memcpy_to_le32(get_dynamic_sa_key_field(sa), key, keylen); (*sa).sa_contents.w = SA_AES_CONTENTS | (keylen << 2); (*sa).sa_command_1.bf.key_len = keylen >> 3;
    memcpy((*ctx).sa_out as *mut c_void, (*ctx).sa_in as *const c_void, (*ctx).sa_len * 4); sa = (*ctx).sa_out; (*sa).sa_command_0.bf.dir = DIR_OUTBOUND; (*sa).sa_command_0.bf.opcode = SA_OPCODE_ENCRYPT; 0
}

pub unsafe fn crypto4xx_setkey_aes_cbc(c: *mut crypto_skcipher, k: *const u8, n: c_uint) -> c_int { crypto4xx_setkey_aes(c,k,n,CRYPTO_MODE_CBC,CRYPTO_FEEDBACK_MODE_NO_FB) }
pub unsafe fn crypto4xx_setkey_aes_ecb(c: *mut crypto_skcipher, k: *const u8, n: c_uint) -> c_int { crypto4xx_setkey_aes(c,k,n,CRYPTO_MODE_ECB,CRYPTO_FEEDBACK_MODE_NO_FB) }
pub unsafe fn crypto4xx_setkey_rfc3686(c: *mut crypto_skcipher, k: *const u8, n: c_uint) -> c_int { let ctx=crypto_skcipher_ctx(c); let rc=crypto4xx_setkey_aes(c,k,n-CTR_RFC3686_NONCE_SIZE,CRYPTO_MODE_CTR,CRYPTO_FEEDBACK_MODE_NO_FB); if rc!=0{return rc;} (*ctx).iv_nonce=cpu_to_le32p(k.add((n-CTR_RFC3686_NONCE_SIZE) as usize) as *const u32); 0 }

pub unsafe fn crypto4xx_rfc3686_encrypt(r: *mut skcipher_request) -> c_int { let c=crypto_skcipher_reqtfm(r); let x=crypto_skcipher_ctx(c); let iv=[(*x).iv_nonce,cpu_to_le32p((*r).iv as *const u32),cpu_to_le32p((*r).iv.add(4) as *const u32),cpu_to_le32(1)]; crypto4xx_build_pd(&mut (*r).base,x,(*r).src,(*r).dst,(*r).cryptlen,iv.as_ptr(),AES_IV_SIZE,(*x).sa_out,(*x).sa_len,0,core::ptr::null_mut()) }
pub unsafe fn crypto4xx_rfc3686_decrypt(r: *mut skcipher_request) -> c_int { crypto4xx_rfc3686_encrypt(r) }
unsafe fn crypto4xx_ctr_crypt(r:*mut skcipher_request, enc:bool)->c_int { let c=crypto_skcipher_reqtfm(r); let x=crypto_skcipher_ctx(c); let n=ALIGN((*r).cryptlen,AES_BLOCK_SIZE)/AES_BLOCK_SIZE; let ctr=be32_to_cpup((*r).iv.add(crypto_skcipher_ivsize(c)-4) as *const __be32); if ctr.wrapping_add(n)<ctr { return if enc { crypto_skcipher_encrypt(core::ptr::null_mut()) } else { crypto_skcipher_decrypt(core::ptr::null_mut()) }; } if enc {crypto4xx_encrypt_iv_stream(r)} else {crypto4xx_decrypt_iv_stream(r)} }
pub unsafe fn crypto4xx_setkey_aes_ctr(c:*mut crypto_skcipher,k:*const u8,n:c_uint)->c_int { let x=crypto_skcipher_ctx(c); let rc=crypto_sync_skcipher_setkey((*x).sw_cipher.cipher,k,n); if rc!=0{return rc;} crypto4xx_setkey_aes(c,k,n,CRYPTO_MODE_CTR,CRYPTO_FEEDBACK_MODE_NO_FB) }
pub unsafe fn crypto4xx_encrypt_ctr(r:*mut skcipher_request)->c_int{crypto4xx_ctr_crypt(r,true)}
pub unsafe fn crypto4xx_decrypt_ctr(r:*mut skcipher_request)->c_int{crypto4xx_ctr_crypt(r,false)}
unsafe fn crypto4xx_aead_need_fallback(r:*mut aead_request,len:c_uint,is_ccm:bool,_:bool)->bool{let a=crypto_aead_reqtfm(r); crypto_aead_authsize(a)&3!=0||len<AES_BLOCK_SIZE||(*r).assoclen&3!=0||(*r).assoclen>1020||(is_ccm&& !((*r).iv[0]==1||(*r).iv[0]==3))}
unsafe fn crypto4xx_aead_fallback(r:*mut aead_request,x:*mut crypto4xx_ctx,d:bool)->c_int{let s=aead_request_ctx(r); aead_request_set_tfm(s,(*x).sw_cipher.aead); aead_request_set_crypt(s,(*r).src,(*r).dst,(*r).cryptlen,(*r).iv); aead_request_set_ad(s,(*r).assoclen); if d{crypto_aead_decrypt(s)}else{crypto_aead_encrypt(s)}}
pub unsafe fn crypto4xx_setauthsize_aead(c:*mut crypto_aead,n:c_uint)->c_int{let x=crypto_tfm_ctx(crypto_aead_tfm(c));crypto_aead_setauthsize((*x).sw_cipher.aead,n)}
pub unsafe fn crypto4xx_setkey_aes_ccm(c:*mut crypto_aead,k:*const u8,n:c_uint)->c_int{let x=crypto_tfm_ctx(crypto_aead_tfm(c));let rc=crypto_aead_setkey((*x).sw_cipher.aead,k,n);if rc!=0{return rc;} if (*x).sa_in!=core::ptr::null_mut(){crypto4xx_free_sa(x);} crypto4xx_alloc_sa(x,SA_AES128_CCM_LEN+(n-16)/4)}
pub unsafe fn crypto4xx_encrypt_aes_ccm(r:*mut aead_request)->c_int{let x=crypto_tfm_ctx((*r).base.tfm);let l=(*r).cryptlen;if crypto4xx_aead_need_fallback(r,l,true,false){return crypto4xx_aead_fallback(r,x,false)} crypto4xx_build_pd(&mut (*r).base,x,(*r).src,(*r).dst,l,(*r).iv as *mut __le32,16,(*x).sa_out,(*x).sa_len,(*r).assoclen,aead_request_ctx(r))}
pub unsafe fn crypto4xx_decrypt_aes_ccm(r:*mut aead_request)->c_int{crypto4xx_encrypt_aes_ccm(r)}
pub unsafe fn crypto4xx_setkey_aes_gcm(c:*mut crypto_aead,k:*const u8,n:c_uint)->c_int{crypto4xx_setkey_aes_ccm(c,k,n)}
pub unsafe fn crypto4xx_encrypt_aes_gcm(r:*mut aead_request)->c_int{crypto4xx_encrypt_aes_ccm(r)}
pub unsafe fn crypto4xx_decrypt_aes_gcm(r:*mut aead_request)->c_int{crypto4xx_decrypt_aes_gcm(r)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
