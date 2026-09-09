/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2000-2006 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// C header dependencies are supplied by the surrounding translation unit.

pub struct getbmap;
pub struct xfs_bmbt_irec;
pub struct xfs_ifork;
pub struct xfs_inode;
pub struct xfs_mount;
pub struct xfs_trans;
pub struct xfs_alloc_arg;
pub struct xfs_btree_cur;
pub struct xfs_iext_cursor;
pub struct xfs_buf;
pub struct xfs_perag;
pub struct iomap;
pub struct list_head;
pub struct xfs_group;
pub struct kmem_cache;

// Argument structure for xfs_bmap_alloc.
#[repr(C)]
pub struct xfs_bmalloca {
    pub tp: *mut xfs_trans,
    pub ip: *mut xfs_inode,
    pub prev: xfs_bmbt_irec,
    pub got: xfs_bmbt_irec,
    pub offset: xfs_fileoff_t,
    pub length: xfs_extlen_t,
    pub blkno: xfs_fsblock_t,
    pub cur: *mut xfs_btree_cur,
    pub icur: xfs_iext_cursor,
    pub nallocs: ::core::ffi::c_int,
    pub logflags: ::core::ffi::c_int,
    pub total: xfs_extlen_t,
    pub minlen: xfs_extlen_t,
    pub minleft: xfs_extlen_t,
    pub eof: bool,
    pub wasdel: bool,
    pub aeof: bool,
    pub conv: bool,
    pub datatype: ::core::ffi::c_int,
    pub flags: u32,
}

pub const XFS_BMAP_MAX_NMAP: u32 = 4;
pub const XFS_BMAPI_ENTIRE: u32 = 1u32 << 0;
pub const XFS_BMAPI_METADATA: u32 = 1u32 << 1;
pub const XFS_BMAPI_ATTRFORK: u32 = 1u32 << 2;
pub const XFS_BMAPI_PREALLOC: u32 = 1u32 << 3;
pub const XFS_BMAPI_CONTIG: u32 = 1u32 << 4;
pub const XFS_BMAPI_CONVERT: u32 = 1u32 << 5;
pub const XFS_BMAPI_ZERO: u32 = 1u32 << 6;
pub const XFS_BMAPI_REMAP: u32 = 1u32 << 7;
pub const XFS_BMAPI_COWFORK: u32 = 1u32 << 8;
pub const XFS_BMAPI_NODISCARD: u32 = 1u32 << 9;
pub const XFS_BMAPI_NORMAP: u32 = 1u32 << 10;
pub const XFS_BMAPI_EXTSZALIGN: u32 = 1u32 << 11;

pub const XFS_BMAPI_FLAGS: &[(u32, &str)] = &[
    (XFS_BMAPI_ENTIRE, "ENTIRE"), (XFS_BMAPI_METADATA, "METADATA"),
    (XFS_BMAPI_ATTRFORK, "ATTRFORK"), (XFS_BMAPI_PREALLOC, "PREALLOC"),
    (XFS_BMAPI_CONTIG, "CONTIG"), (XFS_BMAPI_CONVERT, "CONVERT"),
    (XFS_BMAPI_ZERO, "ZERO"), (XFS_BMAPI_REMAP, "REMAP"),
    (XFS_BMAPI_COWFORK, "COWFORK"), (XFS_BMAPI_NODISCARD, "NODISCARD"),
    (XFS_BMAPI_NORMAP, "NORMAP"), (XFS_BMAPI_EXTSZALIGN, "EXTSZALIGN"),
];

#[inline]
pub unsafe fn xfs_bmapi_aflag(w: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if w == XFS_ATTR_FORK { XFS_BMAPI_ATTRFORK as _ }
    else if w == XFS_COW_FORK { XFS_BMAPI_COWFORK as _ } else { 0 }
}

