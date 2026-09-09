// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2022-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// CONFIG_XFS_BTREE_IN_MEM controls whether these declarations are available.

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
pub struct xfs_trans {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfbtree {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_buftarg {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_rmap_irec {
    _private: [u8; 0],
}

pub const RCBAG_MAGIC: u32 = 0x74826671; // 'JRBG'

#[repr(C)]
pub struct rcbag_key {
    pub rbg_startblock: u32,
    pub rbg_blockcount: u32,
}

#[repr(C)]
pub struct rcbag_rec {
    pub rbg_startblock: u32,
    pub rbg_blockcount: u32,
    pub rbg_refcount: u64,
}

// __be64 is supplied by the surrounding XFS dependencies.
pub type rcbag_ptr_t = __be64;

// Reflinks only exist on crc enabled filesystems.
pub const RCBAG_BLOCK_LEN: usize = XFS_BTREE_LBLOCK_CRC_LEN;

/*
 * Record, key, and pointer address macros for btree blocks.
 *
 * (note that some of these may appear unused, but they are used in userspace)
 */
#[inline]
pub unsafe fn RCBAG_REC_ADDR(block: *mut c_void, index: usize) -> *mut rcbag_rec {
    (block as *mut u8)
        .add(RCBAG_BLOCK_LEN + (index - 1) * core::mem::size_of::<rcbag_rec>())
        as *mut rcbag_rec
}

#[inline]
pub unsafe fn RCBAG_KEY_ADDR(block: *mut c_void, index: usize) -> *mut rcbag_key {
    (block as *mut u8)
        .add(RCBAG_BLOCK_LEN + (index - 1) * core::mem::size_of::<rcbag_key>())
        as *mut rcbag_key
}

#[inline]
pub unsafe fn RCBAG_PTR_ADDR(
    block: *mut c_void,
    index: usize,
    maxrecs: usize,
) -> *mut rcbag_ptr_t {
    (block as *mut u8).add(
        RCBAG_BLOCK_LEN
            + maxrecs * core::mem::size_of::<rcbag_key>()
            + (index - 1) * core::mem::size_of::<rcbag_ptr_t>(),
    ) as *mut rcbag_ptr_t
}

extern "C" {
    pub fn rcbagbt_maxrecs(mp: *mut xfs_mount, blocklen: u32, leaf: bool) -> u32;
    pub fn rcbagbt_calc_size(nr_records: u64) -> u64;
    pub fn rcbagbt_maxlevels_possible() -> u32;

    pub fn rcbagbt_init_cur_cache() -> i32;
    pub fn rcbagbt_destroy_cur_cache();

    pub fn rcbagbt_mem_cursor(
        mp: *mut xfs_mount,
        tp: *mut xfs_trans,
        xfbtree: *mut xfbtree,
    ) -> *mut xfs_btree_cur;
    pub fn rcbagbt_mem_init(
        mp: *mut xfs_mount,
        xfbtree: *mut xfbtree,
        btp: *mut xfs_buftarg,
    ) -> i32;

    pub fn rcbagbt_lookup_eq(
        cur: *mut xfs_btree_cur,
        rmap: *const xfs_rmap_irec,
        success: *mut i32,
    ) -> i32;
    pub fn rcbagbt_get_rec(
        cur: *mut xfs_btree_cur,
        rec: *mut rcbag_rec,
        has: *mut i32,
    ) -> i32;
    pub fn rcbagbt_update(cur: *mut xfs_btree_cur, rec: *const rcbag_rec) -> i32;
    pub fn rcbagbt_insert(
        cur: *mut xfs_btree_cur,
        rec: *const rcbag_rec,
        success: *mut i32,
    ) -> i32;
}

// When CONFIG_XFS_BTREE_IN_MEM is disabled:
// #define rcbagbt_init_cur_cache() 0
// #define rcbagbt_destroy_cur_cache() ((void)0)


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
