// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2008 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Translated from the C header guard __XFS_IOCTL_H__.

pub struct xfs_bstat {
    _private: [u8; 0],
}

pub struct xfs_ibulk {
    _private: [u8; 0],
}

pub struct xfs_inogrp {
    _private: [u8; 0],
}

extern "C" {
    pub fn xfs_ioc_swapext(sxp: *mut xfs_swapext_t) -> ::kernel::ffi::c_int;

    pub fn xfs_fileattr_get(
        dentry: *mut dentry,
        fa: *mut file_kattr,
    ) -> ::kernel::ffi::c_int;

    pub fn xfs_fileattr_set(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        fa: *mut file_kattr,
    ) -> ::kernel::ffi::c_int;

    pub fn xfs_file_ioctl(
        filp: *mut file,
        cmd: ::kernel::ffi::c_uint,
        p: ::kernel::ffi::c_ulong,
    ) -> ::kernel::ffi::c_long;

    pub fn xfs_file_compat_ioctl(
        file: *mut file,
        cmd: ::kernel::ffi::c_uint,
        arg: ::kernel::ffi::c_ulong,
    ) -> ::kernel::ffi::c_long;

    pub fn xfs_fsbulkstat_one_fmt(
        breq: *mut xfs_ibulk,
        bstat: *const xfs_bulkstat,
    ) -> ::kernel::ffi::c_int;

    pub fn xfs_fsinumbers_fmt(
        breq: *mut xfs_ibulk,
        igrp: *const xfs_inumbers,
    ) -> ::kernel::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
