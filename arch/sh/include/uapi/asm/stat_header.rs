/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

#[repr(C)]
pub struct __old_kernel_stat {
    pub st_dev: u16,
    pub st_ino: u16,
    pub st_mode: u16,
    pub st_nlink: u16,
    pub st_uid: u16,
    pub st_gid: u16,
    pub st_rdev: u16,
    pub st_size: libc::c_ulong,
    pub st_atime: libc::c_ulong,
    pub st_mtime: libc::c_ulong,
    pub st_ctime: libc::c_ulong,
}

#[repr(C)]
pub struct stat {
    pub st_dev: libc::c_ulong,
    pub st_ino: libc::c_ulong,
    pub st_mode: u16,
    pub st_nlink: u16,
    pub st_uid: u16,
    pub st_gid: u16,
    pub st_rdev: libc::c_ulong,
    pub st_size: libc::c_ulong,
    pub st_blksize: libc::c_ulong,
    pub st_blocks: libc::c_ulong,
    pub st_atime: libc::c_ulong,
    pub st_atime_nsec: libc::c_ulong,
    pub st_mtime: libc::c_ulong,
    pub st_mtime_nsec: libc::c_ulong,
    pub st_ctime: libc::c_ulong,
    pub st_ctime_nsec: libc::c_ulong,
    pub __unused4: libc::c_ulong,
    pub __unused5: libc::c_ulong,
}

/* This matches struct stat64 in glibc2.1, hence the absolutely
 * insane amounts of padding around dev_t's.
 */
#[repr(C)]
pub struct stat64 {
    pub st_dev: u64,
    pub __pad0: [u8; 4],

    pub __st_ino: libc::c_ulong,

    pub st_mode: u32,
    pub st_nlink: u32,

    pub st_uid: libc::c_ulong,
    pub st_gid: libc::c_ulong,

    pub st_rdev: u64,
    pub __pad3: [u8; 4],

    pub st_size: i64,
    pub st_blksize: libc::c_ulong,

    pub st_blocks: u64, /* Number 512-byte blocks allocated. */

    pub st_atime: libc::c_ulong,
    pub st_atime_nsec: libc::c_ulong,

    pub st_mtime: libc::c_ulong,
    pub st_mtime_nsec: libc::c_ulong,

    pub st_ctime: libc::c_ulong,
    pub st_ctime_nsec: libc::c_ulong,

    pub st_ino: u64,
}

pub const STAT64_HAS_BROKEN_ST_INO: libc::c_int = 1;
pub const STAT_HAVE_NSEC: libc::c_int = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
