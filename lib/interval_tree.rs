// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the Linux interval-tree headers and generic macro.

#[repr(C)]
pub struct RbNode {
    pub __rb_parent_color: usize,
    pub rb_left: *mut RbNode,
    pub rb_right: *mut RbNode,
}

#[repr(C)]
pub struct IntervalTreeNode {
    pub rb: RbNode,
    pub start: usize,
    pub last: usize,
    pub __subtree_last: usize,
}

#[repr(C)]
pub struct RbRootCached {
    pub rb_root: *mut RbNode,
    pub rb_leftmost: *mut RbNode,
}

#[repr(C)]
pub struct IntervalTreeSpanIter {
    pub first_index: usize,
    pub last_index: usize,
    pub start_hole: usize,
    pub last_hole: usize,
    pub start_used: usize,
    pub last_used: usize,
    pub nodes: [*mut IntervalTreeNode; 2],
    pub is_hole: i32,
}

extern "C" {
    pub fn interval_tree_insert(node: *mut IntervalTreeNode, itree: *mut RbRootCached);
    pub fn interval_tree_remove(node: *mut IntervalTreeNode, itree: *mut RbRootCached);
    pub fn interval_tree_subtree_search(
        node: *mut RbNode,
        first: usize,
        last: usize,
    ) -> *mut IntervalTreeNode;
    pub fn interval_tree_iter_first(
        itree: *mut RbRootCached,
        first: usize,
        last: usize,
    ) -> *mut IntervalTreeNode;
    pub fn interval_tree_iter_next(
        node: *mut IntervalTreeNode,
        first: usize,
        last: usize,
    ) -> *mut IntervalTreeNode;
}

unsafe fn interval_tree_span_iter_next_gap(state: *mut IntervalTreeSpanIter) {
    let mut cur = (*state).nodes[1];

    (*state).nodes[0] = cur;
    loop {
        if (*cur).last > (*(*state).nodes[0]).last {
            (*state).nodes[0] = cur;
        }
        cur = interval_tree_iter_next(cur, (*state).first_index, (*state).last_index);
        if !( !cur.is_null()
            && ((*(*state).nodes[0]).last >= (*cur).start
                || (*(*state).nodes[0]).last.wrapping_add(1) == (*cur).start))
        {
            break;
        }
    }
    (*state).nodes[1] = cur;
}

pub unsafe fn interval_tree_span_iter_first(
    iter: *mut IntervalTreeSpanIter,
    itree: *mut RbRootCached,
    first_index: usize,
    last_index: usize,
) {
    (*iter).first_index = first_index;
    (*iter).last_index = last_index;
    (*iter).nodes[0] = core::ptr::null_mut();
    (*iter).nodes[1] = interval_tree_iter_first(itree, first_index, last_index);
    if (*iter).nodes[1].is_null() {
        (*iter).start_hole = first_index;
        (*iter).last_hole = last_index;
        (*iter).is_hole = 1;
        return;
    }
    if (*(*iter).nodes[1]).start > first_index {
        (*iter).start_hole = first_index;
        (*iter).last_hole = (*(*iter).nodes[1]).start.wrapping_sub(1);
        (*iter).is_hole = 1;
        interval_tree_span_iter_next_gap(iter);
        return;
    }
    (*iter).start_used = first_index;
    (*iter).is_hole = 0;
    interval_tree_span_iter_next_gap(iter);
    (*iter).last_used = (*(*iter).nodes[0]).last;
    if (*iter).last_used >= last_index {
        (*iter).last_used = last_index;
        (*iter).nodes[0] = core::ptr::null_mut();
        (*iter).nodes[1] = core::ptr::null_mut();
    }
}

pub unsafe fn interval_tree_span_iter_next(iter: *mut IntervalTreeSpanIter) {
    if (*iter).nodes[0].is_null() && (*iter).nodes[1].is_null() {
        (*iter).is_hole = -1;
        return;
    }
    if (*iter).is_hole != 0 {
        (*iter).start_used = (*iter).last_hole.wrapping_add(1);
        (*iter).last_used = (*(*iter).nodes[0]).last;
        if (*iter).last_used >= (*iter).last_index {
            (*iter).last_used = (*iter).last_index;
            (*iter).nodes[0] = core::ptr::null_mut();
            (*iter).nodes[1] = core::ptr::null_mut();
        }
        (*iter).is_hole = 0;
        return;
    }
    if (*iter).nodes[1].is_null() {
        (*iter).start_hole = (*(*iter).nodes[0]).last.wrapping_add(1);
        (*iter).last_hole = (*iter).last_index;
        (*iter).nodes[0] = core::ptr::null_mut();
        (*iter).is_hole = 1;
        return;
    }
    (*iter).start_hole = (*(*iter).nodes[0]).last.wrapping_add(1);
    (*iter).last_hole = (*(*iter).nodes[1]).start.wrapping_sub(1);
    (*iter).is_hole = 1;
    interval_tree_span_iter_next_gap(iter);
}

pub unsafe fn interval_tree_span_iter_advance(
    iter: *mut IntervalTreeSpanIter,
    itree: *mut RbRootCached,
    new_index: usize,
) {
    if (*iter).is_hole == -1 { return; }
    (*iter).first_index = new_index;
    if new_index > (*iter).last_index { (*iter).is_hole = -1; return; }
    if (*iter).start_hole <= new_index && new_index <= (*iter).last_hole {
        (*iter).start_hole = new_index;
        return;
    }
    if new_index == (*iter).last_hole.wrapping_add(1) {
        interval_tree_span_iter_next(iter);
    } else {
        interval_tree_span_iter_first(iter, itree, new_index, (*iter).last_index);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
