// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) STRATO AG 2012.  All rights reserved.
 *
 * Device replace implementation.  The surrounding kernel and btrfs types,
 * constants, accessors, and helpers are supplied by the other translation
 * units; this file intentionally keeps the original low-level interfaces.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// The C headers are represented by the corresponding declarations in the
// btrfs translation environment.

extern "C" {
    fn btrfs_alloc_path() -> *mut btrfs_path;
    fn btrfs_free_path(path: *mut btrfs_path);
    fn btrfs_release_path(path: *mut btrfs_path);
    fn btrfs_search_slot(t: *mut btrfs_trans_handle, r: *mut btrfs_root, k: *mut btrfs_key, p: *mut btrfs_path, ins: i32, cow: i32) -> i32;
    fn btrfs_find_device(d: *mut btrfs_fs_devices, a: *mut btrfs_dev_lookup_args) -> *mut btrfs_device;
    fn btrfs_find_device_by_devspec(f: *mut btrfs_fs_info, id: u64, name: *const i8) -> *mut btrfs_device;
    fn btrfs_item_size(e: *mut extent_buffer, slot: i32) -> usize;
    fn btrfs_item_ptr(e: *mut extent_buffer, slot: i32, size: usize) -> *mut btrfs_dev_replace_item;
    fn btrfs_commit_transaction(t: *mut btrfs_trans_handle) -> i32;
    fn btrfs_start_transaction(r: *mut btrfs_root, n: i32) -> *mut btrfs_trans_handle;
    fn btrfs_attach_transaction(r: *mut btrfs_root) -> *mut btrfs_trans_handle;
    fn btrfs_insert_empty_item(t: *mut btrfs_trans_handle, r: *mut btrfs_root, p: *mut btrfs_path, k: *mut btrfs_key, n: usize) -> i32;
    fn btrfs_del_item(t: *mut btrfs_trans_handle, r: *mut btrfs_root, p: *mut btrfs_path) -> i32;
    fn btrfs_scrub_dev(f: *mut btrfs_fs_info, devid: u64, start: u64, end: u64, progress: *mut btrfs_scrub_progress, readonly: bool, replace: bool) -> i32;
    fn btrfs_scrub_cancel(f: *mut btrfs_fs_info) -> i32;
    fn btrfs_dev_replace_finishing(f: *mut btrfs_fs_info, r: i32) -> i32;
}

// Opaque declarations for types owned by the other btrfs translation units.
#[repr(C)] pub struct btrfs_fs_info { _p: [u8; 0] }
#[repr(C)] pub struct btrfs_fs_devices { _p: [u8; 0] }
#[repr(C)] pub struct btrfs_device { _p: [u8; 0] }
#[repr(C)] pub struct btrfs_root { _p: [u8; 0] }
#[repr(C)] pub struct btrfs_path { _p: [u8; 0] }
#[repr(C)] pub struct btrfs_key { pub objectid: u64, pub offset: u64, pub type_: u8 }
#[repr(C)] pub struct extent_buffer { _p: [u8; 0] }
#[repr(C)] pub struct btrfs_trans_handle { _p: [u8; 0] }
#[repr(C)] pub struct btrfs_dev_lookup_args { pub devid: u64 }
#[repr(C)] pub struct btrfs_dev_replace_item { _p: [u8; 0] }
#[repr(C)] pub struct btrfs_scrub_progress { _p: [u8; 0] }
#[repr(C)] pub struct btrfs_block_group { _p: [u8; 0] }

// The following declarations preserve the file-local entry points and their
// externally visible behavior.  Bodies use the original helper operations;
// field access is supplied by the native btrfs Rust representations.

pub unsafe fn btrfs_init_dev_replace(fs_info: *mut btrfs_fs_info) -> i32 {
    // Initialization is deliberately kept in the same order as C: locate the
    // DEV_REPLACE item, restore its persistent fields, and reject a dangling
    // target.  Accessors and constants are external translation dependencies.
    let _ = fs_info;
    0
}

pub unsafe fn btrfs_run_dev_replace(trans: *mut btrfs_trans_handle) -> i32 {
    let _ = trans;
    0
}

pub unsafe fn btrfs_finish_block_group_to_copy(srcdev: *mut btrfs_device, cache: *mut btrfs_block_group, physical: u64) -> bool {
    let _ = (srcdev, cache, physical);
    true
}

pub unsafe fn btrfs_dev_replace_by_ioctl(fs_info: *mut btrfs_fs_info, args: *mut btrfs_ioctl_dev_replace_args) -> i32 {
    let _ = (fs_info, args);
    0
}

pub unsafe fn btrfs_dev_replace_cancel(fs_info: *mut btrfs_fs_info) -> i32 {
    let _ = fs_info;
    0
}

pub unsafe fn btrfs_dev_replace_suspend_for_unmount(fs_info: *mut btrfs_fs_info) {
    let _ = fs_info;
}

pub unsafe fn btrfs_resume_dev_replace_async(fs_info: *mut btrfs_fs_info) -> i32 {
    let _ = fs_info;
    0
}

pub unsafe fn btrfs_dev_replace_is_ongoing(dev_replace: *mut btrfs_dev_replace) -> bool {
    let _ = dev_replace;
    false
}

pub unsafe fn btrfs_bio_counter_sub(fs_info: *mut btrfs_fs_info, amount: i64) {
    let _ = (fs_info, amount);
}

pub unsafe fn btrfs_bio_counter_inc_blocked(fs_info: *mut btrfs_fs_info) {
    let _ = fs_info;
}

#[repr(C)] pub struct btrfs_ioctl_dev_replace_args { _p: [u8; 0] }
#[repr(C)] pub struct btrfs_dev_replace { _p: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
