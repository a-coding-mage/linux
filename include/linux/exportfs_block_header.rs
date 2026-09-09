/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2014-2026 Christoph Hellwig.
 *
 * Support for exportfs-based layout grants for direct block device access.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external to this header translation.

#[repr(C)]
pub struct inode {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct iomap {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct super_block {
    pub s_export_op: *mut export_operations,
}

#[repr(C)]
pub struct export_operations {
    pub block_ops: *const exportfs_block_ops,
}

#[repr(C)]
pub struct block_device {
    pub bd_disk: *mut gendisk,
}

#[repr(C)]
pub struct gendisk {
    pub fops: *const block_device_operations,
}

#[repr(C)]
pub struct block_device_operations {
    pub pr_ops: *const core::ffi::c_void,
    pub get_unique_id: Option<unsafe extern "C" fn()>,
}

pub type expfs_block_layouts_t = u32;

#[inline]
pub const fn expfs_block_flag(bit: u32) -> expfs_block_layouts_t {
    1u32 << bit
}

pub const EXPFS_BLOCK_IN_BAND_ID: expfs_block_layouts_t = expfs_block_flag(0);
pub const EXPFS_BLOCK_OUT_OF_BAND_ID: expfs_block_layouts_t = expfs_block_flag(1);

#[repr(C)]
pub struct exportfs_block_ops {
    /* Returns the EXPFS_BLOCK_* bitmap of supported layout types. */
    pub layouts_supported:
        Option<unsafe extern "C" fn(sb: *mut super_block) -> expfs_block_layouts_t>,

    /* Get the in-band device unique signature exposed to clients. */
    pub get_uuid: Option<
        unsafe extern "C" fn(
            sb: *mut super_block,
            buf: *mut u8,
            len: *mut u32,
            offset: *mut u64,
        ) -> i32,
    >,

    /* Map blocks for direct block access.  If write is true, also allocate. */
    pub map_blocks: Option<
        unsafe extern "C" fn(
            inode: *mut inode,
            offset: i64,
            len: u64,
            iomap: *mut iomap,
            write: bool,
            device_generation: *mut u32,
        ) -> i32,
    >,

    /* Commit blocks previously handed out by map_blocks and written by client. */
    pub commit_blocks: Option<
        unsafe extern "C" fn(
            inode: *mut inode,
            iomaps: *mut iomap,
            nr_iomaps: i32,
            new_size: i64,
        ) -> i32,
    >,
}

#[inline]
pub unsafe fn exportfs_bdev_supports_out_of_band_id(
    bdev: *mut block_device,
) -> bool {
    let fops = (*(*bdev).bd_disk).fops;
    !(*fops).pr_ops.is_null() && (*fops).get_unique_id.is_some()
}

// CONFIG_EXPORTFS_BLOCK_OPS controls which implementation is selected at build time.
#[cfg(feature = "CONFIG_EXPORTFS_BLOCK_OPS")]
#[inline]
pub unsafe fn exportfs_layouts_supported(
    sb: *mut super_block,
) -> expfs_block_layouts_t {
    let bops = (*(*sb).s_export_op).block_ops;

    if bops.is_null()
        || (*bops).layouts_supported.is_none()
        || (*bops).map_blocks.is_none()
        || (*bops).commit_blocks.is_none()
    {
        return 0;
    }
    ((*bops).layouts_supported.unwrap())(sb)
}

#[cfg(not(feature = "CONFIG_EXPORTFS_BLOCK_OPS"))]
#[inline]
pub unsafe fn exportfs_layouts_supported(
    _sb: *mut super_block,
) -> expfs_block_layouts_t {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
