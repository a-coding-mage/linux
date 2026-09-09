/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2006  NEC Corporation
 *
 * Created by KaiGai Kohei <kaigai@ak.jp.nec.com>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 *
 */

use core::ffi::c_void;

// The following types, constants, and functions are supplied by the kernel
// and the other JFFS2 translation units.
use crate::{capable, do_jffs2_getxattr, do_jffs2_setxattr, xattr_handler};
use crate::{inode, mnt_idmap, dentry, CAP_SYS_ADMIN, JFFS2_XPREFIX_TRUSTED,
            XATTR_TRUSTED_PREFIX};

unsafe extern "C" {
    // External declarations are provided by the surrounding translation unit.
}

unsafe fn jffs2_trusted_getxattr(
    _handler: *const xattr_handler,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const i8,
    buffer: *mut c_void,
    size: usize,
) -> i32 {
    do_jffs2_getxattr(inode, JFFS2_XPREFIX_TRUSTED, name, buffer, size)
}

unsafe fn jffs2_trusted_setxattr(
    _handler: *const xattr_handler,
    _idmap: *mut mnt_idmap,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const i8,
    buffer: *const c_void,
    size: usize,
    flags: i32,
) -> i32 {
    do_jffs2_setxattr(inode, JFFS2_XPREFIX_TRUSTED, name, buffer, size, flags)
}

unsafe fn jffs2_trusted_listxattr(_dentry: *mut dentry) -> bool {
    capable(CAP_SYS_ADMIN)
}

pub static jffs2_trusted_xattr_handler: xattr_handler = xattr_handler {
    prefix: XATTR_TRUSTED_PREFIX,
    list: Some(jffs2_trusted_listxattr),
    set: Some(jffs2_trusted_setxattr),
    get: Some(jffs2_trusted_getxattr),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
