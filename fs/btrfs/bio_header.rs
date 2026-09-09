/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2007 Oracle.  All rights reserved.
 * Copyright (C) 2022 Christoph Hellwig.
 */

// Dependencies supplied by the surrounding kernel translation unit.

pub type u8 = core::ffi::c_uchar;
pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;
pub type phys_addr_t = usize;
pub type blk_status_t = u8;
pub type blk_opf_t = u32;
pub type atomic_t = core::ffi::c_int;

#[repr(C)]
pub struct btrfs_bio;
#[repr(C)]
pub struct btrfs_fs_info;
#[repr(C)]
pub struct btrfs_inode;
#[repr(C)]
pub struct btrfs_ordered_extent;
#[repr(C)]
pub struct btrfs_ordered_sum;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bvec_iter {
    _private: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct completion {
    _private: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_tree_parent_check {
    _private: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bio {
    _private: [u8; 0],
}

pub const BTRFS_BIO_INLINE_CSUM_SIZE: usize = 64;

pub type btrfs_bio_end_io_t = unsafe extern "C" fn(bbio: *mut btrfs_bio);

#[repr(C)]
pub union btrfs_bio_data {
    pub read: btrfs_bio_read_data,
    pub write: btrfs_bio_write_data,
    pub parent_check: btrfs_tree_parent_check,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_bio_read_data {
    pub csum: *mut u8,
    pub csum_inline: [u8; BTRFS_BIO_INLINE_CSUM_SIZE],
    pub saved_iter: bvec_iter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_bio_write_data {
    pub ordered: *mut btrfs_ordered_extent,
    pub sums: *mut btrfs_ordered_sum,
    pub csum_work: work_struct,
    pub csum_done: completion,
    pub csum_saved_iter: bvec_iter,
    pub orig_physical: u64,
    pub orig_logical: u64,
}

#[repr(C)]
pub struct btrfs_bio {
    pub inode: *mut btrfs_inode,
    pub file_offset: u64,
    pub data: btrfs_bio_data,
    pub end_io_work: work_struct,
    pub end_io: Option<btrfs_bio_end_io_t>,
    pub private: *mut core::ffi::c_void,
    pub pending_ios: atomic_t,
    pub mirror_num: u16,
    pub status: blk_status_t,
    pub csum_search_commit_root: bool,
    pub is_scrub: bool,
    pub is_remap: bool,
    pub async_csum: bool,
    pub can_use_append: bool,
    pub bio: bio,
}

pub const REQ_BTRFS_CGROUP_PUNT: u32 = 1 << 23; // REQ_FS_PRIVATE

pub unsafe fn btrfs_bio(bio: *mut bio) -> *mut btrfs_bio {
    (bio as *mut u8).sub(core::mem::offset_of!(btrfs_bio, bio)) as *mut btrfs_bio
}

unsafe extern "C" {
    pub fn btrfs_bioset_init() -> core::ffi::c_int;
    pub fn btrfs_bioset_exit();
    pub fn btrfs_bio_init(
        bbio: *mut btrfs_bio,
        inode: *mut btrfs_inode,
        file_offset: u64,
        end_io: Option<btrfs_bio_end_io_t>,
        private: *mut core::ffi::c_void,
    );
    pub fn btrfs_bio_alloc(
        nr_vecs: core::ffi::c_uint,
        opf: blk_opf_t,
        inode: *mut btrfs_inode,
        file_offset: u64,
        end_io: Option<btrfs_bio_end_io_t>,
        private: *mut core::ffi::c_void,
    ) -> *mut btrfs_bio;
    pub fn btrfs_bio_end_io(bbio: *mut btrfs_bio, status: blk_status_t);
    pub fn btrfs_submit_bbio(bbio: *mut btrfs_bio, mirror_num: core::ffi::c_int);
    pub fn btrfs_submit_repair_write(
        bbio: *mut btrfs_bio,
        mirror_num: core::ffi::c_int,
        dev_replace: bool,
    );
    pub fn btrfs_repair_io_failure(
        fs_info: *mut btrfs_fs_info,
        ino: u64,
        fileoff: u64,
        length: u32,
        logical: u64,
        paddrs: *const phys_addr_t,
        step: core::ffi::c_uint,
        mirror_num: core::ffi::c_int,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
