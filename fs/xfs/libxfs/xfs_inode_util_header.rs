/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2000-2003,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

pub struct xfs_icluster;

pub unsafe extern "C" {
    pub fn xfs_flags2diflags(ip: *mut xfs_inode, xflags: core::ffi::c_uint) -> u16;
    pub fn xfs_flags2diflags2(ip: *mut xfs_inode, xflags: core::ffi::c_uint) -> u64;
    pub fn xfs_dic2xflags(ip: *mut xfs_inode) -> u32;
    pub fn xfs_ip2xflags(ip: *mut xfs_inode) -> u32;

    pub fn xfs_get_initial_prid(dp: *mut xfs_inode) -> prid_t;
}

/*
 * File creation context.
 *
 * Due to our only partial reliance on the VFS to propagate uid and gid values
 * according to accepted Unix behaviors, callers must initialize idmap to the
 * correct idmapping structure to get the correct inheritance behaviors when
 * XFS_MOUNT_GRPID is set.
 *
 * To create files detached from the directory tree (e.g. quota inodes), set
 * idmap to NULL.  To create a tree root, set pip to NULL.
 */
#[repr(C)]
pub struct xfs_icreate_args {
    pub idmap: *mut mnt_idmap,
    pub pip: *mut xfs_inode, /* parent inode or null */
    pub rdev: dev_t,
    pub mode: umode_t,
    pub flags: u16,
}

pub const XFS_ICREATE_TMPFILE: core::ffi::c_uint = 1u32 << 0; /* create an unlinked file */
pub const XFS_ICREATE_INIT_XATTRS: core::ffi::c_uint = 1u32 << 1; /* will set xattrs immediately */
pub const XFS_ICREATE_UNLINKABLE: core::ffi::c_uint = 1u32 << 2; /* cannot link into dir tree */

/*
 * Flags for xfs_trans_ichgtime().
 */
pub const XFS_ICHGTIME_MOD: core::ffi::c_int = 0x1; /* data fork modification timestamp */
pub const XFS_ICHGTIME_CHG: core::ffi::c_int = 0x2; /* inode field change timestamp */
pub const XFS_ICHGTIME_CREATE: core::ffi::c_int = 0x4; /* inode create timestamp */
pub const XFS_ICHGTIME_ACCESS: core::ffi::c_int = 0x8; /* last access timestamp */

pub unsafe extern "C" {
    pub fn xfs_trans_ichgtime(tp: *mut xfs_trans, ip: *mut xfs_inode, flags: core::ffi::c_int);

    pub fn xfs_inode_init(
        tp: *mut xfs_trans,
        args: *const xfs_icreate_args,
        ip: *mut xfs_inode,
    );

    pub fn xfs_inode_uninit(
        tp: *mut xfs_trans,
        pag: *mut xfs_perag,
        ip: *mut xfs_inode,
        xic: *mut xfs_icluster,
    ) -> core::ffi::c_int;

    pub fn xfs_iunlink(tp: *mut xfs_trans, ip: *mut xfs_inode) -> core::ffi::c_int;
    pub fn xfs_iunlink_remove(
        tp: *mut xfs_trans,
        pag: *mut xfs_perag,
        ip: *mut xfs_inode,
    ) -> core::ffi::c_int;
    pub fn xfs_droplink(tp: *mut xfs_trans, ip: *mut xfs_inode) -> core::ffi::c_int;
    pub fn xfs_bumplink(tp: *mut xfs_trans, ip: *mut xfs_inode);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
