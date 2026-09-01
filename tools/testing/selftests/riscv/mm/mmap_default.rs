// SPDX-License-Identifier: GPL-2.0-only

// C dependencies: <sys/mman.h>, <mmap_test.h>, "kselftest_harness.h"

unsafe extern "C" {
    fn memory_layout() -> i32;
    static TOP_DOWN: i32;
}

#[test]
fn default_rlimit() {
    unsafe {
        assert_eq!(TOP_DOWN, memory_layout());
    }
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
