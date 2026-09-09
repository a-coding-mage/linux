/* SPDX-License-Identifier: GPL-2.0 */

// Translated from btrfs/uuid-tree.h.

use core::ffi::c_void;

#[repr(C)]
pub struct btrfs_trans_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_fs_info {
    _private: [u8; 0],
}

extern "C" {
    pub fn btrfs_uuid_tree_add(
        trans: *mut btrfs_trans_handle,
        uuid: *const u8,
        type_: u8,
        subid: u64,
    ) -> i32;

    pub fn btrfs_uuid_tree_remove(
        trans: *mut btrfs_trans_handle,
        uuid: *const u8,
        type_: u8,
        subid: u64,
    ) -> i32;

    pub fn btrfs_uuid_tree_check_overflow(
        fs_info: *mut btrfs_fs_info,
        uuid: *const u8,
        type_: u8,
    ) -> i32;

    pub fn btrfs_uuid_tree_iterate(fs_info: *mut btrfs_fs_info) -> i32;

    pub fn btrfs_create_uuid_tree(fs_info: *mut btrfs_fs_info) -> i32;

    pub fn btrfs_uuid_scan_kthread(data: *mut c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
