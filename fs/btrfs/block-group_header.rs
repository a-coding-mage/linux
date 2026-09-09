/* SPDX-License-Identifier: GPL-2.0 */

// Kernel includes and build-time configuration are supplied by the surrounding
// translation unit.

use core::ffi::c_int;

pub type u64_t = u64;
pub type u32_t = u32;

pub enum btrfs_chunk_map {}
pub enum btrfs_fs_info {}
pub enum btrfs_inode {}
pub enum btrfs_trans_handle {}
pub enum btrfs_space_info {}
pub enum btrfs_work {}
pub enum btrfs_io_ctl {}
pub enum extent_buffer {}
pub enum list_head {}
pub enum mutex {}
pub enum wait_queue_head_t {}
pub enum spinlock_t {}
pub enum rw_semaphore {}
pub enum rb_node {}
pub enum refcount_t {}
pub enum atomic_t {}
pub enum work_struct {}
pub enum btrfs_free_space_ctl {}

pub const CACHING_CTL_WAKE_UP: u64 = 2 * 1024 * 1024;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_disk_cache_state {
    BTRFS_DC_WRITTEN,
    BTRFS_DC_ERROR,
    BTRFS_DC_CLEAR,
    BTRFS_DC_SETUP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_block_group_size_class {
    BTRFS_BG_SZ_NONE,
    BTRFS_BG_SZ_SMALL,
    BTRFS_BG_SZ_MEDIUM,
    BTRFS_BG_SZ_LARGE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_discard_state {
    BTRFS_DISCARD_EXTENTS,
    BTRFS_DISCARD_BITMAPS,
    BTRFS_DISCARD_RESET_CURSOR,
    BTRFS_DISCARD_FULLY_REMAPPED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_chunk_alloc_enum {
    CHUNK_ALLOC_NO_FORCE,
    CHUNK_ALLOC_LIMITED,
    CHUNK_ALLOC_FORCE,
    CHUNK_ALLOC_FORCE_FOR_EXTENT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_block_group_flags {
    BLOCK_GROUP_FLAG_IREF,
    BLOCK_GROUP_FLAG_REMOVED,
    BLOCK_GROUP_FLAG_TO_COPY,
    BLOCK_GROUP_FLAG_RELOCATING_REPAIR,
    BLOCK_GROUP_FLAG_CHUNK_ITEM_INSERTED,
    BLOCK_GROUP_FLAG_ZONE_IS_ACTIVE,
    BLOCK_GROUP_FLAG_ZONED_DATA_RELOC,
    BLOCK_GROUP_FLAG_NEEDS_FREE_SPACE,
    BLOCK_GROUP_FLAG_FREE_SPACE_ADDED,
    BLOCK_GROUP_FLAG_SEQUENTIAL_ZONE,
    BLOCK_GROUP_FLAG_NEW,
    BLOCK_GROUP_FLAG_FULLY_REMAPPED,
    BLOCK_GROUP_FLAG_STRIPE_REMOVAL_PENDING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_caching_type {
    BTRFS_CACHE_NO,
    BTRFS_CACHE_STARTED,
    BTRFS_CACHE_FINISHED,
    BTRFS_CACHE_ERROR,
}

#[repr(C)]
pub struct btrfs_caching_control {
    pub list: *mut list_head,
    pub mutex: *mut mutex,
    pub wait: *mut wait_queue_head_t,
    pub work: *mut btrfs_work,
    pub block_group: *mut btrfs_block_group,
    pub progress: *mut atomic_t,
    pub count: *mut refcount_t,
}

#[repr(C)]
pub struct btrfs_block_group {
    pub fs_info: *mut btrfs_fs_info,
    pub inode: *mut btrfs_inode,
    pub lock: *mut spinlock_t,
    pub ro: u32,
    pub start: u64,
    pub length: u64,
    pub pinned: u64,
    pub reserved: u64,
    pub used: u64,
    pub delalloc_bytes: u64,
    pub bytes_super: u64,
    pub flags: u64,
    pub cache_generation: u64,
    pub global_root_id: u64,
    pub remap_bytes: u64,
    pub identity_remap_count: u32,
    pub last_identity_remap_count: u32,
    pub last_used: u64,
    pub last_remap_bytes: u64,
    pub last_flags: u64,
    pub bitmap_high_thresh: u32,
    pub bitmap_low_thresh: u32,
    pub data_rwsem: *mut rw_semaphore,
    pub full_stripe_len: usize,
    pub runtime_flags: usize,
    pub disk_cache_state: btrfs_disk_cache_state,
    pub cached: btrfs_caching_type,
    pub caching_ctl: *mut btrfs_caching_control,
    pub space_info: *mut btrfs_space_info,
    pub free_space_ctl: *mut btrfs_free_space_ctl,
    pub cache_node: *mut rb_node,
    pub list: *mut list_head,
    pub refs: *mut refcount_t,
    pub frozen: *mut atomic_t,
    pub cluster_list: *mut list_head,
    pub bg_list: *mut list_head,
    pub ro_list: *mut list_head,
    pub discard_list: *mut list_head,
    pub discard_index: c_int,
    pub discard_state: btrfs_discard_state,
    pub discard_eligible_time: u64,
    pub discard_cursor: u64,
    pub dirty_list: *mut list_head,
    pub io_list: *mut list_head,
    pub io_ctl: btrfs_io_ctl,
    pub reservations: *mut atomic_t,
    pub nocow_writers: *mut atomic_t,
    pub free_space_lock: *mut mutex,
    pub using_free_space_bitmaps: bool,
    pub using_free_space_bitmaps_cached: bool,
    pub size_class: btrfs_block_group_size_class,
    pub reclaim_mark: bool,
    pub swap_extents: c_int,
    pub alloc_offset: u64,
    pub zone_unusable: u64,
    pub zone_capacity: u64,
    pub meta_write_pointer: u64,
    pub physical_map: *mut btrfs_chunk_map,
    pub active_bg_list: *mut list_head,
    pub zone_finish_work: *mut work_struct,
    pub last_eb: *mut extent_buffer,
}

#[inline]
pub unsafe fn btrfs_block_group_end(block_group: *const btrfs_block_group) -> u64 {
    (*block_group).start.wrapping_add((*block_group).length)
}

#[inline]
pub unsafe fn btrfs_is_block_group_used(bg: *const btrfs_block_group) -> bool {
    (*bg).used > 0 || (*bg).reserved > 0 || (*bg).pinned > 0 || (*bg).remap_bytes > 0
}

#[inline]
pub unsafe fn btrfs_is_block_group_data_only(block_group: *const btrfs_block_group) -> bool {
    ((*block_group).flags & BTRFS_BLOCK_GROUP_DATA) != 0
        && ((*block_group).flags & BTRFS_BLOCK_GROUP_METADATA) == 0
}

#[inline]
pub unsafe fn btrfs_block_group_available_space(bg: *const btrfs_block_group) -> u64 {
    (*bg).length
        .wrapping_sub((*bg).used)
        .wrapping_sub((*bg).pinned)
        .wrapping_sub((*bg).reserved)
        .wrapping_sub((*bg).bytes_super)
        .wrapping_sub((*bg).zone_unusable)
}

extern "C" {
    pub fn btrfs_init_block_group() -> c_int;
    pub fn btrfs_exit_block_group();
    pub fn btrfs_lookup_first_block_group(info: *mut btrfs_fs_info, bytenr: u64) -> *mut btrfs_block_group;
    pub fn btrfs_lookup_block_group(info: *mut btrfs_fs_info, bytenr: u64) -> *mut btrfs_block_group;
    pub fn btrfs_next_block_group(cache: *mut btrfs_block_group) -> *mut btrfs_block_group;
    pub fn btrfs_get_block_group(cache: *mut btrfs_block_group);
    pub fn btrfs_put_block_group(cache: *mut btrfs_block_group);
    pub fn btrfs_dec_block_group_reservations(fs_info: *mut btrfs_fs_info, start: u64);
    pub fn btrfs_wait_block_group_reservations(bg: *mut btrfs_block_group);
    pub fn btrfs_inc_nocow_writers(fs_info: *mut btrfs_fs_info, bytenr: u64) -> *mut btrfs_block_group;
    pub fn btrfs_dec_nocow_writers(bg: *mut btrfs_block_group);
    pub fn btrfs_wait_nocow_writers(bg: *mut btrfs_block_group);
    pub fn btrfs_wait_block_group_cache_progress(cache: *mut btrfs_block_group, num_bytes: u64);
    pub fn btrfs_cache_block_group(cache: *mut btrfs_block_group, wait: bool) -> c_int;
    pub fn btrfs_get_caching_control(cache: *mut btrfs_block_group) -> *mut btrfs_caching_control;
    pub fn btrfs_get_alloc_profile(fs_info: *mut btrfs_fs_info, orig_flags: u64) -> u64;
    pub fn btrfs_freeze_block_group(cache: *mut btrfs_block_group);
    pub fn btrfs_unfreeze_block_group(cache: *mut btrfs_block_group);
    pub fn btrfs_calc_block_group_size_class(size: u64) -> btrfs_block_group_size_class;
    pub fn btrfs_add_new_free_space(block_group: *mut btrfs_block_group, start: u64, end: u64, total_added_ret: *mut u64) -> c_int;
    pub fn btrfs_start_trans_remove_block_group(fs_info: *mut btrfs_fs_info, chunk_offset: u64) -> *mut btrfs_trans_handle;
    pub fn btrfs_remove_bg_from_sinfo(bg: *mut btrfs_block_group);
    pub fn btrfs_remove_block_group(trans: *mut btrfs_trans_handle, map: *mut btrfs_chunk_map) -> c_int;
    pub fn btrfs_delete_unused_bgs(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_mark_bg_unused(bg: *mut btrfs_block_group);
    pub fn btrfs_reclaim_block_groups(fs_info: *mut btrfs_fs_info, limit: u32);
    pub fn btrfs_reclaim_bgs_work(work: *mut work_struct);
    pub fn btrfs_reclaim_bgs(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_mark_bg_to_reclaim(bg: *mut btrfs_block_group);
    pub fn btrfs_read_block_groups(info: *mut btrfs_fs_info) -> c_int;
    pub fn btrfs_make_block_group(trans: *mut btrfs_trans_handle, space_info: *mut btrfs_space_info, ty: u64, chunk_offset: u64, size: u64) -> *mut btrfs_block_group;
    pub fn btrfs_create_pending_block_groups(trans: *mut btrfs_trans_handle);
    pub fn btrfs_inc_block_group_ro(cache: *mut btrfs_block_group, do_chunk_alloc: bool) -> c_int;
    pub fn btrfs_dec_block_group_ro(cache: *mut btrfs_block_group);
    pub fn btrfs_start_dirty_block_groups(trans: *mut btrfs_trans_handle) -> c_int;
    pub fn btrfs_write_dirty_block_groups(trans: *mut btrfs_trans_handle) -> c_int;
    pub fn btrfs_setup_space_cache(trans: *mut btrfs_trans_handle) -> c_int;
    pub fn btrfs_update_block_group(trans: *mut btrfs_trans_handle, bytenr: u64, num_bytes: u64, alloc: bool) -> c_int;
    pub fn btrfs_add_reserved_bytes(cache: *mut btrfs_block_group, ram_bytes: u64, num_bytes: u64, delalloc: bool, force_wrong_size_class: bool) -> c_int;
    pub fn btrfs_free_reserved_bytes(cache: *mut btrfs_block_group, num_bytes: u64, is_delalloc: bool);
    pub fn btrfs_chunk_alloc(trans: *mut btrfs_trans_handle, space_info: *mut btrfs_space_info, flags: u64, force: btrfs_chunk_alloc_enum) -> c_int;
    pub fn btrfs_force_chunk_alloc(trans: *mut btrfs_trans_handle, ty: u64) -> c_int;
    pub fn check_system_chunk(trans: *mut btrfs_trans_handle, ty: u64);
    pub fn btrfs_reserve_chunk_metadata(trans: *mut btrfs_trans_handle, is_item_insertion: bool);
    pub fn btrfs_put_block_group_cache(info: *mut btrfs_fs_info);
    pub fn btrfs_free_block_groups(info: *mut btrfs_fs_info) -> c_int;
    pub fn btrfs_rmap_block(fs_info: *mut btrfs_fs_info, chunk_start: u64, physical: u64, logical: *mut *mut u64, naddrs: *mut c_int, stripe_len: *mut c_int) -> c_int;
    pub fn btrfs_inc_block_group_swap_extents(bg: *mut btrfs_block_group) -> bool;
    pub fn btrfs_dec_block_group_swap_extents(bg: *mut btrfs_block_group, amount: c_int);
    pub fn btrfs_use_block_group_size_class(bg: *mut btrfs_block_group, size_class: btrfs_block_group_size_class, force_wrong_size_class: bool) -> c_int;
    pub fn btrfs_block_group_should_use_size_class(bg: *const btrfs_block_group) -> bool;
    pub fn btrfs_mark_bg_fully_remapped(bg: *mut btrfs_block_group, trans: *mut btrfs_trans_handle);
    pub fn btrfs_populate_fully_remapped_bgs_list(fs_info: *mut btrfs_fs_info) -> c_int;
}

#[inline]
pub unsafe fn btrfs_data_alloc_profile(fs_info: *mut btrfs_fs_info) -> u64 {
    btrfs_get_alloc_profile(fs_info, BTRFS_BLOCK_GROUP_DATA)
}

#[inline]
pub unsafe fn btrfs_metadata_alloc_profile(fs_info: *mut btrfs_fs_info) -> u64 {
    btrfs_get_alloc_profile(fs_info, BTRFS_BLOCK_GROUP_METADATA)
}

#[inline]
pub unsafe fn btrfs_system_alloc_profile(fs_info: *mut btrfs_fs_info) -> u64 {
    btrfs_get_alloc_profile(fs_info, BTRFS_BLOCK_GROUP_SYSTEM)
}

#[inline]
pub unsafe fn btrfs_block_group_done(cache: *const btrfs_block_group) -> bool {
    (*cache).cached == btrfs_caching_type::BTRFS_CACHE_FINISHED
        || (*cache).cached == btrfs_caching_type::BTRFS_CACHE_ERROR
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
