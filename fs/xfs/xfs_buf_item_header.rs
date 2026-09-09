// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2001,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// kernel only definitions

// Forward declarations supplied by other translated units.
pub struct xfs_buf;
pub struct xfs_mount;

// buf log item flags
pub const XFS_BLI_HOLD: u32 = 1u32 << 0;
pub const XFS_BLI_DIRTY: u32 = 1u32 << 1;
pub const XFS_BLI_STALE: u32 = 1u32 << 2;
pub const XFS_BLI_LOGGED: u32 = 1u32 << 3;
pub const XFS_BLI_INODE_ALLOC_BUF: u32 = 1u32 << 4;
pub const XFS_BLI_STALE_INODE: u32 = 1u32 << 5;
pub const XFS_BLI_INODE_BUF: u32 = 1u32 << 6;
pub const XFS_BLI_ORDERED: u32 = 1u32 << 7;

// XFS_BLI_FLAGS expands to the following flag/name pairs in C.
pub const XFS_BLI_FLAGS: &[(u32, &str)] = &[
    (XFS_BLI_HOLD, "HOLD"),
    (XFS_BLI_DIRTY, "DIRTY"),
    (XFS_BLI_STALE, "STALE"),
    (XFS_BLI_LOGGED, "LOGGED"),
    (XFS_BLI_INODE_ALLOC_BUF, "INODE_ALLOC"),
    (XFS_BLI_STALE_INODE, "STALE_INODE"),
    (XFS_BLI_INODE_BUF, "INODE_BUF"),
    (XFS_BLI_ORDERED, "ORDERED"),
];

/*
 * This is the in core log item structure used to track information
 * needed to log buffers.  It tracks how many times the lock has been
 * locked, and which 128 byte chunks of the buffer are dirty.
 */
#[repr(C)]
pub struct xfs_buf_log_item {
    pub bli_item: xfs_log_item,
    pub bli_buf: *mut xfs_buf,
    pub bli_flags: u32,
    pub bli_recur: u32,
    pub bli_refcount: atomic_t,
    pub bli_format_count: i32,
    pub bli_formats: *mut xfs_buf_log_format,
    pub __bli_format: xfs_buf_log_format,
}

extern "C" {
    pub fn xfs_buf_item_init(bp: *mut xfs_buf, mp: *mut xfs_mount) -> i32;
    pub fn xfs_buf_item_done(bp: *mut xfs_buf);
    pub fn xfs_buf_item_put(bip: *mut xfs_buf_log_item);
    pub fn xfs_buf_item_log(bip: *mut xfs_buf_log_item, first: u32, last: u32);
    pub fn xfs_buf_item_dirty_format(bip: *mut xfs_buf_log_item) -> bool;
    pub fn xfs_buf_inode_iodone(bp: *mut xfs_buf);

    #[cfg(feature = "CONFIG_XFS_QUOTA")]
    pub fn xfs_buf_dquot_iodone(bp: *mut xfs_buf);

    #[cfg(not(feature = "CONFIG_XFS_QUOTA"))]
    pub unsafe fn xfs_buf_dquot_iodone(_bp: *mut xfs_buf) {}

    pub fn xfs_buf_iodone(bp: *mut xfs_buf);
    pub fn xfs_buf_log_check_iovec(iovec: *mut kvec) -> bool;

    pub fn xfs_buf_inval_log_space(map_count: u32, blocksize: u32) -> u32;

    pub static mut xfs_buf_item_cache: *mut kmem_cache;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
