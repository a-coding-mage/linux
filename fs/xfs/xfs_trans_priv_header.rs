// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000,2002,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Forward declarations supplied by other translated headers.
#[repr(C)]
pub struct xlog {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_log_item {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_mount {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_trans {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_ail {
    pub ail_log: *mut xlog,
    pub ail_task: *mut task_struct,
    pub ail_head: list_head,
    pub ail_cursors: list_head,
    pub ail_lock: spinlock_t,
    pub ail_last_pushed_lsn: xfs_lsn_t,
    pub ail_head_lsn: xfs_lsn_t,
    pub ail_log_flush: ::core::ffi::c_int,
    pub ail_opstate: ::core::ffi::c_ulong,
    pub ail_buf_list: list_head,
    pub ail_empty: wait_queue_head_t,
    pub ail_target: xfs_lsn_t,
}
#[repr(C)]
pub struct xfs_log_vec {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_ail_cursor {
    pub list: list_head,
    pub item: *mut xfs_log_item,
}

extern "C" {
    pub fn xfs_trans_init(mp: *mut xfs_mount);
    pub fn xfs_trans_add_item(tp: *mut xfs_trans, lip: *mut xfs_log_item);
    pub fn xfs_trans_del_item(lip: *mut xfs_log_item);
    pub fn xfs_trans_unreserve_and_mod_sb(tp: *mut xfs_trans);

    pub fn xfs_trans_ail_update_bulk(
        ailp: *mut xfs_ail,
        cur: *mut xfs_ail_cursor,
        log_items: *mut *mut xfs_log_item,
        nr_items: ::core::ffi::c_int,
        lsn: xfs_lsn_t,
    );
    pub fn xfs_trans_ail_insert(ailp: *mut xfs_ail, lip: *mut xfs_log_item, lsn: xfs_lsn_t);
    pub fn xfs_ail_delete_one(ailp: *mut xfs_ail, lip: *mut xfs_log_item) -> xfs_lsn_t;
    pub fn xfs_ail_update_finish(ailp: *mut xfs_ail, old_lsn: xfs_lsn_t);
    pub fn xfs_trans_ail_delete(lip: *mut xfs_log_item, shutdown_type: ::core::ffi::c_int);
    pub fn xfs_ail_push_all_sync(ailp: *mut xfs_ail);
    pub fn xfs_ail_min_lsn(ailp: *mut xfs_ail) -> xfs_lsn_t;
    pub fn xfs_trans_ail_cursor_first(ailp: *mut xfs_ail, cur: *mut xfs_ail_cursor, lsn: xfs_lsn_t) -> *mut xfs_log_item;
    pub fn xfs_trans_ail_cursor_last(ailp: *mut xfs_ail, cur: *mut xfs_ail_cursor, lsn: xfs_lsn_t) -> *mut xfs_log_item;
    pub fn xfs_trans_ail_cursor_next(ailp: *mut xfs_ail, cur: *mut xfs_ail_cursor) -> *mut xfs_log_item;
    pub fn xfs_trans_ail_cursor_done(cur: *mut xfs_ail_cursor);
    pub fn __xfs_ail_assign_tail_lsn(ailp: *mut xfs_ail);
}

pub const XFS_AIL_OPSTATE_PUSH_ALL: ::core::ffi::c_uint = 0u32;

// Types and helpers below are supplied by other translated headers.
extern "C" {
    fn list_first_entry_or_null(head: *mut list_head) -> *mut xfs_log_item;
    fn wake_up_process(task: *mut task_struct);
    fn test_and_set_bit(nr: ::core::ffi::c_uint, addr: *mut ::core::ffi::c_ulong) -> bool;
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
}

pub unsafe fn xfs_ail_min(ailp: *mut xfs_ail) -> *mut xfs_log_item {
    list_first_entry_or_null(&mut (*ailp).ail_head)
}

pub unsafe fn xfs_trans_ail_update(ailp: *mut xfs_ail, lip: *mut xfs_log_item, lsn: xfs_lsn_t) {
    xfs_trans_ail_update_bulk(ailp, core::ptr::null_mut(), &mut lip, 1, lsn);
}

pub unsafe fn xfs_ail_push(ailp: *mut xfs_ail) {
    wake_up_process((*ailp).ail_task);
}

pub unsafe fn xfs_ail_push_all(ailp: *mut xfs_ail) {
    if !test_and_set_bit(XFS_AIL_OPSTATE_PUSH_ALL, &mut (*ailp).ail_opstate) {
        xfs_ail_push(ailp);
    }
}

pub unsafe fn xfs_ail_get_push_target(ailp: *mut xfs_ail) -> xfs_lsn_t {
    core::ptr::read_volatile(&(*ailp).ail_target)
}

pub unsafe fn xfs_ail_assign_tail_lsn(ailp: *mut xfs_ail) {
    spin_lock(&mut (*ailp).ail_lock);
    __xfs_ail_assign_tail_lsn(ailp);
    spin_unlock(&mut (*ailp).ail_lock);
}

// On 32-bit platforms the C implementation locks because xfs_lsn_t is 64-bit.
#[cfg(target_pointer_width = "32")]
pub unsafe fn xfs_trans_ail_copy_lsn(ailp: *mut xfs_ail, dst: *mut xfs_lsn_t, src: *mut xfs_lsn_t) {
    spin_lock(&mut (*ailp).ail_lock);
    *dst = *src;
    spin_unlock(&mut (*ailp).ail_lock);
}

#[cfg(target_pointer_width = "64")]
pub unsafe fn xfs_trans_ail_copy_lsn(_ailp: *mut xfs_ail, dst: *mut xfs_lsn_t, src: *mut xfs_lsn_t) {
    *dst = *src;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
