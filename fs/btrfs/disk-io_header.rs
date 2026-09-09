/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2007 Oracle.  All rights reserved.
 */

/* C header dependencies: linux/sizes.h, linux/compiler_types.h, ctree.h,
 * bio.h, and ordered-data.h.  Their supplied Rust declarations are external.
 */

pub struct block_device;
pub struct super_block;
pub struct extent_buffer;
pub struct btrfs_eb_prealloc;
pub struct btrfs_device;
pub struct btrfs_fs_devices;
pub struct btrfs_fs_info;
pub struct btrfs_super_block;
pub struct btrfs_trans_handle;
pub struct btrfs_tree_parent_check;
pub struct btrfs_transaction;
pub struct btrfs_root;
pub struct btrfs_key;
pub struct btrfs_path;
pub struct btrfs_bio;

pub const BTRFS_SUPER_MIRROR_MAX: i32 = 3;
pub const BTRFS_SUPER_MIRROR_SHIFT: i32 = 12;

/*
 * Fixed blocksize for all devices, applies to specific ways of reading
 * metadata like superblock. Must meet the set_blocksize requirements.
 *
 * Do not change.
 */
pub const BTRFS_BDEV_BLOCKSIZE: u64 = 4096;

#[inline]
pub fn btrfs_sb_offset(mirror: i32) -> u64 {
    let start: u64 = SZ_16K;
    if mirror != 0 {
        return start << (BTRFS_SUPER_MIRROR_SHIFT * mirror);
    }
    BTRFS_SUPER_INFO_OFFSET
}

extern "C" {
    pub fn btrfs_check_leaked_roots(fs_info: *const btrfs_fs_info);
    pub fn btrfs_init_fs_info(fs_info: *mut btrfs_fs_info);
    pub fn read_tree_block(fs_info: *mut btrfs_fs_info, bytenr: u64,
                           check: *mut btrfs_tree_parent_check) -> *mut extent_buffer;
    pub fn btrfs_find_create_tree_block(fs_info: *mut btrfs_fs_info,
                                        pa: *mut btrfs_eb_prealloc,
                                        bytenr: u64, owner_root: u64,
                                        level: i32) -> *mut extent_buffer;
    pub fn btrfs_start_pre_rw_mount(fs_info: *mut btrfs_fs_info) -> i32;
    pub fn btrfs_check_super_csum(fs_info: *mut btrfs_fs_info,
                                  disk_sb: *const btrfs_super_block) -> i32;
    pub fn open_ctree(sb: *mut super_block, fs_devices: *mut btrfs_fs_devices) -> i32;
    pub fn close_ctree(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_validate_super(fs_info: *const btrfs_fs_info,
                                sb: *const btrfs_super_block,
                                mirror_num: i32) -> i32;
    pub fn btrfs_check_features(fs_info: *mut btrfs_fs_info, is_rw_mount: bool) -> i32;
    pub fn write_all_supers(trans: *mut btrfs_trans_handle) -> i32;
    pub fn btrfs_commit_super(fs_info: *mut btrfs_fs_info) -> i32;
    pub fn btrfs_read_tree_root(tree_root: *mut btrfs_root,
                                key: *const btrfs_key) -> *mut btrfs_root;
    pub fn btrfs_insert_fs_root(fs_info: *mut btrfs_fs_info,
                                root: *mut btrfs_root) -> i32;
    pub fn btrfs_free_fs_roots(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_get_fs_root(fs_info: *mut btrfs_fs_info, objectid: u64,
                             check_ref: bool) -> *mut btrfs_root;
    pub fn btrfs_get_new_fs_root(fs_info: *mut btrfs_fs_info, objectid: u64,
                                 anon_dev: *mut dev_t) -> *mut btrfs_root;
    pub fn btrfs_get_fs_root_commit_root(fs_info: *mut btrfs_fs_info,
                                         path: *mut btrfs_path,
                                         objectid: u64) -> *mut btrfs_root;
    pub fn btrfs_global_root_insert(root: *mut btrfs_root) -> i32;
    pub fn btrfs_global_root_delete(root: *mut btrfs_root);
    pub fn btrfs_global_root(fs_info: *mut btrfs_fs_info,
                             key: *const btrfs_key) -> *mut btrfs_root;
    pub fn btrfs_csum_root(fs_info: *mut btrfs_fs_info, bytenr: u64) -> *mut btrfs_root;
    pub fn btrfs_extent_root(fs_info: *mut btrfs_fs_info, bytenr: u64) -> *mut btrfs_root;
    pub fn btrfs_free_fs_info(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_btree_balance_dirty(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_btree_balance_dirty_nodelay(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_drop_and_free_fs_root(fs_info: *mut btrfs_fs_info,
                                       root: *mut btrfs_root);
    pub fn btrfs_validate_extent_buffer(eb: *mut extent_buffer,
                                        check: *const btrfs_tree_parent_check) -> i32;
    pub fn btrfs_put_root(root: *mut btrfs_root);
    pub fn btrfs_mark_buffer_dirty(trans: *mut btrfs_trans_handle,
                                   buf: *mut extent_buffer);
    pub fn btrfs_buffer_uptodate(buf: *mut extent_buffer, parent_transid: u64,
                                 check: *const btrfs_tree_parent_check) -> i32;
    pub fn btrfs_read_extent_buffer(buf: *mut extent_buffer,
                                    check: *const btrfs_tree_parent_check) -> i32;
    pub fn btree_csum_one_bio(bbio: *mut btrfs_bio) -> i32;
    pub fn btrfs_alloc_log_tree_node(trans: *mut btrfs_trans_handle,
                                     root: *mut btrfs_root) -> i32;
    pub fn btrfs_init_log_root_tree(trans: *mut btrfs_trans_handle,
                                    fs_info: *mut btrfs_fs_info) -> i32;
    pub fn btrfs_add_log_tree(trans: *mut btrfs_trans_handle,
                              root: *mut btrfs_root) -> i32;
    pub fn btrfs_cleanup_dirty_bgs(trans: *mut btrfs_transaction,
                                    fs_info: *mut btrfs_fs_info);
    pub fn btrfs_cleanup_one_transaction(trans: *mut btrfs_transaction);
    pub fn btrfs_create_tree(trans: *mut btrfs_trans_handle,
                             objectid: u64) -> *mut btrfs_root;
    pub fn btrfs_get_num_tolerated_disk_barrier_failures(flags: u64) -> i32;
    pub fn btrfs_get_free_objectid(root: *mut btrfs_root, objectid: *mut u64) -> i32;
    pub fn btrfs_init_root_free_objectid(root: *mut btrfs_root) -> i32;
}

#[inline]
pub unsafe fn btrfs_grab_root(root: *mut btrfs_root) -> *mut btrfs_root {
    if root.is_null() {
        return core::ptr::null_mut();
    }
    if refcount_inc_not_zero(&mut (*root).refs) {
        return root;
    }
    core::ptr::null_mut()
}

#[cfg(CONFIG_BTRFS_FS_RUN_SANITY_TESTS)]
extern "C" {
    pub fn btrfs_alloc_dummy_root(fs_info: *mut btrfs_fs_info) -> *mut btrfs_root;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
