// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/ext2/xattr_security.c
 * Handler for storing security labels as extended attributes.
 */

// Dependencies supplied by ext2 and the Linux security/xattr interfaces are
// intentionally referenced here rather than implemented in this translation.

use core::ffi::{c_char, c_int, c_void};

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
        value: *const c_void,
        size: usize,
        flags: c_int,
    ) -> c_int>,
}

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
pub struct xattr {
    pub name: *const c_char,
    pub value: *const c_void,
    pub value_len: usize,
}

pub type XattrInitFn = unsafe extern "C" fn(*mut inode, *const xattr, *mut c_void) -> c_int;

unsafe extern "C" {
    pub static XATTR_SECURITY_PREFIX: *const c_char;
    pub static EXT2_XATTR_INDEX_SECURITY: c_int;

    fn ext2_xattr_get(
        inode: *mut inode,
        index: c_int,
        name: *const c_char,
        buffer: *mut c_void,
        size: usize,
    ) -> c_int;
    fn ext2_xattr_set(
        inode: *mut inode,
        index: c_int,
        name: *const c_char,
        value: *const c_void,
        size: usize,
        flags: c_int,
    ) -> c_int;
    fn security_inode_init_security(
        inode: *mut inode,
        dir: *mut inode,
        qstr: *const qstr,
        initxattrs: Option<XattrInitFn>,
        fs_info: *mut c_void,
    ) -> c_int;
}

unsafe extern "C" fn ext2_xattr_security_get(
    _handler: *const xattr_handler,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const c_char,
    buffer: *mut c_void,
    size: usize,
) -> c_int {
    ext2_xattr_get(inode, EXT2_XATTR_INDEX_SECURITY, name, buffer, size)
}

unsafe extern "C" fn ext2_xattr_security_set(
    _handler: *const xattr_handler,
    _idmap: *mut mnt_idmap,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const c_char,
    value: *const c_void,
    size: usize,
    flags: c_int,
) -> c_int {
    ext2_xattr_set(inode, EXT2_XATTR_INDEX_SECURITY, name, value, size, flags)
}

unsafe extern "C" fn ext2_initxattrs(
    inode: *mut inode,
    xattr_array: *const xattr,
    _fs_info: *mut c_void,
) -> c_int {
    let mut xattr = xattr_array;
    let mut err: c_int = 0;

    while !(*xattr).name.is_null() {
        err = ext2_xattr_set(
            inode,
            EXT2_XATTR_INDEX_SECURITY,
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

pub unsafe extern "C" fn ext2_init_security(
    inode: *mut inode,
    dir: *mut inode,
    qstr: *const qstr,
) -> c_int {
    security_inode_init_security(inode, dir, qstr, Some(ext2_initxattrs), core::ptr::null_mut())
}

#[no_mangle]
pub static ext2_xattr_security_handler: xattr_handler = xattr_handler {
    prefix: unsafe { XATTR_SECURITY_PREFIX },
    get: Some(ext2_xattr_security_get),
    set: Some(ext2_xattr_security_set),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
