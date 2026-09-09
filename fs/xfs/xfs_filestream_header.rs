// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2006-2007 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Forward declarations from the surrounding XFS implementation.
#[repr(C)]
pub struct xfs_mount {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_bmalloca {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_alloc_arg {
    _private: [u8; 0],
}

pub type xfs_extlen_t = u32;

#[repr(C)]
pub struct xfs_inode {
    pub i_mount: *mut xfs_mount,
    pub i_diflags: u64,
}

unsafe extern "C" {
    pub fn xfs_filestream_mount(mp: *mut xfs_mount) -> i32;
    pub fn xfs_filestream_unmount(mp: *mut xfs_mount);
    pub fn xfs_filestream_deassociate(ip: *mut xfs_inode);
    pub fn xfs_filestream_select_ag(
        ap: *mut xfs_bmalloca,
        args: *mut xfs_alloc_arg,
        blen: *mut xfs_extlen_t,
    ) -> i32;

    pub fn xfs_has_filestreams(mp: *mut xfs_mount) -> bool;
}

pub const XFS_DIFLAG_FILESTREAM: u64 = 0x0000_4000;

#[inline]
pub unsafe fn xfs_inode_is_filestream(ip: *mut xfs_inode) -> bool {
    // C: xfs_has_filestreams(ip->i_mount) ||
    //    (ip->i_diflags & XFS_DIFLAG_FILESTREAM)
    xfs_has_filestreams((*ip).i_mount)
        || ((*ip).i_diflags & XFS_DIFLAG_FILESTREAM) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
