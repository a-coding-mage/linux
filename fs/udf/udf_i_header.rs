/* SPDX-License-Identifier: GPL-2.0 */

// C header guard omitted; this file is intended to be included as a Rust item
// translation. Types supplied by other headers are intentionally unresolved
// here: buffer_head, kernel_lb_addr, rw_semaphore, mapping_metadata_bhs,
// spinlock_t, inode, and timespec64.

#[repr(C)]
pub struct extent_position {
    pub bh: *mut buffer_head,
    pub offset: u32,
    pub block: kernel_lb_addr,
}

#[repr(C)]
pub struct udf_ext_cache {
    /* Extent position */
    pub epos: extent_position,
    /* Start logical offset in bytes */
    pub lstart: i64,
}

/*
 * The i_data_sem and i_mutex serve for protection of allocation information
 * of a regular files and symlinks. This includes all extents belonging to
 * the file/symlink, a fact whether data are in-inode or in external data
 * blocks, preallocation, goal block information... When extents are read,
 * i_mutex or i_data_sem must be held (for reading is enough in case of
 * i_data_sem). When extents are changed, i_data_sem must be held for writing
 * and also i_mutex must be held.
 *
 * For directories i_mutex is used for all the necessary protection.
 */

#[repr(C)]
pub struct udf_inode_info {
    pub i_crtime: timespec64,
    /* Physical address of inode */
    pub i_location: kernel_lb_addr,
    pub i_unique: u64,
    pub i_lenEAttr: u32,
    pub i_lenAlloc: u32,
    pub i_lenExtents: u64,
    pub i_next_alloc_block: u32,
    pub i_next_alloc_goal: u32,
    pub i_checkpoint: u32,
    pub i_extraPerms: u32,
    // C bit-fields: widths are 3, 1, 1, 1, 1, 1, and 24 respectively.
    pub i_alloc_type: u32,
    pub i_efe: u32,       /* extendedFileEntry */
    pub i_use: u32,       /* unallocSpaceEntry */
    pub i_strat4096: u32,
    pub i_streamdir: u32,
    pub i_hidden: u32,    /* hidden system inode */
    pub reserved: u32,
    pub i_data: *mut u8,
    pub i_locStreamdir: kernel_lb_addr,
    pub i_lenStreams: u64,
    pub i_data_sem: rw_semaphore,
    pub i_metadata_bhs: mapping_metadata_bhs,
    pub cached_extent: udf_ext_cache,
    /* Spinlock for protecting extent cache */
    pub i_extent_cache_lock: spinlock_t,
    pub vfs_inode: inode,
}

#[inline]
pub unsafe fn UDF_I(inode: *mut inode) -> *mut udf_inode_info {
    // Direct Rust equivalent of container_of(inode, udf_inode_info, vfs_inode).
    (inode as *mut u8).sub(std::mem::offset_of!(udf_inode_info, vfs_inode))
        as *mut udf_inode_info
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
