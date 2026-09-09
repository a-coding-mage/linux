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
    pub st_size: u32,
    pub st_atime: u32,
    pub st_mtime: u32,
    pub st_ctime: u32,
}

pub const STAT_HAVE_NSEC: bool = true;

#[repr(C)]
pub struct stat {
    // __ARMEB__ selects the big-endian ARM layout.
    #[cfg(target_endian = "big")]
    pub st_dev: u16,
    #[cfg(target_endian = "big")]
    pub __pad1: u16,
    #[cfg(not(target_endian = "big"))]
    pub st_dev: u32,
    pub st_ino: u32,
    pub st_mode: u16,
    pub st_nlink: u16,
    pub st_uid: u16,
    pub st_gid: u16,
    #[cfg(target_endian = "big")]
    pub st_rdev: u16,
    #[cfg(target_endian = "big")]
    pub __pad2: u16,
    #[cfg(not(target_endian = "big"))]
    pub st_rdev: u32,
    pub st_size: u32,
    pub st_blksize: u32,
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

/* This matches struct stat64 in glibc2.1, hence the absolutely
 * insane amounts of padding around dev_t's.
 * Note: The kernel zero's the padded region because glibc might read them
 * in the hope that the kernel has stretched to using larger sizes.
 */
#[repr(C)]
pub struct stat64 {
    pub st_dev: u64,
    pub __pad0: [u8; 4],

    pub __st_ino: u32,
    pub st_mode: u32,
    pub st_nlink: u32,

    pub st_uid: u32,
    pub st_gid: u32,

    pub st_rdev: u64,
    pub __pad3: [u8; 4],

    pub st_size: i64,
    pub st_blksize: u32,
    pub st_blocks: u64, /* Number 512-byte blocks allocated. */

    pub st_atime: u32,
    pub st_atime_nsec: u32,

    pub st_mtime: u32,
    pub st_mtime_nsec: u32,

    pub st_ctime: u32,
    pub st_ctime_nsec: u32,

    pub st_ino: u64,
}

pub const STAT64_HAS_BROKEN_ST_INO: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
