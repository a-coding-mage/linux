/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2019 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */

/*
 * Return codes for the inode/inobt walk function are 0 to continue iterating,
 * and non-zero to stop iterating.  Any non-zero value will be passed up to the
 * iwalk or inobt_walk caller.  The special value -ECANCELED can be used to
 * stop iteration, as neither iwalk nor inobt_walk will ever generate that
 * error code on their own.
 */

/* Walk all inodes in the filesystem starting from @startino. */
pub type XfsIwalkFn =
    unsafe extern "C" fn(mp: *mut xfs_mount, tp: *mut xfs_trans, ino: xfs_ino_t, data: *mut core::ffi::c_void) -> core::ffi::c_int;

unsafe extern "C" {
    pub fn xfs_iwalk(
        mp: *mut xfs_mount,
        tp: *mut xfs_trans,
        startino: xfs_ino_t,
        flags: core::ffi::c_uint,
        iwalk_fn: XfsIwalkFn,
        inode_records: core::ffi::c_uint,
        data: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;

    pub fn xfs_iwalk_threaded(
        mp: *mut xfs_mount,
        startino: xfs_ino_t,
        flags: core::ffi::c_uint,
        iwalk_fn: XfsIwalkFn,
        inode_records: core::ffi::c_uint,
        poll: bool,
        data: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
}

/* Only iterate within the same AG as @startino. */
pub const XFS_IWALK_SAME_AG: core::ffi::c_uint = 1u32 << 0;

pub const XFS_IWALK_FLAGS_ALL: core::ffi::c_uint = XFS_IWALK_SAME_AG;

/* Walk all inode btree records in the filesystem starting from @startino. */
pub type XfsInobtWalkFn = unsafe extern "C" fn(
    mp: *mut xfs_mount,
    tp: *mut xfs_trans,
    agno: xfs_agnumber_t,
    irec: *const xfs_inobt_rec_incore,
    data: *mut core::ffi::c_void,
) -> core::ffi::c_int;

unsafe extern "C" {
    pub fn xfs_inobt_walk(
        mp: *mut xfs_mount,
        tp: *mut xfs_trans,
        startino: xfs_ino_t,
        flags: core::ffi::c_uint,
        inobt_walk_fn: XfsInobtWalkFn,
        inobt_records: core::ffi::c_uint,
        data: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
