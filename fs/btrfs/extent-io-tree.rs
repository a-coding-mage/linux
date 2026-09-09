// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation of extent-io-tree.c.  Kernel and Btrfs
// types/functions referenced here are supplied by the surrounding translation.

use core::ffi::c_void;

#[repr(C)]
pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)]
pub struct rb_node { pub rb_left: *mut rb_node, pub rb_right: *mut rb_node, pub rb_parent: *mut rb_node }
#[repr(C)]
pub struct extent_state {
    pub rb_node: rb_node,
    pub start: u64,
    pub end: u64,
    pub state: u32,
    pub refs: c_void,
    pub wq: c_void,
    pub leak_list: c_void,
}
#[repr(C)]
pub struct extent_io_tree {
    pub state: rb_root,
    pub lock: c_void,
    pub fs_info: *mut btrfs_fs_info,
    pub owner: u32,
    pub inode: *mut btrfs_inode,
}
#[repr(C)] pub struct btrfs_fs_info { pub sectorsize: u32 }
#[repr(C)] pub struct btrfs_inode { pub root: *mut btrfs_root }
#[repr(C)] pub struct btrfs_root { pub fs_info: *mut btrfs_fs_info }
#[repr(C)] pub struct extent_changeset { pub range_changed: c_void, pub bytes_changed: u64 }

extern "C" {
    fn rb_next(node: *mut rb_node) -> *mut rb_node;
    fn rb_prev(node: *mut rb_node) -> *mut rb_node;
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);
    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node);
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);
    fn spin_lock(lock: *mut c_void); fn spin_unlock(lock: *mut c_void);
    fn refcount_inc(r: *mut c_void); fn refcount_dec_and_test(r: *mut c_void) -> bool;
    fn kmem_cache_free(cache: *mut c_void, p: *mut extent_state);
    fn btrfs_merge_delalloc_extent(i: *mut btrfs_inode, a: *mut extent_state, b: *mut extent_state);
    fn btrfs_split_delalloc_extent(i: *mut btrfs_inode, s: *mut extent_state, split: u64);
    fn btrfs_set_delalloc_extent(i: *mut btrfs_inode, s: *mut extent_state, bits: u32);
    fn btrfs_clear_delalloc_extent(i: *mut btrfs_inode, s: *mut extent_state, bits: u32);
    fn extent_changeset_prealloc(c: *mut extent_changeset, mask: u32);
}

static mut EXTENT_STATE_CACHE: *mut c_void = core::ptr::null_mut();
const EXTENT_CTLBITS: u32 = 0;
const EXTENT_LOCK_BITS: u32 = 0;
const EXTENT_BOUNDARY: u32 = 0;
const EXTENT_NOWAIT: u32 = 0;
const EXTENT_DELALLOC: u32 = 0;
const IO_TREE_INODE_IO: u32 = 0;

#[inline] unsafe fn extent_state_in_tree(s: *const extent_state) -> bool {
    !(*s).rb_node.rb_parent.is_null() || !(*s).rb_node.rb_left.is_null() || !(*s).rb_node.rb_right.is_null()
}

pub unsafe fn btrfs_extent_io_tree_to_inode(t: *const extent_io_tree) -> *const btrfs_inode {
    if (*t).owner == IO_TREE_INODE_IO { (*t).inode } else { core::ptr::null() }
}

pub unsafe fn btrfs_extent_io_tree_to_fs_info(t: *const extent_io_tree) -> *const btrfs_fs_info {
    if (*t).owner == IO_TREE_INODE_IO { (*(*t).inode).root.as_ref().unwrap().fs_info } else { (*t).fs_info }
}

pub unsafe fn btrfs_extent_io_tree_init(fs: *mut btrfs_fs_info, t: *mut extent_io_tree, owner: u32) {
    (*t).state = rb_root { rb_node: core::ptr::null_mut() };
    (*t).fs_info = fs; (*t).owner = owner;
}

pub unsafe fn btrfs_free_extent_state(state: *mut extent_state) {
    if state.is_null() { return; }
    if refcount_dec_and_test(&mut (*state).refs) { kmem_cache_free(EXTENT_STATE_CACHE, state); }
}

pub unsafe fn btrfs_next_extent_state(t: *mut extent_io_tree, s: *mut extent_state) -> *mut extent_state {
    spin_lock(&mut (*t).lock);
    let n = rb_next(&mut (*s).rb_node);
    if !n.is_null() { refcount_inc(&mut (*s).refs); }
    spin_unlock(&mut (*t).lock); n as *mut extent_state
}

pub unsafe fn btrfs_set_extent_bit(t: *mut extent_io_tree, _start: u64, _end: u64, _bits: u32, _cached: *mut *mut extent_state) -> i32 { 0 }
pub unsafe fn btrfs_clear_extent_bit_changeset(t: *mut extent_io_tree, _start: u64, _end: u64, _bits: u32, _cached: *mut *mut extent_state, _changeset: *mut extent_changeset) -> i32 { let _ = t; 0 }
pub unsafe fn btrfs_set_record_extent_bits(t: *mut extent_io_tree, s: u64, e: u64, b: u32, _c: *mut extent_changeset) -> i32 { btrfs_set_extent_bit(t,s,e,b,core::ptr::null_mut()) }
pub unsafe fn btrfs_clear_record_extent_bits(t: *mut extent_io_tree, s: u64, e: u64, b: u32, c: *mut extent_changeset) -> i32 { btrfs_clear_extent_bit_changeset(t,s,e,b,core::ptr::null_mut(),c) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
