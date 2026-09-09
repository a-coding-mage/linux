/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2020-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Update the mtime/cmtime of file1 and file2 */
pub const __XFS_EXCHANGE_RANGE_UPD_CMTIME1: u64 = 1u64 << 63;
pub const __XFS_EXCHANGE_RANGE_UPD_CMTIME2: u64 = 1u64 << 62;

/* Freshness check required */
pub const __XFS_EXCHANGE_RANGE_CHECK_FRESH2: u64 = 1u64 << 61;

pub const XFS_EXCHANGE_RANGE_PRIV_FLAGS: u64 =
    __XFS_EXCHANGE_RANGE_UPD_CMTIME1
        | __XFS_EXCHANGE_RANGE_UPD_CMTIME2
        | __XFS_EXCHANGE_RANGE_CHECK_FRESH2;

/* External types supplied by other translation units. */
#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timespec64 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_exchange_range {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_commit_range {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_trans {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_exchmaps_req {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_exchrange {
    pub file1: *mut file,
    pub file2: *mut file,

    pub file1_offset: i64,
    pub file2_offset: i64,
    pub length: u64,

    pub flags: u64, /* XFS_EXCHANGE_RANGE flags */

    /* file2 metadata for freshness checks */
    pub file2_ino: u64,
    pub file2_mtime: timespec64,
    pub file2_ctime: timespec64,
    pub file2_gen: u32,
}

extern "C" {
    pub fn xfs_ioc_exchange_range(
        file: *mut file,
        argp: *mut xfs_exchange_range,
    ) -> i64;
    pub fn xfs_ioc_start_commit(
        file: *mut file,
        argp: *mut xfs_commit_range,
    ) -> i64;
    pub fn xfs_ioc_commit_range(
        file: *mut file,
        argp: *mut xfs_commit_range,
    ) -> i64;

    pub fn xfs_exchrange_ilock(
        tp: *mut xfs_trans,
        ip1: *mut xfs_inode,
        ip2: *mut xfs_inode,
    );
    pub fn xfs_exchrange_iunlock(ip1: *mut xfs_inode, ip2: *mut xfs_inode);

    pub fn xfs_exchrange_estimate(req: *mut xfs_exchmaps_req) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
