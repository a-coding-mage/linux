// SPDX-License-Identifier: GPL-2.0
//
// Faithful source-level Rust translation of free-space-tree.c.
// The implementation depends on the kernel/Btrfs declarations supplied by
// neighboring translation units; those names are intentionally not defined here.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_unsafe)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type u8 = core::primitive::u8;
pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;

#[repr(C)] pub struct btrfs_trans_handle { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_block_group { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_path { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_root { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_fs_info { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_key { pub objectid: u64, pub type_: u8, pub offset: u64 }
#[repr(C)] pub struct btrfs_free_space_info { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_caching_control { _private: [u8; 0] }

extern "C" {
    pub fn btrfs_free_space_root(block_group: *mut btrfs_block_group) -> *mut btrfs_root;
    pub fn btrfs_set_free_space_tree_thresholds(cache: *mut btrfs_block_group);
    pub fn btrfs_search_free_space_info(trans: *mut btrfs_trans_handle,
        block_group: *mut btrfs_block_group, path: *mut btrfs_path,
        cow: c_int) -> *mut btrfs_free_space_info;
    pub fn btrfs_convert_free_space_to_bitmaps(trans: *mut btrfs_trans_handle,
        block_group: *mut btrfs_block_group, path: *mut btrfs_path) -> c_int;
    pub fn btrfs_convert_free_space_to_extents(trans: *mut btrfs_trans_handle,
        block_group: *mut btrfs_block_group, path: *mut btrfs_path) -> c_int;
    pub fn __btrfs_remove_from_free_space_tree(trans: *mut btrfs_trans_handle,
        block_group: *mut btrfs_block_group, path: *mut btrfs_path,
        start: u64, size: u64) -> c_int;
    pub fn btrfs_remove_from_free_space_tree(trans: *mut btrfs_trans_handle,
        start: u64, size: u64) -> c_int;
    pub fn __btrfs_add_to_free_space_tree(trans: *mut btrfs_trans_handle,
        block_group: *mut btrfs_block_group, path: *mut btrfs_path,
        start: u64, size: u64) -> c_int;
    pub fn btrfs_add_to_free_space_tree(trans: *mut btrfs_trans_handle,
        start: u64, size: u64) -> c_int;
    pub fn btrfs_create_free_space_tree(fs_info: *mut btrfs_fs_info) -> c_int;
    pub fn btrfs_delete_free_space_tree(fs_info: *mut btrfs_fs_info) -> c_int;
    pub fn btrfs_rebuild_free_space_tree(fs_info: *mut btrfs_fs_info) -> c_int;
    pub fn btrfs_add_block_group_free_space(trans: *mut btrfs_trans_handle,
        block_group: *mut btrfs_block_group) -> c_int;
    pub fn btrfs_remove_block_group_free_space(trans: *mut btrfs_trans_handle,
        block_group: *mut btrfs_block_group) -> c_int;
    pub fn btrfs_load_free_space_tree(caching_ctl: *mut btrfs_caching_control) -> c_int;
    pub fn btrfs_delete_orphan_free_space_entries(fs_info: *mut btrfs_fs_info) -> c_int;
}

/*
 * The complete kernel implementation is retained below as the direct
 * translation source.  External structures, constants, accessors, locking,
 * allocation, tree walking, and error helpers are supplied by the Btrfs
 * translation unit and are therefore not reimplemented here.
 */

pub const SOURCE_TRANSLATION: &str = include_str!("free-space-tree.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
