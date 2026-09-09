// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

/* Inode map on-disk structures. */

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
pub struct xfs_trans {
    _private: [u8; 0],
}

/* These types and constants are supplied by the corresponding XFS headers. */
extern "C" {
    pub fn xfs_has_crc(mp: *const xfs_mount) -> bool;
}

/* Btree block header size depends on a superblock flag. */
#[inline]
pub unsafe fn XFS_INOBT_BLOCK_LEN(mp: *const xfs_mount) -> usize {
    if xfs_has_crc(mp) {
        XFS_BTREE_SBLOCK_CRC_LEN as usize
    } else {
        XFS_BTREE_SBLOCK_LEN as usize
    }
}

/* Record, key, and pointer address macros for btree blocks. */
#[inline]
pub unsafe fn XFS_INOBT_REC_ADDR(
    mp: *const xfs_mount,
    block: *mut core::ffi::c_void,
    index: usize,
) -> *mut xfs_inobt_rec_t {
    (block as *mut u8).add(
        XFS_INOBT_BLOCK_LEN(mp) + (index.wrapping_sub(1)) * core::mem::size_of::<xfs_inobt_rec_t>(),
    ) as *mut xfs_inobt_rec_t
}

#[inline]
pub unsafe fn XFS_INOBT_KEY_ADDR(
    mp: *const xfs_mount,
    block: *mut core::ffi::c_void,
    index: usize,
) -> *mut xfs_inobt_key_t {
    (block as *mut u8).add(
        XFS_INOBT_BLOCK_LEN(mp) + (index.wrapping_sub(1)) * core::mem::size_of::<xfs_inobt_key_t>(),
    ) as *mut xfs_inobt_key_t
}

#[inline]
pub unsafe fn XFS_INOBT_PTR_ADDR(
    mp: *const xfs_mount,
    block: *mut core::ffi::c_void,
    index: usize,
    maxrecs: usize,
) -> *mut xfs_inobt_ptr_t {
    (block as *mut u8).add(
        XFS_INOBT_BLOCK_LEN(mp)
            + maxrecs * core::mem::size_of::<xfs_inobt_key_t>()
            + (index.wrapping_sub(1)) * core::mem::size_of::<xfs_inobt_ptr_t>(),
    ) as *mut xfs_inobt_ptr_t
}

extern "C" {
    pub fn xfs_inobt_init_cursor(
        pag: *mut xfs_perag,
        tp: *mut xfs_trans,
        agbp: *mut xfs_buf,
    ) -> *mut xfs_btree_cur;
    pub fn xfs_finobt_init_cursor(
        pag: *mut xfs_perag,
        tp: *mut xfs_trans,
        agbp: *mut xfs_buf,
    ) -> *mut xfs_btree_cur;
    pub fn xfs_inobt_maxrecs(mp: *mut xfs_mount, blocklen: u32, leaf: bool) -> u32;

    /* ir_holemask to inode allocation bitmap conversion. */
    pub fn xfs_inobt_irec_to_allocmask(irec: *const xfs_inobt_rec_incore) -> u64;

    pub fn xfs_finobt_calc_reserves(
        perag: *mut xfs_perag,
        tp: *mut xfs_trans,
        ask: *mut xfs_extlen_t,
        used: *mut xfs_extlen_t,
    ) -> i32;
    pub fn xfs_iallocbt_calc_size(mp: *mut xfs_mount, len: u64) -> xfs_extlen_t;

    pub fn xfs_inobt_commit_staged_btree(
        cur: *mut xfs_btree_cur,
        tp: *mut xfs_trans,
        agbp: *mut xfs_buf,
    );
    pub fn xfs_iallocbt_maxlevels_ondisk() -> u32;
    pub fn xfs_inobt_init_cur_cache() -> i32;
    pub fn xfs_inobt_destroy_cur_cache();
}

/* DEBUG/XFS_WARN builds provide this check; otherwise the C macro expands to 0. */
#[cfg(any(debug_assertions, feature = "xfs_warn"))]
extern "C" {
    pub fn xfs_inobt_rec_check_count(
        mp: *mut xfs_mount,
        rec: *mut xfs_inobt_rec_incore,
    ) -> i32;
}

#[cfg(not(any(debug_assertions, feature = "xfs_warn")))]
#[inline]
pub const fn xfs_inobt_rec_check_count(
    _mp: *mut xfs_mount,
    _rec: *mut xfs_inobt_rec_incore,
) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
