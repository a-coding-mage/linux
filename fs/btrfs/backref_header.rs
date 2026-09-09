/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2011 STRATO.  All rights reserved.
 */

// Dependencies supplied by the surrounding translation unit provide the
// referenced Btrfs types.

pub const BTRFS_ITERATE_EXTENT_INODES_STOP: i32 = 5;

pub type IterateExtentInodesT = unsafe extern "C" fn(
    inum: u64,
    offset: u64,
    num_bytes: u64,
    root: u64,
    ctx: *mut core::ffi::c_void,
) -> i32;

#[repr(C)]
pub struct BtrfsBackrefWalkCtx {
    pub bytenr: u64,
    pub extent_item_pos: u64,
    pub ignore_extent_item_pos: bool,
    pub skip_inode_ref_list: bool,
    pub trans: *mut BtrfsTransHandle,
    pub fs_info: *mut BtrfsFsInfo,
    pub time_seq: u64,
    pub refs: *mut Ulist,
    pub roots: *mut Ulist,
    pub cache_lookup: Option<unsafe extern "C" fn(
        leaf_bytenr: u64,
        user_ctx: *mut core::ffi::c_void,
        root_ids_ret: *mut *const u64,
        root_count_ret: *mut i32,
    ) -> bool>,
    pub cache_store: Option<unsafe extern "C" fn(
        leaf_bytenr: u64,
        root_ids: *const Ulist,
        user_ctx: *mut core::ffi::c_void,
    )>,
    pub indirect_ref_iterator: Option<IterateExtentInodesT>,
    pub check_extent_item: Option<unsafe extern "C" fn(
        bytenr: u64,
        ei: *const BtrfsExtentItem,
        leaf: *const ExtentBuffer,
        user_ctx: *mut core::ffi::c_void,
    ) -> i32>,
    pub skip_data_ref: Option<unsafe extern "C" fn(
        root: u64,
        ino: u64,
        offset: u64,
        user_ctx: *mut core::ffi::c_void,
    ) -> bool>,
    pub user_ctx: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct InodeFsPaths {
    pub btrfs_path: *mut BtrfsPath,
    pub fs_root: *mut BtrfsRoot,
    pub fspath: *mut BtrfsDataContainer,
}

#[repr(C)]
pub struct BtrfsBackrefSharedCacheEntry {
    pub bytenr: u64,
    pub gen: u64,
    pub is_shared: bool,
}

pub const BTRFS_BACKREF_CTX_PREV_EXTENTS_SIZE: usize = 8;

#[repr(C)]
pub struct BtrfsBackrefShareCheckCtx {
    pub refs: Ulist,
    pub curr_leaf_bytenr: u64,
    pub prev_leaf_bytenr: u64,
    pub path_cache_entries: [BtrfsBackrefSharedCacheEntry; BTRFS_MAX_LEVEL as usize],
    pub use_path_cache: bool,
    pub prev_extents_cache: [PrevExtentCacheEntry; BTRFS_BACKREF_CTX_PREV_EXTENTS_SIZE],
    pub prev_extents_cache_slot: i32,
}

#[repr(C)]
pub struct PrevExtentCacheEntry {
    pub bytenr: u64,
    pub is_shared: bool,
}

pub struct ExtentInodeElem;
pub struct Ulist;
pub struct BtrfsExtentItem;
pub struct BtrfsTransHandle;
pub struct BtrfsFsInfo;
pub struct BtrfsPath;
pub struct BtrfsKey;
pub struct ExtentBuffer;
pub struct BtrfsRoot;
pub struct BtrfsDataContainer;

extern "C" {
    pub fn btrfs_alloc_backref_share_check_ctx() -> *mut BtrfsBackrefShareCheckCtx;
    pub fn btrfs_free_backref_share_ctx(ctx: *mut BtrfsBackrefShareCheckCtx);

    pub fn extent_from_logical(
        fs_info: *mut BtrfsFsInfo,
        logical: u64,
        path: *mut BtrfsPath,
        found_key: *mut BtrfsKey,
        flags: *mut u64,
    ) -> i32;

    pub fn tree_backref_for_extent(
        ptr: *mut usize,
        eb: *mut ExtentBuffer,
        key: *mut BtrfsKey,
        ei: *mut BtrfsExtentItem,
        item_size: u32,
        out_root: *mut u64,
        out_level: *mut u8,
    ) -> i32;

    pub fn iterate_extent_inodes(
        ctx: *mut BtrfsBackrefWalkCtx,
        search_commit_root: bool,
        iterate: Option<IterateExtentInodesT>,
        user_ctx: *mut core::ffi::c_void,
    ) -> i32;

    pub fn iterate_inodes_from_logical(
        logical: u64,
        fs_info: *mut BtrfsFsInfo,
        ctx: *mut core::ffi::c_void,
        ignore_offset: bool,
    ) -> i32;

    pub fn paths_from_inode(inum: u64, ipath: *mut InodeFsPaths) -> i32;
    pub fn btrfs_find_all_leafs(ctx: *mut BtrfsBackrefWalkCtx) -> i32;
    pub fn btrfs_find_all_roots(ctx: *mut BtrfsBackrefWalkCtx, skip_commit_root_sem: bool) -> i32;
    pub fn btrfs_ref_to_path(
        fs_root: *mut BtrfsRoot,
        path: *mut BtrfsPath,
        name_len: u32,
        name_off: usize,
        eb_in: *mut ExtentBuffer,
        parent: u64,
        dest: *mut core::ffi::c_char,
        size: u32,
    ) -> *mut core::ffi::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
