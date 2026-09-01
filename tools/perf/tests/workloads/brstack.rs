/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int};
use core::ptr::{addr_of, addr_of_mut, read_volatile, write_volatile};

// C dependencies:
// #include <stdlib.h>
// #include "../tests.h"
unsafe extern "C" {
    fn atoi(nptr: *const c_char) -> c_int;
}

const BENCH_RUNS: c_int = 999999;

static mut cnt: c_int = 0;

unsafe fn cnt_post_inc() -> c_int {
    let old = unsafe { read_volatile(addr_of!(cnt)) };
    unsafe { write_volatile(addr_of_mut!(cnt), old.wrapping_add(1)) };
    old
}

fn brstack_bar() {
} /* return */

fn brstack_foo() {
    brstack_bar(); /* call */
} /* return */

fn brstack_bench() {
    let brstack_foo_ind: fn() = brstack_foo;

    if unsafe { cnt_post_inc() } % 3 != 0 {
        /* branch (cond) */
        brstack_foo(); /* call */
    }
    brstack_bar(); /* call */
    brstack_foo_ind(); /* call (ind) */
}

unsafe fn brstack(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut num_loops = BENCH_RUNS;

    if argc > 0 {
        num_loops = unsafe { atoi(*argv) };
    }

    loop {
        if unsafe { cnt_post_inc() } > num_loops {
            break;
        }
        brstack_bench(); /* call */
    } /* branch (uncond) */
    0
}

DEFINE_WORKLOAD!(brstack);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
