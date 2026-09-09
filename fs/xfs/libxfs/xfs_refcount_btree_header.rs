// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2016 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */

/*
 * Reference Count Btree on-disk structures
 */

// Opaque declarations supplied by other translation units.
#[repr(C)]
pub struct xfs_buf {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_btree_cur {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_mount {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_perag {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xbtree_afakeroot {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_trans {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_refcount_rec {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_refcount_key {
    _private: [u8; 0],
}

// These types and constants are supplied by other headers.
pub type xfs_refcount_ptr_t = u64;
pub type xfs_extlen_t = u64;
pub type xfs_agblock_t = u64;

/*
 * Btree block header size
 */
// #define XFS_REFCOUNT_BLOCK_LEN XFS_BTREE_SBLOCK_CRC_LEN
pub const XFS_REFCOUNT_BLOCK_LEN: usize = XFS_BTREE_SBLOCK_CRC_LEN;

/*
 * Record, key, and pointer address macros for btree blocks.
 *
 * (note that some of these may appear unused, but they are used in userspace)
 */
#[inline]
pub unsafe fn XFS_REFCOUNT_REC_ADDR(
    block: *mut core::ffi::c_void,
    index: usize,
) -> *mut xfs_refcount_rec {
    (block.cast::<u8>()
        .add(XFS_REFCOUNT_BLOCK_LEN + (index.wrapping_sub(1)) * core::mem::size_of::<xfs_refcount_rec>()))
        .cast()
}

#[inline]
pub unsafe fn XFS_REFCOUNT_KEY_ADDR(
    block: *mut core::ffi::c_void,
    index: usize,
) -> *mut xfs_refcount_key {
    (block.cast::<u8>()
        .add(XFS_REFCOUNT_BLOCK_LEN + (index.wrapping_sub(1)) * core::mem::size_of::<xfs_refcount_key>()))
        .cast()
}

#[inline]
pub unsafe fn XFS_REFCOUNT_PTR_ADDR(
    block: *mut core::ffi::c_void,
    index: usize,
    maxrecs: usize,
) -> *mut xfs_refcount_ptr_t {
    (block.cast::<u8>()
        .add(XFS_REFCOUNT_BLOCK_LEN
            + maxrecs * core::mem::size_of::<xfs_refcount_key>()
            + (index.wrapping_sub(1)) * core::mem::size_of::<xfs_refcount_ptr_t>()))
        .cast()
}

unsafe extern "C" {
    pub fn xfs_refcountbt_init_cursor(
        mp: *mut xfs_mount,
        tp: *mut xfs_trans,
        agbp: *mut xfs_buf,
        pag: *mut xfs_perag,
    ) -> *mut xfs_btree_cur;
    pub fn xfs_refcountbt_maxrecs(
        mp: *mut xfs_mount,
        blocklen: u32,
        leaf: bool,
    ) -> u32;
    pub fn xfs_refcountbt_compute_maxlevels(mp: *mut xfs_mount);
    pub fn xfs_refcountbt_calc_size(mp: *mut xfs_mount, len: u64) -> xfs_extlen_t;
    pub fn xfs_refcountbt_max_size(mp: *mut xfs_mount, agblocks: xfs_agblock_t) -> xfs_extlen_t;
    pub fn xfs_refcountbt_calc_reserves(
        mp: *mut xfs_mount,
        tp: *mut xfs_trans,
        pag: *mut xfs_perag,
        ask: *mut xfs_extlen_t,
        used: *mut xfs_extlen_t,
    ) -> i32;
    pub fn xfs_refcountbt_commit_staged_btree(
        cur: *mut xfs_btree_cur,
        tp: *mut xfs_trans,
        agbp: *mut xfs_buf,
    );
    pub fn xfs_refcountbt_maxlevels_ondisk() -> u32;
    pub fn xfs_refcountbt_init_cur_cache() -> i32;
    pub fn xfs_refcountbt_destroy_cur_cache();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
