// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/ext4/xattr_trusted.c
 * Handler for trusted extended attributes.
 *
 * Copyright (C) 2003 by Andreas Gruenbacher, <a.gruenbacher@computer.org>
 */

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::c_void;

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mnt_idmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xattr_handler {
    pub prefix: *const u8,
    pub list: Option<unsafe extern "C" fn(*mut dentry) -> bool>,
    pub get: Option<unsafe extern "C" fn(
        *const xattr_handler,
        *mut dentry,
        *mut inode,
        *const u8,
        *mut c_void,
        usize,
    ) -> i32>,
    pub set: Option<unsafe extern "C" fn(
        *const xattr_handler,
        *mut mnt_idmap,
        *mut dentry,
        *mut inode,
        *const u8,
        *const c_void,
        usize,
        i32,
    ) -> i32>,
}

extern "C" {
    fn capable(cap: i32) -> bool;
    fn ext4_xattr_get(
        inode: *mut inode,
        index: i32,
        name: *const u8,
        buffer: *mut c_void,
        size: usize,
    ) -> i32;
    fn ext4_xattr_set(
        inode: *mut inode,
        index: i32,
        name: *const u8,
        value: *const c_void,
        size: usize,
        flags: i32,
    ) -> i32;
}

const CAP_SYS_ADMIN: i32 = 21;
const EXT4_XATTR_INDEX_TRUSTED: i32 = 4;

// Build-time kernel definitions supplied by the surrounding translation.
extern "C" {
    static XATTR_TRUSTED_PREFIX: u8;
}

unsafe extern "C" fn ext4_xattr_trusted_list(_dentry: *mut dentry) -> bool {
    capable(CAP_SYS_ADMIN)
}

unsafe extern "C" fn ext4_xattr_trusted_get(
    _handler: *const xattr_handler,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const u8,
    buffer: *mut c_void,
    size: usize,
) -> i32 {
    ext4_xattr_get(inode, EXT4_XATTR_INDEX_TRUSTED, name, buffer, size)
}

unsafe extern "C" fn ext4_xattr_trusted_set(
    _handler: *const xattr_handler,
    _idmap: *mut mnt_idmap,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const u8,
    value: *const c_void,
    size: usize,
    flags: i32,
) -> i32 {
    ext4_xattr_set(inode, EXT4_XATTR_INDEX_TRUSTED, name, value, size, flags)
}

#[no_mangle]
pub static ext4_xattr_trusted_handler: xattr_handler = xattr_handler {
    prefix: unsafe { &XATTR_TRUSTED_PREFIX as *const u8 },
    list: Some(ext4_xattr_trusted_list),
    get: Some(ext4_xattr_trusted_get),
    set: Some(ext4_xattr_trusted_set),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
