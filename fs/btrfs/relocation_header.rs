/* SPDX-License-Identifier: GPL-2.0 */

// Translated from btrfs/relocation.h.
// The Linux type include and build-time definitions are supplied by dependencies.

use core::ffi::c_void;

#[repr(C)]
pub struct extent_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_fs_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_trans_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_ordered_extent {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_pending_snapshot {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_path {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_chunk_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_block_group {
    pub fs_info: *mut btrfs_fs_info,
    pub flags: u64,
}

extern "C" {
    pub fn btrfs_fs_incompat(fs_info: *const btrfs_fs_info, feature: u64) -> bool;

    pub fn btrfs_relocate_block_group(
        fs_info: *mut btrfs_fs_info,
        group_start: u64,
        verbose: bool,
    ) -> i32;
    pub fn btrfs_init_reloc_root(
        trans: *mut btrfs_trans_handle,
        root: *mut btrfs_root,
    ) -> i32;
    pub fn btrfs_update_reloc_root(
        trans: *mut btrfs_trans_handle,
        root: *mut btrfs_root,
    ) -> i32;
    pub fn btrfs_recover_relocation(fs_info: *mut btrfs_fs_info) -> i32;
    pub fn btrfs_reloc_clone_csums(ordered: *mut btrfs_ordered_extent) -> i32;
    pub fn btrfs_reloc_cow_block(
        trans: *mut btrfs_trans_handle,
        root: *mut btrfs_root,
        buf: *const extent_buffer,
        cow: *mut extent_buffer,
    ) -> i32;
    pub fn btrfs_reloc_pre_snapshot(
        pending: *mut btrfs_pending_snapshot,
        bytes_to_reserve: *mut u64,
    );
    pub fn btrfs_reloc_post_snapshot(
        trans: *mut btrfs_trans_handle,
        pending: *mut btrfs_pending_snapshot,
    ) -> i32;
    pub fn btrfs_should_cancel_balance(fs_info: *const btrfs_fs_info) -> i32;
    pub fn find_reloc_root(fs_info: *mut btrfs_fs_info, bytenr: u64) -> *mut btrfs_root;
    pub fn btrfs_should_ignore_reloc_root(root: *const btrfs_root) -> bool;
    pub fn btrfs_get_reloc_bg_bytenr(fs_info: *mut btrfs_fs_info) -> u64;
    pub fn btrfs_translate_remap(
        fs_info: *mut btrfs_fs_info,
        logical: *mut u64,
        length: *mut u64,
    ) -> i32;
    pub fn btrfs_remove_extent_from_remap_tree(
        trans: *mut btrfs_trans_handle,
        path: *mut btrfs_path,
        bytenr: u64,
        num_bytes: u64,
    ) -> i32;
    pub fn btrfs_last_identity_remap_gone(
        chunk_map: *mut btrfs_chunk_map,
        bg: *mut btrfs_block_group,
    ) -> i32;
}

#[inline]
pub unsafe fn should_relocate_using_remap_tree(bg: *const btrfs_block_group) -> bool {
    if !btrfs_fs_incompat((*bg).fs_info, REMAP_TREE) {
        return false;
    }

    if (*bg).flags & (BTRFS_BLOCK_GROUP_SYSTEM | BTRFS_BLOCK_GROUP_METADATA_REMAP) != 0 {
        return false;
    }

    true
}

// Supplied by the surrounding btrfs translation/dependency set.
extern "C" {
    pub static REMAP_TREE: u64;
    pub static BTRFS_BLOCK_GROUP_SYSTEM: u64;
    pub static BTRFS_BLOCK_GROUP_METADATA_REMAP: u64;
}

const _: *const c_void = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
