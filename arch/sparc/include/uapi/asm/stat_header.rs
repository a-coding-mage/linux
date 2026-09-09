/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

use core::ffi::{c_long, c_ulong, c_ulonglong};

// 64 bit sparc: defined when the C build has __sparc__ and __arch64__.
#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
#[repr(C)]
pub struct stat {
    pub st_dev: u32,
    pub st_ino: __kernel_ino_t,
    pub st_mode: __kernel_mode_t,
    pub st_nlink: i16,
    pub st_uid: __kernel_uid32_t,
    pub st_gid: __kernel_gid32_t,
    pub st_rdev: u32,
    pub st_size: c_long,
    pub st_atime: c_long,
    pub st_mtime: c_long,
    pub st_ctime: c_long,
    pub st_blksize: c_long,
    pub st_blocks: c_long,
    pub __unused4: [c_ulong; 2],
}

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
#[repr(C)]
pub struct stat64 {
    pub st_dev: c_ulong,
    pub st_ino: c_ulong,
    pub st_nlink: c_ulong,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: u32,
    pub st_rdev: c_ulong,
    pub st_size: c_long,
    pub st_blksize: c_long,
    pub st_blocks: c_long,
    pub st_atime: c_ulong,
    pub st_atime_nsec: c_ulong,
    pub st_mtime: c_ulong,
    pub st_mtime_nsec: c_ulong,
    pub st_ctime: c_ulong,
    pub st_ctime_nsec: c_ulong,
    pub __unused: [c_long; 3],
}

// 32 bit sparc: the C source uses this branch for all other configurations.
#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
#[repr(C)]
pub struct stat {
    pub st_dev: u16,
    pub st_ino: __kernel_ino_t,
    pub st_mode: __kernel_mode_t,
    pub st_nlink: i16,
    pub st_uid: u16,
    pub st_gid: u16,
    pub st_rdev: u16,
    pub st_size: c_long,
    pub st_atime: c_long,
    pub st_atime_nsec: c_ulong,
    pub st_mtime: c_long,
    pub st_mtime_nsec: c_ulong,
    pub st_ctime: c_long,
    pub st_ctime_nsec: c_ulong,
    pub st_blksize: c_long,
    pub st_blocks: c_long,
    pub __unused4: [c_ulong; 2],
}

pub const STAT_HAVE_NSEC: i32 = 1;

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
#[repr(C)]
pub struct stat64 {
    pub st_dev: c_ulonglong,
    pub st_ino: c_ulonglong,
    pub st_mode: u32,
    pub st_nlink: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: c_ulonglong,
    pub __pad3: [u8; 8],
    pub st_size: i64,
    pub st_blksize: u32,
    pub __pad4: [u8; 8],
    pub st_blocks: u32,
    pub st_atime: u32,
    pub st_atime_nsec: u32,
    pub st_mtime: u32,
    pub st_mtime_nsec: u32,
    pub st_ctime: u32,
    pub st_ctime_nsec: u32,
    pub __unused4: u32,
    pub __unused5: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
