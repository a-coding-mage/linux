/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Latched RB-trees
 *
 * Copyright (C) 2015 Intel Corp., Peter Zijlstra <peterz@infradead.org>
 *
 * Rust translation of the source header.  RB-tree, seqlock, RCU, and
 * container_of symbols are supplied by the corresponding external headers.
 */

#[repr(C)]
pub struct latch_tree_node {
    pub node: [rb_node; 2],
}

#[repr(C)]
pub struct latch_tree_root {
    pub seq: seqcount_latch_t,
    pub tree: [rb_root; 2],
}

#[repr(C)]
pub struct latch_tree_ops {
    pub less: Option<unsafe extern "C" fn(a: *mut latch_tree_node, b: *mut latch_tree_node) -> bool>,
    pub comp: Option<unsafe extern "C" fn(key: *mut core::ffi::c_void, b: *mut latch_tree_node) -> core::ffi::c_int>,
}

#[inline(always)]
unsafe fn __lt_from_rb(node: *mut rb_node, idx: core::ffi::c_int) -> *mut latch_tree_node {
    // Equivalent to container_of(node, struct latch_tree_node, node[idx]).
    let base = node as *mut u8;
    base.sub(core::mem::offset_of!(latch_tree_node, node))
        .sub((idx as usize) * core::mem::size_of::<rb_node>()) as *mut latch_tree_node
}

#[inline(always)]
unsafe fn __lt_insert(
    ltn: *mut latch_tree_node,
    ltr: *mut latch_tree_root,
    idx: core::ffi::c_int,
    less: Option<unsafe extern "C" fn(*mut latch_tree_node, *mut latch_tree_node) -> bool>,
) {
    let root: *mut rb_root = &mut (*ltr).tree[idx as usize];
    let mut link: *mut *mut rb_node = &mut (*root).rb_node;
    let node: *mut rb_node = &mut (*ltn).node[idx as usize];
    let mut parent: *mut rb_node = core::ptr::null_mut();
    let mut ltp: *mut latch_tree_node;

    while !(*link).is_null() {
        parent = *link;
        ltp = __lt_from_rb(parent, idx);

        if less.unwrap()(ltn, ltp) {
            link = &mut (*parent).rb_left;
        } else {
            link = &mut (*parent).rb_right;
        }
    }

    rb_link_node_rcu(node, parent, link);
    rb_insert_color(node, root);
}

#[inline(always)]
unsafe fn __lt_erase(ltn: *mut latch_tree_node, ltr: *mut latch_tree_root, idx: core::ffi::c_int) {
    rb_erase(&mut (*ltn).node[idx as usize], &mut (*ltr).tree[idx as usize]);
}

#[inline(always)]
unsafe fn __lt_find(
    key: *mut core::ffi::c_void,
    ltr: *mut latch_tree_root,
    idx: core::ffi::c_int,
    comp: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut latch_tree_node) -> core::ffi::c_int>,
) -> *mut latch_tree_node {
    let mut node = rcu_dereference_raw((*ltr).tree[idx as usize].rb_node);
    let mut ltn: *mut latch_tree_node;
    let mut c: core::ffi::c_int;

    while !node.is_null() {
        ltn = __lt_from_rb(node, idx);
        c = comp.unwrap()(key, ltn);

        if c < 0 {
            node = rcu_dereference_raw((*node).rb_left);
        } else if c > 0 {
            node = rcu_dereference_raw((*node).rb_right);
        } else {
            return ltn;
        }
    }

    core::ptr::null_mut()
}

#[inline(always)]
pub unsafe fn latch_tree_insert(node: *mut latch_tree_node, root: *mut latch_tree_root, ops: *const latch_tree_ops) {
    write_seqcount_latch_begin(&mut (*root).seq);
    __lt_insert(node, root, 0, (*ops).less);
    write_seqcount_latch(&mut (*root).seq);
    __lt_insert(node, root, 1, (*ops).less);
    write_seqcount_latch_end(&mut (*root).seq);
}

#[inline(always)]
pub unsafe fn latch_tree_erase(node: *mut latch_tree_node, root: *mut latch_tree_root, _ops: *const latch_tree_ops) {
    write_seqcount_latch_begin(&mut (*root).seq);
    __lt_erase(node, root, 0);
    write_seqcount_latch(&mut (*root).seq);
    __lt_erase(node, root, 1);
    write_seqcount_latch_end(&mut (*root).seq);
}

#[inline(always)]
pub unsafe fn latch_tree_find(key: *mut core::ffi::c_void, root: *mut latch_tree_root, ops: *const latch_tree_ops) -> *mut latch_tree_node {
    let mut node: *mut latch_tree_node;
    let mut seq: core::ffi::c_uint;

    loop {
        seq = read_seqcount_latch(&(*root).seq);
        node = __lt_find(key, root, (seq & 1) as core::ffi::c_int, (*ops).comp);
        if !read_seqcount_latch_retry(&(*root).seq, seq) {
            break;
        }
    }

    node
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
