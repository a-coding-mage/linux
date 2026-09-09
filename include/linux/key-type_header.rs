/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Definitions for key type implementations
 *
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* CONFIG_KEYS */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

/* Supplied by the included kernel headers. */
use crate::{key, key_payload, key_restriction, kernel_pkey_params, kernel_pkey_query,
            list_head, lock_class_key, seq_file};

/*
 * Pre-parsed payload, used by key add, update and instantiate.
 *
 * This struct will be cleared and data and datalen will be set with the data
 * and length parameters from the caller and quotalen will be set from
 * def_datalen from the key type.  Then if the preparse() op is provided by the
 * key type, that will be called.  Then the struct will be passed to the
 * instantiate() or the update() op.
 *
 * If the preparse() op is given, the free_preparse() op will be called to
 * clear the contents.
 */
#[repr(C)]
pub struct key_preparsed_payload {
    pub orig_description: *const c_char,
    pub description: *mut c_char,
    pub payload: key_payload,
    pub data: *const c_void,
    pub datalen: usize,
    pub quotalen: usize,
    pub expiry: i64,
}

pub type request_key_actor_t = unsafe extern "C" fn(auth_key: *mut key, aux: *mut c_void) -> c_int;

/* Preparsed matching criterion. */
#[repr(C)]
pub struct key_match_data {
    pub cmp: Option<unsafe extern "C" fn(
        key: *const key,
        match_data: *const key_match_data,
    ) -> bool>,
    pub raw_data: *const c_void,
    pub preparsed: *mut c_void,
    pub lookup_type: c_uint,
}

pub const KEYRING_SEARCH_LOOKUP_DIRECT: c_uint = 0x0000;
pub const KEYRING_SEARCH_LOOKUP_ITERATE: c_uint = 0x0001;

/* Kernel managed key type definition. */
#[repr(C)]
pub struct key_type {
    pub name: *const c_char,
    pub def_datalen: usize,
    pub flags: c_uint,
    pub vet_description: Option<unsafe extern "C" fn(description: *const c_char) -> c_int>,
    pub preparse: Option<unsafe extern "C" fn(prep: *mut key_preparsed_payload) -> c_int>,
    pub free_preparse: Option<unsafe extern "C" fn(prep: *mut key_preparsed_payload)>,
    pub instantiate: Option<unsafe extern "C" fn(key: *mut key, prep: *mut key_preparsed_payload) -> c_int>,
    pub update: Option<unsafe extern "C" fn(key: *mut key, prep: *mut key_preparsed_payload) -> c_int>,
    pub match_preparse: Option<unsafe extern "C" fn(match_data: *mut key_match_data) -> c_int>,
    pub match_free: Option<unsafe extern "C" fn(match_data: *mut key_match_data)>,
    pub revoke: Option<unsafe extern "C" fn(key: *mut key)>,
    pub destroy: Option<unsafe extern "C" fn(key: *mut key)>,
    pub describe: Option<unsafe extern "C" fn(key: *const key, p: *mut seq_file)>,
    pub read: Option<unsafe extern "C" fn(key: *const key, buffer: *mut c_char, buflen: usize) -> c_long>,
    pub request_key: Option<request_key_actor_t>,
    pub lookup_restriction: Option<unsafe extern "C" fn(params: *const c_char) -> *mut key_restriction>,
    pub asym_query: Option<unsafe extern "C" fn(params: *const kernel_pkey_params, info: *mut kernel_pkey_query) -> c_int>,
    pub asym_eds_op: Option<unsafe extern "C" fn(params: *mut kernel_pkey_params, input: *const c_void, output: *mut c_void) -> c_int>,
    pub asym_verify_signature: Option<unsafe extern "C" fn(params: *mut kernel_pkey_params, input: *const c_void, input2: *const c_void) -> c_int>,
    pub link: list_head,
    pub lock_class: lock_class_key,
}

pub const KEY_TYPE_NET_DOMAIN: c_uint = 0x00000001;
pub const KEY_TYPE_INSTANT_REAP: c_uint = 0x00000002;

unsafe extern "C" {
    pub static mut key_type_keyring: key_type;
    pub fn register_key_type(ktype: *mut key_type) -> c_int;
    pub fn unregister_key_type(ktype: *mut key_type);
    pub fn key_payload_reserve(key: *mut key, datalen: usize) -> c_int;
    pub fn key_instantiate_and_link(key: *mut key, data: *const c_void, datalen: usize,
                                    keyring: *mut key, authkey: *mut key) -> c_int;
    pub fn key_reject_and_link(key: *mut key, timeout: c_uint, error: c_uint,
                               keyring: *mut key, authkey: *mut key) -> c_int;
    pub fn complete_request_key(authkey: *mut key, error: c_int);
    pub fn generic_key_instantiate(key: *mut key, prep: *mut key_preparsed_payload) -> c_int;
}

#[inline]
pub unsafe fn key_negate_and_link(key: *mut key, timeout: c_uint,
                                  keyring: *mut key, authkey: *mut key) -> c_int {
    key_reject_and_link(key, timeout, crate::ENOKEY as c_uint, keyring, authkey)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
