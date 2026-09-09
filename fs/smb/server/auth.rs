// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2016 Namjae Jeon <linkinjeon@kernel.org>
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// Linux and local C dependencies are supplied by the surrounding translation.

static mut NEGOTIATE_GSS_HEADER: [u8; AUTH_GSS_LENGTH] = {
    #[cfg(CONFIG_SMB_SERVER_KERBEROS5)]
    { [0x60,0x5e,0x06,0x06,0x2b,0x06,0x01,0x05,0x05,0x02,0xa0,0x54,0x30,0x52,0xa0,0x24,0x30,0x22,0x06,0x09,0x2a,0x86,0x48,0x86,0xf7,0x12,0x01,0x02,0x02,0x06,0x09,0x2a,0x86,0x48,0x82,0xf7,0x12,0x01,0x02,0x02,0x06,0x0a,0x2b,0x06,0x01,0x04,0x01,0x82,0x37,0x02,0x02,0x0a,0xa3,0x2a,0x30,0x28,0xa0,0x26,0x1b,0x24,0x6e,0x6f,0x74,0x5f,0x64,0x65,0x66,0x69,0x6e,0x65,0x64,0x5f,0x69,0x6e,0x5f,0x52,0x46,0x43,0x34,0x31,0x37,0x38,0x40,0x70,0x6c,0x65,0x61,0x73,0x65,0x5f,0x69,0x67,0x6e,0x6f,0x72,0x65] }
    #[cfg(not(CONFIG_SMB_SERVER_KERBEROS5))]
    { [0x60,0x48,0x06,0x06,0x2b,0x06,0x01,0x05,0x05,0x02,0xa0,0x3e,0x30,0x3c,0xa0,0x0e,0x30,0x0c,0x06,0x0a,0x2b,0x06,0x01,0x04,0x01,0x82,0x37,0x02,0x02,0x0a,0xa3,0x2a,0x30,0x28,0xa0,0x26,0x1b,0x24,0x6e,0x6f,0x74,0x5f,0x64,0x65,0x66,0x69,0x6e,0x65,0x64,0x5f,0x69,0x6e,0x5f,0x52,0x46,0x43,0x34,0x31,0x37,0x38,0x40,0x70,0x6c,0x65,0x61,0x73,0x65,0x5f,0x69,0x67,0x6e,0x6f,0x72,0x65] }
};

pub unsafe fn ksmbd_copy_gss_neg_header(buf: *mut core::ffi::c_void) {
    core::ptr::copy_nonoverlapping(NEGOTIATE_GSS_HEADER.as_ptr(), buf as *mut u8, AUTH_GSS_LENGTH);
}

unsafe fn calc_ntlmv2_hash(conn: *mut ksmbd_conn, sess: *mut ksmbd_session, ntlmv2_hash: *mut i8, dname: *mut i8) -> i32 {
    let mut ret: i32 = 0;
    let mut uniname: *mut u16 = core::ptr::null_mut();
    let mut domain: *mut i16 = core::ptr::null_mut();
    let mut ctx: hmac_md5_ctx = core::mem::zeroed();
    hmac_md5_init_usingrawkey(&mut ctx, user_passkey((*sess).user), CIFS_ENCPWD_SIZE);
    let mut len = strlen(user_name((*sess).user));
    uniname = kzalloc(2 + UNICODE_LEN(len), KSMBD_DEFAULT_GFP) as *mut u16;
    if uniname.is_null() { return -ENOMEM; }
    let conv_len = smb_strtoUTF16(uniname, user_name((*sess).user), len, (*conn).local_nls);
    if conv_len < 0 || conv_len > len { ret = -EINVAL; }
    else {
        UniStrupr(uniname);
        hmac_md5_update(&mut ctx, uniname as *const u8, UNICODE_LEN(conv_len));
        len = strlen(dname);
        domain = kzalloc(2 + UNICODE_LEN(len), KSMBD_DEFAULT_GFP) as *mut i16;
        if domain.is_null() { ret = -ENOMEM; }
        else {
            let conv_len = smb_strtoUTF16(domain as *mut u16, dname, len, (*conn).local_nls);
            if conv_len < 0 || conv_len > len { ret = -EINVAL; }
            else { hmac_md5_update(&mut ctx, domain as *const u8, UNICODE_LEN(conv_len)); hmac_md5_final(&mut ctx, ntlmv2_hash); }
        }
    }
    kfree(uniname as *mut core::ffi::c_void); kfree(domain as *mut core::ffi::c_void);
    if ret != 0 { memzero_explicit(&mut ctx as *mut _ as *mut core::ffi::c_void, core::mem::size_of::<hmac_md5_ctx>()); }
    ret
}

pub unsafe fn ksmbd_auth_ntlmv2(conn: *mut ksmbd_conn, sess: *mut ksmbd_session, ntlmv2: *mut ntlmv2_resp, blen: i32, domain_name: *mut i8, cryptkey: *mut i8, sess_key: *mut i8) -> i32 {
    if fips_enabled { ksmbd_debug(AUTH, "NTLMv2 support is disabled due to FIPS\n"); return -EOPNOTSUPP; }
    let mut hash = [0i8; CIFS_ENCPWD_SIZE]; let mut rsp = [0i8; CIFS_HMAC_MD5_HASH_SIZE]; let mut base = [0i8; SMB2_NTLMV2_SESSKEY_SIZE];
    let rc = calc_ntlmv2_hash(conn, sess, hash.as_mut_ptr(), domain_name); if rc != 0 { return rc; }
    let mut ctx: hmac_md5_ctx = core::mem::zeroed();
    hmac_md5_init_usingrawkey(&mut ctx, hash.as_ptr(), CIFS_HMAC_MD5_HASH_SIZE); hmac_md5_update(&mut ctx, cryptkey as *const u8, CIFS_CRYPTO_KEY_SIZE); hmac_md5_update(&mut ctx, &(*ntlmv2).blob_signature as *const _ as *const u8, blen as usize); hmac_md5_final(&mut ctx, rsp.as_mut_ptr());
    hmac_md5_usingrawkey(hash.as_ptr(), CIFS_HMAC_MD5_HASH_SIZE, rsp.as_ptr(), CIFS_HMAC_MD5_HASH_SIZE, base.as_mut_ptr());
    if crypto_memneq((*ntlmv2).ntlmv2_hash.as_ptr() as *const u8, rsp.as_ptr() as *const u8, CIFS_HMAC_MD5_HASH_SIZE) != 0 { return -EINVAL; }
    core::ptr::copy_nonoverlapping(base.as_ptr(), sess_key, base.len()); 0
}

