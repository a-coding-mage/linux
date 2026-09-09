// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/ext2/xattr_trusted.c
 * Handler for trusted extended attributes.
 *
 * Copyright (C) 2003 by Andreas Gruenbacher, <a.gruenbacher@computer.org>
 */

// Dependencies supplied by ext2.h and xattr.h.

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
    pub prefix: *const core::ffi::c_char,
    pub list: Option<unsafe extern "C" fn(*mut dentry) -> bool>,
    pub get: Option<
        unsafe extern "C" fn(
            *const xattr_handler,
            *mut dentry,
            *mut inode,
            *const core::ffi::c_char,
            *mut core::ffi::c_void,
            usize,
        ) -> i32,
    >,
    pub set: Option<
        unsafe extern "C" fn(
            *const xattr_handler,
            *mut mnt_idmap,
            *mut dentry,
            *mut inode,
            *const core::ffi::c_char,
            *const core::ffi::c_void,
            usize,
            i32,
        ) -> i32,
    >,
}

extern "C" {
    fn capable(cap: i32) -> bool;
    fn ext2_xattr_get(
        inode: *mut inode,
        index: i32,
        name: *const core::ffi::c_char,
        buffer: *mut core::ffi::c_void,
        size: usize,
    ) -> i32;
    fn ext2_xattr_set(
        inode: *mut inode,
        index: i32,
        name: *const core::ffi::c_char,
        value: *const core::ffi::c_void,
        size: usize,
        flags: i32,
    ) -> i32;
}

// Supplied by the kernel capability and ext2 xattr headers.
const CAP_SYS_ADMIN: i32 = 21;
const EXT2_XATTR_INDEX_TRUSTED: i32 = 4;
extern "C" {
    static XATTR_TRUSTED_PREFIX: *const core::ffi::c_char;
}

unsafe extern "C" fn ext2_xattr_trusted_list(_dentry: *mut dentry) -> bool {
    capable(CAP_SYS_ADMIN)
}

unsafe extern "C" fn ext2_xattr_trusted_get(
    _handler: *const xattr_handler,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const core::ffi::c_char,
    buffer: *mut core::ffi::c_void,
    size: usize,
) -> i32 {
    ext2_xattr_get(inode, EXT2_XATTR_INDEX_TRUSTED, name, buffer, size)
}

unsafe extern "C" fn ext2_xattr_trusted_set(
    _handler: *const xattr_handler,
    _idmap: *mut mnt_idmap,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const core::ffi::c_char,
    value: *const core::ffi::c_void,
    size: usize,
    flags: i32,
) -> i32 {
    ext2_xattr_set(inode, EXT2_XATTR_INDEX_TRUSTED, name, value, size, flags)
}

pub static ext2_xattr_trusted_handler: xattr_handler = xattr_handler {
    prefix: unsafe { XATTR_TRUSTED_PREFIX },
    list: Some(ext2_xattr_trusted_list),
    get: Some(ext2_xattr_trusted_get),
    set: Some(ext2_xattr_trusted_set),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
