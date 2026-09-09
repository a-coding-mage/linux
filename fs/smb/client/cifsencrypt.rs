// SPDX-License-Identifier: LGPL-2.1
// Encryption and hashing operations relating to NTLM, NTLMv2.
// C kernel headers and symbols are supplied by the surrounding translation.

unsafe fn cifs_sig_step(
    _iter_base: *mut core::ffi::c_void, _progress: usize, len: usize,
    priv_: *mut core::ffi::c_void, _priv2: *mut core::ffi::c_void,
) -> usize {
    let ctx = &mut *(priv_ as *mut cifs_calc_sig_ctx);
    if !ctx.md5.is_null() { md5_update(ctx.md5, _iter_base, len); }
    else if !ctx.hmac.is_null() { hmac_sha256_update(ctx.hmac, _iter_base, len); }
    else { aes_cmac_update(ctx.cmac, _iter_base, len); }
    0
}

unsafe fn cifs_sig_final(ctx: *mut cifs_calc_sig_ctx, out: *mut u8) {
    if !(*ctx).md5.is_null() { md5_final((*ctx).md5, out); }
    else if !(*ctx).hmac.is_null() { hmac_sha256_final((*ctx).hmac, out); }
    else { aes_cmac_final((*ctx).cmac, out); }
}

unsafe fn cifs_sig_iter(iter: *const iov_iter, maxsize: usize,
                        ctx: *mut cifs_calc_sig_ctx) -> i32 {
    let mut tmp_iter = *iter;
    let did = iterate_and_advance_kernel(&mut tmp_iter, maxsize, ctx.cast(), core::ptr::null_mut(), cifs_sig_step);
    if did != maxsize { return smb_EIO2(smb_eio_trace_sig_iter, did, maxsize); }
    0
}

pub unsafe fn __cifs_calc_signature(rqst: *mut smb_rqst, _server: *mut TCP_Server_Info,
                                    signature: *mut i8, ctx: *mut cifs_calc_sig_ctx) -> i32 {
    let mut iter: iov_iter = core::mem::zeroed();
    let mut size = 0usize;
    for i in 0..(*rqst).rq_nvec { size += (*rqst).rq_iov.add(i as usize).read().iov_len; }
    iov_iter_kvec(&mut iter, ITER_SOURCE, (*rqst).rq_iov, (*rqst).rq_nvec, size);
    if iov_iter_count(&iter) <= 4 { return smb_EIO2(smb_eio_trace_sig_data_too_small, iov_iter_count(&iter), 4); }
    let mut rc = cifs_sig_iter(&iter, iov_iter_count(&iter), ctx);
    if rc < 0 { return rc; }
    rc = cifs_sig_iter(&(*rqst).rq_iter, iov_iter_count(&(*rqst).rq_iter), ctx);
    if rc < 0 { return rc; }
    cifs_sig_final(ctx, signature.cast());
    0
}

unsafe fn build_avpair_blob(ses: *mut cifs_ses, nls_cp: *const nls_table) -> i32 {
    let mut size = 2 * core::mem::size_of::<ntlmssp2_name>();
    let defdmname = b"WORKGROUP\0";
    if (*ses).domainName.is_null() {
        (*ses).domainName = kstrdup(defdmname.as_ptr().cast(), GFP_KERNEL);
        if (*ses).domainName.is_null() { return -ENOMEM; }
    }
    let dlen = strlen((*ses).domainName);
    kfree_sensitive((*ses).auth_key.response.cast());
    (*ses).auth_key.len = size + 2 * dlen;
    (*ses).auth_key.response = kzalloc((*ses).auth_key.len, GFP_KERNEL);
    if (*ses).auth_key.response.is_null() { (*ses).auth_key.len = 0; return -ENOMEM; }
    let attr = (*ses).auth_key.response as *mut ntlmssp2_name;
    (*attr).type_ = cpu_to_le16(NTLMSSP_AV_NB_DOMAIN_NAME);
    (*attr).length = cpu_to_le16((2 * dlen) as u16);
    cifs_strtoUTF16((*attr).data.as_mut_ptr().cast(), (*ses).domainName, dlen, nls_cp);
    0
}

