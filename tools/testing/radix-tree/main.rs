// SPDX-License-Identifier: GPL-2.0
//
// Dependencies from the original C includes:
// stdio.h, stdlib.h, unistd.h, time.h, assert.h, limits.h,
// linux/slab.h, linux/radix-tree.h, test.h, regression.h

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct radix_tree_root {
    _private: [u8; 0],
}

const GFP_KERNEL: c_int = 0;
const XA_MARK_0: c_int = 0;
const XA_MARK_1: c_int = 1;
const XA_MARK_2: c_int = 2;
const ITEMS: usize = 50000;

unsafe extern "C" {
    static mut stdout: *mut FILE;
    static mut optarg: *mut c_char;
    static mut test_verbose: c_int;
    static mut nr_allocated: c_int;
    static mut preempt_count: c_int;

    fn rand() -> c_int;
    fn srand(seed: u32);
    fn time(tloc: *mut c_long) -> c_long;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn exit(status: c_int) -> !;

    fn item_insert(tree: *mut radix_tree_root, index: c_ulong);
    fn item_check_absent(tree: *mut radix_tree_root, index: c_ulong);
    fn item_check_present(tree: *mut radix_tree_root, index: c_ulong);
    fn item_gang_check_present(
        tree: *mut radix_tree_root,
        index: c_ulong,
        count: c_ulong,
        chunk: c_int,
        hop: c_int,
    );
    fn item_full_scan(
        tree: *mut radix_tree_root,
        index: c_ulong,
        count: c_ulong,
        chunk: c_int,
    );
    fn item_kill_tree(tree: *mut radix_tree_root);
    fn tree_verify_min_height(tree: *mut radix_tree_root, index: c_ulong);
    fn item_delete(tree: *mut radix_tree_root, index: c_ulong) -> c_int;
    fn item_tag_get(tree: *mut radix_tree_root, index: c_ulong, tag: c_int) -> c_int;
    fn item_tag_set(tree: *mut radix_tree_root, index: c_ulong, tag: c_int);
    fn item_lookup(tree: *mut radix_tree_root, index: c_ulong) -> *mut c_void;
    fn tag_tagged_items(
        tree: *mut radix_tree_root,
        start: c_ulong,
        end: c_ulong,
        nr_to_tag: c_ulong,
        fromtag: c_int,
        totag: c_int,
    ) -> c_ulong;
    fn verify_tag_consistency(tree: *mut radix_tree_root, tag: c_int);

    fn printv(level: c_int, format: *const c_char, ...) -> c_int;
    fn multiorder_checks();
    fn rcu_barrier();
    fn tag_check();
    fn idr_checks();
    fn ida_tests();
    fn xarray_tests();
    fn regression1_test();
    fn regression2_test();
    fn regression3_test();
    fn regression4_test();
    fn iteration_test(start: c_int, count: c_int);
    fn iteration_test2(count: c_int);
    fn rcu_register_thread();
    fn radix_tree_init();
    fn radix_tree_cpu_dead(cpu: c_int);
    fn rcu_unregister_thread();
    fn benchmark();
}

unsafe fn radix_tree_new(_flags: c_int) -> radix_tree_root {
    // Translation of RADIX_TREE(tree, GFP_KERNEL); real layout is supplied by linux/radix-tree.h.
    core::mem::zeroed()
}

#[no_mangle]
pub unsafe extern "C" fn __gang_check(
    mut middle: c_ulong,
    down: c_long,
    up: c_long,
    chunk: c_int,
    hop: c_int,
) {
    let mut idx: c_long;
    let mut tree = radix_tree_new(GFP_KERNEL);

    middle = (1 as c_ulong) << 30;

    idx = -down;
    while idx < up {
        item_insert(&mut tree, middle.wrapping_add(idx as c_ulong));
        idx += 1;
    }

    item_check_absent(&mut tree, middle.wrapping_sub(down as c_ulong).wrapping_sub(1));
    idx = -down;
    while idx < up {
        item_check_present(&mut tree, middle.wrapping_add(idx as c_ulong));
        idx += 1;
    }
    item_check_absent(&mut tree, middle.wrapping_add(up as c_ulong));

    if chunk > 0 {
        item_gang_check_present(
            &mut tree,
            middle.wrapping_sub(down as c_ulong),
            up.wrapping_add(down) as c_ulong,
            chunk,
            hop,
        );
        item_full_scan(
            &mut tree,
            middle.wrapping_sub(down as c_ulong),
            down.wrapping_add(up) as c_ulong,
            chunk,
        );
    }
    item_kill_tree(&mut tree);
}

