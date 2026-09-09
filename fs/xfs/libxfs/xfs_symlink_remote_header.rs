// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * Copyright (c) 2013 Red Hat, Inc.
 * All Rights Reserved.
 */

// C header guard: __XFS_SYMLINK_REMOTE_H

/*
 * Symlink decoding/encoding functions
 */
extern "C" {
    pub fn xfs_symlink_blocks(mp: *mut xfs_mount, pathlen: i32) -> i32;
    pub fn xfs_symlink_hdr_set(
        mp: *mut xfs_mount,
        ino: xfs_ino_t,
        offset: u32,
        size: u32,
        bp: *mut xfs_buf,
    ) -> i32;
    pub fn xfs_symlink_hdr_ok(
        ino: xfs_ino_t,
        offset: u32,
        size: u32,
        bp: *mut xfs_buf,
    ) -> bool;
    pub fn xfs_symlink_local_to_remote(
        tp: *mut xfs_trans,
        bp: *mut xfs_buf,
        ip: *mut xfs_inode,
        ifp: *mut xfs_ifork,
        priv_: *mut core::ffi::c_void,
    );
    pub fn xfs_symlink_shortform_verify(sfp: *mut core::ffi::c_void, size: i64) -> xfs_failaddr_t;
    pub fn xfs_symlink_remote_read(ip: *mut xfs_inode, link: *mut core::ffi::c_char) -> i32;
    pub fn xfs_symlink_write_target(
        tp: *mut xfs_trans,
        ip: *mut xfs_inode,
        owner: xfs_ino_t,
        target_path: *const core::ffi::c_char,
        pathlen: i32,
        fs_blocks: xfs_fsblock_t,
        resblks: u32,
    ) -> i32;
    pub fn xfs_symlink_remote_truncate(tp: *mut xfs_trans, ip: *mut xfs_inode) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
