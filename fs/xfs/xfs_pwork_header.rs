/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2019 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */

// The following types are supplied by other headers/dependencies.
#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_mount {
    _private: [u8; 0],
}

pub type xfs_pwork_work_fn =
    Option<unsafe extern "C" fn(mp: *mut xfs_mount, pwork: *mut xfs_pwork) -> ::core::ffi::c_int>;

/*
 * Parallel work coordination structure.
 */
#[repr(C)]
pub struct xfs_pwork_ctl {
    pub wq: *mut workqueue_struct,
    pub mp: *mut xfs_mount,
    pub work_fn: xfs_pwork_work_fn,
    pub poll_wait: wait_queue_head,
    pub nr_work: atomic_t,
    pub error: ::core::ffi::c_int,
}

/*
 * Embed this parallel work control item inside your own work structure,
 * then queue work with it.
 */
#[repr(C)]
pub struct xfs_pwork {
    pub work: work_struct,
    pub pctl: *mut xfs_pwork_ctl,
}

// XFS_PWORK_SINGLE_THREADED: { .pctl = NULL }
#[macro_export]
macro_rules! XFS_PWORK_SINGLE_THREADED {
    ($work:expr) => {
        $crate::xfs_pwork {
            work: $work,
            pctl: ::core::ptr::null_mut(),
        }
    };
}

/* Have we been told to abort? */
#[inline]
pub unsafe fn xfs_pwork_ctl_want_abort(pctl: *mut xfs_pwork_ctl) -> bool {
    !pctl.is_null() && (*pctl).error != 0
}

/* Have we been told to abort? */
#[inline]
pub unsafe fn xfs_pwork_want_abort(pwork: *mut xfs_pwork) -> bool {
    xfs_pwork_ctl_want_abort((*pwork).pctl)
}

unsafe extern "C" {
    pub fn xfs_pwork_init(
        mp: *mut xfs_mount,
        pctl: *mut xfs_pwork_ctl,
        work_fn: xfs_pwork_work_fn,
        tag: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn xfs_pwork_queue(pctl: *mut xfs_pwork_ctl, pwork: *mut xfs_pwork);
    pub fn xfs_pwork_destroy(pctl: *mut xfs_pwork_ctl) -> ::core::ffi::c_int;
    pub fn xfs_pwork_poll(pctl: *mut xfs_pwork_ctl);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
