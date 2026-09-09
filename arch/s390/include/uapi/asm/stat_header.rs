/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  S390 version
 *
 *  Derived from "include/asm-i386/stat.h"
 */

#[repr(C)]
pub struct stat {
    pub st_dev: ::core::ffi::c_ulong,
    pub st_ino: ::core::ffi::c_ulong,
    pub st_nlink: ::core::ffi::c_ulong,
    pub st_mode: ::core::ffi::c_uint,
    pub st_uid: ::core::ffi::c_uint,
    pub st_gid: ::core::ffi::c_uint,
    pub __pad1: ::core::ffi::c_uint,
    pub st_rdev: ::core::ffi::c_ulong,
    pub st_size: ::core::ffi::c_ulong,
    pub st_atime: ::core::ffi::c_ulong,
    pub st_atime_nsec: ::core::ffi::c_ulong,
    pub st_mtime: ::core::ffi::c_ulong,
    pub st_mtime_nsec: ::core::ffi::c_ulong,
    pub st_ctime: ::core::ffi::c_ulong,
    pub st_ctime_nsec: ::core::ffi::c_ulong,
    pub st_blksize: ::core::ffi::c_ulong,
    pub st_blocks: ::core::ffi::c_long,
    pub __unused: [::core::ffi::c_ulong; 3],
}

pub const STAT_HAVE_NSEC: ::core::ffi::c_int = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
