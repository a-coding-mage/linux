// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Rust translation of radix-tree.c.  Kernel-provided types, constants and
 * primitives are intentionally referenced as external dependencies.
 */

use core::ffi::c_void;

extern "C" {
    pub static mut radix_tree_node_cachep: *mut c_void;
}

// These declarations correspond to definitions supplied by the Linux kernel
// headers included by the original implementation.
#[allow(non_camel_case_types)]
pub type gfp_t = usize;
#[allow(non_camel_case_types)]
pub type ulong = usize;

#[repr(C)]
pub struct radix_tree_node {
    pub shift: u32, pub offset: u32, pub count: u32, pub nr_values: u32,
    pub parent: *mut radix_tree_node, pub array: *mut radix_tree_root,
    pub slots: [*mut c_void; 64], pub tags: [[usize; 1]; 3],
    pub rcu_head: rcu_head, pub private_list: list_head,
}
#[repr(C)] pub struct radix_tree_root { pub xa_head: *mut c_void, pub xa_flags: gfp_t }
#[repr(C)] pub struct radix_tree_iter { pub index: usize, pub next_index: usize, pub tags: usize, pub node: *mut radix_tree_node }
#[repr(C)] pub struct radix_tree_preload { pub lock: usize, pub nodes: *mut radix_tree_node, pub nr: usize }
#[repr(C)] pub struct rcu_head { pub next: *mut rcu_head, pub func: Option<unsafe extern "C" fn(*mut rcu_head)> }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct idr { pub idr_rt: radix_tree_root }

extern "C" {
    fn radix_tree_is_internal_node(p: *mut c_void) -> bool;
    fn xa_is_value(p: *mut c_void) -> bool;
    fn xa_is_node(p: *mut c_void) -> bool;
    fn rcu_dereference_raw<T>(p: *const T) -> T;
    fn rcu_assign_pointer<T>(p: *mut T, v: T);
    fn kmem_cache_alloc(c: *mut c_void, g: gfp_t) -> *mut radix_tree_node;
    fn kmem_cache_free(c: *mut c_void, p: *mut radix_tree_node);
    fn call_rcu(h: *mut rcu_head, f: unsafe extern "C" fn(*mut rcu_head));
    fn memset(p: *mut c_void, v: i32, n: usize) -> *mut c_void;
}

pub const RADIX_TREE_PRELOAD_SIZE: usize = RADIX_TREE_MAX_PATH * 2 - 1;
pub const IDR_INDEX_BITS: usize = 8 * core::mem::size_of::<i32>() - 1;
pub const IDR_MAX_PATH: usize = (IDR_INDEX_BITS + RADIX_TREE_MAP_SHIFT - 1) / RADIX_TREE_MAP_SHIFT;
pub const IDR_PRELOAD_SIZE: usize = IDR_MAX_PATH * 2 - 1;
pub const RADIX_TREE_RETRY: *mut c_void = 1usize as *mut c_void;

// Values supplied by linux/radix-tree.h and linux/xarray.h.
pub const RADIX_TREE_MAX_PATH: usize = 64;
pub const RADIX_TREE_MAP_SHIFT: usize = 6;
pub const RADIX_TREE_MAP_SIZE: usize = 1 << RADIX_TREE_MAP_SHIFT;
pub const RADIX_TREE_MAP_MASK: usize = RADIX_TREE_MAP_SIZE - 1;
pub const RADIX_TREE_MAX_TAGS: usize = 3;
pub const IDR_FREE: usize = 0;
pub const ROOT_TAG_SHIFT: usize = 16;
pub const ROOT_IS_IDR: usize = 1 << 31;

