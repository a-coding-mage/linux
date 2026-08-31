// SPDX-License-Identifier: GPL-2.0

// C dependency: #include "tests.h" provides `struct test_suite`.

unsafe extern "C" {
    pub fn create_script_test_suites() -> *mut *mut test_suite;
}
