/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * FS_IOC_FIEMAP ioctl infrastructure.
 *
 * Some portions copyright (C) 2007 Cluster File Systems, Inc
 *
 * Authors: Mark Fasheh <mfasheh@suse.com>
 *          Kalpak Shah <kalpak.shah@sun.com>
 *          Andreas Dilger <adilger@sun.com>
 */

// Dependency equivalent of #include <linux/types.h>:
// __u64 maps to u64 and __u32 maps to u32.

/**
 * struct fiemap_extent - description of one fiemap extent
 * @fe_logical: byte offset of the extent in the file
 * @fe_physical: byte offset of extent on disk
 * @fe_length: length in bytes for this extent
 * @fe_flags: FIEMAP_EXTENT_* flags for this extent
 */
#[repr(C)]
pub struct fiemap_extent {
    pub fe_logical: u64,
    pub fe_physical: u64,
    pub fe_length: u64,
    /* private: */
    pub fe_reserved64: [u64; 2],
    /* public: */
    pub fe_flags: u32,
    /* private: */
    pub fe_reserved: [u32; 3],
}

/**
 * struct fiemap - file extent mappings
 * @fm_start: byte offset (inclusive) at which to start mapping (in)
 * @fm_length: logical length of mapping which userspace wants (in)
 * @fm_flags: FIEMAP_FLAG_* flags for request (in/out)
 * @fm_mapped_extents: number of extents that were mapped (out)
 * @fm_extent_count: size of fm_extents array (in)
 * @fm_extents: array of mapped extents (out)
 */
#[repr(C)]
pub struct fiemap {
    pub fm_start: u64,
    pub fm_length: u64,
    pub fm_flags: u32,
    pub fm_mapped_extents: u32,
    pub fm_extent_count: u32,
    /* private: */
    pub fm_reserved: u32,
    /* public: */
    pub fm_extents: [fiemap_extent; 0],
}

pub const FIEMAP_MAX_OFFSET: u64 = !0u64;

/* flags used in fm_flags: */
pub const FIEMAP_FLAG_SYNC: u32 = 0x00000001; /* sync file data before map */
pub const FIEMAP_FLAG_XATTR: u32 = 0x00000002; /* map extended attribute tree */
pub const FIEMAP_FLAG_CACHE: u32 = 0x00000004; /* request caching of the extents */

pub const FIEMAP_FLAGS_COMPAT: u32 = FIEMAP_FLAG_SYNC | FIEMAP_FLAG_XATTR;

/* flags used in fe_flags: */
pub const FIEMAP_EXTENT_LAST: u32 = 0x00000001; /* Last extent in file. */
pub const FIEMAP_EXTENT_UNKNOWN: u32 = 0x00000002; /* Data location unknown. */
pub const FIEMAP_EXTENT_DELALLOC: u32 = 0x00000004; /* Location still pending.
                                                       * Sets EXTENT_UNKNOWN. */
pub const FIEMAP_EXTENT_ENCODED: u32 = 0x00000008; /* Data can not be read
                                                       * while fs is unmounted */
pub const FIEMAP_EXTENT_DATA_ENCRYPTED: u32 = 0x00000080; /* Data is encrypted by fs.
                                                             * Sets EXTENT_NO_BYPASS. */
pub const FIEMAP_EXTENT_NOT_ALIGNED: u32 = 0x00000100; /* Extent offsets may not be
                                                           * block aligned. */
pub const FIEMAP_EXTENT_DATA_INLINE: u32 = 0x00000200; /* Data mixed with metadata.
                                                           * Sets EXTENT_NOT_ALIGNED.*/
pub const FIEMAP_EXTENT_DATA_TAIL: u32 = 0x00000400; /* Multiple files in block.
                                                         * Sets EXTENT_NOT_ALIGNED.*/
pub const FIEMAP_EXTENT_UNWRITTEN: u32 = 0x00000800; /* Space allocated, but
                                                         * no data (i.e. zero). */
pub const FIEMAP_EXTENT_MERGED: u32 = 0x00001000; /* File does not natively
                                                     * support extents. Result
                                                     * merged for efficiency. */
pub const FIEMAP_EXTENT_SHARED: u32 = 0x00002000; /* Space shared with other
                                                     * files. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
