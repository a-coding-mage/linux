/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2011 Fujitsu.  All rights reserved.
 * Written by Miao Xie <miaox@cn.fujitsu.com>
 */

// C includes: linux/types.h, linux/rbtree.h, linux/spinlock.h,
// linux/mutex.h, linux/list.h, linux/wait.h, linux/fs.h, linux/atomic.h,
// linux/refcount.h, linux/ref_tracker.h, and "ctree.h".

pub enum btrfs_delayed_item_type {
    BTRFS_DELAYED_INSERTION_ITEM,
    BTRFS_DELAYED_DELETION_ITEM,
}

#[repr(C)]
pub struct btrfs_ref_tracker_dir {
    #[cfg(CONFIG_BTRFS_DEBUG)]
    pub dir: ref_tracker_dir,
    #[cfg(not(CONFIG_BTRFS_DEBUG))]
    pub tracker: (),
}

#[repr(C)]
pub struct btrfs_ref_tracker {
    #[cfg(CONFIG_BTRFS_DEBUG)]
    pub tracker: *mut ref_tracker,
    #[cfg(not(CONFIG_BTRFS_DEBUG))]
    pub tracker: (),
}

pub const BTRFS_DELAYED_NODE_IN_LIST: u32 = 0;
pub const BTRFS_DELAYED_NODE_INODE_DIRTY: u32 = 1;
pub const BTRFS_DELAYED_NODE_DEL_IREF: u32 = 2;

#[repr(C)]
pub struct btrfs_delayed_node {
    pub inode_id: u64,
    pub bytes_reserved: u64,
    pub root: *mut btrfs_root,
    /* Used to add the node into the delayed root's node list. */
    pub n_list: list_head,
    /*
     * Used to add the node into the prepare list, the nodes in this list
     * is waiting to be dealt with by the async worker.
     */
    pub p_list: list_head,
    pub ins_root: rb_root_cached,
    pub del_root: rb_root_cached,
    pub mutex: mutex,
    pub inode_item: btrfs_inode_item,
    pub refs: refcount_t,
    pub count: i32,
    pub index_cnt: u64,
    pub flags: libc::c_ulong,
    /*
     * The size of the next batch of dir index items to insert (if this
     * node is from a directory inode). Protected by @mutex.
     */
    pub curr_index_batch_size: u32,
    /*
     * Number of leaves reserved for inserting dir index items (if this
     * node belongs to a directory inode). This may be larger then the
     * actual number of leaves we end up using. Protected by @mutex.
     */
    pub index_item_leaves: u32,
    /* Track all references to this delayed node. */
    pub ref_dir: btrfs_ref_tracker_dir,
    /* Track delayed node reference stored in node list. */
    pub node_list_tracker: btrfs_ref_tracker,
    /* Track delayed node reference stored in inode cache. */
    pub inode_cache_tracker: btrfs_ref_tracker,
}

#[repr(C)]
pub struct btrfs_delayed_item {
    pub rb_node: rb_node,
    /* Offset value of the corresponding dir index key. */
    pub index: u64,
    pub tree_list: list_head, // used for batch insert/delete items
    pub readdir_list: list_head, // used for readdir items
    /*
     * Used when logging a directory.
     * Insertions and deletions to this list are protected by the parent
     * delayed node's mutex.
     */
    pub log_list: list_head,
    pub bytes_reserved: u64,
    pub delayed_node: *mut btrfs_delayed_node,
    pub refs: refcount_t,
    pub type_: btrfs_delayed_item_type,
    /*
     * Track if this delayed item was already logged.
     * Protected by the mutex of the parent delayed inode.
     */
    pub logged: bool,
    /* The maximum leaf size is 64K, so u16 is more than enough. */
    pub data_len: u16,
    pub data: [libc::c_char; 0],
}

