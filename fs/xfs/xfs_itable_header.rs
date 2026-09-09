// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2001 Silicon Graphics, Inc.  All Rights Reserved.
 */

use core::ffi::{c_int, c_void};

/* External types and constants are supplied by the surrounding translation. */
use crate::{mnt_idmap, xfs_bstat, xfs_bulkstat, xfs_inogrp, xfs_inumbers, xfs_mount, xfs_ino_t, ECANCELED};

/* In-memory representation of a userspace request for batch inode data. */
#[repr(C)]
pub struct xfs_ibulk {
    pub mp: *mut xfs_mount,
    pub idmap: *mut mnt_idmap,
    pub ubuffer: *mut c_void, /* user output buffer */
    pub startino: xfs_ino_t, /* start with this inode */
    pub icount: u32,         /* number of elements in ubuffer */
    pub ocount: u32,         /* number of records returned */
    pub flags: u32,          /* XFS_IBULK_FLAG_* */
    pub iwalk_flags: u32,    /* XFS_IWALK_FLAG_* */
}

/* Fill out the bs_extents64 field if set. */
pub const XFS_IBULK_NREXT64: u32 = 1u32 << 0;

/* Signal that we can return metadata directories. */
pub const XFS_IBULK_METADIR: u32 = 1u32 << 1;

/*
 * Advance the user buffer pointer by one record of the given size.  If the
 * buffer is now full, return the appropriate error code.
 */
#[inline]
pub unsafe fn xfs_ibulk_advance(breq: *mut xfs_ibulk, bytes: usize) -> c_int {
    let b = (*breq).ubuffer as *mut u8;

    (*breq).ubuffer = b.add(bytes) as *mut c_void;
    (*breq).ocount += 1;
    if (*breq).ocount == (*breq).icount {
        -ECANCELED
    } else {
        0
    }
}

/*
 * Return stat information in bulk (by-inode) for the filesystem.
 */

/*
 * Return codes for the formatter function are 0 to continue iterating, and
 * non-zero to stop iterating.  Any non-zero value will be passed up to the
 * bulkstat/inumbers caller.  The special value -ECANCELED can be used to stop
 * iteration, as neither bulkstat nor inumbers will ever generate that error
 * code on their own.
 */

pub type bulkstat_one_fmt_pf = Option<unsafe extern "C" fn(
    breq: *mut xfs_ibulk,
    bstat: *const xfs_bulkstat,
) -> c_int>;

unsafe extern "C" {
    pub fn xfs_bulkstat_one(
        breq: *mut xfs_ibulk,
        formatter: bulkstat_one_fmt_pf,
    ) -> c_int;
    pub fn xfs_bulkstat(
        breq: *mut xfs_ibulk,
        formatter: bulkstat_one_fmt_pf,
    ) -> c_int;
    pub fn xfs_bulkstat_to_bstat(
        mp: *mut xfs_mount,
        bs1: *mut xfs_bstat,
        bstat: *const xfs_bulkstat,
    );
}

pub type inumbers_fmt_pf = Option<unsafe extern "C" fn(
    breq: *mut xfs_ibulk,
    igrp: *const xfs_inumbers,
) -> c_int>;

unsafe extern "C" {
    pub fn xfs_inumbers(
        breq: *mut xfs_ibulk,
        formatter: inumbers_fmt_pf,
    ) -> c_int;
    pub fn xfs_inumbers_to_inogrp(
        ig1: *mut xfs_inogrp,
        ig: *const xfs_inumbers,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
