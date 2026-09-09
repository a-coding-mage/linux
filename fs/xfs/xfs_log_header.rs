// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2003,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

#[repr(C)]
pub struct xlog_format_buf {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_cil_ctx {
    _private: [u8; 0],
}

/* Region types for iovec's i_type */
pub const XLOG_REG_TYPE_BFORMAT: i32 = 1;
pub const XLOG_REG_TYPE_BCHUNK: i32 = 2;
pub const XLOG_REG_TYPE_EFI_FORMAT: i32 = 3;
pub const XLOG_REG_TYPE_EFD_FORMAT: i32 = 4;
pub const XLOG_REG_TYPE_IFORMAT: i32 = 5;
pub const XLOG_REG_TYPE_ICORE: i32 = 6;
pub const XLOG_REG_TYPE_IEXT: i32 = 7;
pub const XLOG_REG_TYPE_IBROOT: i32 = 8;
pub const XLOG_REG_TYPE_ILOCAL: i32 = 9;
pub const XLOG_REG_TYPE_IATTR_EXT: i32 = 10;
pub const XLOG_REG_TYPE_IATTR_BROOT: i32 = 11;
pub const XLOG_REG_TYPE_IATTR_LOCAL: i32 = 12;
pub const XLOG_REG_TYPE_QFORMAT: i32 = 13;
pub const XLOG_REG_TYPE_DQUOT: i32 = 14;
pub const XLOG_REG_TYPE_QUOTAOFF: i32 = 15;
pub const XLOG_REG_TYPE_LRHEADER: i32 = 16;
pub const XLOG_REG_TYPE_UNMOUNT: i32 = 17;
pub const XLOG_REG_TYPE_COMMIT: i32 = 18;
pub const XLOG_REG_TYPE_TRANSHDR: i32 = 19;
pub const XLOG_REG_TYPE_ICREATE: i32 = 20;
pub const XLOG_REG_TYPE_RUI_FORMAT: i32 = 21;
pub const XLOG_REG_TYPE_RUD_FORMAT: i32 = 22;
pub const XLOG_REG_TYPE_CUI_FORMAT: i32 = 23;
pub const XLOG_REG_TYPE_CUD_FORMAT: i32 = 24;
pub const XLOG_REG_TYPE_BUI_FORMAT: i32 = 25;
pub const XLOG_REG_TYPE_BUD_FORMAT: i32 = 26;
pub const XLOG_REG_TYPE_ATTRI_FORMAT: i32 = 27;
pub const XLOG_REG_TYPE_ATTRD_FORMAT: i32 = 28;
pub const XLOG_REG_TYPE_ATTR_NAME: i32 = 29;
pub const XLOG_REG_TYPE_ATTR_VALUE: i32 = 30;
pub const XLOG_REG_TYPE_XMI_FORMAT: i32 = 31;
pub const XLOG_REG_TYPE_XMD_FORMAT: i32 = 32;
pub const XLOG_REG_TYPE_ATTR_NEWNAME: i32 = 33;
pub const XLOG_REG_TYPE_ATTR_NEWVALUE: i32 = 34;
pub const XLOG_REG_TYPE_MAX: i32 = 34;

pub const XFS_LOG_VEC_ORDERED: i32 = -1;

/*
 * Calculate the log iovec length for a given user buffer length. Intended to be
 * used by ->iop_size implementations when sizing buffers of arbitrary
 * alignments.
 */
#[inline]
pub unsafe fn xlog_calc_iovec_len(len: i32) -> i32 {
    roundup(len, core::mem::size_of::<u32>())
}

extern "C" {
    pub fn xlog_format_start(lfb: *mut xlog_format_buf, type_: u16) -> *mut core::ffi::c_void;
    pub fn xlog_format_commit(lfb: *mut xlog_format_buf, data_len: u32);
}

/*
 * Copy the amount of data requested by the caller into a new log iovec.
 */
#[inline]
pub unsafe fn xlog_format_copy(
    lfb: *mut xlog_format_buf,
    type_: u16,
    data: *mut core::ffi::c_void,
    len: u32,
) -> *mut core::ffi::c_void {
    let buf = xlog_format_start(lfb, type_);
    core::ptr::copy_nonoverlapping(data as *const u8, buf as *mut u8, len as usize);
    xlog_format_commit(lfb, len);
    buf
}

/*
 * Flags to xfs_log_force()
 *
 * XFS_LOG_SYNC: Synchronous force in-core log to disk
 */
pub const XFS_LOG_SYNC: u32 = 0x1;

/* Log manager interfaces */
#[repr(C)] pub struct xfs_mount { _private: [u8; 0] }
#[repr(C)] pub struct xlog_in_core { _private: [u8; 0] }
#[repr(C)] pub struct xlog_ticket { _private: [u8; 0] }
#[repr(C)] pub struct xfs_log_item { _private: [u8; 0] }
#[repr(C)] pub struct xfs_item_ops { _private: [u8; 0] }
#[repr(C)] pub struct xfs_trans { _private: [u8; 0] }
#[repr(C)] pub struct xlog { _private: [u8; 0] }

extern "C" {
    pub fn xfs_log_force(mp: *mut xfs_mount, flags: u32) -> i32;
    pub fn xfs_log_force_seq(mp: *mut xfs_mount, seq: xfs_csn_t, flags: u32, log_forced: *mut i32) -> i32;
    pub fn xfs_log_mount(mp: *mut xfs_mount, log_target: *mut xfs_buftarg, start_block: xfs_daddr_t, num_bblocks: i32) -> i32;
    pub fn xfs_log_mount_finish(mp: *mut xfs_mount) -> i32;
    pub fn xfs_log_mount_cancel(mp: *mut xfs_mount);
    pub fn xlog_assign_tail_lsn(mp: *mut xfs_mount) -> xfs_lsn_t;
    pub fn xlog_assign_tail_lsn_locked(mp: *mut xfs_mount) -> xfs_lsn_t;
    pub fn xfs_log_space_wake(mp: *mut xfs_mount);
    pub fn xfs_log_reserve(mp: *mut xfs_mount, length: i32, count: i32, ticket: *mut *mut xlog_ticket, permanent: bool) -> i32;
    pub fn xfs_log_regrant(mp: *mut xfs_mount, tic: *mut xlog_ticket) -> i32;
    pub fn xfs_log_unmount(mp: *mut xfs_mount);
    pub fn xfs_log_writable(mp: *mut xfs_mount) -> bool;
    pub fn xfs_log_ticket_get(ticket: *mut xlog_ticket) -> *mut xlog_ticket;
    pub fn xfs_log_ticket_put(ticket: *mut xlog_ticket);
    pub fn xlog_cil_process_committed(list: *mut list_head);
    pub fn xfs_log_item_in_current_chkpt(lip: *mut xfs_log_item) -> bool;
    pub fn xfs_log_work_queue(mp: *mut xfs_mount);
    pub fn xfs_log_quiesce(mp: *mut xfs_mount) -> i32;
    pub fn xfs_log_clean(mp: *mut xfs_mount);
    pub fn xfs_log_check_lsn(mp: *mut xfs_mount, lsn: xfs_lsn_t) -> bool;
    pub fn xlog_force_shutdown(log: *mut xlog, shutdown_flags: u32) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
