/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Module signature handling.
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the corresponding Linux headers:
// <linux/types.h>
// <uapi/linux/module_signature.h>

use core::ffi::{c_char, c_int};

/// Opaque declaration supplied by <uapi/linux/module_signature.h>.
#[repr(C)]
pub struct module_signature {
    _private: [u8; 0],
}

extern "C" {
    pub fn mod_check_sig(
        ms: *const module_signature,
        file_len: usize,
        name: *const c_char,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
