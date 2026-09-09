/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2021-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// Forward declarations and types are supplied by the corresponding XFS
// dependencies.  Refcounts only exist on crc enabled filesystems.

pub const XFS_RTREFCOUNT_BLOCK_LEN: usize = XFS_BTREE_LBLOCK_CRC_LEN;

unsafe extern "C" {
    pub fn xfs_rtrefcountbt_init_cursor(
        tp: *mut xfs_trans,
        rtg: *mut xfs_rtgroup,
    ) -> *mut xfs_btree_cur;
    pub fn xfs_rtrefcountbt_stage_cursor(
        mp: *mut xfs_mount,
        rtg: *mut xfs_rtgroup,
        ip: *mut xfs_inode,
        ifake: *mut xbtree_ifakeroot,
    ) -> *mut xfs_btree_cur;
    pub fn xfs_rtrefcountbt_commit_staged_btree(
        cur: *mut xfs_btree_cur,
        tp: *mut xfs_trans,
    );
    pub fn xfs_rtrefcountbt_maxrecs(
        mp: *mut xfs_mount,
        blocklen: u32,
        leaf: bool,
    ) -> u32;
    pub fn xfs_rtrefcountbt_compute_maxlevels(mp: *mut xfs_mount);
    pub fn xfs_rtrefcountbt_droot_maxrecs(blocklen: u32, leaf: bool) -> u32;

    pub fn xfs_rtrefcountbt_maxlevels_ondisk() -> u32;
    pub fn xfs_rtrefcountbt_init_cur_cache() -> i32;
    pub fn xfs_rtrefcountbt_destroy_cur_cache();

    pub fn xfs_rtrefcountbt_calc_reserves(mp: *mut xfs_mount) -> xfs_filblks_t;
    pub fn xfs_rtrefcountbt_calc_size(mp: *mut xfs_mount, len: u64) -> u64;

    pub fn xfs_iformat_rtrefcount(ip: *mut xfs_inode, dip: *mut xfs_dinode) -> i32;
    pub fn xfs_rtrefcountbt_to_disk(
        mp: *mut xfs_mount,
        rblock: *mut xfs_btree_block,
        rblocklen: i32,
        dblock: *mut xfs_rtrefcount_root,
        dblocklen: i32,
    );
    pub fn xfs_iflush_rtrefcount(ip: *mut xfs_inode, dip: *mut xfs_dinode);
    pub fn xfs_rtrefcountbt_create(
        rtg: *mut xfs_rtgroup,
        ip: *mut xfs_inode,
        tp: *mut xfs_trans,
        init: bool,
    ) -> i32;
}

#[inline]
pub unsafe fn xfs_rtrefcount_rec_addr(
    block: *mut xfs_btree_block,
    index: u32,
) -> *mut xfs_refcount_rec {
    (block as *mut u8).add(XFS_RTREFCOUNT_BLOCK_LEN
        + ((index - 1) as usize) * core::mem::size_of::<xfs_refcount_rec>())
        as *mut xfs_refcount_rec
}

#[inline]
pub unsafe fn xfs_rtrefcount_key_addr(
    block: *mut xfs_btree_block,
    index: u32,
) -> *mut xfs_refcount_key {
    (block as *mut u8).add(XFS_RTREFCOUNT_BLOCK_LEN
        + ((index - 1) as usize) * core::mem::size_of::<xfs_refcount_key>())
        as *mut xfs_refcount_key
}

#[inline]
pub unsafe fn xfs_rtrefcount_ptr_addr(
    block: *mut xfs_btree_block,
    index: u32,
    maxrecs: u32,
) -> *mut xfs_rtrefcount_ptr_t {
    (block as *mut u8).add(XFS_RTREFCOUNT_BLOCK_LEN
        + (maxrecs as usize) * core::mem::size_of::<xfs_refcount_key>()
        + ((index - 1) as usize) * core::mem::size_of::<xfs_rtrefcount_ptr_t>())
        as *mut xfs_rtrefcount_ptr_t
}

#[inline]
pub unsafe fn xfs_rtrefcount_droot_rec_addr(
    block: *mut xfs_rtrefcount_root,
    index: u32,
) -> *mut xfs_refcount_rec {
    (block.add(1) as *mut u8).add(((index - 1) as usize)
        * core::mem::size_of::<xfs_refcount_rec>()) as *mut xfs_refcount_rec
}

#[inline]
pub unsafe fn xfs_rtrefcount_droot_key_addr(
    block: *mut xfs_rtrefcount_root,
    index: u32,
) -> *mut xfs_refcount_key {
    (block.add(1) as *mut u8).add(((index - 1) as usize)
        * core::mem::size_of::<xfs_refcount_key>()) as *mut xfs_refcount_key
}

#[inline]
pub unsafe fn xfs_rtrefcount_droot_ptr_addr(
    block: *mut xfs_rtrefcount_root,
    index: u32,
    maxrecs: u32,
) -> *mut xfs_rtrefcount_ptr_t {
    (block.add(1) as *mut u8).add((maxrecs as usize)
        * core::mem::size_of::<xfs_refcount_key>()
        + ((index - 1) as usize) * core::mem::size_of::<xfs_rtrefcount_ptr_t>())
        as *mut xfs_rtrefcount_ptr_t
}

#[inline]
pub unsafe fn xfs_rtrefcount_broot_ptr_addr(
    mp: *mut xfs_mount,
    bb: *mut xfs_btree_block,
    index: u32,
    block_size: u32,
) -> *mut xfs_rtrefcount_ptr_t {
    xfs_rtrefcount_ptr_addr(bb, index, xfs_rtrefcountbt_maxrecs(mp, block_size, false))
}

#[inline]
pub unsafe fn xfs_rtrefcount_broot_space_calc(
    _mp: *mut xfs_mount,
    level: u32,
    nrecs: usize,
) -> usize {
    let sz = XFS_RTREFCOUNT_BLOCK_LEN;
    if level > 0 {
        sz + nrecs * (core::mem::size_of::<xfs_refcount_key>()
            + core::mem::size_of::<xfs_rtrefcount_ptr_t>())
    } else {
        sz + nrecs * core::mem::size_of::<xfs_refcount_rec>()
    }
}

#[inline]
pub unsafe fn xfs_rtrefcount_broot_space(
    mp: *mut xfs_mount,
    bb: *mut xfs_rtrefcount_root,
) -> usize {
    xfs_rtrefcount_broot_space_calc(mp, be16_to_cpu((*bb).bb_level),
        be16_to_cpu((*bb).bb_numrecs) as usize)
}

#[inline]
pub unsafe fn xfs_rtrefcount_droot_space_calc(level: u32, nrecs: usize) -> usize {
    let sz = core::mem::size_of::<xfs_rtrefcount_root>();
    if level > 0 {
        sz + nrecs * (core::mem::size_of::<xfs_refcount_key>()
            + core::mem::size_of::<xfs_rtrefcount_ptr_t>())
    } else {
        sz + nrecs * core::mem::size_of::<xfs_refcount_rec>()
    }
}

#[inline]
pub unsafe fn xfs_rtrefcount_droot_space(bb: *mut xfs_btree_block) -> usize {
    xfs_rtrefcount_droot_space_calc(be16_to_cpu((*bb).bb_level),
        be16_to_cpu((*bb).bb_numrecs) as usize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
