/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2009 Oracle.  All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external: linux/rbtree.h, linux/list.h, linux/spinlock.h,
// linux/mutex.h, linux/freezer.h, and fs.h.

use core::ffi::{c_ulong, c_void};

pub enum inode {}
pub enum page {}
pub enum btrfs_fs_info {}
pub enum btrfs_path {}
pub enum btrfs_trans_handle {}
pub enum btrfs_trim_block_group {}
pub enum btrfs_block_group {}
pub enum btrfs_free_cluster {}

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct rb_root {
    _private: [u8; 0],
}
#[repr(C)]
pub struct rb_root_cached {
    _private: [u8; 0],
}
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}
pub type spinlock_t = c_void;
pub type mutex = c_void;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum btrfs_trim_state {
    BTRFS_TRIM_STATE_UNTRIMMED,
    BTRFS_TRIM_STATE_TRIMMED,
    BTRFS_TRIM_STATE_TRIMMING,
}

#[repr(C)]
pub struct btrfs_free_space {
    pub offset_index: rb_node,
    pub bytes_index: rb_node,
    pub offset: u64,
    pub bytes: u64,
    pub max_extent_size: u64,
    pub bitmap: *mut c_ulong,
    pub list: list_head,
    pub trim_state: btrfs_trim_state,
    pub bitmap_extents: i32,
}

#[inline]
pub unsafe fn btrfs_free_space_trimmed(info: *mut btrfs_free_space) -> bool {
    (*info).trim_state == btrfs_trim_state::BTRFS_TRIM_STATE_TRIMMED
}

#[inline]
pub unsafe fn btrfs_free_space_trimming_bitmap(info: *mut btrfs_free_space) -> bool {
    (*info).trim_state == btrfs_trim_state::BTRFS_TRIM_STATE_TRIMMING
}

#[inline]
pub unsafe fn btrfs_trim_interrupted() -> bool {
    fatal_signal_pending(current()) || freezing(current())
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum btrfs_stat {
    BTRFS_STAT_CURR,
    BTRFS_STAT_PREV,
    BTRFS_STAT_NR_ENTRIES,
}

#[repr(C)]
pub struct btrfs_free_space_ctl {
    pub free_space_offset: rb_root,
    pub free_space_bytes: rb_root_cached,
    pub tree_lock: spinlock_t,
    pub extents_thresh: i32,
    pub free_extents: i32,
    pub total_bitmaps: i32,
    pub free_space: u64,
    pub discardable_extents: [i32; 2],
    pub discardable_bytes: [i64; 2],
    pub block_group: *mut btrfs_block_group,
    pub cache_writeout_mutex: mutex,
    pub trimming_ranges: list_head,
}

#[repr(C)]
pub struct btrfs_io_ctl {
    pub cur: *mut c_void,
    pub orig: *mut c_void,
    pub page: *mut page,
    pub pages: *mut *mut page,
    pub fs_info: *mut btrfs_fs_info,
    pub inode: *mut inode,
    pub size: c_ulong,
    pub index: i32,
    pub num_pages: i32,
    pub entries: i32,
    pub bitmaps: i32,
}

extern "C" {
    pub fn btrfs_free_space_init() -> i32;
    pub fn btrfs_free_space_exit();
    pub fn lookup_free_space_inode(block_group: *mut btrfs_block_group, path: *mut btrfs_path) -> *mut inode;
    pub fn create_free_space_inode(trans: *mut btrfs_trans_handle, block_group: *mut btrfs_block_group, path: *mut btrfs_path) -> i32;
    pub fn btrfs_remove_free_space_inode(trans: *mut btrfs_trans_handle, inode: *mut inode, block_group: *mut btrfs_block_group) -> i32;
    pub fn btrfs_truncate_free_space_cache(trans: *mut btrfs_trans_handle, block_group: *mut btrfs_block_group, inode: *mut inode) -> i32;
    pub fn load_free_space_cache(block_group: *mut btrfs_block_group) -> i32;
    pub fn btrfs_wait_cache_io(trans: *mut btrfs_trans_handle, block_group: *mut btrfs_block_group, path: *mut btrfs_path) -> i32;
    pub fn btrfs_write_out_cache(trans: *mut btrfs_trans_handle, block_group: *mut btrfs_block_group, path: *mut btrfs_path) -> i32;
    pub fn btrfs_init_free_space_ctl(block_group: *mut btrfs_block_group, ctl: *mut btrfs_free_space_ctl);
    pub fn btrfs_add_free_space(block_group: *mut btrfs_block_group, bytenr: u64, size: u64) -> i32;
    pub fn btrfs_add_free_space_unused(block_group: *mut btrfs_block_group, bytenr: u64, size: u64) -> i32;
    pub fn btrfs_add_free_space_async_trimmed(block_group: *mut btrfs_block_group, bytenr: u64, size: u64) -> i32;
    pub fn btrfs_remove_free_space(block_group: *mut btrfs_block_group, bytenr: u64, size: u64) -> i32;
    pub fn btrfs_remove_free_space_cache(block_group: *mut btrfs_block_group);
    pub fn btrfs_is_free_space_trimmed(block_group: *mut btrfs_block_group) -> bool;
    pub fn btrfs_find_space_for_alloc(block_group: *mut btrfs_block_group, offset: u64, bytes: u64, empty_size: u64, max_extent_size: *mut u64) -> u64;
    pub fn btrfs_dump_free_space(block_group: *mut btrfs_block_group, bytes: u64);
    pub fn btrfs_find_space_cluster(block_group: *mut btrfs_block_group, cluster: *mut btrfs_free_cluster, offset: u64, bytes: u64, empty_size: u64) -> i32;
    pub fn btrfs_init_free_cluster(cluster: *mut btrfs_free_cluster);
    pub fn btrfs_alloc_from_cluster(block_group: *mut btrfs_block_group, cluster: *mut btrfs_free_cluster, bytes: u64, min_start: u64, max_extent_size: *mut u64) -> u64;
    pub fn btrfs_return_cluster_to_free_space(block_group: *mut btrfs_block_group, cluster: *mut btrfs_free_cluster);
    pub fn btrfs_trim_block_group(block_group: *mut btrfs_block_group, trimmed: *mut u64, start: u64, end: u64, minlen: u64) -> i32;
    pub fn btrfs_trim_block_group_extents(block_group: *mut btrfs_block_group, trimmed: *mut u64, start: u64, end: u64, minlen: u64, async_: bool) -> i32;
    pub fn btrfs_trim_block_group_bitmaps(block_group: *mut btrfs_block_group, trimmed: *mut u64, start: u64, end: u64, minlen: u64, maxlen: u64, async_: bool) -> i32;
    pub fn btrfs_trim_fully_remapped_block_group(bg: *mut btrfs_block_group);
    pub fn btrfs_free_space_cache_v1_active(fs_info: *mut btrfs_fs_info) -> bool;
    pub fn btrfs_set_free_space_cache_v1_active(fs_info: *mut btrfs_fs_info, active: bool) -> i32;
    pub fn btrfs_use_bitmap(ctl: *mut btrfs_free_space_ctl, info: *mut btrfs_free_space) -> bool;
    pub fn test_add_free_space_entry(cache: *mut btrfs_block_group, offset: u64, bytes: u64, bitmap: bool) -> i32;
    pub fn test_check_exists(cache: *mut btrfs_block_group, offset: u64, bytes: u64) -> i32;
}

extern "C" {
    fn fatal_signal_pending(task: *mut c_void) -> bool;
    fn freezing(task: *mut c_void) -> bool;
    fn current() -> *mut c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
