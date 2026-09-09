/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2018-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// rmaps only exist on crc enabled filesystems
pub const XFS_RTRMAP_BLOCK_LEN: usize = XFS_BTREE_LBLOCK_CRC_LEN;

extern "C" {
    pub fn xfs_rtrmapbt_init_cursor(tp: *mut xfs_trans, rtg: *mut xfs_rtgroup) -> *mut xfs_btree_cur;
    pub fn xfs_rtrmapbt_stage_cursor(
        mp: *mut xfs_mount,
        rtg: *mut xfs_rtgroup,
        ip: *mut xfs_inode,
        ifake: *mut xbtree_ifakeroot,
    ) -> *mut xfs_btree_cur;
    pub fn xfs_rtrmapbt_commit_staged_btree(cur: *mut xfs_btree_cur, tp: *mut xfs_trans);
    pub fn xfs_rtrmapbt_maxrecs(mp: *mut xfs_mount, blocklen: u32, leaf: bool) -> u32;
    pub fn xfs_rtrmapbt_compute_maxlevels(mp: *mut xfs_mount);
    pub fn xfs_rtrmapbt_droot_maxrecs(blocklen: u32, leaf: bool) -> u32;
    pub fn xfs_rtrmapbt_maxlevels_ondisk() -> u32;
    pub fn xfs_rtrmapbt_init_cur_cache() -> i32;
    pub fn xfs_rtrmapbt_destroy_cur_cache();
    pub fn xfs_rtrmapbt_calc_reserves(mp: *mut xfs_mount) -> xfs_filblks_t;
}

pub unsafe fn xfs_rtrmap_rec_addr(block: *mut xfs_btree_block, index: u32) -> *mut xfs_rmap_rec {
    (block as *mut u8)
        .add(XFS_RTRMAP_BLOCK_LEN + ((index - 1) as usize) * core::mem::size_of::<xfs_rmap_rec>())
        as *mut xfs_rmap_rec
}

pub unsafe fn xfs_rtrmap_key_addr(block: *mut xfs_btree_block, index: u32) -> *mut xfs_rmap_key {
    (block as *mut u8)
        .add(XFS_RTRMAP_BLOCK_LEN + ((index - 1) as usize) * 2 * core::mem::size_of::<xfs_rmap_key>())
        as *mut xfs_rmap_key
}

pub unsafe fn xfs_rtrmap_high_key_addr(block: *mut xfs_btree_block, index: u32) -> *mut xfs_rmap_key {
    (block as *mut u8)
        .add(XFS_RTRMAP_BLOCK_LEN + core::mem::size_of::<xfs_rmap_key>()
            + ((index - 1) as usize) * 2 * core::mem::size_of::<xfs_rmap_key>())
        as *mut xfs_rmap_key
}

pub unsafe fn xfs_rtrmap_ptr_addr(
    block: *mut xfs_btree_block,
    index: u32,
    maxrecs: u32,
) -> *mut xfs_rtrmap_ptr_t {
    (block as *mut u8)
        .add(XFS_RTRMAP_BLOCK_LEN + (maxrecs as usize) * 2 * core::mem::size_of::<xfs_rmap_key>()
            + ((index - 1) as usize) * core::mem::size_of::<xfs_rtrmap_ptr_t>())
        as *mut xfs_rtrmap_ptr_t
}

pub unsafe fn xfs_rtrmap_droot_rec_addr(block: *mut xfs_rtrmap_root, index: u32) -> *mut xfs_rmap_rec {
    (block.add(1) as *mut u8)
        .add(((index - 1) as usize) * core::mem::size_of::<xfs_rmap_rec>()) as *mut xfs_rmap_rec
}

pub unsafe fn xfs_rtrmap_droot_key_addr(block: *mut xfs_rtrmap_root, index: u32) -> *mut xfs_rmap_key {
    (block.add(1) as *mut u8)
        .add(((index - 1) as usize) * 2 * core::mem::size_of::<xfs_rmap_key>()) as *mut xfs_rmap_key
}

