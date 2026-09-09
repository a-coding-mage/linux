/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2000,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

/*
 * Freespace on-disk structures
 */

/* C forward declarations; the complete definitions are supplied by dependencies. */
#[repr(C)]
pub struct xfs_buf {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct xfs_btree_cur {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct xfs_mount {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct xfs_perag {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct xbtree_afakeroot {
    _opaque: [u8; 0],
}

/* Btree block header size depends on a superblock flag. */
#[macro_export]
macro_rules! XFS_ALLOC_BLOCK_LEN {
    ($mp:expr) => {
        if unsafe { xfs_has_crc($mp) } {
            XFS_BTREE_SBLOCK_CRC_LEN
        } else {
            XFS_BTREE_SBLOCK_LEN
        }
    };
}

/*
 * Record, key, and pointer address macros for btree blocks.
 *
 * (note that some of these may appear unused, but they are used in userspace)
 */
#[macro_export]
macro_rules! XFS_ALLOC_REC_ADDR {
    ($mp:expr, $block:expr, $index:expr) => {
        unsafe {
            (($block as *mut u8).add(
                XFS_ALLOC_BLOCK_LEN!($mp)
                    + (($index - 1) * core::mem::size_of::<xfs_alloc_rec_t>()),
            )) as *mut xfs_alloc_rec_t
        }
    };
}

#[macro_export]
macro_rules! XFS_ALLOC_KEY_ADDR {
    ($mp:expr, $block:expr, $index:expr) => {
        unsafe {
            (($block as *mut u8).add(
                XFS_ALLOC_BLOCK_LEN!($mp)
                    + (($index - 1) * core::mem::size_of::<xfs_alloc_key_t>()),
            )) as *mut xfs_alloc_key_t
        }
    };
}

#[macro_export]
macro_rules! XFS_ALLOC_PTR_ADDR {
    ($mp:expr, $block:expr, $index:expr, $maxrecs:expr) => {
        unsafe {
            (($block as *mut u8).add(
                XFS_ALLOC_BLOCK_LEN!($mp)
                    + ($maxrecs) * core::mem::size_of::<xfs_alloc_key_t>()
                    + (($index - 1) * core::mem::size_of::<xfs_alloc_ptr_t>()),
            )) as *mut xfs_alloc_ptr_t
        }
    };
}

extern "C" {
    pub fn xfs_bnobt_init_cursor(
        mp: *mut xfs_mount,
        tp: *mut xfs_trans,
        bp: *mut xfs_buf,
        pag: *mut xfs_perag,
    ) -> *mut xfs_btree_cur;
    pub fn xfs_cntbt_init_cursor(
        mp: *mut xfs_mount,
        tp: *mut xfs_trans,
        bp: *mut xfs_buf,
        pag: *mut xfs_perag,
    ) -> *mut xfs_btree_cur;
    pub fn xfs_allocbt_maxrecs(
        mp: *mut xfs_mount,
        blocklen: core::ffi::c_uint,
        leaf: bool,
    ) -> core::ffi::c_uint;
    pub fn xfs_allocbt_calc_size(mp: *mut xfs_mount, len: u64) -> xfs_extlen_t;
    pub fn xfs_allocbt_commit_staged_btree(
        cur: *mut xfs_btree_cur,
        tp: *mut xfs_trans,
        agbp: *mut xfs_buf,
    );
    pub fn xfs_allocbt_maxlevels_ondisk() -> core::ffi::c_uint;
    pub fn xfs_allocbt_init_cur_cache() -> core::ffi::c_int;
    pub fn xfs_allocbt_destroy_cur_cache();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
