/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Kerberos 5 crypto
 *
 * Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Dependencies supplied by the surrounding kernel translation. */

pub const KRB5_ENCTYPE_NULL: i32 = 0x0000;
pub const KRB5_ENCTYPE_DES_CBC_CRC: i32 = 0x0001; /* DES cbc mode with CRC-32 */
pub const KRB5_ENCTYPE_DES_CBC_MD4: i32 = 0x0002; /* DES cbc mode with RSA-MD4 */
pub const KRB5_ENCTYPE_DES_CBC_MD5: i32 = 0x0003; /* DES cbc mode with RSA-MD5 */
pub const KRB5_ENCTYPE_DES_CBC_RAW: i32 = 0x0004; /* DES cbc mode raw */
/* XXX deprecated? */
pub const KRB5_ENCTYPE_DES3_CBC_SHA: i32 = 0x0005; /* DES-3 cbc mode with NIST-SHA */
pub const KRB5_ENCTYPE_DES3_CBC_RAW: i32 = 0x0006; /* DES-3 cbc mode raw */
pub const KRB5_ENCTYPE_DES_HMAC_SHA1: i32 = 0x0008;
pub const KRB5_ENCTYPE_DES3_CBC_SHA1: i32 = 0x0010;
pub const KRB5_ENCTYPE_AES128_CTS_HMAC_SHA1_96: i32 = 0x0011;
pub const KRB5_ENCTYPE_AES256_CTS_HMAC_SHA1_96: i32 = 0x0012;
pub const KRB5_ENCTYPE_AES128_CTS_HMAC_SHA256_128: i32 = 0x0013;
pub const KRB5_ENCTYPE_AES256_CTS_HMAC_SHA384_192: i32 = 0x0014;
pub const KRB5_ENCTYPE_ARCFOUR_HMAC: i32 = 0x0017;
pub const KRB5_ENCTYPE_ARCFOUR_HMAC_EXP: i32 = 0x0018;
pub const KRB5_ENCTYPE_CAMELLIA128_CTS_CMAC: i32 = 0x0019;
pub const KRB5_ENCTYPE_CAMELLIA256_CTS_CMAC: i32 = 0x001a;
pub const KRB5_ENCTYPE_UNKNOWN: i32 = 0x01ff;

pub const KRB5_CKSUMTYPE_CRC32: i32 = 0x0001;
pub const KRB5_CKSUMTYPE_RSA_MD4: i32 = 0x0002;
pub const KRB5_CKSUMTYPE_RSA_MD4_DES: i32 = 0x0003;
pub const KRB5_CKSUMTYPE_DESCBC: i32 = 0x0004;
pub const KRB5_CKSUMTYPE_RSA_MD5: i32 = 0x0007;
pub const KRB5_CKSUMTYPE_RSA_MD5_DES: i32 = 0x0008;
pub const KRB5_CKSUMTYPE_NIST_SHA: i32 = 0x0009;
pub const KRB5_CKSUMTYPE_HMAC_SHA1_DES3: i32 = 0x000c;
pub const KRB5_CKSUMTYPE_HMAC_SHA1_96_AES128: i32 = 0x000f;
pub const KRB5_CKSUMTYPE_HMAC_SHA1_96_AES256: i32 = 0x0010;
pub const KRB5_CKSUMTYPE_CMAC_CAMELLIA128: i32 = 0x0011;
pub const KRB5_CKSUMTYPE_CMAC_CAMELLIA256: i32 = 0x0012;
pub const KRB5_CKSUMTYPE_HMAC_SHA256_128_AES128: i32 = 0x0013;
pub const KRB5_CKSUMTYPE_HMAC_SHA384_192_AES256: i32 = 0x0014;
pub const KRB5_CKSUMTYPE_HMAC_MD5_ARCFOUR: i32 = -138; /* Microsoft md5 hmac cksumtype */

/* Constants used for key derivation, from RFC 3961. */
pub const KEY_USAGE_SEED_CHECKSUM: u8 = 0x99;
pub const KEY_USAGE_SEED_ENCRYPTION: u8 = 0xAA;
pub const KEY_USAGE_SEED_INTEGRITY: u8 = 0x55;

