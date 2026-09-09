/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2023 Western Digital Corporation or its affiliates.
 */

// Dependencies supplied by the surrounding BTRFS translation unit:
// linux/types.h, uapi/linux/btrfs_tree.h, fs.h, and accessors.h.

pub const BTRFS_RST_SUPP_BLOCK_GROUP_MASK: u64 = BTRFS_BLOCK_GROUP_DUP
    | BTRFS_BLOCK_GROUP_RAID1_MASK
    | BTRFS_BLOCK_GROUP_RAID0
    | BTRFS_BLOCK_GROUP_RAID10;

#[repr(C)]
pub struct btrfs_io_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_io_stripe {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_fs_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_ordered_extent {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_trans_handle {
    _private: [u8; 0],
}

extern "C" {
    pub fn btrfs_delete_raid_extent(
        trans: *mut btrfs_trans_handle,
        start: u64,
        length: u64,
    ) -> i32;

    pub fn btrfs_get_raid_extent_offset(
        fs_info: *mut btrfs_fs_info,
        logical: u64,
        length: *mut u64,
        map_type: u64,
        stripe_index: u32,
        stripe: *mut btrfs_io_stripe,
    ) -> i32;

    pub fn btrfs_insert_raid_extent(
        trans: *mut btrfs_trans_handle,
        ordered_extent: *mut btrfs_ordered_extent,
    ) -> i32;

    // Preserved from CONFIG_BTRFS_FS_RUN_SANITY_TESTS. Enable this declaration
    // when that build-time configuration is enabled.
    #[cfg(CONFIG_BTRFS_FS_RUN_SANITY_TESTS)]
    pub fn btrfs_insert_one_raid_extent(
        trans: *mut btrfs_trans_handle,
        bioc: *mut btrfs_io_context,
    ) -> i32;
}

#[inline]
pub unsafe fn btrfs_need_stripe_tree_update(
    fs_info: *mut btrfs_fs_info,
    map_type: u64,
) -> bool {
    let type_: u64 = map_type & BTRFS_BLOCK_GROUP_TYPE_MASK;
    let profile: u64 = map_type & BTRFS_BLOCK_GROUP_PROFILE_MASK;

    if !btrfs_fs_incompat(fs_info, RAID_STRIPE_TREE) {
        return false;
    }

    if type_ != BTRFS_BLOCK_GROUP_DATA {
        return false;
    }

    if profile & BTRFS_RST_SUPP_BLOCK_GROUP_MASK != 0 {
        return true;
    }

    false
}

#[inline]
pub fn btrfs_num_raid_stripes(item_size: u32) -> i32 {
    (item_size / core::mem::size_of::<btrfs_raid_stride>() as u32) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
