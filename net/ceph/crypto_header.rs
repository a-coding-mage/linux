/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Dependencies supplied by the surrounding kernel/Ceph translation.
 * The original header includes crypto/sha2.h, linux/ceph/types.h, and
 * linux/ceph/buffer.h.
 */

pub const CEPH_MAX_KEY_LEN: usize = 32;
pub const CEPH_MAX_CON_SECRET_LEN: usize = 64;

/*
 * cryptographic secret
 */
#[repr(C)]
pub struct ceph_crypto_key {
    pub type_: ::kernel::ffi::c_int,
    pub created: ceph_timespec,
    pub len: ::kernel::ffi::c_int,
    pub key: *mut ::kernel::ffi::c_void,
    pub crypto: ceph_crypto_key_crypto,
}

#[repr(C)]
pub union ceph_crypto_key_crypto {
    pub aes_tfm: *mut crypto_sync_skcipher,
    pub hmac: ceph_crypto_key_hmac,
}

#[repr(C)]
pub struct ceph_crypto_key_hmac {
    pub hmac_key: hmac_sha256_key,
    pub krb5_type: *const krb5_enctype,
    pub krb5_tfms: [*mut crypto_aead; 3],
}

extern "C" {
    pub fn ceph_crypto_key_prepare(
        key: *mut ceph_crypto_key,
        key_usages: *const u32,
        key_usage_cnt: ::kernel::ffi::c_int,
    ) -> ::kernel::ffi::c_int;
    pub fn ceph_crypto_key_clone(
        dst: *mut ceph_crypto_key,
        src: *const ceph_crypto_key,
    ) -> ::kernel::ffi::c_int;
    pub fn ceph_crypto_key_decode(
        key: *mut ceph_crypto_key,
        p: *mut *mut ::kernel::ffi::c_void,
        end: *mut ::kernel::ffi::c_void,
    ) -> ::kernel::ffi::c_int;
    pub fn ceph_crypto_key_unarmor(
        key: *mut ceph_crypto_key,
        input: *const ::kernel::ffi::c_char,
    ) -> ::kernel::ffi::c_int;
    pub fn ceph_crypto_key_destroy(key: *mut ceph_crypto_key);

    /* crypto.c */
    pub fn ceph_crypt(
        key: *const ceph_crypto_key,
        usage_slot: ::kernel::ffi::c_int,
        encrypt: bool,
        buf: *mut ::kernel::ffi::c_void,
        buf_len: ::kernel::ffi::c_int,
        in_len: ::kernel::ffi::c_int,
        pout_len: *mut ::kernel::ffi::c_int,
    ) -> ::kernel::ffi::c_int;
    pub fn ceph_crypt_data_offset(key: *const ceph_crypto_key) -> ::kernel::ffi::c_int;
    pub fn ceph_crypt_buflen(
        key: *const ceph_crypto_key,
        data_len: ::kernel::ffi::c_int,
    ) -> ::kernel::ffi::c_int;
    pub fn ceph_hmac_sha256(
        key: *const ceph_crypto_key,
        buf: *const ::kernel::ffi::c_void,
        buf_len: ::kernel::ffi::c_int,
        hmac: *mut u8,
    );
    pub fn ceph_crypto_init() -> ::kernel::ffi::c_int;
    pub fn ceph_crypto_shutdown();

    /* armor.c */
    pub fn ceph_armor(
        dst: *mut ::kernel::ffi::c_char,
        src: *const ::kernel::ffi::c_char,
        end: *const ::kernel::ffi::c_char,
    ) -> ::kernel::ffi::c_int;
    pub fn ceph_unarmor(
        dst: *mut ::kernel::ffi::c_char,
        src: *const ::kernel::ffi::c_char,
        end: *const ::kernel::ffi::c_char,
    ) -> ::kernel::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
