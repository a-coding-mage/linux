/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Asymmetric public-key cryptography key subtype
 *
 * See Documentation/crypto/asymmetric-keys.rst
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the corresponding kernel headers:
// <linux/seq_file.h>
// <keys/asymmetric-type.h>

use core::ffi::{c_char, c_int, c_void};

use crate::key;
use crate::kernel_pkey_params;
use crate::kernel_pkey_query;
use crate::module;
use crate::public_key_signature;
use crate::seq_file;

/*
 * Keys of this type declare a subtype that indicates the handlers and
 * capabilities.
 */
#[repr(C)]
pub struct asymmetric_key_subtype {
    pub owner: *mut module,
    pub name: *const c_char,
    pub name_len: u16, /* length of name */

    /* Describe a key of this subtype for /proc/keys */
    pub describe: Option<unsafe extern "C" fn(key: *const key, m: *mut seq_file)>,

    /* Destroy a key of this subtype */
    pub destroy: Option<unsafe extern "C" fn(payload_crypto: *mut c_void, payload_auth: *mut c_void)>,

    pub query: Option<unsafe extern "C" fn(
        params: *const kernel_pkey_params,
        info: *mut kernel_pkey_query,
    ) -> c_int>,

    /* Encrypt/decrypt/sign data */
    pub eds_op: Option<unsafe extern "C" fn(
        params: *mut kernel_pkey_params,
        input: *const c_void,
        output: *mut c_void,
    ) -> c_int>,

    /* Verify the signature on a key of this subtype (optional) */
    pub verify_signature: Option<unsafe extern "C" fn(
        key: *const key,
        sig: *const public_key_signature,
    ) -> c_int>,
}

/**
 * asymmetric_key_subtype - Get the subtype from an asymmetric key
 * @key: The key of interest.
 *
 * Retrieves and returns the subtype pointer of the asymmetric key from the
 * type-specific data attached to the key.
 */
#[inline]
pub unsafe fn asymmetric_key_subtype(key: *const key) -> *mut asymmetric_key_subtype {
    (*key).payload.data[asym_subtype as usize]
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
