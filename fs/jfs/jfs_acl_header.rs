/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) International Business Machines  Corp., 2002
 */

/* Translated from the C header's CONFIG_JFS_POSIX_ACL conditional. */

/* External types supplied by the surrounding JFS/kernel translation. */
#[allow(non_camel_case_types)]
pub type tid_t = u32;

#[allow(non_camel_case_types)]
pub enum inode {}
#[allow(non_camel_case_types)]
pub enum mnt_idmap {}
#[allow(non_camel_case_types)]
pub enum dentry {}
#[allow(non_camel_case_types)]
pub enum posix_acl {}

#[cfg(feature = "CONFIG_JFS_POSIX_ACL")]
extern "C" {
    pub fn jfs_get_acl(inode: *mut inode, type_: core::ffi::c_int, rcu: bool)
        -> *mut posix_acl;
    pub fn jfs_set_acl(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        acl: *mut posix_acl,
        type_: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn jfs_init_acl(
        tid: tid_t,
        inode: *mut inode,
        dir: *mut inode,
    ) -> core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_JFS_POSIX_ACL"))]
#[inline]
pub fn jfs_init_acl(_tid: tid_t, _inode: *mut inode, _dir: *mut inode) -> core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
