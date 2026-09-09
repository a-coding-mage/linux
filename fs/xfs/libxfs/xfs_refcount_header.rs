// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2016 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */

pub const XFS_REFCOUNT_ITEM_OVERHEAD: u32 = 32;

#[inline]
pub unsafe fn xfs_refcount_encode_startblock(
    startblock: xfs_agblock_t,
    domain: xfs_refc_domain,
) -> u32 {
    let mut start = startblock & !XFS_REFC_COWFLAG;
    if domain != xfs_refc_domain::XFS_REFC_DOMAIN_SHARED {
        start |= XFS_REFC_COWFLAG;
    }
    start
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum xfs_refcount_intent_type {
    XFS_REFCOUNT_INCREASE = 1,
    XFS_REFCOUNT_DECREASE,
    XFS_REFCOUNT_ALLOC_COW,
    XFS_REFCOUNT_FREE_COW,
}

pub const XFS_REFCOUNT_INTENT_STRINGS: &[(xfs_refcount_intent_type, &str)] = &[
    (xfs_refcount_intent_type::XFS_REFCOUNT_INCREASE, "incr"),
    (xfs_refcount_intent_type::XFS_REFCOUNT_DECREASE, "decr"),
    (xfs_refcount_intent_type::XFS_REFCOUNT_ALLOC_COW, "alloc_cow"),
    (xfs_refcount_intent_type::XFS_REFCOUNT_FREE_COW, "free_cow"),
];

#[repr(C)]
pub struct xfs_refcount_intent {
    pub ri_list: list_head,
    pub ri_group: *mut xfs_group,
    pub ri_type: xfs_refcount_intent_type,
    pub ri_blockcount: xfs_extlen_t,
    pub ri_startblock: xfs_fsblock_t,
    pub ri_realtime: bool,
}

#[inline]
pub unsafe fn xfs_refcount_check_domain(irec: *const xfs_refcount_irec) -> bool {
    if (*irec).rc_domain == xfs_refc_domain::XFS_REFC_DOMAIN_COW && (*irec).rc_refcount != 1 {
        return false;
    }
    if (*irec).rc_domain == xfs_refc_domain::XFS_REFC_DOMAIN_SHARED && (*irec).rc_refcount < 2 {
        return false;
    }
    true
}

pub type xfs_refcount_query_range_fn = unsafe extern "C" fn(
    cur: *mut xfs_btree_cur,
    rec: *const xfs_refcount_irec,
    priv_: *mut core::ffi::c_void,
) -> i32;

extern "C" {
    pub fn xfs_refcount_lookup_le(cur: *mut xfs_btree_cur, domain: xfs_refc_domain, bno: xfs_agblock_t, stat: *mut i32) -> i32;
    pub fn xfs_refcount_lookup_ge(cur: *mut xfs_btree_cur, domain: xfs_refc_domain, bno: xfs_agblock_t, stat: *mut i32) -> i32;
    pub fn xfs_refcount_lookup_eq(cur: *mut xfs_btree_cur, domain: xfs_refc_domain, bno: xfs_agblock_t, stat: *mut i32) -> i32;
    pub fn xfs_refcount_get_rec(cur: *mut xfs_btree_cur, irec: *mut xfs_refcount_irec, stat: *mut i32) -> i32;
    pub fn xfs_refcount_increase_extent(tp: *mut xfs_trans, isrt: bool, irec: *mut xfs_bmbt_irec);
    pub fn xfs_refcount_decrease_extent(tp: *mut xfs_trans, isrt: bool, irec: *mut xfs_bmbt_irec);
    pub fn xfs_refcount_finish_one(tp: *mut xfs_trans, ri: *mut xfs_refcount_intent, pcur: *mut *mut xfs_btree_cur) -> i32;
    pub fn xfs_rtrefcount_finish_one(tp: *mut xfs_trans, ri: *mut xfs_refcount_intent, pcur: *mut *mut xfs_btree_cur) -> i32;
    pub fn xfs_refcount_find_shared(cur: *mut xfs_btree_cur, agbno: xfs_agblock_t, aglen: xfs_extlen_t, fbno: *mut xfs_agblock_t, flen: *mut xfs_extlen_t, find_end_of_shared: bool) -> i32;
    pub fn xfs_refcount_alloc_cow_extent(tp: *mut xfs_trans, isrt: bool, fsb: xfs_fsblock_t, len: xfs_extlen_t);
    pub fn xfs_refcount_free_cow_extent(tp: *mut xfs_trans, isrt: bool, fsb: xfs_fsblock_t, len: xfs_extlen_t);
    pub fn xfs_refcount_recover_cow_leftovers(xg: *mut xfs_group) -> i32;
    pub fn xfs_refcount_has_records(cur: *mut xfs_btree_cur, domain: xfs_refc_domain, bno: xfs_agblock_t, len: xfs_extlen_t, outcome: *mut xbtree_recpacking) -> i32;
    pub fn xfs_refcount_btrec_to_irec(rec: *const xfs_btree_rec, irec: *mut xfs_refcount_irec);
    pub fn xfs_refcount_check_irec(pag: *mut xfs_perag, irec: *const xfs_refcount_irec) -> xfs_failaddr_t;
    pub fn xfs_rtrefcount_check_irec(rtg: *mut xfs_rtgroup, irec: *const xfs_refcount_irec) -> xfs_failaddr_t;
    pub fn xfs_refcount_insert(cur: *mut xfs_btree_cur, irec: *mut xfs_refcount_irec, stat: *mut i32) -> i32;
    pub static mut xfs_refcount_intent_cache: *mut kmem_cache;
    // __init is a kernel build attribute with no direct file-local Rust equivalent.
    pub fn xfs_refcount_intent_init_cache() -> i32;
    pub fn xfs_refcount_intent_destroy_cache();
    pub fn xfs_refcount_query_range(cur: *mut xfs_btree_cur, low_rec: *const xfs_refcount_irec, high_rec: *const xfs_refcount_irec, fn_: xfs_refcount_query_range_fn, priv_: *mut core::ffi::c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
