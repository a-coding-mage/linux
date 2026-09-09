// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/ext4/xattr_hurd.c
 * Handler for extended gnu attributes for the Hurd.
 *
 * Copyright (C) 2001 by Andreas Gruenbacher, <a.gruenbacher@computer.org>
 * Copyright (C) 2020 by Jan (janneke) Nieuwenhuizen, <janneke@gnu.org>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/init.h, linux/string.h, ext4.h, and xattr.h

static unsafe fn ext4_xattr_hurd_list(dentry: *mut dentry) -> bool {
    test_opt((*dentry).d_sb, XATTR_USER)
}

static unsafe fn ext4_xattr_hurd_get(
    _handler: *const xattr_handler,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const core::ffi::c_char,
    buffer: *mut core::ffi::c_void,
    size: usize,
) -> i32 {
    if !test_opt((*inode).i_sb, XATTR_USER) {
        return -EOPNOTSUPP;
    }

    ext4_xattr_get(inode, EXT4_XATTR_INDEX_HURD, name, buffer, size)
}

static unsafe fn ext4_xattr_hurd_set(
    _handler: *const xattr_handler,
    _idmap: *mut mnt_idmap,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const core::ffi::c_char,
    value: *const core::ffi::c_void,
    size: usize,
    flags: i32,
) -> i32 {
    if !test_opt((*inode).i_sb, XATTR_USER) {
        return -EOPNOTSUPP;
    }

    ext4_xattr_set(inode, EXT4_XATTR_INDEX_HURD, name, value, size, flags)
}

const ext4_xattr_hurd_handler: xattr_handler = xattr_handler {
    prefix: XATTR_HURD_PREFIX,
    list: Some(ext4_xattr_hurd_list),
    get: Some(ext4_xattr_hurd_get),
    set: Some(ext4_xattr_hurd_set),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
