/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * NILFS meta data file prototype and definitions
 *
 * Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Written by Ryusuke Konishi.
 */

// Dependencies supplied by the surrounding translation unit.

/**
 * struct nilfs_shadow_map - shadow mapping of meta data file
 * @bmap_store: shadow copy of bmap state
 * @inode: holder of page caches used in shadow mapping
 * @frozen_buffers: list of frozen buffers
 */
#[repr(C)]
pub struct nilfs_shadow_map {
    pub bmap_store: nilfs_bmap_store,
    pub inode: *mut inode,
    pub frozen_buffers: list_head,
}

/**
 * struct nilfs_mdt_info - on-memory private data of meta data files
 * @mi_sem: reader/writer semaphore for meta data operations
 * @mi_bgl: per-blockgroup locking
 * @mi_entry_size: size of an entry
 * @mi_first_entry_offset: offset to the first entry
 * @mi_entries_per_block: number of entries in a block
 * @mi_palloc_cache: persistent object allocator cache
 * @mi_shadow: shadow of bmap and page caches
 * @mi_blocks_per_group: number of blocks in a group
 * @mi_blocks_per_desc_block: number of blocks per descriptor block
 */
#[repr(C)]
pub struct nilfs_mdt_info {
    pub mi_sem: rw_semaphore,
    pub mi_bgl: *mut blockgroup_lock,
    pub mi_entry_size: ::core::ffi::c_uint,
    pub mi_first_entry_offset: ::core::ffi::c_uint,
    pub mi_entries_per_block: ::core::ffi::c_ulong,
    pub mi_palloc_cache: *mut nilfs_palloc_cache,
    pub mi_shadow: *mut nilfs_shadow_map,
    pub mi_blocks_per_group: ::core::ffi::c_ulong,
    pub mi_blocks_per_desc_block: ::core::ffi::c_ulong,
}

#[inline]
pub unsafe fn NILFS_MDT(inode: *const inode) -> *mut nilfs_mdt_info {
    (*inode).i_private as *mut nilfs_mdt_info
}

#[inline]
pub unsafe fn nilfs_is_metadata_file_inode(inode: *const inode) -> ::core::ffi::c_int {
    if !(*inode).i_private.is_null() { 1 } else { 0 }
}

/* Default GFP flags using highmem */
pub const NILFS_MDT_GFP: ::core::ffi::c_uint = __GFP_RECLAIM | __GFP_IO | __GFP_HIGHMEM;

pub unsafe extern "C" fn nilfs_mdt_get_block(
    _: *mut inode,
    _: ::core::ffi::c_ulong,
    _: ::core::ffi::c_int,
    _: Option<unsafe extern "C" fn(*mut inode, *mut buffer_head, *mut ::core::ffi::c_void)>,
    _: *mut *mut buffer_head,
) -> ::core::ffi::c_int;
pub unsafe extern "C" fn nilfs_mdt_find_block(
    inode: *mut inode,
    start: ::core::ffi::c_ulong,
    end: ::core::ffi::c_ulong,
    blkoff: *mut ::core::ffi::c_ulong,
    out_bh: *mut *mut buffer_head,
) -> ::core::ffi::c_int;
pub unsafe extern "C" fn nilfs_mdt_delete_block(_: *mut inode, _: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
pub unsafe extern "C" fn nilfs_mdt_forget_block(_: *mut inode, _: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
pub unsafe extern "C" fn nilfs_mdt_fetch_dirty(_: *mut inode) -> ::core::ffi::c_int;
pub unsafe extern "C" fn nilfs_mdt_init(_: *mut inode, _: gfp_t, _: usize) -> ::core::ffi::c_int;
pub unsafe extern "C" fn nilfs_mdt_clear(_: *mut inode);
pub unsafe extern "C" fn nilfs_mdt_destroy(_: *mut inode);
pub unsafe extern "C" fn nilfs_mdt_set_entry_size(_: *mut inode, _: ::core::ffi::c_uint, _: ::core::ffi::c_uint);
pub unsafe extern "C" fn nilfs_mdt_setup_shadow_map(_: *mut inode, _: *mut nilfs_shadow_map) -> ::core::ffi::c_int;
pub unsafe extern "C" fn nilfs_mdt_save_to_shadow_map(_: *mut inode) -> ::core::ffi::c_int;
pub unsafe extern "C" fn nilfs_mdt_restore_from_shadow_map(_: *mut inode);
pub unsafe extern "C" fn nilfs_mdt_clear_shadow_map(_: *mut inode);
pub unsafe extern "C" fn nilfs_mdt_freeze_buffer(_: *mut inode, _: *mut buffer_head) -> ::core::ffi::c_int;
pub unsafe extern "C" fn nilfs_mdt_get_frozen_buffer(_: *mut inode, _: *mut buffer_head) -> *mut buffer_head;

#[inline]
pub unsafe fn nilfs_mdt_mark_dirty(inode: *mut inode) {
    if test_bit(NILFS_I_DIRTY, &mut (*NILFS_I(inode)).i_state) == 0 {
        set_bit(NILFS_I_DIRTY, &mut (*NILFS_I(inode)).i_state);
    }
}

#[inline]
pub unsafe fn nilfs_mdt_clear_dirty(inode: *mut inode) {
    clear_bit(NILFS_I_DIRTY, &mut (*NILFS_I(inode)).i_state);
}

#[inline]
pub unsafe fn nilfs_mdt_cno(inode: *mut inode) -> __u64 {
    (*( (*inode).i_sb ).s_fs_info as *mut the_nilfs).as_ref().unwrap().ns_cno
}

#[inline]
pub unsafe fn nilfs_mdt_bgl_lock(inode: *mut inode, block_group: ::core::ffi::c_uint) -> *mut spinlock_t {
    bgl_lock_ptr(NILFS_MDT(inode).as_ref().unwrap().mi_bgl, block_group)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
