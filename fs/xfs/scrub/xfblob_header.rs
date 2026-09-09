/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2021-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

use core::ffi::{c_char, c_void};

// Types supplied by the surrounding XFS code.
#[repr(C)]
pub struct xfile {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_name {
    pub name: *const u8,
    pub len: u32,
}

#[repr(C)]
pub struct xfblob {
    pub xfile: *mut xfile,
    pub last_offset: i64,
}

pub type xfblob_cookie = i64;

extern "C" {
    pub fn xfblob_create(descr: *const c_char, blobp: *mut *mut xfblob) -> i32;
    pub fn xfblob_destroy(blob: *mut xfblob);
    pub fn xfblob_load(
        blob: *mut xfblob,
        cookie: xfblob_cookie,
        ptr: *mut c_void,
        size: u32,
    ) -> i32;
    pub fn xfblob_store(
        blob: *mut xfblob,
        cookie: *mut xfblob_cookie,
        ptr: *const c_void,
        size: u32,
    ) -> i32;
    pub fn xfblob_free(blob: *mut xfblob, cookie: xfblob_cookie) -> i32;
    pub fn xfblob_bytes(blob: *mut xfblob) -> u64;
    pub fn xfblob_truncate(blob: *mut xfblob);
}

#[inline]
pub unsafe fn xfblob_storename(
    blob: *mut xfblob,
    cookie: *mut xfblob_cookie,
    xname: *const xfs_name,
) -> i32 {
    xfblob_store(
        blob,
        cookie,
        (*xname).name as *const c_void,
        (*xname).len,
    )
}

#[inline]
pub unsafe fn xfblob_loadname(
    blob: *mut xfblob,
    cookie: xfblob_cookie,
    xname: *mut xfs_name,
    size: u32,
) -> i32 {
    let ret = xfblob_load(
        blob,
        cookie,
        (*xname).name as *mut c_void,
        size,
    );
    if ret != 0 {
        return ret;
    }

    (*xname).len = size;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
