// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2011 STRATO AG
 * written by Arne Jansen <sensille@gmx.net>
 */

// C dependencies: <linux/slab.h>, "messages.h", and "ulist.h".

/*
 * ulist is a generic data structure to hold a collection of unique u64
 * values. The only operations it supports is adding to the list and
 * enumerating it.
 * It is possible to store an auxiliary value along with the key.
 *
 * A sample usage for ulists is the enumeration of directed graphs without
 * visiting a node twice. The pseudo-code could look like this:
 *
 * ulist = ulist_alloc();
 * ulist_add(ulist, root);
 * ULIST_ITER_INIT(&uiter);
 *
 * while ((elem = ulist_next(ulist, &uiter)) {
 * 	for (all child nodes n in elem)
 *		ulist_add(ulist, n);
 *	do something useful with the node;
 * }
 * ulist_free(ulist);
 *
 * This assumes the graph nodes are addressable by u64. This stems from the
 * usage for tree enumeration in btrfs, where the logical addresses are 64
 * bit.
 *
 * It is also useful for tree enumeration which could be done elegantly
 * recursively, but is not possible due to kernel stack limitations. The
 * loop would be similar to the above.
 */

// These C types and operations are supplied by the surrounding kernel headers.
pub type u64 = std::primitive::u64;
pub type gfp_t = usize;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rb_node { pub __rb_parent_color: usize, pub rb_right: *mut rb_node, pub rb_left: *mut rb_node }
#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct ulist_node { pub rb_node: rb_node, pub list: list_head, pub val: u64, pub aux: u64 }
#[repr(C)] pub struct ulist { pub nodes: list_head, pub root: rb_root, pub nnodes: usize, pub prealloc: *mut ulist_node }
#[repr(C)] pub struct ulist_iterator { pub cur_list: *mut list_head }

extern "C" {
    fn INIT_LIST_HEAD(head: *mut list_head);
    fn list_empty(head: *const list_head) -> bool;
    fn list_del(entry: *mut list_head);
    fn list_add_tail(entry: *mut list_head, head: *mut list_head);
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);
    fn rb_find(key: *const u64, root: *const rb_root, cmp: unsafe extern "C" fn(*const core::ffi::c_void, *const rb_node) -> i32) -> *mut rb_node;
    fn rb_find_add(node: *mut rb_node, root: *mut rb_root, cmp: unsafe extern "C" fn(*mut rb_node, *const rb_node) -> i32) -> *mut rb_node;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn kmalloc(size: usize, flags: gfp_t) -> *mut core::ffi::c_void;
    fn kzalloc(size: usize, flags: gfp_t) -> *mut core::ffi::c_void;
    fn bug_on(condition: bool);
    fn assertion(condition: bool);
}

const EEXIST: i32 = 17;
const ENOMEM: i32 = 12;
const RB_ROOT: rb_root = rb_root { rb_node: core::ptr::null_mut() };

/*
 * Freshly initialize a ulist.
 *
 * @ulist:	the ulist to initialize
 *
 * Note: don't use this function to init an already used ulist, use
 * ulist_reinit instead.
 */
pub unsafe fn ulist_init(ulist: *mut ulist) {
    INIT_LIST_HEAD(core::ptr::addr_of_mut!((*ulist).nodes));
    (*ulist).root = RB_ROOT;
    (*ulist).nnodes = 0;
    (*ulist).prealloc = core::ptr::null_mut();
}

/* Free up additionally allocated memory for the ulist. */
pub unsafe fn ulist_release(ulist: *mut ulist) {
    let mut node = (*ulist).nodes.next;
    while node != core::ptr::addr_of_mut!((*ulist).nodes) {
        let next = (*node).next;
        kfree(node as *mut core::ffi::c_void);
        node = next;
    }
    kfree((*ulist).prealloc as *mut core::ffi::c_void);
    (*ulist).prealloc = core::ptr::null_mut();
    (*ulist).root = RB_ROOT;
    INIT_LIST_HEAD(core::ptr::addr_of_mut!((*ulist).nodes));
}

/* Prepare a ulist for reuse. */
pub unsafe fn ulist_reinit(ulist: *mut ulist) { ulist_release(ulist); ulist_init(ulist); }

/* Dynamically allocate a ulist. */
pub unsafe fn ulist_alloc(gfp_mask: gfp_t) -> *mut ulist {
    let ulist = kmalloc(core::mem::size_of::<ulist>(), gfp_mask) as *mut ulist;
    if ulist.is_null() { return core::ptr::null_mut(); }
    ulist_init(ulist);
    ulist
}

