// SPDX-License-Identifier: GPL-2.0-only
/*
 * ecryptfs_format.c: helper functions for the encrypted key type
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

use core::ffi::{c_char, c_int, c_uchar, c_ushort};

pub type u8 = c_uchar;
pub type uint16_t = c_ushort;

/*
 * Dependency intent from the original C file:
 *   #include <linux/export.h>
 *   #include <linux/string.h>
 *   #include "ecryptfs_format.h"
 */

#[repr(C)]
pub struct ecryptfs_password {
    pub signature: [c_char; 0],
    pub session_key_encryption_key_bytes: c_int,
    pub flags: c_int,
    pub hash_algo: c_int,
    pub session_key_encryption_key: [u8; 0],
}

#[repr(C)]
pub union ecryptfs_token {
    pub password: core::mem::ManuallyDrop<ecryptfs_password>,
}

#[repr(C)]
pub struct ecryptfs_session_key {
    pub encrypted_key: [u8; 0],
    pub encrypted_key_size: c_int,
}

#[repr(C)]
pub struct ecryptfs_auth_tok {
    pub version: uint16_t,
    pub token_type: c_int,
    pub token: ecryptfs_token,
    pub session_key: ecryptfs_session_key,
}

unsafe extern "C" {
    static ECRYPTFS_VERSION_MAJOR: c_int;
    static ECRYPTFS_VERSION_MINOR: c_int;
    static ECRYPTFS_SUPPORTED_FILE_VERSION: c_int;
    static ECRYPTFS_PASSWORD: c_int;
    static ECRYPTFS_MAX_KEY_BYTES: c_int;
    static ECRYPTFS_SESSION_KEY_ENCRYPTION_KEY_SET: c_int;
    static PGP_DIGEST_ALGO_SHA512: c_int;
    static ECRYPTFS_PERSISTENT_PASSWORD: c_int;

    fn strscpy_pad(dest: *mut c_char, src: *const c_char);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ecryptfs_get_auth_tok_key(
    auth_tok: *mut ecryptfs_auth_tok,
) -> *mut u8 {
    unsafe {
        core::ptr::addr_of_mut!((*auth_tok).token.password.session_key_encryption_key)
            as *mut u8
    }
}
/* EXPORT_SYMBOL(ecryptfs_get_auth_tok_key); */

/*
 * ecryptfs_get_versions()
 *
 * Source code taken from the software 'ecryptfs-utils' version 83.
 *
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ecryptfs_get_versions(
    major: *mut c_int,
    minor: *mut c_int,
    file_version: *mut c_int,
) {
    unsafe {
        *major = ECRYPTFS_VERSION_MAJOR;
        *minor = ECRYPTFS_VERSION_MINOR;
        if !file_version.is_null() {
            *file_version = ECRYPTFS_SUPPORTED_FILE_VERSION;
        }
    }
}
/* EXPORT_SYMBOL(ecryptfs_get_versions); */

/*
 * ecryptfs_fill_auth_tok - fill the ecryptfs_auth_tok structure
 *
 * Fill the ecryptfs_auth_tok structure with required ecryptfs data.
 * The source code is inspired to the original function generate_payload()
 * shipped with the software 'ecryptfs-utils' version 83.
 *
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ecryptfs_fill_auth_tok(
    auth_tok: *mut ecryptfs_auth_tok,
    key_desc: *const c_char,
) -> c_int {
    let mut major: c_int = 0;
    let mut minor: c_int = 0;

    unsafe {
        ecryptfs_get_versions(&mut major, &mut minor, core::ptr::null_mut());
        (*auth_tok).version = ((((major << 8) as uint16_t) & 0xFF00)
            | ((minor as uint16_t) & 0x00FF)) as uint16_t;
        (*auth_tok).token_type = ECRYPTFS_PASSWORD;
        strscpy_pad(
            core::ptr::addr_of_mut!((*auth_tok).token.password.signature) as *mut c_char,
            key_desc,
        );
        (*auth_tok)
            .token
            .password
            .session_key_encryption_key_bytes = ECRYPTFS_MAX_KEY_BYTES;
        /*
         * Removed auth_tok->token.password.salt and
         * auth_tok->token.password.session_key_encryption_key
         * initialization from the original code
         */
        /* TODO: Make the hash parameterizable via policy */
        (*auth_tok).token.password.flags |= ECRYPTFS_SESSION_KEY_ENCRYPTION_KEY_SET;
        /* The kernel code will encrypt the session key. */
        (*auth_tok).session_key.encrypted_key[0] = 0;
        (*auth_tok).session_key.encrypted_key_size = 0;
        /* Default; subject to change by kernel eCryptfs */
        (*auth_tok).token.password.hash_algo = PGP_DIGEST_ALGO_SHA512;
        (*auth_tok).token.password.flags &= !ECRYPTFS_PERSISTENT_PASSWORD;
    }
    0
}
/* EXPORT_SYMBOL(ecryptfs_fill_auth_tok); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
