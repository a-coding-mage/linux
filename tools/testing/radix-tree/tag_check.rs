// SPDX-License-Identifier: GPL-2.0
// Translated from testing/radix-tree/tag_check.c.
// C includes removed; external radix-tree/test harness symbols are declared below.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct radix_tree_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct item {
    pub index: c_ulong,
}

unsafe extern "C" {
    static GFP_KERNEL: c_uint;
    static RADIX_TREE_MAP_SHIFT: c_int;
    static XA_MARK_0: c_int;
    static XA_MARK_1: c_int;
    static nr_allocated: c_int;

    fn RADIX_TREE(gfp: c_uint) -> radix_tree_root;

    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn rand() -> c_int;

    fn item_check_absent(tree: *mut radix_tree_root, index: c_ulong);
    fn item_check_present(tree: *mut radix_tree_root, index: c_ulong);
    fn item_insert(tree: *mut radix_tree_root, index: c_ulong);
    fn item_delete(tree: *mut radix_tree_root, index: c_ulong) -> c_int;
    fn item_lookup(tree: *mut radix_tree_root, index: c_ulong) -> *mut item;
    fn item_tag_get(tree: *mut radix_tree_root, index: c_ulong, tag: c_int) -> c_int;
    fn item_tag_set(tree: *mut radix_tree_root, index: c_ulong, tag: c_int);
    fn item_tag_clear(tree: *mut radix_tree_root, index: c_ulong, tag: c_int);
    fn tag_tagged_items(
        tree: *mut radix_tree_root,
        first: c_ulong,
        last: c_ulong,
        nr_to_tag: c_int,
        fromtag: c_int,
        totag: c_int,
    ) -> c_int;
    fn radix_tree_gang_lookup_tag(
        root: *mut radix_tree_root,
        results: *mut *mut c_void,
        first_index: c_ulong,
        max_items: c_uint,
        tag: c_int,
    ) -> c_uint;
    fn verify_tag_consistency(tree: *mut radix_tree_root, tag: c_int);
    fn item_kill_tree(tree: *mut radix_tree_root);
    fn rcu_barrier();
    fn printv(level: c_int, fmt: *const c_char, ...);
}

unsafe fn __simple_checks(tree: *mut radix_tree_root, index: c_ulong, tag: c_int) {
    let first: c_ulong = 0;
    let mut ret: c_int;

    unsafe {
        item_check_absent(tree, index);
        assert!(item_tag_get(tree, index, tag) == 0);

        item_insert(tree, index);
        assert!(item_tag_get(tree, index, tag) == 0);
        item_tag_set(tree, index, tag);
        ret = item_tag_get(tree, index, tag);
        assert!(ret != 0);
        ret = tag_tagged_items(tree, first, !0 as c_ulong, 10, tag, if tag == 0 { 1 } else { 0 });
        assert!(ret == 1);
        ret = item_tag_get(tree, index, if tag == 0 { 1 } else { 0 });
        assert!(ret != 0);
        ret = item_delete(tree, index);
        assert!(ret != 0);
        item_insert(tree, index);
        ret = item_tag_get(tree, index, tag);
        assert!(ret == 0);
        ret = item_delete(tree, index);
        assert!(ret != 0);
        ret = item_delete(tree, index);
        assert!(ret == 0);
    }
}

pub unsafe extern "C" fn simple_checks() {
    let mut index: c_ulong;
    let mut tree = unsafe { RADIX_TREE(GFP_KERNEL) };

    index = 0;
    while index < 10000 {
        unsafe {
            __simple_checks(&mut tree, index, 0);
            __simple_checks(&mut tree, index, 1);
        }
        index += 1;
    }
    unsafe {
        verify_tag_consistency(&mut tree, 0);
        verify_tag_consistency(&mut tree, 1);
        printv(2, c"before item_kill_tree: %d allocated\n".as_ptr(), nr_allocated);
        item_kill_tree(&mut tree);
        rcu_barrier();
        printv(2, c"after item_kill_tree: %d allocated\n".as_ptr(), nr_allocated);
    }
}

/*
 * Check that tags propagate correctly when extending a tree.
 */
