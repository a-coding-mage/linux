/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2021 Hannes Reinecke, SUSE Software Solutions
 */

// Dependencies supplied by other translation units:
// crypto/kpp.h and crypto/sha2.h

#[repr(C)]
pub struct nvme_dhchap_key {
    pub len: usize,
    pub hash: u8,
    pub key: [u8; 0],
}

extern "C" {
    pub fn nvme_auth_get_seqnum() -> u32;
    pub fn nvme_auth_dhgroup_name(dhgroup_id: u8) -> *const std::os::raw::c_char;
    pub fn nvme_auth_dhgroup_kpp(dhgroup_id: u8) -> *const std::os::raw::c_char;
    pub fn nvme_auth_dhgroup_id(dhgroup_name: *const std::os::raw::c_char) -> u8;

    pub fn nvme_auth_hmac_name(hmac_id: u8) -> *const std::os::raw::c_char;
    pub fn nvme_auth_hmac_hash_len(hmac_id: u8) -> usize;
    pub fn nvme_auth_hmac_id(hmac_name: *const std::os::raw::c_char) -> u8;

    pub fn nvme_auth_hmac_init(
        hmac: *mut nvme_auth_hmac_ctx,
        hmac_id: u8,
        key: *const u8,
        key_len: usize,
    ) -> i32;
    pub fn nvme_auth_hmac_update(
        hmac: *mut nvme_auth_hmac_ctx,
        data: *const u8,
        data_len: usize,
    );
    pub fn nvme_auth_hmac_final(hmac: *mut nvme_auth_hmac_ctx, out: *mut u8);

    pub fn nvme_auth_key_struct_size(key_len: u32) -> u32;
    pub fn nvme_auth_extract_key(
        secret: *const std::os::raw::c_char,
        key_hash: u8,
    ) -> *mut nvme_dhchap_key;
    pub fn nvme_auth_free_key(key: *mut nvme_dhchap_key);
    pub fn nvme_auth_alloc_key(len: u32, hash: u8) -> *mut nvme_dhchap_key;
    pub fn nvme_auth_transform_key(
        key: *const nvme_dhchap_key,
        nqn: *const std::os::raw::c_char,
    ) -> *mut nvme_dhchap_key;
    pub fn nvme_auth_parse_key(
        secret: *const std::os::raw::c_char,
        ret_key: *mut *mut nvme_dhchap_key,
    ) -> i32;
    pub fn nvme_auth_augmented_challenge(
        hmac_id: u8,
        skey: *const u8,
        skey_len: usize,
        challenge: *const u8,
        aug: *mut u8,
        hlen: usize,
    ) -> i32;
    pub fn nvme_auth_gen_privkey(dh_tfm: *mut crypto_kpp, dh_gid: u8) -> i32;
    pub fn nvme_auth_gen_pubkey(
        dh_tfm: *mut crypto_kpp,
        host_key: *mut u8,
        host_key_len: usize,
    ) -> i32;
    pub fn nvme_auth_gen_session_key(
        dh_tfm: *mut crypto_kpp,
        public_key: *const u8,
        public_key_len: usize,
        sess_key: *mut u8,
        sess_key_len: usize,
        hash_id: u8,
    ) -> i32;
    pub fn nvme_auth_generate_psk(
        hmac_id: u8,
        skey: *const u8,
        skey_len: usize,
        c1: *const u8,
        c2: *const u8,
        hash_len: usize,
        ret_psk: *mut *mut u8,
        ret_len: *mut usize,
    ) -> i32;
    pub fn nvme_auth_generate_digest(
        hmac_id: u8,
        psk: *const u8,
        psk_len: usize,
        subsysnqn: *const std::os::raw::c_char,
        hostnqn: *const std::os::raw::c_char,
        ret_digest: *mut *mut std::os::raw::c_char,
    ) -> i32;
    pub fn nvme_auth_derive_tls_psk(
        hmac_id: i32,
        psk: *const u8,
        psk_len: usize,
        psk_digest: *const std::os::raw::c_char,
        ret_psk: *mut *mut u8,
    ) -> i32;
}

#[repr(C)]
pub struct nvme_auth_hmac_ctx {
    pub hmac_id: u8,
    pub _bindgen_union: nvme_auth_hmac_ctx__bindgen_ty_1,
}

#[repr(C)]
pub union nvme_auth_hmac_ctx__bindgen_ty_1 {
    pub sha256: hmac_sha256_ctx,
    pub sha384: hmac_sha384_ctx,
    pub sha512: hmac_sha512_ctx,
}

// Types supplied by crypto/sha2.h.
pub type hmac_sha256_ctx = ::std::os::raw::c_void;
pub type hmac_sha384_ctx = ::std::os::raw::c_void;
pub type hmac_sha512_ctx = ::std::os::raw::c_void;

// Type supplied by crypto/kpp.h.
#[repr(C)]
pub struct crypto_kpp {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
