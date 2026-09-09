// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/ext4/xattr_user.c
 * Handler for extended user attributes.
 *
 * Copyright (C) 2001 by Andreas Gruenbacher, <a.gruenbacher@computer.org>
 */

// Dependencies supplied by the surrounding ext4/kernel translation.

unsafe fn ext4_xattr_user_list(dentry: *mut dentry) -> bool {
    unsafe { test_opt((*dentry).d_sb, XATTR_USER) }
}

unsafe fn ext4_xattr_user_get(
    _handler: *const xattr_handler,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const core::ffi::c_char,
    buffer: *mut core::ffi::c_void,
    size: usize,
) -> i32 {
    if !unsafe { test_opt((*inode).i_sb, XATTR_USER) } {
        return -EOPNOTSUPP;
    }
    unsafe {
        ext4_xattr_get(inode, EXT4_XATTR_INDEX_USER, name, buffer, size)
    }
}

unsafe fn ext4_xattr_user_set(
    _handler: *const xattr_handler,
    _idmap: *mut mnt_idmap,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const core::ffi::c_char,
    value: *const core::ffi::c_void,
    size: usize,
    flags: i32,
) -> i32 {
    if !unsafe { test_opt((*inode).i_sb, XATTR_USER) } {
        return -EOPNOTSUPP;
    }
    unsafe {
        ext4_xattr_set(inode, EXT4_XATTR_INDEX_USER, name, value, size, flags)
    }
}

pub static ext4_xattr_user_handler: xattr_handler = xattr_handler {
    prefix: XATTR_USER_PREFIX,
    list: Some(ext4_xattr_user_list),
    get: Some(ext4_xattr_user_get),
    set: Some(ext4_xattr_user_set),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
