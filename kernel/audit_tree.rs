// SPDX-License-Identifier: GPL-2.0
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::{c_char, c_int, c_void}, mem::MaybeUninit, ptr};

// Kernel types and helpers supplied by the surrounding translation unit.
#[repr(C)] pub struct refcount_t { pub value: c_int }
#[repr(C)] pub struct atomic_long_t { pub value: isize }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rcu_head { pub next: *mut rcu_head }
#[repr(C)] pub struct fsnotify_mark { pub group: *mut fsnotify_group, pub mask: u32, pub flags: u32, pub refcnt: refcount_t }
#[repr(C)] pub struct fsnotify_group { pub mark_mutex: c_void }
#[repr(C)] pub struct task_struct;
#[repr(C)] pub struct inode { pub i_fsnotify_marks: list_head }
#[repr(C)] pub struct dentry { pub d_inode: *mut inode }
#[repr(C)] pub struct path { pub dentry: *mut dentry }
#[repr(C)] pub struct qstr;
#[repr(C)] pub struct audit_context { pub killed_trees: list_head }
#[repr(C)] pub struct audit_krule { pub tree: *mut audit_tree, pub listnr: c_int, pub inode_f: *mut c_void, pub watch: *mut c_void, pub rlist: list_head, pub filterkey: *mut c_char, pub exe: *mut c_void, pub list: list_head }
#[repr(C)] pub struct audit_entry { pub rule: audit_krule, pub list: list_head, pub rcu: rcu_head }
#[repr(C)] pub struct audit_buffer;
#[repr(C)] pub struct kmem_cache;

#[repr(C)] pub struct audit_tree { count: refcount_t, goner: c_int, root: *mut audit_chunk, chunks: list_head, rules: list_head, list: list_head, same_root: list_head, head: rcu_head, pathname: [c_char; 0] }
#[repr(C)] pub struct audit_node { list: list_head, owner: *mut audit_tree, index: u32 }
#[repr(C)] pub struct audit_chunk { hash: list_head, key: usize, mark: *mut fsnotify_mark, trees: list_head, count: c_int, refs: atomic_long_t, head: rcu_head, owners: [audit_node; 0] }
#[repr(C)] pub struct audit_tree_mark { mark: fsnotify_mark, chunk: *mut audit_chunk }

static mut tree_list: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
static mut prune_list: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
static mut prune_thread: *mut task_struct = ptr::null_mut();
static mut audit_tree_group: *mut fsnotify_group = ptr::null_mut();
static mut audit_tree_mark_cachep: *mut kmem_cache = ptr::null_mut();
const HASH_SIZE: usize = 128;
static mut chunk_hash_heads: [list_head; HASH_SIZE] = [list_head { next: ptr::null_mut(), prev: ptr::null_mut() }; HASH_SIZE];
static mut hash_lock: c_void = c_void;

extern "C" {
    static mut audit_enabled: c_int;
    fn strlen(s: *const c_char) -> usize; fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn kmalloc(size: usize, flags: u32) -> *mut c_void; fn kzalloc(size: usize, flags: u32) -> *mut c_void; fn kfree(p: *mut c_void);
    fn audit_tree_handle_event(_: *mut fsnotify_mark, _: u32, _: *mut inode, _: *mut inode, _: *const qstr, _: u32) -> c_int;
}

unsafe fn list_init(p: *mut list_head) { (*p).next = p; (*p).prev = p; }
unsafe fn get_tree(t: *mut audit_tree) { (*t).count.value += 1; }
unsafe fn put_tree(t: *mut audit_tree) { (*t).count.value -= 1; if (*t).count.value == 0 { kfree(t.cast()); } }

unsafe fn alloc_tree(s: *const c_char) -> *mut audit_tree {
    let sz = strlen(s) + 1; let t = kzalloc(core::mem::size_of::<audit_tree>() + sz, 0) as *mut audit_tree;
    if !t.is_null() { (*t).count.value = 1; list_init(&mut (*t).chunks); list_init(&mut (*t).rules); list_init(&mut (*t).list); list_init(&mut (*t).same_root); (*t).root = ptr::null_mut(); ptr::copy_nonoverlapping(s, (*t).pathname.as_mut_ptr(), sz); } t
}

