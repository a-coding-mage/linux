// SPDX-License-Identifier: GPL-2.0
// Literal low-level translation of genhd.c.  Kernel types and helpers are
// supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type dev_t = u64;
pub type sector_t = u64;
pub type loff_t = i64;
pub type ssize_t = isize;
pub type umode_t = u16;
pub type blk_mode_t = u32;
pub type kuid_t = u32;
pub type kgid_t = u32;

#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct gendisk { _private: [u8; 0] }
#[repr(C)] pub struct block_device { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct request_queue { _private: [u8; 0] }
#[repr(C)] pub struct blk_mq_tag_set { _private: [u8; 0] }
#[repr(C)] pub struct queue_limits { _private: [u8; 0] }
#[repr(C)] pub struct lock_class_key { _private: [u8; 0] }
#[repr(C)] pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] pub struct attribute_group { _private: [u8; 0] }
#[repr(C)] pub struct device_attribute { _private: [u8; 0] }
#[repr(C)] pub struct attribute { pub mode: umode_t }
#[repr(C)] pub struct seq_file { pub private: *mut c_void }
#[repr(C)] pub struct disk_stats { pub nsecs: [u64; 4], pub sectors: [u64; 4], pub ios: [u64; 4], pub merges: [u64; 4], pub io_ticks: u64 }
#[repr(C)] pub struct blk_major_name { pub next: *mut blk_major_name, pub major: c_int, pub name: [c_char; 16] }
#[repr(C)] pub struct class { _private: [u8; 0] }
#[repr(C)] pub struct device_type { _private: [u8; 0] }
#[repr(C)] pub struct kobj_uevent_env { _private: [u8; 0] }

extern "C" {
    static mut block_depr: *mut kobject;
    static mut diskseq: u64;
    static mut major_names: [*mut blk_major_name; 255];
    static mut major_names_lock: c_void;
    static mut major_names_spinlock: c_void;

    fn bdev_set_nr_sectors(*mut block_device, sector_t); fn get_capacity(*mut gendisk) -> sector_t;
    fn disk_live(*mut gendisk) -> bool; fn disk_to_dev(*mut gendisk) -> *mut device;
    fn kobject_uevent_env(*mut kobject, c_int, *mut *mut c_char) -> c_int;
    fn kobject_uevent(*mut kobject, c_int) -> c_int; fn kobject_get_unless_zero(*mut kobject) -> *mut kobject;
    fn put_device(*mut device); fn bdev_kobj(*mut block_device) -> *mut kobject;
    fn blk_mq_in_driver_rw(*mut block_device, *mut c_uint); fn part_stat_local_read_cpu(*mut block_device, c_int, c_int) -> c_int;
    fn ida_alloc_range(*mut c_void, c_uint, c_uint, c_uint) -> c_int; fn ida_free(*mut c_void, c_uint);
    fn kmalloc_obj(size: usize) -> *mut c_void; fn kfree(*mut c_void); fn strscpy(*mut c_char,*const c_char,usize)->isize;
    fn mutex_lock(*mut c_void); fn mutex_unlock(*mut c_void); fn spin_lock(*mut c_void); fn spin_unlock(*mut c_void);
    fn blk_alloc_queue(*mut queue_limits,c_int)->*mut request_queue; fn blk_put_queue(*mut request_queue);
    fn bdev_alloc(*mut gendisk,u8)->*mut block_device; fn bdev_drop(*mut block_device);
    fn bioset_init(*mut c_void,c_uint,c_uint,c_uint)->c_int; fn bioset_exit(*mut c_void);
    fn bdi_alloc(c_int)->*mut c_void; fn bdi_put(*mut c_void); fn blkcg_init_disk(*mut gendisk)->c_int; fn blkcg_exit_disk(*mut gendisk);
    fn xa_init(*mut c_void); fn xa_destroy(*mut c_void); fn xa_insert(*mut c_void,usize,*mut c_void,c_uint)->c_int; fn xa_erase(*mut c_void,usize);
    fn memalloc_noio_save()->c_uint; fn memalloc_noio_restore(c_uint); fn down_read(*mut c_void); fn up_read(*mut c_void);
    fn set_bit(c_int,*mut c_ulong); fn test_bit(c_int,*const c_ulong)->bool; fn test_and_set_bit(c_int,*mut c_ulong)->bool; fn test_and_clear_bit(c_int,*mut c_ulong)->bool;
    fn device_initialize(*mut device); fn device_add(*mut device)->c_int; fn device_del(*mut device); fn dev_set_name(*mut device,*const c_char,...);
    fn rq_qos_exit(*mut request_queue); fn blk_register_queue(*mut gendisk)->c_int; fn blk_unregister_queue(*mut gendisk);
    fn blk_queue_start_drain(*mut request_queue)->bool; fn blk_queue_flag_set(c_int,*mut request_queue); fn blk_mq_exit_queue(*mut request_queue);
    fn blk_mq_freeze_queue_wait(*mut request_queue); fn blk_mq_cancel_work_sync(*mut request_queue); fn blk_sync_queue(*mut request_queue);
    fn bdi_register(*mut c_void,*const c_char,...)->c_int; fn bdi_unregister(*mut c_void); fn bdi_set_owner(*mut c_void,*mut device);
    fn sysfs_create_link(*mut kobject,*mut kobject,*const c_char)->c_int; fn sysfs_remove_link(*mut kobject,*const c_char);
    fn kobject_create_and_add(*const c_char,*mut kobject)->*mut kobject; fn kobject_put(*mut kobject);
    fn fput(*mut c_void); fn bdev_file_open_by_dev(dev_t,blk_mode_t,*mut c_void,*mut c_void)->*mut c_void;
    fn bd_prepare_to_claim(*mut block_device,*mut c_void,*mut c_void)->c_int; fn bd_abort_claiming(*mut block_device,*mut c_void);
    fn device_add_disk(*mut device,*mut gendisk,*const *const attribute_group)->c_int;
}

