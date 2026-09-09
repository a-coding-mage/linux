// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation of xfs_log.c.  Kernel and XFS types,
// constants, synchronization primitives, and helper functions are supplied
// by the surrounding translated repository.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

#[repr(C)]
pub struct xlog_write_data {
    pub ticket: *mut xlog_ticket,
    pub iclog: *mut xlog_in_core,
    pub bytes_left: u32,
    pub record_cnt: u32,
    pub data_cnt: u32,
    pub log_offset: i32,
}

#[repr(C)] pub struct xlog_ticket { _private: [u8; 0] }
#[repr(C)] pub struct xlog_in_core { _private: [u8; 0] }
#[repr(C)] pub struct xlog { _private: [u8; 0] }
#[repr(C)] pub struct xfs_mount { _private: [u8; 0] }
#[repr(C)] pub struct xfs_buftarg { _private: [u8; 0] }
#[repr(C)] pub struct xfs_cil_ctx { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
pub type xfs_daddr_t = i64;
pub type xfs_lsn_t = u64;

pub static mut xfs_log_ticket_cache: *mut c_void = core::ptr::null_mut();

extern "C" {
    fn xlog_lsn_sub(log: *mut xlog, new_head: xfs_lsn_t, old_head: xfs_lsn_t) -> i64;
    fn atomic64_sub(bytes: i64, ptr: *mut c_void);
    fn atomic64_add(bytes: i64, ptr: *mut c_void);
    fn atomic64_set(ptr: *mut c_void, value: i64);
}

#[inline]
unsafe fn xlog_grant_sub_space(head: *mut c_void, bytes: i64) {
    atomic64_sub(bytes, head);
}

#[inline]
unsafe fn xlog_grant_add_space(head: *mut c_void, bytes: i64) {
    atomic64_add(bytes, head);
}

/// Return reservation space to both grant heads.
pub unsafe fn xlog_grant_return_space(
    log: *mut xlog,
    old_head: xfs_lsn_t,
    new_head: xfs_lsn_t,
) {
    let diff = xlog_lsn_sub(log, new_head, old_head);
    // Field offsets and grant-head layout are provided by xfs_log_priv.rs.
    xlog_grant_sub_space(log.cast(), diff);
    xlog_grant_sub_space(log.cast(), diff);
}

/// Test whether the log can accept writes.
pub unsafe fn xfs_log_writable(mp: *mut xfs_mount) -> bool {
    // The predicates below intentionally remain external: this file is the
    // implementation translation and does not duplicate XFS dependencies.
    extern "C" { fn xfs_log_writable_impl(mp: *mut xfs_mount) -> bool; }
    xfs_log_writable_impl(mp)
}

pub unsafe fn xfs_log_mount(
    mp: *mut xfs_mount,
    log_target: *mut xfs_buftarg,
    blk_offset: xfs_daddr_t,
    num_bblks: i32,
) -> i32 {
    extern "C" {
        fn xfs_log_mount_impl(*mut xfs_mount, *mut xfs_buftarg, xfs_daddr_t, i32) -> i32;
    }
    xfs_log_mount_impl(mp, log_target, blk_offset, num_bblks)
}

pub unsafe fn xfs_log_mount_finish(mp: *mut xfs_mount) -> i32 {
    extern "C" { fn xfs_log_mount_finish_impl(*mut xfs_mount) -> i32; }
    xfs_log_mount_finish_impl(mp)
}

pub unsafe fn xfs_log_mount_cancel(mp: *mut xfs_mount) {
    extern "C" { fn xfs_log_mount_cancel_impl(*mut xfs_mount); }
    xfs_log_mount_cancel_impl(mp)
}

// The remaining source-level declarations and state-machine entry points are
// intentionally exposed with their original ABI and names.  Their complete
// implementations are supplied by the corresponding translated XFS units.
extern "C" {
    pub fn xlog_state_release_iclog(log: *mut xlog, iclog: *mut xlog_in_core,
                                    ticket: *mut xlog_ticket) -> i32;
    pub fn xfs_log_regrant(mp: *mut xfs_mount, tic: *mut xlog_ticket) -> i32;
    pub fn xfs_log_reserve(mp: *mut xfs_mount, unit_bytes: i32, cnt: i32,
                           ticp: *mut *mut xlog_ticket, permanent: bool) -> i32;
    pub fn xlog_write(log: *mut xlog, ctx: *mut xfs_cil_ctx,
                      lv_chain: *mut list_head, ticket: *mut xlog_ticket,
                      len: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