#[inline]
pub unsafe fn xfs_bmapi_whichfork(bmapi_flags: u32) -> ::core::ffi::c_int {
    if bmapi_flags & XFS_BMAPI_COWFORK != 0 { XFS_COW_FORK }
    else if bmapi_flags & XFS_BMAPI_ATTRFORK != 0 { XFS_ATTR_FORK }
    else { XFS_DATA_FORK }
}

pub unsafe fn xfs_bmap_alloc_account(ap: *mut xfs_bmalloca);

pub const DELAYSTARTBLOCK: xfs_fsblock_t = (-1i64) as xfs_fsblock_t;
pub const HOLESTARTBLOCK: xfs_fsblock_t = (-2i64) as xfs_fsblock_t;

pub const BMAP_LEFT_CONTIG: u32 = 1u32 << 0;
pub const BMAP_RIGHT_CONTIG: u32 = 1u32 << 1;
pub const BMAP_LEFT_FILLING: u32 = 1u32 << 2;
pub const BMAP_RIGHT_FILLING: u32 = 1u32 << 3;
pub const BMAP_LEFT_DELAY: u32 = 1u32 << 4;
pub const BMAP_RIGHT_DELAY: u32 = 1u32 << 5;
pub const BMAP_LEFT_VALID: u32 = 1u32 << 6;
pub const BMAP_RIGHT_VALID: u32 = 1u32 << 7;
pub const BMAP_ATTRFORK: u32 = 1u32 << 8;
pub const BMAP_COWFORK: u32 = 1u32 << 9;
pub const XFS_BMAP_EXT_FLAGS: &[(u32, &str)] = &[
    (BMAP_LEFT_CONTIG, "LC"), (BMAP_RIGHT_CONTIG, "RC"),
    (BMAP_LEFT_FILLING, "LF"), (BMAP_RIGHT_FILLING, "RF"),
    (BMAP_ATTRFORK, "ATTR"), (BMAP_COWFORK, "COW"),
];

#[inline]
pub unsafe fn xfs_bmap_is_real_extent(irec: *const xfs_bmbt_irec) -> bool {
    (*irec).br_startblock != HOLESTARTBLOCK &&
        (*irec).br_startblock != DELAYSTARTBLOCK &&
        !isnullstartblock((*irec).br_startblock)
}

#[inline]
pub unsafe fn xfs_bmap_is_written_extent(irec: *const xfs_bmbt_irec) -> bool {
    xfs_bmap_is_real_extent(irec) && (*irec).br_state != XFS_EXT_UNWRITTEN
}

#[inline]
pub unsafe fn xfs_valid_startblock(ip: *const xfs_inode, startblock: xfs_fsblock_t) -> bool {
    startblock != 0 || XFS_IS_REALTIME_INODE(ip)
}

