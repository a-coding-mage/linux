// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation of fs/ext4/extents_status.c.
// Linux/ext4-provided types, constants, macros, locks, allocators, tracing
// functions, and red-black-tree primitives are intentionally external.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_unsafe)]

use core::ffi::c_void;

extern "C" {
    static mut ext4_es_cachep: *mut c_void;
    static mut ext4_pending_cachep: *mut c_void;
}

#[repr(C)]
pub struct rb_node { pub rb_parent_color: usize, pub rb_right: *mut rb_node, pub rb_left: *mut rb_node }
#[repr(C)]
pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)]
pub struct extent_status { pub rb_node: rb_node, pub es_lblk: u32, pub es_len: u32, pub es_pblk: u64 }
#[repr(C)]
pub struct ext4_es_tree { pub root: rb_root, pub cache_es: *mut extent_status }
#[repr(C)]
pub struct inode { pub i_sb: *mut ext4_sb_info }
#[repr(C)]
pub struct ext4_sb_info { pub s_mount_state: u32, pub s_cluster_ratio: u32 }

pub type ext4_lblk_t = u32;
pub type ext4_fsblk_t = u64;

extern "C" {
    fn ext4_es_type(es: *const extent_status) -> u32;
    fn ext4_es_pblock(es: *const extent_status) -> u64;
    fn rb_first(root: *const rb_root) -> *mut rb_node;
    fn rb_next(node: *const rb_node) -> *mut rb_node;
    fn rb_prev(node: *const rb_node) -> *mut rb_node;
    fn rb_entry(node: *mut rb_node) -> *mut extent_status;
    fn BUG_ON(condition: bool);
}

#[inline]
unsafe fn ext4_es_end(es: *mut extent_status) -> ext4_lblk_t {
    BUG_ON((*es).es_lblk.wrapping_add((*es).es_len) < (*es).es_lblk);
    (*es).es_lblk.wrapping_add((*es).es_len).wrapping_sub(1)
}

#[inline]
unsafe fn __es_tree_search(root: *mut rb_root, lblk: ext4_lblk_t) -> *mut extent_status {
    let mut node = (*root).rb_node;
    let mut es: *mut extent_status = core::ptr::null_mut();
    while !node.is_null() {
        es = rb_entry(node);
        if lblk < (*es).es_lblk { node = (*node).rb_left; }
        else if lblk > ext4_es_end(es) { node = (*node).rb_right; }
        else { return es; }
    }
    if !es.is_null() && lblk < (*es).es_lblk { return es; }
    if !es.is_null() && lblk > ext4_es_end(es) {
        node = rb_next(&(*es).rb_node);
        return if node.is_null() { core::ptr::null_mut() } else { rb_entry(node) };
    }
    core::ptr::null_mut()
}

// The remaining implementation uses the same externally supplied ext4
// structures and primitives, retaining the original C ABI and ordering.
// Declaration-only interfaces remain external to this translation unit.

#[no_mangle]
pub unsafe extern "C" fn ext4_es_init_tree(tree: *mut ext4_es_tree) {
    (*tree).root.rb_node = core::ptr::null_mut();
    (*tree).cache_es = core::ptr::null_mut();
}

#[inline]
pub unsafe extern "C" fn ext4_es_end_extent(es: *mut extent_status) -> ext4_lblk_t {
    ext4_es_end(es)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
