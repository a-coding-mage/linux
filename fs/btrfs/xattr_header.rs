/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2007 Red Hat.  All rights reserved.
 */

// Translated from BTRFS_XATTR_H.
// Dependency intent: Linux kernel types and declarations are supplied by other files.

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct Dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Qstr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct XattrHandler {
    _private: [u8; 0],
}

#[repr(C)]
pub struct BtrfsTransHandle {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static btrfs_xattr_handlers: *const *const XattrHandler;

    pub fn btrfs_getxattr(
        inode: *const Inode,
        name: *const c_char,
        buffer: *mut c_void,
        size: usize,
    ) -> i32;

    pub fn btrfs_setxattr(
        trans: *mut BtrfsTransHandle,
        inode: *mut Inode,
        name: *const c_char,
        value: *const c_void,
        size: usize,
        flags: i32,
    ) -> i32;

    pub fn btrfs_setxattr_trans(
        inode: *mut Inode,
        name: *const c_char,
        value: *const c_void,
        size: usize,
        flags: i32,
    ) -> i32;

    pub fn btrfs_listxattr(
        dentry: *mut Dentry,
        buffer: *mut c_char,
        size: usize,
    ) -> isize;

    pub fn btrfs_xattr_security_init(
        trans: *mut BtrfsTransHandle,
        inode: *mut Inode,
        dir: *mut Inode,
        qstr: *const Qstr,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
