// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2014 Red Hat, Inc.
 * All Rights Reserved.
 */

// C header guard: __XFS_RMAP_BTREE_H__

use core::ffi::c_void;

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
pub struct xbtree_afakeroot {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfbtree {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_trans {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_perag {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_buftarg {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_rmap_rec {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_rmap_key {
    _private: [u8; 0],
}

// rmaps only exist on crc enabled filesystems
pub const XFS_RMAP_BLOCK_LEN: usize = XFS_BTREE_SBLOCK_CRC_LEN;

/*
 * Record, key, and pointer address macros for btree blocks.
 *
 * (note that some of these may appear unused, but they are used in userspace)
 */

#[inline]
pub unsafe fn XFS_RMAP_REC_ADDR(block: *mut c_void, index: usize) -> *mut xfs_rmap_rec {
    (block as *mut u8)
        .add(XFS_RMAP_BLOCK_LEN + (index.wrapping_sub(1)) * core::mem::size_of::<xfs_rmap_rec>())
        as *mut xfs_rmap_rec
}

#[inline]
pub unsafe fn XFS_RMAP_KEY_ADDR(block: *mut c_void, index: usize) -> *mut xfs_rmap_key {
    (block as *mut u8)
        .add(XFS_RMAP_BLOCK_LEN + (index.wrapping_sub(1)) * 2 * core::mem::size_of::<xfs_rmap_key>())
        as *mut xfs_rmap_key
}

#[inline]
pub unsafe fn XFS_RMAP_HIGH_KEY_ADDR(block: *mut c_void, index: usize) -> *mut xfs_rmap_key {
    (block as *mut u8)
        .add(XFS_RMAP_BLOCK_LEN + core::mem::size_of::<xfs_rmap_key>()
            + (index.wrapping_sub(1)) * 2 * core::mem::size_of::<xfs_rmap_key>())
        as *mut xfs_rmap_key
}

#[inline]
pub unsafe fn XFS_RMAP_PTR_ADDR(
    block: *mut c_void,
    index: usize,
    maxrecs: usize,
) -> *mut xfs_rmap_ptr_t {
    (block as *mut u8)
        .add(XFS_RMAP_BLOCK_LEN + maxrecs * 2 * core::mem::size_of::<xfs_rmap_key>()
            + (index.wrapping_sub(1)) * core::mem::size_of::<xfs_rmap_ptr_t>())
        as *mut xfs_rmap_ptr_t
}

extern "C" {
    pub fn xfs_rmapbt_init_cursor(
        mp: *mut xfs_mount,
        tp: *mut xfs_trans,
        bp: *mut xfs_buf,
        pag: *mut xfs_perag,
    ) -> *mut xfs_btree_cur;
    pub fn xfs_rmapbt_commit_staged_btree(
        cur: *mut xfs_btree_cur,
        tp: *mut xfs_trans,
        agbp: *mut xfs_buf,
    );
    pub fn xfs_rmapbt_maxrecs(mp: *mut xfs_mount, blocklen: u32, leaf: bool) -> u32;
    pub fn xfs_rmapbt_compute_maxlevels(mp: *mut xfs_mount);
    pub fn xfs_rmapbt_calc_size(mp: *mut xfs_mount, len: u64) -> xfs_extlen_t;
    pub fn xfs_rmapbt_max_size(mp: *mut xfs_mount, agblocks: xfs_agblock_t) -> xfs_extlen_t;
    pub fn xfs_rmapbt_calc_reserves(
        mp: *mut xfs_mount,
        tp: *mut xfs_trans,
        pag: *mut xfs_perag,
        ask: *mut xfs_extlen_t,
        used: *mut xfs_extlen_t,
    ) -> i32;
    pub fn xfs_rmapbt_maxlevels_ondisk() -> u32;
    // __init
    pub fn xfs_rmapbt_init_cur_cache() -> i32;
    pub fn xfs_rmapbt_destroy_cur_cache();
    pub fn xfs_rmapbt_mem_cursor(
        pag: *mut xfs_perag,
        tp: *mut xfs_trans,
        xfbtree: *mut xfbtree,
    ) -> *mut xfs_btree_cur;
    pub fn xfs_rmapbt_mem_init(
        mp: *mut xfs_mount,
        xfbtree: *mut xfbtree,
        btp: *mut xfs_buftarg,
        agno: xfs_agnumber_t,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
