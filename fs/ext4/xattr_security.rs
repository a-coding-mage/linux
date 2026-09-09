// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/ext4/xattr_security.c
 * Handler for storing security labels as extended attributes.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/string.h, linux/fs.h, linux/security.h, linux/slab.h,
// ext4_jbd2.h, ext4.h, and xattr.h.

use core::ffi::{c_char, c_int, c_void};

// Opaque types and externally supplied symbols are declared by the surrounding
// translation unit.
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
pub struct dentry;
#[repr(C)]
pub struct inode;
#[repr(C)]
pub struct mnt_idmap;
#[repr(C)]
pub struct qstr;
#[repr(C)]
pub struct handle_t;

#[repr(C)]
pub struct xattr {
    pub name: *const c_char,
    pub value: *const c_void,
    pub value_len: usize,
}

extern "C" {
    fn ext4_xattr_get(
        inode: *mut inode,
        name_index: c_int,
        name: *const c_char,
        buffer: *mut c_void,
        size: usize,
    ) -> c_int;
    fn ext4_xattr_set(
        inode: *mut inode,
        name_index: c_int,
        name: *const c_char,
        value: *const c_void,
        size: usize,
        flags: c_int,
    ) -> c_int;
    fn ext4_xattr_set_handle(
        handle: *mut handle_t,
        inode: *mut inode,
        name_index: c_int,
        name: *const c_char,
        value: *const c_void,
        value_len: usize,
        flags: c_int,
    ) -> c_int;
    fn security_inode_init_security(
        inode: *mut inode,
        dir: *mut inode,
        qstr: *const qstr,
        initxattrs: Option<unsafe extern "C" fn(*mut inode, *const xattr, *mut c_void) -> c_int>,
        fs_info: *mut c_void,
    ) -> c_int;
}

extern "C" {
    static XATTR_SECURITY_PREFIX: *const c_char;
}

const EXT4_XATTR_INDEX_SECURITY: c_int = 6;
const XATTR_CREATE: c_int = 1;

unsafe extern "C" fn ext4_xattr_security_get(
    _handler: *const xattr_handler,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const c_char,
    buffer: *mut c_void,
    size: usize,
) -> c_int {
    ext4_xattr_get(inode, EXT4_XATTR_INDEX_SECURITY, name, buffer, size)
}

unsafe extern "C" fn ext4_xattr_security_set(
    _handler: *const xattr_handler,
    _idmap: *mut mnt_idmap,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const c_char,
    value: *const c_void,
    size: usize,
    flags: c_int,
) -> c_int {
    ext4_xattr_set(inode, EXT4_XATTR_INDEX_SECURITY, name, value, size, flags)
}

unsafe extern "C" fn ext4_initxattrs(
    inode: *mut inode,
    xattr_array: *const xattr,
    fs_info: *mut c_void,
) -> c_int {
    let handle = fs_info as *mut handle_t;
    let mut err: c_int = 0;
    let mut xattr = xattr_array;

    while !(*xattr).name.is_null() {
        err = ext4_xattr_set_handle(
            handle,
            inode,
            EXT4_XATTR_INDEX_SECURITY,
            (*xattr).name,
            (*xattr).value,
            (*xattr).value_len,
            XATTR_CREATE,
        );
        if err < 0 {
            break;
        }
        xattr = xattr.add(1);
    }
    err
}

pub unsafe extern "C" fn ext4_init_security(
    handle: *mut handle_t,
    inode: *mut inode,
    dir: *mut inode,
    qstr: *const qstr,
) -> c_int {
    security_inode_init_security(
        inode,
        dir,
        qstr,
        Some(ext4_initxattrs),
        handle as *mut c_void,
    )
}

#[no_mangle]
pub static ext4_xattr_security_handler: xattr_handler = xattr_handler {
    prefix: unsafe { XATTR_SECURITY_PREFIX },
    get: Some(ext4_xattr_security_get),
    set: Some(ext4_xattr_security_set),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
