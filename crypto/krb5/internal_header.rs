/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Kerberos5 crypto internals
 *
 * Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Dependencies supplied by the surrounding kernel/Rust translation. */

/* Profile used for key derivation and encryption. */
#[repr(C)]
pub struct krb5_crypto_profile {
    pub calc_PRF: Option<unsafe extern "C" fn(*const krb5_enctype, *const krb5_buffer, *const krb5_buffer, *mut krb5_buffer, gfp_t) -> i32>,
    pub calc_Kc: Option<unsafe extern "C" fn(*const krb5_enctype, *const krb5_buffer, *const krb5_buffer, *mut krb5_buffer, gfp_t) -> i32>,
    pub calc_Ke: Option<unsafe extern "C" fn(*const krb5_enctype, *const krb5_buffer, *const krb5_buffer, *mut krb5_buffer, gfp_t) -> i32>,
    pub calc_Ki: Option<unsafe extern "C" fn(*const krb5_enctype, *const krb5_buffer, *const krb5_buffer, *mut krb5_buffer, gfp_t) -> i32>,
    pub derive_encrypt_keys: Option<unsafe extern "C" fn(*const krb5_enctype, *const krb5_buffer, u32, *mut krb5_buffer, gfp_t) -> i32>,
    pub load_encrypt_keys: Option<unsafe extern "C" fn(*const krb5_enctype, *const krb5_buffer, *const krb5_buffer, *mut krb5_buffer, gfp_t) -> i32>,
    pub derive_checksum_key: Option<unsafe extern "C" fn(*const krb5_enctype, *const krb5_buffer, u32, *mut krb5_buffer, gfp_t) -> i32>,
    pub load_checksum_key: Option<unsafe extern "C" fn(*const krb5_enctype, *const krb5_buffer, *mut krb5_buffer, gfp_t) -> i32>,
    pub encrypt: Option<unsafe extern "C" fn(*const krb5_enctype, *mut crypto_aead, *mut scatterlist, u32, usize, usize, usize, bool) -> isize>,
    pub decrypt: Option<unsafe extern "C" fn(*const krb5_enctype, *mut crypto_aead, *mut scatterlist, u32, *mut usize, *mut usize) -> i32>,
    pub get_mic: Option<unsafe extern "C" fn(*const krb5_enctype, *mut crypto_shash, *const krb5_buffer, *mut scatterlist, u32, usize, usize, usize) -> isize>,
    pub verify_mic: Option<unsafe extern "C" fn(*const krb5_enctype, *mut crypto_shash, *const krb5_buffer, *mut scatterlist, u32, *mut usize, *mut usize) -> i32>,
}

macro_rules! crypto_roundup { ($x:expr) => { round_up($x, CRYPTO_MINALIGN) }; }
macro_rules! krb5_aead_size { ($tfm:expr) => { crypto_roundup!(core::mem::size_of::<aead_request>() + crypto_aead_reqsize($tfm)) }; }
macro_rules! krb5_aead_ivsize { ($tfm:expr) => { crypto_roundup!(crypto_aead_ivsize($tfm)) }; }
macro_rules! krb5_shash_size { ($tfm:expr) => { crypto_roundup!(core::mem::size_of::<shash_desc>() + crypto_shash_descsize($tfm)) }; }
macro_rules! krb5_digest_size { ($tfm:expr) => { crypto_roundup!(crypto_shash_digestsize($tfm)) }; }
macro_rules! round16 { ($x:expr) => { (($x + 15) & !15) }; }

#[repr(C)] pub struct krb5_prf_test { pub etype: u32, pub name: *const i8, pub key: *const i8, pub octet: *const i8, pub prf: *const i8 }
#[repr(C)] pub struct krb5_key_test_one { pub r#use: u32, pub key: *const i8 }
#[repr(C)] pub struct krb5_key_test { pub etype: u32, pub name: *const i8, pub key: *const i8, pub Kc: krb5_key_test_one, pub Ke: krb5_key_test_one, pub Ki: krb5_key_test_one }
#[repr(C)] pub struct krb5_enc_test { pub etype: u32, pub usage: u32, pub name: *const i8, pub plain: *const i8, pub conf: *const i8, pub K0: *const i8, pub Ke: *const i8, pub Ki: *const i8, pub ct: *const i8 }
#[repr(C)] pub struct krb5_mic_test { pub etype: u32, pub usage: u32, pub name: *const i8, pub plain: *const i8, pub K0: *const i8, pub Kc: *const i8, pub mic: *const i8 }

