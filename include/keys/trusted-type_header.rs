/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2010 IBM Corporation
 * Author: David Safford <safford@us.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left as external names.

use core::ffi::{c_char, c_int, c_void};

pub const PR_FMT: &str = "trusted_key: ";

pub const MIN_KEY_SIZE: usize = 32;
pub const MAX_KEY_SIZE: usize = 128;
#[cfg(feature = "CONFIG_TRUSTED_KEYS_PKWM")]
pub const MAX_BLOB_SIZE: usize = 1152;
#[cfg(not(feature = "CONFIG_TRUSTED_KEYS_PKWM"))]
pub const MAX_BLOB_SIZE: usize = 512;
pub const MAX_PCRINFO_SIZE: usize = 64;
pub const MAX_DIGEST_SIZE: usize = 64;

#[repr(C)]
pub struct trusted_key_payload {
    pub rcu: rcu_head,
    pub key_len: u32,
    pub blob_len: u32,
    pub migratable: u8,
    pub old_format: u8,
    pub key: [u8; MAX_KEY_SIZE + 1],
    pub blob: [u8; MAX_BLOB_SIZE],
}

#[repr(C)]
pub struct trusted_key_options {
    pub keytype: u16,
    pub keyhandle: u32,
    pub keyauth: [u8; TPM_DIGEST_SIZE],
    pub blobauth_len: u32,
    pub blobauth: [u8; TPM_DIGEST_SIZE],
    pub pcrinfo_len: u32,
    pub pcrinfo: [u8; MAX_PCRINFO_SIZE],
    pub pcrlock: c_int,
    pub hash: u32,
    pub policydigest_len: u32,
    pub policydigest: [u8; MAX_DIGEST_SIZE],
    pub policyhandle: u32,
    pub private: *mut c_void,
}

#[repr(C)]
pub struct trusted_key_ops {
    /* flag to indicate if trusted key implementation supports migration or not. */
    pub migratable: u8,

    /* Initialize key interface. */
    pub init: Option<unsafe extern "C" fn() -> c_int>,

    /* Seal a key. */
    pub seal: Option<unsafe extern "C" fn(*mut trusted_key_payload, *mut c_char) -> c_int>,

    /* Unseal a key. */
    pub unseal: Option<unsafe extern "C" fn(*mut trusted_key_payload, *mut c_char) -> c_int>,

    /* Optional: Get a randomized key. */
    pub get_random: Option<unsafe extern "C" fn(*mut u8, usize) -> c_int>,

    /* Exit key interface. */
    pub exit: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct trusted_key_source {
    pub name: *mut c_char,
    pub ops: *mut trusted_key_ops,
}

extern "C" {
    pub static mut key_type_trusted: key_type;
}

#[cfg(feature = "CONFIG_TRUSTED_KEYS_DEBUG")]
extern "C" {
    pub static mut trusted_debug: bool;
}

#[cfg(feature = "CONFIG_TRUSTED_KEYS_DEBUG")]
#[inline]
pub unsafe fn dump_payload(p: *mut trusted_key_payload) {
    if !trusted_debug {
        return;
    }

    pr_debug!(b"key_len %d\n", (*p).key_len);
    print_hex_dump_debug!(b"key ", DUMP_PREFIX_NONE, 16, 1, (*p).key.as_ptr(), (*p).key_len, 0);
    pr_debug!(b"bloblen %d\n", (*p).blob_len);
    print_hex_dump_debug!(b"blob ", DUMP_PREFIX_NONE, 16, 1, (*p).blob.as_ptr(), (*p).blob_len, 0);
    pr_debug!(b"migratable %d\n", (*p).migratable);
}

#[cfg(not(feature = "CONFIG_TRUSTED_KEYS_DEBUG"))]
#[inline]
pub unsafe fn dump_payload(_p: *mut trusted_key_payload) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
