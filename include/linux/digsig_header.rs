/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2011 Nokia Corporation
 * Copyright (C) 2011 Intel Corporation
 *
 * Author:
 * Dmitry Kasatkin <dmitry.kasatkin@nokia.com>
 *                 <dmitry.kasatkin@intel.com>
 */

// Dependency equivalent of: #include <linux/key.h>

#[repr(C)]
pub struct key {
    _private: [u8; 0],
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum pubkey_algo {
    PUBKEY_ALGO_RSA = 0,
    PUBKEY_ALGO_MAX = 1,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum digest_algo {
    DIGEST_ALGO_SHA1 = 0,
    DIGEST_ALGO_SHA256 = 1,
    DIGEST_ALGO_MAX = 2,
}

#[repr(C, packed)]
pub struct pubkey_hdr {
    pub version: u8,       /* key format version */
    pub timestamp: u32,   /* key made, always 0 for now */
    pub algo: u8,
    pub nmpi: u8,
    pub mpi: [core::ffi::c_char; 0],
}

#[repr(C, packed)]
pub struct signature_hdr {
    pub version: u8,       /* signature format version */
    pub timestamp: u32,    /* signature made */
    pub algo: u8,
    pub hash: u8,
    pub keyid: [u8; 8],
    pub nmpi: u8,
    pub mpi: [core::ffi::c_char; 0],
}

#[cfg(any(CONFIG_SIGNATURE, CONFIG_SIGNATURE_MODULE))]
extern "C" {
    pub fn digsig_verify(
        keyring: *mut key,
        sig: *const core::ffi::c_char,
        siglen: i32,
        digest: *const core::ffi::c_char,
        digestlen: i32,
    ) -> i32;
}

#[cfg(not(any(CONFIG_SIGNATURE, CONFIG_SIGNATURE_MODULE)))]
#[inline]
pub unsafe fn digsig_verify(
    _keyring: *mut key,
    _sig: *const core::ffi::c_char,
    _siglen: i32,
    _digest: *const core::ffi::c_char,
    _digestlen: i32,
) -> i32 {
    // Linux EOPNOTSUPP (from the omitted linux errno dependency).
    -95
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
