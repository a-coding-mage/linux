/* SPDX-License-Identifier: GPL-2.0 */

// The declarations below depend on the corresponding list and Btrfs types
// supplied by the surrounding translation unit.

#[repr(C)]
pub struct btrfs_seq_list {
    pub list: crate::list_head,
    pub seq: u64,
}

#[macro_export]
macro_rules! BTRFS_SEQ_LIST_INIT {
    ($name:expr) => {
        btrfs_seq_list {
            list: crate::LIST_HEAD_INIT!($name.list),
            seq: 0,
        }
    };
}

pub const BTRFS_SEQ_LAST: u64 = u64::MAX;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum btrfs_mod_log_op {
    BTRFS_MOD_LOG_KEY_REPLACE,
    BTRFS_MOD_LOG_KEY_ADD,
    BTRFS_MOD_LOG_KEY_REMOVE,
    BTRFS_MOD_LOG_KEY_REMOVE_WHILE_FREEING,
    BTRFS_MOD_LOG_KEY_REMOVE_WHILE_MOVING,
    BTRFS_MOD_LOG_MOVE_KEYS,
    BTRFS_MOD_LOG_ROOT_REPLACE,
}

unsafe extern "C" {
    pub fn btrfs_get_tree_mod_seq(
        fs_info: *mut crate::btrfs_fs_info,
        elem: *mut btrfs_seq_list,
    ) -> u64;
    pub fn btrfs_put_tree_mod_seq(
        fs_info: *mut crate::btrfs_fs_info,
        elem: *mut btrfs_seq_list,
    );
    pub fn btrfs_tree_mod_log_insert_root(
        old_root: *mut crate::extent_buffer,
        new_root: *mut crate::extent_buffer,
        log_removal: bool,
    ) -> i32;
    pub fn btrfs_tree_mod_log_insert_key(
        eb: *const crate::extent_buffer,
        slot: i32,
        op: btrfs_mod_log_op,
    ) -> i32;
    pub fn btrfs_tree_mod_log_free_eb(eb: *mut crate::extent_buffer) -> i32;
    pub fn btrfs_tree_mod_log_rewind(
        fs_info: *mut crate::btrfs_fs_info,
        eb: *mut crate::extent_buffer,
        time_seq: u64,
    ) -> *mut crate::extent_buffer;
    pub fn btrfs_get_old_root(
        root: *mut crate::btrfs_root,
        time_seq: u64,
    ) -> *mut crate::extent_buffer;
    pub fn btrfs_old_root_level(root: *mut crate::btrfs_root, time_seq: u64) -> i32;
    pub fn btrfs_tree_mod_log_eb_copy(
        dst: *mut crate::extent_buffer,
        src: *const crate::extent_buffer,
        dst_offset: usize,
        src_offset: usize,
        nr_items: i32,
    ) -> i32;
    pub fn btrfs_tree_mod_log_insert_move(
        eb: *const crate::extent_buffer,
        dst_slot: i32,
        src_slot: i32,
        nr_items: i32,
    ) -> i32;
    pub fn btrfs_tree_mod_log_lowest_seq(fs_info: *mut crate::btrfs_fs_info) -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
