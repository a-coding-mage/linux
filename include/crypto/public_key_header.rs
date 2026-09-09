/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Asymmetric public-key algorithm definitions
 *
 * See Documentation/crypto/asymmetric-keys.rst
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Dependencies supplied by the surrounding kernel translation are intentionally
 * referenced here rather than implemented in this header translation. */

#[repr(C)]
pub struct public_key {
    pub key: *mut core::ffi::c_void,
    pub keylen: u32,
    pub algo: OID,
    pub params: *mut core::ffi::c_void,
    pub paramlen: u32,
    pub key_is_private: bool,
    pub id_type: *const core::ffi::c_char,
    pub pkey_algo: *const core::ffi::c_char,
    pub key_eflags: usize,
}

pub const KEY_EFLAG_CA: usize = 0;
pub const KEY_EFLAG_DIGITALSIG: usize = 1;
pub const KEY_EFLAG_KEYCERTSIGN: usize = 2;

extern "C" {
    pub fn public_key_free(key: *mut public_key);
}

#[repr(C)]
pub struct public_key_signature {
    pub auth_ids: [*mut asymmetric_key_id; 3],
    pub s: *mut u8,
    pub m: *mut u8,
    pub s_size: u32,
    pub m_size: u32,
    pub m_free: bool,
    pub algo_takes_data: bool,
    pub pkey_algo: *const core::ffi::c_char,
    pub hash_algo: *const core::ffi::c_char,
    pub encoding: *const core::ffi::c_char,
}

extern "C" {
    pub fn public_key_signature_free(sig: *mut public_key_signature);
    pub static mut public_key_subtype: asymmetric_key_subtype;
}

#[repr(C)]
pub struct key {
    _private: [u8; 0],
}

#[repr(C)]
pub struct key_type {
    _private: [u8; 0],
}

#[repr(C)]
pub union key_payload {
    _private: [u8; 0],
}

extern "C" {
    pub fn restrict_link_by_signature(
        dest_keyring: *mut key,
        type_: *const key_type,
        payload: *const key_payload,
        trust_keyring: *mut key,
    ) -> i32;

    pub fn restrict_link_by_key_or_keyring(
        dest_keyring: *mut key,
        type_: *const key_type,
        payload: *const key_payload,
        trusted: *mut key,
    ) -> i32;

    pub fn restrict_link_by_key_or_keyring_chain(
        trust_keyring: *mut key,
        type_: *const key_type,
        payload: *const key_payload,
        trusted: *mut key,
    ) -> i32;
}

/* The following declarations are conditional on
 * IS_REACHABLE(CONFIG_ASYMMETRIC_KEY_TYPE) in the original header. */
#[cfg(feature = "CONFIG_ASYMMETRIC_KEY_TYPE")]
extern "C" {
    pub fn restrict_link_by_ca(
        dest_keyring: *mut key,
        type_: *const key_type,
        payload: *const key_payload,
        trust_keyring: *mut key,
    ) -> i32;

    pub fn restrict_link_by_digsig(
        dest_keyring: *mut key,
        type_: *const key_type,
        payload: *const key_payload,
        trust_keyring: *mut key,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_ASYMMETRIC_KEY_TYPE"))]
pub unsafe fn restrict_link_by_ca(
    _dest_keyring: *mut key,
    _type_: *const key_type,
    _payload: *const key_payload,
    _trust_keyring: *mut key,
) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_ASYMMETRIC_KEY_TYPE"))]
pub unsafe fn restrict_link_by_digsig(
    _dest_keyring: *mut key,
    _type_: *const key_type,
    _payload: *const key_payload,
    _trust_keyring: *mut key,
) -> i32 {
    0
}

extern "C" {
    pub fn query_asymmetric_key(
        params: *const kernel_pkey_params,
        info: *mut kernel_pkey_query,
    ) -> i32;

    pub fn verify_signature(
        key: *const key,
        sig: *const public_key_signature,
    ) -> i32;
}

/* Conditional on IS_REACHABLE(CONFIG_ASYMMETRIC_PUBLIC_KEY_SUBTYPE) in C. */
#[cfg(feature = "CONFIG_ASYMMETRIC_PUBLIC_KEY_SUBTYPE")]
extern "C" {
    pub fn public_key_verify_signature(
        pkey: *const public_key,
        sig: *const public_key_signature,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_ASYMMETRIC_PUBLIC_KEY_SUBTYPE"))]
pub unsafe fn public_key_verify_signature(
    _pkey: *const public_key,
    _sig: *const public_key_signature,
) -> i32 {
    -22 /* -EINVAL */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
