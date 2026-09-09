/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of volumes.h; included C dependencies are supplied elsewhere. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

pub const BTRFS_MAX_DATA_CHUNK_SIZE: u64 = 10u64 * SZ_1G;
pub const BTRFS_MAX_DISCARD_CHUNK_SIZE: u64 = SZ_1G;
pub const BTRFS_STRIPE_LEN: u64 = SZ_64K;
pub const BTRFS_STRIPE_LEN_SHIFT: u32 = 16;
pub const BTRFS_STRIPE_LEN_MASK: u64 = BTRFS_STRIPE_LEN - 1;

pub const BTRFS_DEV_STATE_WRITEABLE: u32 = 0;
pub const BTRFS_DEV_STATE_IN_FS_METADATA: u32 = 1;
pub const BTRFS_DEV_STATE_MISSING: u32 = 2;
pub const BTRFS_DEV_STATE_REPLACE_TGT: u32 = 3;
pub const BTRFS_DEV_STATE_FLUSH_SENT: u32 = 4;
pub const BTRFS_DEV_STATE_NO_READA: u32 = 5;
pub const BTRFS_DEV_STATE_FLUSH_FAILED: u32 = 6;
pub const BTRFS_DEV_STATE_ITEM_FOUND: u32 = 7;
pub const BTRFS_SUPER_PRIMARY_WRITE_ERROR: i32 = INT_MAX / 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum btrfs_raid_types {
    BTRFS_RAID_SINGLE = 0,
    BTRFS_RAID_RAID0 = 1,
    BTRFS_RAID_RAID1 = 2,
    BTRFS_RAID_DUP = 3,
    BTRFS_RAID_RAID10 = 4,
    BTRFS_RAID_RAID5 = 5,
    BTRFS_RAID_RAID6 = 6,
    BTRFS_RAID_RAID1C3 = 7,
    BTRFS_RAID_RAID1C4 = 8,
    BTRFS_NR_RAID_TYPES = 9,
}

#[repr(C)]
pub struct btrfs_device {
    pub dev_list: list_head, pub dev_alloc_list: list_head, pub post_commit_list: list_head,
    pub fs_devices: *mut btrfs_fs_devices, pub fs_info: *mut btrfs_fs_info,
    pub name: *const core::ffi::c_char, pub generation: u64,
    pub bdev_file: *mut file, pub bdev: *mut block_device,
    pub zone_info: *mut btrfs_zoned_device_info, pub dev_state: c_ulong,
    pub devid: u64, pub total_bytes: u64, pub disk_total_bytes: u64, pub bytes_used: u64,
    pub io_align: u32, pub io_width: u32, pub r#type: u64, pub sb_write_errors: atomic_t,
    pub sector_size: u32, pub uuid: [u8; BTRFS_UUID_SIZE as usize],
    pub commit_total_bytes: u64, pub commit_bytes_used: u64,
    pub flush_bio: bio, pub flush_wait: completion, pub scrub_ctx: *mut scrub_ctx,
    pub dev_stats_valid: i32, pub dev_stats_ccnt: atomic_t,
    pub dev_stat_values: [atomic_t; BTRFS_DEV_STAT_VALUES_MAX as usize], pub devt: dev_t,
    pub alloc_state: extent_io_tree, pub kobj_unregister: completion, pub devid_kobj: kobject,
    pub scrub_speed_max: u64, pub per_profile_allocated: u64,
}

#[repr(C)]
pub struct btrfs_swapfile_pin {
    pub node: rb_node, pub ptr: *mut core::ffi::c_void, pub inode: *mut inode,
    pub is_block_group: bool, pub bg_extent_count: i32,
}

#[repr(C)]
pub enum btrfs_chunk_allocation_policy { BTRFS_CHUNK_ALLOC_REGULAR, BTRFS_CHUNK_ALLOC_ZONED }
pub const BTRFS_DEFAULT_RR_MIN_CONTIG_READ: u64 = SZ_256K;
pub const BTRFS_RAID1_MAX_MIRRORS: u32 = 4;
#[repr(C)]
pub enum btrfs_read_policy { BTRFS_READ_POLICY_PID, BTRFS_READ_POLICY_RR, BTRFS_READ_POLICY_DEVID, BTRFS_NR_READ_POLICY }

