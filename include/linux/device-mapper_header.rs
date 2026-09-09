/* SPDX-License-Identifier: GPL-2.0-only */
/* C header translation; external kernel types and functions are supplied elsewhere. */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

pub type sector_t = u64;
pub type blk_status_t = u8;
pub type blk_mode_t = u32;
pub type pgoff_t = u64;
pub type dev_t = u64;
pub type u64_alias = u64;
pub type u32_alias = u32;

#[repr(C)] pub struct dm_dev { pub bdev: *mut block_device, pub bdev_file: *mut file, pub dax_dev: *mut dax_device, pub mode: blk_mode_t, pub name: [c_char; 16] }
#[repr(C)] pub struct dm_target { pub table: *mut dm_table, pub type_: *mut target_type, pub begin: sector_t, pub len: sector_t, pub max_io_len: u32, pub num_flush_bios: u32, pub num_discard_bios: u32, pub num_secure_erase_bios: u32, pub num_write_zeroes_bios: u32, pub per_io_data_size: u32, pub private: *mut c_void, pub error: *mut c_char, pub flush_supported: bool, pub discards_supported: bool, pub zone_reset_all_supported: bool, pub max_discard_granularity: bool, pub limit_swap_bios: bool, pub emulate_zone_append: bool, pub accounts_remapped_io: bool, pub needs_bio_set_dev: bool, pub flush_bypasses_map: bool, pub mempool_needs_integrity: bool }
#[repr(C)] pub struct dm_table { _private: [u8; 0] }
#[repr(C)] pub struct mapped_device { _private: [u8; 0] }
#[repr(C)] pub struct dm_report_zones_args { _private: [u8; 0] }
#[repr(C)] pub struct bio { _private: [u8; 0] }
#[repr(C)] pub struct request { _private: [u8; 0] }
#[repr(C)] pub struct block_device { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct dax_device { _private: [u8; 0] }
#[repr(C)] pub struct queue_limits { pub logical_block_size: u32, pub physical_block_size: u32, pub io_min: u32 }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct gendisk { _private: [u8; 0] }
#[repr(C)] pub struct hd_geometry { _private: [u8; 0] }
#[repr(C)] pub struct dm_ioctl { _private: [u8; 0] }
#[repr(C)] pub struct dm_target_spec { _private: [u8; 0] }
#[repr(C)] pub struct blk_crypto_profile { _private: [u8; 0] }
#[repr(C)] pub struct iov_iter { _private: [u8; 0] }
#[repr(C)] pub struct blk_report_zones_args { _private: [u8; 0] }
pub type report_zones_cb = unsafe extern "C" fn(*mut c_void);
pub type dax_access_mode = c_int;

#[repr(C)] pub enum dm_queue_mode { DM_TYPE_NONE=0, DM_TYPE_BIO_BASED=1, DM_TYPE_REQUEST_BASED=2, DM_TYPE_DAX_BIO_BASED=3 }
#[repr(C)] pub enum status_type_t { STATUSTYPE_INFO, STATUSTYPE_TABLE, STATUSTYPE_IMA }
#[repr(C)] pub union map_info { pub ptr: *mut c_void }