pub unsafe fn ulist_prealloc(ulist: *mut ulist, gfp_mask: gfp_t) {
    if (*ulist).prealloc.is_null() {
        (*ulist).prealloc = kzalloc(core::mem::size_of::<ulist_node>(), gfp_mask) as *mut ulist_node;
    }
}

/* Free dynamically allocated ulist. */
pub unsafe fn ulist_free(ulist: *mut ulist) {
    if ulist.is_null() { return; }
    ulist_release(ulist);
    kfree(ulist as *mut core::ffi::c_void);
}

unsafe extern "C" fn ulist_node_val_key_cmp(key: *const core::ffi::c_void, node: *const rb_node) -> i32 {
    let val = key as *const u64;
    let unode = (node as *const u8).sub(core::mem::offset_of!(ulist_node, rb_node)) as *const ulist_node;
    if (*unode).val < *val { 1 } else if (*unode).val > *val { -1 } else { 0 }
}

unsafe fn ulist_rbtree_search(ulist: *mut ulist, val: u64) -> *mut ulist_node {
    let node = rb_find(&val, core::ptr::addr_of!((*ulist).root), ulist_node_val_key_cmp);
    if node.is_null() { core::ptr::null_mut() } else { (node as *mut u8).sub(core::mem::offset_of!(ulist_node, rb_node)) as *mut ulist_node }
}

unsafe fn ulist_rbtree_erase(ulist: *mut ulist, node: *mut ulist_node) {
    rb_erase(core::ptr::addr_of_mut!((*node).rb_node), core::ptr::addr_of_mut!((*ulist).root));
    list_del(core::ptr::addr_of_mut!((*node).list));
    kfree(node as *mut core::ffi::c_void);
    bug_on((*ulist).nnodes == 0);
    (*ulist).nnodes -= 1;
}

unsafe extern "C" fn ulist_node_val_cmp(new: *mut rb_node, existing: *const rb_node) -> i32 {
    let unode = (new as *mut u8).sub(core::mem::offset_of!(ulist_node, rb_node)) as *mut ulist_node;
    ulist_node_val_key_cmp(core::ptr::addr_of!((*unode).val) as *const core::ffi::c_void, existing)
}

unsafe fn ulist_rbtree_insert(ulist: *mut ulist, ins: *mut ulist_node) -> i32 {
    let node = rb_find_add(core::ptr::addr_of_mut!((*ins).rb_node), core::ptr::addr_of_mut!((*ulist).root), ulist_node_val_cmp);
    if !node.is_null() { -EEXIST } else { 0 }
}

pub unsafe fn ulist_add(ulist: *mut ulist, val: u64, aux: u64, gfp_mask: gfp_t) -> i32 {
    ulist_add_merge(ulist, val, aux, core::ptr::null_mut(), gfp_mask)
}

pub unsafe fn ulist_add_merge(ulist: *mut ulist, val: u64, aux: u64, old_aux: *mut u64, gfp_mask: gfp_t) -> i32 {
    let mut node = ulist_rbtree_search(ulist, val);
    if !node.is_null() {
        if !old_aux.is_null() { *old_aux = (*node).aux; }
        return 0;
    }
    if !(*ulist).prealloc.is_null() {
        node = (*ulist).prealloc;
        (*ulist).prealloc = core::ptr::null_mut();
    } else {
        node = kmalloc(core::mem::size_of::<ulist_node>(), gfp_mask) as *mut ulist_node;
        if node.is_null() { return -ENOMEM; }
    }
    (*node).val = val;
    (*node).aux = aux;
    let ret = ulist_rbtree_insert(ulist, node);
    assertion(ret == 0);
    list_add_tail(core::ptr::addr_of_mut!((*node).list), core::ptr::addr_of_mut!((*ulist).nodes));
    (*ulist).nnodes += 1;
    1
}

pub unsafe fn ulist_del(ulist: *mut ulist, val: u64, aux: u64) -> i32 {
    let node = ulist_rbtree_search(ulist, val);
    if node.is_null() || (*node).aux != aux { return 1; }
    ulist_rbtree_erase(ulist, node);
    0
}

pub unsafe fn ulist_next(ulist: *const ulist, uiter: *mut ulist_iterator) -> *mut ulist_node {
    if list_empty(core::ptr::addr_of!((*ulist).nodes)) { return core::ptr::null_mut(); }
    if !(*uiter).cur_list.is_null() && (*uiter).cur_list.read().next == core::ptr::addr_of!((*ulist).nodes) as *mut list_head { return core::ptr::null_mut(); }
    if !(*uiter).cur_list.is_null() { (*uiter).cur_list = (*uiter).cur_list.read().next; } else { (*uiter).cur_list = (*ulist).nodes.next; }
    ((*uiter).cur_list as *mut u8).sub(core::mem::offset_of!(ulist_node, list)) as *mut ulist_node
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