unsafe fn extend_checks() {
    let mut tree = unsafe { RADIX_TREE(GFP_KERNEL) };

    unsafe {
        item_insert(&mut tree, 43);
        assert!(item_tag_get(&mut tree, 43, 0) == 0);
        item_tag_set(&mut tree, 43, 0);
        assert!(item_tag_get(&mut tree, 43, 0) == 1);
        item_insert(&mut tree, 1000000);
        assert!(item_tag_get(&mut tree, 43, 0) == 1);

        item_insert(&mut tree, 0);
        item_tag_set(&mut tree, 0, 0);
        item_delete(&mut tree, 1000000);
        assert!(item_tag_get(&mut tree, 43, 0) != 0);
        item_delete(&mut tree, 43);
        assert!(item_tag_get(&mut tree, 43, 0) == 0); /* crash */
        assert!(item_tag_get(&mut tree, 0, 0) == 1);

        verify_tag_consistency(&mut tree, 0);

        item_kill_tree(&mut tree);
    }
}

/*
 * Check that tags propagate correctly when contracting a tree.
 */
unsafe fn contract_checks() {
    let mut item_ptr: *mut item = core::ptr::null_mut();
    let tmp: c_int;
    let mut tree = unsafe { RADIX_TREE(GFP_KERNEL) };

    unsafe {
        tmp = 1 << RADIX_TREE_MAP_SHIFT;
        item_insert(&mut tree, tmp as c_ulong);
        item_insert(&mut tree, (tmp + 1) as c_ulong);
        item_tag_set(&mut tree, tmp as c_ulong, 0);
        item_tag_set(&mut tree, tmp as c_ulong, 1);
        item_tag_set(&mut tree, (tmp + 1) as c_ulong, 0);
        item_delete(&mut tree, (tmp + 1) as c_ulong);
        item_tag_clear(&mut tree, tmp as c_ulong, 1);

        assert!(
            radix_tree_gang_lookup_tag(
                &mut tree,
                &mut item_ptr as *mut *mut item as *mut *mut c_void,
                0,
                1,
                0,
            ) == 1
        );
        assert!(
            radix_tree_gang_lookup_tag(
                &mut tree,
                &mut item_ptr as *mut *mut item as *mut *mut c_void,
                0,
                1,
                1,
            ) == 0
        );

        assert!(item_tag_get(&mut tree, tmp as c_ulong, 0) == 1);
        assert!(item_tag_get(&mut tree, tmp as c_ulong, 1) == 0);

        verify_tag_consistency(&mut tree, 0);
        item_kill_tree(&mut tree);
    }
}

/*
 * Stupid tag thrasher
 *
 * Create a large linear array corresponding to the tree.   Each element in
 * the array is coherent with each node in the tree
 */

const NODE_ABSENT: c_char = 0;
const NODE_PRESENT: c_char = 1;
const NODE_TAGGED: c_char = 2;

const THRASH_SIZE: usize = 1000 * 1000;
const N: c_int = 127;
const BATCH: usize = 33;

unsafe fn gang_check(tree: *mut radix_tree_root, thrash_state: *mut c_char, tag: c_int) {
    let mut items: [*mut item; BATCH] = [core::ptr::null_mut(); BATCH];
    let mut nr_found: c_int;
    let mut index: c_ulong = 0;
    let mut last_index: c_ulong = 0;

    loop {
        unsafe {
            nr_found = radix_tree_gang_lookup_tag(
                tree,
                items.as_mut_ptr() as *mut *mut c_void,
                index,
                BATCH as c_uint,
                tag,
            ) as c_int;
        }
        if nr_found == 0 {
            break;
        }

        let mut i: c_int = 0;
        while i < nr_found {
            let item = items[i as usize];

            unsafe {
                while last_index < (*item).index {
                    assert!(*thrash_state.add(last_index as usize) != NODE_TAGGED);
                    last_index += 1;
                }
                assert!(*thrash_state.add(last_index as usize) == NODE_TAGGED);
            }
            last_index += 1;
            i += 1;
        }
        unsafe {
            index = (*items[(nr_found - 1) as usize]).index + 1;
        }
    }
}

