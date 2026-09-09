/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2016 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */

// Declarations supplied by the surrounding translation unit:
// struct xfs_perag;
// struct xfs_trans;
// struct xfs_ag_resv;
// struct xfs_alloc_arg;
// enum xfs_ag_resv_type;
// type xfs_extlen_t;
// XFS_AG_RESV_METADATA and XFS_AG_RESV_RMAPBT.

unsafe extern "C" {
    pub fn xfs_ag_resv_free(pag: *mut xfs_perag);
    pub fn xfs_ag_resv_init(pag: *mut xfs_perag, tp: *mut xfs_trans) -> ::core::ffi::c_int;

    pub fn xfs_ag_resv_critical(
        pag: *mut xfs_perag,
        type_: xfs_ag_resv_type,
    ) -> bool;
    pub fn xfs_ag_resv_needed(
        pag: *mut xfs_perag,
        type_: xfs_ag_resv_type,
    ) -> xfs_extlen_t;

    pub fn xfs_ag_resv_alloc_extent(
        pag: *mut xfs_perag,
        type_: xfs_ag_resv_type,
        args: *mut xfs_alloc_arg,
    );
    pub fn xfs_ag_resv_free_extent(
        pag: *mut xfs_perag,
        type_: xfs_ag_resv_type,
        tp: *mut xfs_trans,
        len: xfs_extlen_t,
    );
}

#[inline]
pub unsafe fn xfs_perag_resv(
    pag: *mut xfs_perag,
    type_: xfs_ag_resv_type,
) -> *mut xfs_ag_resv {
    match type_ {
        XFS_AG_RESV_METADATA => &mut (*pag).pag_meta_resv,
        XFS_AG_RESV_RMAPBT => &mut (*pag).pag_rmapbt_resv,
        _ => ::core::ptr::null_mut(),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
