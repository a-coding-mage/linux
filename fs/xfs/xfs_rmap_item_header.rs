// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2016 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */

/*
 * There are (currently) three pairs of rmap btree redo item types: map, unmap,
 * and convert.  The common abbreviations for these are RUI (rmap update
 * intent) and RUD (rmap update done).  The redo item type is encoded in the
 * flags field of each xfs_map_extent.
 *
 * *I items should be recorded in the *first* of a series of rolled
 * transactions, and the *D items should be recorded in the same transaction
 * that records the associated rmapbt updates.  Typically, the first
 * transaction will record a bmbt update, followed by some number of
 * transactions containing rmapbt updates, and finally transactions with any
 * bnobt/cntbt updates.
 *
 * Should the system crash after the commit of the first transaction but
 * before the commit of the final transaction in a series, log recovery will
 * use the redo information recorded by the intent items to replay the
 * (rmapbt/bnobt/cntbt) metadata updates in the non-first transaction.
 */

/* kernel only RUI/RUD definitions */

pub struct xfs_mount;
pub struct kmem_cache;

/*
 * Max number of extents in fast allocation path.
 */
pub const XFS_RUI_MAX_FAST_EXTENTS: usize = 16;

/*
 * This is the "rmap update intent" log item.  It is used to log the fact that
 * some reverse mappings need to change.  It is used in conjunction with the
 * "rmap update done" log item described below.
 *
 * These log items follow the same rules as struct xfs_efi_log_item; see the
 * comments about that structure (in xfs_extfree_item.h) for more details.
 */
#[repr(C)]
pub struct xfs_rui_log_item {
    pub rui_item: xfs_log_item,
    pub rui_refcount: atomic_t,
    pub rui_next_extent: atomic_t,
    pub rui_format: xfs_rui_log_format,
}

#[inline]
pub unsafe fn xfs_rui_log_item_sizeof(nr: c_uint) -> usize {
    core::mem::offset_of!(xfs_rui_log_item, rui_format)
        + xfs_rui_log_format_sizeof(nr)
}

/*
 * This is the "rmap update done" log item.  It is used to log the fact that
 * some rmapbt updates mentioned in an earlier rui item have been performed.
 */
#[repr(C)]
pub struct xfs_rud_log_item {
    pub rud_item: xfs_log_item,
    pub rud_ruip: *mut xfs_rui_log_item,
    pub rud_format: xfs_rud_log_format,
}

extern "C" {
    pub static mut xfs_rui_cache: *mut kmem_cache;
    pub static mut xfs_rud_cache: *mut kmem_cache;
}

pub struct xfs_rmap_intent;

extern "C" {
    pub fn xfs_rmap_defer_add(tp: *mut xfs_trans, ri: *mut xfs_rmap_intent);
    pub fn xfs_rui_log_space(nr: c_uint) -> c_uint;
    pub fn xfs_rud_log_space() -> c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
