/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2008 Oracle.  All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/list.h, linux/fs.h, linux/fscrypt.h, and transaction.h.

pub enum btrfs_log_mode {
    /* Log everything about an inode. */
    LOG_INODE_ALL,
    /* Log just enough to recreate the inode during log replay. */
    LOG_INODE_EXISTS,
}

/* Opaque types declared by the surrounding translation. */
pub struct inode;
pub struct dentry;
pub struct btrfs_ordered_extent;
pub struct btrfs_root;
pub struct btrfs_trans_handle;

/* return value for btrfs_log_dentry_safe that means we don't need to log it at all */
pub const BTRFS_NO_LOG_SYNC: i32 = 256;

/*
 * We can't use the tree log for whatever reason, force a transaction commit.
 * We use a negative value because there are functions through the logging code
 * that need to return an error (< 0 value), false (0) or true (1). Any negative
 * value will do, as it will cause the log to be marked for a full sync.
 */
pub const BTRFS_LOG_FORCE_COMMIT: i32 = -(MAX_ERRNO as i32 + 1);

#[repr(C)]
pub struct btrfs_log_ctx {
    pub log_ret: i32,
    pub log_transid: i32,
    pub log_new_dentries: bool,
    pub logging_new_name: bool,
    pub logging_new_delayed_dentries: bool,
    /* Indicate if the inode being logged was logged before. */
    pub logged_before: bool,
    pub inode: *mut btrfs_inode,
    pub list: list_head,
    /* Only used for fast fsyncs. */
    pub ordered_extents: list_head,
    pub conflict_inodes: list_head,
    pub num_conflict_inodes: i32,
    pub logging_conflict_inodes: bool,
    /*
     * Used for fsyncs that need to copy items from the subvolume tree to
     * the log tree (full sync flag set or copy everything flag set) to
     * avoid allocating a temporary extent buffer while holding a lock on
     * an extent buffer of the subvolume tree and under the log transaction.
     * Also helps to avoid allocating and freeing a temporary extent buffer
     * in case we need to process multiple leaves from the subvolume tree.
     */
    pub scratch_eb: *mut extent_buffer,
}

pub fn btrfs_init_log_ctx(ctx: *mut btrfs_log_ctx, inode: *mut btrfs_inode);
pub fn btrfs_init_log_ctx_scratch_eb(ctx: *mut btrfs_log_ctx);
pub fn btrfs_release_log_ctx_extents(ctx: *mut btrfs_log_ctx);

pub unsafe fn btrfs_set_log_full_commit(trans: *mut btrfs_trans_handle) {
    // WRITE_ONCE(trans->fs_info->last_trans_log_full_commit, trans->transid);
    core::ptr::write_volatile(
        &mut (*(*trans).fs_info).last_trans_log_full_commit,
        (*trans).transid,
    );
}

pub unsafe fn btrfs_need_log_full_commit(trans: *mut btrfs_trans_handle) -> i32 {
    // READ_ONCE(trans->fs_info->last_trans_log_full_commit) == trans->transid;
    (core::ptr::read_volatile(&(*(*trans).fs_info).last_trans_log_full_commit)
        == (*trans).transid) as i32
}

pub fn btrfs_sync_log(
    trans: *mut btrfs_trans_handle,
    root: *mut btrfs_root,
    ctx: *mut btrfs_log_ctx,
) -> i32;
pub fn btrfs_free_log(trans: *mut btrfs_trans_handle, root: *mut btrfs_root);
pub fn btrfs_free_log_root_tree(
    trans: *mut btrfs_trans_handle,
    fs_info: *mut btrfs_fs_info,
);
pub fn btrfs_recover_log_trees(tree_root: *mut btrfs_root) -> i32;
pub fn btrfs_log_dentry_safe(
    trans: *mut btrfs_trans_handle,
    dentry: *mut dentry,
    ctx: *mut btrfs_log_ctx,
) -> i32;
pub fn btrfs_del_dir_entries_in_log(
    trans: *mut btrfs_trans_handle,
    name: *const fscrypt_str,
    dir: *mut btrfs_inode,
    index: u64,
);
pub fn btrfs_del_inode_ref_in_log(
    trans: *mut btrfs_trans_handle,
    name: *const fscrypt_str,
    inode: *mut btrfs_inode,
    dir: *mut btrfs_inode,
);
pub fn btrfs_end_log_trans(root: *mut btrfs_root);
pub fn btrfs_pin_log_trans(root: *mut btrfs_root);
pub fn btrfs_record_unlink_dir(
    trans: *mut btrfs_trans_handle,
    dir: *mut btrfs_inode,
    inode: *mut btrfs_inode,
    for_rename: bool,
);
pub fn btrfs_record_snapshot_destroy(
    trans: *mut btrfs_trans_handle,
    dir: *mut btrfs_inode,
);
pub fn btrfs_record_new_subvolume(
    trans: *const btrfs_trans_handle,
    dir: *mut btrfs_inode,
);
pub fn btrfs_log_new_name(
    trans: *mut btrfs_trans_handle,
    old_dentry: *mut dentry,
    old_dir: *mut btrfs_inode,
    old_dir_index: u64,
    parent: *mut dentry,
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
