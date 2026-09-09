// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2017 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */

// Forward declarations from external dependencies.
pub enum fsmap {}
pub enum fsmap_head {}

/* internal fsmap representation */
#[repr(C)]
pub struct xfs_fsmap {
    pub fmr_device: dev_t,         /* device id */
    pub fmr_flags: u32,            /* mapping flags */
    pub fmr_physical: u64,         /* device offset of segment */
    pub fmr_owner: u64,            /* owner id */
    pub fmr_offset: xfs_fileoff_t, /* file offset of segment */
    pub fmr_length: xfs_filblks_t, /* length of segment, blocks */
}

#[repr(C)]
pub struct xfs_fsmap_head {
    pub fmh_iflags: u32,   /* control flags */
    pub fmh_oflags: u32,   /* output flags */
    pub fmh_count: c_uint, /* # of entries in array incl. input */
    pub fmh_entries: c_uint, /* # of entries filled in (output). */

    pub fmh_keys: [xfs_fsmap; 2], /* low and high keys */
}

/* internal fsmap record format */
#[repr(C)]
pub struct xfs_fsmap_irec {
    pub start_daddr: xfs_daddr_t,
    pub len_daddr: xfs_daddr_t,
    pub owner: u64,       /* extent owner */
    pub offset: u64,      /* offset within the owner */
    pub rm_flags: c_uint, /* rmap state flags */

    /*
     * rmapbt startblock corresponding to start_daddr, if the record came
     * from an rmap btree.
     */
    pub rec_key: xfs_agblock_t,
}

extern "C" {
    pub fn xfs_ioc_getfsmap(ip: *mut xfs_inode, arg: *mut fsmap_head) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