#[repr(C)]
pub struct btrfs_fs_devices {
    pub fsid: [u8; BTRFS_FSID_SIZE as usize], pub metadata_uuid: [u8; BTRFS_FSID_SIZE as usize],
    pub fs_list: list_head, pub num_devices: u64, pub open_devices: u64, pub rw_devices: u64,
    pub missing_devices: u64, pub total_rw_bytes: u64, pub total_devices: u64, pub latest_generation: u64,
    pub latest_dev: *mut btrfs_device, pub device_list_mutex: mutex, pub devices: list_head,
    pub alloc_list: list_head, pub seed_list: list_head, pub opened: i32, pub holding: i32,
    pub rotating: bool, pub discardable: bool, pub seeding: bool, pub temp_fsid: bool, pub collect_fs_stats: bool,
    pub fs_info: *mut btrfs_fs_info, pub fsid_kobj: kobject, pub devices_kobj: *mut kobject,
    pub devinfo_kobj: *mut kobject, pub kobj_unregister: completion,
    pub chunk_alloc_policy: btrfs_chunk_allocation_policy, pub read_policy: btrfs_read_policy,
    pub per_profile_avail: [u64; BTRFS_NR_RAID_TYPES as usize], pub per_profile_lock: spinlock_t,
}

#[repr(C)]
pub struct btrfs_io_stripe { pub dev: *mut btrfs_device, pub physical: u64, pub rst_search_commit_root: bool, pub bioc: *mut btrfs_io_context }
#[repr(C)]
pub struct btrfs_discard_stripe { pub dev: *mut btrfs_device, pub physical: u64, pub length: u64 }
#[repr(C)]
pub struct btrfs_io_context {
    pub refs: refcount_t, pub fs_info: *mut btrfs_fs_info, pub map_type: u64, pub orig_bio: *mut bio,
    pub error: atomic_t, pub max_errors: u16, pub use_rst: bool, pub logical: u64, pub size: u64,
    pub rst_ordered_entry: list_head, pub num_stripes: u16, pub mirror_num: u16,
    pub replace_nr_stripes: u16, pub replace_stripe_src: i16, pub full_stripe_logical: u64,
    pub stripes: [btrfs_io_stripe; 0],
}
#[repr(C)] pub struct btrfs_device_info { pub dev: *mut btrfs_device, pub dev_offset: u64, pub max_avail: u64, pub total_avail: u64 }
#[repr(C)] pub struct btrfs_raid_attr { pub sub_stripes:u8, pub dev_stripes:u8, pub devs_max:u8, pub devs_min:u8, pub tolerated_failures:u8, pub devs_increment:u8, pub ncopies:u8, pub nparity:u8, pub mindev_error:u8, pub raid_name:[c_char;8], pub bg_flag:u64 }
extern "C" { pub static btrfs_raid_array: [btrfs_raid_attr; BTRFS_NR_RAID_TYPES as usize]; }
#[repr(C)] pub struct btrfs_chunk_map { pub rb_node: rb_node, pub verified_stripes:i32, pub refs:refcount_t, pub start:u64, pub chunk_len:u64, pub stripe_size:u64, pub r#type:u64, pub on_disk_type:u64, pub num_stripes:i32, pub sub_stripes:i32, pub stripes:[btrfs_io_stripe;0] }
#[repr(C)] pub struct btrfs_balance_control { pub data:btrfs_balance_args, pub meta:btrfs_balance_args, pub sys:btrfs_balance_args, pub flags:u64, pub stat:btrfs_balance_progress }
#[repr(C)] pub struct btrfs_dev_lookup_args { pub devid:u64, pub uuid:*mut u8, pub fsid:*mut u8, pub devt:dev_t, pub missing:bool }
#[repr(C)] pub enum btrfs_map_op { BTRFS_MAP_READ, BTRFS_MAP_WRITE, BTRFS_MAP_GET_READ_MIRRORS }

