/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2018-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// The C header guard is omitted; Rust modules provide equivalent inclusion
// protection.

use core::ffi::c_char;

// Types and constants supplied by the surrounding kernel environment.
#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct folio {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    pub i_blocks: u64,
}

pub type loff_t = i64;

#[repr(C)]
pub struct xfile {
    pub file: *mut file,
}

extern "C" {
    pub fn xfile_create(
        description: *const c_char,
        isize: loff_t,
        xfilep: *mut *mut xfile,
    ) -> i32;
    pub fn xfile_destroy(xf: *mut xfile);

    pub fn xfile_load(xf: *mut xfile, buf: *mut core::ffi::c_void, count: usize, pos: loff_t) -> i32;
    pub fn xfile_store(
        xf: *mut xfile,
        buf: *const core::ffi::c_void,
        count: usize,
        pos: loff_t,
    ) -> i32;

    pub fn xfile_discard(xf: *mut xfile, pos: loff_t, count: u64);
    pub fn xfile_seek_data(xf: *mut xfile, pos: loff_t) -> loff_t;

    pub fn file_inode(file: *mut file) -> *mut inode;
}

// XFILE_MAX_FOLIO_SIZE = (PAGE_SIZE << MAX_PAGECACHE_ORDER)
pub const XFILE_MAX_FOLIO_SIZE: usize = PAGE_SIZE << MAX_PAGECACHE_ORDER;

pub const XFILE_ALLOC: u32 = 1 << 0; /* allocate folio if not present */

extern "C" {
    pub fn xfile_get_folio(
        xf: *mut xfile,
        offset: loff_t,
        len: usize,
        flags: u32,
    ) -> *mut folio;
    pub fn xfile_put_folio(xf: *mut xfile, folio: *mut folio);
}

pub unsafe fn xfile_bytes(xf: *mut xfile) -> u64 {
    ((*file_inode((*xf).file)).i_blocks) << SECTOR_SHIFT
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
