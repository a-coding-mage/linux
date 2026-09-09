/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Asymmetric public-key cryptography data parser
 *
 * See Documentation/crypto/asymmetric-keys.rst
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// C forward declaration: struct key_preparsed_payload;
pub enum key_preparsed_payload {}

/*
 * Key data parser.  Called during key instantiation.
 */
#[repr(C)]
pub struct asymmetric_key_parser {
    pub link: list_head,
    pub owner: *mut module,
    pub name: *const ::core::ffi::c_char,

    /* Attempt to parse a key from the data blob passed to add_key() or
     * keyctl_instantiate().  Should also generate a proposed description
     * that the caller can optionally use for the key.
     *
     * Return EBADMSG if not recognised.
     */
    pub parse: Option<unsafe extern "C" fn(prep: *mut key_preparsed_payload) -> ::core::ffi::c_int>,
}

// External types supplied by other translation units.
#[allow(non_camel_case_types)]
pub enum list_head {}
#[allow(non_camel_case_types)]
pub enum module {}

unsafe extern "C" {
    pub fn register_asymmetric_key_parser(parser: *mut asymmetric_key_parser) -> ::core::ffi::c_int;
    pub fn unregister_asymmetric_key_parser(parser: *mut asymmetric_key_parser);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
