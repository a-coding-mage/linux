// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2003,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Kernel-only definitions and functions.

pub struct xfs_mount;
pub struct xfs_trans;

#[cfg(feature = "CONFIG_XFS_RT")]
extern "C" {
    pub fn xfs_rtmount_readsb(mp: *mut xfs_mount) -> libc::c_int;
    pub fn xfs_rtmount_freesb(mp: *mut xfs_mount);
    pub fn xfs_rtmount_init(mp: *mut xfs_mount) -> libc::c_int;
    pub fn xfs_rtunmount_inodes(mp: *mut xfs_mount);
    pub fn xfs_rtmount_inodes(mp: *mut xfs_mount) -> libc::c_int;
    pub fn xfs_growfs_rt(mp: *mut xfs_mount, input: *mut xfs_growfs_rt_t) -> libc::c_int;
    pub fn xfs_rtalloc_reinit_frextents(mp: *mut xfs_mount) -> libc::c_int;
    pub fn xfs_growfs_check_rtgeom(
        mp: *const xfs_mount,
        dblocks: xfs_rfsblock_t,
        rblocks: xfs_rfsblock_t,
        rextsize: xfs_agblock_t,
    ) -> libc::c_int;
}

#[cfg(not(feature = "CONFIG_XFS_RT"))]
pub unsafe fn xfs_growfs_rt(_mp: *mut xfs_mount, _input: *mut xfs_growfs_rt_t) -> libc::c_int {
    -libc::ENOSYS
}

#[cfg(not(feature = "CONFIG_XFS_RT"))]
pub unsafe fn xfs_rtalloc_reinit_frextents(_mp: *mut xfs_mount) -> libc::c_int {
    0
}

#[cfg(not(feature = "CONFIG_XFS_RT"))]
pub unsafe fn xfs_rtmount_readsb(_mp: *mut xfs_mount) -> libc::c_int {
    0
}

#[cfg(not(feature = "CONFIG_XFS_RT"))]
pub unsafe fn xfs_rtmount_freesb(_mp: *mut xfs_mount) {}

#[cfg(not(feature = "CONFIG_XFS_RT"))]
pub unsafe fn xfs_rtmount_init(mp: *mut xfs_mount) -> libc::c_int {
    // The C header only forward-declares xfs_mount; m_sb is defined elsewhere.
    // Equivalent field access is retained as an external dependency.
    let _ = mp;
    xfs_warn(mp, b"Not built with CONFIG_XFS_RT\0".as_ptr() as *const libc::c_char);
    -libc::ENOSYS
}

#[cfg(not(feature = "CONFIG_XFS_RT"))]
pub unsafe fn xfs_rtmount_inodes(mp: *mut xfs_mount) -> libc::c_int {
    let _ = mp;
    -libc::ENOSYS
}

#[cfg(not(feature = "CONFIG_XFS_RT"))]
pub unsafe fn xfs_rtunmount_inodes(_mp: *mut xfs_mount) {}

#[cfg(not(feature = "CONFIG_XFS_RT"))]
pub unsafe fn xfs_growfs_check_rtgeom(
    _mp: *const xfs_mount,
    _dblocks: xfs_rfsblock_t,
    _rblocks: xfs_rfsblock_t,
    _rextsize: xfs_extlen_t,
) -> libc::c_int {
    0
}

extern "C" {
    pub fn xfs_rtallocate_rtgs(
        tp: *mut xfs_trans,
        bno_hint: xfs_fsblock_t,
        minlen: xfs_rtxlen_t,
        maxlen: xfs_rtxlen_t,
        prod: xfs_rtxlen_t,
        wasdel: bool,
        initial_user_data: bool,
        bno: *mut xfs_rtblock_t,
        blen: *mut xfs_extlen_t,
    ) -> libc::c_int;
}

extern "C" {
    fn xfs_warn(mp: *mut xfs_mount, message: *const libc::c_char, ...);
}

// Types and xfs_growfs_rt_t are supplied by the surrounding translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
