// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2002,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

/* kernel only transaction subsystem defines */

#[repr(C)]
pub struct xlog { _private: [u8; 0] }
#[repr(C)]
pub struct xlog_format_buf { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_buf { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_buftarg { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_efd_log_item { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_efi_log_item { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_inode { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_mount { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_trans_res { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_dquot_acct { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_rud_log_item { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_rui_log_item { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_btree_cur { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_cui_log_item { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_cud_log_item { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_bui_log_item { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_bud_log_item { _private: [u8; 0] }
#[repr(C)] pub struct xfs_ail { _private: [u8; 0] }
#[repr(C)] pub struct xfs_log_vec { _private: [u8; 0] }
#[repr(C)] pub struct xfs_dquot { _private: [u8; 0] }

#[repr(C)]
pub struct xfs_log_item {
    pub li_ail: list_head,
    pub li_trans: list_head,
    pub li_lsn: xfs_lsn_t,
    pub li_log: *mut xlog,
    pub li_ailp: *mut xfs_ail,
    pub li_type: uint,
    pub li_flags: c_ulong,
    pub li_buf: *mut xfs_buf,
    pub li_bio_list: list_head,
    pub li_ops: *const xfs_item_ops,
    pub li_cil: list_head,
    pub li_lv: *mut xfs_log_vec,
    pub li_lv_shadow: *mut xfs_log_vec,
    pub li_seq: xfs_csn_t,
    pub li_order_id: u32,
}

pub const XFS_LI_IN_AIL: u32 = 0;
pub const XFS_LI_ABORTED: u32 = 1;
pub const XFS_LI_FAILED: u32 = 2;
pub const XFS_LI_DIRTY: u32 = 3;
pub const XFS_LI_WHITEOUT: u32 = 4;
pub const XFS_LI_FLUSHING: u32 = 5;
/* XFS_LI_FLAGS is a C initializer list; retain its source-level meaning. */
pub const XFS_LI_FLAGS: &[(u32, &str)] = &[
    (1u32 << XFS_LI_IN_AIL, "IN_AIL"), (1u32 << XFS_LI_ABORTED, "ABORTED"),
    (1u32 << XFS_LI_FAILED, "FAILED"), (1u32 << XFS_LI_DIRTY, "DIRTY"),
    (1u32 << XFS_LI_WHITEOUT, "WHITEOUT"), (1u32 << XFS_LI_FLUSHING, "FLUSHING"),
];

#[repr(C)]
pub struct xfs_item_ops {
    pub flags: c_uint,
    pub iop_size: Option<unsafe extern "C" fn(*mut xfs_log_item, *mut c_int, *mut c_int)>,
    pub iop_format: Option<unsafe extern "C" fn(*mut xfs_log_item, *mut xlog_format_buf)>,
    pub iop_pin: Option<unsafe extern "C" fn(*mut xfs_log_item)>,
    pub iop_unpin: Option<unsafe extern "C" fn(*mut xfs_log_item, c_int)>,
    pub iop_sort: Option<unsafe extern "C" fn(*mut xfs_log_item) -> u64>,
    pub iop_precommit: Option<unsafe extern "C" fn(*mut xfs_trans, *mut xfs_log_item) -> c_int>,
    pub iop_committing: Option<unsafe extern "C" fn(*mut xfs_log_item, xfs_csn_t)>,
    pub iop_committed: Option<unsafe extern "C" fn(*mut xfs_log_item, xfs_lsn_t) -> xfs_lsn_t>,
    pub iop_push: Option<unsafe extern "C" fn(*mut xfs_log_item, *mut list_head) -> uint>,
    pub iop_release: Option<unsafe extern "C" fn(*mut xfs_log_item)>,
    pub iop_match: Option<unsafe extern "C" fn(*mut xfs_log_item, u64) -> bool>,
    pub iop_intent: Option<unsafe extern "C" fn(*mut xfs_log_item) -> *mut xfs_log_item>,
}

pub const XFS_ITEM_RELEASE_WHEN_COMMITTED: c_uint = 1 << 0;
pub const XFS_ITEM_INTENT: c_uint = 1 << 1;
pub const XFS_ITEM_INTENT_DONE: c_uint = 1 << 2;

#[inline]
pub unsafe fn xlog_item_is_intent(lip: *mut xfs_log_item) -> bool { (*(*lip).li_ops).flags & XFS_ITEM_INTENT != 0 }
#[inline]
pub unsafe fn xlog_item_is_intent_done(lip: *mut xfs_log_item) -> bool { (*(*lip).li_ops).flags & XFS_ITEM_INTENT_DONE != 0 }

pub const XFS_ITEM_SUCCESS: c_int = 0;
pub const XFS_ITEM_PINNED: c_int = 1;
pub const XFS_ITEM_LOCKED: c_int = 2;
pub const XFS_ITEM_FLUSHING: c_int = 3;

#[repr(C)]
pub struct xfs_trans {
    pub t_log_res: c_uint, pub t_log_count: c_uint, pub t_blk_res: c_uint,
    pub t_blk_res_used: c_uint, pub t_rtx_res: c_uint, pub t_rtx_res_used: c_uint,
    pub t_flags: c_uint, pub t_highest_agno: xfs_agnumber_t,
    pub t_ticket: *mut xlog_ticket, pub t_mountp: *mut xfs_mount,
    pub t_dqinfo: *mut xfs_dquot_acct,
    pub t_icount_delta: i64, pub t_ifree_delta: i64, pub t_fdblocks_delta: i64,
    pub t_res_fdblocks_delta: i64, pub t_frextents_delta: i64,
    pub t_res_frextents_delta: i64, pub t_dblocks_delta: i64, pub t_agcount_delta: i64,
    pub t_imaxpct_delta: i64, pub t_rextsize_delta: i64, pub t_rbmblocks_delta: i64,
    pub t_rblocks_delta: i64, pub t_rextents_delta: i64, pub t_rextslog_delta: i64,
    pub t_rgcount_delta: i64, pub t_items: list_head, pub t_busy: list_head,
    pub t_dfops: list_head, pub t_pflags: c_ulong,
}
pub type xfs_trans_t = xfs_trans;

/* XFS transaction mechanism exported interfaces that are actually macros. */
#[inline] pub unsafe fn xfs_trans_set_sync(tp: *mut xfs_trans) { (*tp).t_flags |= XFS_TRANS_SYNC; }

/* External declarations and inline wrappers. */
unsafe extern "C" {
    pub fn xfs_log_item_init(mp: *mut xfs_mount, item: *mut xfs_log_item, typ: c_int, ops: *const xfs_item_ops);
    pub fn xfs_trans_alloc(mp: *mut xfs_mount, resp: *mut xfs_trans_res, blocks: uint, rtextents: uint, flags: uint, tpp: *mut *mut xfs_trans) -> c_int;
    pub fn xfs_trans_reserve_more(tp: *mut xfs_trans, blocks: c_uint, rtextents: c_uint) -> c_int;
    pub fn xfs_trans_alloc_empty(mp: *mut xfs_mount) -> *mut xfs_trans;
    pub fn xfs_trans_mod_sb(tp: *mut xfs_trans, field: uint, delta: i64);
    pub fn xfs_trans_get_buf_map(tp: *mut xfs_trans, target: *mut xfs_buftarg, map: *mut xfs_buf_map, nmaps: c_int, flags: xfs_buf_flags_t, bpp: *mut *mut xfs_buf) -> c_int;
    /* C inline wrapper xfs_trans_get_buf uses DEFINE_SINGLE_BUF_MAP and the map call. */
    pub fn xfs_trans_get_buf(tp: *mut xfs_trans, target: *mut xfs_buftarg, blkno: xfs_daddr_t, numblks: c_int, flags: xfs_buf_flags_t, bpp: *mut *mut xfs_buf) -> c_int;
    pub fn xfs_trans_read_buf_map(mp: *mut xfs_mount, tp: *mut xfs_trans, target: *mut xfs_buftarg, map: *mut xfs_buf_map, nmaps: c_int, flags: xfs_buf_flags_t, bpp: *mut *mut xfs_buf, ops: *const xfs_buf_ops) -> c_int;
    /* C inline wrapper xfs_trans_read_buf likewise constructs one buffer map. */
    pub fn xfs_trans_read_buf(mp: *mut xfs_mount, tp: *mut xfs_trans, target: *mut xfs_buftarg, blkno: xfs_daddr_t, numblks: c_int, flags: xfs_buf_flags_t, bpp: *mut *mut xfs_buf, ops: *const xfs_buf_ops) -> c_int;
    pub fn xfs_trans_getsb(tp: *mut xfs_trans) -> *mut xfs_buf;
    pub fn xfs_trans_getrtsb(tp: *mut xfs_trans) -> *mut xfs_buf;
    pub fn xfs_trans_brelse(tp: *mut xfs_trans, bp: *mut xfs_buf);
    pub fn xfs_trans_bjoin(tp: *mut xfs_trans, bp: *mut xfs_buf);
    pub fn xfs_trans_bdetach(tp: *mut xfs_trans, bp: *mut xfs_buf);
    pub fn xfs_trans_bhold(tp: *mut xfs_trans, bp: *mut xfs_buf);
    pub fn xfs_trans_bhold_release(tp: *mut xfs_trans, bp: *mut xfs_buf);
    pub fn xfs_trans_binval(tp: *mut xfs_trans, bp: *mut xfs_buf);
    pub fn xfs_trans_inode_buf(tp: *mut xfs_trans, bp: *mut xfs_buf);
    pub fn xfs_trans_stale_inode_buf(tp: *mut xfs_trans, bp: *mut xfs_buf);
    pub fn xfs_trans_ordered_buf(tp: *mut xfs_trans, bp: *mut xfs_buf) -> bool;
    pub fn xfs_trans_dquot_buf(tp: *mut xfs_trans, bp: *mut xfs_buf, type_: uint);
    pub fn xfs_trans_inode_alloc_buf(tp: *mut xfs_trans, bp: *mut xfs_buf);
    pub fn xfs_trans_ijoin(tp: *mut xfs_trans, ip: *mut xfs_inode, flags: uint);
    pub fn xfs_trans_log_buf(tp: *mut xfs_trans, bp: *mut xfs_buf, first: uint, last: uint);
    pub fn xfs_trans_dirty_buf(tp: *mut xfs_trans, bp: *mut xfs_buf);
    pub fn xfs_trans_buf_is_dirty(bp: *mut xfs_buf) -> bool;
    pub fn xfs_trans_log_inode(tp: *mut xfs_trans, ip: *mut xfs_inode, flags: uint);
    pub fn xfs_trans_commit(tp: *mut xfs_trans) -> c_int;
    pub fn xfs_trans_roll(tpp: *mut *mut xfs_trans) -> c_int;
    pub fn xfs_trans_roll_inode(tpp: *mut *mut xfs_trans, ip: *mut xfs_inode) -> c_int;
    pub fn xfs_trans_cancel(tp: *mut xfs_trans);
    pub fn xfs_trans_ail_init(mp: *mut xfs_mount) -> c_int;
    pub fn xfs_trans_ail_destroy(mp: *mut xfs_mount);
    pub fn xfs_trans_alloc_inode(ip: *mut xfs_inode, resv: *mut xfs_trans_res, dblocks: c_uint, rblocks: c_uint, force: bool, tpp: *mut *mut xfs_trans) -> c_int;
    pub fn xfs_trans_reserve_more_inode(tp: *mut xfs_trans, ip: *mut xfs_inode, dblocks: c_uint, rblocks: c_uint, force_quota: bool) -> c_int;
    pub fn xfs_trans_alloc_icreate(mp: *mut xfs_mount, resv: *mut xfs_trans_res, udqp: *mut xfs_dquot, gdqp: *mut xfs_dquot, pdqp: *mut xfs_dquot, dblocks: c_uint, tpp: *mut *mut xfs_trans) -> c_int;
    pub fn xfs_trans_alloc_ichange(ip: *mut xfs_inode, udqp: *mut xfs_dquot, gdqp: *mut xfs_dquot, pdqp: *mut xfs_dquot, force: bool, tpp: *mut *mut xfs_trans) -> c_int;
    pub fn xfs_trans_alloc_dir(dp: *mut xfs_inode, resv: *mut xfs_trans_res, ip: *mut xfs_inode, dblocks: *mut c_uint, tpp: *mut *mut xfs_trans, nospace_error: *mut c_int) -> c_int;
    pub fn xfs_trans_buf_set_type(tp: *mut xfs_trans, bp: *mut xfs_buf, typ: xfs_blft);
    pub fn xfs_trans_buf_copy_type(dst_bp: *mut xfs_buf, src_bp: *mut xfs_buf);
}

/* The cache is supplied by the kernel allocator. */
unsafe extern "C" { pub static mut xfs_trans_cache: *mut kmem_cache; }

#[inline] pub unsafe fn xfs_trans_set_context(tp: *mut xfs_trans) { (*tp).t_pflags = memalloc_nofs_save(); }
#[inline] pub unsafe fn xfs_trans_clear_context(tp: *mut xfs_trans) { memalloc_nofs_restore((*tp).t_pflags); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
