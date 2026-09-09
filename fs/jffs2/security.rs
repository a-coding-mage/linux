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

use core::ffi::{c_char, c_int, c_void};

/* Declarations supplied by the kernel and by nodelist.h. */
#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mnt_idmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct qstr {
    _private: [u8; 0],
}
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
pub struct xattr {
    pub name: *const c_char,
    pub value: *const c_void,
    pub value_len: usize,
}

pub const JFFS2_XPREFIX_SECURITY: c_int = 3;
pub const XATTR_SECURITY_PREFIX: *const c_char = b"security.\0".as_ptr() as *const c_char;

unsafe extern "C" {
    fn do_jffs2_setxattr(
        inode: *mut inode,
        xprefix: c_int,
        name: *const c_char,
        value: *const c_void,
        value_len: usize,
        flags: c_int,
    ) -> c_int;
    fn do_jffs2_getxattr(
        inode: *mut inode,
        xprefix: c_int,
        name: *const c_char,
        buffer: *mut c_void,
        size: usize,
    ) -> c_int;
    fn security_inode_init_security(
        inode: *mut inode,
        dir: *mut inode,
        qstr: *const qstr,
        initxattrs: Option<unsafe extern "C" fn(*mut inode, *const xattr, *mut c_void) -> c_int>,
        fs_info: *mut c_void,
    ) -> c_int;
}

/* ---- Initial Security Label(s) Attachment callback --- */
unsafe extern "C" fn jffs2_initxattrs(
    inode: *mut inode,
    xattr_array: *const xattr,
    _fs_info: *mut c_void,
) -> c_int {
    let mut xattr = xattr_array;
    let mut err: c_int = 0;

    while !(*xattr).name.is_null() {
        err = do_jffs2_setxattr(
            inode,
            JFFS2_XPREFIX_SECURITY,
            (*xattr).name,
            (*xattr).value,
            (*xattr).value_len,
            0,
        );
        if err < 0 {
            break;
        }
        xattr = xattr.add(1);
    }
    err
}

/* ---- Initial Security Label(s) Attachment ----------- */
pub unsafe extern "C" fn jffs2_init_security(
    inode: *mut inode,
    dir: *mut inode,
    qstr: *const qstr,
) -> c_int {
    security_inode_init_security(inode, dir, qstr, Some(jffs2_initxattrs), core::ptr::null_mut())
}

/* ---- XATTR Handler for "security.*" ----------------- */
unsafe extern "C" fn jffs2_security_getxattr(
    _handler: *const xattr_handler,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const c_char,
    buffer: *mut c_void,
    size: usize,
) -> c_int {
    do_jffs2_getxattr(inode, JFFS2_XPREFIX_SECURITY, name, buffer, size)
}

unsafe extern "C" fn jffs2_security_setxattr(
    _handler: *const xattr_handler,
    _idmap: *mut mnt_idmap,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const c_char,
    buffer: *const c_void,
    size: usize,
    flags: c_int,
) -> c_int {
    do_jffs2_setxattr(inode, JFFS2_XPREFIX_SECURITY, name, buffer, size, flags)
}

pub static jffs2_security_xattr_handler: xattr_handler = xattr_handler {
    prefix: XATTR_SECURITY_PREFIX,
    set: Some(jffs2_security_setxattr),
    get: Some(jffs2_security_getxattr),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
