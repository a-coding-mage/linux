// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/block-range.c. External types and helpers come from
// the Rust equivalents of block-range.h, annotate.h, and the rbtree support.

use std::ffi::c_void;
use std::mem::offset_of;
use std::ptr;

pub type u64 = u64;

#[repr(C)]
pub struct rb_node {
    pub rb_left: *mut rb_node,
    pub rb_right: *mut rb_node,
    pub __rb_parent_color: usize,
}

#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct annotated_branch {
    pub max_coverage: u64,
}

#[repr(C)]
pub struct annotation {
    pub branch: *mut annotated_branch,
}

#[repr(C)]
pub struct block_range {
    pub node: rb_node,
    pub start: u64,
    pub end: u64,
    pub is_target: i32,
    pub is_branch: i32,
    pub coverage: u64,
    pub taken: u64,
    pub pred: u64,
    pub entry: u64,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct block_range_iter {
    pub start: *mut block_range,
    pub end: *mut block_range,
}

#[repr(C)]
struct block_ranges_state {
    root: rb_root,
    blocks: u64,
}

static mut block_ranges: block_ranges_state = block_ranges_state {
    root: rb_root {
        rb_node: ptr::null_mut(),
    },
    blocks: 0,
};

unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;

    fn rb_first(root: *const rb_root) -> *mut rb_node;
    fn rb_next(node: *const rb_node) -> *mut rb_node;
    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, rb_link: *mut *mut rb_node);
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);

    fn block_range__next(br: *mut block_range) -> *mut block_range;
    fn symbol__annotation(sym: *mut symbol) -> *mut annotation;
}

#[inline]
unsafe fn rb_entry_block_range(node: *mut rb_node) -> *mut block_range {
    (node as *mut u8).sub(offset_of!(block_range, node)) as *mut block_range
}