pub unsafe fn xfs_bmap_longest_free_extent(pag: *mut xfs_perag, tp: *mut xfs_trans, blen: *mut xfs_extlen_t) -> ::core::ffi::c_int;
pub unsafe fn xfs_trim_extent(irec: *mut xfs_bmbt_irec, bno: xfs_fileoff_t, len: xfs_filblks_t);
pub unsafe fn xfs_bmap_compute_attr_offset(mp: *mut xfs_mount) -> u32;
pub unsafe fn xfs_bmap_add_attrfork(tp: *mut xfs_trans, ip: *mut xfs_inode, size: ::core::ffi::c_int, rsvd: ::core::ffi::c_int) -> ::core::ffi::c_int;
pub unsafe fn xfs_bmap_local_to_extents_empty(tp: *mut xfs_trans, ip: *mut xfs_inode, whichfork: ::core::ffi::c_int);
pub unsafe fn xfs_bmap_local_to_extents(tp: *mut xfs_trans, ip: *mut xfs_inode, total: xfs_extlen_t, logflagsp: *mut ::core::ffi::c_int, whichfork: ::core::ffi::c_int, init_fn: Option<unsafe extern "C" fn(*mut xfs_trans, *mut xfs_buf, *mut xfs_inode, *mut xfs_ifork, *mut ::core::ffi::c_void)>, priv_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;

pub unsafe fn xfs_bmap_compute_maxlevels(mp: *mut xfs_mount, whichfork: ::core::ffi::c_int);
pub unsafe fn xfs_bmap_first_unused(tp: *mut xfs_trans, ip: *mut xfs_inode, len: xfs_extlen_t, unused: *mut xfs_fileoff_t, whichfork: ::core::ffi::c_int) -> ::core::ffi::c_int;
pub unsafe fn xfs_bmap_last_before(tp: *mut xfs_trans, ip: *mut xfs_inode, last_block: *mut xfs_fileoff_t, whichfork: ::core::ffi::c_int) -> ::core::ffi::c_int;
pub unsafe fn xfs_bmap_last_offset(ip: *mut xfs_inode, unused: *mut xfs_fileoff_t, whichfork: ::core::ffi::c_int) -> ::core::ffi::c_int;
pub unsafe fn xfs_bmapi_read(ip: *mut xfs_inode, bno: xfs_fileoff_t, len: xfs_filblks_t, mval: *mut xfs_bmbt_irec, nmap: *mut ::core::ffi::c_int, flags: u32) -> ::core::ffi::c_int;
pub unsafe fn xfs_bmapi_write(tp: *mut xfs_trans, ip: *mut xfs_inode, bno: xfs_fileoff_t, len: xfs_filblks_t, flags: u32, total: xfs_extlen_t, mval: *mut xfs_bmbt_irec, nmap: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
pub unsafe fn xfs_bunmapi(tp: *mut xfs_trans, ip: *mut xfs_inode, bno: xfs_fileoff_t, len: xfs_filblks_t, flags: u32, nexts: xfs_extnum_t, done: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
pub unsafe fn xfs_bmap_del_extent_delay(ip: *mut xfs_inode, whichfork: ::core::ffi::c_int, cur: *mut xfs_iext_cursor, got: *mut xfs_bmbt_irec, del: *mut xfs_bmbt_irec, bflags: u32);
pub unsafe fn xfs_bmap_del_extent_cow(ip: *mut xfs_inode, cur: *mut xfs_iext_cursor, got: *mut xfs_bmbt_irec, del: *mut xfs_bmbt_irec);
pub unsafe fn xfs_default_attroffset(ip: *mut xfs_inode) -> ::core::ffi::c_uint;
pub unsafe fn xfs_bmap_collapse_extents(tp: *mut xfs_trans, ip: *mut xfs_inode, next_fsb: *mut xfs_fileoff_t, offset_shift_fsb: xfs_fileoff_t, done: *mut bool) -> ::core::ffi::c_int;
pub unsafe fn xfs_bmap_can_insert_extents(ip: *mut xfs_inode, off: xfs_fileoff_t, shift: xfs_fileoff_t) -> ::core::ffi::c_int;
pub unsafe fn xfs_bmap_insert_extents(tp: *mut xfs_trans, ip: *mut xfs_inode, next_fsb: *mut xfs_fileoff_t, offset_shift_fsb: xfs_fileoff_t, done: *mut bool, stop_fsb: xfs_fileoff_t) -> ::core::ffi::c_int;
pub unsafe fn xfs_bmap_split_extent(tp: *mut xfs_trans, ip: *mut xfs_inode, split_offset: xfs_fileoff_t) -> ::core::ffi::c_int;
pub unsafe fn xfs_bmapi_convert_delalloc(ip: *mut xfs_inode, whichfork: ::core::ffi::c_int, offset: xfs_off_t, iomap: *mut iomap, seq: *mut u32) -> ::core::ffi::c_int;
pub unsafe fn xfs_bmap_add_extent_unwritten_real(tp: *mut xfs_trans, ip: *mut xfs_inode, whichfork: ::core::ffi::c_int, icur: *mut xfs_iext_cursor, curp: *mut *mut xfs_btree_cur, new_: *mut xfs_bmbt_irec, logflagsp: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
pub unsafe fn xfs_bmapi_minleft(tp: *mut xfs_trans, ip: *mut xfs_inode, fork: ::core::ffi::c_int) -> xfs_extlen_t;
pub unsafe fn xfs_bmap_btalloc_low_space(ap: *mut xfs_bmalloca, args: *mut xfs_alloc_arg) -> ::core::ffi::c_int;
pub unsafe fn xfs_bmap_worst_indlen(ip: *mut xfs_inode, len: xfs_filblks_t) -> xfs_filblks_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum xfs_bmap_intent_type { XFS_BMAP_MAP = 1, XFS_BMAP_UNMAP }
pub const XFS_BMAP_INTENT_STRINGS: &[(xfs_bmap_intent_type, &str)] = &[(xfs_bmap_intent_type::XFS_BMAP_MAP, "map"), (xfs_bmap_intent_type::XFS_BMAP_UNMAP, "unmap")];

#[repr(C)]
pub struct xfs_bmap_intent { pub bi_list: list_head, pub bi_type: xfs_bmap_intent_type, pub bi_whichfork: ::core::ffi::c_int, pub bi_owner: *mut xfs_inode, pub bi_group: *mut xfs_group, pub bi_bmap: xfs_bmbt_irec }

pub unsafe fn xfs_bmap_finish_one(tp: *mut xfs_trans, bi: *mut xfs_bmap_intent) -> ::core::ffi::c_int;
pub unsafe fn xfs_bmap_map_extent(tp: *mut xfs_trans, ip: *mut xfs_inode, whichfork: ::core::ffi::c_int, imap: *mut xfs_bmbt_irec);
pub unsafe fn xfs_bmap_unmap_extent(tp: *mut xfs_trans, ip: *mut xfs_inode, whichfork: ::core::ffi::c_int, imap: *mut xfs_bmbt_irec);

#[inline]
pub fn xfs_bmap_fork_to_state(whichfork: ::core::ffi::c_int) -> u32 {
    match whichfork { XFS_ATTR_FORK => BMAP_ATTRFORK, XFS_COW_FORK => BMAP_COWFORK, _ => 0 }
}

pub unsafe fn xfs_bmap_validate_extent_raw(mp: *mut xfs_mount, rtfile: bool, whichfork: ::core::ffi::c_int, irec: *mut xfs_bmbt_irec) -> xfs_failaddr_t;
pub unsafe fn xfs_bmap_validate_extent(ip: *mut xfs_inode, whichfork: ::core::ffi::c_int, irec: *mut xfs_bmbt_irec) -> xfs_failaddr_t;
pub unsafe fn xfs_bmap_complain_bad_rec(ip: *mut xfs_inode, whichfork: ::core::ffi::c_int, fa: xfs_failaddr_t, irec: *const xfs_bmbt_irec) -> ::core::ffi::c_int;
pub unsafe fn xfs_bmapi_remap(tp: *mut xfs_trans, ip: *mut xfs_inode, bno: xfs_fileoff_t, len: xfs_filblks_t, startblock: xfs_fsblock_t, flags: u32) -> ::core::ffi::c_int;
pub unsafe fn xfs_bunmapi_range(tpp: *mut *mut xfs_trans, ip: *mut xfs_inode, flags: u32, startoff: xfs_fileoff_t, endoff: xfs_fileoff_t) -> ::core::ffi::c_int;
pub static mut xfs_bmap_intent_cache: *mut kmem_cache;
pub unsafe fn xfs_bmap_intent_init_cache() -> ::core::ffi::c_int;
pub unsafe fn xfs_bmap_intent_destroy_cache();

pub type xfs_bmap_query_range_fn = unsafe extern "C" fn(*mut xfs_btree_cur, *mut xfs_bmbt_irec, *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
pub unsafe fn xfs_bmap_query_all(cur: *mut xfs_btree_cur, fn_: xfs_bmap_query_range_fn, priv_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
pub unsafe fn xfs_get_extsz_hint(ip: *mut xfs_inode) -> xfs_extlen_t;
pub unsafe fn xfs_get_cowextsz_hint(ip: *mut xfs_inode) -> xfs_extlen_t;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
