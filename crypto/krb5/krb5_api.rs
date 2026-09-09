// SPDX-License-Identifier: GPL-2.0-or-later
/* Kerberos 5 crypto library.
 *
 * Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Kernel and internal project declarations are supplied by the surrounding build.

extern "C" {
    static krb5_aes128_cts_hmac_sha1_96: krb5_enctype;
    static krb5_aes256_cts_hmac_sha1_96: krb5_enctype;
    static krb5_aes128_cts_hmac_sha256_128: krb5_enctype;
    static krb5_aes256_cts_hmac_sha384_192: krb5_enctype;
    static krb5_camellia128_cts_cmac: krb5_enctype;
    static krb5_camellia256_cts_cmac: krb5_enctype;
    fn krb5_selftest() -> i32;
}

#[repr(C)]
pub struct krb5_enctype {
    pub etype: u32,
    pub cksum_len: usize,
    pub conf_len: usize,
    pub encrypt_name: *const core::ffi::c_char,
    pub cksum_name: *const core::ffi::c_char,
    pub profile: *const krb5_profile,
}

#[repr(C)]
pub struct krb5_profile {
    pub derive_encrypt_keys: unsafe extern "C" fn(*const krb5_enctype, *const krb5_buffer, u32, *mut krb5_buffer, gfp_t) -> i32,
    pub derive_checksum_key: unsafe extern "C" fn(*const krb5_enctype, *const krb5_buffer, u32, *mut krb5_buffer, gfp_t) -> i32,
    pub encrypt: unsafe extern "C" fn(*const krb5_enctype, *mut crypto_aead, *mut scatterlist, u32, usize, usize, usize, bool) -> isize,
    pub decrypt: unsafe extern "C" fn(*const krb5_enctype, *mut crypto_aead, *mut scatterlist, u32, *mut usize, *mut usize) -> i32,
    pub get_mic: unsafe extern "C" fn(*const krb5_enctype, *mut crypto_shash, *const krb5_buffer, *mut scatterlist, u32, usize, usize, usize) -> isize,
    pub verify_mic: unsafe extern "C" fn(*const krb5_enctype, *mut crypto_shash, *const krb5_buffer, *mut scatterlist, u32, *mut usize, *mut usize) -> i32,
}

#[repr(C)]
pub struct krb5_buffer { pub data: *mut u8, pub len: usize }
#[repr(C)] pub struct crypto_aead { _private: [u8; 0] }
#[repr(C)] pub struct crypto_shash { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
pub type gfp_t = u32;
pub type krb5_crypto_mode = i32;
pub const KRB5_CHECKSUM_MODE: krb5_crypto_mode = 0;
pub const KRB5_ENCRYPT_MODE: krb5_crypto_mode = 1;

const ENOENT: i32 = 2;
const ENOPKG: i32 = 65;
const ENOMEM: i32 = 12;
const EMSGSIZE: i32 = 90;
const EBADMSG: i32 = 74;
const EINVAL: i32 = 22;

static KRb5_SUPPORTED_ENCTYPES: [*const krb5_enctype; 6] = [
    unsafe { &krb5_aes128_cts_hmac_sha1_96 }, unsafe { &krb5_aes256_cts_hmac_sha1_96 },
    unsafe { &krb5_aes128_cts_hmac_sha256_128 }, unsafe { &krb5_aes256_cts_hmac_sha384_192 },
    unsafe { &krb5_camellia128_cts_cmac }, unsafe { &krb5_camellia256_cts_cmac },
];

pub unsafe fn crypto_krb5_find_enctype(enctype: u32) -> *const krb5_enctype {
    for krb5 in KRb5_SUPPORTED_ENCTYPES {
        if (*krb5).etype == enctype { return krb5; }
    }
    core::ptr::null()
}

pub unsafe fn crypto_krb5_how_much_buffer(krb5: *const krb5_enctype, mode: krb5_crypto_mode, data_size: usize, offset: *mut usize) -> usize {
    match mode {
        KRB5_CHECKSUM_MODE => { *offset = (*krb5).cksum_len; (*krb5).cksum_len + data_size }
        KRB5_ENCRYPT_MODE => { *offset = (*krb5).conf_len; (*krb5).conf_len + data_size + (*krb5).cksum_len }
        _ => { *offset = 0; 0 }
    }
}

pub unsafe fn crypto_krb5_how_much_data(krb5: *const krb5_enctype, mode: krb5_crypto_mode, buffer_size: *mut usize, offset: *mut usize) -> usize {
    let size = *buffer_size;
    match mode {
        KRB5_CHECKSUM_MODE if size >= (*krb5).cksum_len + 1 => { *offset = (*krb5).cksum_len; size - (*krb5).cksum_len }
        KRB5_ENCRYPT_MODE if size >= (*krb5).conf_len + 1 + (*krb5).cksum_len => { *offset = (*krb5).conf_len; size - (*krb5).cksum_len - (*krb5).conf_len }
        _ => { *offset = 0; 0 }
    }
}

pub unsafe fn crypto_krb5_where_is_the_data(krb5: *const krb5_enctype, mode: krb5_crypto_mode, offset: *mut usize, len: *mut usize) -> i32 {
    match mode {
        KRB5_CHECKSUM_MODE if *len >= (*krb5).cksum_len => { *offset += (*krb5).cksum_len; *len -= (*krb5).cksum_len; 0 }
        KRB5_ENCRYPT_MODE if *len >= (*krb5).conf_len + (*krb5).cksum_len => { *offset += (*krb5).conf_len; *len -= (*krb5).conf_len + (*krb5).cksum_len; 0 }
        KRB5_CHECKSUM_MODE | KRB5_ENCRYPT_MODE => -EBADMSG,
        _ => -EINVAL,
    }
}

pub unsafe fn crypto_krb5_check_data_len(krb5: *const krb5_enctype, mode: krb5_crypto_mode, len: usize, min_content: usize) -> i32 {
    match mode {
        KRB5_CHECKSUM_MODE if len >= (*krb5).cksum_len && len - (*krb5).cksum_len >= min_content => 0,
        KRB5_ENCRYPT_MODE if len >= (*krb5).conf_len + (*krb5).cksum_len && len - (*krb5).conf_len - (*krb5).cksum_len >= min_content => 0,
        KRB5_CHECKSUM_MODE | KRB5_ENCRYPT_MODE => -EBADMSG,
        _ => -EINVAL,
    }
}

extern "C" {
    fn crypto_alloc_aead(name: *const core::ffi::c_char, type_: u32, mask: u32) -> *mut crypto_aead;
    fn crypto_aead_setkey(ci: *mut crypto_aead, data: *mut u8, len: usize) -> i32;
    fn crypto_aead_setauthsize(ci: *mut crypto_aead, len: usize) -> i32;
    fn crypto_free_aead(ci: *mut crypto_aead);
    fn crypto_alloc_shash(name: *const core::ffi::c_char, type_: u32, mask: u32) -> *mut crypto_shash;
    fn crypto_shash_setkey(ci: *mut crypto_shash, data: *mut u8, len: usize) -> i32;
    fn crypto_free_shash(ci: *mut crypto_shash);
    fn kfree_sensitive(data: *mut u8);
}

pub unsafe fn krb5_prepare_encryption(krb5: *const krb5_enctype, keys: *const krb5_buffer, _gfp: gfp_t) -> *mut crypto_aead {
    let ci = crypto_alloc_aead((*krb5).encrypt_name, 0, 0);
    if ci.is_null() { return core::ptr::null_mut(); }
    if crypto_aead_setkey(ci, (*keys).data, (*keys).len) < 0 || crypto_aead_setauthsize(ci, (*krb5).cksum_len) < 0 {
        crypto_free_aead(ci); return core::ptr::null_mut();
    }
    ci
}

pub unsafe fn crypto_krb5_prepare_encryption(krb5: *const krb5_enctype, tk: *const krb5_buffer, usage: u32, gfp: gfp_t) -> *mut crypto_aead {
    let mut keys = krb5_buffer { data: core::ptr::null_mut(), len: 0 };
    let ret = ((*(*krb5).profile).derive_encrypt_keys)(krb5, tk, usage, &mut keys, gfp);
    if ret < 0 { return core::ptr::null_mut(); }
    let ci = krb5_prepare_encryption(krb5, &keys, gfp);
    kfree_sensitive(keys.data); ci
}

pub unsafe fn krb5_prepare_checksum(krb5: *const krb5_enctype, kc: *const krb5_buffer, _gfp: gfp_t) -> *mut crypto_shash {
    let ci = crypto_alloc_shash((*krb5).cksum_name, 0, 0);
    if ci.is_null() { return core::ptr::null_mut(); }
    if crypto_shash_setkey(ci, (*kc).data, (*kc).len) < 0 { crypto_free_shash(ci); return core::ptr::null_mut(); }
    ci
}

pub unsafe fn crypto_krb5_prepare_checksum(krb5: *const krb5_enctype, tk: *const krb5_buffer, usage: u32, gfp: gfp_t) -> *mut crypto_shash {
    let mut keys = krb5_buffer { data: core::ptr::null_mut(), len: 0 };
    let ret = ((*(*krb5).profile).derive_checksum_key)(krb5, tk, usage, &mut keys, gfp);
    if ret < 0 { return core::ptr::null_mut(); }
    let ci = krb5_prepare_checksum(krb5, &keys, gfp);
    kfree_sensitive(keys.data); ci
}

pub unsafe fn crypto_krb5_encrypt(krb5: *const krb5_enctype, aead: *mut crypto_aead, sg: *mut scatterlist, nr_sg: u32, sg_len: usize, data_offset: usize, data_len: usize, preconfounded: bool) -> isize {
    if data_offset > sg_len || data_len > sg_len || data_offset > sg_len - data_len { return -(EMSGSIZE as isize); }
    ((*(*krb5).profile).encrypt)(krb5, aead, sg, nr_sg, sg_len, data_offset, data_len, preconfounded)
}
pub unsafe fn crypto_krb5_decrypt(krb5: *const krb5_enctype, aead: *mut crypto_aead, sg: *mut scatterlist, nr_sg: u32, offset: *mut usize, len: *mut usize) -> i32 { ((*(*krb5).profile).decrypt)(krb5, aead, sg, nr_sg, offset, len) }
pub unsafe fn crypto_krb5_get_mic(krb5: *const krb5_enctype, shash: *mut crypto_shash, metadata: *const krb5_buffer, sg: *mut scatterlist, nr_sg: u32, sg_len: usize, data_offset: usize, data_len: usize) -> isize {
    if data_offset > sg_len || data_len > sg_len || data_offset > sg_len - data_len { return -(EMSGSIZE as isize); }
    ((*(*krb5).profile).get_mic)(krb5, shash, metadata, sg, nr_sg, sg_len, data_offset, data_len)
}
pub unsafe fn crypto_krb5_verify_mic(krb5: *const krb5_enctype, shash: *mut crypto_shash, metadata: *const krb5_buffer, sg: *mut scatterlist, nr_sg: u32, offset: *mut usize, len: *mut usize) -> i32 { ((*(*krb5).profile).verify_mic)(krb5, shash, metadata, sg, nr_sg, offset, len) }

#[allow(dead_code)]
unsafe fn crypto_krb5_init() -> i32 { krb5_selftest() }
#[allow(dead_code)]
unsafe fn crypto_krb5_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
