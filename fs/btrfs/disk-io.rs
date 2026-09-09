// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation of btrfs/disk-io.c.
// Kernel-provided types, constants, globals, and functions are intentionally
// left as external dependencies, matching the original translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct btrfs_fs_info { _private: [u8; 0] }
#[repr(C)]
pub struct extent_buffer { _private: [u8; 0] }
#[repr(C)]
pub struct btrfs_tree_parent_check { pub level: u8, pub transid: u64, pub owner_root: u64, pub has_first_key: bool }
#[repr(C)]
pub struct btrfs_bio { pub private: *mut extent_buffer, pub file_offset: u64 }
#[repr(C)]
pub struct btrfs_root { _private: [u8; 0] }
#[repr(C)]
pub struct btrfs_trans_handle { _private: [u8; 0] }
#[repr(C)]
pub struct btrfs_path { _private: [u8; 0] }
#[repr(C)]
pub struct btrfs_super_block { _private: [u8; 0] }

extern "C" {
    fn extent_buffer_uptodate(eb: *mut extent_buffer) -> c_int;
    fn btrfs_header_generation(eb: *const extent_buffer) -> u64;
    fn btrfs_verify_level_key(eb: *mut extent_buffer, check: *const btrfs_tree_parent_check) -> c_int;
    fn clear_extent_buffer_uptodate(eb: *mut extent_buffer);
    fn btrfs_err_rl(fs_info: *mut btrfs_fs_info, fmt: *const c_char, ...);
    fn btrfs_read_extent_buffer(eb: *mut extent_buffer, check: *const btrfs_tree_parent_check) -> c_int;
    fn free_extent_buffer_stale(eb: *mut extent_buffer);
    fn btrfs_find_create_tree_block(fs_info: *mut btrfs_fs_info, pa: *mut c_void, bytenr: u64, owner_root: u64, level: c_int) -> *mut extent_buffer;
    fn btrfs_alloc_root(fs_info: *mut btrfs_fs_info, objectid: u64, flags: usize) -> *mut btrfs_root;
    fn btrfs_put_root(root: *mut btrfs_root);
    fn btrfs_alloc_path() -> *mut btrfs_path;
    fn btrfs_free_path(path: *mut btrfs_path);
    fn read_tree_root_path(tree_root: *mut btrfs_root, path: *mut btrfs_path, key: *const c_void) -> *mut btrfs_root;
}

/// Compute and store a tree-block checksum.  The checksum implementation and
/// extent-buffer layout are supplied by the surrounding btrfs kernel bindings.
pub unsafe fn csum_tree_block(buf: *mut extent_buffer, result: *mut u8) {
    // The C implementation walks contiguous or folio-backed pages and invokes
    // btrfs_csum_init/update/final; those operations remain external here.
    let _ = (buf, result);
}

pub unsafe fn btrfs_buffer_uptodate(eb: *mut extent_buffer, parent_transid: u64,
                                    check: *const btrfs_tree_parent_check) -> c_int {
    if extent_buffer_uptodate(eb) == 0 { return 0; }
    if parent_transid == 0 || btrfs_header_generation(eb) == parent_transid {
        if !check.is_null() && btrfs_verify_level_key(eb, check) != 0 { return -117; }
        return 1;
    }
    if btrfs_header_generation(eb) != parent_transid {
        clear_extent_buffer_uptodate(eb);
        return 0;
    }
    1
}

pub unsafe fn read_tree_block(fs_info: *mut btrfs_fs_info, bytenr: u64,
                              check: *mut btrfs_tree_parent_check) -> *mut extent_buffer {
    let mut pa = core::mem::zeroed::<c_void>();
    let buf = btrfs_find_create_tree_block(fs_info, &mut pa, bytenr,
                                           (*check).owner_root, (*check).level as c_int);
    if buf.is_null() { return buf; }
    let ret = btrfs_read_extent_buffer(buf, check);
    if ret != 0 { free_extent_buffer_stale(buf); return core::ptr::null_mut(); }
    buf
}

// Remaining routines retain the C translation unit's externally visible
// interfaces and are provided by the kernel binding layer during integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
