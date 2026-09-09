/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// block-group.h and locking.h

#[repr(C)]
pub struct extent_buffer;
#[repr(C)]
pub struct btrfs_free_cluster;
#[repr(C)]
pub struct btrfs_fs_info;
#[repr(C)]
pub struct btrfs_root;
#[repr(C)]
pub struct btrfs_path;
#[repr(C)]
pub struct btrfs_ref;
#[repr(C)]
pub struct btrfs_disk_key;
#[repr(C)]
pub struct btrfs_delayed_ref_head;
#[repr(C)]
pub struct btrfs_delayed_ref_root;
#[repr(C)]
pub struct btrfs_extent_inline_ref;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum btrfs_extent_allocation_policy {
    BTRFS_EXTENT_ALLOC_CLUSTERED,
    BTRFS_EXTENT_ALLOC_ZONED,
}

#[repr(C)]
pub struct find_free_extent_ctl {
    /* Basic allocation info */
    pub ram_bytes: u64,
    pub num_bytes: u64,
    pub min_alloc_size: u64,
    pub empty_size: u64,
    pub flags: u64,
    /* Where to start the search inside the bg */
    pub search_start: u64,
    /* For clustered allocation */
    pub empty_cluster: u64,
    pub last_ptr: *mut btrfs_free_cluster,
    pub use_cluster: bool,
    pub delalloc: bool,
    pub have_caching_bg: bool,
    pub orig_have_caching_bg: bool,
    /* Allocation is called for tree-log */
    pub for_treelog: bool,
    /* Allocation is called for data relocation */
    pub for_data_reloc: bool,
    /*
     * Set to true if we're retrying the allocation on this block group
     * after waiting for caching progress, this is so that we retry only
     * once before moving on to another block group.
     */
    pub retry_uncached: bool,
    /* Whether or not the allocator is currently following a hint. */
    pub hinted: bool,
    /* RAID index, converted from flags */
    pub index: i32,
    /*
     * Current loop number, check find_free_extent_update_loop() for details
     */
    pub loop_: i32,
    /* If current block group is cached */
    pub cached: i32,
    /* Max contiguous hole found */
    pub max_extent_size: u64,
    /* Total free space from free space cache, not always contiguous */
    pub total_free_space: u64,
    /* Found result */
    pub found_offset: u64,
    /* Hint where to start looking for an empty space */
    pub hint_byte: u64,
    /* Allocation policy */
    pub policy: btrfs_extent_allocation_policy,
    /* Size class of block groups to prefer in early loops */
    pub size_class: btrfs_block_group_size_class,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum btrfs_inline_ref_type {
    BTRFS_REF_TYPE_INVALID,
    BTRFS_REF_TYPE_BLOCK,
    BTRFS_REF_TYPE_DATA,
    BTRFS_REF_TYPE_ANY,
}

extern "C" {
    pub fn btrfs_get_extent_inline_ref_type(
        eb: *const extent_buffer,
        iref: *const btrfs_extent_inline_ref,
        is_data: btrfs_inline_ref_type,
    ) -> i32;
    pub fn hash_extent_data_ref(root_objectid: u64, owner: u64, offset: u64) -> u64;
    pub fn btrfs_run_delayed_refs(trans: *mut btrfs_trans_handle, min_bytes: u64) -> i32;
    pub fn btrfs_cleanup_ref_head_accounting(
        fs_info: *mut btrfs_fs_info,
        delayed_refs: *mut btrfs_delayed_ref_root,
        head: *mut btrfs_delayed_ref_head,
    ) -> u64;
    pub fn btrfs_lookup_data_extent(fs_info: *mut btrfs_fs_info, start: u64, len: u64) -> i32;
    pub fn btrfs_lookup_extent_info(
        trans: *mut btrfs_trans_handle, fs_info: *mut btrfs_fs_info, bytenr: u64,
        offset: u64, metadata: i32, refs: *mut u64, flags: *mut u64, owner_root: *mut u64,
    ) -> i32;
    pub fn btrfs_pin_extent(trans: *mut btrfs_trans_handle, bytenr: u64, num: u64) -> i32;
    pub fn btrfs_pin_extent_for_log_replay(trans: *mut btrfs_trans_handle, eb: *const extent_buffer) -> i32;
    pub fn btrfs_exclude_logged_extents(eb: *mut extent_buffer) -> i32;
    pub fn btrfs_cross_ref_exist(inode: *mut btrfs_inode, offset: u64, bytenr: u64, path: *mut btrfs_path) -> i32;
    pub fn btrfs_alloc_tree_block(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, parent: u64, root_objectid: u64, key: *const btrfs_disk_key, level: i32, hint: u64, empty_size: u64, reloc_src_root: u64, nest: btrfs_lock_nesting) -> *mut extent_buffer;
    pub fn btrfs_free_tree_block(trans: *mut btrfs_trans_handle, root_id: u64, buf: *mut extent_buffer, parent: u64, last_ref: i32) -> i32;
    pub fn btrfs_alloc_reserved_file_extent(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, owner: u64, offset: u64, ram_bytes: u64, ins: *mut btrfs_key) -> i32;
    pub fn btrfs_alloc_logged_file_extent(trans: *mut btrfs_trans_handle, root_objectid: u64, owner: u64, offset: u64, ins: *mut btrfs_key) -> i32;
    pub fn btrfs_reserve_extent(root: *mut btrfs_root, ram_bytes: u64, num_bytes: u64, min_alloc_size: u64, empty_size: u64, hint_byte: u64, ins: *mut btrfs_key, is_data: bool, delalloc: bool) -> i32;
    pub fn btrfs_inc_ref(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, buf: *mut extent_buffer, full_backref: bool) -> i32;
    pub fn btrfs_dec_ref(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, buf: *mut extent_buffer, full_backref: bool) -> i32;
    pub fn btrfs_set_disk_extent_flags(trans: *mut btrfs_trans_handle, eb: *mut extent_buffer, flags: u64) -> i32;
    pub fn btrfs_free_extent(trans: *mut btrfs_trans_handle, ref_: *mut btrfs_ref) -> i32;
    pub fn btrfs_get_extent_owner_root(fs_info: *mut btrfs_fs_info, leaf: *mut extent_buffer, slot: i32) -> u64;
    pub fn btrfs_free_reserved_extent(fs_info: *mut btrfs_fs_info, start: u64, len: u64, is_delalloc: bool) -> i32;
    pub fn btrfs_pin_reserved_extent(trans: *mut btrfs_trans_handle, eb: *const extent_buffer) -> i32;
    pub fn btrfs_finish_extent_commit(trans: *mut btrfs_trans_handle) -> i32;
    pub fn btrfs_inc_extent_ref(trans: *mut btrfs_trans_handle, generic_ref: *mut btrfs_ref) -> i32;
    pub fn btrfs_drop_snapshot(root: *mut btrfs_root, update_ref: bool, for_reloc: bool) -> i32;
    pub fn btrfs_drop_subtree(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, node: *mut extent_buffer, parent: *mut extent_buffer) -> i32;
    pub fn btrfs_error_unpin_extent_range(fs_info: *mut btrfs_fs_info, start: u64, end: u64);
    pub fn btrfs_discard_extent(fs_info: *mut btrfs_fs_info, bytenr: u64, num_bytes: u64, actual_bytes: *mut u64, do_remap: bool) -> i32;
    pub fn btrfs_trim_fs(fs_info: *mut btrfs_fs_info, range: *mut fstrim_range) -> i32;
    pub fn btrfs_handle_fully_remapped_bgs(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_complete_bg_remapping(bg: *mut btrfs_block_group) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
