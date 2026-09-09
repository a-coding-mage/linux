// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Declarations supplied by the surrounding XFS translation.
#[repr(C)]
pub struct xfs_buf;
#[repr(C)]
pub struct xfs_dinode;
#[repr(C)]
pub struct xfs_imap;
#[repr(C)]
pub struct xfs_mount;
#[repr(C)]
pub struct xfs_trans;
#[repr(C)]
pub struct xfs_btree_cur;
#[repr(C)]
pub struct xfs_perag;
#[repr(C)]
pub struct xfs_icreate_args;
#[repr(C)]
pub union xfs_btree_rec {
    _private: [u8; 0],
}

pub type xfs_ino_t = u64;
pub type xfs_agino_t = u32;
pub type xfs_agnumber_t = u32;
pub type xfs_agblock_t = u32;
pub type xfs_extlen_t = u32;
pub type xfs_inofree_t = u64;
pub type xfs_buf_flags_t = u32;
pub type xfs_failaddr_t = *mut core::ffi::c_void;
pub type xfs_lookup_t = u32;
pub type uint = u32;

#[repr(C)]
pub struct xfs_icluster {
    pub deleted: bool,
    pub first_ino: xfs_ino_t,
    pub alloc: u64,
}

pub const XFS_INODE_BIG_CLUSTER_SIZE: u32 = 8192;
pub const XFS_IALLOC_FLAG_TRYLOCK: u32 = 1u32 << 0;

// The following member access and external symbols are provided by the
// corresponding translated XFS headers and implementation.
#[inline]
pub unsafe fn xfs_make_iptr(mp: *mut xfs_mount, b: *mut xfs_buf, o: i32) -> *mut xfs_dinode {
    xfs_buf_offset(b, o << (*mp).m_sb.sb_inodelog)
}

extern "C" {
    fn xfs_buf_offset(b: *mut xfs_buf, offset: i32) -> *mut xfs_dinode;

    pub fn xfs_dialloc(tpp: *mut *mut xfs_trans, args: *const xfs_icreate_args,
                       new_ino: *mut xfs_ino_t) -> i32;
    pub fn xfs_difree(tp: *mut xfs_trans, pag: *mut xfs_perag, ino: xfs_ino_t,
                      ifree: *mut xfs_icluster) -> i32;
    pub fn xfs_imap(pag: *mut xfs_perag, tp: *mut xfs_trans, ino: xfs_ino_t,
                    imap: *mut xfs_imap, flags: uint) -> i32;
    pub fn xfs_ialloc_log_agi(tp: *mut xfs_trans, bp: *mut xfs_buf, fields: u32);
    pub fn xfs_read_agi(pag: *mut xfs_perag, tp: *mut xfs_trans, flags: xfs_buf_flags_t,
                        agibpp: *mut *mut xfs_buf) -> i32;
    pub fn xfs_ialloc_read_agi(pag: *mut xfs_perag, tp: *mut xfs_trans, flags: i32,
                               agibpp: *mut *mut xfs_buf) -> i32;
    pub fn xfs_inobt_lookup(cur: *mut xfs_btree_cur, ino: xfs_agino_t,
                            dir: xfs_lookup_t, stat: *mut i32) -> i32;
    pub fn xfs_inobt_get_rec(cur: *mut xfs_btree_cur, rec: *mut xfs_inobt_rec_incore,
                             stat: *mut i32) -> i32;
    pub fn xfs_inobt_rec_freecount(irec: *const xfs_inobt_rec_incore) -> u8;
    pub fn xfs_ialloc_inode_init(mp: *mut xfs_mount, tp: *mut xfs_trans,
                                 buffer_list: *mut list_head, icount: i32,
                                 agno: xfs_agnumber_t, agbno: xfs_agblock_t,
                                 length: xfs_agblock_t, gen: u32) -> i32;
    pub fn xfs_inobt_btrec_to_irec(mp: *mut xfs_mount, rec: *const xfs_btree_rec,
                                   irec: *mut xfs_inobt_rec_incore);
    pub fn xfs_inobt_check_irec(pag: *mut xfs_perag,
                                irec: *const xfs_inobt_rec_incore) -> xfs_failaddr_t;
    pub fn xfs_ialloc_has_inodes_at_extent(cur: *mut xfs_btree_cur, bno: xfs_agblock_t,
                                           len: xfs_extlen_t, outcome: *mut xbtree_recpacking) -> i32;
    pub fn xfs_ialloc_count_inodes(cur: *mut xfs_btree_cur, count: *mut xfs_agino_t,
                                   freecount: *mut xfs_agino_t) -> i32;
    pub fn xfs_inobt_insert_rec(cur: *mut xfs_btree_cur, holemask: u16, count: u8,
                                freecount: i32, free: xfs_inofree_t, stat: *mut i32) -> i32;
    pub fn xfs_ialloc_cluster_alignment(mp: *mut xfs_mount) -> i32;
    pub fn xfs_ialloc_setup_geometry(mp: *mut xfs_mount);
    pub fn xfs_ialloc_calc_rootino(mp: *mut xfs_mount, sunit: i32) -> xfs_ino_t;
    pub fn xfs_ialloc_check_shrink(pag: *mut xfs_perag, tp: *mut xfs_trans,
                                   agibp: *mut xfs_buf, new_length: xfs_agblock_t) -> i32;
}

#[repr(C)] pub struct xfs_inobt_rec_incore { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub enum xbtree_recpacking { _private }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
