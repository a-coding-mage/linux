/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2000-2002,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// C header dependencies and conditional compilation are supplied by the surrounding build.

use core::ffi::c_void;

#[repr(C)] pub struct xfs_buf { pub b_mount: *mut xfs_mount, pub b_addr: *mut u8 }
#[repr(C)] pub struct xfs_btree_cur;
#[repr(C)] pub struct xfs_mount;
#[repr(C)] pub struct xfs_perag;
#[repr(C)] pub struct xfs_trans;
#[repr(C)] pub struct workqueue_struct;
#[repr(C)] pub struct xfs_owner_info;
#[repr(C)] pub struct xfs_alloc_rec_incore;
#[repr(C)] pub struct xfs_agf;
#[repr(C)] pub struct xfs_group;
#[repr(C)] pub struct xfs_defer_pending;
#[repr(C)] pub struct kmem_cache;
#[repr(C)] pub struct list_head;
#[repr(C)] pub union xfs_btree_rec { _opaque: u64 }
#[repr(C)] pub struct xfs_agfl;
pub type xfs_fsblock_t = u64;
pub type xfs_agnumber_t = u32;
pub type xfs_agblock_t = u32;
pub type xfs_extlen_t = u32;
pub type xfs_filblks_t = u64;
pub type xfs_failaddr_t = *mut c_void;
pub type xfs_ag_resv_type = i32;
pub type xbtree_recpacking = i32;
pub type __be32 = u32;

