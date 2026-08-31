// SPDX-License-Identifier: GPL-2.0
/*
 * Regression3
 * Description:
 * Helper radix_tree_iter_retry resets next_index to the current index.
 * In following radix_tree_next_slot current chunk size becomes zero.
 * This isn't checked and it tries to dereference null pointer in slot.
 *
 * Helper radix_tree_iter_resume reset slot to NULL and next_index to index + 1,
 * for tagger iteraction it also must reset cached tags in iterator to abort
 * next radix_tree_next_slot and go to slow-path into radix_tree_next_chunk.
 *
 * Running:
 * This test should run to completion immediately. The above bug would
 * cause it to segfault.
 *
 * Upstream commit:
 * Not yet
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

const GFP_KERNEL: c_ulong = 0;
const RADIX_TREE_ITER_TAGGED: c_uint = 1;

type c_uint = u32;

#[repr(C)]
pub struct radix_tree_root {
    pub xa_flags: c_ulong,
    pub xa_head: *mut c_void,
}

#[repr(C)]
pub struct radix_tree_iter {
    pub index: c_ulong,
    pub next_index: c_ulong,
    pub tags: c_ulong,
}

unsafe extern "C" {
    fn printv(level: c_int, fmt: *const c_char, ...);

    fn radix_tree_insert(
        root: *mut radix_tree_root,
        index: c_ulong,
        item: *mut c_void,
    ) -> c_int;
    fn radix_tree_tag_set(root: *mut radix_tree_root, index: c_ulong, tag: c_uint);
    fn radix_tree_delete(root: *mut radix_tree_root, index: c_ulong) -> *mut c_void;
    fn radix_tree_deref_retry(arg: *mut c_void) -> bool;
    fn radix_tree_iter_retry(iter: *mut radix_tree_iter) -> *mut *mut c_void;
    fn radix_tree_iter_resume(
        slot: *mut *mut c_void,
        iter: *mut radix_tree_iter,
    ) -> *mut *mut c_void;
    fn radix_tree_next_chunk(
        root: *mut radix_tree_root,
        iter: *mut radix_tree_iter,
        flags: c_uint,
    ) -> *mut *mut c_void;
    fn radix_tree_next_slot(slot: *mut *mut c_void, iter: *mut radix_tree_iter) -> *mut *mut c_void;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn regression3_test() {
    let mut root = radix_tree_root {
        xa_flags: GFP_KERNEL,
        xa_head: ptr::null_mut(),
    };
    let ptr0 = 4usize as *mut c_void;
    let ptr = 8usize as *mut c_void;
    let mut iter = radix_tree_iter {
        index: 0,
        next_index: 0,
        tags: 0,
    };
    let mut slot: *mut *mut c_void;
    let mut first: bool;

    printv(
        1,
        c"running regression test 3 (should take milliseconds)\n".as_ptr(),
    );

    radix_tree_insert(&mut root, 0, ptr0);
    radix_tree_tag_set(&mut root, 0, 0);

    first = true;
    slot = radix_tree_next_chunk(&mut root, &mut iter, RADIX_TREE_ITER_TAGGED);
    while !slot.is_null() {
        while !slot.is_null() {
            printv(2, c"tagged %ld %p\n".as_ptr(), iter.index, *slot);
            if first {
                radix_tree_insert(&mut root, 1, ptr);
                radix_tree_tag_set(&mut root, 1, 0);
                first = false;
            }
            if radix_tree_deref_retry(*slot) {
                printv(2, c"retry at %ld\n".as_ptr(), iter.index);
                slot = radix_tree_iter_retry(&mut iter);
                continue;
            }
            slot = radix_tree_next_slot(slot, &mut iter);
        }
        slot = radix_tree_next_chunk(&mut root, &mut iter, RADIX_TREE_ITER_TAGGED);
    }
    radix_tree_delete(&mut root, 1);

    first = true;
    slot = radix_tree_next_chunk(&mut root, &mut iter, 0);
    while !slot.is_null() {
        while !slot.is_null() {
            printv(2, c"slot %ld %p\n".as_ptr(), iter.index, *slot);
            if first {
                radix_tree_insert(&mut root, 1, ptr);
                first = false;
            }
            if radix_tree_deref_retry(*slot) {
                printv(2, c"retry at %ld\n".as_ptr(), iter.index);
                slot = radix_tree_iter_retry(&mut iter);
                continue;
            }
            slot = radix_tree_next_slot(slot, &mut iter);
        }
        slot = radix_tree_next_chunk(&mut root, &mut iter, 0);
    }

    slot = radix_tree_next_chunk(&mut root, &mut iter, 0);
    while !slot.is_null() {
        while !slot.is_null() {
            printv(2, c"slot %ld %p\n".as_ptr(), iter.index, *slot);
            if iter.index == 0 {
                printv(2, c"next at %ld\n".as_ptr(), iter.index);
                slot = radix_tree_iter_resume(slot, &mut iter);
            }
            slot = radix_tree_next_slot(slot, &mut iter);
        }
        slot = radix_tree_next_chunk(&mut root, &mut iter, 0);
    }

    radix_tree_tag_set(&mut root, 0, 0);
    radix_tree_tag_set(&mut root, 1, 0);
    slot = radix_tree_next_chunk(&mut root, &mut iter, RADIX_TREE_ITER_TAGGED);
    while !slot.is_null() {
        while !slot.is_null() {
            printv(2, c"tagged %ld %p\n".as_ptr(), iter.index, *slot);
            if iter.index == 0 {
                printv(2, c"next at %ld\n".as_ptr(), iter.index);
                slot = radix_tree_iter_resume(slot, &mut iter);
            }
            slot = radix_tree_next_slot(slot, &mut iter);
        }
        slot = radix_tree_next_chunk(&mut root, &mut iter, RADIX_TREE_ITER_TAGGED);
    }

    radix_tree_delete(&mut root, 0);
    radix_tree_delete(&mut root, 1);

    printv(1, c"regression test 3 passed\n".as_ptr());
}
