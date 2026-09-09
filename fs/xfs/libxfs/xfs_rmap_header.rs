// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2016 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */

// Forward declarations supplied by other translation units.
pub struct xfs_perag;
pub struct xfs_rtgroup;

pub unsafe fn xfs_rmap_ino_bmbt_owner(
    oi: *mut xfs_owner_info,
    ino: xfs_ino_t,
    whichfork: ::std::os::raw::c_int,
) {
    (*oi).oi_owner = ino;
    (*oi).oi_offset = 0;
    (*oi).oi_flags = XFS_OWNER_INFO_BMBT_BLOCK;
    if whichfork == XFS_ATTR_FORK {
        (*oi).oi_flags |= XFS_OWNER_INFO_ATTR_FORK;
    }
}

#[macro_export]
macro_rules! xfs_rmap_inode_bmbt_owner {
    ($oi:expr, $ip:expr, $whichfork:expr) => {
        xfs_rmap_ino_bmbt_owner($oi, I_INO($ip), $whichfork)
    };
}

pub unsafe fn xfs_rmap_ino_owner(
    oi: *mut xfs_owner_info,
    ino: xfs_ino_t,
    whichfork: ::std::os::raw::c_int,
    offset: xfs_fileoff_t,
) {
    (*oi).oi_owner = ino;
    (*oi).oi_offset = offset;
    (*oi).oi_flags = 0;
    if whichfork == XFS_ATTR_FORK {
        (*oi).oi_flags |= XFS_OWNER_INFO_ATTR_FORK;
    }
}

#[macro_export]
macro_rules! xfs_rmap_inode_owner {
    ($oi:expr, $ip:expr, $whichfork:expr, $offset:expr) => {
        xfs_rmap_ino_owner($oi, I_INO($ip), $whichfork, $offset)
    };
}

pub unsafe fn xfs_rmap_should_skip_owner_update(oi: *const xfs_owner_info) -> bool {
    (*oi).oi_owner == XFS_RMAP_OWN_NULL
}

/* Reverse mapping functions. */

pub struct xfs_buf;

pub unsafe fn xfs_rmap_irec_offset_pack(irec: *const xfs_rmap_irec) -> u64 {
    let mut x = XFS_RMAP_OFF((*irec).rm_offset);
    if (*irec).rm_flags & XFS_RMAP_ATTR_FORK != 0 { x |= XFS_RMAP_OFF_ATTR_FORK; }
    if (*irec).rm_flags & XFS_RMAP_BMBT_BLOCK != 0 { x |= XFS_RMAP_OFF_BMBT_BLOCK; }
    if (*irec).rm_flags & XFS_RMAP_UNWRITTEN != 0 { x |= XFS_RMAP_OFF_UNWRITTEN; }
    x
}

pub unsafe fn xfs_rmap_irec_offset_unpack(offset: u64, irec: *mut xfs_rmap_irec) -> xfs_failaddr_t {
    if offset & !(XFS_RMAP_OFF_MASK | XFS_RMAP_OFF_FLAGS) != 0 { return __this_address; }
    (*irec).rm_offset = XFS_RMAP_OFF(offset);
    (*irec).rm_flags = 0;
    if offset & XFS_RMAP_OFF_ATTR_FORK != 0 { (*irec).rm_flags |= XFS_RMAP_ATTR_FORK; }
    if offset & XFS_RMAP_OFF_BMBT_BLOCK != 0 { (*irec).rm_flags |= XFS_RMAP_BMBT_BLOCK; }
    if offset & XFS_RMAP_OFF_UNWRITTEN != 0 { (*irec).rm_flags |= XFS_RMAP_UNWRITTEN; }
    ::std::ptr::null_mut()
}

pub unsafe fn xfs_owner_info_unpack(oinfo: *const xfs_owner_info, owner: *mut u64, offset: *mut u64, flags: *mut ::std::os::raw::c_uint) {
    let mut r: ::std::os::raw::c_uint = 0;
    *owner = (*oinfo).oi_owner;
    *offset = (*oinfo).oi_offset;
    if (*oinfo).oi_flags & XFS_OWNER_INFO_ATTR_FORK != 0 { r |= XFS_RMAP_ATTR_FORK; }
    if (*oinfo).oi_flags & XFS_OWNER_INFO_BMBT_BLOCK != 0 { r |= XFS_RMAP_BMBT_BLOCK; }
    *flags = r;
}

