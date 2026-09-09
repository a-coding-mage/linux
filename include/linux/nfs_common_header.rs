/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This file contains constants and methods used by both NFS client and server.
 */

// C dependencies: <linux/errno.h> and <uapi/linux/nfs.h>.

/* Mapping from NFS error code to "errno" error code. */

extern "C" {
    pub fn nfs_stat_to_errno(status: nfs_stat) -> i32;
    pub fn nfs4_stat_to_errno(stat: i32) -> i32;
    pub fn nfs_localio_errno_to_nfs4_stat(errno: i32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
