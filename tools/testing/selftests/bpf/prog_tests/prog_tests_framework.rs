// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

// C dependencies:
// #include "test_progs.h"
// #include "testing_helpers.h"

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct test_state {
    pub error_cnt: c_int,
    pub sub_succ_cnt: c_int,
    pub skip_cnt: c_int,
    pub subtest_num: c_int,
    pub log_buf: *const c_char,
}

#[repr(C)]
pub struct expect_msg {
    pub substr: *const c_char,
    pub negative: bool,
}

#[repr(C)]
pub struct expected_msgs {
    pub patterns: *mut expect_msg,
    pub cnt: c_int,
}

#[repr(C)]
pub struct test_env {
    pub test_state: *mut test_state,
    pub subtest_state: *mut test_state,
}

unsafe extern "C" {
    static mut env: test_env;

    fn test__start_subtest(name: *const c_char) -> bool;
    fn test__end_subtest();
    fn test__skip();
    fn test__fail();
    fn validate_msgs(
        log: *const c_char,
        msgs: *mut expected_msgs,
        emit: Option<unsafe extern "C" fn(buf: *const c_char, force: bool)>,
    );
    fn fflush(stream: *mut core::ffi::c_void) -> c_int;

    static mut stderr: *mut core::ffi::c_void;

    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_HAS_SUBSTR(str_: *const c_char, substr: *const c_char, name: *const c_char) -> bool;
    fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, name: *const c_char) -> bool;
}