unsafe fn find_next_av(ses: *mut cifs_ses, mut av: *mut ntlmssp2_name) -> *mut ntlmssp2_name {
    let end = (*ses).auth_key.response.add((*ses).auth_key.len);
    if av.is_null() {
        if (*ses).auth_key.response.is_null() || (*ses).auth_key.len == 0 { return core::ptr::null_mut(); }
        av = (*ses).auth_key.response.cast();
    } else { av = (av as *mut u8).add(core::mem::size_of::<ntlmssp2_name>() + le16_to_cpu((*av).length) as usize).cast(); }
    if (av as *mut u8).add(core::mem::size_of::<ntlmssp2_name>()) > end { return core::ptr::null_mut(); }
    let len = le16_to_cpu((*av).length) as usize;
    if le16_to_cpu((*av).type_) == NTLMSSP_AV_EOL || (av as *mut u8).add(core::mem::size_of::<ntlmssp2_name>() + len) > end { return core::ptr::null_mut(); }
    av
}

unsafe fn find_av_name(ses: *mut cifs_ses, type_: u16, name: *mut *mut i8, maxlen: u16) -> i32 {
    if !(*name).is_null() { return 0; }
    let mut av = core::ptr::null_mut();
    loop {
        av = find_next_av(ses, av); if av.is_null() { break; }
        let len = le16_to_cpu((*av).length);
        if le16_to_cpu((*av).type_) != type_ || len == 0 || len % core::mem::size_of::<u16>() as u16 != 0 { continue; }
        let nlen = len / 2; if nlen <= maxlen { *name = kmalloc((nlen + 1) as usize, GFP_KERNEL).cast(); if (*name).is_null() { return -ENOMEM; } cifs_from_utf16(*name, (*av).data.as_ptr().cast(), (nlen + 1) as usize, len, (*ses).local_nls, NO_MAP_UNI_RSVD); break; }
    } 0
}

unsafe fn find_timestamp(ses: *mut cifs_ses) -> le64 {
    let mut av = core::ptr::null_mut();
    loop { av = find_next_av(ses, av); if av.is_null() { break; } if le16_to_cpu((*av).type_) == NTLMSSP_AV_TIMESTAMP && le16_to_cpu((*av).length) as usize == core::mem::size_of::<u64>() { return *( (*av).data.as_ptr() as *const le64); } }
    let mut ts: timespec64 = core::mem::zeroed(); ktime_get_real_ts64(&mut ts); cpu_to_le64(cifs_UnixTimeToNT(&ts))
}

unsafe fn calc_ntlmv2_hash(ses: *mut cifs_ses, hash: *mut i8, nls: *const nls_table) -> i32 {
    let mut nt_hash = [0i8; CIFS_NTHASH_SIZE]; let mut h: hmac_md5_ctx = core::mem::zeroed();
    E_md4hash((*ses).password, nt_hash.as_mut_ptr(), nls); hmac_md5_init_usingrawkey(&mut h, nt_hash.as_ptr().cast(), CIFS_NTHASH_SIZE); memzero_explicit(nt_hash.as_mut_ptr().cast(), nt_hash.len());
    let len = if (*ses).user_name.is_null() { 0 } else { strlen((*ses).user_name) }; let user = kmalloc(2 + len * 2, GFP_KERNEL).cast::<u16>(); if user.is_null() { memzero_explicit(&mut h as *mut _ as *mut _, core::mem::size_of::<hmac_md5_ctx>()); return -ENOMEM; }
    let ulen = if len != 0 { let n = cifs_strtoUTF16(user.cast(), (*ses).user_name, len, nls); UniStrupr(user.cast()); n } else { *user = 0; 0 }; hmac_md5_update(&mut h, user.cast(), 2 * ulen); kfree(user.cast());
    let src = if !(*ses).domainName.is_null() { (*ses).domainName } else { (*ses).ip_addr }; let n = strlen(src); let s = kmalloc(2 + 2*n, GFP_KERNEL).cast::<u16>(); if s.is_null() { memzero_explicit(&mut h as *mut _ as *mut _, core::mem::size_of::<hmac_md5_ctx>()); return -ENOMEM; }
    let n = cifs_strtoUTF16(s.cast(), src, n, nls); hmac_md5_update(&mut h, s.cast(), 2*n); kfree(s.cast()); hmac_md5_final(&mut h, hash.cast()); 0
}

