/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2016 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */

/* Translated from xfs_defer.h. */

#[allow(non_camel_case_types)]
pub struct xfs_btree_cur;
#[allow(non_camel_case_types)]
pub struct list_head;
#[allow(non_camel_case_types)]
pub struct xfs_log_item;
#[allow(non_camel_case_types)]
pub struct xfs_trans;
#[allow(non_camel_case_types)]
pub struct xfs_buf;
#[allow(non_camel_case_types)]
pub struct xfs_inode;
#[allow(non_camel_case_types)]
pub struct xfs_mount;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct xfs_defer_pending {
    pub dfp_list: list_head,
    pub dfp_work: list_head,
    pub dfp_intent: *mut xfs_log_item,
    pub dfp_done: *mut xfs_log_item,
    pub dfp_ops: *const xfs_defer_op_type,
    pub dfp_count: u32,
    pub dfp_flags: u32,
}

pub const XFS_DEFER_PAUSED: u32 = 1u32 << 0;
pub const XFS_DEFER_PENDING_STRINGS: &[(u32, &str)] = &[(XFS_DEFER_PAUSED, "paused")];

extern "C" {
    pub fn xfs_defer_item_pause(tp: *mut xfs_trans, dfp: *mut xfs_defer_pending);
    pub fn xfs_defer_item_unpause(tp: *mut xfs_trans, dfp: *mut xfs_defer_pending);

    pub fn xfs_defer_add(
        tp: *mut xfs_trans,
        h: *mut list_head,
        ops: *const xfs_defer_op_type,
    ) -> *mut xfs_defer_pending;
    pub fn xfs_defer_finish_noroll(tp: *mut *mut xfs_trans) -> i32;
    pub fn xfs_defer_finish(tp: *mut *mut xfs_trans) -> i32;
    pub fn xfs_defer_finish_one(tp: *mut xfs_trans, dfp: *mut xfs_defer_pending) -> i32;
    pub fn xfs_defer_cancel(tp: *mut xfs_trans);
    pub fn xfs_defer_move(dtp: *mut xfs_trans, stp: *mut xfs_trans);
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct xfs_defer_op_type {
    pub name: *const core::ffi::c_char,
    pub max_items: u32,
    pub create_intent: Option<unsafe extern "C" fn(*mut xfs_trans, *mut list_head, u32, bool) -> *mut xfs_log_item>,
    pub abort_intent: Option<unsafe extern "C" fn(*mut xfs_log_item)>,
    pub create_done: Option<unsafe extern "C" fn(*mut xfs_trans, *mut xfs_log_item, u32) -> *mut xfs_log_item>,
    pub finish_item: Option<unsafe extern "C" fn(*mut xfs_trans, *mut xfs_log_item, *mut list_head, *mut *mut xfs_btree_cur) -> i32>,
    pub finish_cleanup: Option<unsafe extern "C" fn(*mut xfs_trans, *mut xfs_btree_cur, i32)>,
    pub cancel_item: Option<unsafe extern "C" fn(*mut list_head)>,
    pub recover_work: Option<unsafe extern "C" fn(*mut xfs_defer_pending, *mut list_head) -> i32>,
    pub relog_intent: Option<unsafe extern "C" fn(*mut xfs_trans, *mut xfs_log_item, *mut xfs_log_item) -> *mut xfs_log_item>,
}

extern "C" {
    pub static xfs_bmap_update_defer_type: xfs_defer_op_type;
    pub static xfs_refcount_update_defer_type: xfs_defer_op_type;
    pub static xfs_rtrefcount_update_defer_type: xfs_defer_op_type;
    pub static xfs_rmap_update_defer_type: xfs_defer_op_type;
    pub static xfs_rtrmap_update_defer_type: xfs_defer_op_type;
    pub static xfs_extent_free_defer_type: xfs_defer_op_type;
    pub static xfs_agfl_free_defer_type: xfs_defer_op_type;
    pub static xfs_rtextent_free_defer_type: xfs_defer_op_type;
    pub static xfs_attr_defer_type: xfs_defer_op_type;
    pub static xfs_exchmaps_defer_type: xfs_defer_op_type;
}

pub const XFS_DEFER_OPS_NR_INODES: usize = 5;
pub const XFS_DEFER_OPS_NR_BUFS: usize = 2;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct xfs_defer_resources {
    pub dr_bp: [*mut xfs_buf; XFS_DEFER_OPS_NR_BUFS],
    pub dr_ip: [*mut xfs_inode; XFS_DEFER_OPS_NR_INODES],
    pub dr_bufs: u16,
    pub dr_ordered: u16,
    pub dr_inos: u16,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct xfs_defer_capture {
    pub dfc_list: list_head,
    pub dfc_dfops: list_head,
    pub dfc_tpflags: u32,
    pub dfc_blkres: u32,
    pub dfc_rtxres: u32,
    pub dfc_logres: u32,
    pub dfc_held: xfs_defer_resources,
}

extern "C" {
    pub fn xfs_defer_ops_capture_and_commit(tp: *mut xfs_trans, capture_list: *mut list_head) -> i32;
    pub fn xfs_defer_ops_continue(d: *mut xfs_defer_capture, tp: *mut xfs_trans, dres: *mut xfs_defer_resources);
    pub fn xfs_defer_ops_capture_abort(mp: *mut xfs_mount, d: *mut xfs_defer_capture);
    pub fn xfs_defer_resources_rele(dres: *mut xfs_defer_resources);
    pub fn xfs_defer_start_recovery(lip: *mut xfs_log_item, r_dfops: *mut list_head, ops: *const xfs_defer_op_type);
    pub fn xfs_defer_cancel_recovery(mp: *mut xfs_mount, dfp: *mut xfs_defer_pending);
    pub fn xfs_defer_finish_recovery(mp: *mut xfs_mount, dfp: *mut xfs_defer_pending, capture_list: *mut list_head) -> i32;
}

#[inline]
pub unsafe fn xfs_defer_add_item(dfp: *mut xfs_defer_pending, work: *mut list_head) {
    /* list_add_tail(work, &dfp->dfp_work); */
    extern "C" { fn list_add_tail(new: *mut list_head, head: *mut list_head); }
    list_add_tail(work, &mut (*dfp).dfp_work);
    (*dfp).dfp_count = (*dfp).dfp_count.wrapping_add(1);
}

extern "C" {
    pub fn xfs_defer_init_item_caches() -> i32;
    pub fn xfs_defer_destroy_item_caches();
    pub fn xfs_defer_add_barrier(tp: *mut xfs_trans);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
