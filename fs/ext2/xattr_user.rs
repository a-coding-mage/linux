// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/ext2/xattr_user.c
 * Handler for extended user attributes.
 *
 * Copyright (C) 2001 by Andreas Gruenbacher, <a.gruenbacher@computer.org>
 */

// Dependencies supplied by the corresponding ext2 and xattr headers.

use core::ffi::{c_char, c_int, c_void};

unsafe fn ext2_xattr_user_list(dentry: *mut crate::dentry) -> bool {
    unsafe { crate::test_opt((*(*dentry).d_sb), crate::XATTR_USER) }
}

unsafe fn ext2_xattr_user_get(
    _handler: *const crate::xattr_handler,
    _unused: *mut crate::dentry,
    inode: *mut crate::inode,
    name: *const c_char,
    buffer: *mut c_void,
    size: usize,
) -> c_int {
    if !unsafe { crate::test_opt((*(*inode).i_sb), crate::XATTR_USER) } {
        return -crate::EOPNOTSUPP;
    }
    unsafe {
        crate::ext2_xattr_get(
            inode,
            crate::EXT2_XATTR_INDEX_USER,
            name,
            buffer,
            size,
        )
    }
}

unsafe fn ext2_xattr_user_set(
    _handler: *const crate::xattr_handler,
    _idmap: *mut crate::mnt_idmap,
    _unused: *mut crate::dentry,
    inode: *mut crate::inode,
    name: *const c_char,
    value: *const c_void,
    size: usize,
    flags: c_int,
) -> c_int {
    if !unsafe { crate::test_opt((*(*inode).i_sb), crate::XATTR_USER) } {
        return -crate::EOPNOTSUPP;
    }

    unsafe {
        crate::ext2_xattr_set(
            inode,
            crate::EXT2_XATTR_INDEX_USER,
            name,
            value,
            size,
            flags,
        )
    }
}

pub static ext2_xattr_user_handler: crate::xattr_handler = crate::xattr_handler {
    prefix: crate::XATTR_USER_PREFIX,
    list: Some(ext2_xattr_user_list),
    get: Some(ext2_xattr_user_get),
    set: Some(ext2_xattr_user_set),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
