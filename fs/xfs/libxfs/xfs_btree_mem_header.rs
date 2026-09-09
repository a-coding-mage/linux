/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2021-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

pub type xfbno_t = u64;

pub const XFBNO_BLOCKSIZE: _ = XMBUF_BLOCKSIZE;
pub const XFBNO_BBSHIFT: _ = XMBUF_BLOCKSHIFT - BBSHIFT;
pub const XFBNO_BBSIZE: _ = XFBNO_BLOCKSIZE >> BBSHIFT;

#[inline]
pub unsafe fn xfbno_to_daddr(blkno: xfbno_t) -> xfs_daddr_t {
    blkno << XFBNO_BBSHIFT
}

#[inline]
pub unsafe fn xfs_daddr_to_xfbno(daddr: xfs_daddr_t) -> xfbno_t {
    daddr >> XFBNO_BBSHIFT
}

#[repr(C)]
pub struct xfbtree {
    /* buffer cache target for this in-memory btree */
    pub target: *mut xfs_buftarg,

    /* Highest block number that has been written to. */
    pub highest_bno: xfbno_t,

    /* Owner of this btree. */
    pub owner: u64,

    /* Btree header */
    pub root: xfs_btree_ptr,
    pub nlevels: u32,

    /* Minimum and maximum records per block. */
    pub maxrecs: [u32; 2],
    pub minrecs: [u32; 2],
}

#[cfg(feature = "CONFIG_XFS_BTREE_IN_MEM")]
#[inline]
pub unsafe fn xfbtree_verify_bno(xfbt: *mut xfbtree, bno: xfbno_t) -> bool {
    xmbuf_verify_daddr((*xfbt).target, xfbno_to_daddr(bno))
}

#[cfg(feature = "CONFIG_XFS_BTREE_IN_MEM")]
extern "C" {
    pub fn xfbtree_set_root(
        cur: *mut xfs_btree_cur,
        ptr: *const xfs_btree_ptr,
        inc: ::core::ffi::c_int,
    );
    pub fn xfbtree_init_ptr_from_cur(cur: *mut xfs_btree_cur, ptr: *mut xfs_btree_ptr);
    pub fn xfbtree_dup_cursor(cur: *mut xfs_btree_cur) -> *mut xfs_btree_cur;

    pub fn xfbtree_get_minrecs(cur: *mut xfs_btree_cur, level: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn xfbtree_get_maxrecs(cur: *mut xfs_btree_cur, level: ::core::ffi::c_int) -> ::core::ffi::c_int;

    pub fn xfbtree_alloc_block(
        cur: *mut xfs_btree_cur,
        start: *const xfs_btree_ptr,
        ptr: *mut xfs_btree_ptr,
        stat: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn xfbtree_free_block(cur: *mut xfs_btree_cur, bp: *mut xfs_buf) -> ::core::ffi::c_int;

    /* Callers must set xfbt->target and xfbt->owner before calling this */
    pub fn xfbtree_init(
        mp: *mut xfs_mount,
        xfbt: *mut xfbtree,
        btp: *mut xfs_buftarg,
        ops: *const xfs_btree_ops,
    ) -> ::core::ffi::c_int;
    pub fn xfbtree_destroy(xfbt: *mut xfbtree);

    pub fn xfbtree_trans_commit(xfbt: *mut xfbtree, tp: *mut xfs_trans) -> ::core::ffi::c_int;
    pub fn xfbtree_trans_cancel(xfbt: *mut xfbtree, tp: *mut xfs_trans);
}

#[cfg(not(feature = "CONFIG_XFS_BTREE_IN_MEM"))]
#[macro_export]
macro_rules! xfbtree_verify_bno {
    ($($arg:tt)*) => { false };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
