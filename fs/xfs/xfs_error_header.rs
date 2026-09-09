// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2002,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Forward declaration: struct xfs_mount;
#[repr(C)]
pub struct xfs_mount {
    _private: [u8; 0],
}

extern "C" {
    pub fn xfs_error_report(
        tag: *const ::std::os::raw::c_char,
        level: ::std::os::raw::c_int,
        mp: *mut xfs_mount,
        filename: *const ::std::os::raw::c_char,
        linenum: ::std::os::raw::c_int,
        failaddr: xfs_failaddr_t,
    );
    pub fn xfs_corruption_error(
        tag: *const ::std::os::raw::c_char,
        level: ::std::os::raw::c_int,
        mp: *mut xfs_mount,
        buf: *const ::std::ffi::c_void,
        bufsize: usize,
        filename: *const ::std::os::raw::c_char,
        linenum: ::std::os::raw::c_int,
        failaddr: xfs_failaddr_t,
    );
    pub fn xfs_buf_corruption_error(bp: *mut xfs_buf, fa: xfs_failaddr_t);
    pub fn xfs_buf_verifier_error(
        bp: *mut xfs_buf,
        error: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
        buf: *const ::std::ffi::c_void,
        bufsz: usize,
        failaddr: xfs_failaddr_t,
    );
    pub fn xfs_verifier_error(
        bp: *mut xfs_buf,
        error: ::std::os::raw::c_int,
        failaddr: xfs_failaddr_t,
    );
    pub fn xfs_inode_verifier_error(
        ip: *mut xfs_inode,
        error: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
        buf: *const ::std::ffi::c_void,
        bufsz: usize,
        failaddr: xfs_failaddr_t,
    );
}

#[macro_export]
macro_rules! XFS_ERROR_REPORT {
    ($e:expr, $lvl:expr, $mp:expr) => {
        unsafe { $crate::xfs_error_report($e, $lvl, $mp, file!().as_ptr() as *const _, line!() as _, core::ptr::null_mut()) }
    };
}

#[macro_export]
macro_rules! XFS_CORRUPTION_ERROR {
    ($e:expr, $lvl:expr, $mp:expr, $buf:expr, $bufsize:expr) => {
        unsafe { $crate::xfs_corruption_error($e, $lvl, $mp, $buf, $bufsize, file!().as_ptr() as *const _, line!() as _, core::ptr::null_mut()) }
    };
}

pub const XFS_ERRLEVEL_OFF: ::std::os::raw::c_int = 0;
pub const XFS_ERRLEVEL_LOW: ::std::os::raw::c_int = 1;
pub const XFS_ERRLEVEL_HIGH: ::std::os::raw::c_int = 5;
pub const XFS_CORRUPTION_DUMP_LEN: usize = 128;

#[cfg(debug_assertions)]
extern "C" {
    pub fn xfs_errortag_init(mp: *mut xfs_mount) -> ::std::os::raw::c_int;
    pub fn xfs_errortag_del(mp: *mut xfs_mount);
    pub fn xfs_errortag_test(mp: *mut xfs_mount, file: *const ::std::os::raw::c_char, line: ::std::os::raw::c_int, error_tag: u32) -> bool;
    pub fn xfs_errortag_delay(mp: *mut xfs_mount, file: *const ::std::os::raw::c_char, line: ::std::os::raw::c_int, error_tag: u32);
    pub fn xfs_errortag_add(mp: *mut xfs_mount, error_tag: u32) -> ::std::os::raw::c_int;
    pub fn xfs_errortag_add_name(mp: *mut xfs_mount, tag_name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn xfs_errortag_copy(dst_mp: *mut xfs_mount, src_mp: *mut xfs_mount);
    pub fn xfs_errortag_clearall(mp: *mut xfs_mount) -> ::std::os::raw::c_int;
}

#[cfg(not(debug_assertions))]
pub const fn xfs_errortag_init(_: *mut xfs_mount) -> i32 { 0 }
#[cfg(not(debug_assertions))]
pub const fn xfs_errortag_del(_: *mut xfs_mount) {}
#[cfg(not(debug_assertions))]
pub const fn xfs_errortag_add(_: *mut xfs_mount, _: u32) -> i32 { -38 }
#[cfg(not(debug_assertions))]
pub const fn xfs_errortag_add_name(_: *mut xfs_mount, _: *const ::std::os::raw::c_char) -> i32 { -38 }
#[cfg(not(debug_assertions))]
pub const fn xfs_errortag_clearall(_: *mut xfs_mount) -> i32 { -38 }

pub const XFS_NO_PTAG: u32 = 0u32;
pub const XFS_PTAG_IFLUSH: u32 = 1u32 << 0;
pub const XFS_PTAG_LOGRES: u32 = 1u32 << 1;
pub const XFS_PTAG_AILDELETE: u32 = 1u32 << 2;
pub const XFS_PTAG_ERROR_REPORT: u32 = 1u32 << 3;
pub const XFS_PTAG_SHUTDOWN_CORRUPT: u32 = 1u32 << 4;
pub const XFS_PTAG_SHUTDOWN_IOERROR: u32 = 1u32 << 5;
pub const XFS_PTAG_SHUTDOWN_LOGERROR: u32 = 1u32 << 6;
pub const XFS_PTAG_FSBLOCK_ZERO: u32 = 1u32 << 7;
pub const XFS_PTAG_VERIFIER_ERROR: u32 = 1u32 << 8;
pub const XFS_PTAG_MASK: u32 = XFS_PTAG_IFLUSH | XFS_PTAG_LOGRES | XFS_PTAG_AILDELETE | XFS_PTAG_ERROR_REPORT | XFS_PTAG_SHUTDOWN_CORRUPT | XFS_PTAG_SHUTDOWN_IOERROR | XFS_PTAG_SHUTDOWN_LOGERROR | XFS_PTAG_FSBLOCK_ZERO | XFS_PTAG_VERIFIER_ERROR;

// XFS_PTAG_STRINGS is a C initializer-list macro.
pub const XFS_PTAG_STRINGS: &[(u32, &str)] = &[
    (XFS_NO_PTAG, "none"), (XFS_PTAG_IFLUSH, "iflush"),
    (XFS_PTAG_LOGRES, "logres"), (XFS_PTAG_AILDELETE, "aildelete"),
    (XFS_PTAG_ERROR_REPORT, "error_report"), (XFS_PTAG_SHUTDOWN_CORRUPT, "corrupt"),
    (XFS_PTAG_SHUTDOWN_IOERROR, "ioerror"), (XFS_PTAG_SHUTDOWN_LOGERROR, "logerror"),
    (XFS_PTAG_FSBLOCK_ZERO, "fsb_zero"), (XFS_PTAG_VERIFIER_ERROR, "verifier"),
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
