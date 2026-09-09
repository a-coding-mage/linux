/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from the Alpha C header; the original include guard is omitted. */

#[repr(C)]
pub struct stat {
    pub st_dev: u32,
    pub st_ino: u32,
    pub st_mode: u32,
    pub st_nlink: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: u32,
    pub st_size: i64,
    pub st_atime: u64,
    pub st_mtime: u64,
    pub st_ctime: u64,
    pub st_blksize: u32,
    pub st_blocks: u32,
    pub st_flags: u32,
    pub st_gen: u32,
}

/* The stat64 structure increases the size of dev_t, blkcnt_t, adds
   nanosecond resolution times, and padding for expansion. */
#[repr(C)]
pub struct stat64 {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blocks: u64,

    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_blksize: u32,
    pub st_nlink: u32,
    pub __pad0: u32,

    pub st_atime: u64,
    pub st_atime_nsec: u64,
    pub st_mtime: u64,
    pub st_mtime_nsec: u64,
    pub st_ctime: u64,
    pub st_ctime_nsec: u64,
    pub __unused: [i64; 3],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
