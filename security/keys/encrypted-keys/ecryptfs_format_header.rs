// SPDX-License-Identifier: GPL-2.0-only
/*
 * ecryptfs_format.h: helper functions for the encrypted key type
 *
 * Copyright (C) 2006 International Business Machines Corp.
 * Copyright (C) 2010 Politecnico di Torino, Italy
 *                    TORSEC group -- https://security.polito.it
 *
 * Authors:
 * Michael A. Halcrow <mahalcro@us.ibm.com>
 * Tyler Hicks <tyhicks@ou.edu>
 * Roberto Sassu <roberto.sassu@polito.it>
 */

// C dependency: #include <linux/ecryptfs.h>
// Provides the external `ecryptfs_auth_tok` type referenced below.

use core::ffi::{c_char, c_int};

pub const PGP_DIGEST_ALGO_SHA512: u32 = 10;

unsafe extern "C" {
    pub fn ecryptfs_get_auth_tok_key(auth_tok: *mut crate::ecryptfs_auth_tok) -> *mut u8;
    pub fn ecryptfs_get_versions(major: *mut c_int, minor: *mut c_int, file_version: *mut c_int);
    pub fn ecryptfs_fill_auth_tok(
        auth_tok: *mut crate::ecryptfs_auth_tok,
        key_desc: *const c_char,
    ) -> c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
