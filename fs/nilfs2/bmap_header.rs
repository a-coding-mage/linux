/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * NILFS block mapping.
 *
 * Copyright (C) 2006-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Written by Koji Sato.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/types.h, linux/fs.h, linux/buffer_head.h, linux/nilfs2_ondisk.h,
// alloc.h, and dat.h.

pub const NILFS_BMAP_INVALID_PTR: u64 = 0;

#[inline]
pub const fn nilfs_bmap_keydiff_abs(diff: i64) -> i64 {
    if diff < 0 { -diff } else { diff }
}

#[repr(C)]
pub union nilfs_bmap_ptr_req {
    pub bpr_ptr: u64,
    pub bpr_req: nilfs_palloc_req,
}

#[repr(C)]
pub struct nilfs_bmap_stats {
    pub bs_nblocks: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct nilfs_bmap_operations {
    pub bop_lookup: Option<unsafe extern "C" fn(*const nilfs_bmap, u64, ::core::ffi::c_int, *mut u64) -> ::core::ffi::c_int>,
    pub bop_lookup_contig: Option<unsafe extern "C" fn(*const nilfs_bmap, u64, *mut u64, ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub bop_insert: Option<unsafe extern "C" fn(*mut nilfs_bmap, u64, u64) -> ::core::ffi::c_int>,
    pub bop_delete: Option<unsafe extern "C" fn(*mut nilfs_bmap, u64, bool) -> ::core::ffi::c_int>,
    pub bop_clear: Option<unsafe extern "C" fn(*mut nilfs_bmap)>,
    pub bop_propagate: Option<unsafe extern "C" fn(*mut nilfs_bmap, *mut buffer_head) -> ::core::ffi::c_int>,
    pub bop_lookup_dirty_buffers: Option<unsafe extern "C" fn(*mut nilfs_bmap, *mut list_head)>,
    pub bop_assign: Option<unsafe extern "C" fn(*mut nilfs_bmap, *mut *mut buffer_head, sector_t, *mut nilfs_binfo) -> ::core::ffi::c_int>,
    pub bop_mark: Option<unsafe extern "C" fn(*mut nilfs_bmap, u64, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub bop_seek_key: Option<unsafe extern "C" fn(*const nilfs_bmap, u64, *mut u64) -> ::core::ffi::c_int>,
    pub bop_last_key: Option<unsafe extern "C" fn(*const nilfs_bmap, *mut u64) -> ::core::ffi::c_int>,
    pub bop_check_insert: Option<unsafe extern "C" fn(*const nilfs_bmap, u64) -> ::core::ffi::c_int>,
    pub bop_check_delete: Option<unsafe extern "C" fn(*mut nilfs_bmap, u64) -> ::core::ffi::c_int>,
    pub bop_gather_data: Option<unsafe extern "C" fn(*mut nilfs_bmap, *mut u64, *mut u64, ::core::ffi::c_int) -> ::core::ffi::c_int>,
}

pub const NILFS_BMAP_SIZE: usize = NILFS_INODE_BMAP_SIZE * core::mem::size_of::<u64>();
pub const NILFS_BMAP_KEY_BIT: usize = BITS_PER_LONG;
pub const NILFS_BMAP_NEW_PTR_INIT: usize = 1usize << (BITS_PER_LONG - 1);

#[inline]
pub unsafe fn nilfs_bmap_is_new_ptr(ptr: usize) -> ::core::ffi::c_int {
    ((ptr & NILFS_BMAP_NEW_PTR_INIT) != 0) as ::core::ffi::c_int
}

#[repr(C)]
pub union nilfs_bmap_b_u {
    pub u_flags: u8,
    pub u_data: [__le64; NILFS_BMAP_SIZE / core::mem::size_of::<__le64>()],
}

#[repr(C)]
pub struct nilfs_bmap {
    pub b_u: nilfs_bmap_b_u,
    pub b_sem: rw_semaphore,
    pub b_inode: *mut inode,
    pub b_ops: *const nilfs_bmap_operations,
    pub b_last_allocated_key: u64,
    pub b_last_allocated_ptr: u64,
    pub b_ptr_type: ::core::ffi::c_int,
    pub b_state: ::core::ffi::c_int,
    pub b_nchildren_per_block: u16,
}

pub const NILFS_BMAP_PTR_P: i32 = 0;
pub const NILFS_BMAP_PTR_VS: i32 = 1;
pub const NILFS_BMAP_PTR_VM: i32 = 2;
pub const NILFS_BMAP_PTR_U: i32 = -1;

#[inline]
pub unsafe fn NILFS_BMAP_USE_VBN(bmap: *const nilfs_bmap) -> bool { (*bmap).b_ptr_type > 0 }
pub const NILFS_BMAP_DIRTY: i32 = 0x00000001;

#[repr(C)]
pub struct nilfs_bmap_store {
    pub data: [__le64; NILFS_BMAP_SIZE / core::mem::size_of::<__le64>()],
    pub last_allocated_key: u64,
    pub last_allocated_ptr: u64,
    pub state: ::core::ffi::c_int,
}

extern "C" {
    pub fn nilfs_bmap_test_and_clear_dirty(bmap: *mut nilfs_bmap) -> ::core::ffi::c_int;
    pub fn nilfs_bmap_read(bmap: *mut nilfs_bmap, raw_inode: *mut nilfs_inode) -> ::core::ffi::c_int;
    pub fn nilfs_bmap_write(bmap: *mut nilfs_bmap, raw_inode: *mut nilfs_inode);
    pub fn nilfs_bmap_lookup_contig(bmap: *mut nilfs_bmap, key: u64, ptr: *mut u64, nkeys: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn nilfs_bmap_insert(bmap: *mut nilfs_bmap, key: u64, rec: usize) -> ::core::ffi::c_int;
    pub fn nilfs_bmap_delete(bmap: *mut nilfs_bmap, key: u64) -> ::core::ffi::c_int;
    pub fn nilfs_bmap_seek_key(bmap: *mut nilfs_bmap, start: u64, keyp: *mut u64) -> ::core::ffi::c_int;
    pub fn nilfs_bmap_last_key(bmap: *mut nilfs_bmap, keyp: *mut u64) -> ::core::ffi::c_int;
    pub fn nilfs_bmap_truncate(bmap: *mut nilfs_bmap, key: u64) -> ::core::ffi::c_int;
    pub fn nilfs_bmap_clear(bmap: *mut nilfs_bmap);
    pub fn nilfs_bmap_propagate(bmap: *mut nilfs_bmap, bh: *mut buffer_head) -> ::core::ffi::c_int;
    pub fn nilfs_bmap_lookup_dirty_buffers(bmap: *mut nilfs_bmap, list: *mut list_head);
    pub fn nilfs_bmap_assign(bmap: *mut nilfs_bmap, bh: *mut *mut buffer_head, blocknr: usize, binfo: *mut nilfs_binfo) -> ::core::ffi::c_int;
    pub fn nilfs_bmap_lookup_at_level(bmap: *mut nilfs_bmap, key: u64, level: ::core::ffi::c_int, ptr: *mut u64) -> ::core::ffi::c_int;
    pub fn nilfs_bmap_mark(bmap: *mut nilfs_bmap, key: u64, level: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn nilfs_bmap_init_gc(bmap: *mut nilfs_bmap);
    pub fn nilfs_bmap_save(bmap: *const nilfs_bmap, store: *mut nilfs_bmap_store);
    pub fn nilfs_bmap_restore(bmap: *mut nilfs_bmap, store: *const nilfs_bmap_store);
    pub fn nilfs_bmap_get_dat(bmap: *const nilfs_bmap) -> *mut inode;
    pub fn nilfs_bmap_data_get_key(bmap: *const nilfs_bmap, bh: *const buffer_head) -> u64;
    pub fn nilfs_bmap_find_target_seq(bmap: *const nilfs_bmap, key: u64) -> u64;
    pub fn nilfs_bmap_find_target_in_group(bmap: *const nilfs_bmap) -> u64;
}

#[inline]
pub unsafe fn nilfs_bmap_lookup(bmap: *mut nilfs_bmap, key: u64, ptr: *mut u64) -> ::core::ffi::c_int {
    nilfs_bmap_lookup_at_level(bmap, key, 1, ptr)
}

#[inline]
pub unsafe fn nilfs_bmap_prepare_alloc_ptr(bmap: *mut nilfs_bmap, req: *mut nilfs_bmap_ptr_req, dat: *mut inode) -> ::core::ffi::c_int {
    if !dat.is_null() { nilfs_dat_prepare_alloc(dat, &mut (*req).bpr_req) } else { (*req).bpr_ptr = (*bmap).b_last_allocated_ptr; (*bmap).b_last_allocated_ptr = (*bmap).b_last_allocated_ptr.wrapping_add(1); 0 }
}

#[inline]
pub unsafe fn nilfs_bmap_commit_alloc_ptr(_bmap: *mut nilfs_bmap, req: *mut nilfs_bmap_ptr_req, dat: *mut inode) { if !dat.is_null() { nilfs_dat_commit_alloc(dat, &mut (*req).bpr_req); } }

#[inline]
pub unsafe fn nilfs_bmap_abort_alloc_ptr(bmap: *mut nilfs_bmap, req: *mut nilfs_bmap_ptr_req, dat: *mut inode) { if !dat.is_null() { nilfs_dat_abort_alloc(dat, &mut (*req).bpr_req); } else { (*bmap).b_last_allocated_ptr = (*bmap).b_last_allocated_ptr.wrapping_sub(1); } }

#[inline]
pub unsafe fn nilfs_bmap_prepare_end_ptr(_bmap: *mut nilfs_bmap, req: *mut nilfs_bmap_ptr_req, dat: *mut inode) -> ::core::ffi::c_int { if !dat.is_null() { nilfs_dat_prepare_end(dat, &mut (*req).bpr_req) } else { 0 } }

#[inline]
pub unsafe fn nilfs_bmap_commit_end_ptr(bmap: *mut nilfs_bmap, req: *mut nilfs_bmap_ptr_req, dat: *mut inode) { if !dat.is_null() { nilfs_dat_commit_end(dat, &mut (*req).bpr_req, (*bmap).b_ptr_type == NILFS_BMAP_PTR_VS); } }

#[inline]
pub unsafe fn nilfs_bmap_abort_end_ptr(_bmap: *mut nilfs_bmap, req: *mut nilfs_bmap_ptr_req, dat: *mut inode) { if !dat.is_null() { nilfs_dat_abort_end(dat, &mut (*req).bpr_req); } }

#[inline]
pub unsafe fn nilfs_bmap_set_target_v(bmap: *mut nilfs_bmap, key: u64, ptr: u64) { (*bmap).b_last_allocated_key = key; (*bmap).b_last_allocated_ptr = ptr; }

#[inline]
pub unsafe fn nilfs_bmap_dirty(bmap: *const nilfs_bmap) -> ::core::ffi::c_int { (((*bmap).b_state & NILFS_BMAP_DIRTY) != 0) as ::core::ffi::c_int }
#[inline]
pub unsafe fn nilfs_bmap_set_dirty(bmap: *mut nilfs_bmap) { (*bmap).b_state |= NILFS_BMAP_DIRTY; }
#[inline]
pub unsafe fn nilfs_bmap_clear_dirty(bmap: *mut nilfs_bmap) { (*bmap).b_state &= !NILFS_BMAP_DIRTY; }

pub const NILFS_BMAP_LARGE: i32 = 0x1;
pub const NILFS_BMAP_SMALL_LOW: _ = NILFS_DIRECT_KEY_MIN;
pub const NILFS_BMAP_SMALL_HIGH: _ = NILFS_DIRECT_KEY_MAX;
pub const NILFS_BMAP_LARGE_LOW: _ = NILFS_BTREE_ROOT_NCHILDREN_MAX;
pub const NILFS_BMAP_LARGE_HIGH: _ = NILFS_BTREE_KEY_MAX;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