unsafe fn block_range__debug() {
    #[cfg(debug_assertions)]
    {
        let mut rb: *mut rb_node;
        let mut old: u64 = 0; /* NULL isn't executable */

        rb = rb_first(ptr::addr_of!(block_ranges.root));
        while !rb.is_null() {
            let entry = rb_entry_block_range(rb);

            assert!(old < (*entry).start);
            assert!((*entry).start <= (*entry).end); /* single instruction block; jump to a jump */

            old = (*entry).end;
            rb = rb_next(rb);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn block_range__find(addr: u64) -> *mut block_range {
    let mut p: *mut *mut rb_node = ptr::addr_of_mut!(block_ranges.root.rb_node);
    let mut parent: *mut rb_node;
    let mut entry: *mut block_range;

    while !(*p).is_null() {
        parent = *p;
        entry = rb_entry_block_range(parent);

        if addr < (*entry).start {
            p = ptr::addr_of_mut!((*parent).rb_left);
        } else if addr > (*entry).end {
            p = ptr::addr_of_mut!((*parent).rb_right);
        } else {
            return entry;
        }
    }

    ptr::null_mut()
}

#[inline]
unsafe fn rb_link_left_of_node(left: *mut rb_node, mut node: *mut rb_node) {
    let mut p: *mut *mut rb_node = ptr::addr_of_mut!((*node).rb_left);
    while !(*p).is_null() {
        node = *p;
        p = ptr::addr_of_mut!((*node).rb_right);
    }
    rb_link_node(left, node, p);
}

#[inline]
unsafe fn rb_link_right_of_node(right: *mut rb_node, mut node: *mut rb_node) {
    let mut p: *mut *mut rb_node = ptr::addr_of_mut!((*node).rb_right);
    while !(*p).is_null() {
        node = *p;
        p = ptr::addr_of_mut!((*node).rb_left);
    }
    rb_link_node(right, node, p);
}

/**
 * block_range__create
 * @start: branch target starting this basic block
 * @end:   branch ending this basic block
 *
 * Create all the required block ranges to precisely span the given range.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn block_range__create(start: u64, end: u64) -> block_range_iter {
    let mut p: *mut *mut rb_node = ptr::addr_of_mut!(block_ranges.root.rb_node);
    let mut n: *mut rb_node;
    let mut parent: *mut rb_node = ptr::null_mut();
    let mut next: *mut block_range;
    let mut entry: *mut block_range = ptr::null_mut();
    let mut iter = block_range_iter {
        start: ptr::null_mut(),
        end: ptr::null_mut(),
    };

    while !(*p).is_null() {
        parent = *p;
        entry = rb_entry_block_range(parent);

        if start < (*entry).start {
            p = ptr::addr_of_mut!((*parent).rb_left);
        } else if start > (*entry).end {
            p = ptr::addr_of_mut!((*parent).rb_right);
        } else {
            break;
        }
    }

    /*
     * Didn't find anything.. there's a hole at @start, however @end might
     * be inside/behind the next range.
     */
    if (*p).is_null() {
        if entry.is_null() {
            /* tree empty */
            entry = malloc(std::mem::size_of::<block_range>()) as *mut block_range;
            if entry.is_null() {
                return iter;
            }

            ptr::write(
                entry,
                block_range {
                    node: std::mem::zeroed(),
                    start,
                    end,
                    is_target: 1,
                    is_branch: 1,
                    coverage: 0,
                    taken: 0,
                    pred: 0,
                    entry: 0,
                    sym: ptr::null_mut(),
                },
            );

            rb_link_node(ptr::addr_of_mut!((*entry).node), parent, p);
            rb_insert_color(
                ptr::addr_of_mut!((*entry).node),
                ptr::addr_of_mut!(block_ranges.root),
            );
            block_range__debug();

            iter.start = entry;
            iter.end = entry;
            assert!((*iter.start).start == start && (*iter.start).is_target != 0);
            assert!((*iter.end).end == end && (*iter.end).is_branch != 0);
            block_ranges.blocks += 1;
            return iter;
        }

        /*
         * If the last node is before, advance one to find the next.
         */
        n = parent;
        if (*entry).end < start {
            n = rb_next(n);
            if n.is_null() {
                entry = malloc(std::mem::size_of::<block_range>()) as *mut block_range;
                if entry.is_null() {
                    return iter;
                }

                ptr::write(
                    entry,
                    block_range {
                        node: std::mem::zeroed(),
                        start,
                        end,
                        is_target: 1,
                        is_branch: 1,
                        coverage: 0,
                        taken: 0,
                        pred: 0,
                        entry: 0,
                        sym: ptr::null_mut(),
                    },
                );

                rb_link_node(ptr::addr_of_mut!((*entry).node), parent, p);
                rb_insert_color(
                    ptr::addr_of_mut!((*entry).node),
                    ptr::addr_of_mut!(block_ranges.root),
                );
                block_range__debug();

                iter.start = entry;
                iter.end = entry;
                assert!((*iter.start).start == start && (*iter.start).is_target != 0);
                assert!((*iter.end).end == end && (*iter.end).is_branch != 0);
                block_ranges.blocks += 1;
                return iter;
            }
        }
        next = rb_entry_block_range(n);

        if (*next).start <= end {
            /* add head: [start...][n->start...] */
            let head = malloc(std::mem::size_of::<block_range>()) as *mut block_range;
            if head.is_null() {
                return iter;
            }

            ptr::write(
                head,
                block_range {
                    node: std::mem::zeroed(),
                    start,
                    end: (*next).start - 1,
                    is_target: 1,
                    is_branch: 0,
                    coverage: 0,
                    taken: 0,
                    pred: 0,
                    entry: 0,
                    sym: ptr::null_mut(),
                },
            );

            rb_link_left_of_node(ptr::addr_of_mut!((*head).node), ptr::addr_of_mut!((*next).node));
            rb_insert_color(
                ptr::addr_of_mut!((*head).node),
                ptr::addr_of_mut!(block_ranges.root),
            );
            block_range__debug();

            iter.start = head;
        } else {
            /*
             * The whole [start..end] range is non-overlapping.
             */
            entry = malloc(std::mem::size_of::<block_range>()) as *mut block_range;
            if entry.is_null() {
                return iter;
            }

            ptr::write(
                entry,
                block_range {
                    node: std::mem::zeroed(),
                    start,
                    end,
                    is_target: 1,
                    is_branch: 1,
                    coverage: 0,
                    taken: 0,
                    pred: 0,
                    entry: 0,
                    sym: ptr::null_mut(),
                },
            );

            rb_link_node(ptr::addr_of_mut!((*entry).node), parent, p);
            rb_insert_color(
                ptr::addr_of_mut!((*entry).node),
                ptr::addr_of_mut!(block_ranges.root),
            );
            block_range__debug();

            iter.start = entry;
            iter.end = entry;
            assert!((*iter.start).start == start && (*iter.start).is_target != 0);
            assert!((*iter.end).end == end && (*iter.end).is_branch != 0);
            block_ranges.blocks += 1;
            return iter;
        }
    } else {
        /*
         * We found a range that overlapped with ours, split if needed.
         */
        if (*entry).start < start {
            /* split: [e->start...][start...] */
            let head = malloc(std::mem::size_of::<block_range>()) as *mut block_range;
            if head.is_null() {
                return iter;
            }

            ptr::write(
                head,
                block_range {
                    node: std::mem::zeroed(),
                    start: (*entry).start,
                    end: start - 1,
                    is_target: (*entry).is_target,
                    is_branch: 0,
                    coverage: (*entry).coverage,
                    taken: 0,
                    pred: 0,
                    entry: (*entry).entry,
                    sym: ptr::null_mut(),
                },
            );

            (*entry).start = start;
            (*entry).is_target = 1;
            (*entry).entry = 0;

            rb_link_left_of_node(ptr::addr_of_mut!((*head).node), ptr::addr_of_mut!((*entry).node));
            rb_insert_color(
                ptr::addr_of_mut!((*head).node),
                ptr::addr_of_mut!(block_ranges.root),
            );
            block_range__debug();
        } else if (*entry).start == start {
            (*entry).is_target = 1;
        }

        iter.start = entry;
    }

    /*
     * At this point we've got: @iter.start = [@start...] but @end can still be
     * inside or beyond it.
     */
    entry = iter.start;
    loop {
        /*
         * If @end is inside @entry, split.
         */
        if end < (*entry).end {
            /* split: [...end][...e->end] */
            let tail = malloc(std::mem::size_of::<block_range>()) as *mut block_range;
            if tail.is_null() {
                return iter;
            }

            ptr::write(
                tail,
                block_range {
                    node: std::mem::zeroed(),
                    start: end + 1,
                    end: (*entry).end,
                    is_target: 0,
                    is_branch: (*entry).is_branch,
                    coverage: (*entry).coverage,
                    taken: (*entry).taken,
                    pred: (*entry).pred,
                    entry: 0,
                    sym: ptr::null_mut(),
                },
            );

            (*entry).end = end;
            (*entry).is_branch = 1;
            (*entry).taken = 0;
            (*entry).pred = 0;

            rb_link_right_of_node(ptr::addr_of_mut!((*tail).node), ptr::addr_of_mut!((*entry).node));
            rb_insert_color(
                ptr::addr_of_mut!((*tail).node),
                ptr::addr_of_mut!(block_ranges.root),
            );
            block_range__debug();

            iter.end = entry;
            break;
        }

        /*
         * If @end matches @entry, done
         */
        if end == (*entry).end {
            (*entry).is_branch = 1;
            iter.end = entry;
            break;
        }

        next = block_range__next(entry);
        if next.is_null() || end < (*next).start {
            /*
             * If @end is in beyond @entry but not inside @next, add tail.
             */
            /* add tail: [...e->end][...end] */
            let tail = malloc(std::mem::size_of::<block_range>()) as *mut block_range;
            if tail.is_null() {
                return iter;
            }

            ptr::write(
                tail,
                block_range {
                    node: std::mem::zeroed(),
                    start: (*entry).end + 1,
                    end,
                    is_target: 0,
                    is_branch: 1,
                    coverage: 0,
                    taken: 0,
                    pred: 0,
                    entry: 0,
                    sym: ptr::null_mut(),
                },
            );

            rb_link_right_of_node(ptr::addr_of_mut!((*tail).node), ptr::addr_of_mut!((*entry).node));
            rb_insert_color(
                ptr::addr_of_mut!((*tail).node),
                ptr::addr_of_mut!(block_ranges.root),
            );
            block_range__debug();

            iter.end = tail;
            break;
        }

        /*
         * If there is a hole between @entry and @next, fill it.
         */
        if (*entry).end + 1 != (*next).start {
            let hole = malloc(std::mem::size_of::<block_range>()) as *mut block_range;
            if hole.is_null() {
                return iter;
            }

            ptr::write(
                hole,
                block_range {
                    node: std::mem::zeroed(),
                    start: (*entry).end + 1,
                    end: (*next).start - 1,
                    is_target: 0,
                    is_branch: 0,
                    coverage: 0,
                    taken: 0,
                    pred: 0,
                    entry: 0,
                    sym: ptr::null_mut(),
                },
            );

            rb_link_left_of_node(ptr::addr_of_mut!((*hole).node), ptr::addr_of_mut!((*next).node));
            rb_insert_color(
                ptr::addr_of_mut!((*hole).node),
                ptr::addr_of_mut!(block_ranges.root),
            );
            block_range__debug();
        }

        entry = next;
    }

    assert!((*iter.start).start == start && (*iter.start).is_target != 0);
    assert!((*iter.end).end == end && (*iter.end).is_branch != 0);

    block_ranges.blocks += 1;

    iter
}

/*
 * Compute coverage as:
 *
 *    br->coverage / br->sym->max_coverage
 *
 * This ensures each symbol has a 100% spot, to reflect that each symbol has a
 * most covered section.
 *
 * Returns [0-1] for coverage and -1 if we had no data what so ever or the
 * symbol does not exist.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn block_range__coverage(br: *mut block_range) -> f64 {
    let sym: *mut symbol;
    let branch: *mut annotated_branch;

    if br.is_null() {
        if block_ranges.blocks != 0 {
            return 0.0;
        }

        return -1.0;
    }

    sym = (*br).sym;
    if sym.is_null() {
        return -1.0;
    }

    branch = (*symbol__annotation(sym)).branch;
    if branch.is_null() {
        return -1.0;
    }

    (*br).coverage as f64 / (*branch).max_coverage as f64
}
