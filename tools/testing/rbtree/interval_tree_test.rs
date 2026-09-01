// SPDX-License-Identifier: GPL-2.0
/*
 * interval_tree.c: Userspace Interval Tree test-suite
 * Copyright (c) 2025 Wei Yang <richard.weiyang@gmail.com>
 */

// C dependencies:
// #include <linux/math64.h>
// #include <linux/kern_levels.h>
// #include "shared.h"
// #include "maple-shared.h"
// #include "../../../lib/interval_tree_test.c"

use core::ffi::{c_char, c_int, c_ulong, c_void};

unsafe extern "C" {
    static mut stderr: *mut c_void;
    static mut optarg: *mut c_char;

    static mut nnodes: c_ulong;
    static mut perf_loops: c_ulong;
    static mut nsearches: c_ulong;
    static mut search_loops: c_ulong;
    static mut search_all: bool;
    static mut max_endpoint: c_ulong;
    static mut seed: c_ulong;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;

    fn interval_tree_test_init();
    fn interval_tree_test_exit();
    fn maple_tree_init();
}

pub unsafe fn usage() -> c_int {
    unsafe {
        fprintf(
            stderr,
            c"Userland interval tree test cases\n".as_ptr(),
        );
        fprintf(
            stderr,
            c"  -n: Number of nodes in the interval tree\n".as_ptr(),
        );
        fprintf(
            stderr,
            c"  -p: Number of iterations modifying the tree\n".as_ptr(),
        );
        fprintf(
            stderr,
            c"  -q: Number of searches to the interval tree\n".as_ptr(),
        );
        fprintf(
            stderr,
            c"  -s: Number of iterations searching the tree\n".as_ptr(),
        );
        fprintf(
            stderr,
            c"  -a: Searches will iterate all nodes in the tree\n".as_ptr(),
        );
        fprintf(
            stderr,
            c"  -m: Largest value for the interval's endpoint\n".as_ptr(),
        );
        fprintf(stderr, c"  -r: Random seed\n".as_ptr());
        exit(-1);
    }
}

pub unsafe fn interval_tree_tests() {
    unsafe {
        interval_tree_test_init();
        interval_tree_test_exit();
    }
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut opt: c_int;

    unsafe {
        loop {
            opt = getopt(argc, argv, c"n:p:q:s:am:r:".as_ptr());
            if opt == -1 {
                break;
            }

            if opt == b'n' as c_int {
                nnodes = strtoul(optarg, core::ptr::null_mut(), 0);
            } else if opt == b'p' as c_int {
                perf_loops = strtoul(optarg, core::ptr::null_mut(), 0);
            } else if opt == b'q' as c_int {
                nsearches = strtoul(optarg, core::ptr::null_mut(), 0);
            } else if opt == b's' as c_int {
                search_loops = strtoul(optarg, core::ptr::null_mut(), 0);
            } else if opt == b'a' as c_int {
                search_all = true;
            } else if opt == b'm' as c_int {
                max_endpoint = strtoul(optarg, core::ptr::null_mut(), 0);
            } else if opt == b'r' as c_int {
                seed = strtoul(optarg, core::ptr::null_mut(), 0);
            } else {
                usage();
            }
        }

        maple_tree_init();
        interval_tree_tests();
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