unsafe fn calc_ntlmv2_response(ses: *mut cifs_ses, hash: *mut i8) { let n = ((*ses).auth_key.response.add(CIFS_SESS_KEY_SIZE) as *mut ntlmv2_resp); let l = (*ses).auth_key.len - CIFS_SESS_KEY_SIZE - core::mem::offset_of!(ntlmv2_resp, challenge); let key = if (*ses).server.negflavor == CIFS_NEGFLAVOR_EXTENDED { (*ses).ntlmssp.cryptkey.as_ptr() } else { (*ses).server.cryptkey.as_ptr() }; core::ptr::copy_nonoverlapping(key, (*n).challenge.key.as_mut_ptr(), CIFS_SERVER_CHALLENGE_SIZE); hmac_md5_usingrawkey(hash.cast(), CIFS_HMAC_MD5_HASH_SIZE, (*n).challenge.key.as_ptr(), l, (*n).ntlmv2_hash.as_ptr()); }

pub unsafe fn setup_ntlmv2_rsp(ses: *mut cifs_ses, nls: *const nls_table) -> i32 {
    if nls.is_null() { return -EINVAL; }
    if (*ses).server.negflavor != CIFS_NEGFLAVOR_EXTENDED { let rc = build_avpair_blob(ses, nls); if rc != 0 { return rc; } }
    let mut hash = [0i8; 16]; let n = (*ses).auth_key.response.add(CIFS_SESS_KEY_SIZE) as *mut ntlmv2_resp; (*n).blob_signature = cpu_to_le32(0x101); (*n).reserved = 0; (*n).time = find_timestamp(ses); get_random_bytes(&mut (*n).client_chal as *mut _ as *mut _, core::mem::size_of::<u64>()); (*n).reserved2 = 0;
    let rc = calc_ntlmv2_hash(ses, hash.as_mut_ptr(), nls); if rc == 0 { calc_ntlmv2_response(ses, hash.as_mut_ptr()); hmac_md5_usingrawkey(hash.as_ptr().cast(), CIFS_HMAC_MD5_HASH_SIZE, (*n).ntlmv2_hash.as_ptr(), CIFS_HMAC_MD5_HASH_SIZE, (*ses).auth_key.response); } memzero_explicit(hash.as_mut_ptr().cast(), hash.len()); kfree_sensitive((*ses).auth_key.response.cast()); rc
}

pub unsafe fn calc_seckey(ses: *mut cifs_ses) -> i32 { if fips_enabled { return -ENODEV; } let mut key=[0u8; CIFS_SESS_KEY_SIZE]; get_random_bytes(key.as_mut_ptr().cast(), key.len()); let c=kmalloc(core::mem::size_of::<arc4_ctx>(), GFP_KERNEL).cast::<arc4_ctx>(); if c.is_null(){return -ENOMEM;} arc4_setkey(c, (*ses).auth_key.response, CIFS_SESS_KEY_SIZE); arc4_crypt(c, (*ses).ntlmssp.ciphertext.as_mut_ptr(), key.as_mut_ptr(), CIFS_CPHTXT_SIZE); core::ptr::copy_nonoverlapping(key.as_ptr(), (*ses).auth_key.response, key.len()); (*ses).auth_key.len=CIFS_SESS_KEY_SIZE; memzero_explicit(key.as_mut_ptr().cast(),key.len()); kfree_sensitive(c.cast()); 0 }
pub unsafe fn cifs_crypto_secmech_release(server: *mut TCP_Server_Info) { if !(*server).secmech.enc.is_null() { crypto_free_aead((*server).secmech.enc); (*server).secmech.enc = core::ptr::null_mut(); } if !(*server).secmech.dec.is_null() { crypto_free_aead((*server).secmech.dec); (*server).secmech.dec = core::ptr::null_mut(); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
