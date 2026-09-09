/* SPDX-License-Identifier: GPL-2.0 */
/*
 * (C) 2001 Clemson University and The University of Chicago
 *
 * See COPYING in top-level directory.
 */

/*
 * Types and constants shared between user space and kernel space for
 * device interaction using a common protocol.
 *
 * The declarations from upcall.h and downcall.h are supplied by other
 * translated files.
 */

/*
 * Valid orangefs kernel operation types.
 */
pub const ORANGEFS_VFS_OP_INVALID: u32 = 0xFF00_0000;
pub const ORANGEFS_VFS_OP_FILE_IO: u32 = 0xFF00_0001;
pub const ORANGEFS_VFS_OP_LOOKUP: u32 = 0xFF00_0002;
pub const ORANGEFS_VFS_OP_CREATE: u32 = 0xFF00_0003;
pub const ORANGEFS_VFS_OP_GETATTR: u32 = 0xFF00_0004;
pub const ORANGEFS_VFS_OP_REMOVE: u32 = 0xFF00_0005;
pub const ORANGEFS_VFS_OP_MKDIR: u32 = 0xFF00_0006;
pub const ORANGEFS_VFS_OP_READDIR: u32 = 0xFF00_0007;
pub const ORANGEFS_VFS_OP_SETATTR: u32 = 0xFF00_0008;
pub const ORANGEFS_VFS_OP_SYMLINK: u32 = 0xFF00_0009;
pub const ORANGEFS_VFS_OP_RENAME: u32 = 0xFF00_000A;
pub const ORANGEFS_VFS_OP_STATFS: u32 = 0xFF00_000B;
pub const ORANGEFS_VFS_OP_TRUNCATE: u32 = 0xFF00_000C;
pub const ORANGEFS_VFS_OP_RA_FLUSH: u32 = 0xFF00_000D;
pub const ORANGEFS_VFS_OP_FS_MOUNT: u32 = 0xFF00_000E;
pub const ORANGEFS_VFS_OP_FS_UMOUNT: u32 = 0xFF00_000F;
pub const ORANGEFS_VFS_OP_GETXATTR: u32 = 0xFF00_0010;
pub const ORANGEFS_VFS_OP_SETXATTR: u32 = 0xFF00_0011;
pub const ORANGEFS_VFS_OP_LISTXATTR: u32 = 0xFF00_0012;
pub const ORANGEFS_VFS_OP_REMOVEXATTR: u32 = 0xFF00_0013;
pub const ORANGEFS_VFS_OP_PARAM: u32 = 0xFF00_0014;
pub const ORANGEFS_VFS_OP_PERF_COUNT: u32 = 0xFF00_0015;
pub const ORANGEFS_VFS_OP_CANCEL: u32 = 0xFF00_EE00;
pub const ORANGEFS_VFS_OP_FSYNC: u32 = 0xFF00_EE01;
pub const ORANGEFS_VFS_OP_FSKEY: u32 = 0xFF00_EE02;
pub const ORANGEFS_VFS_OP_READDIRPLUS: u32 = 0xFF00_EE03;
pub const ORANGEFS_VFS_OP_FEATURES: u32 = 0xFF00_EE05; /* 2.9.6 */

/* Features is a 64-bit unsigned bitmask. */
pub const ORANGEFS_FEATURE_READAHEAD: u64 = 1;

/*
 * Misc constants. Please retain them as multiples of 8!
 * Otherwise 32-64 bit interactions will be messed up :)
 */
pub const ORANGEFS_MAX_DEBUG_STRING_LEN: u32 = 0x0000_0800;

pub const ORANGEFS_MAX_DIRENT_COUNT_READDIR: u32 = 512;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
