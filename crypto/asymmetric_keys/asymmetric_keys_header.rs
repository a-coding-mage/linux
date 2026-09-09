/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Internal definitions for asymmetric key type
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

use core::ffi::{c_char, c_void};

/* Dependency supplied by <keys/asymmetric-type.h>. */
pub enum asymmetric_key_id {}
pub enum kernel_pkey_params {}

extern "C" {
    pub fn asymmetric_key_hex_to_key_id(id: *const c_char) -> *mut asymmetric_key_id;

    pub fn __asymmetric_key_hex_to_key_id(
        id: *const c_char,
        match_id: *mut asymmetric_key_id,
        hexlen: usize,
    ) -> i32;

    pub fn asymmetric_key_eds_op(
        params: *mut kernel_pkey_params,
        input: *const c_void,
        output: *mut c_void,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
