// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Each log item type (XFS_LI_*) gets its own xlog_recover_item_ops to
// define how recovery should work for that type of log item.

#[repr(C)]
pub struct xlog_recover_item_ops {
    pub item_type: u16, // XFS_LI_* type code.
    pub reorder: Option<unsafe extern "C" fn(item: *mut xlog_recover_item) -> xlog_recover_reorder>,
    pub ra_pass2: Option<unsafe extern "C" fn(log: *mut xlog, item: *mut xlog_recover_item)>,
    pub commit_pass1: Option<unsafe extern "C" fn(log: *mut xlog, item: *mut xlog_recover_item) -> ::core::ffi::c_int>,
    pub commit_pass2: Option<unsafe extern "C" fn(
        log: *mut xlog,
        buffer_list: *mut list_head,
        item: *mut xlog_recover_item,
        lsn: xfs_lsn_t,
    ) -> ::core::ffi::c_int>,
}

// Sorting hat for log items as they're read in.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum xlog_recover_reorder {
    XLOG_REORDER_BUFFER_LIST,
    XLOG_REORDER_ITEM_LIST,
    XLOG_REORDER_INODE_BUFFER_LIST,
    XLOG_REORDER_CANCEL_LIST,
}

extern "C" {
    pub static xlog_icreate_item_ops: xlog_recover_item_ops;
    pub static xlog_buf_item_ops: xlog_recover_item_ops;
    pub static xlog_inode_item_ops: xlog_recover_item_ops;
    pub static xlog_dquot_item_ops: xlog_recover_item_ops;
    pub static xlog_quotaoff_item_ops: xlog_recover_item_ops;
    pub static xlog_bui_item_ops: xlog_recover_item_ops;
    pub static xlog_bud_item_ops: xlog_recover_item_ops;
    pub static xlog_efi_item_ops: xlog_recover_item_ops;
    pub static xlog_efd_item_ops: xlog_recover_item_ops;
    pub static xlog_rui_item_ops: xlog_recover_item_ops;
    pub static xlog_rud_item_ops: xlog_recover_item_ops;
    pub static xlog_cui_item_ops: xlog_recover_item_ops;
    pub static xlog_cud_item_ops: xlog_recover_item_ops;
    pub static xlog_attri_item_ops: xlog_recover_item_ops;
    pub static xlog_attrd_item_ops: xlog_recover_item_ops;
    pub static xlog_xmi_item_ops: xlog_recover_item_ops;
    pub static xlog_xmd_item_ops: xlog_recover_item_ops;
    pub static xlog_rtefi_item_ops: xlog_recover_item_ops;
    pub static xlog_rtefd_item_ops: xlog_recover_item_ops;
    pub static xlog_rtrui_item_ops: xlog_recover_item_ops;
    pub static xlog_rtrud_item_ops: xlog_recover_item_ops;
    pub static xlog_rtcui_item_ops: xlog_recover_item_ops;
    pub static xlog_rtcud_item_ops: xlog_recover_item_ops;
}

pub const XLOG_RHASH_BITS: u32 = 4;
pub const XLOG_RHASH_SIZE: u32 = 16;
pub const XLOG_RHASH_SHIFT: u32 = 2;
#[inline]
pub const fn XLOG_RHASH(tid: u32) -> u32 {
    ((tid >> XLOG_RHASH_SHIFT) & (XLOG_RHASH_SIZE - 1))
}
pub const XLOG_MAX_REGIONS_IN_ITEM: u32 = XFS_MAX_BLOCKSIZE / XFS_BLF_CHUNK / 2 + 1;

#[repr(C)]
pub struct xlog_recover_item {
    pub ri_list: list_head,
    pub ri_cnt: ::core::ffi::c_int,
    pub ri_total: ::core::ffi::c_int,
    pub ri_buf: *mut kvec,
    pub ri_ops: *const xlog_recover_item_ops,
}

#[repr(C)]
pub struct xlog_recover {
    pub r_list: hlist_node,
    pub r_log_tid: xlog_tid_t,
    pub r_theader: xfs_trans_header,
    pub r_state: ::core::ffi::c_int,
    pub r_lsn: xfs_lsn_t,
    pub r_itemq: list_head,
}

#[inline]
pub unsafe fn ITEM_TYPE(i: *mut xlog_recover_item) -> u16 {
    *( (*i).ri_buf ).add(0) .iov_base as *const u16
}

pub const XLOG_RECOVER_CRCPASS: ::core::ffi::c_int = 0;
pub const XLOG_RECOVER_PASS1: ::core::ffi::c_int = 1;
pub const XLOG_RECOVER_PASS2: ::core::ffi::c_int = 2;

extern "C" {
    pub fn xlog_buf_readahead(log: *mut xlog, blkno: xfs_daddr_t, len: u32, ops: *const xfs_buf_ops);
    pub fn xlog_is_buffer_cancelled(log: *mut xlog, blkno: xfs_daddr_t, len: u32) -> bool;
    pub fn xlog_recover_iget(mp: *mut xfs_mount, ino: xfs_ino_t, ipp: *mut *mut xfs_inode) -> ::core::ffi::c_int;
    pub fn xlog_recover_iget_handle(mp: *mut xfs_mount, ino: xfs_ino_t, gen: u32, ipp: *mut *mut xfs_inode) -> ::core::ffi::c_int;
    pub fn xlog_recover_release_intent(log: *mut xlog, intent_type: u16, intent_id: u64);
    pub fn xlog_alloc_buf_cancel_table(log: *mut xlog) -> ::core::ffi::c_int;
    pub fn xlog_free_buf_cancel_table(log: *mut xlog);
    pub fn xlog_recover_intent_item(log: *mut xlog, lip: *mut xfs_log_item, lsn: xfs_lsn_t, ops: *const xfs_defer_op_type);
    pub fn xlog_recover_finish_intent(tp: *mut xfs_trans, dfp: *mut xfs_defer_pending) -> ::core::ffi::c_int;
}

#[cfg(debug_assertions)]
extern "C" { pub fn xlog_check_buf_cancel_table(log: *mut xlog); }

#[inline]
pub unsafe fn xlog_recover_resv(r: *const xfs_trans_res) -> xfs_trans_res {
    xfs_trans_res { tr_logres: (*r).tr_logres, tr_logcount: 1, tr_logflags: (*r).tr_logflags }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