extern "C" {
    pub static mut uuid_mutex: mutex;
    pub fn btrfs_get_bioc(bioc:*mut btrfs_io_context); pub fn btrfs_put_bioc(bioc:*mut btrfs_io_context);
    pub fn btrfs_map_block(fs_info:*mut btrfs_fs_info, op:btrfs_map_op, logical:u64, length:*mut u64, bioc_ret:*mut *mut btrfs_io_context, smap:*mut btrfs_io_stripe, mirror_num_ret:*mut i32)->i32;
    pub fn btrfs_map_repair_block(fs_info:*mut btrfs_fs_info,smap:*mut btrfs_io_stripe,logical:u64,length:u32,mirror_num:i32)->i32;
    pub fn btrfs_map_discard(fs_info:*mut btrfs_fs_info,logical:u64,length_ret:*mut u64,num_stripes:*mut u32,do_remap:bool)->*mut btrfs_discard_stripe;
    pub fn btrfs_read_sys_array(fs_info:*mut btrfs_fs_info)->i32; pub fn btrfs_read_chunk_tree(fs_info:*mut btrfs_fs_info)->i32;
    pub fn btrfs_mapping_tree_free(fs_info:*mut btrfs_fs_info); pub fn btrfs_close_devices(fs_devices:*mut btrfs_fs_devices);
    pub fn btrfs_num_copies(fs_info:*mut btrfs_fs_info,logical:u64,len:u64)->i32; pub fn btrfs_shrink_device(device:*mut btrfs_device,new_size:u64)->i32;
    pub fn btrfs_chunk_map_size(n: i32) -> usize;
}

#[inline] pub unsafe fn btrfs_stripe_nr_to_offset(stripe_nr:u32)->u64 { (stripe_nr as u64) << BTRFS_STRIPE_LEN_SHIFT }
#[inline] pub unsafe fn btrfs_chunk_item_size(num_stripes:i32)->usize { core::mem::size_of::<btrfs_chunk>() + core::mem::size_of::<btrfs_stripe>() * (num_stripes as usize - 1) }

