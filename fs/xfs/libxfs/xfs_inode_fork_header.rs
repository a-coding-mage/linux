// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2003,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// C header dependencies and build-time definitions are supplied externally.

pub struct xfs_inode_log_item;
pub struct xfs_dinode;

/*
 * File incore extent information, present for each of data & attr forks.
 */
#[repr(C)]
pub struct xfs_ifork {
    pub if_bytes: i64,
    pub if_broot: *mut xfs_btree_block,
    pub if_seq: u32,
    pub if_height: i32,
    pub if_data: *mut core::ffi::c_void,
    pub if_nextents: xfs_extnum_t,
    pub if_broot_bytes: i16,
    pub if_format: i8,
    pub if_needextents: u8,
}

pub const XFS_IEXT_ADD_NOSPLIT_CNT: usize = 1;
pub const XFS_IEXT_PUNCH_HOLE_CNT: usize = 1;

#[inline]
pub fn XFS_IEXT_ATTR_MANIP_CNT(rmt_blks: i64) -> i64 {
    (XFS_DA_NODE_MAXDEPTH as i64) + core::cmp::max(1, rmt_blks)
}

pub const XFS_IEXT_WRITE_UNWRITTEN_CNT: usize = 2;
pub const XFS_IEXT_REFLINK_END_COW_CNT: usize = 2;
pub const XFS_IEXT_SWAP_RMAP_CNT: usize = 1;

#[inline]
pub unsafe fn XFS_IFORK_MAXEXT(ip: *mut xfs_inode, w: i32) -> usize {
    xfs_inode_fork_size(ip, w) / core::mem::size_of::<xfs_bmbt_rec_t>()
}

#[inline]
pub unsafe fn xfs_ifork_has_extents(ifp: *mut xfs_ifork) -> bool {
    (*ifp).if_format == XFS_DINODE_FMT_EXTENTS || (*ifp).if_format == XFS_DINODE_FMT_BTREE
}

#[inline]
pub unsafe fn xfs_ifork_nextents(ifp: *mut xfs_ifork) -> xfs_extnum_t {
    if ifp.is_null() { return 0; }
    (*ifp).if_nextents
}

#[inline]
pub unsafe fn xfs_ifork_format(ifp: *mut xfs_ifork) -> i8 {
    if ifp.is_null() { return XFS_DINODE_FMT_EXTENTS; }
    (*ifp).if_format
}

#[inline]
pub fn xfs_iext_max_nextents(has_large_extent_counts: bool, whichfork: i32) -> xfs_extnum_t {
    match whichfork {
        XFS_DATA_FORK | XFS_COW_FORK => if has_large_extent_counts { XFS_MAX_EXTCNT_DATA_FORK_LARGE } else { XFS_MAX_EXTCNT_DATA_FORK_SMALL },
        XFS_ATTR_FORK => if has_large_extent_counts { XFS_MAX_EXTCNT_ATTR_FORK_LARGE } else { XFS_MAX_EXTCNT_ATTR_FORK_SMALL },
        _ => { unsafe { ASSERT(0); } 0 }
    }
}

#[inline]
pub unsafe fn xfs_dfork_data_extents(dip: *mut xfs_dinode) -> xfs_extnum_t {
    if xfs_dinode_has_large_extent_counts(dip) { return be64_to_cpu((*dip).di_big_nextents); }
    be32_to_cpu((*dip).di_nextents)
}

#[inline]
pub unsafe fn xfs_dfork_attr_extents(dip: *mut xfs_dinode) -> xfs_extnum_t {
    if xfs_dinode_has_large_extent_counts(dip) { return be32_to_cpu((*dip).di_big_anextents); }
    be16_to_cpu((*dip).di_anextents)
}

#[inline]
pub unsafe fn xfs_dfork_nextents(dip: *mut xfs_dinode, whichfork: i32) -> xfs_extnum_t {
    match whichfork {
        XFS_DATA_FORK => xfs_dfork_data_extents(dip),
        XFS_ATTR_FORK => xfs_dfork_attr_extents(dip),
        _ => { ASSERT(0); 0 }
    }
}