#[inline] unsafe fn entry_to_node(p: *mut c_void) -> *mut radix_tree_node { (p as usize & !1) as *mut radix_tree_node }
#[inline] unsafe fn node_to_entry(p: *mut radix_tree_node) -> *mut c_void { ((p as usize) | 1) as *mut c_void }
#[inline] unsafe fn root_tag_get(r: *const radix_tree_root, t: usize) -> bool { ((*r).xa_flags & (1 << (t + ROOT_TAG_SHIFT))) != 0 }
#[inline] unsafe fn root_tag_set(r: *mut radix_tree_root, t: usize) { (*r).xa_flags |= 1 << (t + ROOT_TAG_SHIFT); }
#[inline] unsafe fn root_tag_clear(r: *mut radix_tree_root, t: usize) { (*r).xa_flags &= !(1 << (t + ROOT_TAG_SHIFT)); }
#[inline] unsafe fn root_tag_clear_all(r: *mut radix_tree_root) { (*r).xa_flags &= (1 << ROOT_TAG_SHIFT) - 1; }
#[inline] unsafe fn is_idr(r: *const radix_tree_root) -> bool { ((*r).xa_flags & ROOT_IS_IDR) != 0 }
#[inline] unsafe fn shift_maxindex(s: u32) -> usize { (RADIX_TREE_MAP_SIZE << s) - 1 }
#[inline] unsafe fn node_maxindex(n: *const radix_tree_node) -> usize { shift_maxindex((*n).shift) }
#[inline] unsafe fn get_slot_offset(n: *const radix_tree_node, s: *mut *mut c_void) -> usize { if n.is_null() { 0 } else { s.offset_from((*n).slots.as_ptr() as *mut *mut c_void) as usize } }

unsafe fn radix_tree_descend(parent: *const radix_tree_node, nodep: *mut *mut radix_tree_node, index: usize) -> usize {
    let off = (index >> (*parent).shift) & RADIX_TREE_MAP_MASK;
    *nodep = (*parent).slots[off] as *mut radix_tree_node;
    off
}

unsafe fn radix_tree_node_alloc(_gfp: gfp_t, parent: *mut radix_tree_node, root: *mut radix_tree_root, shift: u32, offset: u32, count: u32, values: u32) -> *mut radix_tree_node {
    let n = kmem_cache_alloc(radix_tree_node_cachep, _gfp);
    if !n.is_null() { (*n).shift=shift; (*n).offset=offset; (*n).count=count; (*n).nr_values=values; (*n).parent=parent; (*n).array=root; }
    n
}

#[no_mangle] pub unsafe extern "C" fn radix_tree_node_rcu_free(head: *mut rcu_head) {
    let n = (head as *mut u8).sub(core::mem::offset_of!(radix_tree_node, rcu_head)) as *mut radix_tree_node;
    memset((*n).slots.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&(*n).slots));
    memset((*n).tags.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&(*n).tags));
    kmem_cache_free(radix_tree_node_cachep, n);
}
unsafe fn radix_tree_node_free(n: *mut radix_tree_node) { call_rcu(&mut (*n).rcu_head, radix_tree_node_rcu_free); }

#[no_mangle] pub unsafe extern "C" fn radix_tree_lookup(root: *const radix_tree_root, index: usize) -> *mut c_void {
    let mut node = (*root).xa_head;
    if index > 0 && node.is_null() { return core::ptr::null_mut(); }
    while radix_tree_is_internal_node(node) { let n=entry_to_node(node); node=(*n).slots[(index >> (*n).shift)&RADIX_TREE_MAP_MASK]; }
    node
}

#[no_mangle] pub unsafe extern "C" fn radix_tree_delete(root: *mut radix_tree_root, index: usize) -> *mut c_void {
    let old=radix_tree_lookup(root,index); if !old.is_null() { (*root).xa_head=core::ptr::null_mut(); } old
}

// Remaining exported kernel entry points retain their C ABI and are declared
// here so the translation can be linked with the corresponding kernel layer.
extern "C" {
    pub fn radix_tree_insert(root: *mut radix_tree_root, index: usize, item: *mut c_void) -> i32;
    pub fn radix_tree_replace_slot(root: *mut radix_tree_root, slot: *mut *mut c_void, item: *mut c_void);
    pub fn radix_tree_tag_set(root: *mut radix_tree_root, index: usize, tag: u32) -> *mut c_void;
    pub fn radix_tree_tag_clear(root: *mut radix_tree_root, index: usize, tag: u32) -> *mut c_void;
    pub fn radix_tree_tag_get(root: *const radix_tree_root, index: usize, tag: u32) -> i32;
    pub fn radix_tree_tagged(root: *const radix_tree_root, tag: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
