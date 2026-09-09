/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2018-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

use core::ffi::c_char;

extern "C" {
    pub fn xfs_metafile_type_str(
        metatype: enum_xfs_metafile_type,
    ) -> *const c_char;
}

/* All metadata files must have these flags set. */
pub const XFS_METAFILE_DIFLAGS: u32 = XFS_DIFLAG_IMMUTABLE
    | XFS_DIFLAG_SYNC
    | XFS_DIFLAG_NOATIME
    | XFS_DIFLAG_NODUMP
    | XFS_DIFLAG_NODEFRAG;

/* All metadata directories must have these flags set. */
pub const XFS_METADIR_DIFLAGS: u32 = XFS_METAFILE_DIFLAGS | XFS_DIFLAG_NOSYMLINKS;

extern "C" {
    pub fn xfs_metafile_set_iflag(
        tp: *mut xfs_trans,
        ip: *mut xfs_inode,
        metafile_type: enum_xfs_metafile_type,
    );
    pub fn xfs_metafile_clear_iflag(tp: *mut xfs_trans, ip: *mut xfs_inode);
}

/* Space reservations for metadata inodes. */
#[repr(C)]
pub struct xfs_alloc_arg {
    _private: [u8; 0],
}

extern "C" {
    pub fn xfs_metafile_resv_critical(mp: *mut xfs_mount) -> bool;
    pub fn xfs_metafile_resv_alloc_space(ip: *mut xfs_inode, args: *mut xfs_alloc_arg);
    pub fn xfs_metafile_resv_free_space(
        ip: *mut xfs_inode,
        tp: *mut xfs_trans,
        len: xfs_filblks_t,
    );
    pub fn xfs_metafile_resv_free(mp: *mut xfs_mount);
    pub fn xfs_metafile_resv_init(mp: *mut xfs_mount) -> i32;
}

/* Code specific to kernel/userspace; must be provided externally. */

extern "C" {
    pub fn xfs_trans_metafile_iget(
        tp: *mut xfs_trans,
        ino: xfs_ino_t,
        metafile_type: enum_xfs_metafile_type,
        ipp: *mut *mut xfs_inode,
    ) -> i32;
    pub fn xfs_metafile_iget(
        mp: *mut xfs_mount,
        ino: xfs_ino_t,
        metafile_type: enum_xfs_metafile_type,
        ipp: *mut *mut xfs_inode,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
