/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 */

// Dependency supplied by the surrounding translation unit: <linux/types.h>.

pub const STAT_HAVE_NSEC: i32 = 1;

// Corresponds to the source condition `!__powerpc64__`.
#[cfg(not(target_arch = "powerpc64"))]
#[repr(C)]
pub struct __old_kernel_stat {
    pub st_dev: u16,
    pub st_ino: u16,
    pub st_mode: u16,
    pub st_nlink: u16,
    pub st_uid: u16,
    pub st_gid: u16,
    pub st_rdev: u16,
    pub st_size: ::core::ffi::c_ulong,
    pub st_atime: ::core::ffi::c_ulong,
    pub st_mtime: ::core::ffi::c_ulong,
    pub st_ctime: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct stat {
    pub st_dev: ::core::ffi::c_ulong,
    pub st_ino: __kernel_ino_t,
    // Corresponds to the source condition `__powerpc64__`.
    #[cfg(target_arch = "powerpc64")]
    pub st_nlink: ::core::ffi::c_ulong,
    #[cfg(target_arch = "powerpc64")]
    pub st_mode: __kernel_mode_t,
    #[cfg(not(target_arch = "powerpc64"))]
    pub st_mode: __kernel_mode_t,
    #[cfg(not(target_arch = "powerpc64"))]
    pub st_nlink: u16,
    pub st_uid: __kernel_uid32_t,
    pub st_gid: __kernel_gid32_t,
    pub st_rdev: ::core::ffi::c_ulong,
    pub st_size: ::core::ffi::c_long,
    pub st_blksize: ::core::ffi::c_ulong,
    pub st_blocks: ::core::ffi::c_ulong,
    pub st_atime: ::core::ffi::c_ulong,
    pub st_atime_nsec: ::core::ffi::c_ulong,
    pub st_mtime: ::core::ffi::c_ulong,
    pub st_mtime_nsec: ::core::ffi::c_ulong,
    pub st_ctime: ::core::ffi::c_ulong,
    pub st_ctime_nsec: ::core::ffi::c_ulong,
    pub __unused4: ::core::ffi::c_ulong,
    pub __unused5: ::core::ffi::c_ulong,
    #[cfg(target_arch = "powerpc64")]
    pub __unused6: ::core::ffi::c_ulong,
}

/* This matches struct stat64 in glibc2.1. Only used for 32 bit. */
#[repr(C)]
pub struct stat64 {
    pub st_dev: ::core::ffi::c_ulonglong, // Device.
    pub st_ino: ::core::ffi::c_ulonglong, // File serial number.
    pub st_mode: ::core::ffi::c_uint, // File mode.
    pub st_nlink: ::core::ffi::c_uint, // Link count.
    pub st_uid: ::core::ffi::c_uint, // User ID of the file's owner.
    pub st_gid: ::core::ffi::c_uint, // Group ID of the file's group.
    pub st_rdev: ::core::ffi::c_ulonglong, // Device number, if device.
    pub __pad2: u16,
    pub st_size: ::core::ffi::c_longlong, // Size of file, in bytes.
    pub st_blksize: ::core::ffi::c_int, // Optimal block size for I/O.
    pub st_blocks: ::core::ffi::c_longlong, // Number 512-byte blocks allocated.
    pub st_atime: ::core::ffi::c_int, // Time of last access.
    pub st_atime_nsec: ::core::ffi::c_uint,
    pub st_mtime: ::core::ffi::c_int, // Time of last modification.
    pub st_mtime_nsec: ::core::ffi::c_uint,
    pub st_ctime: ::core::ffi::c_int, // Time of last status change.
    pub st_ctime_nsec: ::core::ffi::c_uint,
    pub __unused4: ::core::ffi::c_uint,
    pub __unused5: ::core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