#[no_mangle]
pub unsafe extern "C" fn gang_check() {
    __gang_check((1 as c_ulong) << 30, 128, 128, 35, 2);
    __gang_check((1 as c_ulong) << 31, 128, 128, 32, 32);
    __gang_check((1 as c_ulong) << 31, 128, 128, 32, 100);
    __gang_check((1 as c_ulong) << 31, 128, 128, 17, 7);
    __gang_check(0xffff0000 as c_ulong, 0, 65536, 17, 7);
    __gang_check(0xfffffffe as c_ulong, 1, 1, 17, 7);
}

#[no_mangle]
pub unsafe extern "C" fn __big_gang_check() {
    let mut start: c_ulong;
    let mut wrapped: c_int = 0;

    start = 0;
    loop {
        let old_start: c_ulong;

        // printf("0x%08lx\n", start);
        __gang_check(
            start,
            (rand() % 113 + 1) as c_long,
            (rand() % 71) as c_long,
            rand() % 157,
            rand() % 91 + 1,
        );
        old_start = start;
        start = start.wrapping_add((rand() % 1000000) as c_ulong);
        start %= (1_u64 << 33) as c_ulong;
        if start < old_start {
            wrapped = 1;
        }
        if wrapped != 0 {
            break;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn big_gang_check(long_run: bool) {
    let mut i: c_int;

    i = 0;
    while i < if long_run { 1000 } else { 3 } {
        __big_gang_check();
        printv(2, c"%d ".as_ptr(), i);
        fflush(stdout);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn add_and_check() {
    let mut tree = radix_tree_new(GFP_KERNEL);

    item_insert(&mut tree, 44);
    item_check_present(&mut tree, 44);
    item_check_absent(&mut tree, 43);
    item_kill_tree(&mut tree);
}

#[no_mangle]
pub unsafe extern "C" fn dynamic_height_check() {
    let mut i: c_int;
    let mut tree = radix_tree_new(GFP_KERNEL);
    tree_verify_min_height(&mut tree, 0);

    item_insert(&mut tree, 42);
    tree_verify_min_height(&mut tree, 42);

    item_insert(&mut tree, 1000000);
    tree_verify_min_height(&mut tree, 1000000);

    assert!(item_delete(&mut tree, 1000000) != 0);
    tree_verify_min_height(&mut tree, 42);

    assert!(item_delete(&mut tree, 42) != 0);
    tree_verify_min_height(&mut tree, 0);

    i = 0;
    while i < 1000 {
        item_insert(&mut tree, i as c_ulong);
        tree_verify_min_height(&mut tree, i as c_ulong);
        i += 1;
    }

    i -= 1;
    loop {
        assert!(item_delete(&mut tree, i as c_ulong) != 0);
        if i == 0 {
            tree_verify_min_height(&mut tree, 0);
            break;
        }
        i -= 1;
        tree_verify_min_height(&mut tree, i as c_ulong);
    }

    item_kill_tree(&mut tree);
}

#[no_mangle]
pub unsafe extern "C" fn check_copied_tags(
    tree: *mut radix_tree_root,
    start: c_ulong,
    end: c_ulong,
    idx: *mut c_ulong,
    count: c_int,
    fromtag: c_int,
    totag: c_int,
) {
    let mut i: c_int;

    i = 0;
    while i < count {
        /*
        if (i % 1000 == 0)
            putchar('.');
        */
        let cur_idx = *idx.add(i as usize);
        if cur_idx < start || cur_idx > end {
            if item_tag_get(tree, cur_idx, totag) != 0 {
                printv(
                    2,
                    c"%lu-%lu: %lu, tags %d-%d\n".as_ptr(),
                    start,
                    end,
                    cur_idx,
                    item_tag_get(tree, cur_idx, fromtag),
                    item_tag_get(tree, cur_idx, totag),
                );
            }
            assert!(item_tag_get(tree, cur_idx, totag) == 0);
            i += 1;
            continue;
        }
        if (item_tag_get(tree, cur_idx, fromtag) ^ item_tag_get(tree, cur_idx, totag)) != 0 {
            printv(
                2,
                c"%lu-%lu: %lu, tags %d-%d\n".as_ptr(),
                start,
                end,
                cur_idx,
                item_tag_get(tree, cur_idx, fromtag),
                item_tag_get(tree, cur_idx, totag),
            );
        }
        assert!((item_tag_get(tree, cur_idx, fromtag) ^ item_tag_get(tree, cur_idx, totag)) == 0);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn copy_tag_check() {
    let mut tree = radix_tree_new(GFP_KERNEL);
    let mut idx: [c_ulong; ITEMS] = [0; ITEMS];
    let mut start: c_ulong;
    let mut end: c_ulong;
    let mut count: c_ulong = 0;
    let mut tagged: c_ulong;
    let mut cur: c_ulong;
    let mut tmp: c_ulong;
    let mut i: c_int;

    // printf("generating radix tree indices...\n");
    start = rand() as c_ulong;
    end = rand() as c_ulong;
    if start > end && (rand() % 10) != 0 {
        cur = start;
        start = end;
        end = cur;
    }
    /* Specifically create items around the start and the end of the range
     * with high probability to check for off by one errors */
    cur = rand() as c_ulong;
    if (cur & 1) != 0 {
        item_insert(&mut tree, start);
        if (cur & 2) != 0 {
            if start <= end {
                count += 1;
            }
            item_tag_set(&mut tree, start, 0);
        }
    }
    if (cur & 4) != 0 {
        item_insert(&mut tree, start.wrapping_sub(1));
        if (cur & 8) != 0 {
            item_tag_set(&mut tree, start.wrapping_sub(1), 0);
        }
    }
    if (cur & 16) != 0 {
        item_insert(&mut tree, end);
        if (cur & 32) != 0 {
            if start <= end {
                count += 1;
            }
            item_tag_set(&mut tree, end, 0);
        }
    }
    if (cur & 64) != 0 {
        item_insert(&mut tree, end.wrapping_add(1));
        if (cur & 128) != 0 {
            item_tag_set(&mut tree, end.wrapping_add(1), 0);
        }
    }

    i = 0;
    while i < ITEMS as c_int {
        loop {
            idx[i as usize] = rand() as c_ulong;
            if item_lookup(&mut tree, idx[i as usize]).is_null() {
                break;
            }
        }

        item_insert(&mut tree, idx[i as usize]);
        if (rand() & 1) != 0 {
            item_tag_set(&mut tree, idx[i as usize], 0);
            if idx[i as usize] >= start && idx[i as usize] <= end {
                count += 1;
            }
        }
        /*
        if (i % 1000 == 0)
            putchar('.');
        */
        i += 1;
    }

    // printf("\ncopying tags...\n");
    tagged = tag_tagged_items(&mut tree, start, end, ITEMS as c_ulong, XA_MARK_0, XA_MARK_1);

    // printf("checking copied tags\n");
    assert!(tagged == count);
    check_copied_tags(&mut tree, start, end, idx.as_mut_ptr(), ITEMS as c_int, 0, 1);

    /* Copy tags in several rounds */
    // printf("\ncopying tags...\n");
    tmp = (rand() as c_ulong) % (count / 10 + 2);
    tagged = tag_tagged_items(&mut tree, start, end, tmp, XA_MARK_0, XA_MARK_2);
    assert!(tagged == count);

    // printf("%lu %lu %lu\n", tagged, tmp, count);
    // printf("checking copied tags\n");
    check_copied_tags(&mut tree, start, end, idx.as_mut_ptr(), ITEMS as c_int, 0, 2);
    verify_tag_consistency(&mut tree, 0);
    verify_tag_consistency(&mut tree, 1);
    verify_tag_consistency(&mut tree, 2);
    // printf("\n");
    item_kill_tree(&mut tree);
}

unsafe fn single_thread_tests(long_run: bool) {
    let mut i: c_int;

    printv(
        1,
        c"starting single_thread_tests: %d allocated, preempt %d\n".as_ptr(),
        nr_allocated,
        preempt_count,
    );
    multiorder_checks();
    rcu_barrier();
    printv(
        2,
        c"after multiorder_check: %d allocated, preempt %d\n".as_ptr(),
        nr_allocated,
        preempt_count,
    );
    tag_check();
    rcu_barrier();
    printv(
        2,
        c"after tag_check: %d allocated, preempt %d\n".as_ptr(),
        nr_allocated,
        preempt_count,
    );
    gang_check();
    rcu_barrier();
    printv(
        2,
        c"after gang_check: %d allocated, preempt %d\n".as_ptr(),
        nr_allocated,
        preempt_count,
    );
    add_and_check();
    rcu_barrier();
    printv(
        2,
        c"after add_and_check: %d allocated, preempt %d\n".as_ptr(),
        nr_allocated,
        preempt_count,
    );
    dynamic_height_check();
    rcu_barrier();
    printv(
        2,
        c"after dynamic_height_check: %d allocated, preempt %d\n".as_ptr(),
        nr_allocated,
        preempt_count,
    );
    idr_checks();
    ida_tests();
    rcu_barrier();
    printv(
        2,
        c"after idr_checks: %d allocated, preempt %d\n".as_ptr(),
        nr_allocated,
        preempt_count,
    );
    big_gang_check(long_run);
    rcu_barrier();
    printv(
        2,
        c"after big_gang_check: %d allocated, preempt %d\n".as_ptr(),
        nr_allocated,
        preempt_count,
    );
    i = 0;
    while i < if long_run { 2000 } else { 3 } {
        copy_tag_check();
        printv(2, c"%d ".as_ptr(), i);
        fflush(stdout);
        i += 1;
    }
    rcu_barrier();
    printv(
        2,
        c"after copy_tag_check: %d allocated, preempt %d\n".as_ptr(),
        nr_allocated,
        preempt_count,
    );
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut long_run = false;
    let mut opt: c_int;
    let mut seed: u32 = time(core::ptr::null_mut()) as u32;

    loop {
        opt = getopt(argc, argv, c"ls:v".as_ptr());
        if opt == -1 {
            break;
        }
        if opt == 'l' as c_int {
            long_run = true;
        } else if opt == 's' as c_int {
            seed = strtoul(optarg, core::ptr::null_mut(), 0) as u32;
        } else if opt == 'v' as c_int {
            test_verbose += 1;
        }
    }

    printf(c"random seed %u\n".as_ptr(), seed);
    srand(seed);

    printf(c"running tests\n".as_ptr());

    rcu_register_thread();
    radix_tree_init();

    xarray_tests();
    regression1_test();
    regression2_test();
    regression3_test();
    regression4_test();
    iteration_test(0, 10 + 90 * long_run as c_int);
    iteration_test(7, 10 + 90 * long_run as c_int);
    iteration_test2(10 + 90 * long_run as c_int);
    single_thread_tests(long_run);

    /* Free any remaining preallocated nodes */
    radix_tree_cpu_dead(0);

    benchmark();

    rcu_barrier();
    printv(
        2,
        c"after rcu_barrier: %d allocated, preempt %d\n".as_ptr(),
        nr_allocated,
        preempt_count,
    );
    rcu_unregister_thread();

    printf(c"tests completed\n".as_ptr());

    exit(0);
}
