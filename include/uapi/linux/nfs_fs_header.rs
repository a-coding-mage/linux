/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  linux/include/linux/nfs_fs.h
 *
 *  Copyright (C) 1992  Rick Sladkey
 *
 *  OS-specific nfs filesystem definitions and declarations
 */

/* Dependency supplied by the Linux headers; HZ is intentionally left external. */

/* Default timeout values */
pub const NFS_DEF_UDP_TIMEO: i32 = 11;
pub const NFS_DEF_UDP_RETRANS: i32 = 3;
pub const NFS_DEF_TCP_TIMEO: i32 = 600;
pub const NFS_DEF_TCP_RETRANS: i32 = 2;

pub const NFS_MAX_UDP_TIMEOUT: i32 = 60 * HZ;
pub const NFS_MAX_TCP_TIMEOUT: i32 = 600 * HZ;

pub const NFS_DEF_ACREGMIN: i32 = 3;
pub const NFS_DEF_ACREGMAX: i32 = 60;
pub const NFS_DEF_ACDIRMIN: i32 = 30;
pub const NFS_DEF_ACDIRMAX: i32 = 60;

/*
 * When flushing a cluster of dirty pages, there can be different
 * strategies:
 */
pub const FLUSH_SYNC: i32 = 1; /* file being synced, or contention */
pub const FLUSH_STABLE: i32 = 4; /* commit to stable storage */
pub const FLUSH_LOWPRI: i32 = 8; /* low priority background flush */
pub const FLUSH_HIGHPRI: i32 = 16; /* high priority memory reclaim flush */
pub const FLUSH_COND_STABLE: i32 = 32; /* conditional stable write - only stable
                                         * if everything fits in one RPC */

/*
 * NFS debug flags
 */
pub const NFSDBG_VFS: i32 = 0x0001;
pub const NFSDBG_DIRCACHE: i32 = 0x0002;
pub const NFSDBG_LOOKUPCACHE: i32 = 0x0004;
pub const NFSDBG_PAGECACHE: i32 = 0x0008;
pub const NFSDBG_PROC: i32 = 0x0010;
pub const NFSDBG_XDR: i32 = 0x0020;
pub const NFSDBG_FILE: i32 = 0x0040;
pub const NFSDBG_ROOT: i32 = 0x0080;
pub const NFSDBG_CALLBACK: i32 = 0x0100;
pub const NFSDBG_CLIENT: i32 = 0x0200;
pub const NFSDBG_MOUNT: i32 = 0x0400;
pub const NFSDBG_FSCACHE: i32 = 0x0800; /* unused */
pub const NFSDBG_PNFS: i32 = 0x1000;
pub const NFSDBG_PNFS_LD: i32 = 0x2000;
pub const NFSDBG_STATE: i32 = 0x4000;
pub const NFSDBG_XATTRCACHE: i32 = 0x8000;
pub const NFSDBG_ALL: i32 = 0xFFFF;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
