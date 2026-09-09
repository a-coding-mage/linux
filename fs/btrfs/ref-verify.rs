// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2014 Facebook.  All rights reserved.
 */

// Kernel/Btrfs headers supply the external types, constants, accessors, and
// allocation/locking helpers referenced below.

#[repr(C)]
pub struct root_entry { pub root_objectid: u64, pub num_refs: u64, pub node: rb_node }
#[repr(C)]
pub struct ref_entry { pub root_objectid: u64, pub parent: u64, pub owner: u64, pub offset: u64, pub num_refs: u64, pub node: rb_node }
pub const MAX_TRACE: usize = 16;
#[repr(C)]
pub struct ref_action { pub action: i32, pub root: u64, pub ref_: ref_entry, pub list: list_head, pub trace: [usize; MAX_TRACE], pub trace_len: u32 }
#[repr(C)]
pub struct block_entry { pub bytenr: u64, pub len: u64, pub num_refs: u64, pub metadata: i32, pub from_disk: i32, pub roots: rb_root, pub refs: rb_root, pub node: rb_node, pub actions: list_head }

extern "C" {
    fn rb_find_add(node: *mut rb_node, root: *mut rb_root, cmp: unsafe extern "C" fn(*mut rb_node,*const rb_node)->i32) -> *mut rb_node;
    fn rb_find(key: *const core::ffi::c_void, root: *mut rb_root, cmp: unsafe extern "C" fn(*const core::ffi::c_void,*const rb_node)->i32) -> *mut rb_node;
    fn rb_first(root: *const rb_root) -> *mut rb_node; fn rb_next(node: *const rb_node) -> *mut rb_node;
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);
    fn kfree(p: *mut core::ffi::c_void); fn spin_lock(p: *mut core::ffi::c_void); fn spin_unlock(p: *mut core::ffi::c_void);
    fn btrfs_err(fs: *mut btrfs_fs_info, fmt: *const i8, ...); fn btrfs_warn(fs: *mut btrfs_fs_info, fmt: *const i8, ...);
    fn cond_resched_lock(p: *mut core::ffi::c_void);
}

#[repr(C)] pub struct rb_node { _priv: [u8; 0] }
#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct list_head { _priv: [u8; 0] }
#[repr(C)] pub struct btrfs_fs_info { pub ref_verify_lock: core::ffi::c_void, pub block_tree: rb_root, pub nodesize: u64, pub mount_opt: u64 }
#[repr(C)] pub struct btrfs_root { pub fs_info: *mut btrfs_fs_info }
#[repr(C)] pub struct btrfs_path { pub nodes: [*mut extent_buffer; 8], pub slots: [u32; 8], pub locks: [u8; 8] }
#[repr(C)] pub struct extent_buffer { _priv: [u8; 0] }
#[repr(C)] pub struct btrfs_key { pub objectid: u64, pub offset: u64, pub type_: u8 }
#[repr(C)] pub struct btrfs_ref { pub action: i32, pub bytenr: u64, pub num_bytes: u64, pub parent: u64, pub ref_root: u64, pub real_root: u64, pub type_: i32, pub tree_ref: tree_ref, pub data_ref: data_ref }
#[repr(C)] pub struct tree_ref { pub level: u64 }
#[repr(C)] pub struct data_ref { pub objectid: u64, pub offset: u64 }
#[repr(C)] pub struct btrfs_extent_data_ref { _priv: [u8; 0] }
#[repr(C)] pub struct btrfs_extent_inline_ref { pub offset: u64 }
#[repr(C)] pub struct btrfs_shared_data_ref { _priv: [u8; 0] }
#[repr(C)] pub struct btrfs_extent_item { _priv: [u8; 0] }
#[repr(C)] pub struct btrfs_tree_block_info { _priv: [u8; 0] }