pub type dm_ctr_fn = unsafe extern "C" fn(*mut dm_target, u32, *mut *mut c_char) -> c_int;
pub type dm_dtr_fn = unsafe extern "C" fn(*mut dm_target);
pub type dm_map_fn = unsafe extern "C" fn(*mut dm_target, *mut bio) -> c_int;
pub type dm_clone_and_map_request_fn = unsafe extern "C" fn(*mut dm_target,*mut request,*mut map_info,*mut *mut request)->c_int;
pub type dm_release_clone_request_fn = unsafe extern "C" fn(*mut request,*mut map_info);
pub type dm_endio_fn = unsafe extern "C" fn(*mut dm_target,*mut bio,*mut blk_status_t)->c_int;
pub type dm_request_endio_fn = unsafe extern "C" fn(*mut dm_target,*mut request,blk_status_t,*mut map_info)->c_int;
pub type dm_presuspend_fn = unsafe extern "C" fn(*mut dm_target); pub type dm_presuspend_undo_fn=dm_presuspend_fn; pub type dm_postsuspend_fn=dm_presuspend_fn; pub type dm_resume_fn=dm_presuspend_fn;
pub type dm_preresume_fn = unsafe extern "C" fn(*mut dm_target)->c_int;
pub type dm_status_fn = unsafe extern "C" fn(*mut dm_target,status_type_t,u32,*mut c_char,u32);
pub type dm_message_fn = unsafe extern "C" fn(*mut dm_target,u32,*mut *mut c_char,*mut c_char,u32)->c_int;
pub type dm_prepare_ioctl_fn = unsafe extern "C" fn(*mut dm_target,*mut *mut block_device,u32,c_ulong,*mut bool)->c_int;
#[cfg(feature="CONFIG_BLK_DEV_ZONED")] pub type dm_report_zones_fn=unsafe extern "C" fn(*mut dm_target,*mut dm_report_zones_args,u32)->c_int;
#[cfg(not(feature="CONFIG_BLK_DEV_ZONED"))] pub type dm_report_zones_fn=unsafe extern "C" fn(*mut dm_target)->c_int;
pub type iterate_devices_callout_fn=unsafe extern "C" fn(*mut dm_target,*mut dm_dev,sector_t,sector_t,*mut c_void)->c_int;
pub type dm_iterate_devices_fn=unsafe extern "C" fn(*mut dm_target,iterate_devices_callout_fn,*mut c_void)->c_int;
pub type dm_io_hints_fn=unsafe extern "C" fn(*mut dm_target,*mut queue_limits);
pub type dm_busy_fn=unsafe extern "C" fn(*mut dm_target)->c_int;
pub type dm_dax_direct_access_fn=unsafe extern "C" fn(*mut dm_target,pgoff_t,c_long,c_int,*mut *mut c_void,*mut c_ulong)->c_long;
pub type dm_dax_zero_page_range_fn=unsafe extern "C" fn(*mut dm_target,pgoff_t,usize)->c_int;
pub type dm_dax_recovery_write_fn=unsafe extern "C" fn(*mut dm_target,pgoff_t,*mut c_void,usize,*mut iov_iter)->usize;

#[repr(C)] pub struct target_type { pub features:u64, pub name:*const c_char, pub module:*mut module, pub version:[u32;3], pub ctr:Option<dm_ctr_fn>, pub dtr:Option<dm_dtr_fn>, pub map:Option<dm_map_fn>, pub clone_and_map_rq:Option<dm_clone_and_map_request_fn>, pub release_clone_rq:Option<dm_release_clone_request_fn>, pub end_io:Option<dm_endio_fn>, pub rq_end_io:Option<dm_request_endio_fn>, pub presuspend:Option<dm_presuspend_fn>, pub presuspend_undo:Option<dm_presuspend_undo_fn>, pub postsuspend:Option<dm_postsuspend_fn>, pub preresume:Option<dm_preresume_fn>, pub resume:Option<dm_resume_fn>, pub status:Option<dm_status_fn>, pub message:Option<dm_message_fn>, pub prepare_ioctl:Option<dm_prepare_ioctl_fn>, pub report_zones:Option<dm_report_zones_fn>, pub busy:Option<dm_busy_fn>, pub iterate_devices:Option<dm_iterate_devices_fn>, pub io_hints:Option<dm_io_hints_fn>, pub direct_access:Option<dm_dax_direct_access_fn>, pub dax_zero_page_range:Option<dm_dax_zero_page_range_fn>, pub dax_recovery_write:Option<dm_dax_recovery_write_fn>, pub list:list_head }