unsafe fn do_thrash(tree: *mut radix_tree_root, thrash_state: *mut c_char, tag: c_int) {
    let mut insert_chunk: c_int;
    let mut delete_chunk: c_int;
    let mut tag_chunk: c_int;
    let mut untag_chunk: c_int;
    let mut total_tagged: c_int = 0;
    let mut total_present: c_int = 0;

    insert_chunk = 1;
    while insert_chunk < THRASH_SIZE as c_int {
        delete_chunk = 1;
        while delete_chunk < THRASH_SIZE as c_int {
            tag_chunk = 1;
            while tag_chunk < THRASH_SIZE as c_int {
                untag_chunk = 1;
                while untag_chunk < THRASH_SIZE as c_int {
                    let mut i: c_int;
                    let mut index: c_ulong;
                    let mut nr_inserted: c_int = 0;
                    let mut nr_deleted: c_int = 0;
                    let mut nr_tagged: c_int = 0;
                    let mut nr_untagged: c_int = 0;
                    let mut actual_total_tagged: c_int;
                    let mut actual_total_present: c_int;

                    i = 0;
                    while i < insert_chunk {
                        unsafe {
                            index = (rand() as usize % THRASH_SIZE) as c_ulong;
                            if *thrash_state.add(index as usize) != NODE_ABSENT {
                                i += 1;
                                continue;
                            }
                            item_check_absent(tree, index);
                            item_insert(tree, index);
                            assert!(*thrash_state.add(index as usize) != NODE_PRESENT);
                            *thrash_state.add(index as usize) = NODE_PRESENT;
                        }
                        nr_inserted += 1;
                        total_present += 1;
                        i += 1;
                    }

                    i = 0;
                    while i < delete_chunk {
                        unsafe {
                            index = (rand() as usize % THRASH_SIZE) as c_ulong;
                            if *thrash_state.add(index as usize) == NODE_ABSENT {
                                i += 1;
                                continue;
                            }
                            item_check_present(tree, index);
                            if item_tag_get(tree, index, tag) != 0 {
                                assert!(*thrash_state.add(index as usize) == NODE_TAGGED);
                                total_tagged -= 1;
                            } else {
                                assert!(*thrash_state.add(index as usize) == NODE_PRESENT);
                            }
                            item_delete(tree, index);
                            assert!(*thrash_state.add(index as usize) != NODE_ABSENT);
                            *thrash_state.add(index as usize) = NODE_ABSENT;
                        }
                        nr_deleted += 1;
                        total_present -= 1;
                        i += 1;
                    }

                    i = 0;
                    while i < tag_chunk {
                        unsafe {
                            index = (rand() as usize % THRASH_SIZE) as c_ulong;
                            if *thrash_state.add(index as usize) != NODE_PRESENT {
                                if !item_lookup(tree, index).is_null() {
                                    assert!(item_tag_get(tree, index, tag) != 0);
                                }
                                i += 1;
                                continue;
                            }
                            item_tag_set(tree, index, tag);
                            item_tag_set(tree, index, tag);
                            assert!(*thrash_state.add(index as usize) != NODE_TAGGED);
                            *thrash_state.add(index as usize) = NODE_TAGGED;
                        }
                        nr_tagged += 1;
                        total_tagged += 1;
                        i += 1;
                    }

                    i = 0;
                    while i < untag_chunk {
                        unsafe {
                            index = (rand() as usize % THRASH_SIZE) as c_ulong;
                            if *thrash_state.add(index as usize) != NODE_TAGGED {
                                i += 1;
                                continue;
                            }
                            item_check_present(tree, index);
                            assert!(item_tag_get(tree, index, tag) != 0);
                            item_tag_clear(tree, index, tag);
                            item_tag_clear(tree, index, tag);
                            assert!(*thrash_state.add(index as usize) != NODE_PRESENT);
                            *thrash_state.add(index as usize) = NODE_PRESENT;
                        }
                        nr_untagged += 1;
                        total_tagged -= 1;
                        i += 1;
                    }

                    actual_total_tagged = 0;
                    actual_total_present = 0;
                    index = 0;
                    while index < THRASH_SIZE as c_ulong {
                        unsafe {
                            match *thrash_state.add(index as usize) {
                                NODE_ABSENT => {
                                    item_check_absent(tree, index);
                                }
                                NODE_PRESENT => {
                                    item_check_present(tree, index);
                                    assert!(item_tag_get(tree, index, tag) == 0);
                                    actual_total_present += 1;
                                }
                                NODE_TAGGED => {
                                    item_check_present(tree, index);
                                    assert!(item_tag_get(tree, index, tag) != 0);
                                    actual_total_present += 1;
                                    actual_total_tagged += 1;
                                }
                                _ => {}
                            }
                        }
                        index += 1;
                    }

                    unsafe {
                        gang_check(tree, thrash_state, tag);

                        printv(
                            2,
                            c"%d(%d) %d(%d) %d(%d) %d(%d) / %d(%d) present, %d(%d) tagged\n"
                                .as_ptr(),
                            insert_chunk,
                            nr_inserted,
                            delete_chunk,
                            nr_deleted,
                            tag_chunk,
                            nr_tagged,
                            untag_chunk,
                            nr_untagged,
                            total_present,
                            actual_total_present,
                            total_tagged,
                            actual_total_tagged,
                        );
                    }

                    untag_chunk *= N;
                }
                tag_chunk *= N;
            }
            delete_chunk *= N;
        }
        insert_chunk *= N;
    }
}