pub unsafe fn xfs_owner_info_pack(oinfo: *mut xfs_owner_info, owner: u64, offset: u64, flags: ::std::os::raw::c_uint) {
    (*oinfo).oi_owner = owner;
    (*oinfo).oi_offset = XFS_RMAP_OFF(offset);
    (*oinfo).oi_flags = 0;
    if flags & XFS_RMAP_ATTR_FORK != 0 { (*oinfo).oi_flags |= XFS_OWNER_INFO_ATTR_FORK; }
    if flags & XFS_RMAP_BMBT_BLOCK != 0 { (*oinfo).oi_flags |= XFS_OWNER_INFO_BMBT_BLOCK; }
}

extern "C" {
    pub fn xfs_rmap_alloc(tp: *mut xfs_trans, agbp: *mut xfs_buf, pag: *mut xfs_perag, bno: xfs_agblock_t, len: xfs_extlen_t, oinfo: *const xfs_owner_info) -> ::std::os::raw::c_int;
    pub fn xfs_rmap_free(tp: *mut xfs_trans, agbp: *mut xfs_buf, pag: *mut xfs_perag, bno: xfs_agblock_t, len: xfs_extlen_t, oinfo: *const xfs_owner_info) -> ::std::os::raw::c_int;
    pub fn xfs_rmap_lookup_le(cur: *mut xfs_btree_cur, bno: xfs_agblock_t, owner: u64, offset: u64, flags: ::std::os::raw::c_uint, irec: *mut xfs_rmap_irec, stat: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn xfs_rmap_lookup_eq(cur: *mut xfs_btree_cur, bno: xfs_agblock_t, len: xfs_extlen_t, owner: u64, offset: u64, flags: ::std::os::raw::c_uint, stat: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn xfs_rmap_insert(rcur: *mut xfs_btree_cur, agbno: xfs_agblock_t, len: xfs_extlen_t, owner: u64, offset: u64, flags: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
    pub fn xfs_rmap_get_rec(cur: *mut xfs_btree_cur, irec: *mut xfs_rmap_irec, stat: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}

pub type xfs_rmap_query_range_fn = unsafe extern "C" fn(*mut xfs_btree_cur, *const xfs_rmap_irec, *mut ::std::ffi::c_void) -> ::std::os::raw::c_int;

extern "C" {
    pub fn xfs_rmap_query_range(cur: *mut xfs_btree_cur, low_rec: *const xfs_rmap_irec, high_rec: *const xfs_rmap_irec, fn_: xfs_rmap_query_range_fn, priv_: *mut ::std::ffi::c_void) -> ::std::os::raw::c_int;
    pub fn xfs_rmap_query_all(cur: *mut xfs_btree_cur, fn_: xfs_rmap_query_range_fn, priv_: *mut ::std::ffi::c_void) -> ::std::os::raw::c_int;
}

#[repr(C)]
pub enum xfs_rmap_intent_type { XFS_RMAP_MAP, XFS_RMAP_MAP_SHARED, XFS_RMAP_UNMAP, XFS_RMAP_UNMAP_SHARED, XFS_RMAP_CONVERT, XFS_RMAP_CONVERT_SHARED, XFS_RMAP_ALLOC, XFS_RMAP_FREE }

#[macro_export]
macro_rules! XFS_RMAP_INTENT_STRINGS { () => { [(XFS_RMAP_MAP, "map"), (XFS_RMAP_MAP_SHARED, "map_shared"), (XFS_RMAP_UNMAP, "unmap"), (XFS_RMAP_UNMAP_SHARED, "unmap_shared"), (XFS_RMAP_CONVERT, "cvt"), (XFS_RMAP_CONVERT_SHARED, "cvt_shared"), (XFS_RMAP_ALLOC, "alloc"), (XFS_RMAP_FREE, "free")] }; }

#[repr(C)]
pub struct xfs_rmap_intent {
    pub ri_list: list_head,
    pub ri_type: xfs_rmap_intent_type,
    pub ri_whichfork: ::std::os::raw::c_int,
    pub ri_owner: u64,
    pub ri_bmap: xfs_bmbt_irec,
    pub ri_group: *mut xfs_group,
    pub ri_realtime: bool,
}

/* functions for updating the rmapbt based on bmbt map/unmap operations */
extern "C" {
    pub fn xfs_rmap_map_extent(tp: *mut xfs_trans, ip: *mut xfs_inode, whichfork: ::std::os::raw::c_int, imap: *mut xfs_bmbt_irec);
    pub fn xfs_rmap_unmap_extent(tp: *mut xfs_trans, ip: *mut xfs_inode, whichfork: ::std::os::raw::c_int, imap: *mut xfs_bmbt_irec);
    pub fn xfs_rmap_convert_extent(mp: *mut xfs_mount, tp: *mut xfs_trans, ip: *mut xfs_inode, whichfork: ::std::os::raw::c_int, imap: *mut xfs_bmbt_irec);
    pub fn xfs_rmap_alloc_extent(tp: *mut xfs_trans, isrt: bool, fsbno: xfs_fsblock_t, len: xfs_extlen_t, owner: u64);
    pub fn xfs_rmap_free_extent(tp: *mut xfs_trans, isrt: bool, fsbno: xfs_fsblock_t, len: xfs_extlen_t, owner: u64);
    pub fn xfs_rmap_finish_one(tp: *mut xfs_trans, ri: *mut xfs_rmap_intent, pcur: *mut *mut xfs_btree_cur) -> ::std::os::raw::c_int;
    pub fn __xfs_rmap_finish_intent(rcur: *mut xfs_btree_cur, op: xfs_rmap_intent_type, bno: xfs_agblock_t, len: xfs_extlen_t, oinfo: *const xfs_owner_info, unwritten: bool) -> ::std::os::raw::c_int;
    pub fn xfs_rmap_lookup_le_range(cur: *mut xfs_btree_cur, bno: xfs_agblock_t, owner: u64, offset: u64, flags: ::std::os::raw::c_uint, irec: *mut xfs_rmap_irec, stat: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn xfs_rmap_compare(a: *const xfs_rmap_irec, b: *const xfs_rmap_irec) -> ::std::os::raw::c_int;
    pub fn xfs_rmap_btrec_to_irec(rec: *const xfs_btree_rec, irec: *mut xfs_rmap_irec) -> xfs_failaddr_t;
    pub fn xfs_rmap_check_irec(pag: *mut xfs_perag, irec: *const xfs_rmap_irec) -> xfs_failaddr_t;
    pub fn xfs_rtrmap_check_irec(rtg: *mut xfs_rtgroup, irec: *const xfs_rmap_irec) -> xfs_failaddr_t;
    pub fn xfs_rmap_has_records(cur: *mut xfs_btree_cur, bno: xfs_agblock_t, len: xfs_extlen_t, outcome: *mut xbtree_recpacking) -> ::std::os::raw::c_int;
}

pub union xfs_btree_rec { _bindgen_opaque_blob: [u8; 0] }

#[repr(C)]
pub struct xfs_rmap_matches {
    /* Number of owner matches. */
    pub matches: u64,
    /* Number of non-owner matches. */
    pub non_owner_matches: u64,
    /* Number of non-owner matches that conflict with the owner matches. */
    pub bad_non_owner_matches: u64,
}

extern "C" {
    pub fn xfs_rmap_count_owners(cur: *mut xfs_btree_cur, bno: xfs_agblock_t, len: xfs_extlen_t, oinfo: *const xfs_owner_info, rmatch: *mut xfs_rmap_matches) -> ::std::os::raw::c_int;
    pub fn xfs_rmap_has_other_keys(cur: *mut xfs_btree_cur, bno: xfs_agblock_t, len: xfs_extlen_t, oinfo: *const xfs_owner_info, has_other: *mut bool) -> ::std::os::raw::c_int;
    pub fn xfs_rmap_map_raw(cur: *mut xfs_btree_cur, rmap: *mut xfs_rmap_irec) -> ::std::os::raw::c_int;
    pub static XFS_RMAP_OINFO_SKIP_UPDATE: xfs_owner_info;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