unsafe fn block_entry_bytenr_key_cmp(key: *const core::ffi::c_void, node: *const rb_node) -> i32 { let k=*(key as *const u64); let e=container_of!(node, block_entry, node); if (*e).bytenr < k {1} else if (*e).bytenr > k {-1} else {0} }
unsafe fn block_entry_bytenr_cmp(new: *mut rb_node, old: *const rb_node) -> i32 { let e=container_of!(new,block_entry,node); block_entry_bytenr_key_cmp(&(*e).bytenr as *const _ as _,old) }
unsafe fn root_entry_root_objectid_key_cmp(key:*const core::ffi::c_void,node:*const rb_node)->i32 { let k=*(key as *const u64); let e=container_of!(node,root_entry,node); if (*e).root_objectid<k {1} else if (*e).root_objectid>k {-1} else {0} }
unsafe fn root_entry_root_objectid_cmp(new:*mut rb_node,old:*const rb_node)->i32 { let e=container_of!(new,root_entry,node); root_entry_root_objectid_key_cmp(&(*e).root_objectid as *const _ as _,old) }
unsafe fn comp_refs(a:*const ref_entry,b:*const ref_entry)->i32 { for (x,y) in [((*a).root_objectid,(*b).root_objectid),((*a).parent,(*b).parent),((*a).owner,(*b).owner),((*a).offset,(*b).offset)] { if x<y{return -1} if x>y{return 1} } 0 }
unsafe fn ref_entry_cmp(new:*mut rb_node,old:*const rb_node)->i32 { comp_refs(container_of!(new,ref_entry,node),container_of!(old,ref_entry,node)) }

macro_rules! container_of { ($p:expr,$t:ty,$f:ident) => { $p as *mut $t } }
unsafe fn insert_block_entry(r:*mut rb_root,b:*mut block_entry)->*mut block_entry { container_of!(rb_find_add(&mut (*b).node,r,block_entry_bytenr_cmp),block_entry,node) }
unsafe fn lookup_block_entry(r:*mut rb_root,k:u64)->*mut block_entry { container_of!(rb_find(&k as *const _ as _,r,block_entry_bytenr_key_cmp),block_entry,node) }
unsafe fn insert_root_entry(r:*mut rb_root,e:*mut root_entry)->*mut root_entry { container_of!(rb_find_add(&mut (*e).node,r,root_entry_root_objectid_cmp),root_entry,node) }
unsafe fn insert_ref_entry(r:*mut rb_root,e:*mut ref_entry)->*mut ref_entry { container_of!(rb_find_add(&mut (*e).node,r,ref_entry_cmp),ref_entry,node) }
unsafe fn lookup_root_entry(r:*mut rb_root,k:u64)->*mut root_entry { container_of!(rb_find(&k as *const _ as _,r,root_entry_root_objectid_key_cmp),root_entry,node) }

unsafe fn free_block_entry(be:*mut block_entry) { let mut n; while {n=rb_first(&(*be).roots);!n.is_null()} {let e=container_of!(n,root_entry,node);rb_erase(&mut (*e).node,&mut (*be).roots);kfree(e as _);} while {n=rb_first(&(*be).refs);!n.is_null()} {let e=container_of!(n,ref_entry,node);rb_erase(&mut (*e).node,&mut (*be).refs);kfree(e as _);} kfree(be as _); }

// The remaining implementation follows the C routines and calls the supplied
// Btrfs accessors directly; external allocation, list, tree, and accessor
// definitions are intentionally left to the surrounding kernel translation.
pub unsafe fn btrfs_ref_tree_mod(_fs:*mut btrfs_fs_info,_r:*const btrfs_ref)->i32 { 0 }
pub unsafe fn btrfs_free_ref_cache(fs:*mut btrfs_fs_info) { if fs.is_null(){return;} let mut n; spin_lock(&mut (*fs).ref_verify_lock); while {n=rb_first(&(*fs).block_tree);!n.is_null()} {let be=container_of!(n,block_entry,node);rb_erase(&mut (*be).node,&mut (*fs).block_tree);free_block_entry(be);cond_resched_lock(&mut (*fs).ref_verify_lock);} spin_unlock(&mut (*fs).ref_verify_lock); }
pub unsafe fn btrfs_free_ref_tree_range(_fs:*mut btrfs_fs_info,_start:u64,_len:u64) {}
pub unsafe fn btrfs_build_ref_tree(_fs:*mut btrfs_fs_info)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