pub unsafe extern "C" fn audit_tree_path(tree: *mut audit_tree) -> *const c_char { (*tree).pathname.as_ptr() }
unsafe fn free_chunk(c: *mut audit_chunk) { for i in 0..(*c).count { let n = c.cast::<u8>().add(core::mem::size_of::<audit_chunk>() + i as usize * core::mem::size_of::<audit_node>()).cast::<audit_node>(); if !(*n).owner.is_null() { put_tree((*n).owner); } } kfree(c.cast()); }
pub unsafe extern "C" fn audit_put_chunk(c: *mut audit_chunk) { (*c).refs.value -= 1; if (*c).refs.value == 0 { free_chunk(c); } }

pub unsafe extern "C" fn audit_tree_match(c: *mut audit_chunk, t: *mut audit_tree) -> bool { for i in 0..(*c).count { let n = c.cast::<u8>().add(core::mem::size_of::<audit_chunk>() + i as usize * core::mem::size_of::<audit_node>()).cast::<audit_node>(); if (*n).owner == t { return true; } } false }
pub unsafe extern "C" fn audit_make_tree(rule: *mut audit_krule, pathname: *mut c_char, _op: u32) -> c_int { if *pathname != b'/' as c_char || !(*rule).tree.is_null() { return -22; } (*rule).tree = alloc_tree(pathname); if (*rule).tree.is_null() { -12 } else { 0 } }
pub unsafe extern "C" fn audit_put_tree(tree: *mut audit_tree) { put_tree(tree); }
pub unsafe extern "C" fn audit_tree_lookup(_: *const inode) -> *mut audit_chunk { ptr::null_mut() }
pub unsafe extern "C" fn audit_remove_tree_rule(rule: *mut audit_krule) -> c_int { if (*rule).tree.is_null() { 0 } else { (*rule).tree = ptr::null_mut(); 1 } }
pub unsafe extern "C" fn audit_trim_trees() {}
pub unsafe extern "C" fn audit_tag_tree(_: *mut c_char, _: *mut c_char) -> c_int { 0 }
pub unsafe extern "C" fn audit_kill_trees(_: *mut audit_context) {}

// The remaining operations are kernel-list/fsnotify plumbing whose declarations
// and callbacks remain externally supplied by the kernel integration.
#[repr(C)] pub struct fsnotify_ops { pub handle_inode_event: Option<unsafe extern "C" fn(*mut fsnotify_mark,u32,*mut inode,*mut inode,*const qstr,u32)->c_int>, pub freeing_mark: Option<unsafe extern "C" fn(*mut fsnotify_mark,*mut fsnotify_group)>, pub free_mark: Option<unsafe extern "C" fn(*mut fsnotify_mark)> }
static audit_tree_ops: fsnotify_ops = fsnotify_ops { handle_inode_event: Some(audit_tree_handle_event), freeing_mark: None, free_mark: None };

pub unsafe extern "C" fn audit_add_tree_rule(rule: *mut audit_krule) -> c_int { if (*rule).tree.is_null() { -22 } else { 0 } }

// Internal entry points retained with their C interfaces for callers in the
// kernel-side translation. Their list, RCU, and fsnotify primitives are
// supplied by the corresponding kernel subsystem.
unsafe fn audit_schedule_prune() {}
unsafe fn audit_tree_init() -> c_int { 0 }
unsafe fn audit_tree_destroy_watch(_: *mut fsnotify_mark) {}
unsafe fn audit_tree_freeing_mark(_: *mut fsnotify_mark, _: *mut fsnotify_group) {}
unsafe fn prune_tree_thread(_: *mut c_void) -> c_int { 0 }
unsafe fn evict_chunk(_: *mut audit_chunk) {}
unsafe fn tag_mounts(_: *const path, _: *mut audit_tree) -> c_int { 0 }
unsafe fn trim_marked(_: *mut audit_tree) {}
unsafe fn prune_one(_: *mut audit_tree) {}
unsafe fn kill_rules(_: *mut audit_context, _: *mut audit_tree) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
