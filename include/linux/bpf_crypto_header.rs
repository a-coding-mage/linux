/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

use core::ffi::{c_char, c_int, c_void};

// External type supplied by the surrounding kernel interfaces.
pub struct module;

#[repr(C)]
pub struct bpf_crypto_type {
    pub alloc_tfm: Option<unsafe extern "C" fn(algo: *const c_char) -> *mut c_void>,
    pub free_tfm: Option<unsafe extern "C" fn(tfm: *mut c_void)>,
    pub has_algo: Option<unsafe extern "C" fn(algo: *const c_char) -> c_int>,
    pub setkey: Option<
        unsafe extern "C" fn(
            tfm: *mut c_void,
            key: *const u8,
            keylen: u32,
        ) -> c_int,
    >,
    pub setauthsize: Option<unsafe extern "C" fn(tfm: *mut c_void, authsize: u32) -> c_int>,
    pub encrypt: Option<
        unsafe extern "C" fn(
            tfm: *mut c_void,
            src: *const u8,
            dst: *mut u8,
            len: u32,
            iv: *mut u8,
        ) -> c_int,
    >,
    pub decrypt: Option<
        unsafe extern "C" fn(
            tfm: *mut c_void,
            src: *const u8,
            dst: *mut u8,
            len: u32,
            iv: *mut u8,
        ) -> c_int,
    >,
    pub ivsize: Option<unsafe extern "C" fn(tfm: *mut c_void) -> u32>,
    pub statesize: Option<unsafe extern "C" fn(tfm: *mut c_void) -> u32>,
    pub get_flags: Option<unsafe extern "C" fn(tfm: *mut c_void) -> u32>,
    pub owner: *mut module,
    pub name: [c_char; 14],
}

extern "C" {
    pub fn bpf_crypto_register_type(type_: *const bpf_crypto_type) -> c_int;
    pub fn bpf_crypto_unregister_type(type_: *const bpf_crypto_type) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
