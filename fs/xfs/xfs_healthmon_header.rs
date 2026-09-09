/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2024-2026 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// C header guard: __XFS_HEALTHMON_H__

#[repr(C)]
pub struct xfs_healthmon {
    pub mount_cookie: uintptr_t,
    pub dev: dev_t,
    pub ref_: refcount_t,
    pub lock: mutex,
    pub first_event: *mut xfs_healthmon_event,
    pub last_event: *mut xfs_healthmon_event,
    pub unmount_event: *mut xfs_healthmon_event,
    pub events: ::core::ffi::c_uint,
    // C bit-field: bool verbose:1;
    pub verbose: bool,
    pub wait: wait_queue_head,
    pub buffer: *mut ::core::ffi::c_char,
    pub bufsize: size_t,
    pub bufhead: size_t,
    pub buftail: size_t,
    pub lost_prev_event: ::core::ffi::c_ulonglong,
    pub total_events: ::core::ffi::c_ulonglong,
    pub total_lost: ::core::ffi::c_ulonglong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum xfs_healthmon_type {
    XFS_HEALTHMON_RUNNING,
    XFS_HEALTHMON_LOST,
    XFS_HEALTHMON_UNMOUNT,
    XFS_HEALTHMON_SHUTDOWN,
    XFS_HEALTHMON_SICK,
    XFS_HEALTHMON_CORRUPT,
    XFS_HEALTHMON_HEALTHY,
    XFS_HEALTHMON_MEDIA_ERROR,
    XFS_HEALTHMON_BUFREAD,
    XFS_HEALTHMON_BUFWRITE,
    XFS_HEALTHMON_DIOREAD,
    XFS_HEALTHMON_DIOWRITE,
    XFS_HEALTHMON_DATALOST,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum xfs_healthmon_domain {
    XFS_HEALTHMON_MOUNT,
    XFS_HEALTHMON_FS,
    XFS_HEALTHMON_AG,
    XFS_HEALTHMON_INODE,
    XFS_HEALTHMON_RTGROUP,
    XFS_HEALTHMON_DATADEV,
    XFS_HEALTHMON_RTDEV,
    XFS_HEALTHMON_LOGDEV,
    XFS_HEALTHMON_FILERANGE,
}

#[repr(C)]
pub union xfs_healthmon_event_data {
    pub lostcount: u64,
    pub fsmask: ::core::ffi::c_uint,
    pub group_data: xfs_healthmon_group_data,
    pub inode_data: xfs_healthmon_inode_data,
    pub flags: ::core::ffi::c_uint,
    pub media_data: xfs_healthmon_media_data,
    pub file_range_data: xfs_healthmon_file_range_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_healthmon_group_data {
    pub grpmask: ::core::ffi::c_uint,
    pub group: ::core::ffi::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_healthmon_inode_data {
    pub imask: ::core::ffi::c_uint,
    pub gen: u32,
    pub ino: xfs_ino_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_healthmon_media_data {
    pub daddr: xfs_daddr_t,
    pub bbcount: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_healthmon_file_range_data {
    pub fino: xfs_ino_t,
    pub fpos: loff_t,
    pub flen: u64,
    pub fgen: u32,
    pub error: ::core::ffi::c_int,
}

#[repr(C)]
pub struct xfs_healthmon_event {
    pub next: *mut xfs_healthmon_event,
    pub type_: xfs_healthmon_type,
    pub domain: xfs_healthmon_domain,
    pub time_ns: u64,
    pub data: xfs_healthmon_event_data,
}

extern "C" {
    pub fn xfs_healthmon_unmount(mp: *mut xfs_mount);
    pub fn xfs_healthmon_report_fs(mp: *mut xfs_mount, type_: xfs_healthmon_type,
        old_mask: ::core::ffi::c_uint, new_mask: ::core::ffi::c_uint);
    pub fn xfs_healthmon_report_group(xg: *mut xfs_group, type_: xfs_healthmon_type,
        old_mask: ::core::ffi::c_uint, new_mask: ::core::ffi::c_uint);
    pub fn xfs_healthmon_report_inode(ip: *mut xfs_inode, type_: xfs_healthmon_type,
        old_mask: ::core::ffi::c_uint, new_mask: ::core::ffi::c_uint);
    pub fn xfs_healthmon_report_shutdown(mp: *mut xfs_mount, flags: u32);
    pub fn xfs_healthmon_report_media(mp: *mut xfs_mount, fdev: xfs_device,
        daddr: xfs_daddr_t, bbcount: u64);
    pub fn xfs_healthmon_report_file_ioerror(ip: *mut xfs_inode, p: *const fserror_event);
    pub fn xfs_ioc_health_monitor(file: *mut file, arg: *mut xfs_health_monitor) -> ::core::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
