/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2014 Facebook. All rights reserved. */

use core::ffi::c_int;

/* Types supplied by the surrounding kernel/Btrfs translation. */
#[repr(C)] pub struct extent_buffer { _private: [u8; 0] }
#[repr(C)] pub struct extent_changeset { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_delayed_extent_op { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_fs_info { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_root { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_ioctl_quota_ctl_args { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_trans_handle { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_delayed_ref_root { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_inode { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_transaction { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_block_group { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_qgroup_swapped_blocks { _private: [u8; 0] }
#[repr(C)] pub struct ulist { _private: [u8; 0] }
#[repr(C)] pub struct rb_node { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_key { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_qgroup_limit { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_qgroup_inherit { _private: [u8; 0] }

pub const BTRFS_QGROUP_RUNTIME_FLAG_CANCEL_RESCAN: u64 = 1u64 << 63;
pub const BTRFS_QGROUP_RUNTIME_FLAG_NO_ACCOUNTING: u64 = 1u64 << 62;
pub const BTRFS_QGROUP_DROP_SUBTREE_THRES_DEFAULT: u32 = 3;

#[repr(C)]
pub struct btrfs_qgroup_extent_record {
    pub num_bytes: u64,
    pub data_rsv: u32,
    pub data_rsv_refroot: u64,
    pub old_roots: *mut ulist,
}

