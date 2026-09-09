/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2017-2018 HUAWEI, Inc.
 *             https://www.huawei.com/
 */

// Dependency intent preserved from C:
// #include "internal.h"
// #include <linux/posix_acl_xattr.h>
// #include <linux/xattr.h>

#[cfg(feature = "CONFIG_EROFS_FS_XATTR")]
extern "C" {
    pub static erofs_xattr_handlers: *const *const xattr_handler;

    pub fn erofs_xattr_prefixes_init(sb: *mut super_block) -> ::std::os::raw::c_int;
    pub fn erofs_xattr_prefixes_cleanup(sb: *mut super_block);
    pub fn erofs_listxattr(
        dentry: *mut dentry,
        buffer: *mut ::std::os::raw::c_char,
        size: usize,
    ) -> isize;
}

#[cfg(not(feature = "CONFIG_EROFS_FS_XATTR"))]
#[inline]
pub unsafe fn erofs_xattr_prefixes_init(_sb: *mut super_block) -> ::std::os::raw::c_int {
    0
}

#[cfg(not(feature = "CONFIG_EROFS_FS_XATTR"))]
#[inline]
pub unsafe fn erofs_xattr_prefixes_cleanup(_sb: *mut super_block) {}

#[cfg(not(feature = "CONFIG_EROFS_FS_XATTR"))]
pub const erofs_listxattr: Option<
    unsafe extern "C" fn(
        *mut dentry,
        *mut ::std::os::raw::c_char,
        usize,
    ) -> isize,
> = None;

#[cfg(not(feature = "CONFIG_EROFS_FS_XATTR"))]
pub const erofs_xattr_handlers: Option<*const *const xattr_handler> = None;

#[cfg(feature = "CONFIG_EROFS_FS_POSIX_ACL")]
extern "C" {
    pub fn erofs_get_acl(
        inode: *mut inode,
        type_: ::std::os::raw::c_int,
        rcu: bool,
    ) -> *mut posix_acl;
}

#[cfg(not(feature = "CONFIG_EROFS_FS_POSIX_ACL"))]
pub const erofs_get_acl: Option<
    unsafe extern "C" fn(*mut inode, ::std::os::raw::c_int, bool) -> *mut posix_acl,
> = None;

extern "C" {
    pub fn erofs_xattr_fill_inode_fingerprint(
        fp: *mut erofs_inode_fingerprint,
        inode: *mut inode,
        domain_id: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn erofs_inode_has_noacl(
        inode: *mut inode,
        kaddr: *mut ::std::ffi::c_void,
        ofs: ::std::os::raw::c_uint,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