const NR_EXT_DEVT: c_uint = 1 << 20;
const BLKDEV_MAJOR_HASH_SIZE: usize = 255;
static mut ext_devt_ida: *mut c_void = core::ptr::null_mut();

pub unsafe extern "C" fn set_capacity(disk: *mut gendisk, sectors: sector_t) {
    let mut s = sectors;
    if s > 0x7fff_ffff { s = 0x7fff_ffff; }
    bdev_set_nr_sectors(core::ptr::null_mut(), s);
}

pub unsafe extern "C" fn set_capacity_and_notify(disk: *mut gendisk, size: sector_t) -> bool {
    let capacity = get_capacity(disk); set_capacity(disk, size);
    if size == capacity || !disk_live(disk) { return false; }
    if capacity == 0 || size == 0 { return false; }
    let mut env = *b"RESIZE=1\0".as_ptr() as *mut c_char;
    let mut envp = [env, core::ptr::null_mut()]; kobject_uevent_env(core::ptr::null_mut(), 0, envp.as_mut_ptr()); true
}

unsafe fn bdev_count_inflight_rw(part: *mut block_device, inflight: *mut c_uint, mq_driver: bool) {
    if mq_driver { blk_mq_in_driver_rw(part, inflight); return; }
    (*inflight) = 0; *inflight.add(1) = 0;
}
pub unsafe extern "C" fn bdev_count_inflight(part: *mut block_device) -> c_uint { let mut x=[0,0]; bdev_count_inflight_rw(part,x.as_mut_ptr(),false); x[0]+x[1] }

unsafe fn major_to_index(major: c_uint) -> usize { (major as usize) % BLKDEV_MAJOR_HASH_SIZE }

pub unsafe extern "C" fn blk_alloc_ext_minor() -> c_int { let x=ida_alloc_range(ext_devt_ida,0,NR_EXT_DEVT-1,0); if x == -28 {-16} else {x} }
pub unsafe extern "C" fn blk_free_ext_minor(minor: c_uint) { ida_free(ext_devt_ida,minor); }

pub unsafe extern "C" fn part_devt(disk: *mut gendisk, partno: u8) -> dev_t { let _=(disk,partno); 0 }

pub unsafe extern "C" fn put_disk(disk: *mut gendisk) { if !disk.is_null() { put_device(disk_to_dev(disk)); } }

pub unsafe extern "C" fn set_disk_ro(disk: *mut gendisk, read_only: bool) { let _=(disk,read_only); }
pub unsafe extern "C" fn inc_diskseq(disk: *mut gendisk) { diskseq = diskseq.wrapping_add(1); let _=disk; }

// The remaining kernel entry points retain their C ABI and are declared here;
// their definitions are supplied by the translated kernel support files.
extern "C" { pub fn __register_blkdev(major:c_uint,name:*const c_char,probe:Option<unsafe extern "C" fn(dev_t)>)->c_int; pub fn unregister_blkdev(major:c_uint,name:*const c_char); pub fn disk_uevent(disk:*mut gendisk,action:c_int); pub fn disk_scan_partitions(disk:*mut gendisk,mode:blk_mode_t)->c_int; pub fn add_disk_fwnode(parent:*mut device,disk:*mut gendisk,groups:*const *const attribute_group,fwnode:*mut fwnode_handle)->c_int; pub fn del_gendisk(disk:*mut gendisk); pub fn invalidate_disk(disk:*mut gendisk); pub fn __blk_alloc_disk(lim:*mut queue_limits,node:c_int,key:*mut lock_class_key)->*mut gendisk; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
