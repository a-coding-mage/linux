/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2007 Oracle. All rights reserved. */

// Dependencies supplied by the surrounding kernel translation.

pub const BTRFS_TRANS_DIO_WRITE_STUB: *mut core::ffi::c_void = 1 as *mut core::ffi::c_void;
pub const BTRFS_ROOT_TRANS_TAG: u32 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum btrfs_trans_state {
    TRANS_STATE_RUNNING,
    TRANS_STATE_COMMIT_PREP,
    TRANS_STATE_COMMIT_START,
    TRANS_STATE_COMMIT_DOING,
    TRANS_STATE_UNBLOCKED,
    TRANS_STATE_SUPER_COMMITTED,
    TRANS_STATE_COMPLETED,
    TRANS_STATE_MAX,
}

pub const BTRFS_TRANS_HAVE_FREE_BGS: u32 = 0;
pub const BTRFS_TRANS_DIRTY_BG_RUN: u32 = 1;
pub const BTRFS_TRANS_CACHE_ENOSPC: u32 = 2;

#[repr(C)]
pub struct btrfs_transaction {
    pub transid: u64,
    pub num_extwriters: atomic_t,
    pub num_writers: atomic_t,
    pub use_count: refcount_t,
    pub flags: core::ffi::c_ulong,
    pub state: btrfs_trans_state,
    pub aborted: core::ffi::c_int,
    pub list: list_head,
    pub dirty_pages: extent_io_tree,
    pub start_time: time64_t,
    pub writer_wait: wait_queue_head_t,
    pub commit_wait: wait_queue_head_t,
    pub pending_snapshots: list_head,
    pub dev_update_list: list_head,
    pub switch_commits: list_head,
    pub dirty_bgs: list_head,
    pub io_bgs: list_head,
    pub dropped_roots: list_head,
    pub pinned_extents: extent_io_tree,
    pub cache_write_mutex: mutex,
    pub dirty_bgs_lock: spinlock_t,
    pub deleted_bgs: list_head,
    pub dropped_roots_lock: spinlock_t,
    pub delayed_refs: btrfs_delayed_ref_root,
    pub fs_info: *mut btrfs_fs_info,
    pub pending_ordered: atomic_t,
    pub pending_wait: wait_queue_head_t,
}

pub const __TRANS_FREEZABLE: u32 = 0;
pub const __TRANS_START: u32 = 1;
pub const __TRANS_ATTACH: u32 = 2;
pub const __TRANS_JOIN: u32 = 3;
pub const __TRANS_JOIN_NOLOCK: u32 = 4;
pub const __TRANS_DUMMY: u32 = 5;
pub const __TRANS_JOIN_NOSTART: u32 = 6;
pub const TRANS_START: u32 = __TRANS_START | __TRANS_FREEZABLE;
pub const TRANS_ATTACH: u32 = __TRANS_ATTACH;
pub const TRANS_JOIN: u32 = __TRANS_JOIN | __TRANS_FREEZABLE;
pub const TRANS_JOIN_NOLOCK: u32 = __TRANS_JOIN_NOLOCK;
pub const TRANS_JOIN_NOSTART: u32 = __TRANS_JOIN_NOSTART;
pub const TRANS_EXTWRITERS: u32 = __TRANS_START | __TRANS_ATTACH;

pub const BTRFS_INHIBITED_EBS_SLOTS: usize = 8;

#[repr(C)]
pub struct btrfs_trans_handle {
    pub transid: u64,
    pub bytes_reserved: u64,
    pub delayed_refs_bytes_reserved: u64,
    pub chunk_bytes_reserved: u64,
    pub delayed_ref_updates: core::ffi::c_ulong,
    pub delayed_ref_csum_deletions: core::ffi::c_ulong,
    pub transaction: *mut btrfs_transaction,
    pub block_rsv: *mut btrfs_block_rsv,
    pub orig_rsv: *mut btrfs_block_rsv,
    pub pending_snapshot: *mut btrfs_pending_snapshot,
    pub use_count: refcount_t,
    pub type_: core::ffi::c_uint,
    pub aborted: i16,
    pub adding_csums: bool,
    pub allocating_chunk: bool,
    pub removing_chunk: bool,
    pub reloc_reserved: bool,
    pub in_fsync: bool,
    pub fs_info: *mut btrfs_fs_info,
    pub new_bgs: list_head,
    pub delayed_rsv: btrfs_block_rsv,
    pub inhibited_ebs: [*mut extent_buffer; BTRFS_INHIBITED_EBS_SLOTS],
    pub inhibited_ebs_referenced: u32,
    pub nr_inhibited_ebs: u32,
    pub inhibited_ebs_hand: u32,
}

