/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2012 Fusion-io  All rights reserved.
 * Copyright (C) 2012 Intel Corp. All rights reserved.
 */

// C dependencies: linux/types.h, linux/list.h, linux/spinlock.h,
// linux/bio.h, linux/refcount.h, linux/workqueue.h, and volumes.h.

pub struct page;
pub struct btrfs_fs_info;
pub struct btrfs_device;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum btrfs_rbio_ops {
    BTRFS_RBIO_WRITE,
    BTRFS_RBIO_READ_REBUILD,
    BTRFS_RBIO_PARITY_SCRUB,
}

#[repr(C)]
pub struct btrfs_raid_bio {
    pub bioc: *mut btrfs_io_context,
    pub hash_list: list_head,
    pub stripe_cache: list_head,
    pub work: work_struct,
    pub bio_list: bio_list,
    pub bio_list_lock: spinlock_t,
    pub plug_list: list_head,
    pub flags: ::core::ffi::c_ulong,
    pub operation: btrfs_rbio_ops,
    pub nr_pages: u16,
    pub nr_sectors: u16,
    pub nr_data: u8,
    pub real_stripes: u8,
    pub stripe_npages: u8,
    pub stripe_nsectors: u8,
    pub sector_nsteps: u8,
    pub scrubp: u8,
    pub bio_list_bytes: ::core::ffi::c_int,
    pub refs: refcount_t,
    pub stripes_pending: atomic_t,
    pub io_wait: wait_queue_head_t,
    pub dbitmap: ::core::ffi::c_ulong,
    pub finish_pbitmap: ::core::ffi::c_ulong,
    pub stripe_pages: *mut *mut page,
    pub bio_paddrs: *mut phys_addr_t,
    pub stripe_paddrs: *mut phys_addr_t,
    pub stripe_uptodate_bitmap: *mut ::core::ffi::c_ulong,
    pub finish_pointers: *mut *mut ::core::ffi::c_void,
    pub error_bitmap: *mut ::core::ffi::c_ulong,
    pub csum_buf: *mut u8,
    pub csum_bitmap: *mut ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct raid56_bio_trace_info {
    pub devid: u64,
    pub offset: u32,
    pub stripe_nr: u8,
}

#[inline]
pub unsafe fn nr_data_stripes(map: *const btrfs_chunk_map) -> ::core::ffi::c_int {
    (*map).num_stripes - btrfs_nr_parity_stripes((*map).type_)
}

#[inline]
pub unsafe fn nr_bioc_data_stripes(bioc: *const btrfs_io_context) -> ::core::ffi::c_int {
    (*bioc).num_stripes - btrfs_nr_parity_stripes((*bioc).map_type)
}

pub const RAID5_P_STRIPE: u64 = (-2i64) as u64;
pub const RAID6_Q_STRIPE: u64 = (-1i64) as u64;

#[inline]
pub const fn is_parity_stripe(x: u64) -> bool {
    x == RAID5_P_STRIPE || x == RAID6_Q_STRIPE
}

extern "C" {
    pub fn raid56_parity_recover(
        bio: *mut bio,
        bioc: *mut btrfs_io_context,
        mirror_num: ::core::ffi::c_int,
    );
    pub fn raid56_parity_write(bio: *mut bio, bioc: *mut btrfs_io_context);

    pub fn raid56_parity_alloc_scrub_rbio(
        bio: *mut bio,
        bioc: *mut btrfs_io_context,
        scrub_dev: *mut btrfs_device,
        dbitmap: *mut ::core::ffi::c_ulong,
        stripe_nsectors: ::core::ffi::c_int,
    ) -> *mut btrfs_raid_bio;
    pub fn raid56_parity_submit_scrub_rbio(rbio: *mut btrfs_raid_bio);

    pub fn raid56_parity_cache_data_folios(
        rbio: *mut btrfs_raid_bio,
        vaddr: *mut ::core::ffi::c_void,
        data_logical: u64,
    );

    pub fn btrfs_alloc_stripe_hash_table(info: *mut btrfs_fs_info) -> ::core::ffi::c_int;
    pub fn btrfs_free_stripe_hash_table(info: *mut btrfs_fs_info);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
