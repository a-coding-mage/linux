/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies supplied by the surrounding translation unit.

pub const BTRFS_DEFAULT_RECLAIM_THRESH: u32 = 75;

#[repr(C)]
pub struct btrfs_zoned_device_info {
    pub zone_size: u64,
    pub zone_size_shift: u8,
    pub nr_zones: u32,
    pub max_active_zones: ::core::ffi::c_uint,
    pub reserved_active_zones: ::core::ffi::c_int,
    pub active_zones_left: atomic_t,
    pub seq_zones: *mut ::core::ffi::c_ulong,
    pub empty_zones: *mut ::core::ffi::c_ulong,
    pub active_zones: *mut ::core::ffi::c_ulong,
    pub zone_cache: *mut blk_zone,
    pub sb_zones: [blk_zone; 2 * BTRFS_SUPER_MIRROR_MAX as usize],
}

pub fn btrfs_finish_ordered_zoned(ordered: *mut btrfs_ordered_extent);

// CONFIG_BLK_DEV_ZONED declarations.
#[cfg(feature = "CONFIG_BLK_DEV_ZONED")]
extern "C" {
    pub fn btrfs_get_dev_zone_info_all_devices(fs_info: *mut btrfs_fs_info) -> ::core::ffi::c_int;
    pub fn btrfs_get_dev_zone_info(device: *mut btrfs_device, populate_cache: bool) -> ::core::ffi::c_int;
    pub fn btrfs_destroy_dev_zone_info(device: *mut btrfs_device);
    pub fn btrfs_clone_dev_zone_info(orig_dev: *mut btrfs_device) -> *mut btrfs_zoned_device_info;
    pub fn btrfs_check_zoned_mode(fs_info: *mut btrfs_fs_info) -> ::core::ffi::c_int;
    pub fn btrfs_check_mountopts_zoned(info: *const btrfs_fs_info, mount_opt: *mut u64) -> ::core::ffi::c_int;
    pub fn btrfs_sb_log_location_bdev(bdev: *mut block_device, mirror: ::core::ffi::c_int, rw: ::core::ffi::c_int, bytenr_ret: *mut u64) -> ::core::ffi::c_int;
    pub fn btrfs_sb_log_location(device: *mut btrfs_device, mirror: ::core::ffi::c_int, rw: ::core::ffi::c_int, bytenr_ret: *mut u64) -> ::core::ffi::c_int;
    pub fn btrfs_advance_sb_log(device: *mut btrfs_device, mirror: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn btrfs_reset_sb_log_zones(bdev: *mut block_device, mirror: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn btrfs_find_allocatable_zones(device: *mut btrfs_device, hole_start: u64, hole_end: u64, num_bytes: u64) -> u64;
    pub fn btrfs_reset_device_zone(device: *mut btrfs_device, physical: u64, length: u64, bytes: *mut u64) -> ::core::ffi::c_int;
    pub fn btrfs_ensure_empty_zones(device: *mut btrfs_device, start: u64, size: u64) -> ::core::ffi::c_int;
    pub fn btrfs_load_block_group_zone_info(cache: *mut btrfs_block_group, new_: bool) -> ::core::ffi::c_int;
    pub fn btrfs_calc_zone_unusable(cache: *mut btrfs_block_group);
    pub fn btrfs_use_zone_append(bbio: *mut btrfs_bio) -> bool;
    pub fn btrfs_record_physical_zoned(bbio: *mut btrfs_bio);
    pub fn btrfs_check_meta_write_pointer(fs_info: *mut btrfs_fs_info, ctx: *mut btrfs_eb_write_context) -> ::core::ffi::c_int;
    pub fn btrfs_zoned_issue_zeroout(device: *mut btrfs_device, physical: u64, length: u64) -> ::core::ffi::c_int;
    pub fn btrfs_sync_zone_write_pointer(tgt_dev: *mut btrfs_device, logical: u64, physical_start: u64, physical_pos: u64) -> ::core::ffi::c_int;
    pub fn btrfs_zone_activate(block_group: *mut btrfs_block_group) -> bool;
    pub fn btrfs_zone_finish(block_group: *mut btrfs_block_group) -> ::core::ffi::c_int;
    pub fn btrfs_can_activate_zone(fs_devices: *mut btrfs_fs_devices, flags: u64) -> bool;
    pub fn btrfs_zone_finish_endio(fs_info: *mut btrfs_fs_info, logical: u64, length: u64) -> ::core::ffi::c_int;
    pub fn btrfs_schedule_zone_finish_bg(bg: *mut btrfs_block_group, eb: *mut extent_buffer);
    pub fn btrfs_clear_data_reloc_bg(bg: *mut btrfs_block_group);
    pub fn btrfs_zoned_reserve_data_reloc_bg(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_free_zone_cache(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_zoned_should_reclaim(fs_info: *const btrfs_fs_info) -> bool;
    pub fn btrfs_zoned_release_data_reloc_bg(fs_info: *mut btrfs_fs_info, logical: u64, length: u64);
    pub fn btrfs_zone_finish_one_bg(fs_info: *mut btrfs_fs_info) -> ::core::ffi::c_int;
    pub fn btrfs_zoned_activate_one_bg(space_info: *mut btrfs_space_info, do_finish: bool) -> ::core::ffi::c_int;
    pub fn btrfs_check_active_zone_reservation(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_reset_unused_block_groups(space_info: *mut btrfs_space_info, num_bytes: u64) -> ::core::ffi::c_int;
    pub fn btrfs_show_zoned_stats(fs_info: *mut btrfs_fs_info, seq: *mut seq_file);
}

// The non-zoned configuration supplies the inline stubs from the C header.
// Their exact bodies depend on external kernel structures and helpers.

#[inline]
pub unsafe fn btrfs_dev_is_sequential(device: *mut btrfs_device, pos: u64) -> bool {
    let zone_info = (*device).zone_info;
    if zone_info.is_null() { return false; }
    test_bit(pos >> (*zone_info).zone_size_shift, (*zone_info).seq_zones)
}

#[inline]
pub unsafe fn btrfs_dev_is_empty_zone(device: *mut btrfs_device, pos: u64) -> bool {
    let zone_info = (*device).zone_info;
    if zone_info.is_null() { return true; }
    test_bit(pos >> (*zone_info).zone_size_shift, (*zone_info).empty_zones)
}

#[inline]
pub unsafe fn btrfs_dev_set_empty_zone_bit(device: *mut btrfs_device, pos: u64, set: bool) {
    let zone_info = (*device).zone_info;
    if zone_info.is_null() { return; }
    let zno = pos >> (*zone_info).zone_size_shift;
    if set { set_bit(zno, (*zone_info).empty_zones); } else { clear_bit(zno, (*zone_info).empty_zones); }
}

#[inline] pub unsafe fn btrfs_dev_set_zone_empty(device: *mut btrfs_device, pos: u64) { btrfs_dev_set_empty_zone_bit(device, pos, true); }
#[inline] pub unsafe fn btrfs_dev_clear_zone_empty(device: *mut btrfs_device, pos: u64) { btrfs_dev_set_empty_zone_bit(device, pos, false); }

#[inline]
pub unsafe fn btrfs_can_zone_reset(device: *mut btrfs_device, physical: u64, length: u64) -> bool {
    if !btrfs_dev_is_sequential(device, physical) { return false; }
    let zone_size = (*(*device).zone_info).zone_size;
    if !is_aligned(physical, zone_size) || !is_aligned(length, zone_size) { return false; }
    true
}

// C inline helpers involving filesystem locks and fields are retained as declarations.
extern "C" {
    pub fn btrfs_zoned_meta_io_lock(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_zoned_meta_io_unlock(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_clear_treelog_bg(bg: *mut btrfs_block_group);
    pub fn btrfs_zoned_data_reloc_lock(inode: *mut btrfs_inode);
    pub fn btrfs_zoned_data_reloc_unlock(inode: *mut btrfs_inode);
    pub fn btrfs_zoned_bg_is_full(bg: *const btrfs_block_group) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
