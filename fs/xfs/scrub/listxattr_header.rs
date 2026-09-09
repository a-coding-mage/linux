/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2022-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// Opaque types supplied by the surrounding XFS scrub implementation.
#[repr(C)]
pub struct xfs_scrub {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_inode {
    _private: [u8; 0],
}

pub type xchk_xattr_fn = unsafe extern "C" fn(
    sc: *mut xfs_scrub,
    ip: *mut xfs_inode,
    attr_flags: ::core::ffi::c_uint,
    name: *const ::core::ffi::c_uchar,
    namelen: ::core::ffi::c_uint,
    value: *const ::core::ffi::c_void,
    valuelen: ::core::ffi::c_uint,
    priv_: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int;

pub type xchk_xattrleaf_fn = unsafe extern "C" fn(
    sc: *mut xfs_scrub,
    priv_: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int;

extern "C" {
    pub fn xchk_xattr_walk(
        sc: *mut xfs_scrub,
        ip: *mut xfs_inode,
        attr_fn: xchk_xattr_fn,
        leaf_fn: xchk_xattrleaf_fn,
        priv_: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
