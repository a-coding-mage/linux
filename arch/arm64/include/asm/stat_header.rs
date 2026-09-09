/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependency corresponding to: #include <uapi/asm/stat.h>

// CONFIG_COMPAT
// Dependencies corresponding to: #include <linux/time.h> and #include <asm/compat.h>

/*
 * struct stat64 is needed for compat tasks only. Its definition is different
 * from the generic struct stat64.
 */
#[repr(C)]
pub struct stat64 {
    pub st_dev: compat_u64,
    pub __pad0: [u8; 4],

    pub __st_ino: compat_ulong_t,
    pub st_mode: compat_uint_t,
    pub st_nlink: compat_uint_t,

    pub st_uid: compat_ulong_t,
    pub st_gid: compat_ulong_t,

    pub st_rdev: compat_u64,
    pub __pad3: [u8; 4],

    pub st_size: compat_s64,
    pub st_blksize: compat_ulong_t,
    /// Number of 512-byte blocks allocated.
    pub st_blocks: compat_u64,

    pub st_atime: compat_ulong_t,
    pub st_atime_nsec: compat_ulong_t,

    pub st_mtime: compat_ulong_t,
    pub st_mtime_nsec: compat_ulong_t,

    pub st_ctime: compat_ulong_t,
    pub st_ctime_nsec: compat_ulong_t,

    pub st_ino: compat_u64,
}

pub const STAT64_HAS_BROKEN_ST_INO: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
