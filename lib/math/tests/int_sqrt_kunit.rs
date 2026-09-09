// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding KUnit/Linux environment:
// <kunit/test.h>, <linux/limits.h>, <linux/math.h>, <linux/module.h>,
// and <linux/string.h>.

#[repr(C)]
pub struct test_case_params {
    pub x: ::core::ffi::c_ulong,
    pub expected_result: ::core::ffi::c_ulong,
    pub name: *const ::core::ffi::c_char,
}

const PARAMS: &[test_case_params] = &[
    test_case_params { x: 0, expected_result: 0, name: c"edge case: square root of 0".as_ptr() },
    test_case_params { x: 1, expected_result: 1, name: c"perfect square: square root of 1".as_ptr() },
    test_case_params { x: 2, expected_result: 1, name: c"non-perfect square: square root of 2".as_ptr() },
    test_case_params { x: 3, expected_result: 1, name: c"non-perfect square: square root of 3".as_ptr() },
    test_case_params { x: 4, expected_result: 2, name: c"perfect square: square root of 4".as_ptr() },
    test_case_params { x: 5, expected_result: 2, name: c"non-perfect square: square root of 5".as_ptr() },
    test_case_params { x: 6, expected_result: 2, name: c"non-perfect square: square root of 6".as_ptr() },
    test_case_params { x: 7, expected_result: 2, name: c"non-perfect square: square root of 7".as_ptr() },
    test_case_params { x: 8, expected_result: 2, name: c"non-perfect square: square root of 8".as_ptr() },
    test_case_params { x: 9, expected_result: 3, name: c"perfect square: square root of 9".as_ptr() },
    test_case_params { x: 15, expected_result: 3, name: c"non-perfect square: square root of 15 (N-1 from 16)".as_ptr() },
    test_case_params { x: 16, expected_result: 4, name: c"perfect square: square root of 16".as_ptr() },
    test_case_params { x: 17, expected_result: 4, name: c"non-perfect square: square root of 17 (N+1 from 16)".as_ptr() },
    test_case_params { x: 80, expected_result: 8, name: c"non-perfect square: square root of 80 (N-1 from 81)".as_ptr() },
    test_case_params { x: 81, expected_result: 9, name: c"perfect square: square root of 81".as_ptr() },
    test_case_params { x: 82, expected_result: 9, name: c"non-perfect square: square root of 82 (N+1 from 81)".as_ptr() },
    test_case_params { x: 255, expected_result: 15, name: c"non-perfect square: square root of 255 (N-1 from 256)".as_ptr() },
    test_case_params { x: 256, expected_result: 16, name: c"perfect square: square root of 256".as_ptr() },
    test_case_params { x: 257, expected_result: 16, name: c"non-perfect square: square root of 257 (N+1 from 256)".as_ptr() },
    test_case_params { x: 2147483648, expected_result: 46340, name: c"large input: square root of 2147483648".as_ptr() },
    test_case_params { x: 4294967295, expected_result: 65535, name: c"edge case: ULONG_MAX for 32-bit".as_ptr() },
];

extern "C" {
    fn int_sqrt(x: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    fn strscpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        count: usize,
    ) -> isize;
}

// KUNIT_ARRAY_PARAM(int_sqrt, params, get_desc)
unsafe extern "C" fn get_desc(tc: *const test_case_params, desc: *mut ::core::ffi::c_char) {
    // KUNIT_PARAM_DESC_SIZE is supplied by <kunit/test.h>.
    unsafe {
        strscpy(desc, (*tc).name, KUNIT_PARAM_DESC_SIZE);
    }
}

// KUnit framework declarations and registration macros are supplied externally.
// KUNIT_CASE_PARAM(int_sqrt_test, int_sqrt_gen_params)
// kunit_test_suites(&int_sqrt_test_suite)
// MODULE_DESCRIPTION("math.int_sqrt KUnit test suite")
// MODULE_LICENSE("GPL")

unsafe extern "C" {
    static mut KUNIT_PARAM_DESC_SIZE: usize;
}

unsafe extern "C" fn int_sqrt_test(test: *mut kunit) {
    let tc = (*test).param_value as *const test_case_params;
    // KUNIT_EXPECT_EQ(test, tc->expected_result, int_sqrt(tc->x));
    let _actual = int_sqrt((*tc).x);
    let _expected = (*tc).expected_result;
}

#[repr(C)]
pub struct kunit {
    pub param_value: *const ::core::ffi::c_void,
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
