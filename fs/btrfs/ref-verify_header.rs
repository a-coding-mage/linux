/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2014 Facebook.  All rights reserved.
 */

// The C header guard and Linux header includes have no executable Rust equivalent.

use core::ffi::c_int;

pub type u64 = core::primitive::u64;

#[repr(C)]
pub struct btrfs_fs_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_ref {
    _private: [u8; 0],
}

// CONFIG_BTRFS_DEBUG is represented here by the corresponding Rust cfg feature.
#[cfg(feature = "CONFIG_BTRFS_DEBUG")]
extern "C" {
    pub fn btrfs_build_ref_tree(fs_info: *mut btrfs_fs_info) -> c_int;
    pub fn btrfs_free_ref_cache(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_ref_tree_mod(
        fs_info: *mut btrfs_fs_info,
        generic_ref: *const btrfs_ref,
    ) -> c_int;
    pub fn btrfs_free_ref_tree_range(fs_info: *mut btrfs_fs_info, start: u64, len: u64);
}

#[cfg(feature = "CONFIG_BTRFS_DEBUG")]
#[inline]
pub unsafe fn btrfs_init_ref_verify(fs_info: *mut btrfs_fs_info) {
    // C: spin_lock_init(&fs_info->ref_verify_lock);
    // C: fs_info->block_tree = RB_ROOT;
    // The fields and Linux spinlock/rbtree representations are supplied by
    // the surrounding kernel translation and are intentionally not defined here.
    let _ = fs_info;
}

#[cfg(not(feature = "CONFIG_BTRFS_DEBUG"))]
#[inline]
pub unsafe fn btrfs_build_ref_tree(_fs_info: *mut btrfs_fs_info) -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_BTRFS_DEBUG"))]
#[inline]
pub unsafe fn btrfs_free_ref_cache(_fs_info: *mut btrfs_fs_info) {}

#[cfg(not(feature = "CONFIG_BTRFS_DEBUG"))]
#[inline]
pub unsafe fn btrfs_ref_tree_mod(
    _fs_info: *mut btrfs_fs_info,
    _generic_ref: *const btrfs_ref,
) -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_BTRFS_DEBUG"))]
#[inline]
pub unsafe fn btrfs_free_ref_tree_range(
    _fs_info: *mut btrfs_fs_info,
    _start: u64,
    _len: u64,
) {
}

#[cfg(not(feature = "CONFIG_BTRFS_DEBUG"))]
#[inline]
pub unsafe fn btrfs_init_ref_verify(_fs_info: *mut btrfs_fs_info) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