#[repr(C)]
pub struct btrfs_pending_snapshot {
    pub dentry: *mut dentry,
    pub dir: *mut btrfs_inode,
    pub root: *mut btrfs_root,
    pub root_item: *mut btrfs_root_item,
    pub snap: *mut btrfs_root,
    pub inherit: *mut btrfs_qgroup_inherit,
    pub path: *mut btrfs_path,
    pub block_rsv: btrfs_block_rsv,
    pub error: core::ffi::c_int,
    pub anon_dev: dev_t,
    pub readonly: bool,
    pub list: list_head,
}

pub unsafe fn btrfs_set_inode_last_trans(trans: *mut btrfs_trans_handle, inode: *mut btrfs_inode) {
    spin_lock(&mut (*inode).lock);
    (*inode).last_trans = (*(*trans).transaction).transid;
    (*inode).last_sub_trans = btrfs_get_root_log_transid((*inode).root);
    (*inode).last_log_commit = (*inode).last_sub_trans.wrapping_sub(1);
    spin_unlock(&mut (*inode).lock);
}

pub unsafe fn btrfs_set_skip_qgroup(trans: *mut btrfs_trans_handle, qgroupid: u64) {
    let delayed_refs = &mut (*(*trans).transaction).delayed_refs;
    WARN_ON(delayed_refs.qgroup_to_skip != 0);
    delayed_refs.qgroup_to_skip = qgroupid;
}

pub unsafe fn btrfs_clear_skip_qgroup(trans: *mut btrfs_trans_handle) {
    let delayed_refs = &mut (*(*trans).transaction).delayed_refs;
    WARN_ON(delayed_refs.qgroup_to_skip == 0);
    delayed_refs.qgroup_to_skip = 0;
}

pub const fn btrfs_abort_should_print_stack(error: i32) -> bool {
    !matches!(error, -EIO | -EROFS | -ENOMEM)
}

extern "C" {
    pub fn btrfs_end_transaction(trans: *mut btrfs_trans_handle) -> i32;
    pub fn btrfs_start_transaction(root: *mut btrfs_root, num_items: u32) -> *mut btrfs_trans_handle;
    pub fn btrfs_start_transaction_fallback_global_rsv(root: *mut btrfs_root, num_items: u32) -> *mut btrfs_trans_handle;
    pub fn btrfs_join_transaction(root: *mut btrfs_root) -> *mut btrfs_trans_handle;
    pub fn btrfs_join_transaction_spacecache(root: *mut btrfs_root) -> *mut btrfs_trans_handle;
    pub fn btrfs_join_transaction_nostart(root: *mut btrfs_root) -> *mut btrfs_trans_handle;
    pub fn btrfs_attach_transaction(root: *mut btrfs_root) -> *mut btrfs_trans_handle;
    pub fn btrfs_attach_transaction_barrier(root: *mut btrfs_root) -> *mut btrfs_trans_handle;
    pub fn btrfs_wait_for_commit(fs_info: *mut btrfs_fs_info, transid: u64) -> i32;
    pub fn btrfs_add_dead_root(root: *mut btrfs_root);
    pub fn btrfs_maybe_wake_unfinished_drop(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_clean_one_deleted_snapshot(fs_info: *mut btrfs_fs_info) -> i32;
    pub fn btrfs_commit_transaction(trans: *mut btrfs_trans_handle) -> i32;
    pub fn btrfs_commit_transaction_async(trans: *mut btrfs_trans_handle);
    pub fn btrfs_commit_current_transaction(root: *mut btrfs_root) -> i32;
    pub fn btrfs_end_transaction_throttle(trans: *mut btrfs_trans_handle) -> i32;
    pub fn btrfs_should_end_transaction(trans: *mut btrfs_trans_handle) -> bool;
    pub fn btrfs_throttle(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_record_root_in_trans(trans: *mut btrfs_trans_handle, root: *mut btrfs_root) -> i32;
    pub fn btrfs_write_marked_extents(fs_info: *mut btrfs_fs_info, dirty_pages: *mut extent_io_tree, mark: i32) -> i32;
    pub fn btrfs_wait_tree_log_extents(root: *mut btrfs_root, mark: i32) -> i32;
    pub fn btrfs_transaction_blocked(info: *mut btrfs_fs_info) -> i32;
    pub fn btrfs_put_transaction(transaction: *mut btrfs_transaction);
    pub fn btrfs_add_dropped_root(trans: *mut btrfs_trans_handle, root: *mut btrfs_root);
    pub fn btrfs_trans_release_chunk_metadata(trans: *mut btrfs_trans_handle);
    pub fn __btrfs_abort_transaction(trans: *mut btrfs_trans_handle, function: *const core::ffi::c_char, line: u32, error: i32);
    pub fn btrfs_transaction_init() -> i32;
    pub fn btrfs_transaction_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
