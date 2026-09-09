// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// Translated from range_tree.c. Kernel interval-tree, rbtree, allocator, and
// range_tree definitions are supplied by external dependencies.

use core::ffi::c_void;

#[repr(C)]
pub struct range_node {
    pub rn_rbnode: rb_node,
    pub rb_range_size: rb_node,
    pub rn_start: u32,
    pub rn_last: u32,
    pub __rn_subtree_last: u32,
}

extern "C" {
    fn rb_to_range_node(rb: *mut rb_node) -> *mut range_node;
    fn rn_size(rn: *mut range_node) -> u32;
    fn __range_size_insert(rn: *mut range_node, root: *mut rb_root_cached);
    fn __range_it_insert(rn: *mut range_node, root: *mut rb_root_cached);
    fn __range_it_remove(rn: *mut range_node, root: *mut rb_root_cached);
    fn __range_it_iter_first(root: *mut rb_root_cached, start: u32, last: u32)
        -> *mut range_node;
    fn rb_erase_cached(node: *mut rb_node, root: *mut rb_root_cached);
    fn RB_CLEAR_NODE(node: *mut rb_node);
    fn kmalloc_nolock(size: usize, flags: u32, node: i32) -> *mut c_void;
    fn kfree_nolock(ptr: *mut range_node);
}

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root_cached {
    _private: [u8; 0],
}

#[repr(C)]
pub struct range_tree {
    pub it_root: rb_root_cached,
    pub range_size_root: rb_root_cached,
}

const __GFP_ACCOUNT: u32 = 0;
const NUMA_NO_NODE: i32 = -1;
const ENOENT: i64 = 2;
const ENOMEM: i32 = 12;
const ESRCH: i32 = 3;
const EFAULT: i32 = 14;

unsafe fn find_range(rt: *mut range_tree, len: u32) -> *mut range_node {
    let mut rb: *mut rb_node = core::ptr::null_mut();
    let mut best: *mut range_node = core::ptr::null_mut();
    // The rb_root layout and access are supplied by the kernel dependency.
    rb = (*rt).range_size_root._private_root_node();
    while !rb.is_null() {
        let rn = rb_to_range_node(rb);
        if len <= rn_size(rn) {
            best = rn;
            rb = (*rb)._right();
        } else {
            rb = (*rb)._left();
        }
    }
    best
}

pub unsafe fn range_tree_find(rt: *mut range_tree, len: u32) -> i64 {
    let rn = find_range(rt, len);
    if rn.is_null() { -ENOENT } else { (*rn).rn_start as i64 }
}

unsafe fn range_it_insert(rn: *mut range_node, rt: *mut range_tree) {
    __range_size_insert(rn, &mut (*rt).range_size_root);
    __range_it_insert(rn, &mut (*rt).it_root);
}

unsafe fn range_it_remove(rn: *mut range_node, rt: *mut range_tree) {
    rb_erase_cached(&mut (*rn).rb_range_size, &mut (*rt).range_size_root);
    RB_CLEAR_NODE(&mut (*rn).rb_range_size);
    __range_it_remove(rn, &mut (*rt).it_root);
}

unsafe fn range_it_iter_first(rt: *mut range_tree, start: u32, last: u32) -> *mut range_node {
    __range_it_iter_first(&mut (*rt).it_root, start, last)
}

pub unsafe fn range_tree_clear(rt: *mut range_tree, start: u32, len: u32) -> i32 {
    let last = start.wrapping_add(len).wrapping_sub(1);
    let mut rn;
    while { rn = range_it_iter_first(rt, start, last); !rn.is_null() } {
        if (*rn).rn_start < start && (*rn).rn_last > last {
            let old_last = (*rn).rn_last;
            range_it_remove(rn, rt);
            (*rn).rn_last = start.wrapping_sub(1);
            range_it_insert(rn, rt);
            let new_rn = kmalloc_nolock(core::mem::size_of::<range_node>(), __GFP_ACCOUNT, NUMA_NO_NODE) as *mut range_node;
            if new_rn.is_null() { return -ENOMEM; }
            (*new_rn).rn_start = last.wrapping_add(1);
            (*new_rn).rn_last = old_last;
            range_it_insert(new_rn, rt);
        } else if (*rn).rn_start < start {
            range_it_remove(rn, rt);
            (*rn).rn_last = start.wrapping_sub(1);
            range_it_insert(rn, rt);
        } else if (*rn).rn_last > last {
            range_it_remove(rn, rt);
            (*rn).rn_start = last.wrapping_add(1);
            range_it_insert(rn, rt);
            break;
        } else {
            range_it_remove(rn, rt);
            kfree_nolock(rn);
        }
    }
    0
}

pub unsafe fn is_range_tree_set(rt: *mut range_tree, start: u32, len: u32) -> i32 {
    let last = start.wrapping_add(len).wrapping_sub(1);
    let left = range_it_iter_first(rt, start, last);
    if !left.is_null() && (*left).rn_start <= start && (*left).rn_last >= last { 0 } else { -ESRCH }
}

pub unsafe fn range_tree_set(rt: *mut range_tree, start: u32, len: u32) -> i32 {
    let last = start.wrapping_add(len).wrapping_sub(1);
    let mut left = range_it_iter_first(rt, start, last);
    if !left.is_null() && (*left).rn_start <= start && (*left).rn_last >= last { return 0; }
    let err = range_tree_clear(rt, start, len);
    if err != 0 { return err; }
    left = range_it_iter_first(rt, start.wrapping_sub(1), start.wrapping_sub(1));
    if !left.is_null() && (*left).rn_last.wrapping_add(1) != start { return -EFAULT; }
    let right = range_it_iter_first(rt, last.wrapping_add(1), last.wrapping_add(1));
    if !right.is_null() && (*right).rn_start != last.wrapping_add(1) { return -EFAULT; }
    if !left.is_null() && !right.is_null() {
        range_it_remove(left, rt); range_it_remove(right, rt);
        (*left).rn_last = (*right).rn_last; range_it_insert(left, rt); kfree_nolock(right);
    } else if !left.is_null() {
        range_it_remove(left, rt); (*left).rn_last = last; range_it_insert(left, rt);
    } else if !right.is_null() {
        range_it_remove(right, rt); (*right).rn_start = start; range_it_insert(right, rt);
    } else {
        left = kmalloc_nolock(core::mem::size_of::<range_node>(), __GFP_ACCOUNT, NUMA_NO_NODE) as *mut range_node;
        if left.is_null() { return -ENOMEM; }
        (*left).rn_start = start; (*left).rn_last = last; range_it_insert(left, rt);
    }
    0
}

pub unsafe fn range_tree_destroy(rt: *mut range_tree) {
    loop {
        let rn = range_it_iter_first(rt, 0, u32::MAX);
        if rn.is_null() { break; }
        range_it_remove(rn, rt); kfree_nolock(rn);
    }
}

pub unsafe fn range_tree_init(rt: *mut range_tree) {
    (*rt).it_root = rb_root_cached::default_root();
    (*rt).range_size_root = rb_root_cached::default_root();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
