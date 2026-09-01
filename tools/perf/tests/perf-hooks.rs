// SPDX-License-Identifier: GPL-2.0
// C dependencies: <signal.h>, <stdlib.h>, "tests.h", "debug.h", "perf-hooks.h"

use core::ffi::{c_char, c_int, c_void};

const SIGSEGV: c_int = 11;
const SIG_DFL: usize = 0;

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn pr_debug(fmt: *const c_char, ...) -> c_int;
    fn perf_hooks__recover();
    fn perf_hooks__set_hook(
        name: *const c_char,
        hook: Option<unsafe extern "C" fn(*mut c_void)>,
        hook_data: *mut c_void,
    );
    fn perf_hooks__invoke_test();
    fn perf_hooks__get_hook(name: *const c_char) -> *mut c_void;

    fn signal(signum: c_int, handler: usize) -> usize;
    fn raise(signum: c_int) -> c_int;
    fn exit(status: c_int) -> !;

    static TEST_FAIL: c_int;
    static TEST_OK: c_int;
}

unsafe extern "C" fn sigsegv_handler(_sig: c_int) {
    unsafe {
        pr_debug(c"SIGSEGV is observed as expected, try to recover.\n".as_ptr());
        perf_hooks__recover();
        signal(SIGSEGV, SIG_DFL);
        raise(SIGSEGV);
        exit(-1);
    }
}

unsafe extern "C" fn the_hook(_hook_flags: *mut c_void) {
    let hook_flags = _hook_flags as *mut c_int;

    unsafe {
        *hook_flags = 1234;

        /* Generate a segfault, test perf_hooks__recover */
        raise(SIGSEGV);
    }
}

unsafe extern "C" fn test__perf_hooks(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let mut hook_flags: c_int = 0;

    unsafe {
        signal(SIGSEGV, sigsegv_handler as usize);
        perf_hooks__set_hook(
            c"test".as_ptr(),
            Some(the_hook),
            &mut hook_flags as *mut c_int as *mut c_void,
        );
        perf_hooks__invoke_test();

        /* hook is triggered? */
        if hook_flags != 1234 {
            pr_debug(
                c"Setting failed: %d (%p)\n".as_ptr(),
                hook_flags,
                &mut hook_flags as *mut c_int,
            );
            return TEST_FAIL;
        }

        /* the buggy hook is removed? */
        if !perf_hooks__get_hook(c"test".as_ptr()).is_null() {
            return TEST_FAIL;
        }
        TEST_OK
    }
}

// C test harness registration:
// DEFINE_SUITE("perf hooks", perf_hooks);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
