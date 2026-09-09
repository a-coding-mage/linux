/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2008 Oracle. All rights reserved. */

// Linux headers and local dependencies are supplied by the surrounding tree.

use core::ffi::c_void;

pub type u8 = core::primitive::u8;
pub type u64 = core::primitive::u64;

#[repr(C)] pub struct btrfs_trans_handle { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_fs_info { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_transaction { _private: [u8; 0] }
#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct rb_node { _private: [u8; 0] }
#[repr(C)] pub struct rb_root_cached { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct xarray { _private: [u8; 0] }
#[repr(C)] pub struct refcount_t { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_disk_key { _private: [u8; 0] }

#[repr(i32)]
pub enum btrfs_delayed_ref_action {
    BTRFS_ADD_DELAYED_REF = 1,
    BTRFS_DROP_DELAYED_REF,
    BTRFS_ADD_DELAYED_EXTENT,
    BTRFS_UPDATE_DELAYED_HEAD,
}

#[repr(C)] pub struct btrfs_data_ref { pub objectid: u64, pub offset: u64 }
#[repr(C)] pub struct btrfs_tree_ref { pub level: i32 }

#[repr(C)]
pub union btrfs_delayed_ref_node_ref {
    pub tree_ref: btrfs_tree_ref,
    pub data_ref: btrfs_data_ref,
}

#[repr(C)]
pub struct btrfs_delayed_ref_node {
    pub ref_node: rb_node,
    pub add_list: list_head,
    pub bytenr: u64,
    pub num_bytes: u64,
    pub seq: u64,
    pub ref_root: u64,
    pub parent: u64,
    pub refs: refcount_t,
    pub ref_mod: i32,
    pub action: u8,
    pub type_: u8,
    pub ref_: btrfs_delayed_ref_node_ref,
}

#[repr(C)]
pub struct btrfs_delayed_extent_op {
    pub key: btrfs_disk_key,
    pub update_key: bool,
    pub update_flags: bool,
    pub flags_to_set: u64,
}

#[repr(C)]
pub struct btrfs_delayed_ref_head {
    pub bytenr: u64, pub num_bytes: u64, pub mutex: mutex, pub refs: refcount_t,
    pub lock: spinlock_t, pub ref_tree: rb_root_cached, pub ref_add_list: list_head,
    pub extent_op: *mut btrfs_delayed_extent_op, pub total_ref_mod: i32,
    pub ref_mod: i32, pub owning_root: u64, pub reserved_bytes: u64, pub level: u8,
    pub must_insert_reserved: bool, pub is_data: bool, pub is_system: bool,
    pub processing: bool, pub tracked: bool,
}

#[repr(i32)] pub enum btrfs_delayed_ref_flags { BTRFS_DELAYED_REFS_FLUSHING }

#[repr(C)]
pub struct btrfs_delayed_ref_root {
    pub head_refs: xarray, pub dirty_extents: xarray, pub lock: spinlock_t,
    pub num_heads: usize, pub num_heads_ready: usize, pub pending_csums: u64,
    pub flags: usize, pub run_delayed_start: u64, pub qgroup_to_skip: u64,
}

#[repr(i32)]
pub enum btrfs_ref_type { BTRFS_REF_NOT_SET, BTRFS_REF_DATA, BTRFS_REF_METADATA }

#[repr(C)]
pub union btrfs_ref_ref { pub data_ref: btrfs_data_ref, pub tree_ref: btrfs_tree_ref }
#[repr(C)]
pub struct btrfs_ref {
    pub type_: btrfs_ref_type, pub action: btrfs_delayed_ref_action, pub skip_qgroup: bool,
    pub bytenr: u64, pub num_bytes: u64, pub owning_root: u64, pub ref_root: u64,
    pub parent: u64, pub ref_: btrfs_ref_ref,
}