extern "C" {
    pub fn xfs_ifork_zap_attr(ip: *mut xfs_inode);
    pub fn xfs_ifork_init_attr(ip: *mut xfs_inode, format: xfs_dinode_fmt, nextents: xfs_extnum_t);
    pub fn xfs_iext_state_to_fork(ip: *mut xfs_inode, state: i32) -> *mut xfs_ifork;
    pub fn xfs_iformat_data_fork(ip: *mut xfs_inode, dip: *mut xfs_dinode) -> i32;
    pub fn xfs_iformat_attr_fork(ip: *mut xfs_inode, dip: *mut xfs_dinode) -> i32;
    pub fn xfs_iflush_fork(ip: *mut xfs_inode, dip: *mut xfs_dinode, ilf: *mut xfs_inode_log_item, whichfork: i32);
    pub fn xfs_idestroy_fork(ifp: *mut xfs_ifork);
    pub fn xfs_idata_realloc(ip: *mut xfs_inode, byte_diff: i64, whichfork: i32) -> *mut core::ffi::c_void;
    pub fn xfs_broot_alloc(ifp: *mut xfs_ifork, new_size: usize) -> *mut xfs_btree_block;
    pub fn xfs_broot_realloc(ifp: *mut xfs_ifork, new_size: usize) -> *mut xfs_btree_block;
    pub fn xfs_iread_extents(tp: *mut xfs_trans, ip: *mut xfs_inode, fork: i32) -> i32;
    pub fn xfs_iextents_copy(ip: *mut xfs_inode, rec: *mut xfs_bmbt_rec, whichfork: i32) -> i32;
    pub fn xfs_init_local_fork(ip: *mut xfs_inode, whichfork: i32, data: *const core::ffi::c_void, size: i64);
    pub fn xfs_iext_count(ifp: *mut xfs_ifork) -> xfs_extnum_t;
    pub fn xfs_iext_insert_raw(ifp: *mut xfs_ifork, cur: *mut xfs_iext_cursor, irec: *mut xfs_bmbt_irec);
    pub fn xfs_iext_insert(ip: *mut xfs_inode, cur: *mut xfs_iext_cursor, irec: *mut xfs_bmbt_irec, state: i32);
    pub fn xfs_iext_remove(ip: *mut xfs_inode, cur: *mut xfs_iext_cursor, state: i32);
    pub fn xfs_iext_destroy(ifp: *mut xfs_ifork);
    pub fn xfs_iext_lookup_extent(ip: *mut xfs_inode, ifp: *mut xfs_ifork, bno: xfs_fileoff_t, cur: *mut xfs_iext_cursor, gotp: *mut xfs_bmbt_irec) -> bool;
    pub fn xfs_iext_lookup_extent_before(ip: *mut xfs_inode, ifp: *mut xfs_ifork, end: *mut xfs_fileoff_t, cur: *mut xfs_iext_cursor, gotp: *mut xfs_bmbt_irec) -> bool;
    pub fn xfs_iext_get_extent(ifp: *mut xfs_ifork, cur: *mut xfs_iext_cursor, gotp: *mut xfs_bmbt_irec) -> bool;
    pub fn xfs_iext_update_extent(ip: *mut xfs_inode, state: i32, cur: *mut xfs_iext_cursor, gotp: *mut xfs_bmbt_irec);
    pub fn xfs_iext_first(ifp: *mut xfs_ifork, cur: *mut xfs_iext_cursor);
    pub fn xfs_iext_last(ifp: *mut xfs_ifork, cur: *mut xfs_iext_cursor);
    pub fn xfs_iext_next(ifp: *mut xfs_ifork, cur: *mut xfs_iext_cursor);
    pub fn xfs_iext_prev(ifp: *mut xfs_ifork, cur: *mut xfs_iext_cursor);
    pub fn xfs_ifork_verify_local_data(ip: *mut xfs_inode) -> i32;
    pub fn xfs_ifork_verify_local_attr(ip: *mut xfs_inode) -> i32;
    pub fn xfs_iext_count_extend(tp: *mut xfs_trans, ip: *mut xfs_inode, whichfork: i32, nr_to_add: u32) -> i32;
    pub fn xfs_ifork_is_realtime(ip: *mut xfs_inode, whichfork: i32) -> bool;
    pub fn xfs_ifork_init_cow(ip: *mut xfs_inode);
    pub static mut xfs_ifork_cache: *mut kmem_cache;
}

#[inline]
pub unsafe fn xfs_iext_next_extent(ifp: *mut xfs_ifork, cur: *mut xfs_iext_cursor, gotp: *mut xfs_bmbt_irec) -> bool {
    xfs_iext_next(ifp, cur); xfs_iext_get_extent(ifp, cur, gotp)
}

#[inline]
pub unsafe fn xfs_iext_prev_extent(ifp: *mut xfs_ifork, cur: *mut xfs_iext_cursor, gotp: *mut xfs_bmbt_irec) -> bool {
    xfs_iext_prev(ifp, cur); xfs_iext_get_extent(ifp, cur, gotp)
}

/* Return the extent after cur in gotp without updating the cursor. */
#[inline]
pub unsafe fn xfs_iext_peek_next_extent(ifp: *mut xfs_ifork, cur: *mut xfs_iext_cursor, gotp: *mut xfs_bmbt_irec) -> bool {
    let mut ncur = *cur;
    xfs_iext_next(ifp, &mut ncur); xfs_iext_get_extent(ifp, &mut ncur, gotp)
}

/* Return the extent before cur in gotp without updating the cursor. */
#[inline]
pub unsafe fn xfs_iext_peek_prev_extent(ifp: *mut xfs_ifork, cur: *mut xfs_iext_cursor, gotp: *mut xfs_bmbt_irec) -> bool {
    let mut ncur = *cur;
    xfs_iext_prev(ifp, &mut ncur); xfs_iext_get_extent(ifp, &mut ncur, gotp)
}

/* C macro for_each_xfs_iext is represented by explicit first/get/next calls. */

#[inline]
pub unsafe fn xfs_need_iread_extents(ifp: *const xfs_ifork) -> bool {
    /* see xfs_iformat_{data,attr}_fork() for needextents semantics */
    core::ptr::read_volatile(&(*ifp).if_needextents) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
