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
        name: *const kernel::ffi::c_char,
        value: *const kernel::ffi::c_void,
        size: usize,
        flags: kernel::ffi::c_int,
    ) -> kernel::ffi::c_int;

    pub fn hfsplus_setxattr(
        inode: *mut inode,
        name: *const kernel::ffi::c_char,
        value: *const kernel::ffi::c_void,
        size: usize,
        flags: kernel::ffi::c_int,
        prefix: *const kernel::ffi::c_char,
        prefixlen: usize,
    ) -> kernel::ffi::c_int;

    pub fn __hfsplus_getxattr(
        inode: *mut inode,
        name: *const kernel::ffi::c_char,
        value: *mut kernel::ffi::c_void,
        size: usize,
    ) -> isize;

    pub fn hfsplus_getxattr(
        inode: *mut inode,
        name: *const kernel::ffi::c_char,
        value: *mut kernel::ffi::c_void,
        size: usize,
        prefix: *const kernel::ffi::c_char,
        prefixlen: usize,
    ) -> isize;

    pub fn hfsplus_listxattr(
        dentry: *mut dentry,
        buffer: *mut kernel::ffi::c_char,
        size: usize,
    ) -> isize;

    pub fn hfsplus_init_security(
        inode: *mut inode,
        dir: *mut inode,
        qstr: *const qstr,
    ) -> kernel::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
