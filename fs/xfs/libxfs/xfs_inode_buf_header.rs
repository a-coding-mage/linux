// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2003,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// C header guard: __XFS_INODE_BUF_H__

pub struct xfs_inode;
pub struct xfs_dinode;
pub struct xfs_perag;
pub struct xfs_trans;
pub struct xfs_buf;
pub struct xfs_mount;

/*
 * Inode location information.  Stored in the inode and passed to
 * xfs_read_icluster() to get a buffer and dinode for a given inode.
 */
#[repr(C, packed)]
pub struct xfs_imap {
    pub im_agbno: xfs_agblock_t, /* starting agbno of inode cluster */
    pub im_boffset: u16,         /* offset in inode cluster in bytes */
}

extern "C" {
    pub fn xfs_read_icluster(
        pag: *mut xfs_perag,
        tp: *mut xfs_trans,
        agbno: xfs_agblock_t,
        bpp: *mut *mut xfs_buf,
    ) -> ::core::ffi::c_int;
    pub fn xfs_dinode_calc_crc(mp: *mut xfs_mount, dip: *mut xfs_dinode);
    pub fn xfs_inode_to_disk(ip: *mut xfs_inode, to: *mut xfs_dinode, lsn: xfs_lsn_t);
    pub fn xfs_inode_from_disk(ip: *mut xfs_inode, from: *mut xfs_dinode) -> ::core::ffi::c_int;

    pub fn xfs_dinode_verify(
        mp: *mut xfs_mount,
        ino: xfs_ino_t,
        dip: *mut xfs_dinode,
    ) -> xfs_failaddr_t;
    pub fn xfs_dinode_verify_metadir(
        mp: *mut xfs_mount,
        dip: *mut xfs_dinode,
        mode: u16,
        flags: u16,
        flags2: u64,
    ) -> xfs_failaddr_t;
    pub fn xfs_inode_validate_extsize(
        mp: *mut xfs_mount,
        extsize: u32,
        mode: u16,
        flags: u16,
    ) -> xfs_failaddr_t;
    pub fn xfs_inode_validate_cowextsize(
        mp: *mut xfs_mount,
        cowextsize: u32,
        mode: u16,
        flags: u16,
        flags2: u64,
    ) -> xfs_failaddr_t;

    pub fn xfs_unix_to_bigtime(sec: i64) -> u64;
    pub fn xfs_inode_from_disk_ts(dip: *mut xfs_dinode, ts: xfs_timestamp_t) -> timespec64;
    pub fn xfs_has_v3inodes(mp: *mut xfs_mount) -> bool;
}

pub const NSEC_PER_SEC: u64 = 1_000_000_000;

#[inline]
pub unsafe fn xfs_inode_encode_bigtime(tv: timespec64) -> u64 {
    xfs_unix_to_bigtime(tv.tv_sec)
        .wrapping_mul(NSEC_PER_SEC)
        .wrapping_add(tv.tv_nsec as u64)
}

#[inline]
pub unsafe fn xfs_dinode_good_version(mp: *mut xfs_mount, version: u8) -> bool {
    if xfs_has_v3inodes(mp) {
        version == 3
    } else {
        version == 1 || version == 2
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