unsafe fn thrash_tags() {
    let mut tree = unsafe { RADIX_TREE(GFP_KERNEL) };
    let thrash_state: *mut c_char;

    unsafe {
        thrash_state = malloc(THRASH_SIZE) as *mut c_char;
        memset(thrash_state as *mut c_void, 0, THRASH_SIZE);

        do_thrash(&mut tree, thrash_state, 0);

        verify_tag_consistency(&mut tree, 0);
        item_kill_tree(&mut tree);
        free(thrash_state as *mut c_void);
    }
}

unsafe fn leak_check() {
    let mut tree = unsafe { RADIX_TREE(GFP_KERNEL) };

    unsafe {
        item_insert(&mut tree, 1000000);
        item_delete(&mut tree, 1000000);
        item_kill_tree(&mut tree);
    }
}

unsafe fn __leak_check() {
    let mut tree = unsafe { RADIX_TREE(GFP_KERNEL) };

    unsafe {
        printv(2, c"%d: nr_allocated=%d\n".as_ptr(), line!() as c_int, nr_allocated);
        item_insert(&mut tree, 1000000);
        printv(2, c"%d: nr_allocated=%d\n".as_ptr(), line!() as c_int, nr_allocated);
        item_delete(&mut tree, 1000000);
        printv(2, c"%d: nr_allocated=%d\n".as_ptr(), line!() as c_int, nr_allocated);
        item_kill_tree(&mut tree);
        printv(2, c"%d: nr_allocated=%d\n".as_ptr(), line!() as c_int, nr_allocated);
    }
}

unsafe fn single_check() {
    let mut items: [*mut item; BATCH] = [core::ptr::null_mut(); BATCH];
    let mut tree = unsafe { RADIX_TREE(GFP_KERNEL) };
    let mut ret: c_int;
    let first: c_ulong = 0;

    unsafe {
        item_insert(&mut tree, 0);
        item_tag_set(&mut tree, 0, 0);
        ret = radix_tree_gang_lookup_tag(&mut tree, items.as_mut_ptr() as *mut *mut c_void, 0, BATCH as c_uint, 0) as c_int;
        assert!(ret == 1);
        ret = radix_tree_gang_lookup_tag(&mut tree, items.as_mut_ptr() as *mut *mut c_void, 1, BATCH as c_uint, 0) as c_int;
        assert!(ret == 0);
        verify_tag_consistency(&mut tree, 0);
        verify_tag_consistency(&mut tree, 1);
        ret = tag_tagged_items(&mut tree, first, 10, 10, XA_MARK_0, XA_MARK_1);
        assert!(ret == 1);
        ret = radix_tree_gang_lookup_tag(&mut tree, items.as_mut_ptr() as *mut *mut c_void, 0, BATCH as c_uint, 1) as c_int;
        assert!(ret == 1);
        item_tag_clear(&mut tree, 0, 0);
        ret = radix_tree_gang_lookup_tag(&mut tree, items.as_mut_ptr() as *mut *mut c_void, 0, BATCH as c_uint, 0) as c_int;
        assert!(ret == 0);
        item_kill_tree(&mut tree);
    }
}

pub unsafe extern "C" fn tag_check() {
    unsafe {
        single_check();
        extend_checks();
        contract_checks();
        rcu_barrier();
        printv(2, c"after extend_checks: %d allocated\n".as_ptr(), nr_allocated);
        __leak_check();
        leak_check();
        rcu_barrier();
        printv(2, c"after leak_check: %d allocated\n".as_ptr(), nr_allocated);
        simple_checks();
        rcu_barrier();
        printv(2, c"after simple_checks: %d allocated\n".as_ptr(), nr_allocated);
        thrash_tags();
        rcu_barrier();
        printv(2, c"after thrash_tags: %d allocated\n".as_ptr(), nr_allocated);
    }
}
