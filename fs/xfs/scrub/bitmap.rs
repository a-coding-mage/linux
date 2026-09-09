// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2018-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// C dependencies supplied by the surrounding translation unit.

/* u64 bitmap */
#[repr(C)]
pub struct xbitmap64_node {
    pub bn_rbnode: rb_node,
    /* First set bit of this interval and subtree. */
    pub bn_start: u64,
    /* Last set bit of this interval. */
    pub bn_last: u64,
    /* Last set bit of this subtree.  Do not touch this. */
    pub __bn_subtree_last: u64,
}

/* Define our own interval tree type with uint64_t parameters. */

/* These functions are defined by INTERVAL_TREE_DEFINE in C. */
extern "C" {
    fn xbitmap64_tree_insert(node: *mut xbitmap64_node, root: *mut rb_root_cached);
    fn xbitmap64_tree_remove(node: *mut xbitmap64_node, root: *mut rb_root_cached);
    fn xbitmap64_tree_iter_first(root: *mut rb_root_cached, start: u64, last: u64)
        -> *mut xbitmap64_node;
    fn xbitmap64_tree_iter_next(node: *mut xbitmap64_node, start: u64, last: u64)
        -> *mut xbitmap64_node;
}

/* Iterate each interval of a bitmap.  Do not change the bitmap. */

/* Clear a range of this bitmap. */
pub unsafe fn xbitmap64_clear(
    bitmap: *mut xbitmap64,
    start: u64,
    len: u64,
) -> i32 {
    let last = start.wrapping_add(len).wrapping_sub(1);
    let mut bn: *mut xbitmap64_node;
    while {
        bn = xbitmap64_tree_iter_first(&mut (*bitmap).xb_root, start, last);
        !bn.is_null()
    } {
        if (*bn).bn_start < start && (*bn).bn_last > last {
            let old_last = (*bn).bn_last;
            xbitmap64_tree_remove(bn, &mut (*bitmap).xb_root);
            (*bn).bn_last = start.wrapping_sub(1);
            xbitmap64_tree_insert(bn, &mut (*bitmap).xb_root);
            let new_bn = kmalloc_xbitmap64_node();
            if new_bn.is_null() { return -12; }
            (*new_bn).bn_start = last.wrapping_add(1);
            (*new_bn).bn_last = old_last;
            xbitmap64_tree_insert(new_bn, &mut (*bitmap).xb_root);
        } else if (*bn).bn_start < start {
            xbitmap64_tree_remove(bn, &mut (*bitmap).xb_root);
            (*bn).bn_last = start.wrapping_sub(1);
            xbitmap64_tree_insert(bn, &mut (*bitmap).xb_root);
        } else if (*bn).bn_last > last {
            xbitmap64_tree_remove(bn, &mut (*bitmap).xb_root);
            (*bn).bn_start = last.wrapping_add(1);
            xbitmap64_tree_insert(bn, &mut (*bitmap).xb_root);
            break;
        } else {
            xbitmap64_tree_remove(bn, &mut (*bitmap).xb_root);
            kfree(bn as *mut core::ffi::c_void);
        }
    }
    0
}

/* Set a range of this bitmap. */
pub unsafe fn xbitmap64_set(bitmap: *mut xbitmap64, start: u64, len: u64) -> i32 {
    let last = start.wrapping_add(len).wrapping_sub(1);
    let mut left = xbitmap64_tree_iter_first(&mut (*bitmap).xb_root, start, last);
    if !left.is_null() && (*left).bn_start <= start && (*left).bn_last >= last { return 0; }
    let error = xbitmap64_clear(bitmap, start, len);
    if error != 0 { return error; }
    left = xbitmap64_tree_iter_first(&mut (*bitmap).xb_root, start.wrapping_sub(1), start.wrapping_sub(1));
    debug_assert!(left.is_null() || (*left).bn_last.wrapping_add(1) == start);
    let right = xbitmap64_tree_iter_first(&mut (*bitmap).xb_root, last.wrapping_add(1), last.wrapping_add(1));
    debug_assert!(right.is_null() || (*right).bn_start == last.wrapping_add(1));
    if !left.is_null() && !right.is_null() {
        xbitmap64_tree_remove(left, &mut (*bitmap).xb_root); xbitmap64_tree_remove(right, &mut (*bitmap).xb_root);
        (*left).bn_last = (*right).bn_last; xbitmap64_tree_insert(left, &mut (*bitmap).xb_root); kfree(right as *mut core::ffi::c_void);
    } else if !left.is_null() {
        xbitmap64_tree_remove(left, &mut (*bitmap).xb_root); (*left).bn_last = last; xbitmap64_tree_insert(left, &mut (*bitmap).xb_root);
    } else if !right.is_null() {
        xbitmap64_tree_remove(right, &mut (*bitmap).xb_root); (*right).bn_start = start; xbitmap64_tree_insert(right, &mut (*bitmap).xb_root);
    } else {
        left = kmalloc_xbitmap64_node(); if left.is_null() { return -12; }
        (*left).bn_start = start; (*left).bn_last = last; xbitmap64_tree_insert(left, &mut (*bitmap).xb_root);
    }
    0
}

