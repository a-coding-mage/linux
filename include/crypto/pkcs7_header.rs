/* SPDX-License-Identifier: GPL-2.0-or-later */
/* PKCS#7 crypto data parser
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

use core::ffi::c_void;

/* Types declared by the included headers. */
pub type key_being_used_for = crate::key_being_used_for;
pub type hash_algo = crate::hash_algo;

pub type u8 = core::primitive::u8;
pub type u32 = core::primitive::u32;

pub enum key {}
pub enum pkcs7_message {}

/*
 * pkcs7_parser.c
 */
unsafe extern "C" {
    pub fn pkcs7_parse_message(data: *const c_void, datalen: usize) -> *mut pkcs7_message;
    pub fn pkcs7_free_message(pkcs7: *mut pkcs7_message);

    pub fn pkcs7_get_content_data(
        pkcs7: *const pkcs7_message,
        _data: *mut *const c_void,
        _datalen: *mut usize,
        _headerlen: *mut usize,
    ) -> i32;

    /*
     * pkcs7_trust.c
     */
    pub fn pkcs7_validate_trust(
        pkcs7: *mut pkcs7_message,
        trust_keyring: *mut key,
    ) -> i32;

    /*
     * pkcs7_verify.c
     */
    pub fn pkcs7_verify(
        pkcs7: *mut pkcs7_message,
        usage: key_being_used_for,
    ) -> i32;

    pub fn pkcs7_supply_detached_data(
        pkcs7: *mut pkcs7_message,
        data: *const c_void,
        datalen: usize,
    ) -> i32;

    pub fn pkcs7_get_digest(
        pkcs7: *mut pkcs7_message,
        buf: *mut *const u8,
        len: *mut u32,
        hash_algo: *mut hash_algo,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