#[repr(C)]
pub struct btrfs_qgroup_swapped_block {
    pub node: rb_node,
    pub level: c_int,
    pub trace_leaf: bool,
    pub subvol_bytenr: u64,
    pub subvol_generation: u64,
    pub reloc_bytenr: u64,
    pub reloc_generation: u64,
    pub last_snapshot: u64,
    pub first_key: btrfs_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum btrfs_qgroup_rsv_type {
    BTRFS_QGROUP_RSV_DATA,
    BTRFS_QGROUP_RSV_META_PERTRANS,
    BTRFS_QGROUP_RSV_META_PREALLOC,
    BTRFS_QGROUP_RSV_LAST,
}

#[repr(C)]
pub struct btrfs_qgroup_rsv {
    pub values: [u64; BTRFS_QGROUP_RSV_LAST as usize],
}

#[repr(C)]
pub struct btrfs_qgroup {
    pub qgroupid: u64,
    pub rfer: u64,
    pub rfer_cmpr: u64,
    pub excl: u64,
    pub excl_cmpr: u64,
    pub lim_flags: u64,
    pub max_rfer: u64,
    pub max_excl: u64,
    pub rsv_rfer: u64,
    pub rsv_excl: u64,
    pub rsv: btrfs_qgroup_rsv,
    pub groups: list_head,
    pub members: list_head,
    pub dirty: list_head,
    pub iterator: list_head,
    pub nested_iterator: list_head,
    pub node: rb_node,
    pub old_refcnt: u64,
    pub new_refcnt: u64,
    pub kobj: kobject,
}

#[repr(C)]
pub struct btrfs_qgroup_list {
    pub next_group: list_head,
    pub next_member: list_head,
    pub group: *mut btrfs_qgroup,
    pub member: *mut btrfs_qgroup,
}

#[repr(C)]
pub struct btrfs_squota_delta {
    pub root: u64,
    pub num_bytes: u64,
    pub generation: u64,
    pub is_inc: bool,
    pub is_data: bool,
}

/* BTRFS_QGROUP_LEVEL_SHIFT is supplied by btrfs_tree.h. */
pub fn btrfs_qgroup_subvolid(qgroupid: u64) -> u64 {
    qgroupid & ((1u64 << unsafe { BTRFS_QGROUP_LEVEL_SHIFT }) - 1)
}

pub const QGROUP_RESERVE: u32 = 1 << 0;
pub const QGROUP_RELEASE: u32 = 1 << 1;
pub const QGROUP_FREE: u32 = 1 << 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum btrfs_qgroup_mode {
    BTRFS_QGROUP_MODE_DISABLED,
    BTRFS_QGROUP_MODE_FULL,
    BTRFS_QGROUP_MODE_SIMPLE,
}

extern "C" {
    pub fn btrfs_qgroup_mode(fs_info: *const btrfs_fs_info) -> btrfs_qgroup_mode;
    pub fn btrfs_qgroup_enabled(fs_info: *const btrfs_fs_info) -> bool;
    pub fn btrfs_qgroup_full_accounting(fs_info: *const btrfs_fs_info) -> bool;
    pub fn btrfs_quota_enable(fs_info: *mut btrfs_fs_info, quota_ctl_args: *mut btrfs_ioctl_quota_ctl_args) -> c_int;
    pub fn btrfs_quota_disable(fs_info: *mut btrfs_fs_info) -> c_int;
    pub fn btrfs_qgroup_rescan(fs_info: *mut btrfs_fs_info) -> c_int;
    pub fn btrfs_qgroup_rescan_resume(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_qgroup_wait_for_completion(fs_info: *mut btrfs_fs_info, interruptible: bool) -> c_int;
    pub fn btrfs_add_qgroup_relation(trans: *mut btrfs_trans_handle, src: u64, dst: u64, prealloc: *mut btrfs_qgroup_list) -> c_int;
    pub fn btrfs_del_qgroup_relation(trans: *mut btrfs_trans_handle, src: u64, dst: u64) -> c_int;
    pub fn btrfs_create_qgroup(trans: *mut btrfs_trans_handle, qgroupid: u64) -> c_int;
    pub fn btrfs_remove_qgroup(trans: *mut btrfs_trans_handle, qgroupid: u64) -> c_int;
    pub fn btrfs_qgroup_cleanup_dropped_subvolume(fs_info: *mut btrfs_fs_info, subvolid: u64) -> c_int;
    pub fn btrfs_limit_qgroup(trans: *mut btrfs_trans_handle, qgroupid: u64, limit: *mut btrfs_qgroup_limit) -> c_int;
    pub fn btrfs_read_qgroup_config(fs_info: *mut btrfs_fs_info) -> c_int;
    pub fn btrfs_free_qgroup_config(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_qgroup_trace_extent_nolock(fs_info: *mut btrfs_fs_info, delayed_refs: *mut btrfs_delayed_ref_root, record: *mut btrfs_qgroup_extent_record, bytenr: u64) -> c_int;
    pub fn btrfs_qgroup_trace_extent_post(trans: *mut btrfs_trans_handle, qrecord: *mut btrfs_qgroup_extent_record, bytenr: u64) -> c_int;
    pub fn btrfs_qgroup_trace_extent(trans: *mut btrfs_trans_handle, bytenr: u64, num_bytes: u64) -> c_int;
    pub fn btrfs_qgroup_trace_leaf_items(trans: *mut btrfs_trans_handle, eb: *mut extent_buffer) -> c_int;
    pub fn btrfs_qgroup_trace_subtree(trans: *mut btrfs_trans_handle, root_eb: *mut extent_buffer, root_gen: u64, root_level: c_int) -> c_int;
    pub fn btrfs_qgroup_account_extent(trans: *mut btrfs_trans_handle, bytenr: u64, num_bytes: u64, old_roots: *mut ulist, new_roots: *mut ulist) -> c_int;
    pub fn btrfs_qgroup_account_extents(trans: *mut btrfs_trans_handle) -> c_int;
    pub fn btrfs_run_qgroups(trans: *mut btrfs_trans_handle) -> c_int;
    pub fn btrfs_qgroup_check_inherit(fs_info: *mut btrfs_fs_info, inherit: *mut btrfs_qgroup_inherit, size: usize) -> c_int;
    pub fn btrfs_qgroup_inherit(trans: *mut btrfs_trans_handle, srcid: u64, objectid: u64, inode_rootid: u64, inherit: *mut btrfs_qgroup_inherit) -> c_int;
    pub fn btrfs_qgroup_free_refroot(fs_info: *mut btrfs_fs_info, ref_root: u64, num_bytes: u64, ty: btrfs_qgroup_rsv_type);
    pub fn btrfs_qgroup_reserve_data(inode: *mut btrfs_inode, reserved: *mut *mut extent_changeset, start: u64, len: u64) -> c_int;
    pub fn btrfs_qgroup_release_data(inode: *mut btrfs_inode, start: u64, len: u64, released: *mut u64) -> c_int;
    pub fn btrfs_qgroup_free_data(inode: *mut btrfs_inode, reserved: *mut extent_changeset, start: u64, len: u64, freed: *mut u64) -> c_int;
    pub fn btrfs_qgroup_reserve_meta_prealloc(root: *mut btrfs_root, num_bytes: c_int, enforce: bool, noflush: bool) -> c_int;
    pub fn btrfs_qgroup_free_meta_prealloc(root: *mut btrfs_root, num_bytes: c_int);
    pub fn btrfs_qgroup_free_meta_all_pertrans(root: *mut btrfs_root);
    pub fn btrfs_qgroup_convert_reserved_meta(root: *mut btrfs_root, num_bytes: c_int);
    pub fn btrfs_qgroup_check_reserved_leak(inode: *mut btrfs_inode);
    pub fn btrfs_qgroup_init_swapped_blocks(swapped_blocks: *mut btrfs_qgroup_swapped_blocks);
    pub fn btrfs_qgroup_clean_swapped_blocks(root: *mut btrfs_root);
    pub fn btrfs_qgroup_add_swapped_blocks(subvol_root: *mut btrfs_root, bg: *mut btrfs_block_group, subvol_parent: *mut extent_buffer, subvol_slot: c_int, reloc_parent: *mut extent_buffer, reloc_slot: c_int, last_snapshot: u64) -> c_int;
    pub fn btrfs_qgroup_trace_subtree_after_cow(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, eb: *mut extent_buffer) -> c_int;
    pub fn btrfs_qgroup_destroy_extent_records(trans: *mut btrfs_transaction);
    pub fn btrfs_check_quota_leak(fs_info: *const btrfs_fs_info) -> bool;
    pub fn btrfs_record_squota_delta(fs_info: *mut btrfs_fs_info, delta: *const btrfs_squota_delta) -> c_int;
}

extern "C" {
    static BTRFS_QGROUP_LEVEL_SHIFT: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
