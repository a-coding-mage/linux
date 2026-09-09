// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2001-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// C forward declarations.
#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct posix_acl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mnt_idmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

// CONFIG_XFS_POSIX_ACL is a build-time condition from the C header.
#[cfg(feature = "CONFIG_XFS_POSIX_ACL")]
extern "C" {
    pub fn xfs_get_acl(inode: *mut inode, type_: i32, rcu: bool) -> *mut posix_acl;
    pub fn xfs_set_acl(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        acl: *mut posix_acl,
        type_: i32,
    ) -> i32;
    pub fn __xfs_set_acl(inode: *mut inode, acl: *mut posix_acl, type_: i32) -> i32;
    pub fn xfs_forget_acl(inode: *mut inode, name: *const core::ffi::c_char);
}

#[cfg(not(feature = "CONFIG_XFS_POSIX_ACL"))]
pub const xfs_get_acl: Option<unsafe extern "C" fn(*mut inode, i32, bool) -> *mut posix_acl> =
    None;

#[cfg(not(feature = "CONFIG_XFS_POSIX_ACL"))]
pub const xfs_set_acl: Option<
    unsafe extern "C" fn(*mut mnt_idmap, *mut dentry, *mut posix_acl, i32) -> i32,
> = None;

#[cfg(not(feature = "CONFIG_XFS_POSIX_ACL"))]
#[inline]
pub unsafe fn __xfs_set_acl(_inode: *mut inode, _acl: *mut posix_acl, _type_: i32) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_XFS_POSIX_ACL"))]
#[inline]
pub unsafe fn xfs_forget_acl(_inode: *mut inode, _name: *const core::ffi::c_char) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
