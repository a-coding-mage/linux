/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * FS_IOC_GETFSMAP ioctl infrastructure.
 *
 * Copyright (C) 2017 Oracle.  All Rights Reserved.
 *
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */

/* Dependency: <linux/types.h> supplies __u32, __u64, and __kernel_size_t. */

/*
 * Structure for FS_IOC_GETFSMAP.
 *
 * The memory layout is the scalar values defined in struct fsmap_head,
 * followed by two struct fsmap bounds and an array of returned mappings.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsmap {
    pub fmr_device: __u32,       /* device id */
    pub fmr_flags: __u32,        /* mapping flags */
    pub fmr_physical: __u64,     /* device offset of segment */
    pub fmr_owner: __u64,        /* owner id */
    pub fmr_offset: __u64,       /* file offset of segment */
    pub fmr_length: __u64,       /* length of segment */
    pub fmr_reserved: [__u64; 3], /* must be zero */
}

#[repr(C)]
pub struct fsmap_head {
    pub fmh_iflags: __u32,       /* control flags */
    pub fmh_oflags: __u32,       /* output flags */
    pub fmh_count: __u32,        /* # of entries in array incl. input */
    pub fmh_entries: __u32,      /* # of entries filled in (output). */
    pub fmh_reserved: [__u64; 6], /* must be zero */
    pub fmh_keys: [fsmap; 2],    /* low and high keys for the mapping search */
    pub fmh_recs: [fsmap; 0],    /* returned records */
}

/* Size of an fsmap_head with room for nr records. */
#[inline]
pub unsafe fn fsmap_sizeof(nr: core::ffi::c_uint) -> __kernel_size_t {
    (core::mem::size_of::<fsmap_head>() as __kernel_size_t)
        .wrapping_add((nr as __kernel_size_t).wrapping_mul(core::mem::size_of::<fsmap>() as __kernel_size_t))
}

/* Start the next fsmap query at the end of the current query results. */
#[inline]
pub unsafe fn fsmap_advance(head: *mut fsmap_head) {
    (*head).fmh_keys[0] = (*head).fmh_recs[(*head).fmh_entries.wrapping_sub(1) as usize];
}

/* fmh_iflags values - set by FS_IOC_GETFSMAP caller in the header. */
/* no flags defined yet */
pub const FMH_IF_VALID: __u32 = 0;

/* fmh_oflags values - returned in the header segment only. */
pub const FMH_OF_DEV_T: __u32 = 0x1; /* fmr_device values will be dev_t */

/* fmr_flags values - returned for each non-header segment */
pub const FMR_OF_PREALLOC: __u32 = 0x1; /* segment = unwritten pre-allocation */
pub const FMR_OF_ATTR_FORK: __u32 = 0x2; /* segment = attribute fork */
pub const FMR_OF_EXTENT_MAP: __u32 = 0x4; /* segment = extent map */
pub const FMR_OF_SHARED: __u32 = 0x8; /* segment = shared with another file */
pub const FMR_OF_SPECIAL_OWNER: __u32 = 0x10; /* owner is a special value */
pub const FMR_OF_LAST: __u32 = 0x20; /* segment is the last in the dataset */

/* Each FS gets to define its own special owner codes. */
#[inline]
pub const fn FMR_OWNER(type_: __u32, code: __u32) -> __u64 {
    ((type_ as __u64) << 32) | ((code as __u64) & 0xFFFF_FFFF)
}

#[inline]
pub const fn FMR_OWNER_TYPE(owner: __u64) -> __u32 {
    (owner >> 32) as __u32
}

#[inline]
pub const fn FMR_OWNER_CODE(owner: __u64) -> __u32 {
    (owner & 0xFFFF_FFFF) as __u32
}

pub const FMR_OWN_FREE: __u64 = FMR_OWNER(0, 1); /* free space */
pub const FMR_OWN_UNKNOWN: __u64 = FMR_OWNER(0, 2); /* unknown owner */
pub const FMR_OWN_METADATA: __u64 = FMR_OWNER(0, 3); /* metadata */

/* Dependency: _IOWR is supplied by the ioctl definitions. */
pub const FS_IOC_GETFSMAP: _ = _IOWR(b'X' as _, 59, fsmap_head);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
