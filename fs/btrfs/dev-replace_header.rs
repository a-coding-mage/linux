/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) STRATO AG 2012.  All rights reserved.
 */

// Declarations corresponding to the C header's Linux type/compiler includes
// are expected to be supplied by the surrounding translation unit.

#[repr(C)]
pub struct btrfs_ioctl_dev_replace_args {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_fs_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_trans_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_dev_replace {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_block_group {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_device {
    _private: [u8; 0],
}

extern "C" {
    pub fn btrfs_init_dev_replace(fs_info: *mut btrfs_fs_info) -> i32;
    pub fn btrfs_run_dev_replace(trans: *mut btrfs_trans_handle) -> i32;
    pub fn btrfs_dev_replace_by_ioctl(
        fs_info: *mut btrfs_fs_info,
        args: *mut btrfs_ioctl_dev_replace_args,
    ) -> i32;
    pub fn btrfs_dev_replace_status(
        fs_info: *mut btrfs_fs_info,
        args: *mut btrfs_ioctl_dev_replace_args,
    );
    pub fn btrfs_dev_replace_cancel(fs_info: *mut btrfs_fs_info) -> i32;
    pub fn btrfs_dev_replace_suspend_for_unmount(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_resume_dev_replace_async(fs_info: *mut btrfs_fs_info) -> i32;

    // C declaration has the __pure compiler attribute.
    pub fn btrfs_dev_replace_is_ongoing(dev_replace: *mut btrfs_dev_replace) -> bool;
    pub fn btrfs_finish_block_group_to_copy(
        srcdev: *mut btrfs_device,
        cache: *mut btrfs_block_group,
        physical: u64,
    ) -> bool;
    pub fn btrfs_bio_counter_inc_blocked(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_bio_counter_sub(fs_info: *mut btrfs_fs_info, amount: i64);
}

#[inline]
pub unsafe fn btrfs_bio_counter_dec(fs_info: *mut btrfs_fs_info) {
    btrfs_bio_counter_sub(fs_info, 1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