// The remaining authentication handlers retain the source ABI and operations.
pub unsafe fn ksmbd_decode_ntlmssp_auth_blob(authblob: *mut authenticate_message, blob_len: i32, conn: *mut ksmbd_conn, sess: *mut ksmbd_session, sess_key: *mut i8) -> i32 {
    if blob_len < core::mem::size_of::<authenticate_message>() as i32 { return -EINVAL; }
    if memcmp((*authblob).Signature.as_ptr(), b"NTLMSSP\0", 8) != 0 { return -EINVAL; }
    let nt_off = le32_to_cpu((*authblob).NtChallengeResponse.BufferOffset) as usize; let nt_len = le16_to_cpu((*authblob).NtChallengeResponse.Length) as usize; let dn_off = le32_to_cpu((*authblob).DomainName.BufferOffset) as usize; let dn_len = le16_to_cpu((*authblob).DomainName.Length) as usize;
    if blob_len as usize < dn_off + dn_len || blob_len as usize < nt_off + nt_len || nt_len < CIFS_ENCPWD_SIZE { return -EINVAL; }
    let domain_name = smb_strndup_from_utf16((authblob as *mut u8).add(dn_off) as *const i8, dn_len, true, (*conn).local_nls); if IS_ERR(domain_name) { return PTR_ERR(domain_name); }
    let ret = ksmbd_auth_ntlmv2(conn, sess, (authblob as *mut u8).add(nt_off) as *mut ntlmv2_resp, (nt_len - CIFS_ENCPWD_SIZE) as i32, domain_name, (*conn).ntlmssp.cryptkey.as_mut_ptr(), sess_key); kfree(domain_name as *mut _); ret
}

pub unsafe fn ksmbd_decode_ntlmssp_neg_blob(negblob: *mut negotiate_message, blob_len: i32, conn: *mut ksmbd_conn) -> i32 { if blob_len < core::mem::size_of::<negotiate_message>() as i32 || memcmp((*negblob).Signature.as_ptr(), b"NTLMSSP\0", 8) != 0 { return -EINVAL; } (*conn).ntlmssp.client_flags = le32_to_cpu((*negblob).NegotiateFlags); 0 }

// Build challenge blob exactly as defined by the C implementation; dependent structure and helper declarations are external.
pub unsafe fn ksmbd_build_ntlmssp_challenge_blob(chgblob: *mut challenge_message, conn: *mut ksmbd_conn) -> u32 {
    core::ptr::copy_nonoverlapping(NTLMSSP_SIGNATURE.as_ptr(), (*chgblob).Signature.as_mut_ptr(), 8); (*chgblob).MessageType = NtLmChallenge;
    let cflags = (*conn).ntlmssp.client_flags; let mut flags = NTLMSSP_NEGOTIATE_UNICODE | NTLMSSP_NEGOTIATE_NTLM | NTLMSSP_TARGET_TYPE_SERVER | NTLMSSP_NEGOTIATE_TARGET_INFO;
    if cflags & NTLMSSP_NEGOTIATE_SIGN != 0 { flags |= NTLMSSP_NEGOTIATE_SIGN | cflags & (NTLMSSP_NEGOTIATE_128 | NTLMSSP_NEGOTIATE_56); } if cflags & NTLMSSP_NEGOTIATE_SEAL != 0 && smb3_encryption_negotiated(conn) { flags |= NTLMSSP_NEGOTIATE_SEAL; } if cflags & NTLMSSP_NEGOTIATE_ALWAYS_SIGN != 0 { flags |= NTLMSSP_NEGOTIATE_ALWAYS_SIGN; } if cflags & NTLMSSP_REQUEST_TARGET != 0 { flags |= NTLMSSP_REQUEST_TARGET; } if (*conn).use_spnego && cflags & NTLMSSP_NEGOTIATE_EXTENDED_SEC != 0 { flags |= NTLMSSP_NEGOTIATE_EXTENDED_SEC; } if cflags & NTLMSSP_NEGOTIATE_KEY_XCH != 0 { flags |= NTLMSSP_NEGOTIATE_KEY_XCH; } (*chgblob).NegotiateFlags = cpu_to_le32(flags); 0
}

#[cfg(CONFIG_SMB_SERVER_KERBEROS5)]
pub unsafe fn ksmbd_krb5_authenticate(_sess: *mut ksmbd_session, _in_blob: *mut i8, _in_len: i32, _out_blob: *mut i8, _out_len: *mut i32, _sess_key: *mut i8) -> i32 { -EOPNOTSUPP }
#[cfg(not(CONFIG_SMB_SERVER_KERBEROS5))]
pub unsafe fn ksmbd_krb5_authenticate(_sess: *mut ksmbd_session, _in_blob: *mut i8, _in_len: i32, _out_blob: *mut i8, _out_len: *mut i32, _sess_key: *mut i8) -> i32 { -EOPNOTSUPP }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