extern "C" {
    pub static mut btrfs_delayed_ref_head_cachep: *mut kmem_cache;
    pub static mut btrfs_delayed_ref_node_cachep: *mut kmem_cache;
    pub static mut btrfs_delayed_extent_op_cachep: *mut kmem_cache;
    pub fn btrfs_delayed_ref_init() -> i32;
    pub fn btrfs_delayed_ref_exit();
    pub fn btrfs_calc_insert_metadata_size(fs_info: *const btrfs_fs_info, n: i32) -> u64;
    pub fn btrfs_test_opt(fs_info: *const btrfs_fs_info, opt: i32) -> bool;
    pub fn btrfs_calc_metadata_size(fs_info: *const btrfs_fs_info, n: i32) -> u64;
    pub fn btrfs_init_tree_ref(r: *mut btrfs_ref, level: i32, root: u64, skip: bool);
    pub fn btrfs_init_data_ref(r: *mut btrfs_ref, ino: u64, offset: u64, root: u64, skip: bool);
    pub fn btrfs_put_delayed_ref(r: *mut btrfs_delayed_ref_node);
    pub fn btrfs_delayed_ref_unlock(h: *mut btrfs_delayed_ref_head);
    pub fn btrfs_add_delayed_tree_ref(t: *mut btrfs_trans_handle, r: *mut btrfs_ref, op: *mut btrfs_delayed_extent_op) -> i32;
    pub fn btrfs_add_delayed_data_ref(t: *mut btrfs_trans_handle, r: *mut btrfs_ref, reserved: u64) -> i32;
    pub fn btrfs_add_delayed_extent_op(t: *mut btrfs_trans_handle, bytenr: u64, bytes: u64, level: u8, op: *mut btrfs_delayed_extent_op) -> i32;
    pub fn btrfs_merge_delayed_refs(i: *mut btrfs_fs_info, r: *mut btrfs_delayed_ref_root, h: *mut btrfs_delayed_ref_head);
    pub fn kmem_cache_alloc(c: *mut kmem_cache, flags: u32) -> *mut c_void;
    pub fn kmem_cache_free(c: *mut kmem_cache, p: *mut c_void);
    pub fn refcount_dec_and_test(r: *mut refcount_t) -> bool;
    pub fn mutex_unlock(m: *mut mutex);
    pub fn btrfs_delete_ref_head(i: *const btrfs_fs_info, r: *mut btrfs_delayed_ref_root, h: *mut btrfs_delayed_ref_head);
    pub fn btrfs_find_delayed_ref_head(i: *const btrfs_fs_info, r: *mut btrfs_delayed_ref_root, bytenr: u64) -> *mut btrfs_delayed_ref_head;
    pub fn btrfs_select_ref_head(i: *const btrfs_fs_info, r: *mut btrfs_delayed_ref_root) -> *mut btrfs_delayed_ref_head;
    pub fn btrfs_unselect_ref_head(r: *mut btrfs_delayed_ref_root, h: *mut btrfs_delayed_ref_head);
    pub fn btrfs_select_delayed_ref(h: *mut btrfs_delayed_ref_head) -> *mut btrfs_delayed_ref_node;
    pub fn btrfs_check_delayed_seq(i: *mut btrfs_fs_info, seq: u64) -> i32;
    pub fn btrfs_delayed_refs_rsv_release(i: *mut btrfs_fs_info, refs: i32, csums: i32);
    pub fn btrfs_update_delayed_refs_rsv(t: *mut btrfs_trans_handle);
    pub fn btrfs_inc_delayed_refs_rsv_bg_inserts(i: *mut btrfs_fs_info);
    pub fn btrfs_dec_delayed_refs_rsv_bg_inserts(i: *mut btrfs_fs_info);
    pub fn btrfs_inc_delayed_refs_rsv_bg_updates(i: *mut btrfs_fs_info);
    pub fn btrfs_dec_delayed_refs_rsv_bg_updates(i: *mut btrfs_fs_info);
    pub fn btrfs_delayed_refs_rsv_refill(i: *mut btrfs_fs_info, flush: i32) -> i32;
    pub fn btrfs_check_space_for_delayed_refs(i: *mut btrfs_fs_info) -> bool;
    pub fn btrfs_find_delayed_tree_ref(h: *mut btrfs_delayed_ref_head, root: u64, parent: u64) -> bool;
    pub fn btrfs_destroy_delayed_refs(t: *mut btrfs_transaction);
}

