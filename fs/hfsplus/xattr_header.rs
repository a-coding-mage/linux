/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/fs/hfsplus/xattr.h
 *
 * Vyacheslav Dubeyko <slava@dubeyko.com>
 *
 * Logic of processing extended attributes
 */

// Dependency provided by <linux/xattr.h>.

extern "C" {
    pub static hfsplus_xattr_osx_handler: xattr_handler;
    pub static hfsplus_xattr_user_handler: xattr_handler;
    pub static hfsplus_xattr_trusted_handler: xattr_handler;
    pub static hfsplus_xattr_security_handler: xattr_handler;

    pub static hfsplus_xattr_handlers: *const *const xattr_handler;

    pub fn __hfsplus_setxattr(
        inode: *mut inode,
        name: *const core::ffi::c_char,
        value: *const core::ffi::c_void,
        size: usize,
        flags: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub fn hfsplus_setxattr(
        inode: *mut inode,
        name: *const core::ffi::c_char,
        value: *const core::ffi::c_void,
        size: usize,
        flags: core::ffi::c_int,
        prefix: *const core::ffi::c_char,
        prefixlen: usize,
    ) -> core::ffi::c_int;

    pub fn __hfsplus_getxattr(
        inode: *mut inode,
        name: *const core::ffi::c_char,
        value: *mut core::ffi::c_void,
        size: usize,
    ) -> isize;

    pub fn hfsplus_getxattr(
        inode: *mut inode,
        name: *const core::ffi::c_char,
        value: *mut core::ffi::c_void,
        size: usize,
        prefix: *const core::ffi::c_char,
        prefixlen: usize,
    ) -> isize;

    pub fn hfsplus_listxattr(
        dentry: *mut dentry,
        buffer: *mut core::ffi::c_char,
        size: usize,
    ) -> isize;

    pub fn hfsplus_init_security(
        inode: *mut inode,
        dir: *mut inode,
        qstr: *const qstr,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
