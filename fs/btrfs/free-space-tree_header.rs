/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2015 Facebook.  All rights reserved.
 */

// C dependency: <linux/bits.h>

#[repr(C)]
pub struct btrfs_caching_control {
    _private: [u8; 0],
}
#[repr(C)]
pub struct btrfs_fs_info {
    _private: [u8; 0],
}
#[repr(C)]
pub struct btrfs_path {
    _private: [u8; 0],
}
#[repr(C)]
pub struct btrfs_block_group {
    _private: [u8; 0],
}
#[repr(C)]
pub struct btrfs_trans_handle {
    _private: [u8; 0],
}
pub struct btrfs_free_space_info;
pub struct btrfs_root;

/*
 * The default size for new free space bitmap items. The last bitmap in a block
 * group may be truncated, and none of the free space tree code assumes that
 * existing bitmaps are this size.
 */
pub const BTRFS_FREE_SPACE_BITMAP_SIZE: u64 = 256;
// BITS_PER_BYTE is supplied by the translated Linux dependency.
pub const BTRFS_FREE_SPACE_BITMAP_BITS: u64 =
    BTRFS_FREE_SPACE_BITMAP_SIZE * BITS_PER_BYTE;

extern "C" {
    pub fn btrfs_set_free_space_tree_thresholds(block_group: *mut btrfs_block_group);
    pub fn btrfs_create_free_space_tree(fs_info: *mut btrfs_fs_info) -> i32;
    pub fn btrfs_delete_free_space_tree(fs_info: *mut btrfs_fs_info) -> i32;
    pub fn btrfs_rebuild_free_space_tree(fs_info: *mut btrfs_fs_info) -> i32;
    pub fn btrfs_load_free_space_tree(caching_ctl: *mut btrfs_caching_control) -> i32;
    pub fn btrfs_add_block_group_free_space(
        trans: *mut btrfs_trans_handle,
        block_group: *mut btrfs_block_group,
    ) -> i32;
    pub fn btrfs_remove_block_group_free_space(
        trans: *mut btrfs_trans_handle,
        block_group: *mut btrfs_block_group,
    ) -> i32;
    pub fn btrfs_add_to_free_space_tree(trans: *mut btrfs_trans_handle, start: u64, size: u64) -> i32;
    pub fn btrfs_remove_from_free_space_tree(
        trans: *mut btrfs_trans_handle,
        start: u64,
        size: u64,
    ) -> i32;
    pub fn btrfs_delete_orphan_free_space_entries(fs_info: *mut btrfs_fs_info) -> i32;
    pub fn btrfs_search_free_space_info(
        trans: *mut btrfs_trans_handle,
        block_group: *mut btrfs_block_group,
        path: *mut btrfs_path,
        cow: i32,
    ) -> *mut btrfs_free_space_info;
    pub fn btrfs_free_space_root(block_group: *mut btrfs_block_group) -> *mut btrfs_root;

    // Preserved from: #ifdef CONFIG_BTRFS_FS_RUN_SANITY_TESTS
    pub fn __btrfs_add_to_free_space_tree(
        trans: *mut btrfs_trans_handle,
        block_group: *mut btrfs_block_group,
        path: *mut btrfs_path,
        start: u64,
        size: u64,
    ) -> i32;
    pub fn __btrfs_remove_from_free_space_tree(
        trans: *mut btrfs_trans_handle,
        block_group: *mut btrfs_block_group,
        path: *mut btrfs_path,
        start: u64,
        size: u64,
    ) -> i32;
    pub fn btrfs_convert_free_space_to_bitmaps(
        trans: *mut btrfs_trans_handle,
        block_group: *mut btrfs_block_group,
        path: *mut btrfs_path,
    ) -> i32;
    pub fn btrfs_convert_free_space_to_extents(
        trans: *mut btrfs_trans_handle,
        block_group: *mut btrfs_block_group,
        path: *mut btrfs_path,
    ) -> i32;
    pub fn btrfs_free_space_test_bit(
        block_group: *mut btrfs_block_group,
        path: *mut btrfs_path,
        offset: u64,
    ) -> bool;
    // Preserved from: #endif
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
