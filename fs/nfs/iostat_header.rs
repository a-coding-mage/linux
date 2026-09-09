/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  linux/fs/nfs/iostat.h
 *
 *  Declarations for NFS client per-mount statistics
 *
 *  Copyright (C) 2005, 2006 Chuck Lever <cel@netapp.com>
 *
 */

// #include <linux/percpu.h>
// #include <linux/cache.h>
// #include <linux/nfs_iostat.h>

#[repr(C)]
pub struct nfs_iostats {
    pub bytes: [::core::ffi::c_ulonglong; __NFSIOS_BYTESMAX],
    pub events: [::core::ffi::c_ulong; __NFSIOS_COUNTSMAX],
}

#[inline]
pub unsafe fn nfs_inc_server_stats(
    server: *const nfs_server,
    stat: nfs_stat_eventcounters,
) {
    this_cpu_inc((*server).io_stats.events[stat as usize]);
}

#[inline]
pub unsafe fn nfs_inc_stats(
    inode: *const inode,
    stat: nfs_stat_eventcounters,
) {
    nfs_inc_server_stats(NFS_SERVER(inode), stat);
}

#[inline]
pub unsafe fn nfs_add_server_stats(
    server: *const nfs_server,
    stat: nfs_stat_bytecounters,
    addend: ::core::ffi::c_long,
) {
    this_cpu_add((*server).io_stats.bytes[stat as usize], addend);
}

#[inline]
pub unsafe fn nfs_add_stats(
    inode: *const inode,
    stat: nfs_stat_bytecounters,
    addend: ::core::ffi::c_long,
) {
    nfs_add_server_stats(NFS_SERVER(inode), stat, addend);
}

/*
 * This specialized allocator has to be a macro for its allocations to be
 * accounted separately (to have a separate alloc_tag).
 */
#[macro_export]
macro_rules! nfs_alloc_iostats {
    () => {
        alloc_percpu!(nfs_iostats)
    };
}

#[inline]
pub unsafe fn nfs_free_iostats(stats: *mut nfs_iostats) {
    if !stats.is_null() {
        free_percpu(stats);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
