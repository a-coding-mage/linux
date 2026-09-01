// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::ptr;

type bool_ = bool;
type u32 = u32;
type u64 = u64;
type size_t = usize;
type loff_t = i64;

#[repr(C)]
pub struct bpf_rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_list_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_refcount {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_rb_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bin_attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct node_data {
    pub key: i64,
    pub list_data: i64,
    pub r: bpf_rb_node,
    pub l: bpf_list_node,
    pub ref_: bpf_refcount,
}

#[repr(C)]
pub struct map_value {
    pub node: *mut node_data,
}

#[repr(C)]
pub struct node_acquire {
    pub key: i64,
    pub data: i64,
    pub node: bpf_rb_node,
    pub refcount: bpf_refcount,
}

#[repr(C)]
pub struct uninit_head_val {
    pub lock: bpf_spin_lock,
    pub head: bpf_list_head,
}

#[repr(C)]
pub struct StashedNodesMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct UninitHeadMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PercpuHashMap {
    _private: [u8; 0],
}

/* C map definitions:
 * struct { __uint(type, BPF_MAP_TYPE_ARRAY); __type(key, int);
 * __type(value, struct map_value); __uint(max_entries, 2); } stashed_nodes SEC(".maps");
 * struct { __uint(type, BPF_MAP_TYPE_ARRAY); __type(key, int);
 * __type(value, struct uninit_head_val); __uint(max_entries, 1); } uninit_head_map SEC(".maps");
 * struct { __uint(type, BPF_MAP_TYPE_PERCPU_HASH); __type(key, int);
 * __type(value, struct map_value); __uint(max_entries, 1); } percpu_hash SEC(".maps");
 */
extern "C" {
    static mut stashed_nodes: StashedNodesMap;
    static mut uninit_head_map: UninitHeadMap;
    static mut percpu_hash: PercpuHashMap;

    fn bpf_rcu_read_lock();
    fn bpf_rcu_read_unlock();
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_rbtree_add(
        root: *mut bpf_rb_root,
        node: *mut bpf_rb_node,
        less: unsafe extern "C" fn(*mut bpf_rb_node, *const bpf_rb_node) -> bool_,
    ) -> i64;
    fn bpf_rbtree_first(root: *mut bpf_rb_root) -> *mut bpf_rb_node;
    fn bpf_rbtree_remove(root: *mut bpf_rb_root, node: *mut bpf_rb_node) -> *mut bpf_rb_node;
    fn bpf_list_push_front(head: *mut bpf_list_head, node: *mut bpf_list_node) -> i64;
    fn bpf_list_push_back(head: *mut bpf_list_head, node: *mut bpf_list_node) -> i64;
    fn bpf_list_pop_front(head: *mut bpf_list_head) -> *mut bpf_list_node;
    fn bpf_list_empty(head: *mut bpf_list_head) -> bool_;
    fn bpf_list_is_first(head: *mut bpf_list_head, node: *mut bpf_list_node) -> bool_;
    fn bpf_list_is_last(head: *mut bpf_list_head, node: *mut bpf_list_node) -> bool_;
    fn bpf_list_front(head: *mut bpf_list_head) -> *mut bpf_list_node;
    fn bpf_list_back(head: *mut bpf_list_head) -> *mut bpf_list_node;
    fn bpf_list_del(head: *mut bpf_list_head, node: *mut bpf_list_node) -> *mut bpf_list_node;
    fn bpf_list_add(
        head: *mut bpf_list_head,
        node: *mut bpf_list_node,
        prev: *mut bpf_list_node,
    ) -> i64;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_map_lookup_percpu_elem(map: *mut c_void, key: *const c_void, cpu: u64) -> *mut c_void;
    fn bpf_probe_read_kernel(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> i64;
}

unsafe fn bpf_obj_new_node_data() -> *mut node_data {
    MaybeUninit::<*mut node_data>::zeroed().assume_init()
}

unsafe fn bpf_obj_new_node_acquire() -> *mut node_acquire {
    MaybeUninit::<*mut node_acquire>::zeroed().assume_init()
}

unsafe fn bpf_refcount_acquire_node_data(n: *mut node_data) -> *mut node_data {
    n
}

unsafe fn bpf_refcount_acquire_node_acquire(n: *mut node_acquire) -> *mut node_acquire {
    n
}

unsafe fn bpf_obj_drop<T>(_p: *mut T) {}

unsafe fn bpf_kptr_xchg_node_data(slot: *mut *mut node_data, n: *mut node_data) -> *mut node_data {
    let old = *slot;
    *slot = n;
    old
}

const fn offset_node_data_r() -> usize {
    core::mem::offset_of!(node_data, r)
}
const fn offset_node_data_l() -> usize {
    core::mem::offset_of!(node_data, l)
}
const fn offset_node_acquire_node() -> usize {
    core::mem::offset_of!(node_acquire, node)
}

unsafe fn node_data_from_r(p: *mut bpf_rb_node) -> *mut node_data {
    (p as *mut u8).sub(offset_node_data_r()) as *mut node_data
}
unsafe fn node_data_from_const_r(p: *const bpf_rb_node) -> *mut node_data {
    (p as *const u8).sub(offset_node_data_r()) as *mut node_data
}
unsafe fn node_data_from_l(p: *mut bpf_list_node) -> *mut node_data {
    (p as *mut u8).sub(offset_node_data_l()) as *mut node_data
}
unsafe fn node_acquire_from_node(p: *mut bpf_rb_node) -> *mut node_acquire {
    (p as *mut u8).sub(offset_node_acquire_node()) as *mut node_acquire
}
unsafe fn node_acquire_from_const_node(p: *const bpf_rb_node) -> *mut node_acquire {
    (p as *const u8).sub(offset_node_acquire_node()) as *mut node_acquire
}

/* private(name) SEC(".bss." #name) __hidden __attribute__((aligned(8))) */
#[repr(align(8))]
pub struct Align8<T>(pub T);

static mut lock: Align8<bpf_spin_lock> = Align8(bpf_spin_lock { _private: [] });
static mut root: Align8<bpf_rb_root> = Align8(bpf_rb_root { _private: [] });
static mut head: Align8<bpf_list_head> = Align8(bpf_list_head { _private: [] });

static mut alock: Align8<bpf_spin_lock> = Align8(bpf_spin_lock { _private: [] });
static mut aroot: Align8<bpf_rb_root> = Align8(bpf_rb_root { _private: [] });

static mut block: Align8<bpf_spin_lock> = Align8(bpf_spin_lock { _private: [] });
static mut broot: Align8<bpf_rb_root> = Align8(bpf_rb_root { _private: [] });

static mut ref_: Align8<u64> = Align8(0);

unsafe extern "C" fn less(node_a: *mut bpf_rb_node, node_b: *const bpf_rb_node) -> bool_ {
    let a = node_data_from_r(node_a);
    let b = node_data_from_const_r(node_b);
    (*a).key < (*b).key
}

unsafe extern "C" fn less_a(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool_ {
    let node_a = node_acquire_from_node(a);
    let node_b = node_acquire_from_const_node(b);
    (*node_a).key < (*node_b).key
}

unsafe fn __insert_in_tree_and_list(
    head: *mut bpf_list_head,
    root: *mut bpf_rb_root,
    lock: *mut bpf_spin_lock,
) -> i64 {
    let n = bpf_obj_new_node_data();
    if n.is_null() {
        return -1;
    }

    let m = bpf_refcount_acquire_node_data(n);
    (*m).key = 123;
    (*m).list_data = 456;

    bpf_spin_lock(lock);
    if bpf_rbtree_add(root, &mut (*n).r, less) != 0 {
        /* Failure to insert - unexpected */
        bpf_spin_unlock(lock);
        bpf_obj_drop(m);
        return -2;
    }
    bpf_spin_unlock(lock);

    bpf_spin_lock(lock);
    if bpf_list_push_front(head, &mut (*m).l) != 0 {
        /* Failure to insert - unexpected */
        bpf_spin_unlock(lock);
        return -3;
    }
    bpf_spin_unlock(lock);
    0
}

unsafe fn __stash_map_insert_tree(
    idx: i32,
    val: i32,
    root: *mut bpf_rb_root,
    lock: *mut bpf_spin_lock,
) -> i64 {
    let mapval = bpf_map_lookup_elem(
        &mut stashed_nodes as *mut _ as *mut c_void,
        &idx as *const _ as *const c_void,
    ) as *mut map_value;
    if mapval.is_null() {
        return -1;
    }

    let mut n = bpf_obj_new_node_data();
    if n.is_null() {
        return -2;
    }

    (*n).key = val as i64;
    let m = bpf_refcount_acquire_node_data(n);

    n = bpf_kptr_xchg_node_data(&mut (*mapval).node, n);
    if !n.is_null() {
        bpf_obj_drop(n);
        bpf_obj_drop(m);
        return -3;
    }

    bpf_spin_lock(lock);
    if bpf_rbtree_add(root, &mut (*m).r, less) != 0 {
        /* Failure to insert - unexpected */
        bpf_spin_unlock(lock);
        return -4;
    }
    bpf_spin_unlock(lock);
    0
}

unsafe fn __read_from_tree(
    root: *mut bpf_rb_root,
    lock: *mut bpf_spin_lock,
    remove_from_tree: bool_,
) -> i64 {
    let mut res: i64 = -99;

    bpf_spin_lock(lock);

    let mut rb = bpf_rbtree_first(root);
    if rb.is_null() {
        bpf_spin_unlock(lock);
        return -1;
    }

    let mut n = node_data_from_r(rb);
    res = (*n).key;

    if !remove_from_tree {
        bpf_spin_unlock(lock);
        return res;
    }

    rb = bpf_rbtree_remove(root, rb);
    bpf_spin_unlock(lock);
    if rb.is_null() {
        return -2;
    }
    n = node_data_from_r(rb);
    bpf_obj_drop(n);
    res
}

unsafe fn __read_from_list(
    head: *mut bpf_list_head,
    lock: *mut bpf_spin_lock,
    remove_from_list: bool_,
) -> i64 {
    let mut res: i64 = -99;

    bpf_spin_lock(lock);

    let l = bpf_list_pop_front(head);
    if l.is_null() {
        bpf_spin_unlock(lock);
        return -1;
    }

    let n = node_data_from_l(l);
    res = (*n).list_data;

    if !remove_from_list {
        if bpf_list_push_back(head, &mut (*n).l) != 0 {
            bpf_spin_unlock(lock);
            return -2;
        }
    }

    bpf_spin_unlock(lock);

    if remove_from_list {
        bpf_obj_drop(n);
    }
    res
}

unsafe fn __read_from_unstash(idx: i32) -> i64 {
    let mut n: *mut node_data = ptr::null_mut();
    let mut val: i64 = -99;

    let mapval = bpf_map_lookup_elem(
        &mut stashed_nodes as *mut _ as *mut c_void,
        &idx as *const _ as *const c_void,
    ) as *mut map_value;
    if mapval.is_null() {
        return -1;
    }

    n = bpf_kptr_xchg_node_data(&mut (*mapval).node, n);
    if n.is_null() {
        return -2;
    }

    val = (*n).key;
    bpf_obj_drop(n);
    val
}

unsafe fn insert_and_remove_tree_list_impl(rem_tree: bool_, rem_list: bool_) -> i64 {
    let mut err: i64;
    let tree_data: i64;
    let list_data: i64;

    err = __insert_in_tree_and_list(&mut head.0, &mut root.0, &mut lock.0);
    if err != 0 {
        return err;
    }

    err = __read_from_tree(&mut root.0, &mut lock.0, rem_tree);
    if err < 0 {
        return err;
    } else {
        tree_data = err;
    }

    err = __read_from_list(&mut head.0, &mut lock.0, rem_list);
    if err < 0 {
        return err;
    } else {
        list_data = err;
    }

    tree_data + list_data
}

/* After successful insert of struct node_data into both collections:
 *   - it should have refcount = 2
 *   - removing / not removing the node_data from a collection after
 *     reading should have no effect on ability to read / remove from
 *     the other collection
 */
/* SEC("tc") __description("insert_read_both: remove from tree + list") __success __retval(579) */
pub unsafe extern "C" fn insert_and_remove_tree_true_list_true(_ctx: *mut c_void) -> i64 {
    insert_and_remove_tree_list_impl(true, true)
}
/* SEC("tc") __description("insert_read_both: remove from neither") __success __retval(579) */
pub unsafe extern "C" fn insert_and_remove_tree_false_list_false(_ctx: *mut c_void) -> i64 {
    insert_and_remove_tree_list_impl(false, false)
}
/* SEC("tc") __description("insert_read_both: remove from tree") __success __retval(579) */
pub unsafe extern "C" fn insert_and_remove_tree_true_list_false(_ctx: *mut c_void) -> i64 {
    insert_and_remove_tree_list_impl(true, false)
}
/* SEC("tc") __description("insert_read_both: remove from list") __success __retval(579) */
pub unsafe extern "C" fn insert_and_remove_tree_false_list_true(_ctx: *mut c_void) -> i64 {
    insert_and_remove_tree_list_impl(false, true)
}

unsafe fn insert_and_remove_lf_tree_list_impl(rem_tree: bool_, rem_list: bool_) -> i64 {
    let mut err: i64;
    let tree_data: i64;
    let list_data: i64;

    err = __insert_in_tree_and_list(&mut head.0, &mut root.0, &mut lock.0);
    if err != 0 {
        return err;
    }

    err = __read_from_list(&mut head.0, &mut lock.0, rem_list);
    if err < 0 {
        return err;
    } else {
        list_data = err;
    }

    err = __read_from_tree(&mut root.0, &mut lock.0, rem_tree);
    if err < 0 {
        return err;
    } else {
        tree_data = err;
    }

    tree_data + list_data
}

/* Similar to insert_read_both, but list data is read and possibly removed
 * first
 *
 * Results should be no different than reading and possibly removing rbtree
 * node first
 */
/* SEC("tc") __description("insert_read_both_list_first: remove from tree + list") __success __retval(579) */
pub unsafe extern "C" fn insert_and_remove_lf_tree_true_list_true(_ctx: *mut c_void) -> i64 {
    insert_and_remove_lf_tree_list_impl(true, true)
}
/* SEC("tc") __description("insert_read_both_list_first: remove from neither") __success __retval(579) */
pub unsafe extern "C" fn insert_and_remove_lf_tree_false_list_false(_ctx: *mut c_void) -> i64 {
    insert_and_remove_lf_tree_list_impl(false, false)
}
/* SEC("tc") __description("insert_read_both_list_first: remove from tree") __success __retval(579) */
pub unsafe extern "C" fn insert_and_remove_lf_tree_true_list_false(_ctx: *mut c_void) -> i64 {
    insert_and_remove_lf_tree_list_impl(true, false)
}
/* SEC("tc") __description("insert_read_both_list_first: remove from list") __success __retval(579) */
pub unsafe extern "C" fn insert_and_remove_lf_tree_false_list_true(_ctx: *mut c_void) -> i64 {
    insert_and_remove_lf_tree_list_impl(false, true)
}

unsafe fn insert_double_read_from_tree_and_del_root_impl() -> i64 {
    let mut err: i64;
    let list_data: i64;

    err = __insert_in_tree_and_list(&mut head.0, &mut root.0, &mut lock.0);
    if err != 0 {
        return err;
    }

    err = __read_from_tree(&mut root.0, &mut lock.0, true);
    if err < 0 {
        return err;
    } else {
        list_data = err;
    }

    err = __read_from_tree(&mut root.0, &mut lock.0, true);
    if err < 0 {
        return err;
    }

    err + list_data
}

unsafe fn insert_double_read_from_list_and_del_head_impl() -> i64 {
    let mut err: i64;
    let list_data: i64;

    err = __insert_in_tree_and_list(&mut head.0, &mut root.0, &mut lock.0);
    if err != 0 {
        return err;
    }

    err = __read_from_list(&mut head.0, &mut lock.0, true);
    if err < 0 {
        return err;
    } else {
        list_data = err;
    }

    err = __read_from_list(&mut head.0, &mut lock.0, true);
    if err < 0 {
        return err;
    }

    err + list_data
}

/* Insert into both tree and list, then try reading-and-removing from either twice
 *
 * The second read-and-remove should fail on read step since the node has
 * already been removed
 */
/* SEC("tc") __description("insert_double_del: 2x read-and-del from tree") __success __retval(-1) */
pub unsafe extern "C" fn insert_double___read_from_tree_and_del_root(_ctx: *mut c_void) -> i64 {
    insert_double_read_from_tree_and_del_root_impl()
}
/* SEC("tc") __description("insert_double_del: 2x read-and-del from list") __success __retval(-1) */
pub unsafe extern "C" fn insert_double___read_from_list_and_del_head(_ctx: *mut c_void) -> i64 {
    insert_double_read_from_list_and_del_head_impl()
}

unsafe fn insert_rbtree_and_stash_del_tree_impl(rem_tree: bool_) -> i64 {
    let mut err: i64;
    let tree_data: i64;
    let map_data: i64;

    err = __stash_map_insert_tree(0, 42, &mut root.0, &mut lock.0);
    if err != 0 {
        return err;
    }

    err = __read_from_tree(&mut root.0, &mut lock.0, rem_tree);
    if err < 0 {
        return err;
    } else {
        tree_data = err;
    }

    err = __read_from_unstash(0);
    if err < 0 {
        return err;
    } else {
        map_data = err;
    }

    tree_data + map_data
}

/* Stash a refcounted node in map_val, insert same node into tree, then try
 * reading data from tree then unstashed map_val, possibly removing from tree
 *
 * Removing from tree should have no effect on map_val kptr validity
 */
/* SEC("tc") __description("insert_stash_read: remove from tree") __success __retval(84) */
pub unsafe extern "C" fn insert_rbtree_and_stash__del_tree_true(_ctx: *mut c_void) -> i64 {
    insert_rbtree_and_stash_del_tree_impl(true)
}
/* SEC("tc") __description("insert_stash_read: don't remove from tree") __success __retval(84) */
pub unsafe extern "C" fn insert_rbtree_and_stash__del_tree_false(_ctx: *mut c_void) -> i64 {
    insert_rbtree_and_stash_del_tree_impl(false)
}

/* SEC("tc") __description("list_empty_test: list empty before add, non-empty after add") __success __retval(0) */
pub unsafe extern "C" fn list_empty_test(_ctx: *mut c_void) -> i32 {
    bpf_spin_lock(&mut lock.0);
    if !bpf_list_empty(&mut head.0) {
        bpf_spin_unlock(&mut lock.0);
        return -1;
    }
    bpf_spin_unlock(&mut lock.0);

    let node_new = bpf_obj_new_node_data();
    if node_new.is_null() {
        return -2;
    }

    bpf_spin_lock(&mut lock.0);
    bpf_list_push_front(&mut head.0, &mut (*node_new).l);

    if bpf_list_empty(&mut head.0) {
        bpf_spin_unlock(&mut lock.0);
        return -3;
    }
    bpf_spin_unlock(&mut lock.0);
    0
}

unsafe fn __add_in_list(head: *mut bpf_list_head, lock: *mut bpf_spin_lock) -> *mut node_data {
    let node_new = bpf_obj_new_node_data();
    if node_new.is_null() {
        return ptr::null_mut();
    }

    let node_ref = bpf_refcount_acquire_node_data(node_new);

    bpf_spin_lock(lock);
    bpf_list_push_front(head, &mut (*node_new).l);
    bpf_spin_unlock(lock);
    node_ref
}

/* SEC("tc") __description("list_is_edge_test1: is_first on first node, is_last on last node") __success __retval(0) */
pub unsafe extern "C" fn list_is_edge_test1(_ctx: *mut c_void) -> i32 {
    let mut err: i32 = 0;

    let node_last = __add_in_list(&mut head.0, &mut lock.0);
    if node_last.is_null() {
        return -1;
    }

    let node_first = __add_in_list(&mut head.0, &mut lock.0);
    if node_first.is_null() {
        bpf_obj_drop(node_last);
        return -2;
    }

    bpf_spin_lock(&mut lock.0);
    if !bpf_list_is_first(&mut head.0, &mut (*node_first).l) {
        err = -3;
    } else if !bpf_list_is_last(&mut head.0, &mut (*node_last).l) {
        err = -4;
    }

    bpf_spin_unlock(&mut lock.0);
    bpf_obj_drop(node_first);
    bpf_obj_drop(node_last);
    err
}

/* SEC("tc") __description("list_is_edge_test2: accept list_front/list_back return value") __success __retval(0) */
pub unsafe extern "C" fn list_is_edge_test2(_ctx: *mut c_void) -> i32 {
    let mut err: i64 = 0;

    let a = __add_in_list(&mut head.0, &mut lock.0);
    if a.is_null() {
        return -1;
    }

    let b = __add_in_list(&mut head.0, &mut lock.0);
    if b.is_null() {
        bpf_obj_drop(a);
        return -2;
    }

    bpf_spin_lock(&mut lock.0);
    let front = bpf_list_front(&mut head.0);
    let back = bpf_list_back(&mut head.0);
    if front.is_null() || back.is_null() {
        err = -3;
    } else if !bpf_list_is_first(&mut head.0, front) || bpf_list_is_last(&mut head.0, front) {
        err = -4;
    } else if !bpf_list_is_last(&mut head.0, back) || bpf_list_is_first(&mut head.0, back) {
        err = -5;
    }

    bpf_spin_unlock(&mut lock.0);
    bpf_obj_drop(a);
    bpf_obj_drop(b);
    err as i32
}

/* SEC("tc") __description("list_is_edge_test3: single node is both first and last") __success __retval(0) */
pub unsafe extern "C" fn list_is_edge_test3(_ctx: *mut c_void) -> i32 {
    let mut err: i64 = 0;

    let tmp = __add_in_list(&mut head.0, &mut lock.0);
    if tmp.is_null() {
        return -1;
    }

    bpf_spin_lock(&mut lock.0);
    let node = bpf_list_front(&mut head.0);
    if node.is_null() {
        bpf_spin_unlock(&mut lock.0);
        bpf_obj_drop(tmp);
        return -2;
    }

    if !bpf_list_is_first(&mut head.0, node) || !bpf_list_is_last(&mut head.0, node) {
        err = -3;
    }
    bpf_spin_unlock(&mut lock.0);

    bpf_obj_drop(tmp);
    err as i32
}

/* SEC("tc") __description("list_del_test1: del returns removed nodes") __success __retval(0) */
pub unsafe extern "C" fn list_del_test1(_ctx: *mut c_void) -> i32 {
    let mut err: i32 = 0;

    let node_last = __add_in_list(&mut head.0, &mut lock.0);
    if node_last.is_null() {
        return -1;
    }

    let node_first = __add_in_list(&mut head.0, &mut lock.0);
    if node_first.is_null() {
        bpf_obj_drop(node_last);
        return -2;
    }

    bpf_spin_lock(&mut lock.0);
    let bpf_node_last = bpf_list_del(&mut head.0, &mut (*node_last).l);
    let bpf_node_first = bpf_list_del(&mut head.0, &mut (*node_first).l);
    bpf_spin_unlock(&mut lock.0);

    if !bpf_node_first.is_null() {
        bpf_obj_drop(node_data_from_l(bpf_node_first));
    } else {
        err = -3;
    }

    if !bpf_node_last.is_null() {
        bpf_obj_drop(node_data_from_l(bpf_node_last));
    } else {
        err = -4;
    }

    bpf_obj_drop(node_first);
    bpf_obj_drop(node_last);
    err
}

/* SEC("tc") __description("list_del_test2: remove an arbitrary node from the list") __success __retval(0) */
pub unsafe extern "C" fn list_del_test2(_ctx: *mut c_void) -> i32 {
    let err = __insert_in_tree_and_list(&mut head.0, &mut root.0, &mut lock.0);
    if err != 0 {
        return err as i32;
    }

    bpf_spin_lock(&mut lock.0);
    let mut rb = bpf_rbtree_first(&mut root.0);
    if rb.is_null() {
        bpf_spin_unlock(&mut lock.0);
        return -4;
    }

    rb = bpf_rbtree_remove(&mut root.0, rb);
    if rb.is_null() {
        bpf_spin_unlock(&mut lock.0);
        return -5;
    }

    let n = node_data_from_r(rb);
    let l = bpf_list_del(&mut head.0, &mut (*n).l);
    bpf_spin_unlock(&mut lock.0);
    bpf_obj_drop(n);
    if l.is_null() {
        return -6;
    }

    bpf_obj_drop(node_data_from_l(l));
    0
}

/* SEC("tc") __description("list_del_test3: list_del accepts list_front return value as node") __success __retval(0) */
pub unsafe extern "C" fn list_del_test3(_ctx: *mut c_void) -> i32 {
    let mut err: i64 = 0;

    let tmp = __add_in_list(&mut head.0, &mut lock.0);
    if tmp.is_null() {
        return -1;
    }

    bpf_spin_lock(&mut lock.0);
    let bpf_node = bpf_list_front(&mut head.0);
    if bpf_node.is_null() {
        bpf_spin_unlock(&mut lock.0);
        err = -2;
        bpf_obj_drop(tmp);
        return err as i32;
    }

    let l = bpf_list_del(&mut head.0, bpf_node);
    bpf_spin_unlock(&mut lock.0);
    if l.is_null() {
        err = -3;
        bpf_obj_drop(tmp);
        return err as i32;
    }

    bpf_obj_drop(node_data_from_l(l));
    bpf_obj_drop(tmp);
    0
}

/* SEC("tc") __description("list_add_test1: insert new node after prev") __success __retval(0) */
pub unsafe extern "C" fn list_add_test1(_ctx: *mut c_void) -> i32 {
    let mut err: i64 = 0;

    let node_first = __add_in_list(&mut head.0, &mut lock.0);
    if node_first.is_null() {
        return -1;
    }

    let new_node = bpf_obj_new_node_data();
    if new_node.is_null() {
        err = -2;
        bpf_obj_drop(node_first);
        return err as i32;
    }

    bpf_spin_lock(&mut lock.0);
    err = bpf_list_add(&mut head.0, &mut (*new_node).l, &mut (*node_first).l);
    bpf_spin_unlock(&mut lock.0);
    if err != 0 {
        err = -3;
    }

    bpf_obj_drop(node_first);
    err as i32
}

/* SEC("tc") __description("list_add_test2: list_add accepts list_front return value as prev") __success __retval(0) */
pub unsafe extern "C" fn list_add_test2(_ctx: *mut c_void) -> i32 {
    let mut err: i64 = 0;

    let tmp = __add_in_list(&mut head.0, &mut lock.0);
    if tmp.is_null() {
        return -1;
    }

    let new_node = bpf_obj_new_node_data();
    if new_node.is_null() {
        err = -2;
        bpf_obj_drop(tmp);
        return err as i32;
    }

    bpf_spin_lock(&mut lock.0);
    let bpf_node = bpf_list_front(&mut head.0);
    if bpf_node.is_null() {
        bpf_spin_unlock(&mut lock.0);
        bpf_obj_drop(new_node);
        err = -3;
        bpf_obj_drop(tmp);
        return err as i32;
    }

    err = bpf_list_add(&mut head.0, &mut (*new_node).l, bpf_node);
    bpf_spin_unlock(&mut lock.0);
    if err != 0 {
        err = -4;
    }

    bpf_obj_drop(tmp);
    err as i32
}

/* SEC("tc") __description("list_push_back_uninit_head: push_back on 0-initialized list head") __success __retval(0) */
pub unsafe extern "C" fn list_push_back_uninit_head(_ctx: *mut c_void) -> i32 {
    let key: i32 = 0;
    let mut ret: i32 = -1;

    let st = bpf_map_lookup_elem(
        &mut uninit_head_map as *mut _ as *mut c_void,
        &key as *const _ as *const c_void,
    ) as *mut uninit_head_val;
    if st.is_null() {
        return -1;
    }

    let node = bpf_obj_new_node_data();
    if node.is_null() {
        return -1;
    }

    bpf_spin_lock(&mut (*st).lock);
    ret = bpf_list_push_back(&mut (*st).head, &mut (*node).l) as i32;
    bpf_spin_unlock(&mut (*st).lock);

    ret
}

/* SEC("?tc") __failure __msg("bpf_spin_lock at off=32 must be held for bpf_list_head") */
pub unsafe extern "C" fn list_del_without_lock_fail(_ctx: *mut c_void) -> i64 {
    let n = bpf_obj_new_node_data();
    if n.is_null() {
        return -1;
    }

    /* Error case: delete list node without holding lock */
    let l = bpf_list_del(&mut head.0, &mut (*n).l);
    bpf_obj_drop(n);
    if l.is_null() {
        return -2;
    }
    bpf_obj_drop(node_data_from_l(l));

    0
}

/* SEC("?tc") __failure __msg("bpf_spin_lock at off=32 must be held for bpf_list_head") */
pub unsafe extern "C" fn list_add_without_lock_fail(_ctx: *mut c_void) -> i64 {
    let n = bpf_obj_new_node_data();
    if n.is_null() {
        return -1;
    }

    let prev = bpf_obj_new_node_data();
    if prev.is_null() {
        bpf_obj_drop(n);
        return -1;
    }

    /* Error case: add list node without holding lock */
    let err = bpf_list_add(&mut head.0, &mut (*n).l, &mut (*prev).l);
    bpf_obj_drop(prev);
    if err != 0 {
        return -2;
    }

    0
}

/* SEC("tc") __success */
pub unsafe extern "C" fn rbtree_refcounted_node_ref_escapes(_ctx: *mut c_void) -> i64 {
    let n = bpf_obj_new_node_acquire();
    if n.is_null() {
        return 1;
    }

    bpf_spin_lock(&mut alock.0);
    bpf_rbtree_add(&mut aroot.0, &mut (*n).node, less_a);
    let m = bpf_refcount_acquire_node_acquire(n);
    bpf_spin_unlock(&mut alock.0);
    if m.is_null() {
        return 2;
    }

    (*m).key = 2;
    bpf_obj_drop(m);
    0
}

/* SEC("tc") __success */
pub unsafe extern "C" fn rbtree_refcounted_node_ref_escapes_owning_input(_ctx: *mut c_void) -> i64 {
    let n = bpf_obj_new_node_acquire();
    if n.is_null() {
        return 1;
    }

    let m = bpf_refcount_acquire_node_acquire(n);
    (*m).key = 2;

    bpf_spin_lock(&mut alock.0);
    bpf_rbtree_add(&mut aroot.0, &mut (*n).node, less_a);
    bpf_spin_unlock(&mut alock.0);

    bpf_obj_drop(m);

    0
}

unsafe fn __stash_map_empty_xchg(mut n: *mut node_data, idx: i32) -> i64 {
    let mapval = bpf_map_lookup_elem(
        &mut stashed_nodes as *mut _ as *mut c_void,
        &idx as *const _ as *const c_void,
    ) as *mut map_value;

    if mapval.is_null() {
        bpf_obj_drop(n);
        return 1;
    }
    n = bpf_kptr_xchg_node_data(&mut (*mapval).node, n);
    if !n.is_null() {
        bpf_obj_drop(n);
        return 2;
    }
    0
}

/* SEC("tc") */
pub unsafe extern "C" fn rbtree_wrong_owner_remove_fail_a1(_ctx: *mut c_void) -> i64 {
    let n = bpf_obj_new_node_data();
    if n.is_null() {
        return 1;
    }
    let m = bpf_refcount_acquire_node_data(n);

    if __stash_map_empty_xchg(n, 0) != 0 {
        bpf_obj_drop(m);
        return 2;
    }

    if __stash_map_empty_xchg(m, 1) != 0 {
        return 3;
    }

    0
}

/* SEC("tc") */
pub unsafe extern "C" fn rbtree_wrong_owner_remove_fail_b(_ctx: *mut c_void) -> i64 {
    let idx: i32 = 0;

    let mapval = bpf_map_lookup_elem(
        &mut stashed_nodes as *mut _ as *mut c_void,
        &idx as *const _ as *const c_void,
    ) as *mut map_value;
    if mapval.is_null() {
        return 1;
    }

    let n = bpf_kptr_xchg_node_data(&mut (*mapval).node, ptr::null_mut());
    if n.is_null() {
        return 2;
    }

    bpf_spin_lock(&mut block.0);

    bpf_rbtree_add(&mut broot.0, &mut (*n).r, less);

    bpf_spin_unlock(&mut block.0);
    0
}

/* SEC("tc") */
pub unsafe extern "C" fn rbtree_wrong_owner_remove_fail_a2(_ctx: *mut c_void) -> i64 {
    let idx: i32 = 1;

    let mapval = bpf_map_lookup_elem(
        &mut stashed_nodes as *mut _ as *mut c_void,
        &idx as *const _ as *const c_void,
    ) as *mut map_value;
    if mapval.is_null() {
        return 1;
    }

    let m = bpf_kptr_xchg_node_data(&mut (*mapval).node, ptr::null_mut());
    if m.is_null() {
        return 2;
    }
    bpf_spin_lock(&mut lock.0);

    /* make m non-owning ref */
    bpf_list_push_back(&mut head.0, &mut (*m).l);
    let res = bpf_rbtree_remove(&mut root.0, &mut (*m).r);

    bpf_spin_unlock(&mut lock.0);
    if !res.is_null() {
        bpf_obj_drop(node_data_from_r(res));
        return 3;
    }
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") __success */
pub unsafe extern "C" fn rbtree_sleepable_rcu(
    _file: *mut file,
    _kobj: *mut kobject,
    _bin_attr: *mut bin_attribute,
    _buf: *mut i8,
    _off: loff_t,
    _len: size_t,
) -> i32 {
    let mut m: *mut node_data = ptr::null_mut();

    let n = bpf_obj_new_node_data();
    if n.is_null() {
        return 0;
    }

    bpf_rcu_read_lock();
    bpf_spin_lock(&mut lock.0);
    bpf_rbtree_add(&mut root.0, &mut (*n).r, less);
    let mut rb = bpf_rbtree_first(&mut root.0);
    if !rb.is_null() {
        rb = bpf_rbtree_remove(&mut root.0, rb);
        if !rb.is_null() {
            m = node_data_from_r(rb);
        }
    }

    bpf_spin_unlock(&mut lock.0);
    bpf_rcu_read_unlock();
    if !m.is_null() {
        bpf_obj_drop(m);
    }
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") __success */
pub unsafe extern "C" fn rbtree_sleepable_rcu_no_explicit_rcu_lock(
    _file: *mut file,
    _kobj: *mut kobject,
    _bin_attr: *mut bin_attribute,
    _buf: *mut i8,
    _off: loff_t,
    _len: size_t,
) -> i32 {
    let mut m: *mut node_data = ptr::null_mut();

    let n = bpf_obj_new_node_data();
    if n.is_null() {
        return 0;
    }

    /* No explicit bpf_rcu_read_lock */
    bpf_spin_lock(&mut lock.0);
    bpf_rbtree_add(&mut root.0, &mut (*n).r, less);
    let mut rb = bpf_rbtree_first(&mut root.0);
    if !rb.is_null() {
        rb = bpf_rbtree_remove(&mut root.0, rb);
        if !rb.is_null() {
            m = node_data_from_r(rb);
        }
    }

    bpf_spin_unlock(&mut lock.0);
    /* No explicit bpf_rcu_read_unlock */
    if !m.is_null() {
        bpf_obj_drop(m);
    }
    0
}

unsafe fn probe_read_refcount() -> i32 {
    let mut refcount: u32 = 0;

    bpf_probe_read_kernel(
        &mut refcount as *mut _ as *mut c_void,
        core::mem::size_of_val(&refcount) as u32,
        ref_.0 as *const c_void,
    );
    refcount as i32
}

unsafe fn __insert_in_list(
    head: *mut bpf_list_head,
    lock: *mut bpf_spin_lock,
    node: *mut *mut node_data,
) -> i32 {
    let node_new = bpf_obj_new_node_data();
    if node_new.is_null() {
        return -1;
    }

    let node_ref = bpf_refcount_acquire_node_data(node_new);
    let node_old = bpf_kptr_xchg_node_data(node, node_new);
    if !node_old.is_null() {
        bpf_obj_drop(node_old);
        bpf_obj_drop(node_ref);
        return -2;
    }

    bpf_spin_lock(lock);
    bpf_list_push_front(head, &mut (*node_ref).l);
    ref_.0 = &mut (*node_ref).ref_ as *mut _ as u64;
    bpf_spin_unlock(lock);
    probe_read_refcount()
}

/* SEC("tc") */
pub unsafe extern "C" fn percpu_hash_refcount_leak(_ctx: *mut c_void) -> i32 {
    let key: i32 = 0;

    let v = bpf_map_lookup_percpu_elem(
        &mut percpu_hash as *mut _ as *mut c_void,
        &key as *const _ as *const c_void,
        0,
    ) as *mut map_value;
    if v.is_null() {
        return 0;
    }

    __insert_in_list(&mut head.0, &mut lock.0, &mut (*v).node)
}

/* SEC("syscall") */
pub unsafe extern "C" fn clear_percpu_hash_kptr(_ctx: *mut c_void) -> i32 {
    let key: i32 = 0;

    let v = bpf_map_lookup_percpu_elem(
        &mut percpu_hash as *mut _ as *mut c_void,
        &key as *const _ as *const c_void,
        0,
    ) as *mut map_value;
    if v.is_null() {
        return 0;
    }

    let n = bpf_kptr_xchg_node_data(&mut (*v).node, ptr::null_mut());
    if n.is_null() {
        return 0;
    }
    bpf_obj_drop(n);
    probe_read_refcount()
}

/* SEC("tc") */
pub unsafe extern "C" fn check_percpu_hash_refcount(_ctx: *mut c_void) -> i32 {
    probe_read_refcount()
}

/* char _license[] SEC("license") = "GPL"; */
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