/* The remaining declarations retain the header's external interfaces. */
extern "C" {
    pub fn btrfs_init_new_device(fs_info:*mut btrfs_fs_info,path:*const c_char)->i32;
    pub fn btrfs_cancel_balance(fs_info:*mut btrfs_fs_info)->i32;
    pub fn btrfs_chunk_writeable(fs_info:*mut btrfs_fs_info,chunk_offset:u64)->bool;
    pub fn btrfs_update_per_profile_avail(fs_info:*mut btrfs_fs_info);
    pub fn btrfs_repair_one_zone(fs_info:*mut btrfs_fs_info,logical:u64)->bool;
    pub fn btrfs_pinned_by_swapfile(fs_info:*mut btrfs_fs_info,ptr:*mut core::ffi::c_void)->bool;
    pub fn btrfs_create_chunk(trans:*mut btrfs_trans_handle,space_info:*mut btrfs_space_info,r#type:u64)->*mut btrfs_block_group;
    pub fn btrfs_open_devices(fs_devices:*mut btrfs_fs_devices,flags:blk_mode_t,holder:*mut core::ffi::c_void)->i32;
    pub fn btrfs_scan_one_device(path:*const c_char,mount_arg_dev:bool)->*mut btrfs_device;
    pub fn btrfs_forget_devices(devt:dev_t)->i32; pub fn btrfs_release_device_allow_freeze(bdev_file:*mut file);
    pub fn btrfs_free_extra_devids(fs_devices:*mut btrfs_fs_devices);
    pub fn btrfs_assign_next_active_device(device:*mut btrfs_device,this_dev:*mut btrfs_device);
    pub fn btrfs_find_device_by_devspec(fs_info:*mut btrfs_fs_info,devid:u64,devpath:*const c_char)->*mut btrfs_device;
    pub fn btrfs_get_dev_args_from_path(fs_info:*mut btrfs_fs_info,args:*mut btrfs_dev_lookup_args,path:*const c_char)->i32;
    pub fn btrfs_alloc_device(fs_info:*mut btrfs_fs_info,devid:*const u64,uuid:*const u8,path:*const c_char)->*mut btrfs_device;
    pub fn btrfs_put_dev_args_from_path(args:*mut btrfs_dev_lookup_args);
    pub fn btrfs_rm_device(fs_info:*mut btrfs_fs_info,args:*mut btrfs_dev_lookup_args,bdev_file:*mut *mut file)->i32;
    pub fn btrfs_cleanup_fs_uuids(); pub fn btrfs_grow_device(trans:*mut btrfs_trans_handle,device:*mut btrfs_device,new_size:u64)->i32;
    pub fn btrfs_find_device(fs_devices:*const btrfs_fs_devices,args:*const btrfs_dev_lookup_args)->*mut btrfs_device;
    pub fn btrfs_open_device_deny_freeze(path:*const c_char,sb:*mut super_block)->*mut file;
    pub fn btrfs_balance(fs_info:*mut btrfs_fs_info,bctl:*mut btrfs_balance_control,bargs:*mut btrfs_ioctl_balance_args)->i32;
    pub fn btrfs_describe_block_groups(flags:u64,buf:*mut c_char,size_buf:u32);
    pub fn btrfs_resume_balance_async(fs_info:*mut btrfs_fs_info)->i32; pub fn btrfs_recover_balance(fs_info:*mut btrfs_fs_info)->i32;
    pub fn btrfs_pause_balance(fs_info:*mut btrfs_fs_info)->i32; pub fn btrfs_relocate_chunk(fs_info:*mut btrfs_fs_info,chunk_offset:u64,verbose:bool)->i32;
    pub fn btrfs_dev_stat_inc_and_print(dev:*mut btrfs_device,index:i32);
    pub fn btrfs_init_devices_late(fs_info:*mut btrfs_fs_info)->i32; pub fn btrfs_init_dev_stats(fs_info:*mut btrfs_fs_info)->i32;
    pub fn btrfs_init_writeback_bio_size(fs_info:*mut btrfs_fs_info)->i32; pub fn btrfs_run_dev_stats(trans:*mut btrfs_trans_handle)->i32;
    pub fn btrfs_full_stripe_len(fs_info:*mut btrfs_fs_info,logical:u64)->c_ulong; pub fn btrfs_calc_stripe_length(map:*const btrfs_chunk_map)->u64;
    pub fn btrfs_nr_parity_stripes(r#type:u64)->i32; pub fn btrfs_remove_chunk(trans:*mut btrfs_trans_handle,chunk_offset:u64)->i32;
    pub fn btrfs_find_chunk_map(fs_info:*mut btrfs_fs_info,logical:u64,length:u64)->*mut btrfs_chunk_map;
    pub fn btrfs_find_chunk_map_nolock(fs_info:*mut btrfs_fs_info,logical:u64,length:u64)->*mut btrfs_chunk_map;
    pub fn btrfs_get_chunk_map(fs_info:*mut btrfs_fs_info,logical:u64,length:u64)->*mut btrfs_chunk_map;
    pub fn btrfs_remove_chunk_map(fs_info:*mut btrfs_fs_info,map:*mut btrfs_chunk_map);
    pub fn btrfs_read_disk_super(bdev:*mut block_device,copy_num:i32,drop_cache:bool)->*mut btrfs_super_block;
    pub fn btrfs_release_disk_super(super_:*mut btrfs_super_block);
    pub fn btrfs_commit_device_sizes(trans:*mut btrfs_transaction);
    pub fn btrfs_get_fs_uuids()->*mut list_head; pub fn btrfs_check_rw_degradable(fs_info:*mut btrfs_fs_info,failing_dev:*mut btrfs_device)->bool;
    pub fn btrfs_scratch_superblocks(fs_info:*mut btrfs_fs_info,device:*mut btrfs_device);
    pub fn btrfs_bg_flags_to_raid_index(flags:u64)->btrfs_raid_types; pub fn btrfs_bg_type_to_factor(flags:u64)->i32;
    pub fn btrfs_bg_type_to_raid_name(flags:u64)->*const c_char; pub fn btrfs_verify_dev_extents(fs_info:*mut btrfs_fs_info)->i32;
    pub fn btrfs_verify_dev_items(fs_info:*const btrfs_fs_info)->bool; pub fn btrfs_sb_fsid_ptr(sb:*const btrfs_super_block)->*const u8;
    pub fn btrfs_update_device(trans:*mut btrfs_trans_handle,device:*mut btrfs_device)->i32;
    pub fn btrfs_chunk_map_device_clear_bits(map:*mut btrfs_chunk_map,bits:c_uint);
    pub fn btrfs_first_pending_extent(device:*mut btrfs_device,start:u64,len:u64,pending_start:*mut u64,pending_end:*mut u64)->bool;
    pub fn btrfs_find_hole_in_pending_extents(device:*mut btrfs_device,start:*mut u64,len:*mut u64,min_hole_size:u64)->bool;
    pub fn btrfs_remove_dev_stat_item(trans:*mut btrfs_trans_handle,devid:u64)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
