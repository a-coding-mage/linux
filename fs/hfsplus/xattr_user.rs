// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/hfsplus/xattr_user.c
 *
 * Vyacheslav Dubeyko <slava@dubeyko.com>
 *
 * Handler for user extended attributes.
 */

use core::ffi::{c_char, c_int, c_void};

// These types and functions are supplied by the HFS+ and kernel dependencies.
#[repr(C)]
pub struct xattr_handler {
    pub prefix: *const c_char,
    pub get: Option<unsafe extern "C" fn(
        handler: *const xattr_handler,
        unused: *mut dentry,
        inode: *mut inode,
        name: *const c_char,
        buffer: *mut c_void,
        size: usize,
    ) -> c_int>,
    pub set: Option<unsafe extern "C" fn(
        handler: *const xattr_handler,
        idmap: *mut mnt_idmap,
        unused: *mut dentry,
        inode: *mut inode,
        name: *const c_char,
        buffer: *const c_void,
        size: usize,
        flags: c_int,
    ) -> c_int>,
}

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

pub const XATTR_USER_PREFIX: &[u8] = b"user.\0";
pub const XATTR_USER_PREFIX_LEN: usize = 5;

unsafe extern "C" {
    fn hfsplus_getxattr(
        inode: *mut inode,
        name: *const c_char,
        buffer: *mut c_void,
        size: usize,
        prefix: *const c_char,
        prefix_len: usize,
    ) -> c_int;

    fn hfsplus_setxattr(
        inode: *mut inode,
        name: *const c_char,
        buffer: *const c_void,
        size: usize,
        flags: c_int,
        prefix: *const c_char,
        prefix_len: usize,
    ) -> c_int;
}

unsafe extern "C" fn hfsplus_user_getxattr(
    _handler: *const xattr_handler,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const c_char,
    buffer: *mut c_void,
    size: usize,
) -> c_int {
    hfsplus_getxattr(
        inode,
        name,
        buffer,
        size,
        XATTR_USER_PREFIX.as_ptr() as *const c_char,
        XATTR_USER_PREFIX_LEN,
    )
}

unsafe extern "C" fn hfsplus_user_setxattr(
    _handler: *const xattr_handler,
    _idmap: *mut mnt_idmap,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const c_char,
    buffer: *const c_void,
    size: usize,
    flags: c_int,
) -> c_int {
    hfsplus_setxattr(
        inode,
        name,
        buffer,
        size,
        flags,
        XATTR_USER_PREFIX.as_ptr() as *const c_char,
        XATTR_USER_PREFIX_LEN,
    )
}

#[no_mangle]
pub static hfsplus_xattr_user_handler: xattr_handler = xattr_handler {
    prefix: XATTR_USER_PREFIX.as_ptr() as *const c_char,
    get: Some(hfsplus_user_getxattr),
    set: Some(hfsplus_user_setxattr),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
