// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.

// Depends on: <linux/types.h>, <linux/audit.h>, policy.h

use std::ffi::c_char;

/// Digest information structure containing algorithm name and digest data
#[repr(C)]
pub struct digest_info {
    /// Algorithm name
    pub alg: *const c_char,
    /// Digest data
    pub digest: *const u8,
    /// Length of digest
    pub digest_len: usize,
}

/// Opaque audit buffer type
pub struct audit_buffer;

extern "C" {
    /// Parse a digest string and return a digest_info structure
    pub fn ipe_digest_parse(valstr: *const c_char) -> *mut digest_info;

    /// Free a digest_info structure
    pub fn ipe_digest_free(digest_info: *mut digest_info);

    /// Audit a digest value
    pub fn ipe_digest_audit(ab: *mut audit_buffer, val: *const digest_info);

    /// Evaluate if two digests are equal
    pub fn ipe_digest_eval(expected: *const digest_info, digest: *const digest_info) -> bool;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
