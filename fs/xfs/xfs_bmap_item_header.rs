// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2016 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */

/*
 * There are (currently) two pairs of bmap btree redo item types: map & unmap.
 * The common abbreviations for these are BUI (bmap update intent) and BUD
 * (bmap update done).  The redo item type is encoded in the flags field of
 * each xfs_map_extent.
 *
 * *I items should be recorded in the *first* of a series of rolled
 * transactions, and the *D items should be recorded in the same transaction
 * that records the associated bmbt updates.
 *
 * Should the system crash after the commit of the first transaction but
 * before the commit of the final transaction in a series, log recovery will
 * use the redo information recorded by the intent items to replay the
 * bmbt metadata updates in the non-first transaction.
 */

/* kernel only BUI/BUD definitions */

#[repr(C)]
pub struct xfs_mount;
#[repr(C)]
pub struct kmem_cache;
#[repr(C)]
pub struct xfs_log_item;
#[repr(C)]
pub struct atomic_t;
#[repr(C)]
pub struct xfs_bui_log_format;
#[repr(C)]
pub struct xfs_trans;
#[repr(C)]
pub struct xfs_bmap_intent;

/* Max number of extents in fast allocation path. */
pub const XFS_BUI_MAX_FAST_EXTENTS: u32 = 1;

/*
 * This is the "bmap update intent" log item.  It is used to log the fact that
 * some reverse mappings need to change.  It is used in conjunction with the
 * "bmap update done" log item described below.
 *
 * These log items follow the same rules as struct xfs_efi_log_item; see the
 * comments about that structure (in xfs_extfree_item.h) for more details.
 */
#[repr(C)]
pub struct xfs_bui_log_item {
    pub bui_item: xfs_log_item,
    pub bui_refcount: atomic_t,
    pub bui_next_extent: atomic_t,
    pub bui_format: xfs_bui_log_format,
}

#[inline]
pub unsafe fn xfs_bui_log_item_sizeof(nr: core::ffi::c_uint) -> usize {
    core::mem::offset_of!(xfs_bui_log_item, bui_format)
        + xfs_bui_log_format_sizeof(nr)
}

/*
 * This is the "bmap update done" log item.  It is used to log the fact that
 * some bmbt updates mentioned in an earlier bui item have been performed.
 */
#[repr(C)]
pub struct xfs_bud_log_item {
    pub bud_item: xfs_log_item,
    pub bud_buip: *mut xfs_bui_log_item,
    pub bud_format: xfs_bui_log_format,
}

extern "C" {
    pub static mut xfs_bui_cache: *mut kmem_cache;
    pub static mut xfs_bud_cache: *mut kmem_cache;

    pub fn xfs_bmap_defer_add(tp: *mut xfs_trans, bi: *mut xfs_bmap_intent);

    pub fn xfs_bui_log_format_sizeof(nr: core::ffi::c_uint) -> usize;
    pub fn xfs_bui_log_space(nr: core::ffi::c_uint) -> core::ffi::c_uint;
    pub fn xfs_bud_log_space() -> core::ffi::c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
