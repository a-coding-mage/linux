// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2017 Oracle.  All Rights Reserved.
 *
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */

// C dependency declarations supplied by other headers.
pub struct fsmap;
pub struct list_head;
pub struct super_block;

/* internal fsmap representation */
#[repr(C)]
pub struct ext4_fsmap {
    pub fmr_list: list_head,
    pub fmr_device: dev_t,       /* device id */
    pub fmr_flags: u32,          /* mapping flags */
    pub fmr_physical: u64,       /* device offset of segment */
    pub fmr_owner: u64,          /* owner id */
    pub fmr_length: u64,         /* length of segment, blocks */
}

#[repr(C)]
pub struct ext4_fsmap_head {
    pub fmh_iflags: u32,         /* control flags */
    pub fmh_oflags: u32,         /* output flags */
    pub fmh_count: c_uint,       /* # of entries in array incl. input */
    pub fmh_entries: c_uint,     /* # of entries filled in (output). */

    pub fmh_keys: [ext4_fsmap; 2], /* low and high keys */
}

extern "C" {
    pub fn ext4_fsmap_from_internal(
        sb: *mut super_block,
        dest: *mut fsmap,
        src: *mut ext4_fsmap,
    );
    pub fn ext4_fsmap_to_internal(
        sb: *mut super_block,
        dest: *mut ext4_fsmap,
        src: *mut fsmap,
    );
}

/* fsmap to userspace formatter - copy to user & advance pointer */
pub type ext4_fsmap_format_t = Option<unsafe extern "C" fn(*mut ext4_fsmap, *mut core::ffi::c_void) -> i32>;

extern "C" {
    pub fn ext4_getfsmap(
        sb: *mut super_block,
        head: *mut ext4_fsmap_head,
        formatter: ext4_fsmap_format_t,
        arg: *mut core::ffi::c_void,
    ) -> i32;
}

pub const EXT4_QUERY_RANGE_ABORT: i32 = 1;
pub const EXT4_QUERY_RANGE_CONTINUE: i32 = 0;

/* fmr_owner special values for FS_IOC_GETFSMAP; some share w/ XFS */
pub const EXT4_FMR_OWN_FREE: u64 = FMR_OWN_FREE;          /* free space */
pub const EXT4_FMR_OWN_UNKNOWN: u64 = FMR_OWN_UNKNOWN;    /* unknown owner */
pub const EXT4_FMR_OWN_FS: u64 = FMR_OWNER(b'X', 1);       /* static fs metadata */
pub const EXT4_FMR_OWN_LOG: u64 = FMR_OWNER(b'X', 2);     /* journalling log */
pub const EXT4_FMR_OWN_INODES: u64 = FMR_OWNER(b'X', 5);  /* inodes */
pub const EXT4_FMR_OWN_GDT: u64 = FMR_OWNER(b'f', 1);     /* group descriptors */
pub const EXT4_FMR_OWN_RESV_GDT: u64 = FMR_OWNER(b'f', 2); /* reserved gdt blocks */
pub const EXT4_FMR_OWN_BLKBM: u64 = FMR_OWNER(b'f', 3);   /* block bitmap */
pub const EXT4_FMR_OWN_INOBM: u64 = FMR_OWNER(b'f', 4);   /* inode bitmap */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
