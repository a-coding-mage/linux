// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/hfsplus/xattr_trusted.c
 *
 * Vyacheslav Dubeyko <slava@dubeyko.com>
 *
 * Handler for storing security labels as extended attributes.
 */

use core::ffi::{c_char, c_int, c_void};

// Dependencies supplied by the Linux security, NLS, HFS+, and xattr code.
extern "C" {
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
    fn __hfsplus_setxattr(
        inode: *mut inode,
        name: *const c_char,
        value: *const c_void,
        value_len: usize,
        flags: c_int,
    ) -> c_int;
    fn security_inode_init_security(
        inode: *mut inode,
        dir: *mut inode,
        qstr: *const qstr,
        initxattrs: unsafe extern "C" fn(*mut inode, *const xattr, *mut c_void) -> c_int,
        fs_data: *mut c_void,
    ) -> c_int;
    fn kmalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn strcmp(lhs: *const c_char, rhs: *const c_char) -> c_int;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn memset(dst: *mut c_void, value: c_int, count: usize) -> *mut c_void;
    fn strlen(value: *const c_char) -> usize;
}

// Types and constants are supplied by the included HFS+ and kernel headers.
#[repr(C)]
pub struct xattr_handler {
    pub prefix: *const c_char,
    pub get: Option<unsafe extern "C" fn(
        *const xattr_handler,
        *mut dentry,
        *mut inode,
        *const c_char,
        *mut c_void,
        usize,
    ) -> c_int>,
    pub set: Option<unsafe extern "C" fn(
        *const xattr_handler,
        *mut mnt_idmap,
        *mut dentry,
        *mut inode,
        *const c_char,
        *const c_void,
        usize,
        c_int,
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

#[repr(C)]
pub struct qstr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xattr {
    pub name: *const c_char,
    pub value: *const c_void,
    pub value_len: usize,
}

extern "C" {
    static XATTR_SECURITY_PREFIX: *const c_char;
    static XATTR_SECURITY_PREFIX_LEN: usize;
    static NLS_MAX_CHARSET_SIZE: usize;
    static HFSPLUS_ATTR_MAX_STRLEN: usize;
    static GFP_KERNEL: c_int;
}

unsafe extern "C" fn hfsplus_security_getxattr(
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
        XATTR_SECURITY_PREFIX,
        XATTR_SECURITY_PREFIX_LEN,
    )
}

unsafe extern "C" fn hfsplus_security_setxattr(
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
        XATTR_SECURITY_PREFIX,
        XATTR_SECURITY_PREFIX_LEN,
    )
}

unsafe extern "C" fn hfsplus_initxattrs(
    inode: *mut inode,
    xattr_array: *const xattr,
    _fs_info: *mut c_void,
) -> c_int {
    let mut xattr_name: *mut c_char = kmalloc(
        NLS_MAX_CHARSET_SIZE * HFSPLUS_ATTR_MAX_STRLEN + 1,
        GFP_KERNEL,
    ) as *mut c_char;
    if xattr_name.is_null() {
        return -12; // -ENOMEM
    }

    let mut xattr = xattr_array;
    let mut err: c_int = 0;
    while !(*xattr).name.is_null() {
        if strcmp((*xattr).name, b"\0".as_ptr() as *const c_char) == 0 {
            xattr = xattr.add(1);
            continue;
        }

        strcpy(xattr_name, XATTR_SECURITY_PREFIX);
        strcpy(
            xattr_name.add(XATTR_SECURITY_PREFIX_LEN),
            (*xattr).name,
        );
        memset(
            xattr_name.add(XATTR_SECURITY_PREFIX_LEN + strlen((*xattr).name)) as *mut c_void,
            0,
            1,
        );

        err = __hfsplus_setxattr(
            inode,
            xattr_name,
            (*xattr).value,
            (*xattr).value_len,
            0,
        );
        if err != 0 {
            break;
        }
        xattr = xattr.add(1);
    }
    kfree(xattr_name as *mut c_void);
    err
}

#[no_mangle]
pub unsafe extern "C" fn hfsplus_init_security(
    inode: *mut inode,
    dir: *mut inode,
    qstr: *const qstr,
) -> c_int {
    security_inode_init_security(inode, dir, qstr, hfsplus_initxattrs, core::ptr::null_mut())
}

#[no_mangle]
pub static hfsplus_xattr_security_handler: xattr_handler = xattr_handler {
    prefix: XATTR_SECURITY_PREFIX,
    get: Some(hfsplus_security_getxattr),
    set: Some(hfsplus_security_setxattr),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