/* Standard Kerberos error codes. */
pub const KRB5_PROG_KEYTYPE_NOSUPP: i32 = -1765328233;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum krb5_crypto_mode {
    KRB5_CHECKSUM_MODE,
    KRB5_ENCRYPT_MODE,
}

#[repr(C)]
pub struct krb5_buffer {
    pub len: core::ffi::c_uint,
    pub data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct krb5_enctype {
    pub etype: i32,
    pub ctype: i32,
    pub name: *const core::ffi::c_char,
    pub encrypt_name: *const core::ffi::c_char,
    pub cksum_name: *const core::ffi::c_char,
    pub hash_name: *const core::ffi::c_char,
    pub derivation_enc: *const core::ffi::c_char,
    pub block_len: u16,
    pub conf_len: u16,
    pub cksum_len: u16,
    pub key_bytes: u16,
    pub key_len: u16,
    pub hash_len: u16,
    pub prf_len: u16,
    pub Kc_len: u16,
    pub Ke_len: u16,
    pub Ki_len: u16,
    pub keyed_cksum: bool,
    pub profile: *const krb5_crypto_profile,
    pub random_to_key: Option<unsafe extern "C" fn(
        krb5: *const krb5_enctype,
        input: *const krb5_buffer,
        output: *mut krb5_buffer,
    ) -> i32>,
}

pub enum krb5_crypto_profile {}
pub enum crypto_aead {}
pub enum crypto_shash {}
pub enum scatterlist {}

extern "C" {
    pub fn crypto_krb5_find_enctype(enctype: u32) -> *const krb5_enctype;
    pub fn crypto_krb5_how_much_buffer(krb5: *const krb5_enctype, mode: krb5_crypto_mode, data_size: usize, offset: *mut usize) -> usize;
    pub fn crypto_krb5_how_much_data(krb5: *const krb5_enctype, mode: krb5_crypto_mode, buffer_size: *mut usize, offset: *mut usize) -> usize;
    pub fn crypto_krb5_where_is_the_data(krb5: *const krb5_enctype, mode: krb5_crypto_mode, offset: *mut usize, len: *mut usize) -> i32;
    pub fn crypto_krb5_check_data_len(krb5: *const krb5_enctype, mode: krb5_crypto_mode, len: usize, min_content: usize) -> i32;
    pub fn crypto_krb5_prepare_encryption(krb5: *const krb5_enctype, tk: *const krb5_buffer, usage: u32, gfp: u32) -> *mut crypto_aead;
    pub fn crypto_krb5_prepare_checksum(krb5: *const krb5_enctype, tk: *const krb5_buffer, usage: u32, gfp: u32) -> *mut crypto_shash;
    pub fn crypto_krb5_encrypt(krb5: *const krb5_enctype, aead: *mut crypto_aead, sg: *mut scatterlist, nr_sg: core::ffi::c_uint, sg_len: usize, data_offset: usize, data_len: usize, preconfounded: bool) -> isize;
    pub fn crypto_krb5_decrypt(krb5: *const krb5_enctype, aead: *mut crypto_aead, sg: *mut scatterlist, nr_sg: core::ffi::c_uint, offset: *mut usize, len: *mut usize) -> i32;
    pub fn crypto_krb5_get_mic(krb5: *const krb5_enctype, shash: *mut crypto_shash, metadata: *const krb5_buffer, sg: *mut scatterlist, nr_sg: core::ffi::c_uint, sg_len: usize, data_offset: usize, data_len: usize) -> isize;
    pub fn crypto_krb5_verify_mic(krb5: *const krb5_enctype, shash: *mut crypto_shash, metadata: *const krb5_buffer, sg: *mut scatterlist, nr_sg: core::ffi::c_uint, offset: *mut usize, len: *mut usize) -> i32;
    pub fn crypto_krb5_calc_PRFplus(krb5: *const krb5_enctype, k: *const krb5_buffer, l: core::ffi::c_uint, s: *const krb5_buffer, result: *mut krb5_buffer, gfp: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
