// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/evsel-tp-sched.c
// C dependencies: <linux/err.h>, <event-parse.h>, "evsel.h", "tests.h", "debug.h"

use core::ffi::{c_char, c_int, c_long};

const TEP_FIELD_IS_SIGNED: c_int = 1 << 0;
const EACCES: c_long = 13;

extern "C" {
    static TEST_OK: c_int;
    static TEST_FAIL: c_int;
    static TEST_SKIP: c_int;
}

#[repr(C)]
pub struct evsel {
    pub name: *const c_char,
}

#[repr(C)]
pub struct tep_format_field {
    pub flags: c_int,
    pub size: c_int,
}

#[repr(C)]
pub struct test_suite {
    pub desc: *const c_char,
    pub test_cases: *mut test_case,
}

#[repr(C)]
pub struct test_case {
    pub name: *const c_char,
    pub run_case: Option<unsafe extern "C" fn(*mut test_suite, c_int) -> c_int>,
    pub reason: *const c_char,
}

extern "C" {
    fn evsel__field(evsel: *mut evsel, name: *const c_char) -> *mut tep_format_field;
    fn evsel__newtp(sys: *const c_char, name: *const c_char) -> *mut evsel;
    fn evsel__put(evsel: *mut evsel);
    fn pr_debug(fmt: *const c_char, ...);
}

#[inline]
unsafe fn IS_ERR(ptr: *const evsel) -> bool {
    (ptr as isize) >= -(4095isize)
}

#[inline]
unsafe fn PTR_ERR(ptr: *const evsel) -> c_long {
    ptr as c_long
}

unsafe extern "C" fn evsel__test_field(
    evsel: *mut evsel,
    name: *const c_char,
    size: c_int,
    should_be_signed: bool,
) -> c_int {
    let field = evsel__field(evsel, name);
    let is_signed: c_int;
    let mut ret: c_int = 0;

    if field.is_null() {
        pr_debug(
            b"%s: \"%s\" field not found!\n\0".as_ptr() as *const c_char,
            (*evsel).name,
            name,
        );
        return -1;
    }

    is_signed = (((*field).flags & TEP_FIELD_IS_SIGNED) != 0) as c_int;
    if should_be_signed && is_signed == 0 {
        pr_debug(
            b"%s: \"%s\" signedness(%d) is wrong, should be %d\n\0".as_ptr() as *const c_char,
            (*evsel).name,
            name,
            is_signed,
            should_be_signed as c_int,
        );
        ret = -1;
    }

    if (*field).size != size {
        pr_debug(
            b"%s: \"%s\" size (%d) should be %d!\n\0".as_ptr() as *const c_char,
            (*evsel).name,
            name,
            (*field).size,
            size,
        );
        ret = -1;
    }

    ret
}

unsafe extern "C" fn test__perf_evsel__tp_sched_test(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut evsel = evsel__newtp(
        b"sched\0".as_ptr() as *const c_char,
        b"sched_switch\0".as_ptr() as *const c_char,
    );
    let mut ret: c_int = TEST_OK;

    if IS_ERR(evsel) {
        pr_debug(
            b"evsel__newtp failed with %ld\n\0".as_ptr() as *const c_char,
            PTR_ERR(evsel),
        );
        return if PTR_ERR(evsel) == -EACCES {
            TEST_SKIP
        } else {
            TEST_FAIL
        };
    }

    if evsel__test_field(evsel, b"prev_comm\0".as_ptr() as *const c_char, 16, false) != 0 {
        ret = TEST_FAIL;
    }

    if evsel__test_field(evsel, b"prev_pid\0".as_ptr() as *const c_char, 4, true) != 0 {
        ret = TEST_FAIL;
    }

    if evsel__test_field(evsel, b"prev_prio\0".as_ptr() as *const c_char, 4, true) != 0 {
        ret = TEST_FAIL;
    }

    if evsel__test_field(
        evsel,
        b"prev_state\0".as_ptr() as *const c_char,
        core::mem::size_of::<c_long>() as c_int,
        true,
    ) != 0
    {
        ret = TEST_FAIL;
    }

    if evsel__test_field(evsel, b"next_comm\0".as_ptr() as *const c_char, 16, false) != 0 {
        ret = TEST_FAIL;
    }

    if evsel__test_field(evsel, b"next_pid\0".as_ptr() as *const c_char, 4, true) != 0 {
        ret = TEST_FAIL;
    }

    if evsel__test_field(evsel, b"next_prio\0".as_ptr() as *const c_char, 4, true) != 0 {
        ret = TEST_FAIL;
    }

    evsel__put(evsel);

    evsel = evsel__newtp(
        b"sched\0".as_ptr() as *const c_char,
        b"sched_wakeup\0".as_ptr() as *const c_char,
    );

    if IS_ERR(evsel) {
        pr_debug(
            b"evsel__newtp failed with %ld\n\0".as_ptr() as *const c_char,
            PTR_ERR(evsel),
        );
        return TEST_FAIL;
    }

    if evsel__test_field(evsel, b"comm\0".as_ptr() as *const c_char, 16, false) != 0 {
        ret = TEST_FAIL;
    }

    if evsel__test_field(evsel, b"pid\0".as_ptr() as *const c_char, 4, true) != 0 {
        ret = TEST_FAIL;
    }

    if evsel__test_field(evsel, b"prio\0".as_ptr() as *const c_char, 4, true) != 0 {
        ret = TEST_FAIL;
    }

    if evsel__test_field(evsel, b"target_cpu\0".as_ptr() as *const c_char, 4, true) != 0 {
        ret = TEST_FAIL;
    }

    evsel__put(evsel);
    ret
}

#[no_mangle]
pub static mut tests__perf_evsel__tp_sched_test: [test_case; 2] = [
    test_case {
        name: b"Parse sched tracepoints fields\0".as_ptr() as *const c_char,
        run_case: Some(test__perf_evsel__tp_sched_test),
        reason: b"permissions\0".as_ptr() as *const c_char,
    },
    test_case {
        name: core::ptr::null(),
        run_case: None,
        reason: core::ptr::null(),
    },
];

#[no_mangle]
pub static mut suite__perf_evsel__tp_sched_test: test_suite = test_suite {
    desc: b"Parse sched tracepoints fields\0".as_ptr() as *const c_char,
    test_cases: unsafe { tests__perf_evsel__tp_sched_test.as_mut_ptr() },
};