extern "C" {
    pub static mut xfs_alloc_wq: *mut workqueue_struct;

    pub fn xfs_agfl_size(mp: *mut xfs_mount) -> u32;
    pub fn xfs_alloc_set_aside(mp: *mut xfs_mount) -> u32;
    pub fn xfs_alloc_ag_max_usable(mp: *mut xfs_mount) -> u32;
    pub fn xfs_alloc_longest_free_extent(pag: *mut xfs_perag, need: xfs_extlen_t, reserved: xfs_extlen_t) -> xfs_extlen_t;
    pub fn xfs_alloc_min_freelist(mp: *mut xfs_mount, pag: *mut xfs_perag) -> u32;
    pub fn xfs_alloc_get_freelist(pag: *mut xfs_perag, tp: *mut xfs_trans, agfbp: *mut xfs_buf, bnop: *mut xfs_agblock_t, btreeblk: i32) -> i32;
    pub fn xfs_alloc_put_freelist(pag: *mut xfs_perag, tp: *mut xfs_trans, agfbp: *mut xfs_buf, agflbp: *mut xfs_buf, bno: xfs_agblock_t, btreeblk: i32) -> i32;
    pub fn xfs_free_ag_extent(tp: *mut xfs_trans, agbp: *mut xfs_buf, bno: xfs_agblock_t, len: xfs_extlen_t, oinfo: *const xfs_owner_info, type_: xfs_ag_resv_type) -> i32;
    pub fn xfs_alloc_compute_maxlevels(mp: *mut xfs_mount);
    pub fn xfs_alloc_log_agf(tp: *mut xfs_trans, bp: *mut xfs_buf, fields: u32);
    pub fn xfs_alloc_vextent_this_ag(args: *mut xfs_alloc_arg, agno: xfs_agnumber_t) -> i32;
    pub fn xfs_alloc_vextent_near_bno(args: *mut xfs_alloc_arg, target: xfs_fsblock_t) -> i32;
    pub fn xfs_alloc_vextent_exact_bno(args: *mut xfs_alloc_arg, target: xfs_fsblock_t) -> i32;
    pub fn xfs_alloc_vextent_start_ag(args: *mut xfs_alloc_arg, target: xfs_fsblock_t) -> i32;
    pub fn xfs_alloc_vextent_first_ag(args: *mut xfs_alloc_arg, target: xfs_fsblock_t) -> i32;
    pub fn __xfs_free_extent(tp: *mut xfs_trans, pag: *mut xfs_perag, agbno: xfs_agblock_t, len: xfs_extlen_t, oinfo: *const xfs_owner_info, type_: xfs_ag_resv_type, skip_discard: bool) -> i32;
    pub fn xfs_alloc_lookup_le(cur: *mut xfs_btree_cur, bno: xfs_agblock_t, len: xfs_extlen_t, stat: *mut i32) -> i32;
    pub fn xfs_alloc_lookup_ge(cur: *mut xfs_btree_cur, bno: xfs_agblock_t, len: xfs_extlen_t, stat: *mut i32) -> i32;
    pub fn xfs_alloc_get_rec(cur: *mut xfs_btree_cur, bno: *mut xfs_agblock_t, len: *mut xfs_extlen_t, stat: *mut i32) -> i32;
    pub fn xfs_alloc_btrec_to_irec(rec: *const xfs_btree_rec, irec: *mut xfs_alloc_rec_incore);
    pub fn xfs_alloc_check_irec(pag: *mut xfs_perag, irec: *const xfs_alloc_rec_incore) -> xfs_failaddr_t;
    pub fn xfs_read_agf(pag: *mut xfs_perag, tp: *mut xfs_trans, flags: i32, agfbpp: *mut *mut xfs_buf) -> i32;
    pub fn xfs_alloc_read_agf(pag: *mut xfs_perag, tp: *mut xfs_trans, flags: i32, agfbpp: *mut *mut xfs_buf) -> i32;
    pub fn xfs_alloc_read_agfl(pag: *mut xfs_perag, tp: *mut xfs_trans, bpp: *mut *mut xfs_buf) -> i32;
    pub fn xfs_alloc_fix_freelist(args: *mut xfs_alloc_arg, alloc_flags: u32) -> i32;
    pub fn xfs_free_extent_fix_freelist(tp: *mut xfs_trans, pag: *mut xfs_perag, agbp: *mut *mut xfs_buf) -> i32;
    pub fn xfs_prealloc_blocks(mp: *mut xfs_mount) -> xfs_extlen_t;
    pub fn xfs_alloc_query_range(cur: *mut xfs_btree_cur, low_rec: *const xfs_alloc_rec_incore, high_rec: *const xfs_alloc_rec_incore, fn_: xfs_alloc_query_range_fn, priv_: *mut c_void) -> i32;
    pub fn xfs_alloc_query_all(cur: *mut xfs_btree_cur, fn_: xfs_alloc_query_range_fn, priv_: *mut c_void) -> i32;
    pub fn xfs_alloc_has_records(cur: *mut xfs_btree_cur, bno: xfs_agblock_t, len: xfs_extlen_t, outcome: *mut xbtree_recpacking) -> i32;
    pub fn xfs_agfl_walk(mp: *mut xfs_mount, agf: *mut xfs_agf, agflbp: *mut xfs_buf, walk_fn: xfs_agfl_walk_fn, priv_: *mut c_void) -> i32;
    pub fn xfs_free_extent_later(tp: *mut xfs_trans, bno: xfs_fsblock_t, len: xfs_filblks_t, oinfo: *const xfs_owner_info, type_: xfs_ag_resv_type, free_flags: u32) -> i32;
    pub fn xfs_alloc_schedule_autoreap(args: *const xfs_alloc_arg, free_flags: u32, aarp: *mut xfs_alloc_autoreap) -> i32;
    pub fn xfs_alloc_cancel_autoreap(tp: *mut xfs_trans, aarp: *mut xfs_alloc_autoreap);
    pub fn xfs_alloc_commit_autoreap(tp: *mut xfs_trans, aarp: *mut xfs_alloc_autoreap);
    pub static mut xfs_extfree_item_cache: *mut kmem_cache;
    pub fn xfs_extfree_intent_init_cache() -> i32;
    pub fn xfs_extfree_intent_destroy_cache();
    pub fn xfs_validate_ag_length(bp: *mut xfs_buf, seqno: u32, length: u32) -> xfs_failaddr_t;
}

pub const XFS_ALLOC_FLAG_TRYLOCK: u32 = 1U << 0;
pub const XFS_ALLOC_FLAG_FREEING: u32 = 1U << 1;
pub const XFS_ALLOC_FLAG_NORMAP: u32 = 1U << 2;
pub const XFS_ALLOC_FLAG_NOSHRINK: u32 = 1U << 3;
pub const XFS_ALLOC_FLAG_CHECK: u32 = 1U << 4;
pub const XFS_ALLOC_FLAG_TRYFLUSH: u32 = 1U << 5;