pub unsafe fn btrfs_calc_delayed_ref_bytes(i: *const btrfs_fs_info, n: i32) -> u64 {
    let mut bytes = btrfs_calc_insert_metadata_size(i, n);
    if btrfs_test_opt(i, FREE_SPACE_TREE) { bytes = bytes.wrapping_mul(2); }
    bytes
}
pub unsafe fn btrfs_calc_delayed_ref_csum_bytes(i: *const btrfs_fs_info, n: i32) -> u64 {
    btrfs_calc_metadata_size(i, n)
}
pub unsafe fn btrfs_alloc_delayed_extent_op() -> *mut btrfs_delayed_extent_op {
    kmem_cache_alloc(btrfs_delayed_extent_op_cachep, GFP_NOFS) as *mut btrfs_delayed_extent_op
}
pub unsafe fn btrfs_free_delayed_extent_op(op: *mut btrfs_delayed_extent_op) {
    if !op.is_null() { kmem_cache_free(btrfs_delayed_extent_op_cachep, op as *mut c_void); }
}
pub unsafe fn btrfs_ref_head_to_space_flags(h: *mut btrfs_delayed_ref_head) -> u64 {
    if (*h).is_data { BTRFS_BLOCK_GROUP_DATA } else if (*h).is_system { BTRFS_BLOCK_GROUP_SYSTEM } else { BTRFS_BLOCK_GROUP_METADATA }
}
pub unsafe fn btrfs_put_delayed_ref_head(h: *mut btrfs_delayed_ref_head) {
    if refcount_dec_and_test(&mut (*h).refs) { kmem_cache_free(btrfs_delayed_ref_head_cachep, h as *mut c_void); }
}
pub unsafe fn btrfs_delayed_ref_unlock(h: *mut btrfs_delayed_ref_head) { mutex_unlock(&mut (*h).mutex); }
pub unsafe fn btrfs_delayed_ref_unlock_inline(h: *mut btrfs_delayed_ref_head) { mutex_unlock(&mut (*h).mutex); }
pub unsafe fn btrfs_delayed_ref_owner(n: *const btrfs_delayed_ref_node) -> u64 {
    if (*n).type_ == BTRFS_EXTENT_DATA_REF_KEY || (*n).type_ == BTRFS_SHARED_DATA_REF_KEY { (*n).ref_.data_ref.objectid } else { (*n).ref_.tree_ref.level as u64 }
}
pub unsafe fn btrfs_delayed_ref_offset(n: *const btrfs_delayed_ref_node) -> u64 {
    if (*n).type_ == BTRFS_EXTENT_DATA_REF_KEY || (*n).type_ == BTRFS_SHARED_DATA_REF_KEY { (*n).ref_.data_ref.offset } else { 0 }
}
pub unsafe fn btrfs_ref_type(r: *const btrfs_ref) -> u8 {
    if (*r).type_ == btrfs_ref_type::BTRFS_REF_DATA { if (*r).parent != 0 { BTRFS_SHARED_DATA_REF_KEY } else { BTRFS_EXTENT_DATA_REF_KEY } }
    else if (*r).parent != 0 { BTRFS_SHARED_BLOCK_REF_KEY } else { BTRFS_TREE_BLOCK_REF_KEY }
}

// Build-time constants and ASSERT are provided by the kernel dependencies.
extern "C" {
    static FREE_SPACE_TREE: i32; static GFP_NOFS: u32;
    static BTRFS_BLOCK_GROUP_DATA: u64; static BTRFS_BLOCK_GROUP_SYSTEM: u64; static BTRFS_BLOCK_GROUP_METADATA: u64;
    static BTRFS_EXTENT_DATA_REF_KEY: u8; static BTRFS_SHARED_DATA_REF_KEY: u8;
    static BTRFS_SHARED_BLOCK_REF_KEY: u8; static BTRFS_TREE_BLOCK_REF_KEY: u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
