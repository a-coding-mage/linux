/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the Linux red-black-tree implementation.
use crate::linux::rbtree::{rb_node, rb_root_cached};

#[repr(C)]
pub struct interval_tree_node {
    pub rb: rb_node,
    pub start: ::core::ffi::c_ulong, // Start of interval
    pub last: ::core::ffi::c_ulong, // Last location _in_ interval
    pub __subtree_last: ::core::ffi::c_ulong,
}

extern "C" {
    pub fn interval_tree_insert(
        node: *mut interval_tree_node,
        root: *mut rb_root_cached,
    );

    pub fn interval_tree_remove(
        node: *mut interval_tree_node,
        root: *mut rb_root_cached,
    );

    pub fn interval_tree_subtree_search(
        node: *mut interval_tree_node,
        start: ::core::ffi::c_ulong,
        last: ::core::ffi::c_ulong,
    ) -> *mut interval_tree_node;

    pub fn interval_tree_iter_first(
        root: *mut rb_root_cached,
        start: ::core::ffi::c_ulong,
        last: ::core::ffi::c_ulong,
    ) -> *mut interval_tree_node;

    pub fn interval_tree_iter_next(
        node: *mut interval_tree_node,
        start: ::core::ffi::c_ulong,
        last: ::core::ffi::c_ulong,
    ) -> *mut interval_tree_node;
}

/**
 * struct interval_tree_span_iter - Find used and unused spans.
 * @start_hole: Start of an interval for a hole when is_hole == 1
 * @last_hole: Inclusive end of an interval for a hole when is_hole == 1
 * @start_used: Start of a used interval when is_hole == 0
 * @last_used: Inclusive end of a used interval when is_hole == 0
 * @is_hole: 0 == used, 1 == is_hole, -1 == done iteration
 *
 * This iterator travels over spans in an interval tree. It does not return
 * nodes but classifies each span as either a hole, where no nodes intersect, or
 * a used, which is fully covered by nodes. Each iteration step toggles between
 * hole and used until the entire range is covered. The returned spans always
 * fully cover the requested range.
 *
 * The iterator is greedy, it always returns the largest hole or used possible,
 * consolidating all consecutive nodes.
 *
 * Use interval_tree_span_iter_done() to detect end of iteration.
 */
#[repr(C)]
pub struct interval_tree_span_iter {
    // private: not for use by the caller
    pub nodes: [*mut interval_tree_node; 2],
    pub first_index: ::core::ffi::c_ulong,
    pub last_index: ::core::ffi::c_ulong,

    // public:
    pub start: interval_tree_span_iter_start,
    pub last: interval_tree_span_iter_last,
    pub is_hole: ::core::ffi::c_int,
}

#[repr(C)]
pub union interval_tree_span_iter_start {
    pub start_hole: ::core::ffi::c_ulong,
    pub start_used: ::core::ffi::c_ulong,
}

#[repr(C)]
pub union interval_tree_span_iter_last {
    pub last_hole: ::core::ffi::c_ulong,
    pub last_used: ::core::ffi::c_ulong,
}

extern "C" {
    pub fn interval_tree_span_iter_first(
        state: *mut interval_tree_span_iter,
        itree: *mut rb_root_cached,
        first_index: ::core::ffi::c_ulong,
        last_index: ::core::ffi::c_ulong,
    );
    pub fn interval_tree_span_iter_advance(
        iter: *mut interval_tree_span_iter,
        itree: *mut rb_root_cached,
        new_index: ::core::ffi::c_ulong,
    );
    pub fn interval_tree_span_iter_next(state: *mut interval_tree_span_iter);
}

#[inline]
pub unsafe fn interval_tree_span_iter_done(state: *mut interval_tree_span_iter) -> bool {
    (*state).is_hole == -1
}

#[macro_export]
macro_rules! interval_tree_for_each_span {
    ($span:expr, $itree:expr, $first_index:expr, $last_index:expr) => {
        for _ in unsafe {
            $crate::interval_tree_span_iter_first(
                $span,
                $itree,
                $first_index,
                $last_index,
            )
        } {
            if unsafe { $crate::interval_tree_span_iter_done($span) } {
                break;
            }
            unsafe { $crate::interval_tree_span_iter_next($span) };
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