pub const DM_TARGET_SINGLETON:u64=0x1; pub const DM_TARGET_ALWAYS_WRITEABLE:u64=0x2; pub const DM_TARGET_IMMUTABLE:u64=0x4; pub const DM_TARGET_WILDCARD:u64=0x8; pub const DM_TARGET_INTEGRITY:u64=0x10; pub const DM_TARGET_PASSES_INTEGRITY:u64=0x20; pub const DM_TARGET_ZONED_HM:u64=0x40; pub const DM_TARGET_NOWAIT:u64=0x80; pub const DM_TARGET_PASSES_CRYPTO:u64=0x100; pub const DM_TARGET_MIXED_ZONED_MODEL:u64=0x200; pub const DM_TARGET_ATOMIC_WRITES:u64=0x400;
#[inline] pub unsafe fn dm_target_needs_singleton(t:*const target_type)->u64 { (*t).features & DM_TARGET_SINGLETON } #[inline] pub unsafe fn dm_target_always_writeable(t:*const target_type)->u64 { (*t).features & DM_TARGET_ALWAYS_WRITEABLE } #[inline] pub unsafe fn dm_target_is_immutable(t:*const target_type)->u64 { (*t).features & DM_TARGET_IMMUTABLE } #[inline] pub unsafe fn dm_target_is_wildcard(t:*const target_type)->u64 { (*t).features & DM_TARGET_WILDCARD } #[inline] pub unsafe fn dm_target_has_integrity(t:*const target_type)->u64 { (*t).features & DM_TARGET_INTEGRITY } #[inline] pub unsafe fn dm_target_passes_integrity(t:*const target_type)->u64 { (*t).features & DM_TARGET_PASSES_INTEGRITY } #[inline] pub unsafe fn dm_target_supports_zoned_hm(t:*const target_type)->u64 { (*t).features & DM_TARGET_ZONED_HM } #[inline] pub unsafe fn dm_target_supports_nowait(t:*const target_type)->u64 { (*t).features & DM_TARGET_NOWAIT } #[inline] pub unsafe fn dm_target_passes_crypto(t:*const target_type)->u64 { (*t).features & DM_TARGET_PASSES_CRYPTO } #[inline] pub unsafe fn dm_target_supports_mixed_zoned_model(t:*const target_type)->u64 { (*t).features & DM_TARGET_MIXED_ZONED_MODEL } #[inline] pub unsafe fn dm_target_supports_atomic_writes(t:*const target_type)->u64 { (*t).features & DM_TARGET_ATOMIC_WRITES }
pub const DM_ANY_MINOR:c_int=-1; pub const DM_ENDIO_DONE:u32=0; pub const DM_ENDIO_INCOMPLETE:u32=1; pub const DM_ENDIO_REQUEUE:u32=2; pub const DM_ENDIO_DELAY_REQUEUE:u32=3; pub const DM_MAPIO_SUBMITTED:u32=0; pub const DM_MAPIO_REMAPPED:u32=1; pub const DM_MAPIO_REQUEUE:u32=2; pub const DM_MAPIO_DELAY_REQUEUE:u32=3; pub const DM_MAPIO_KILL:u32=4;
#[repr(C)] pub struct dm_arg_set { pub argc:u32, pub argv:*mut *mut c_char } #[repr(C)] pub struct dm_arg { pub min:u32,pub max:u32,pub error:*mut c_char }
#[cfg(feature="CONFIG_BLK_DEV_ZONED")] #[repr(C)] pub struct dm_report_zones_args_full { pub tgt:*mut dm_target,pub disk:*mut gendisk,pub next_sector:sector_t,pub zone_idx:u32,pub rep_args:*mut blk_report_zones_args,pub cb:report_zones_cb,pub data:*mut c_void,pub start:sector_t }

