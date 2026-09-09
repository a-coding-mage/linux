/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2000,2002-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Translated from xfs_bmap_btree.h.  C headers and build-time dependencies
// are supplied by the surrounding translation unit.

#[repr(C)]
pub struct xfs_btree_cur { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_btree_block { pub bb_numrecs: u16 }
#[repr(C)]
pub struct xfs_mount { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_inode { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_trans { _private: [u8; 0] }
#[repr(C)]
pub struct xbtree_ifakeroot { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_bmdr_block { pub bb_numrecs: u16 }
#[repr(C)]
pub struct xfs_bmbt_rec { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_bmbt_irec { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_bmbt_key { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_buf { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { _private: [u8; 0] }

extern "C" {
    pub fn xfs_bmdr_to_bmbt(ip: *mut xfs_inode, r: *mut xfs_bmdr_block,
        fork: ::core::ffi::c_int, block: *mut xfs_btree_block,
        numrecs: ::core::ffi::c_int);
    pub fn xfs_bmbt_disk_set_all(r: *mut xfs_bmbt_rec, s: *mut xfs_bmbt_irec);
    pub fn xfs_bmbt_disk_get_blockcount(r: *const xfs_bmbt_rec) -> xfs_filblks_t;
    pub fn xfs_bmbt_disk_get_startoff(r: *const xfs_bmbt_rec) -> xfs_fileoff_t;
    pub fn xfs_bmbt_disk_get_all(r: *const xfs_bmbt_rec, s: *mut xfs_bmbt_irec);
    pub fn xfs_bmbt_to_bmdr(mp: *mut xfs_mount, block: *mut xfs_btree_block,
        level: ::core::ffi::c_int, r: *mut xfs_bmdr_block,
        numrecs: ::core::ffi::c_int);
    pub fn xfs_bmbt_get_maxrecs(cur: *mut xfs_btree_cur, level: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn xfs_bmdr_maxrecs(blocklen: ::core::ffi::c_int, leaf: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn xfs_bmbt_maxrecs(mp: *mut xfs_mount, blocklen: u32, leaf: bool) -> u32;
    pub fn xfs_bmbt_change_owner(tp: *mut xfs_trans, ip: *mut xfs_inode,
        whichfork: ::core::ffi::c_int, new_owner: xfs_ino_t,
        buffer_list: *mut list_head) -> ::core::ffi::c_int;
    pub fn xfs_bmbt_init_cursor(mp: *mut xfs_mount, tp: *mut xfs_trans,
        ip: *mut xfs_inode, whichfork: ::core::ffi::c_int) -> *mut xfs_btree_cur;
    pub fn xfs_bmbt_commit_staged_btree(cur: *mut xfs_btree_cur,
        tp: *mut xfs_trans, whichfork: ::core::ffi::c_int);
    pub fn xfs_bmbt_calc_size(mp: *mut xfs_mount, len: u64) -> u64;
    pub fn xfs_bmbt_maxlevels_ondisk() -> u32;
    pub fn xfs_bmbt_init_cur_cache() -> ::core::ffi::c_int;
    pub fn xfs_bmbt_destroy_cur_cache();
    pub fn xfs_bmbt_init_block(ip: *mut xfs_inode, buf: *mut xfs_btree_block,
        bp: *mut xfs_buf, level: u16, numrecs: u16);
    pub fn xfs_has_crc(mp: *mut xfs_mount) -> bool;
    pub fn be16_to_cpu(v: u16) -> u16;
    pub fn xfs_bmap_broot_realloc(ip: *mut xfs_inode, whichfork: ::core::ffi::c_int,
        new_numrecs: u32) -> *mut xfs_btree_block;
}

pub type xfs_bmbt_ptr_t = u64;
pub type xfs_filblks_t = u64;
pub type xfs_fileoff_t = u64;
pub type xfs_ino_t = u64;

#[inline]
pub unsafe fn XFS_BM_MAXLEVELS(mp: *mut xfs_mount, w: usize) -> u32 {
    // The m_bm_maxlevels array is supplied by the translated xfs_mount layout.
    *((mp as *mut u8).add(w * core::mem::size_of::<u32>()) as *const u32)
}

pub const XFS_BTREE_LBLOCK_CRC_LEN: usize = 0;
pub const XFS_BTREE_LBLOCK_LEN: usize = 0;

#[inline]
pub unsafe fn xfs_bmbt_block_len(mp: *mut xfs_mount) -> usize {
    if xfs_has_crc(mp) { XFS_BTREE_LBLOCK_CRC_LEN } else { XFS_BTREE_LBLOCK_LEN }
}

#[inline]
pub unsafe fn xfs_bmbt_rec_addr(mp: *mut xfs_mount, block: *mut xfs_btree_block, index: u32) -> *mut xfs_bmbt_rec {
    (block as *mut u8).add(xfs_bmbt_block_len(mp) + ((index - 1) as usize) * core::mem::size_of::<xfs_bmbt_rec>()) as *mut xfs_bmbt_rec
}

#[inline]
pub unsafe fn xfs_bmbt_key_addr(mp: *mut xfs_mount, block: *mut xfs_btree_block, index: u32) -> *mut xfs_bmbt_key {
    (block as *mut u8).add(xfs_bmbt_block_len(mp) + ((index - 1) as usize) * core::mem::size_of::<xfs_bmbt_key>()) as *mut xfs_bmbt_key
}

#[inline]
pub unsafe fn xfs_bmbt_ptr_addr(mp: *mut xfs_mount, block: *mut xfs_btree_block, index: u32, maxrecs: u32) -> *mut xfs_bmbt_ptr_t {
    (block as *mut u8).add(xfs_bmbt_block_len(mp) + (maxrecs as usize) * core::mem::size_of::<xfs_bmbt_key>() + ((index - 1) as usize) * core::mem::size_of::<xfs_bmbt_ptr_t>()) as *mut xfs_bmbt_ptr_t
}

#[inline]
pub unsafe fn xfs_bmdr_rec_addr(block: *mut xfs_bmdr_block, index: u32) -> *mut xfs_bmbt_rec {
    (block as *mut u8).add(core::mem::size_of::<xfs_bmdr_block>() + ((index - 1) as usize) * core::mem::size_of::<xfs_bmbt_rec>()) as *mut xfs_bmbt_rec
}
#[inline]
pub unsafe fn xfs_bmdr_key_addr(block: *mut xfs_bmdr_block, index: u32) -> *mut xfs_bmbt_key {
    (block as *mut u8).add(core::mem::size_of::<xfs_bmdr_block>() + ((index - 1) as usize) * core::mem::size_of::<xfs_bmbt_key>()) as *mut xfs_bmbt_key
}
#[inline]
pub unsafe fn xfs_bmdr_ptr_addr(block: *mut xfs_bmdr_block, index: u32, maxrecs: u32) -> *mut xfs_bmbt_ptr_t {
    (block as *mut u8).add(core::mem::size_of::<xfs_bmdr_block>() + (maxrecs as usize) * core::mem::size_of::<xfs_bmbt_key>() + ((index - 1) as usize) * core::mem::size_of::<xfs_bmbt_ptr_t>()) as *mut xfs_bmbt_ptr_t
}

#[inline]
pub unsafe fn xfs_bmap_broot_ptr_addr(mp: *mut xfs_mount, bb: *mut xfs_btree_block, i: u32, sz: u32) -> *mut xfs_bmbt_ptr_t {
    xfs_bmbt_ptr_addr(mp, bb, i, xfs_bmbt_maxrecs(mp, sz, false))
}
#[inline]
pub unsafe fn xfs_bmap_broot_space_calc(mp: *mut xfs_mount, nrecs: u32) -> usize {
    xfs_bmbt_block_len(mp) + (nrecs as usize) * (core::mem::size_of::<xfs_bmbt_key>() + core::mem::size_of::<xfs_bmbt_ptr_t>())
}
#[inline]
pub unsafe fn xfs_bmap_broot_space(mp: *mut xfs_mount, bb: *mut xfs_bmdr_block) -> usize {
    xfs_bmap_broot_space_calc(mp, be16_to_cpu((*bb).bb_numrecs))
}
#[inline]
pub unsafe fn xfs_bmdr_space_calc(nrecs: u32) -> usize {
    core::mem::size_of::<xfs_bmdr_block>() + (nrecs as usize) * (core::mem::size_of::<xfs_bmbt_key>() + core::mem::size_of::<xfs_bmbt_ptr_t>())
}
#[inline]
pub unsafe fn xfs_bmap_bmdr_space(bb: *mut xfs_btree_block) -> usize {
    xfs_bmdr_space_calc(be16_to_cpu((*bb).bb_numrecs))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
