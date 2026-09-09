/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2026 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// Translated from xfs_verify_media.h. The C header guard is intentionally
// omitted because Rust items are namespaced by their containing module.

#[repr(C)]
pub struct xfs_verify_media {
    _private: [u8; 0],
}

// `struct file` is supplied by an external dependency.
#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

extern "C" {
    pub fn xfs_ioc_verify_media(
        file: *mut file,
        arg: *mut xfs_verify_media,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
