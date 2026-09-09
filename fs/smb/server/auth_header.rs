/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// Dependency declarations from ntlmssp.h are supplied by other translation units.

use core::ffi::c_char;

#[cfg(feature = "CONFIG_SMB_SERVER_KERBEROS5")]
pub const AUTH_GSS_LENGTH: u32 = 96;
#[cfg(feature = "CONFIG_SMB_SERVER_KERBEROS5")]
pub const AUTH_GSS_PADDING: u32 = 0;
#[cfg(not(feature = "CONFIG_SMB_SERVER_KERBEROS5"))]
pub const AUTH_GSS_LENGTH: u32 = 74;
#[cfg(not(feature = "CONFIG_SMB_SERVER_KERBEROS5"))]
pub const AUTH_GSS_PADDING: u32 = 6;

pub const CIFS_HMAC_MD5_HASH_SIZE: u32 = 16;
pub const CIFS_NTHASH_SIZE: u32 = 16;

/*
 * Size of the ntlm client response
 */
pub const CIFS_AUTH_RESP_SIZE: u32 = 24;
pub const CIFS_SMB1_SIGNATURE_SIZE: u32 = 8;
pub const CIFS_SMB1_SESSKEY_SIZE: u32 = 16;

pub const KSMBD_AUTH_NTLMSSP: u32 = 0x0001;
pub const KSMBD_AUTH_KRB5: u32 = 0x0002;
pub const KSMBD_AUTH_MSKRB5: u32 = 0x0004;
pub const KSMBD_AUTH_KRB5U2U: u32 = 0x0008;

#[repr(C)]
pub struct ksmbd_session {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ksmbd_conn {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ksmbd_work {
    _private: [u8; 0],
}
#[repr(C)]
pub struct kvec {
    _private: [u8; 0],
}

// Types declared by ntlmssp.h.
#[repr(C)]
pub struct ntlmv2_resp {
    _private: [u8; 0],
}
#[repr(C)]
pub struct authenticate_message {
    _private: [u8; 0],
}
#[repr(C)]
pub struct negotiate_message {
    _private: [u8; 0],
}
#[repr(C)]
pub struct challenge_message {
    _private: [u8; 0],
}

extern "C" {
    pub fn ksmbd_crypt_message(
        work: *mut ksmbd_work,
        iov: *mut kvec,
        nvec: u32,
        enc: i32,
    ) -> i32;
    pub fn ksmbd_crypt_rdma(
        conn: *mut ksmbd_conn,
        key: *const u8,
        buf: *mut core::ffi::c_void,
        buflen: u32,
        nonce: *const u8,
        nonce_len: u32,
        tag: *mut u8,
        tag_len: u32,
        enc: bool,
    ) -> i32;
    pub fn ksmbd_copy_gss_neg_header(buf: *mut core::ffi::c_void);
    pub fn ksmbd_auth_ntlmv2(
        conn: *mut ksmbd_conn,
        sess: *mut ksmbd_session,
        ntlmv2: *mut ntlmv2_resp,
        blen: i32,
        domain_name: *mut c_char,
        cryptkey: *mut c_char,
        sess_key: *mut c_char,
    ) -> i32;
    pub fn ksmbd_decode_ntlmssp_auth_blob(
        authblob: *mut authenticate_message,
        blob_len: i32,
        conn: *mut ksmbd_conn,
        sess: *mut ksmbd_session,
        sess_key: *mut c_char,
    ) -> i32;
    pub fn ksmbd_decode_ntlmssp_neg_blob(
        negblob: *mut negotiate_message,
        blob_len: i32,
        conn: *mut ksmbd_conn,
    ) -> i32;
    pub fn ksmbd_build_ntlmssp_challenge_blob(
        chgblob: *mut challenge_message,
        conn: *mut ksmbd_conn,
    ) -> u32;
    pub fn ksmbd_krb5_authenticate(
        sess: *mut ksmbd_session,
        in_blob: *mut c_char,
        in_len: i32,
        out_blob: *mut c_char,
        out_len: *mut i32,
        sess_key: *mut c_char,
    ) -> i32;
    pub fn ksmbd_sign_smb2_pdu(
        conn: *mut ksmbd_conn,
        key: *mut c_char,
        iov: *mut kvec,
        n_vec: i32,
        sig: *mut c_char,
    );
    pub fn ksmbd_sign_smb3_pdu(
        conn: *mut ksmbd_conn,
        key: *mut c_char,
        iov: *mut kvec,
        n_vec: i32,
        sig: *mut c_char,
    );
    pub fn ksmbd_gen_smb30_signingkey(
        sess: *mut ksmbd_session,
        conn: *mut ksmbd_conn,
    ) -> i32;
    pub fn ksmbd_gen_smb311_signingkey(
        sess: *mut ksmbd_session,
        conn: *mut ksmbd_conn,
    ) -> i32;
    pub fn ksmbd_gen_smb30_encryptionkey(
        conn: *mut ksmbd_conn,
        sess: *mut ksmbd_session,
    );
    pub fn ksmbd_gen_smb311_encryptionkey(
        conn: *mut ksmbd_conn,
        sess: *mut ksmbd_session,
    );
    pub fn ksmbd_gen_preauth_integrity_hash(
        conn: *mut ksmbd_conn,
        buf: *mut c_char,
        pi_hash: *mut u8,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
