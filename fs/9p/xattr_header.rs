/* SPDX-License-Identifier: LGPL-2.1 */
/*
 * Copyright IBM Corporation, 2010
 * Author Aneesh Kumar K.V <aneesh.kumar@linux.vnet.ibm.com>
 */

// Dependencies supplied by the Linux xattr and 9P headers are represented by
// the corresponding external types and ABI-compatible Rust types here.

use ::core::ffi::{c_char, c_int, c_void};

pub type ssize_t = isize;
pub type size_t = usize;

pub struct xattr_handler;
pub struct p9_fid;
pub struct dentry;

extern "C" {
    pub static v9fs_xattr_handlers: *const *const xattr_handler;

    pub fn v9fs_fid_xattr_get(
        fid: *mut p9_fid,
        name: *const c_char,
        buffer: *mut c_void,
        buffer_size: size_t,
    ) -> ssize_t;

    pub fn v9fs_xattr_get(
        dentry: *mut dentry,
        name: *const c_char,
        buffer: *mut c_void,
        buffer_size: size_t,
    ) -> ssize_t;

    pub fn v9fs_fid_xattr_set(
        fid: *mut p9_fid,
        name: *const c_char,
        value: *const c_void,
        value_len: size_t,
        flags: c_int,
    ) -> c_int;

    pub fn v9fs_xattr_set(
        dentry: *mut dentry,
        name: *const c_char,
        value: *const c_void,
        value_len: size_t,
        flags: c_int,
    ) -> c_int;

    pub fn v9fs_listxattr(
        dentry: *mut dentry,
        buffer: *mut c_char,
        buffer_size: size_t,
    ) -> ssize_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