extern "C" {
 pub fn dm_error(message:*const c_char); pub fn dm_get_device(ti:*mut dm_target,path:*const c_char,mode:blk_mode_t,result:*mut *mut dm_dev)->c_int; pub fn dm_put_device(ti:*mut dm_target,d:*mut dm_dev); pub fn dm_devt_from_path(path:*const c_char,dev_p:*mut dev_t)->c_int;
 pub fn dm_per_bio_data(bio:*mut bio,data_size:usize)->*mut c_void; pub fn dm_bio_from_per_bio_data(data:*mut c_void,data_size:usize)->*mut bio; pub fn dm_bio_get_target_bio_nr(bio:*const bio)->u32; pub fn dm_start_time_ns_from_clone(bio:*mut bio)->u64;
 pub fn dm_register_target(t:*mut target_type)->c_int; pub fn dm_unregister_target(t:*mut target_type); pub fn dm_read_arg(arg:*const dm_arg,arg_set:*mut dm_arg_set,value:*mut u32,error:*mut *mut c_char)->c_int; pub fn dm_read_arg_group(arg:*const dm_arg,arg_set:*mut dm_arg_set,num_args:*mut u32,error:*mut *mut c_char)->c_int; pub fn dm_shift_arg(as_:*mut dm_arg_set)->*const c_char; pub fn dm_consume_args(as_:*mut dm_arg_set,num_args:u32);
 pub fn dm_create(minor:c_int,md:*mut *mut mapped_device)->c_int; pub fn dm_get_md(dev:dev_t)->*mut mapped_device; pub fn dm_get(md:*mut mapped_device); pub fn dm_hold(md:*mut mapped_device)->c_int; pub fn dm_put(md:*mut mapped_device); pub fn dm_set_mdptr(md:*mut mapped_device,ptr:*mut c_void); pub fn dm_get_mdptr(md:*mut mapped_device)->*mut c_void; pub fn dm_suspend(md:*mut mapped_device,flags:u32)->c_int; pub fn dm_resume(md:*mut mapped_device)->c_int;
 pub fn dm_get_event_nr(md:*mut mapped_device)->u32; pub fn dm_wait_event(md:*mut mapped_device,event_nr:c_int)->c_int; pub fn dm_next_uevent_seq(md:*mut mapped_device)->u32; pub fn dm_uevent_add(md:*mut mapped_device,elist:*mut list_head); pub fn dm_device_name(md:*mut mapped_device)->*const c_char; pub fn dm_copy_name_and_uuid(md:*mut mapped_device,name:*mut c_char,uuid:*mut c_char)->c_int; pub fn dm_disk(md:*mut mapped_device)->*mut gendisk; pub fn dm_suspended(ti:*mut dm_target)->c_int; pub fn dm_post_suspending(ti:*mut dm_target)->c_int; pub fn dm_noflush_suspending(ti:*mut dm_target)->c_int; pub fn dm_accept_partial_bio(bio:*mut bio,n_sectors:u32); pub fn dm_submit_bio_remap(clone:*mut bio,tgt_clone:*mut bio);
 pub fn dm_early_create(dmi:*mut dm_ioctl,spec_array:*mut *mut dm_target_spec,target_params_array:*mut *mut c_char)->c_int;
 pub fn dm_get_geometry(md:*mut mapped_device,geo:*mut hd_geometry)->c_int; pub fn dm_set_geometry(md:*mut mapped_device,geo:*mut hd_geometry)->c_int; pub fn dm_table_create(result:*mut *mut dm_table,mode:blk_mode_t,num_targets:u32,md:*mut mapped_device)->c_int; pub fn dm_table_add_target(t:*mut dm_table,type_:*const c_char,start:sector_t,len:sector_t,params:*mut c_char)->c_int; pub fn dm_table_set_type(t:*mut dm_table,type_:dm_queue_mode); pub fn dm_table_complete(t:*mut dm_table)->c_int; pub fn dm_table_destroy(t:*mut dm_table); pub fn dm_set_target_max_io_len(ti:*mut dm_target,len:sector_t)->c_int; pub fn dm_get_live_table(md:*mut mapped_device,srcu_idx:*mut c_int)->*mut dm_table; pub fn dm_put_live_table(md:*mut mapped_device,srcu_idx:c_int); pub fn dm_sync_table(md:*mut mapped_device); pub fn dm_table_get_size(t:*mut dm_table)->sector_t; pub fn dm_table_get_mode(t:*mut dm_table)->blk_mode_t; pub fn dm_table_get_md(t:*mut dm_table)->*mut mapped_device; pub fn dm_table_device_name(t:*mut dm_table)->*const c_char; pub fn dm_table_event(t:*mut dm_table); pub fn dm_table_run_md_queue_async(t:*mut dm_table); pub fn dm_swap_table(md:*mut mapped_device,t:*mut dm_table)->*mut dm_table; pub fn dm_destroy_crypto_profile(profile:*mut blk_crypto_profile);
 #[cfg(feature="CONFIG_BLK_DEV_ZONED")] pub fn dm_report_zones(bdev:*mut block_device,start:sector_t,sector:sector_t,args:*mut dm_report_zones_args,nr_zones:u32)->c_int;
}

pub const DM_NAME:&[u8]=b"device-mapper\0"; pub const SECTOR_SHIFT:u32=9;
#[inline] pub const fn to_sector(n:u64)->sector_t { n >> SECTOR_SHIFT }
#[inline] pub const fn to_bytes(n:sector_t)->u64 { n << SECTOR_SHIFT }
#[inline] pub const fn dm_div_up(n:u64,sz:u64)->u64 { (n + sz - 1) / sz }
#[inline] pub const fn dm_round_up(n:u64,sz:u64)->u64 { dm_div_up(n,sz) * sz }
#[inline] pub unsafe fn dm_sector_div_up(n:sector_t,sz:sector_t)->sector_t { dm_div_up(n,sz) }
#[inline] pub unsafe fn dm_target_offset(ti:*const dm_target,sector:sector_t)->sector_t { sector - (*ti).begin }
#[inline] pub unsafe fn dm_stack_bs_limits(limits:*mut queue_limits,bs:u32) { (*limits).logical_block_size=(*limits).logical_block_size.max(bs); (*limits).physical_block_size=(*limits).physical_block_size.max(bs); (*limits).io_min=(*limits).io_min.max(bs); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
