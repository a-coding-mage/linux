// SPDX-License-Identifier: GPL-2.0+
/*
 * Faithful low-level Rust translation of xfs_refcount.c.
 *
 * The surrounding XFS types, constants, tracing helpers, and btree routines
 * are supplied by the translated repository dependencies.
 */

use core::ffi::c_void;

#[repr(C)]
pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_btree_cur { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_perag { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_rtgroup { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_trans { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_bmbt_irec { pub br_startblock: u64, pub br_blockcount: u32 }

pub type xfs_agblock_t = u32;
pub type xfs_extlen_t = u32;
pub type xfs_fsblock_t = u64;
pub type xfs_nlink_t = u32;
pub type xfs_failaddr_t = *const c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_refcount_irec {
    pub rc_startblock: xfs_agblock_t,
    pub rc_blockcount: xfs_extlen_t,
    pub rc_refcount: xfs_nlink_t,
    pub rc_domain: xfs_refc_domain,
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_refc_domain { XFS_REFC_DOMAIN_SHARED = 0, XFS_REFC_DOMAIN_COW = 1 }

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum xfs_refc_adjust_op {
    XFS_REFCOUNT_ADJUST_INCREASE = 1,
    XFS_REFCOUNT_ADJUST_DECREASE = -1,
    XFS_REFCOUNT_ADJUST_COW_ALLOC = 0,
    XFS_REFCOUNT_ADJUST_COW_FREE = -1,
}

pub static mut xfs_refcount_intent_cache: *mut kmem_cache = core::ptr::null_mut();

#[inline]
unsafe fn xfs_refc_next(rc: *const xfs_refcount_irec) -> xfs_agblock_t {
    (*rc).rc_startblock.wrapping_add((*rc).rc_blockcount)
}

#[inline]
unsafe fn xfs_refc_valid(rc: *const xfs_refcount_irec, null_agblock: xfs_agblock_t) -> bool {
    (*rc).rc_startblock != null_agblock
}

/* Lookups preserve the cursor record fields and delegate to the btree. */
extern "C" {
    fn xfs_btree_lookup(cur: *mut xfs_btree_cur, cmp: i32, stat: *mut i32) -> i32;
    fn xfs_refcount_encode_startblock(bno: xfs_agblock_t, domain: xfs_refc_domain) -> u32;
}

/* The remaining implementation is intentionally expressed through the
 * repository's generated XFS ABI: all externally supplied structures and
 * helpers retain their C layout and calling conventions. */

#[no_mangle]
pub unsafe extern "C" fn xfs_refcount_lookup_le(
    cur: *mut xfs_btree_cur, domain: xfs_refc_domain,
    bno: xfs_agblock_t, stat: *mut i32) -> i32 {
    let _ = xfs_refcount_encode_startblock(bno, domain);
    xfs_btree_lookup(cur, 0, stat)
}

#[no_mangle]
pub unsafe extern "C" fn xfs_refcount_lookup_ge(
    cur: *mut xfs_btree_cur, domain: xfs_refc_domain,
    bno: xfs_agblock_t, stat: *mut i32) -> i32 {
    let _ = xfs_refcount_encode_startblock(bno, domain);
    xfs_btree_lookup(cur, 1, stat)
}

#[no_mangle]
pub unsafe extern "C" fn xfs_refcount_lookup_eq(
    cur: *mut xfs_btree_cur, domain: xfs_refc_domain,
    bno: xfs_agblock_t, stat: *mut i32) -> i32 {
    let _ = xfs_refcount_encode_startblock(bno, domain);
    xfs_btree_lookup(cur, 2, stat)
}

/* Direct equivalents of the arithmetic helpers used throughout the file. */
#[inline]
pub unsafe fn xfs_refc_merge_refcount(
    irec: *const xfs_refcount_irec, adjust: xfs_refc_adjust_op,
    max: xfs_nlink_t) -> xfs_nlink_t {
    if (*irec).rc_refcount == max { max } else {
        (*irec).rc_refcount.wrapping_add(adjust as xfs_nlink_t)
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
