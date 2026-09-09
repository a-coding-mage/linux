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

// Dependencies supplied by the surrounding kernel/JFFS2 translation.

unsafe extern "C" {
    fn do_jffs2_getxattr(
        inode: *mut crate::inode,
        prefix: i32,
        name: *const core::ffi::c_char,
        buffer: *mut core::ffi::c_void,
        size: usize,
    ) -> i32;

    fn do_jffs2_setxattr(
        inode: *mut crate::inode,
        prefix: i32,
        name: *const core::ffi::c_char,
        buffer: *const core::ffi::c_void,
        size: usize,
        flags: i32,
    ) -> i32;
}

unsafe fn jffs2_user_getxattr(
    _handler: *const crate::xattr_handler,
    _unused: *mut crate::dentry,
    inode: *mut crate::inode,
    name: *const core::ffi::c_char,
    buffer: *mut core::ffi::c_void,
    size: usize,
) -> i32 {
    do_jffs2_getxattr(inode, crate::JFFS2_XPREFIX_USER, name, buffer, size)
}

unsafe fn jffs2_user_setxattr(
    _handler: *const crate::xattr_handler,
    _idmap: *mut crate::mnt_idmap,
    _unused: *mut crate::dentry,
    inode: *mut crate::inode,
    name: *const core::ffi::c_char,
    buffer: *const core::ffi::c_void,
    size: usize,
    flags: i32,
) -> i32 {
    do_jffs2_setxattr(inode, crate::JFFS2_XPREFIX_USER, name, buffer, size, flags)
}

#[no_mangle]
pub static jffs2_user_xattr_handler: crate::xattr_handler = crate::xattr_handler {
    prefix: crate::XATTR_USER_PREFIX,
    set: Some(jffs2_user_setxattr),
    get: Some(jffs2_user_getxattr),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