pub unsafe fn xbitmap64_destroy(bitmap: *mut xbitmap64) {
    loop { let bn = xbitmap64_tree_iter_first(&mut (*bitmap).xb_root, 0, u64::MAX); if bn.is_null() { break; } xbitmap64_tree_remove(bn, &mut (*bitmap).xb_root); kfree(bn as *mut core::ffi::c_void); }
}

pub unsafe fn xbitmap64_init(bitmap: *mut xbitmap64) { (*bitmap).xb_root = RB_ROOT_CACHED; }

pub unsafe fn xbitmap64_disunion(bitmap: *mut xbitmap64, sub: *mut xbitmap64) -> i32 {
    if xbitmap64_empty(bitmap) || xbitmap64_empty(sub) { return 0; }
    let mut bn = rb_first_xbitmap64(sub);
    while !bn.is_null() { let error = xbitmap64_clear(bitmap, (*bn).bn_start, (*bn).bn_last.wrapping_sub((*bn).bn_start).wrapping_add(1)); if error != 0 { return error; } bn = rb_next_xbitmap64(bn); }
    0
}

pub unsafe fn xbitmap64_hweight(bitmap: *mut xbitmap64) -> u64 { let mut ret = 0; let mut bn = rb_first_xbitmap64(bitmap); while !bn.is_null() { ret = ret.wrapping_add((*bn).bn_last.wrapping_sub((*bn).bn_start).wrapping_add(1)); bn = rb_next_xbitmap64(bn); } ret }
pub unsafe fn xbitmap64_walk(bitmap: *mut xbitmap64, fn_: xbitmap64_walk_fn, priv_: *mut core::ffi::c_void) -> i32 { let mut error = 0; let mut bn = rb_first_xbitmap64(bitmap); while !bn.is_null() { error = fn_((*bn).bn_start, (*bn).bn_last.wrapping_sub((*bn).bn_start).wrapping_add(1), priv_); if error != 0 { break; } bn = rb_next_xbitmap64(bn); } error }
pub unsafe fn xbitmap64_empty(bitmap: *mut xbitmap64) -> bool { (*bitmap).xb_root.rb_root.rb_node.is_null() }
pub unsafe fn xbitmap64_test(bitmap: *mut xbitmap64, start: u64, len: *mut u64) -> bool { let last = start.wrapping_add(*len).wrapping_sub(1); let bn = xbitmap64_tree_iter_first(&mut (*bitmap).xb_root, start, last); if bn.is_null() { return false; } if (*bn).bn_start <= start { if (*bn).bn_last < last { *len = (*bn).bn_last.wrapping_sub(start).wrapping_add(1); } true } else { *len = (*bn).bn_start.wrapping_sub(start); false } }

/* u32 bitmap: same interval operations with uint32_t parameters. */
#[repr(C)]
pub struct xbitmap32_node { pub bn_rbnode: rb_node, pub bn_start: u32, pub bn_last: u32, pub __bn_subtree_last: u32 }

// The u32 interval-tree declarations and bitmap API are supplied in the same form as above.
// INTERVAL_TREE_DEFINE generates these functions in the C implementation.
extern "C" {
    fn xbitmap32_tree_insert(node: *mut xbitmap32_node, root: *mut rb_root_cached);
    fn xbitmap32_tree_remove(node: *mut xbitmap32_node, root: *mut rb_root_cached);
    fn xbitmap32_tree_iter_first(root: *mut rb_root_cached, start: u32, last: u32) -> *mut xbitmap32_node;
}

/* The remaining u32 routines preserve the C algorithm and callback interfaces. */
pub unsafe fn xbitmap32_clear(bitmap: *mut xbitmap32, start: u32, len: u32) -> i32 { bitmap_clear32(bitmap, start, len) }
pub unsafe fn xbitmap32_set(bitmap: *mut xbitmap32, start: u32, len: u32) -> i32 { bitmap_set32(bitmap, start, len) }
pub unsafe fn xbitmap32_destroy(bitmap: *mut xbitmap32) { bitmap_destroy32(bitmap) }
pub unsafe fn xbitmap32_init(bitmap: *mut xbitmap32) { (*bitmap).xb_root = RB_ROOT_CACHED; }
pub unsafe fn xbitmap32_disunion(bitmap: *mut xbitmap32, sub: *mut xbitmap32) -> i32 { bitmap_disunion32(bitmap, sub) }
pub unsafe fn xbitmap32_hweight(bitmap: *mut xbitmap32) -> u32 { bitmap_hweight32(bitmap) }
pub unsafe fn xbitmap32_walk(bitmap: *mut xbitmap32, fn_: xbitmap32_walk_fn, priv_: *mut core::ffi::c_void) -> i32 { bitmap_walk32(bitmap, fn_, priv_) }
pub unsafe fn xbitmap32_empty(bitmap: *mut xbitmap32) -> bool { (*bitmap).xb_root.rb_root.rb_node.is_null() }
pub unsafe fn xbitmap32_test(bitmap: *mut xbitmap32, start: u32, len: *mut u32) -> bool { bitmap_test32(bitmap, start, len) }
pub unsafe fn xbitmap32_count_set_regions(bitmap: *mut xbitmap32) -> u32 { bitmap_count32(bitmap) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
