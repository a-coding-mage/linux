/* SPDX-License-Identifier: GPL-2.0-or-later */
/* PE Binary parser bits
 *
 * Copyright (C) 2014 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the surrounding translation unit:
// crypto/pkcs7.h and crypto/hash_info.h

use core::ffi::{c_char, c_uint, c_void};

#[repr(C)]
pub struct pefile_context {
    pub header_size: c_uint,
    pub image_checksum_offset: c_uint,
    pub cert_dirent_offset: c_uint,
    pub n_data_dirents: c_uint,
    pub n_sections: c_uint,
    pub certs_size: c_uint,
    pub sig_offset: c_uint,
    pub sig_len: c_uint,
    pub secs: *const section_header,

    /* PKCS#7 MS Individual Code Signing content */
    pub digest: *const c_void,       /* Digest */
    pub digest_len: c_uint,          /* Digest length */
    pub digest_algo: *const c_char,  /* Digest algorithm */
}

// mscode_parser.c
extern "C" {
    pub fn mscode_parse(
        _ctx: *mut c_void,
        content_data: *const c_void,
        data_len: usize,
        asn1hdrlen: usize,
    ) -> i32;
}

#[macro_export]
macro_rules! kenter {
    ($fmt:expr $(, $arg:expr)*) => {
        pr_devel!("==> %s(" $fmt ")\n", module_path!() $(, $arg)*);
    };
}

#[macro_export]
macro_rules! kleave {
    ($fmt:expr $(, $arg:expr)*) => {
        pr_devel!("<== %s()" $fmt "\n", module_path!() $(, $arg)*);
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
