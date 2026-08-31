// SPDX-License-Identifier: GPL-2.0
// C dependencies: linux/membarrier.h, syscall.h, stdio.h, errno.h, string.h,
// pthread.h, and "membarrier_test_impl.h".

use core::ffi::{c_char, c_int};

extern "C" {
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn test_membarrier_get_registrations(cmd: c_int);
    fn test_membarrier_query();
    fn test_membarrier_fail();
    fn test_membarrier_success();
    fn ksft_exit_pass() -> !;
}

fn main() {
    let argc: c_int = std::env::args().count() as c_int;
    let argv: *mut *mut c_char = core::ptr::null_mut();
    let _ = (argc, argv);

    unsafe {
        ksft_print_header();
        ksft_set_plan(18);

        test_membarrier_get_registrations(0);

        test_membarrier_query();

        test_membarrier_fail();

        test_membarrier_success();

        test_membarrier_get_registrations(0);

        ksft_exit_pass();
    }
}
