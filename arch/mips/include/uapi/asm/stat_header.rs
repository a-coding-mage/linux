/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 1999, 2000 Ralf Baechle
 * Copyright (C) 2000 Silicon Graphics, Inc.
 */

/* The following cfg names correspond to the C _MIPS_SIM ABI conditions. */
#[cfg(any(mips_sim_abi32, mips_sim_nabi32))]
#[repr(C)]
pub struct stat {
    pub st_dev: u32,
    pub st_pad1: [::core::ffi::c_long; 3], /* Reserved for network id */
    pub st_ino: __kernel_ino_t,
    pub st_mode: __kernel_mode_t,
    pub st_nlink: u32,
    pub st_uid: __kernel_uid32_t,
    pub st_gid: __kernel_gid32_t,
    pub st_rdev: u32,
    pub st_pad2: [::core::ffi::c_long; 2],
    pub st_size: ::core::ffi::c_long,
    pub st_pad3: ::core::ffi::c_long,
    /*
     * Actually this should be timestruc_t st_atime, st_mtime and st_ctime
     * but we don't have it under Linux.
     */
    pub st_atime: ::core::ffi::c_long,
    pub st_atime_nsec: ::core::ffi::c_long,
    pub st_mtime: ::core::ffi::c_long,
    pub st_mtime_nsec: ::core::ffi::c_long,
    pub st_ctime: ::core::ffi::c_long,
    pub st_ctime_nsec: ::core::ffi::c_long,
    pub st_blksize: ::core::ffi::c_long,
    pub st_blocks: ::core::ffi::c_long,
    pub st_pad4: [::core::ffi::c_long; 14],
}

/*
 * This matches struct stat64 in glibc2.1, hence the absolutely insane
 * amounts of padding around dev_t's.  The memory layout is the same as of
 * struct stat of the 64-bit kernel.
 */
#[cfg(any(mips_sim_abi32, mips_sim_nabi32))]
#[repr(C)]
pub struct stat64 {
    pub st_dev: ::core::ffi::c_ulong,
    pub st_pad0: [::core::ffi::c_ulong; 3], /* Reserved for st_dev expansion  */
    pub st_ino: u64,
    pub st_mode: __kernel_mode_t,
    pub st_nlink: u32,
    pub st_uid: __kernel_uid32_t,
    pub st_gid: __kernel_gid32_t,
    pub st_rdev: ::core::ffi::c_ulong,
    pub st_pad1: [::core::ffi::c_ulong; 3], /* Reserved for st_rdev expansion  */
    pub st_size: i64,
    /*
     * Actually this should be timestruc_t st_atime, st_mtime and st_ctime
     * but we don't have it under Linux.
     */
    pub st_atime: ::core::ffi::c_long,
    pub st_atime_nsec: ::core::ffi::c_ulong, /* Reserved for st_atime expansion  */
    pub st_mtime: ::core::ffi::c_long,
    pub st_mtime_nsec: ::core::ffi::c_ulong, /* Reserved for st_mtime expansion  */
    pub st_ctime: ::core::ffi::c_long,
    pub st_ctime_nsec: ::core::ffi::c_ulong, /* Reserved for st_ctime expansion  */
    pub st_blksize: ::core::ffi::c_ulong,
    pub st_pad2: ::core::ffi::c_ulong,
    pub st_blocks: i64,
}

/* The memory layout is the same as of struct stat64 of the 32-bit kernel.  */
#[cfg(mips_sim_abi64)]
#[repr(C)]
pub struct stat {
    pub st_dev: u32,
    pub st_pad0: [u32; 3], /* Reserved for st_dev expansion */
    pub st_ino: ::core::ffi::c_ulong,
    pub st_mode: __kernel_mode_t,
    pub st_nlink: u32,
    pub st_uid: __kernel_uid32_t,
    pub st_gid: __kernel_gid32_t,
    pub st_rdev: u32,
    pub st_pad1: [u32; 3], /* Reserved for st_rdev expansion */
    pub st_size: ::core::ffi::c_long,
    /*
     * Actually this should be timestruc_t st_atime, st_mtime and st_ctime
     * but we don't have it under Linux.
     */
    pub st_atime: u32,
    pub st_atime_nsec: u32,
    pub st_mtime: u32,
    pub st_mtime_nsec: u32,
    pub st_ctime: u32,
    pub st_ctime_nsec: u32,
    pub st_blksize: u32,
    pub st_pad2: u32,
    pub st_blocks: ::core::ffi::c_ulong,
}

pub const STAT_HAVE_NSEC: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
