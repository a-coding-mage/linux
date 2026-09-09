// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/hfsplus/xattr_trusted.c
 *
 * Vyacheslav Dubeyko <slava@dubeyko.com>
 *
 * Handler for trusted extended attributes.
 */

// Dependencies supplied by linux/nls.h, hfsplus_fs.h, and xattr.h.

extern "C" {
    fn hfsplus_getxattr(
        inode: *mut inode,
        name: *const core::ffi::c_char,
        buffer: *mut core::ffi::c_void,
        size: usize,
        prefix: *const core::ffi::c_char,
        prefix_len: usize,
    ) -> core::ffi::c_int;
    fn hfsplus_setxattr(
        inode: *mut inode,
        name: *const core::ffi::c_char,
        buffer: *const core::ffi::c_void,
        size: usize,
        flags: core::ffi::c_int,
        prefix: *const core::ffi::c_char,
        prefix_len: usize,
    ) -> core::ffi::c_int;
}

unsafe extern "C" fn hfsplus_trusted_getxattr(
    _handler: *const xattr_handler,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const core::ffi::c_char,
    buffer: *mut core::ffi::c_void,
    size: usize,
) -> core::ffi::c_int {
    hfsplus_getxattr(
        inode,
        name,
        buffer,
        size,
        XATTR_TRUSTED_PREFIX,
        XATTR_TRUSTED_PREFIX_LEN,
    )
}

unsafe extern "C" fn hfsplus_trusted_setxattr(
    _handler: *const xattr_handler,
    _idmap: *mut mnt_idmap,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const core::ffi::c_char,
    buffer: *const core::ffi::c_void,
    size: usize,
    flags: core::ffi::c_int,
) -> core::ffi::c_int {
    hfsplus_setxattr(
        inode,
        name,
        buffer,
        size,
        flags,
        XATTR_TRUSTED_PREFIX,
        XATTR_TRUSTED_PREFIX_LEN,
    )
}

pub static hfsplus_xattr_trusted_handler: xattr_handler = xattr_handler {
    prefix: XATTR_TRUSTED_PREFIX,
    get: Some(hfsplus_trusted_getxattr),
    set: Some(hfsplus_trusted_setxattr),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