unsafe extern "C" {
    pub fn btrfs_init_delayed_root(delayed_root: *mut btrfs_delayed_root);
    pub fn btrfs_insert_delayed_dir_index(trans: *mut btrfs_trans_handle, name: *const libc::c_char, name_len: i32, dir: *mut btrfs_inode, disk_key: *const btrfs_disk_key, flags: u8, index: u64) -> i32;
    pub fn btrfs_delete_delayed_dir_index(trans: *mut btrfs_trans_handle, dir: *mut btrfs_inode, index: u64) -> i32;
    pub fn btrfs_inode_delayed_dir_index_count(inode: *mut btrfs_inode) -> i32;
    pub fn btrfs_run_delayed_items(trans: *mut btrfs_trans_handle) -> i32;
    pub fn btrfs_run_delayed_items_nr(trans: *mut btrfs_trans_handle, nr: i32) -> i32;
    pub fn btrfs_balance_delayed_items(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_commit_inode_delayed_items(trans: *mut btrfs_trans_handle, inode: *mut btrfs_inode) -> i32;
    /* Used for evicting the inode. */
    pub fn btrfs_remove_delayed_node(inode: *mut btrfs_inode);
    pub fn btrfs_kill_delayed_inode_items(inode: *mut btrfs_inode);
    pub fn btrfs_commit_inode_delayed_inode(inode: *mut btrfs_inode) -> i32;
    pub fn btrfs_delayed_update_inode(trans: *mut btrfs_trans_handle, inode: *mut btrfs_inode) -> i32;
    pub fn btrfs_fill_inode(inode: *mut btrfs_inode, rdev: *mut u32) -> i32;
    pub fn btrfs_delayed_delete_inode_ref(inode: *mut btrfs_inode) -> i32;
    /* Used for drop dead root */
    pub fn btrfs_kill_all_delayed_nodes(root: *mut btrfs_root);
    /* Used for clean the transaction */
    pub fn btrfs_destroy_delayed_inodes(fs_info: *mut btrfs_fs_info);
    /* Used for readdir() */
    pub fn btrfs_readdir_get_delayed_items(inode: *mut btrfs_inode, last_index: u64, ins_list: *mut list_head, del_list: *mut list_head) -> bool;
    pub fn btrfs_readdir_put_delayed_items(inode: *mut btrfs_inode, ins_list: *mut list_head, del_list: *mut list_head);
    pub fn btrfs_should_delete_dir_index(del_list: *const list_head, index: u64) -> bool;
    pub fn btrfs_readdir_delayed_dir_index(ctx: *mut dir_context, ins_list: *const list_head) -> bool;
    /* Used during directory logging. */
    pub fn btrfs_log_get_delayed_items(inode: *mut btrfs_inode, ins_list: *mut list_head, del_list: *mut list_head);
    pub fn btrfs_log_put_delayed_items(inode: *mut btrfs_inode, ins_list: *mut list_head, del_list: *mut list_head);
    /* for init */
    pub fn btrfs_delayed_inode_init() -> i32;
    pub fn btrfs_delayed_inode_exit();
    /* for debugging */
    pub fn btrfs_assert_delayed_root_empty(fs_info: *mut btrfs_fs_info);
}

pub const BTRFS_DELAYED_NODE_REF_TRACKER_QUARANTINE_COUNT: u32 = 16;
pub const BTRFS_DELAYED_NODE_REF_TRACKER_DISPLAY_LIMIT: u32 = 16;

#[cfg(CONFIG_BTRFS_DEBUG)]
pub unsafe fn btrfs_delayed_node_ref_tracker_dir_init(node: *mut btrfs_delayed_node) {
    if !btrfs_test_opt((*(*node).root).fs_info, REF_TRACKER) { return; }
    ref_tracker_dir_init(&mut (*node).ref_dir.dir, BTRFS_DELAYED_NODE_REF_TRACKER_QUARANTINE_COUNT, b"delayed_node\0".as_ptr() as *const libc::c_char);
}

#[cfg(CONFIG_BTRFS_DEBUG)]
pub unsafe fn btrfs_delayed_node_ref_tracker_dir_exit(node: *mut btrfs_delayed_node) {
    if !btrfs_test_opt((*(*node).root).fs_info, REF_TRACKER) { return; }
    ref_tracker_dir_exit(&mut (*node).ref_dir.dir);
}

#[cfg(CONFIG_BTRFS_DEBUG)]
pub unsafe fn btrfs_delayed_node_ref_tracker_dir_print(node: *mut btrfs_delayed_node) {
    if !btrfs_test_opt((*(*node).root).fs_info, REF_TRACKER) { return; }
    /* Only print if there are leaked references. The caller is holding one reference, so if refs == 1 there is no leak. */
    if refcount_read(&(*node).refs) == 1 { return; }
    ref_tracker_dir_print(&mut (*node).ref_dir.dir, BTRFS_DELAYED_NODE_REF_TRACKER_DISPLAY_LIMIT);
}

#[cfg(CONFIG_BTRFS_DEBUG)]
pub unsafe fn btrfs_delayed_node_ref_tracker_alloc(node: *mut btrfs_delayed_node, tracker: *mut btrfs_ref_tracker, gfp: gfp_t) -> i32 {
    if !btrfs_test_opt((*(*node).root).fs_info, REF_TRACKER) { return 0; }
    ref_tracker_alloc(&mut (*node).ref_dir.dir, &mut (*tracker).tracker, gfp)
}

#[cfg(CONFIG_BTRFS_DEBUG)]
pub unsafe fn btrfs_delayed_node_ref_tracker_free(node: *mut btrfs_delayed_node, tracker: *mut btrfs_ref_tracker) -> i32 {
    if !btrfs_test_opt((*(*node).root).fs_info, REF_TRACKER) { return 0; }
    ref_tracker_free(&mut (*node).ref_dir.dir, &mut (*tracker).tracker)
}

#[cfg(not(CONFIG_BTRFS_DEBUG))]
pub unsafe fn btrfs_delayed_node_ref_tracker_dir_init(_node: *mut btrfs_delayed_node) {}
#[cfg(not(CONFIG_BTRFS_DEBUG))]
pub unsafe fn btrfs_delayed_node_ref_tracker_dir_exit(_node: *mut btrfs_delayed_node) {}
#[cfg(not(CONFIG_BTRFS_DEBUG))]
pub unsafe fn btrfs_delayed_node_ref_tracker_dir_print(_node: *mut btrfs_delayed_node) {}
#[cfg(not(CONFIG_BTRFS_DEBUG))]
pub unsafe fn btrfs_delayed_node_ref_tracker_alloc(_node: *mut btrfs_delayed_node, _tracker: *mut btrfs_ref_tracker, _gfp: gfp_t) -> i32 { 0 }
#[cfg(not(CONFIG_BTRFS_DEBUG))]
pub unsafe fn btrfs_delayed_node_ref_tracker_free(_node: *mut btrfs_delayed_node, _tracker: *mut btrfs_ref_tracker) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
