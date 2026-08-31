// SPDX-License-Identifier: GPL-2.0-only

// C dependencies:
// #include "kselftest_harness.h"
// #include "v_helpers.h"

use std::os::raw::{c_char, c_int};

const NEXT_PROGRAM: *const c_char = b"./v_exec_initval_nolibc\0".as_ptr() as *const c_char;

extern "C" {
    fn is_vector_supported() -> bool;
    fn is_xtheadvector_supported() -> bool;
    fn launch_test(program: *const c_char, arg: c_int, xtheadvector: c_int) -> c_int;
}

#[test]
fn v_initval() {
    let mut xtheadvector: c_int = 0;

    unsafe {
        if !is_vector_supported() {
            if is_xtheadvector_supported() {
                xtheadvector = 1;
            } else {
                // C: SKIP(return, "Vector not supported");
                return;
            }
        }

        assert_eq!(0, launch_test(NEXT_PROGRAM, 0, xtheadvector));
    }
}

// C: TEST_HARNESS_MAIN
