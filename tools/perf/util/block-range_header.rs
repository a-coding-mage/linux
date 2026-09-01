/* SPDX-License-Identifier: GPL-2.0 */

// Translated from perf/util/block-range.h.
// C dependencies: <stdbool.h>, <linux/rbtree.h>, <linux/types.h>.

use core::mem::offset_of;
use core::ptr;

/*
 * struct block_range - non-overlapping parts of basic blocks
 * @node:	treenode
 * @start:	inclusive start of range
 * @end:	inclusive end of range
 * @is_target:	@start is a jump target
 * @is_branch:	@end is a branch instruction
 * @coverage:	number of blocks that cover this range
 * @taken:	number of times the branch is taken (requires @is_branch)
 * @pred:	number of times the taken branch was predicted
 */
#[repr(C)]
pub struct block_range {
    pub node: rb_node,

    pub sym: *mut symbol,

    pub start: u64,
    pub end: u64,

    pub is_target: i32,
    pub is_branch: i32,

    pub coverage: u64,
    pub entry: u64,
    pub taken: u64,
    pub pred: u64,
}

#[inline]
pub unsafe fn block_range__next(br: *mut block_range) -> *mut block_range {
    let n: *mut rb_node = unsafe { rb_next(ptr::addr_of_mut!((*br).node)) };
    if n.is_null() {
        return ptr::null_mut();
    }
    (n as *mut u8).wrapping_sub(offset_of!(block_range, node)) as *mut block_range
}

#[repr(C)]
pub struct block_range_iter {
    pub start: *mut block_range,
    pub end: *mut block_range,
}

#[inline]
pub unsafe fn block_range_iter(iter: *mut block_range_iter) -> *mut block_range {
    unsafe { (*iter).start }
}

#[inline]
pub unsafe fn block_range_iter__next(iter: *mut block_range_iter) -> bool {
    if unsafe { (*iter).start == (*iter).end } {
        return false;
    }

    unsafe {
        (*iter).start = block_range__next((*iter).start);
    }
    true
}

#[inline]
pub unsafe fn block_range_iter__valid(iter: *mut block_range_iter) -> bool {
    if unsafe { (*iter).start.is_null() || (*iter).end.is_null() } {
        return false;
    }
    true
}

unsafe extern "C" {
    pub fn rb_next(node: *mut rb_node) -> *mut rb_node;

    pub fn block_range__find(addr: u64) -> *mut block_range;
    pub fn block_range__create(start: u64, end: u64) -> block_range_iter;
    pub fn block_range__coverage(br: *mut block_range) -> f64;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
