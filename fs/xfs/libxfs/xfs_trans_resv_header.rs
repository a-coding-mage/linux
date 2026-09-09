// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2002,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// struct xfs_mount;

/*
 * structure for maintaining pre-calculated transaction reservations.
 */
#[repr(C)]
pub struct xfs_trans_res {
    pub tr_logres: libc::c_uint,  /* log space unit in bytes per log ticket */
    pub tr_logcount: libc::c_int, /* number of log operations per log ticket */
    pub tr_logflags: libc::c_int, /* log flags, currently only used for indicating
                                   * a reservation request is permanent or not */
}

#[repr(C)]
pub struct xfs_trans_resv {
    pub tr_write: xfs_trans_res,
    pub tr_itruncate: xfs_trans_res,
    pub tr_rename: xfs_trans_res,
    pub tr_link: xfs_trans_res,
    pub tr_remove: xfs_trans_res,
    pub tr_symlink: xfs_trans_res,
    pub tr_create: xfs_trans_res,
    pub tr_create_tmpfile: xfs_trans_res,
    pub tr_mkdir: xfs_trans_res,
    pub tr_ifree: xfs_trans_res,
    pub tr_ichange: xfs_trans_res,
    pub tr_growdata: xfs_trans_res,
    pub tr_addafork: xfs_trans_res,
    pub tr_writeid: xfs_trans_res,
    pub tr_attrinval: xfs_trans_res,
    pub tr_attrsetm: xfs_trans_res,
    pub tr_attrsetrt: xfs_trans_res,
    pub tr_attrrm: xfs_trans_res,
    pub tr_clearagi: xfs_trans_res,
    pub tr_growrtalloc: xfs_trans_res,
    pub tr_growrtzero: xfs_trans_res,
    pub tr_growrtfree: xfs_trans_res,
    pub tr_qm_setqlim: xfs_trans_res,
    pub tr_qm_dqalloc: xfs_trans_res,
    pub tr_sb: xfs_trans_res,
    pub tr_fsyncts: xfs_trans_res,
    pub tr_atomic_ioend: xfs_trans_res,
}

/* shorthand way of accessing reservation structure */
#[macro_export]
macro_rules! M_RES { ($mp:expr) => { unsafe { &mut (*$mp).m_resv } }; }

/* Per-directory log reservation for any directory change. */
#[macro_export]
macro_rules! XFS_DIROP_LOG_RES {
    ($mp:expr) => { XFS_FSB_TO_B($mp, XFS_DAENTER_BLOCKS($mp, XFS_DATA_FORK)) + XFS_FSB_TO_B($mp, XFS_DAENTER_BMAPS($mp, XFS_DATA_FORK) + 1) };
}
#[macro_export]
macro_rules! XFS_DIROP_LOG_COUNT {
    ($mp:expr) => { XFS_DAENTER_BLOCKS($mp, XFS_DATA_FORK) + XFS_DAENTER_BMAPS($mp, XFS_DATA_FORK) + 1 };
}

pub const XFS_DEFAULT_LOG_COUNT: libc::c_int = 1;
pub const XFS_DEFAULT_PERM_LOG_COUNT: libc::c_int = 2;
pub const XFS_ITRUNCATE_LOG_COUNT: libc::c_int = 2;
pub const XFS_INACTIVE_LOG_COUNT: libc::c_int = 2;
pub const XFS_CREATE_LOG_COUNT: libc::c_int = 2;
pub const XFS_CREATE_TMPFILE_LOG_COUNT: libc::c_int = 2;
pub const XFS_MKDIR_LOG_COUNT: libc::c_int = 3;
pub const XFS_SYMLINK_LOG_COUNT: libc::c_int = 3;
pub const XFS_REMOVE_LOG_COUNT: libc::c_int = 2;
pub const XFS_LINK_LOG_COUNT: libc::c_int = 2;
pub const XFS_RENAME_LOG_COUNT: libc::c_int = 2;
pub const XFS_WRITE_LOG_COUNT: libc::c_int = 2;
pub const XFS_ADDAFORK_LOG_COUNT: libc::c_int = 2;
pub const XFS_ATTRINVAL_LOG_COUNT: libc::c_int = 1;
pub const XFS_ATTRSET_LOG_COUNT: libc::c_int = 3;
pub const XFS_ATTRRM_LOG_COUNT: libc::c_int = 3;

/* Retained purely for minimum log size calculations; not for runtime reservations. */
pub const XFS_ITRUNCATE_LOG_COUNT_REFLINK: libc::c_int = 8;
pub const XFS_WRITE_LOG_COUNT_REFLINK: libc::c_int = 8;

extern "C" {
    pub fn xfs_trans_resv_calc(mp: *mut xfs_mount, resp: *mut xfs_trans_resv);
    pub fn xfs_allocfree_block_count(mp: *mut xfs_mount, num_ops: libc::c_uint) -> libc::c_uint;
    pub fn xfs_calc_finish_bui_reservation(mp: *mut xfs_mount, nr_ops: libc::c_uint) -> libc::c_uint;
    pub fn xfs_calc_finish_efi_reservation(mp: *mut xfs_mount, nr_ops: libc::c_uint) -> libc::c_uint;
    pub fn xfs_calc_finish_rt_efi_reservation(mp: *mut xfs_mount, nr_ops: libc::c_uint) -> libc::c_uint;
    pub fn xfs_calc_finish_rui_reservation(mp: *mut xfs_mount, nr_ops: libc::c_uint) -> libc::c_uint;
    pub fn xfs_calc_finish_rt_rui_reservation(mp: *mut xfs_mount, nr_ops: libc::c_uint) -> libc::c_uint;
    pub fn xfs_calc_finish_cui_reservation(mp: *mut xfs_mount, nr_ops: libc::c_uint) -> libc::c_uint;
    pub fn xfs_calc_finish_rt_cui_reservation(mp: *mut xfs_mount, nr_ops: libc::c_uint) -> libc::c_uint;
    pub fn xfs_calc_itruncate_reservation_minlogsize(mp: *mut xfs_mount) -> libc::c_uint;
    pub fn xfs_calc_write_reservation_minlogsize(mp: *mut xfs_mount) -> libc::c_uint;
    pub fn xfs_calc_qm_dqalloc_reservation_minlogsize(mp: *mut xfs_mount) -> libc::c_uint;
    pub fn xfs_calc_max_atomic_write_fsblocks(mp: *mut xfs_mount) -> xfs_extlen_t;
    pub fn xfs_calc_atomic_write_log_geometry(mp: *mut xfs_mount, blockcount: xfs_extlen_t, new_logres: *mut libc::c_uint) -> xfs_extlen_t;
    pub fn xfs_calc_atomic_write_reservation(mp: *mut xfs_mount, blockcount: xfs_extlen_t) -> libc::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