pub unsafe fn xfs_rtrmap_droot_ptr_addr(
    block: *mut xfs_rtrmap_root,
    index: u32,
    maxrecs: u32,
) -> *mut xfs_rtrmap_ptr_t {
    (block.add(1) as *mut u8)
        .add((maxrecs as usize) * 2 * core::mem::size_of::<xfs_rmap_key>()
            + ((index - 1) as usize) * core::mem::size_of::<xfs_rtrmap_ptr_t>())
        as *mut xfs_rtrmap_ptr_t
}

pub unsafe fn xfs_rtrmap_broot_ptr_addr(
    mp: *mut xfs_mount,
    bb: *mut xfs_btree_block,
    index: u32,
    block_size: u32,
) -> *mut xfs_rtrmap_ptr_t {
    xfs_rtrmap_ptr_addr(bb, index, xfs_rtrmapbt_maxrecs(mp, block_size, false))
}

pub unsafe fn xfs_rtrmap_broot_space_calc(_mp: *mut xfs_mount, level: u32, nrecs: u32) -> usize {
    let sz = XFS_RTRMAP_BLOCK_LEN;
    if level > 0 {
        return sz + (nrecs as usize) * (2 * core::mem::size_of::<xfs_rmap_key>()
            + core::mem::size_of::<xfs_rtrmap_ptr_t>());
    }
    sz + (nrecs as usize) * core::mem::size_of::<xfs_rmap_rec>()
}

pub unsafe fn xfs_rtrmap_broot_space(mp: *mut xfs_mount, bb: *mut xfs_rtrmap_root) -> usize {
    xfs_rtrmap_broot_space_calc(mp, be16_to_cpu((*bb).bb_level), be16_to_cpu((*bb).bb_numrecs))
}

pub unsafe fn xfs_rtrmap_droot_space_calc(level: u32, nrecs: u32) -> usize {
    let sz = core::mem::size_of::<xfs_rtrmap_root>();
    if level > 0 {
        return sz + (nrecs as usize) * (2 * core::mem::size_of::<xfs_rmap_key>()
            + core::mem::size_of::<xfs_rtrmap_ptr_t>());
    }
    sz + (nrecs as usize) * core::mem::size_of::<xfs_rmap_rec>()
}

pub unsafe fn xfs_rtrmap_droot_space(bb: *mut xfs_btree_block) -> usize {
    xfs_rtrmap_droot_space_calc(be16_to_cpu((*bb).bb_level), be16_to_cpu((*bb).bb_numrecs))
}

extern "C" {
    pub fn xfs_iformat_rtrmap(ip: *mut xfs_inode, dip: *mut xfs_dinode) -> i32;
    pub fn xfs_rtrmapbt_to_disk(mp: *mut xfs_mount, rblock: *mut xfs_btree_block, rblocklen: u32,
        dblock: *mut xfs_rtrmap_root, dblocklen: u32);
    pub fn xfs_iflush_rtrmap(ip: *mut xfs_inode, dip: *mut xfs_dinode);
    pub fn xfs_rtrmapbt_create(rtg: *mut xfs_rtgroup, ip: *mut xfs_inode, tp: *mut xfs_trans, init: bool) -> i32;
    pub fn xfs_rtrmapbt_init_rtsb(mp: *mut xfs_mount, rtg: *mut xfs_rtgroup, tp: *mut xfs_trans) -> i32;
    pub fn xfs_rtrmapbt_calc_size(mp: *mut xfs_mount, len: u64) -> u64;
    pub fn xfs_rtrmapbt_mem_cursor(rtg: *mut xfs_rtgroup, tp: *mut xfs_trans, xfbtree: *mut xfbtree) -> *mut xfs_btree_cur;
    pub fn xfs_rtrmapbt_mem_init(mp: *mut xfs_mount, xfbtree: *mut xfbtree, btp: *mut xfs_buftarg, rgno: xfs_rgnumber_t) -> i32;
    pub fn xfs_rtrmap_highest_rgbno(rtg: *mut xfs_rtgroup) -> xfs_rgblock_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
