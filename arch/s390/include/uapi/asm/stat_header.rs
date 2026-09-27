/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  S390 version
 *
 *  Derived from "include/asm-i386/stat.h"
 */

#[repr(C)]
pub struct stat {
    pub st_dev: ::kernel::ffi::c_ulong,
    pub st_ino: ::kernel::ffi::c_ulong,
    pub st_nlink: ::kernel::ffi::c_ulong,
    pub st_mode: ::kernel::ffi::c_uint,
    pub st_uid: ::kernel::ffi::c_uint,
    pub st_gid: ::kernel::ffi::c_uint,
    pub __pad1: ::kernel::ffi::c_uint,
    pub st_rdev: ::kernel::ffi::c_ulong,
    pub st_size: ::kernel::ffi::c_ulong,
    pub st_atime: ::kernel::ffi::c_ulong,
    pub st_atime_nsec: ::kernel::ffi::c_ulong,
    pub st_mtime: ::kernel::ffi::c_ulong,
    pub st_mtime_nsec: ::kernel::ffi::c_ulong,
    pub st_ctime: ::kernel::ffi::c_ulong,
    pub st_ctime_nsec: ::kernel::ffi::c_ulong,
    pub st_blksize: ::kernel::ffi::c_ulong,
    pub st_blocks: ::kernel::ffi::c_long,
    pub __unused: [::kernel::ffi::c_ulong; 3],
}

pub const STAT_HAVE_NSEC: ::kernel::ffi::c_int = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
