/* SPDX-License-Identifier: GPL-2.0-or-later */
/* System keyring containing trusted public keys.
 *
 * Copyright (C) 2013 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// C dependency: <linux/key.h>

#[repr(C)]
pub enum blacklist_hash_type {
    /* TBSCertificate hash */
    BLACKLIST_HASH_X509_TBS = 1,
    /* Raw data hash */
    BLACKLIST_HASH_BINARY = 2,
}

// CONFIG_SYSTEM_TRUSTED_KEYRING
extern "C" {
    pub fn restrict_link_by_builtin_trusted(
        keyring: *mut key,
        type_: *const key_type,
        payload: *const key_payload,
        restriction_key: *mut key,
    ) -> ::core::ffi::c_int;
    pub fn restrict_link_by_digsig_builtin(
        dest_keyring: *mut key,
        type_: *const key_type,
        payload: *const key_payload,
        restriction_key: *mut key,
    ) -> ::core::ffi::c_int;
    pub fn load_module_cert(keyring: *mut key) -> ::core::ffi::c_int;
}

// When CONFIG_SYSTEM_TRUSTED_KEYRING is disabled:
// restrict_link_by_builtin_trusted and restrict_link_by_digsig_builtin alias
// restrict_link_reject, and load_module_cert returns 0.
// The aliases and fallback are selected by the build configuration.
#[inline]
pub unsafe fn load_module_cert_disabled(_keyring: *mut key) -> ::core::ffi::c_int {
    0
}

// CONFIG_SECONDARY_TRUSTED_KEYRING
extern "C" {
    pub fn restrict_link_by_builtin_and_secondary_trusted(
        keyring: *mut key,
        type_: *const key_type,
        payload: *const key_payload,
        restriction_key: *mut key,
    ) -> ::core::ffi::c_int;
    pub fn restrict_link_by_digsig_builtin_and_secondary(
        keyring: *mut key,
        type_: *const key_type,
        payload: *const key_payload,
        restriction_key: *mut key,
    ) -> ::core::ffi::c_int;
    pub fn add_to_secondary_keyring(source: *const ::core::ffi::c_char, data: *const ::core::ffi::c_void, len: usize);
}

// When CONFIG_SECONDARY_TRUSTED_KEYRING is disabled, the restriction
// functions alias their builtin counterparts and add_to_secondary_keyring is
// an empty inline function.
#[inline]
pub unsafe fn add_to_secondary_keyring_disabled(
    _source: *const ::core::ffi::c_char,
    _data: *const ::core::ffi::c_void,
    _len: usize,
) {
}

// CONFIG_INTEGRITY_MACHINE_KEYRING
extern "C" {
    pub fn restrict_link_by_builtin_secondary_and_machine(
        dest_keyring: *mut key,
        type_: *const key_type,
        payload: *const key_payload,
        restrict_key: *mut key,
    ) -> ::core::ffi::c_int;
    pub fn set_machine_trusted_keys(keyring: *mut key);
}

// When CONFIG_INTEGRITY_MACHINE_KEYRING is disabled,
// restrict_link_by_builtin_secondary_and_machine aliases
// restrict_link_by_builtin_trusted and set_machine_trusted_keys is empty.
#[inline]
pub unsafe fn set_machine_trusted_keys_disabled(_keyring: *mut key) {
}

// CONFIG_SYSTEM_BLACKLIST_KEYRING
extern "C" {
    pub fn mark_hash_blacklisted(
        hash: *const u8,
        hash_len: usize,
        hash_type: blacklist_hash_type,
    ) -> ::core::ffi::c_int;
    pub fn is_hash_blacklisted(
        hash: *const u8,
        hash_len: usize,
        hash_type: blacklist_hash_type,
    ) -> ::core::ffi::c_int;
    pub fn is_binary_blacklisted(hash: *const u8, hash_len: usize) -> ::core::ffi::c_int;
}

// When CONFIG_SYSTEM_BLACKLIST_KEYRING is disabled, is_hash_blacklisted and
// is_binary_blacklisted return 0.
#[inline]
pub unsafe fn is_hash_blacklisted_disabled(
    _hash: *const u8,
    _hash_len: usize,
    _hash_type: blacklist_hash_type,
) -> ::core::ffi::c_int {
    0
}

#[inline]
pub unsafe fn is_binary_blacklisted_disabled(_hash: *const u8, _hash_len: usize) -> ::core::ffi::c_int {
    0
}

#[repr(C)]
pub struct pkcs7_message {
    _private: [u8; 0],
}

// CONFIG_SYSTEM_REVOCATION_LIST
extern "C" {
    pub fn add_key_to_revocation_list(data: *const ::core::ffi::c_char, size: usize) -> ::core::ffi::c_int;
    pub fn is_key_on_revocation_list(pkcs7: *mut pkcs7_message) -> ::core::ffi::c_int;
}

// When CONFIG_SYSTEM_REVOCATION_LIST is disabled, add_key_to_revocation_list
// returns 0 and is_key_on_revocation_list returns -ENOKEY.
#[inline]
pub unsafe fn add_key_to_revocation_list_disabled(
    _data: *const ::core::ffi::c_char,
    _size: usize,
) -> ::core::ffi::c_int {
    0
}

#[inline]
pub unsafe fn is_key_on_revocation_list_disabled(_pkcs7: *mut pkcs7_message) -> ::core::ffi::c_int {
    -ENOKEY
}

// CONFIG_IMA_BLACKLIST_KEYRING
extern "C" {
    pub static mut ima_blacklist_keyring: *mut key;
}

#[inline]
pub unsafe fn get_ima_blacklist_keyring() -> *mut key {
    ima_blacklist_keyring
}

// When CONFIG_IMA_BLACKLIST_KEYRING is disabled, get_ima_blacklist_keyring
// returns NULL.
#[inline]
pub unsafe fn get_ima_blacklist_keyring_disabled() -> *mut key {
    ::core::ptr::null_mut()
}

// CONFIG_INTEGRITY_PLATFORM_KEYRING && CONFIG_SYSTEM_TRUSTED_KEYRING
extern "C" {
    pub fn set_platform_trusted_keys(keyring: *mut key);
}

// When either platform-keyring condition is disabled, set_platform_trusted_keys
// is an empty inline function.
#[inline]
pub unsafe fn set_platform_trusted_keys_disabled(_keyring: *mut key) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
