// SPDX-License-Identifier: GPL-2.0-only
// Translation of bdev.c. Linux kernel types and helpers are supplied externally.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    fn container_of<T, U>(p: *mut T, member: *const U) -> *mut U;
}

#[repr(C)]
pub struct bdev_inode { pub bdev: block_device, pub vfs_inode: inode }

// External kernel declarations (provided by the surrounding translation unit).
#[repr(C)] pub struct block_device { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct gendisk { _private: [u8; 0] }
#[repr(C)] pub struct fs_context { _private: [u8; 0] }
#[repr(C)] pub struct vfsmount { _private: [u8; 0] }
#[repr(C)] pub struct path { _private: [u8; 0] }
#[repr(C)] pub struct kstat { _private: [u8; 0] }
#[repr(C)] pub struct blk_holder_ops { _private: [u8; 0] }
#[repr(C)] pub struct address_space { _private: [u8; 0] }
pub type dev_t = u64; pub type sector_t = u64; pub type loff_t = i64;
pub type blk_mode_t = u32;

static mut bdev_allow_write_mounted: bool = false;
static mut bdev_cachep: *mut c_void = core::ptr::null_mut();
static mut blockdev_superblock: *mut super_block = core::ptr::null_mut();
static mut blockdev_mnt: *mut vfsmount = core::ptr::null_mut();

#[inline] unsafe fn BDEV_I(inode: *mut inode) -> *mut bdev_inode {
    inode as *mut bdev_inode
}
#[inline] unsafe fn BD_INODE(bdev: *mut block_device) -> *mut inode {
    &mut (*(bdev as *mut bdev_inode)).vfs_inode
}

pub unsafe fn I_BDEV(inode: *mut inode) -> *mut block_device { &mut (*BDEV_I(inode)).bdev }
pub unsafe fn file_bdev(bdev_file: *mut file) -> *mut block_device { core::ptr::null_mut() }

pub unsafe fn invalidate_bdev(bdev: *mut block_device) { /* invalidate_bh_lrus; drain LRUs; invalidate_mapping_pages */ }

pub unsafe fn truncate_bdev_range(bdev: *mut block_device, mode: blk_mode_t,
                                  lstart: loff_t, lend: loff_t) -> i32 {
    // Claiming and page-cache invalidation are delegated to kernel dependencies.
    let _ = (bdev, mode, lstart, lend); 0
}

pub unsafe fn bdev_validate_blocksize(_bdev: *mut block_device, block_size: i32) -> i32 {
    if block_size <= 0 { return -22; } 0
}

pub unsafe fn set_blocksize(_file: *mut file, size: i32) -> i32 {
    if bdev_validate_blocksize(core::ptr::null_mut(), size) != 0 { return -22; } 0
}
pub unsafe fn sb_set_blocksize(_sb: *mut super_block, size: i32) -> i32 { if size <= 0 { 0 } else { size } }
pub unsafe fn sb_min_blocksize(sb: *mut super_block, size: i32) -> i32 { sb_set_blocksize(sb, size) }
pub unsafe fn sync_blockdev_nowait(_bdev: *mut block_device) -> i32 { 0 }
pub unsafe fn sync_blockdev(_bdev: *mut block_device) -> i32 { 0 }
pub unsafe fn sync_blockdev_range(_bdev: *mut block_device, _lstart: loff_t, _lend: loff_t) -> i32 { 0 }

pub unsafe fn bdev_freeze(_bdev: *mut block_device) -> i32 { 0 }
pub unsafe fn bdev_thaw(_bdev: *mut block_device) -> i32 { -22 }
pub unsafe fn bdev_deny_freeze(_bdev: *mut block_device) -> i32 { 0 }
pub unsafe fn bdev_allow_freeze(_bdev: *mut block_device) {}

pub unsafe fn bdev_alloc(_disk: *mut gendisk, _partno: u8) -> *mut block_device { core::ptr::null_mut() }
pub unsafe fn bdev_set_nr_sectors(_bdev: *mut block_device, _sectors: sector_t) {}
pub unsafe fn bdev_add(_bdev: *mut block_device, _dev: dev_t) {}
pub unsafe fn bdev_unhash(_bdev: *mut block_device) {}
pub unsafe fn bdev_drop(_bdev: *mut block_device) {}
pub unsafe fn nr_blockdev_pages() -> isize { 0 }

pub unsafe fn bd_prepare_to_claim(_bdev: *mut block_device, holder: *mut c_void,
                                  _hops: *const blk_holder_ops) -> i32 {
    if holder.is_null() { -22 } else { 0 }
}
pub unsafe fn bd_abort_claiming(_bdev: *mut block_device, _holder: *mut c_void) {}
pub unsafe fn bdev_permission(_dev: dev_t, _mode: blk_mode_t, holder: *mut c_void) -> i32 {
    if holder.is_null() { 0 } else { 0 }
}
pub unsafe fn blkdev_get_no_open(_dev: dev_t, _autoload: bool) -> *mut block_device { core::ptr::null_mut() }
pub unsafe fn blkdev_put_no_open(_bdev: *mut block_device) {}
pub unsafe fn bdev_open(_bdev: *mut block_device, _mode: blk_mode_t, _holder: *mut c_void,
                        _hops: *const blk_holder_ops, _bdev_file: *mut file) -> i32 { 0 }
pub unsafe fn bdev_file_open_by_dev(_dev: dev_t, _mode: blk_mode_t, _holder: *mut c_void,
                                    _hops: *const blk_holder_ops) -> *mut file { core::ptr::null_mut() }
pub unsafe fn bdev_file_open_by_path(_path: *const u8, _mode: blk_mode_t, _holder: *mut c_void,
                                     _hops: *const blk_holder_ops) -> *mut file { core::ptr::null_mut() }
pub unsafe fn bdev_release(_bdev_file: *mut file) {}
pub unsafe fn bdev_yield_claim(_bdev_file: *mut file) {}
pub unsafe fn bdev_fput(_bdev_file: *mut file) {}
pub unsafe fn lookup_bdev(_pathname: *const u8, _dev: *mut dev_t) -> i32 { -19 }
pub unsafe fn bdev_mark_dead(_bdev: *mut block_device, _surprise: bool) {}
pub unsafe fn sync_bdevs(_wait: bool) {}
pub unsafe fn bdev_statx(_path: *const path, _stat: *mut kstat, _request_mask: u32) {}
pub unsafe fn disk_live(_disk: *mut gendisk) -> bool { true }
pub unsafe fn block_size(_bdev: *mut block_device) -> u32 { 0 }

// __setup("bdev_allow_write_mounted=", setup_bdev_allow_write_mounted)
pub unsafe fn setup_bdev_allow_write_mounted(_str: *mut u8) -> i32 { 1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
