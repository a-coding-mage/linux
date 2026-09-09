// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// kernel only definitions

#[repr(C)]
pub struct xfs_buf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_bmbt_rec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_mount {
    _private: [u8; 0],
}

// Supplied by the corresponding inode definitions.
#[repr(C)]
pub struct xfs_inode {
    pub i_itemp: *mut xfs_inode_log_item,
}

// Supplied by the corresponding logging definitions.
#[repr(C)]
pub struct xfs_log_item {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_inode_log_format {
    _private: [u8; 0],
}

pub type xfs_lsn_t = u64;
pub type xfs_csn_t = u64;

// XFS_ILOG_ALL is supplied by the inode logging definitions.
extern "C" {
    pub static xfs_ili_cache: *mut kmem_cache;

    pub fn xfs_inode_item_init(ip: *mut xfs_inode, mp: *mut xfs_mount);
    pub fn xfs_inode_item_destroy(ip: *mut xfs_inode);
    pub fn xfs_iflush_abort(ip: *mut xfs_inode);
    pub fn xfs_iflush_shutdown_abort(ip: *mut xfs_inode);
    pub fn xfs_inode_item_format_convert(
        buf: *mut kvec,
        in_f: *mut xfs_inode_log_format,
    ) -> ::std::os::raw::c_int;
}

#[repr(C)]
pub struct kmem_cache {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_inode_log_item {
    pub ili_item: xfs_log_item,
    pub ili_inode: *mut xfs_inode,
    pub ili_lock_flags: u16,
    pub ili_dirty_flags: u32,
    /*
     * The ili_lock protects the interactions between the dirty state and
     * the flush state of the inode log item. This allows us to do atomic
     * modifications of multiple state fields without having to hold a
     * specific inode lock to serialise them.
     *
     * We need atomic changes between inode dirtying, inode flushing and
     * inode completion, but these all hold different combinations of
     * ILOCK and IFLUSHING and hence we need some other method of
     * serialising updates to the flush state.
     */
    pub ili_lock: spinlock_t,
    pub ili_last_fields: u32,
    pub ili_fields: u32,
    pub ili_flush_lsn: xfs_lsn_t,
    /*
     * We record the sequence number for every inode modification, as
     * well as those that only require fdatasync operations for data
     * integrity. This allows optimisation of the O_DSYNC/fdatasync path
     * without needing to track what modifications the journal is currently
     * carrying for the inode. These are protected by the above ili_lock.
     */
    pub ili_commit_seq: xfs_csn_t,
    pub ili_datasync_seq: xfs_csn_t,
}

// XFS_ILOG_ALL is supplied by the inode logging definitions.
#[inline]
pub unsafe fn xfs_inode_clean(ip: *mut xfs_inode) -> bool {
    (*ip).i_itemp.is_null()
        || ((*(*ip).i_itemp).ili_fields & XFS_ILOG_ALL) == 0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
