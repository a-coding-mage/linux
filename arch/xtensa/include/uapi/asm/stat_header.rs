/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * include/asm-xtensa/stat.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2007 Tensilica Inc.
 */

pub const STAT_HAVE_NSEC: i32 = 1;

#[repr(C)]
pub struct stat {
    pub st_dev: ::kernel::ffi::c_ulong,
    pub st_ino: ::kernel::ffi::c_ulong,
    pub st_mode: ::kernel::ffi::c_uint,
    pub st_nlink: ::kernel::ffi::c_uint,
    pub st_uid: ::kernel::ffi::c_uint,
    pub st_gid: ::kernel::ffi::c_uint,
    pub st_rdev: ::kernel::ffi::c_ulong,
    pub st_size: ::kernel::ffi::c_long,
    pub st_blksize: ::kernel::ffi::c_ulong,
    pub st_blocks: ::kernel::ffi::c_ulong,
    pub st_atime: ::kernel::ffi::c_ulong,
    pub st_atime_nsec: ::kernel::ffi::c_ulong,
    pub st_mtime: ::kernel::ffi::c_ulong,
    pub st_mtime_nsec: ::kernel::ffi::c_ulong,
    pub st_ctime: ::kernel::ffi::c_ulong,
    pub st_ctime_nsec: ::kernel::ffi::c_ulong,
    pub __unused4: ::kernel::ffi::c_ulong,
    pub __unused5: ::kernel::ffi::c_ulong,
}

#[repr(C)]
pub struct stat64 {
    pub st_dev: ::kernel::ffi::c_ulonglong, // Device
    pub st_ino: ::kernel::ffi::c_ulonglong, // File serial number
    pub st_mode: ::kernel::ffi::c_uint, // File mode.
    pub st_nlink: ::kernel::ffi::c_uint, // Link count.
    pub st_uid: ::kernel::ffi::c_uint, // User ID of the file's owner.
    pub st_gid: ::kernel::ffi::c_uint, // Group ID of the file's group.
    pub st_rdev: ::kernel::ffi::c_ulonglong, // Device number, if device.
    pub st_size: ::kernel::ffi::c_longlong, // Size of file, in bytes.
    pub st_blksize: ::kernel::ffi::c_ulong, // Optimal block size for I/O.
    pub __unused2: ::kernel::ffi::c_ulong,
    pub st_blocks: ::kernel::ffi::c_ulonglong, // Number 512-byte blocks allocated.
    pub st_atime: ::kernel::ffi::c_ulong, // Time of last access.
    pub st_atime_nsec: ::kernel::ffi::c_ulong,
    pub st_mtime: ::kernel::ffi::c_ulong, // Time of last modification.
    pub st_mtime_nsec: ::kernel::ffi::c_ulong,
    pub st_ctime: ::kernel::ffi::c_ulong, // Time of last status change.
    pub st_ctime_nsec: ::kernel::ffi::c_ulong,
    pub __unused4: ::kernel::ffi::c_ulong,
    pub __unused5: ::kernel::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
