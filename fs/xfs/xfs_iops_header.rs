// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2003,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// C header guard: __XFS_IOPS_H__

// External kernel types supplied by other translated units.
#[repr(C)]
pub struct xfs_inode {
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
pub struct iattr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct qstr {
    _private: [u8; 0],
}

extern "C" {
    pub fn xfs_vn_listxattr(
        dentry: *mut dentry,
        data: *mut ::core::ffi::c_char,
        size: usize,
    ) -> isize;

    pub fn xfs_vn_setattr_size(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        vap: *mut iattr,
    ) -> ::core::ffi::c_int;

    pub fn xfs_inode_init_security(
        inode: *mut inode,
        dir: *mut inode,
        qstr: *const qstr,
    ) -> ::core::ffi::c_int;

    pub fn xfs_setup_inode(ip: *mut xfs_inode);
    pub fn xfs_setup_iops(ip: *mut xfs_inode);
    pub fn xfs_diflags_to_iflags(ip: *mut xfs_inode, init: bool);
    pub fn xfs_get_atomic_write_min(ip: *mut xfs_inode) -> u32;
    pub fn xfs_get_atomic_write_max(ip: *mut xfs_inode) -> u32;
    pub fn xfs_get_atomic_write_max_opt(ip: *mut xfs_inode) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
