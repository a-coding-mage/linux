// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2016 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */

/*
 * There are (currently) two pairs of refcount btree redo item types:
 * increase and decrease.  The log items for these are CUI (refcount
 * update intent) and CUD (refcount update done).  The redo item type
 * is encoded in the flags field of each xfs_map_extent.
 *
 * *I items should be recorded in the *first* of a series of rolled
 * transactions, and the *D items should be recorded in the same
 * transaction that records the associated refcountbt updates.
 *
 * Should the system crash after the commit of the first transaction
 * but before the commit of the final transaction in a series, log
 * recovery will use the redo information recorded by the intent items
 * to replay the refcountbt metadata updates.
 */

/* kernel only CUI/CUD definitions */

/*
 * Max number of extents in fast allocation path.
 */
pub const XFS_CUI_MAX_FAST_EXTENTS: u32 = 16;

/*
 * This is the "refcount update intent" log item.  It is used to log
 * the fact that some reverse mappings need to change.  It is used in
 * conjunction with the "refcount update done" log item described
 * below.
 *
 * These log items follow the same rules as struct xfs_efi_log_item;
 * see the comments about that structure (in xfs_extfree_item.h) for
 * more details.
 */
#[repr(C)]
pub struct xfs_cui_log_item {
    pub cui_item: xfs_log_item,
    pub cui_refcount: atomic_t,
    pub cui_next_extent: atomic_t,
    pub cui_format: xfs_cui_log_format,
}

pub unsafe fn xfs_cui_log_item_sizeof(nr: ::core::ffi::c_uint) -> usize {
    ::core::mem::offset_of!(xfs_cui_log_item, cui_format)
        + xfs_cui_log_format_sizeof(nr)
}

/*
 * This is the "refcount update done" log item.  It is used to log
 * the fact that some refcountbt updates mentioned in an earlier cui item
 * have been performed.
 */
#[repr(C)]
pub struct xfs_cud_log_item {
    pub cud_item: xfs_log_item,
    pub cud_cuip: *mut xfs_cui_log_item,
    pub cud_format: xfs_cud_log_format,
}

unsafe extern "C" {
    pub static mut xfs_cui_cache: *mut kmem_cache;
    pub static mut xfs_cud_cache: *mut kmem_cache;

    pub fn xfs_refcount_defer_add(
        tp: *mut xfs_trans,
        ri: *mut xfs_refcount_intent,
    );

    pub fn xfs_cui_log_space(nr: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn xfs_cud_log_space() -> ::core::ffi::c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
