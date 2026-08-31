// SPDX-License-Identifier: GPL-2.0
/*
 * rbtree_test.c: Userspace Red Black Tree test-suite
 * Copyright (c) 2025 Wei Yang <richard.weiyang@gmail.com>
 */
/* C dependencies: <linux/init.h>, <linux/math64.h>, <linux/kern_levels.h>,
 * "shared.h", and "../../../lib/rbtree_test.c".
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

unsafe extern "C" {
    static mut stderr: *mut c_void;
    static mut optarg: *mut c_char;

    static mut nnodes: c_ulong;
    static mut perf_loops: c_ulong;
    static mut check_loops: c_ulong;
    static mut seed: c_ulong;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;

    fn rbtree_test_init();
    fn rbtree_test_exit();
}

#[no_mangle]
pub unsafe extern "C" fn usage() -> c_int {
    unsafe {
        fprintf(stderr, c"Userland rbtree test cases\n".as_ptr());
        fprintf(stderr, c"  -n: Number of nodes in the rb-tree\n".as_ptr());
        fprintf(
            stderr,
            c"  -p: Number of iterations modifying the rb-tree\n".as_ptr(),
        );
        fprintf(
            stderr,
            c"  -c: Number of iterations modifying and verifying the rb-tree\n".as_ptr(),
        );
        fprintf(stderr, c"  -r: Random seed\n".as_ptr());
        exit(-1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn rbtree_tests() {
    unsafe {
        rbtree_test_init();
        rbtree_test_exit();
    }
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut opt: c_int;

    loop {
        unsafe {
            opt = getopt(argc, argv, c"n:p:c:r:".as_ptr());
        }
        if opt == -1 {
            break;
        }

        unsafe {
            if opt == 'n' as c_int {
                nnodes = strtoul(optarg, ptr::null_mut(), 0);
            } else if opt == 'p' as c_int {
                perf_loops = strtoul(optarg, ptr::null_mut(), 0);
            } else if opt == 'c' as c_int {
                check_loops = strtoul(optarg, ptr::null_mut(), 0);
            } else if opt == 'r' as c_int {
                seed = strtoul(optarg, ptr::null_mut(), 0);
            } else {
                usage();
            }
        }
    }

    unsafe {
        rbtree_tests();
    }
    0
}