#[repr(C)]
pub struct xfs_alloc_arg {
    pub tp: *mut xfs_trans, pub mp: *mut xfs_mount, pub agbp: *mut xfs_buf, pub pag: *mut xfs_perag,
    pub fsbno: xfs_fsblock_t, pub agno: xfs_agnumber_t, pub agbno: xfs_agblock_t,
    pub minlen: xfs_extlen_t, pub maxlen: xfs_extlen_t, pub r#mod: xfs_extlen_t, pub prod: xfs_extlen_t,
    pub minleft: xfs_extlen_t, pub total: xfs_extlen_t, pub alignment: xfs_extlen_t, pub minalignslop: xfs_extlen_t,
    pub min_agbno: xfs_agblock_t, pub max_agbno: xfs_agblock_t, pub len: xfs_extlen_t,
    pub datatype: i32, pub wasdel: i8, pub wasfromfl: i8, pub alloc_minlen_only: bool,
    pub oinfo: xfs_owner_info, pub resv: xfs_ag_resv_type,
}
pub type xfs_alloc_arg_t = xfs_alloc_arg;
pub const XFS_ALLOC_USERDATA: i32 = 1 << 0;
pub const XFS_ALLOC_INITIAL_USER_DATA: i32 = 1 << 1;
pub const XFS_ALLOC_NOBUSY: i32 = 1 << 2;

#[inline]
pub unsafe fn xfs_free_extent(tp: *mut xfs_trans, pag: *mut xfs_perag, agbno: xfs_agblock_t, len: xfs_extlen_t, oinfo: *const xfs_owner_info, type_: xfs_ag_resv_type) -> i32 { __xfs_free_extent(tp, pag, agbno, len, oinfo, type_, false) }

pub type xfs_alloc_query_range_fn = Option<unsafe extern "C" fn(*mut xfs_btree_cur, *const xfs_alloc_rec_incore, *mut c_void) -> i32>;
pub type xfs_agfl_walk_fn = Option<unsafe extern "C" fn(*mut xfs_mount, xfs_agblock_t, *mut c_void) -> i32>;

#[inline]
pub unsafe fn xfs_buf_to_agfl_bno(bp: *mut xfs_buf) -> *mut __be32 {
    // xfs_has_crc is supplied by the surrounding XFS implementation.
    if xfs_has_crc((*bp).b_mount) { (*bp).b_addr.add(core::mem::size_of::<xfs_agfl>()) as *mut __be32 } else { (*bp).b_addr as *mut __be32 }
}
extern "C" { pub fn xfs_has_crc(mp: *mut xfs_mount) -> bool; }

pub const XFS_FREE_EXTENT_SKIP_DISCARD: u32 = 1U << 0;
pub const XFS_FREE_EXTENT_REALTIME: u32 = 1U << 1;
pub const XFS_FREE_EXTENT_ALL_FLAGS: u32 = XFS_FREE_EXTENT_SKIP_DISCARD | XFS_FREE_EXTENT_REALTIME;

#[repr(C)] pub struct xfs_extent_free_item { pub xefi_list: list_head, pub xefi_owner: u64, pub xefi_startblock: xfs_fsblock_t, pub xefi_blockcount: xfs_extlen_t, pub xefi_group: *mut xfs_group, pub xefi_flags: u32, pub xefi_agresv: xfs_ag_resv_type }
pub const XFS_EFI_SKIP_DISCARD: u32 = 1U << 0;
pub const XFS_EFI_ATTR_FORK: u32 = 1U << 1;
pub const XFS_EFI_BMBT_BLOCK: u32 = 1U << 2;
pub const XFS_EFI_CANCELLED: u32 = 1U << 3;
pub const XFS_EFI_REALTIME: u32 = 1U << 4;
#[inline] pub unsafe fn xfs_efi_is_realtime(xefi: *const xfs_extent_free_item) -> bool { ((*xefi).xefi_flags & XFS_EFI_REALTIME) != 0 }

#[repr(C)] pub struct xfs_alloc_autoreap { pub dfp: *mut xfs_defer_pending }

pub const XFS_ALLOC_HEADER_TRANSLATION_NOTE: &str = "C header guard and includes omitted; external types are supplied by dependencies.";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
