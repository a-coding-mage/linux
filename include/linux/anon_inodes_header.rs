/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  include/linux/anon_inodes.h
 *
 *  Copyright (C) 2007  Davide Libenzi <davidel@xmailserver.org>
 *
 */

// C dependency: <linux/types.h>

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

// Supplied by the Linux type definitions.
pub type fmode_t = u32;

extern "C" {
    pub fn anon_inode_getfile(
        name: *const c_char,
        fops: *const file_operations,
        priv_: *mut c_void,
        flags: c_int,
    ) -> *mut file;

    pub fn anon_inode_getfile_fmode(
        name: *const c_char,
        fops: *const file_operations,
        priv_: *mut c_void,
        flags: c_int,
        f_mode: fmode_t,
    ) -> *mut file;

    pub fn anon_inode_create_getfile(
        name: *const c_char,
        fops: *const file_operations,
        priv_: *mut c_void,
        flags: c_int,
        context_inode: *const inode,
    ) -> *mut file;

    pub fn anon_inode_getfd(
        name: *const c_char,
        fops: *const file_operations,
        priv_: *mut c_void,
        flags: c_int,
    ) -> c_int;

    pub fn anon_inode_create_getfd(
        name: *const c_char,
        fops: *const file_operations,
        priv_: *mut c_void,
        flags: c_int,
        context_inode: *const inode,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