extern "C" {
    pub fn krb5_prepare_encryption(krb5: *const krb5_enctype, keys: *const krb5_buffer, gfp: gfp_t) -> *mut crypto_aead;
    pub fn krb5_prepare_checksum(krb5: *const krb5_enctype, Kc: *const krb5_buffer, gfp: gfp_t) -> *mut crypto_shash;
    pub fn krb5_derive_Kc(krb5: *const krb5_enctype, TK: *const krb5_buffer, usage: u32, key: *mut krb5_buffer, gfp: gfp_t) -> i32;
    pub fn krb5_derive_Ke(krb5: *const krb5_enctype, TK: *const krb5_buffer, usage: u32, key: *mut krb5_buffer, gfp: gfp_t) -> i32;
    pub fn krb5_derive_Ki(krb5: *const krb5_enctype, TK: *const krb5_buffer, usage: u32, key: *mut krb5_buffer, gfp: gfp_t) -> i32;
    pub static rfc3961_simplified_profile: krb5_crypto_profile;
    pub fn crypto_shash_update_sg(desc: *mut shash_desc, sg: *mut scatterlist, offset: usize, len: usize) -> i32;
    pub fn authenc_derive_encrypt_keys(krb5: *const krb5_enctype, TK: *const krb5_buffer, usage: u32, setkey: *mut krb5_buffer, gfp: gfp_t) -> i32;
    pub fn authenc_load_encrypt_keys(krb5: *const krb5_enctype, Ke: *const krb5_buffer, Ki: *const krb5_buffer, setkey: *mut krb5_buffer, gfp: gfp_t) -> i32;
    pub fn rfc3961_derive_checksum_key(krb5: *const krb5_enctype, TK: *const krb5_buffer, usage: u32, setkey: *mut krb5_buffer, gfp: gfp_t) -> i32;
    pub fn rfc3961_load_checksum_key(krb5: *const krb5_enctype, Kc: *const krb5_buffer, setkey: *mut krb5_buffer, gfp: gfp_t) -> i32;
    pub fn krb5_aead_encrypt(krb5: *const krb5_enctype, aead: *mut crypto_aead, sg: *mut scatterlist, nr_sg: u32, sg_len: usize, data_offset: usize, data_len: usize, preconfounded: bool) -> isize;
    pub fn krb5_aead_decrypt(krb5: *const krb5_enctype, aead: *mut crypto_aead, sg: *mut scatterlist, nr_sg: u32, offset: *mut usize, len: *mut usize) -> i32;
    pub fn rfc3961_get_mic(krb5: *const krb5_enctype, shash: *mut crypto_shash, metadata: *const krb5_buffer, sg: *mut scatterlist, nr_sg: u32, sg_len: usize, data_offset: usize, data_len: usize) -> isize;
    pub fn rfc3961_verify_mic(krb5: *const krb5_enctype, shash: *mut crypto_shash, metadata: *const krb5_buffer, sg: *mut scatterlist, nr_sg: u32, offset: *mut usize, len: *mut usize) -> i32;
    pub static krb5_aes128_cts_hmac_sha1_96: krb5_enctype;
    pub static krb5_aes256_cts_hmac_sha1_96: krb5_enctype;
    pub static krb5_camellia128_cts_cmac: krb5_enctype;
    pub static krb5_camellia256_cts_cmac: krb5_enctype;
    pub static krb5_aes128_cts_hmac_sha256_128: krb5_enctype;
    pub static krb5_aes256_cts_hmac_sha384_192: krb5_enctype;
    pub fn krb5_selftest() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