unsafe fn clear_test_state(state: *mut test_state) {
    unsafe {
        (*state).error_cnt = 0;
        (*state).sub_succ_cnt = 0;
        (*state).skip_cnt = 0;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_prog_tests_framework() {
    unsafe {
        let state: *mut test_state = env.test_state;

        /* in all the ASSERT calls below we need to return on the first
         * error due to the fact that we are cleaning the test state after
         * each dummy subtest
         */

        /* test we properly count skipped tests with subtests */
        if test__start_subtest(c"test_good_subtest".as_ptr()) {
            test__end_subtest();
        }
        if !ASSERT_EQ((*state).skip_cnt, 0, c"skip_cnt_check".as_ptr()) {
            return;
        }
        if !ASSERT_EQ((*state).error_cnt, 0, c"error_cnt_check".as_ptr()) {
            return;
        }
        if !ASSERT_EQ((*state).subtest_num, 1, c"subtest_num_check".as_ptr()) {
            return;
        }
        clear_test_state(state);

        if test__start_subtest(c"test_skip_subtest".as_ptr()) {
            test__skip();
            test__end_subtest();
        }
        if test__start_subtest(c"test_skip_subtest".as_ptr()) {
            test__skip();
            test__end_subtest();
        }
        if !ASSERT_EQ((*state).skip_cnt, 2, c"skip_cnt_check".as_ptr()) {
            return;
        }
        if !ASSERT_EQ((*state).subtest_num, 3, c"subtest_num_check".as_ptr()) {
            return;
        }
        clear_test_state(state);

        if test__start_subtest(c"test_fail_subtest".as_ptr()) {
            test__fail();
            test__end_subtest();
        }
        if !ASSERT_EQ((*state).error_cnt, 1, c"error_cnt_check".as_ptr()) {
            return;
        }
        if !ASSERT_EQ((*state).subtest_num, 4, c"subtest_num_check".as_ptr()) {
            return;
        }
        clear_test_state(state);
    }
}

unsafe extern "C" fn dummy_emit(_buf: *const c_char, _force: bool) {}

#[repr(C)]
struct expected_msgs_case {
    name: *const c_char,
    log: *const c_char,
    expected: *const c_char,
    pats: *mut expect_msg,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_prog_tests_framework_expected_msgs() {
    unsafe {
        let mut pats_simple_ok = [
            expect_msg {
                substr: c"aaa".as_ptr(),
                negative: false,
            },
            expect_msg {
                substr: c"ccc".as_ptr(),
                negative: false,
            },
            expect_msg {
                substr: core::ptr::null(),
                negative: false,
            },
        ];
        let mut pats_simple_fail = [
            expect_msg {
                substr: c"aaa".as_ptr(),
                negative: false,
            },
            expect_msg {
                substr: c"ccc".as_ptr(),
                negative: false,
            },
            expect_msg {
                substr: core::ptr::null(),
                negative: false,
            },
        ];
        let mut pats_negative_ok_mid = [
            expect_msg {
                substr: c"aaa".as_ptr(),
                negative: false,
            },
            expect_msg {
                substr: c"foo".as_ptr(),
                negative: true,
            },
            expect_msg {
                substr: c"bar".as_ptr(),
                negative: true,
            },
            expect_msg {
                substr: c"ccc".as_ptr(),
                negative: false,
            },
            expect_msg {
                substr: core::ptr::null(),
                negative: false,
            },
        ];
        let mut pats_negative_ok_tail = [
            expect_msg {
                substr: c"aaa".as_ptr(),
                negative: false,
            },
            expect_msg {
                substr: c"foo".as_ptr(),
                negative: true,
            },
            expect_msg {
                substr: core::ptr::null(),
                negative: false,
            },
        ];
        let mut pats_negative_ok_head = [
            expect_msg {
                substr: c"foo".as_ptr(),
                negative: true,
            },
            expect_msg {
                substr: c"ccc".as_ptr(),
                negative: false,
            },
            expect_msg {
                substr: core::ptr::null(),
                negative: false,
            },
        ];
        let mut pats_negative_fail_head = [
            expect_msg {
                substr: c"aaa".as_ptr(),
                negative: true,
            },
            expect_msg {
                substr: c"bbb".as_ptr(),
                negative: false,
            },
            expect_msg {
                substr: core::ptr::null(),
                negative: false,
            },
        ];
        let mut pats_negative_fail_tail = [
            expect_msg {
                substr: c"bbb".as_ptr(),
                negative: false,
            },
            expect_msg {
                substr: c"ccc".as_ptr(),
                negative: true,
            },
            expect_msg {
                substr: core::ptr::null(),
                negative: false,
            },
        ];
        let mut pats_negative_fail_mid_1 = [
            expect_msg {
                substr: c"aaa".as_ptr(),
                negative: false,
            },
            expect_msg {
                substr: c"bbb".as_ptr(),
                negative: true,
            },
            expect_msg {
                substr: c"ccc".as_ptr(),
                negative: false,
            },
            expect_msg {
                substr: core::ptr::null(),
                negative: false,
            },
        ];
        let mut pats_negative_fail_mid_2 = [
            expect_msg {
                substr: c"aaa".as_ptr(),
                negative: false,
            },
            expect_msg {
                substr: c"222".as_ptr(),
                negative: true,
            },
            expect_msg {
                substr: c"bbb".as_ptr(),
                negative: true,
            },
            expect_msg {
                substr: c"ccc".as_ptr(),
                negative: false,
            },
            expect_msg {
                substr: core::ptr::null(),
                negative: false,
            },
        ];

        let cases = [
            expected_msgs_case {
                name: c"simple-ok".as_ptr(),
                log: c"aaabbbccc".as_ptr(),
                expected: core::ptr::null(),
                pats: pats_simple_ok.as_mut_ptr(),
            },
            expected_msgs_case {
                name: c"simple-fail".as_ptr(),
                log: c"aaabbbddd".as_ptr(),
                expected: c"MATCHED    SUBSTR: 'aaa'\nEXPECTED   SUBSTR: 'ccc'\n".as_ptr(),
                pats: pats_simple_fail.as_mut_ptr(),
            },
            expected_msgs_case {
                name: c"negative-ok-mid".as_ptr(),
                log: c"aaabbbccc".as_ptr(),
                expected: core::ptr::null(),
                pats: pats_negative_ok_mid.as_mut_ptr(),
            },
            expected_msgs_case {
                name: c"negative-ok-tail".as_ptr(),
                log: c"aaabbbccc".as_ptr(),
                expected: core::ptr::null(),
                pats: pats_negative_ok_tail.as_mut_ptr(),
            },
            expected_msgs_case {
                name: c"negative-ok-head".as_ptr(),
                log: c"aaabbbccc".as_ptr(),
                expected: core::ptr::null(),
                pats: pats_negative_ok_head.as_mut_ptr(),
            },
            expected_msgs_case {
                name: c"negative-fail-head".as_ptr(),
                log: c"aaabbbccc".as_ptr(),
                expected: c"UNEXPECTED SUBSTR: 'aaa'\n".as_ptr(),
                pats: pats_negative_fail_head.as_mut_ptr(),
            },
            expected_msgs_case {
                name: c"negative-fail-tail".as_ptr(),
                log: c"aaabbbccc".as_ptr(),
                expected: c"UNEXPECTED SUBSTR: 'ccc'\n".as_ptr(),
                pats: pats_negative_fail_tail.as_mut_ptr(),
            },
            expected_msgs_case {
                name: c"negative-fail-mid-1".as_ptr(),
                log: c"aaabbbccc".as_ptr(),
                expected: c"UNEXPECTED SUBSTR: 'bbb'\n".as_ptr(),
                pats: pats_negative_fail_mid_1.as_mut_ptr(),
            },
            expected_msgs_case {
                name: c"negative-fail-mid-2".as_ptr(),
                log: c"aaabbb222ccc".as_ptr(),
                expected: c"UNEXPECTED SUBSTR: '222'\n".as_ptr(),
                pats: pats_negative_fail_mid_2.as_mut_ptr(),
            },
        ];

        let mut msgs: expected_msgs = core::mem::zeroed();
        let mut i: c_int = 0;
        while (i as usize) < cases.len() {
            if test__start_subtest(cases[i as usize].name) {
                let error_cnt: c_int = (*env.subtest_state).error_cnt;
                msgs.patterns = cases[i as usize].pats;
                msgs.cnt = 0;
                let mut j: c_int = 0;
                while !(*cases[i as usize].pats.add(j as usize)).substr.is_null() {
                    msgs.cnt += 1;
                    j += 1;
                }
                validate_msgs(cases[i as usize].log, &mut msgs, Some(dummy_emit));
                fflush(stderr);
                (*env.subtest_state).error_cnt = error_cnt;
                if !cases[i as usize].expected.is_null() {
                    ASSERT_HAS_SUBSTR(
                        (*env.subtest_state).log_buf,
                        cases[i as usize].expected,
                        c"expected output".as_ptr(),
                    );
                } else {
                    ASSERT_STREQ(
                        (*env.subtest_state).log_buf,
                        c"".as_ptr(),
                        c"expected no output".as_ptr(),
                    );
                }
                test__end_subtest();
            }
            i += 1;
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
