// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of btrfs/scrub.c.
// Kernel and Btrfs types/functions are supplied by the surrounding crate.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const SCRUB_STRIPES_PER_GROUP: usize = 8;
pub const SCRUB_GROUPS_PER_SCTX: usize = 16;
pub const SCRUB_TOTAL_STRIPES: usize = SCRUB_GROUPS_PER_SCTX * SCRUB_STRIPES_PER_GROUP;

#[repr(C)]
pub union scrub_sector_verification {
    pub csum: *mut u8,
    pub generation: u64,
}

#[repr(u32)]
pub enum scrub_stripe_flags {
    SCRUB_STRIPE_FLAG_INITIALIZED,
    SCRUB_STRIPE_FLAG_REPAIR_DONE,
    SCRUB_STRIPE_FLAG_NO_REPORT,
}

#[repr(C)]
pub struct scrub_stripe {
    pub sctx: *mut scrub_ctx,
    pub bg: *mut btrfs_block_group,
    pub sectors: *mut scrub_sector_verification,
    pub dev: *mut btrfs_device,
    pub buffer: *mut c_void,
    pub logical: u64,
    pub physical: u64,
    pub mirror_num: u16,
    pub nr_sectors: u16,
    pub nr_data_extents: u16,
    pub nr_meta_extents: u16,
    pub pending_io: atomic_t,
    pub io_wait: wait_queue_head_t,
    pub repair_wait: wait_queue_head_t,
    pub state: usize,
    pub bitmaps: [usize; 8],
    pub write_error_bitmap: usize,
    pub write_error_lock: spinlock_t,
    pub csums: *mut u8,
    pub work: work_struct,
}

#[repr(C)]
pub struct scrub_ctx {
    pub stripes: [scrub_stripe; SCRUB_TOTAL_STRIPES],
    pub raid56_data_stripes: *mut scrub_stripe,
    pub fs_info: *mut btrfs_fs_info,
    pub extent_path: btrfs_path,
    pub csum_path: btrfs_path,
    pub first_free: i32,
    pub cur_stripe: i32,
    pub cancel_req: atomic_t,
    pub readonly: i32,
    pub throttle_deadline: ktime_t,
    pub throttle_sent: u64,
    pub is_dev_replace: bool,
    pub write_pointer: u64,
    pub wr_lock: mutex,
    pub wr_tgtdev: *mut btrfs_device,
    pub stat: btrfs_scrub_progress,
    pub stat_lock: spinlock_t,
    pub refs: refcount_t,
}

#[repr(C)]
pub struct scrub_warning { pub path: *mut btrfs_path, pub extent_item_size: u64, pub errstr: *const i8, pub physical: u64, pub logical: u64, pub dev: *mut btrfs_device }
#[repr(C)]
pub struct scrub_error_records { pub init_error_bitmap: usize, pub nr_io_errors: u32, pub nr_csum_errors: u32, pub nr_meta_errors: u32, pub nr_meta_gen_errors: u32 }

// Opaque declarations imported from the other Btrfs translation units.
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct refcount_t { _private: [u8; 0] }
pub type ktime_t = i64;
pub enum btrfs_fs_info {}
pub enum btrfs_block_group {}
pub enum btrfs_device {}
pub enum btrfs_path {}
pub enum btrfs_scrub_progress {}

extern "C" {
    pub fn btrfs_scrub_dev(fs_info: *mut btrfs_fs_info, devid: u64, start: u64, end: u64, progress: *mut btrfs_scrub_progress, readonly: bool, is_dev_replace: bool) -> i32;
    pub fn btrfs_scrub_pause(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_scrub_continue(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_scrub_cancel(fs_info: *mut btrfs_fs_info) -> i32;
    pub fn btrfs_scrub_cancel_dev(dev: *mut btrfs_device) -> i32;
    pub fn btrfs_scrub_progress(fs_info: *mut btrfs_fs_info, devid: u64, progress: *mut btrfs_scrub_progress) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
