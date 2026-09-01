// SPDX-License-Identifier: GPL-2.0-only
/*
 * benchmark.c:
 * Author: Konstantin Khlebnikov <koct9i@gmail.com>
 */
// C dependencies: <linux/radix-tree.h>, <linux/slab.h>, <linux/errno.h>,
// <time.h>, "test.h"

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const NSEC_PER_SEC: c_long = 1000000000;
const CLOCK_MONOTONIC: c_int = 1;
const GFP_KERNEL: c_int = 0;

#[repr(C)]
pub struct radix_tree_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct radix_tree_iter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

unsafe extern "C" {
    static RADIX_TREE_MAP_SHIFT: c_int;

    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn item_insert(root: *mut radix_tree_root, index: c_ulong);
    fn item_delete(root: *mut radix_tree_root, index: c_ulong);
    fn item_kill_tree(root: *mut radix_tree_root);
    fn radix_tree_tag_set(root: *mut radix_tree_root, index: c_ulong, tag: c_int);
    fn rcu_barrier();
    fn printv(level: c_int, fmt: *const c_char, ...);

    fn radix_tree_for_each_tagged(
        slot: *mut *mut *mut c_void,
        root: *mut radix_tree_root,
        iter: *mut radix_tree_iter,
        start: c_ulong,
        tag: c_int,
    ) -> bool;
    fn radix_tree_for_each_slot(
        slot: *mut *mut *mut c_void,
        root: *mut radix_tree_root,
        iter: *mut radix_tree_iter,
        start: c_ulong,
    ) -> bool;
}

unsafe fn benchmark_iter(root: *mut radix_tree_root, tagged: bool) -> i64 {
    let mut sink: c_ulong = 0;
    let mut iter: radix_tree_iter = core::mem::zeroed();
    let mut start: timespec = core::mem::zeroed();
    let mut finish: timespec = core::mem::zeroed();
    let mut nsec: i64;
    let mut l: c_int;
    let mut loops: c_int = 1;
    let mut slot: *mut c_void = core::ptr::null_mut();

    // Original C conditionally repeats this block under #ifdef BENCHMARK.
    loop {
        clock_gettime(CLOCK_MONOTONIC, &mut start);
        l = 0;
        while l < loops {
            if tagged {
                while radix_tree_for_each_tagged(&mut slot, root, &mut iter, 0, 0) {
                    sink ^= (&mut slot as *mut *mut c_void) as c_ulong;
                }
            } else {
                while radix_tree_for_each_slot(&mut slot, root, &mut iter, 0) {
                    sink ^= (&mut slot as *mut *mut c_void) as c_ulong;
                }
            }
            l += 1;
        }
        clock_gettime(CLOCK_MONOTONIC, &mut finish);

        nsec = ((finish.tv_sec - start.tv_sec) * NSEC_PER_SEC
            + (finish.tv_nsec - start.tv_nsec)) as i64;

        // #ifdef BENCHMARK
        if cfg!(feature = "BENCHMARK") && loops == 1 && nsec * 5 < NSEC_PER_SEC as i64 {
            loops = (NSEC_PER_SEC as i64 / nsec / 4 + 1) as c_int;
            continue;
        }
        // #endif

        break;
    }

    nsec /= loops as i64;
    let _ = core::ptr::read_volatile(&sink);
    nsec
}

unsafe fn benchmark_insert(root: *mut radix_tree_root, size: c_ulong, step: c_ulong) {
    let mut start: timespec = core::mem::zeroed();
    let mut finish: timespec = core::mem::zeroed();
    let mut index: c_ulong;
    let nsec: i64;

    clock_gettime(CLOCK_MONOTONIC, &mut start);

    index = 0;
    while index < size {
        item_insert(root, index);
        index = index.wrapping_add(step);
    }

    clock_gettime(CLOCK_MONOTONIC, &mut finish);

    nsec = ((finish.tv_sec - start.tv_sec) * NSEC_PER_SEC
        + (finish.tv_nsec - start.tv_nsec)) as i64;

    printv(
        2,
        b"Size: %8ld, step: %8ld, insertion: %15lld ns\n\0".as_ptr() as *const c_char,
        size,
        step,
        nsec,
    );
}

