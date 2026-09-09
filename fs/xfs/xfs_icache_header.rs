// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2006 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// The C header guard `XFS_SYNC_H` is omitted from Rust source.

// External types supplied by other translation units.
// struct xfs_mount;
// struct xfs_perag;

#[repr(C)]
pub struct xfs_icwalk {
    pub icw_flags: u32,
    pub icw_uid: kuid_t,
    pub icw_gid: kgid_t,
    pub icw_prid: prid_t,
    pub icw_min_file_size: u64,
    pub icw_scan_limit: isize,
}

/* Flags that reflect xfs_fs_eofblocks functionality. */
pub const XFS_ICWALK_FLAG_SYNC: u32 = 1u32 << 0; // sync/wait mode scan
pub const XFS_ICWALK_FLAG_UID: u32 = 1u32 << 1; // filter by uid
pub const XFS_ICWALK_FLAG_GID: u32 = 1u32 << 2; // filter by gid
pub const XFS_ICWALK_FLAG_PRID: u32 = 1u32 << 3; // filter by project id
pub const XFS_ICWALK_FLAG_MINFILESIZE: u32 = 1u32 << 4; // filter by min file size

pub const XFS_ICWALK_FLAGS_VALID: u32 = XFS_ICWALK_FLAG_SYNC
    | XFS_ICWALK_FLAG_UID
    | XFS_ICWALK_FLAG_GID
    | XFS_ICWALK_FLAG_PRID
    | XFS_ICWALK_FLAG_MINFILESIZE;

/*
 * Flags for xfs_iget()
 */
pub const XFS_IGET_CREATE: u32 = 1u32 << 0;
pub const XFS_IGET_UNTRUSTED: u32 = 1u32 << 1;
pub const XFS_IGET_DONTCACHE: u32 = 1u32 << 2;
/* don't read from disk or reinit */
pub const XFS_IGET_INCORE: u32 = 1u32 << 3;
/* Return -EAGAIN immediately if the inode is unavailable. */
pub const XFS_IGET_NORETRY: u32 = 1u32 << 4;

unsafe extern "C" {
    pub fn xfs_iget(
        mp: *mut xfs_mount,
        tp: *mut xfs_trans,
        ino: xfs_ino_t,
        flags: u32,
        lock_flags: u32,
        ipp: *mut *mut xfs_inode_t,
    ) -> core::ffi::c_int;

    /* recovery needs direct inode allocation capability */
    pub fn xfs_inode_alloc(mp: *mut xfs_mount, ino: xfs_ino_t) -> *mut xfs_inode;
    pub fn xfs_inode_free(ip: *mut xfs_inode);

    pub fn xfs_reclaim_worker(work: *mut work_struct);

    pub fn xfs_reclaim_inodes(mp: *mut xfs_mount);
    pub fn xfs_reclaim_inodes_count(mp: *mut xfs_mount) -> isize;
    pub fn xfs_reclaim_inodes_nr(mp: *mut xfs_mount, nr_to_scan: usize) -> isize;

    pub fn xfs_inode_mark_reclaimable(ip: *mut xfs_inode);

    pub fn xfs_blockgc_free_dquots(
        mp: *mut xfs_mount,
        udqp: *mut xfs_dquot,
        gdqp: *mut xfs_dquot,
        pdqp: *mut xfs_dquot,
        iwalk_flags: u32,
    ) -> core::ffi::c_int;
    pub fn xfs_blockgc_free_quota(ip: *mut xfs_inode, iwalk_flags: u32) -> core::ffi::c_int;
    pub fn xfs_blockgc_free_space(mp: *mut xfs_mount, icm: *mut xfs_icwalk) -> core::ffi::c_int;
    pub fn xfs_blockgc_flush_all(mp: *mut xfs_mount) -> core::ffi::c_int;

    pub fn xfs_inode_set_eofblocks_tag(ip: *mut xfs_inode);
    pub fn xfs_inode_clear_eofblocks_tag(ip: *mut xfs_inode);

    pub fn xfs_inode_set_cowblocks_tag(ip: *mut xfs_inode);
    pub fn xfs_inode_clear_cowblocks_tag(ip: *mut xfs_inode);

    pub fn xfs_blockgc_worker(work: *mut work_struct);
    pub fn xfs_blockgc_stop(mp: *mut xfs_mount);
    pub fn xfs_blockgc_start(mp: *mut xfs_mount);

    pub fn xfs_inodegc_worker(work: *mut work_struct);
    pub fn xfs_inodegc_push(mp: *mut xfs_mount);
    pub fn xfs_inodegc_flush(mp: *mut xfs_mount) -> core::ffi::c_int;
    pub fn xfs_inodegc_stop(mp: *mut xfs_mount);
    pub fn xfs_inodegc_start(mp: *mut xfs_mount);
    pub fn xfs_inodegc_register_shrinker(mp: *mut xfs_mount) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