unsafe fn benchmark_tagging(root: *mut radix_tree_root, size: c_ulong, step: c_ulong) {
    let mut start: timespec = core::mem::zeroed();
    let mut finish: timespec = core::mem::zeroed();
    let mut index: c_ulong;
    let nsec: i64;

    clock_gettime(CLOCK_MONOTONIC, &mut start);

    index = 0;
    while index < size {
        radix_tree_tag_set(root, index, 0);
        index = index.wrapping_add(step);
    }

    clock_gettime(CLOCK_MONOTONIC, &mut finish);

    nsec = ((finish.tv_sec - start.tv_sec) * NSEC_PER_SEC
        + (finish.tv_nsec - start.tv_nsec)) as i64;

    printv(
        2,
        b"Size: %8ld, step: %8ld, tagging: %17lld ns\n\0".as_ptr() as *const c_char,
        size,
        step,
        nsec,
    );
}

unsafe fn benchmark_delete(root: *mut radix_tree_root, size: c_ulong, step: c_ulong) {
    let mut start: timespec = core::mem::zeroed();
    let mut finish: timespec = core::mem::zeroed();
    let mut index: c_ulong;
    let nsec: i64;

    clock_gettime(CLOCK_MONOTONIC, &mut start);

    index = 0;
    while index < size {
        item_delete(root, index);
        index = index.wrapping_add(step);
    }

    clock_gettime(CLOCK_MONOTONIC, &mut finish);

    nsec = ((finish.tv_sec - start.tv_sec) * NSEC_PER_SEC
        + (finish.tv_nsec - start.tv_nsec)) as i64;

    printv(
        2,
        b"Size: %8ld, step: %8ld, deletion: %16lld ns\n\0".as_ptr() as *const c_char,
        size,
        step,
        nsec,
    );
}

unsafe fn benchmark_size(size: c_ulong, step: c_ulong) {
    // RADIX_TREE(tree, GFP_KERNEL);
    let mut tree: radix_tree_root = core::mem::zeroed();
    let normal: i64;
    let tagged: i64;

    let _ = GFP_KERNEL;

    benchmark_insert(&mut tree, size, step);
    benchmark_tagging(&mut tree, size, step);

    tagged = benchmark_iter(&mut tree, true);
    normal = benchmark_iter(&mut tree, false);

    printv(
        2,
        b"Size: %8ld, step: %8ld, tagged iteration: %8lld ns\n\0".as_ptr() as *const c_char,
        size,
        step,
        tagged,
    );
    printv(
        2,
        b"Size: %8ld, step: %8ld, normal iteration: %8lld ns\n\0".as_ptr() as *const c_char,
        size,
        step,
        normal,
    );

    benchmark_delete(&mut tree, size, step);

    item_kill_tree(&mut tree);
    rcu_barrier();
}

#[no_mangle]
pub unsafe extern "C" fn benchmark() {
    let size: [c_ulong; 3] = [1 << 10, 1 << 20, 0];
    let step: [c_ulong; 12] = [1, 2, 7, 15, 63, 64, 65, 128, 256, 512, 12345, 0];
    let mut c: c_int;
    let mut s: c_int;

    printv(1, b"starting benchmarks\n\0".as_ptr() as *const c_char);
    printv(
        1,
        b"RADIX_TREE_MAP_SHIFT = %d\n\0".as_ptr() as *const c_char,
        RADIX_TREE_MAP_SHIFT,
    );

    c = 0;
    while size[c as usize] != 0 {
        s = 0;
        while step[s as usize] != 0 {
            benchmark_size(size[c as usize], step[s as usize]);
            s += 1;
        }
        c += 1;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
