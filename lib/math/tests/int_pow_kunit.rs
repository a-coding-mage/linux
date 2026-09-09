// SPDX-License-Identifier: GPL-2.0-only

// Translated from the Linux KUnit test source.  The KUnit and math symbols
// below are supplied by the surrounding kernel environment.

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct test_case_params {
    pub base: u64,
    pub exponent: u32,
    pub expected_result: u64,
    pub name: *const c_char,
}

#[repr(C)]
pub struct kunit {
    pub param_value: *const c_void,
}

unsafe extern "C" {
    fn int_pow(base: u64, exponent: u32) -> u64;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn kunit_expect_eq(test: *mut kunit, left: u64, right: u64);
}

pub const KUNIT_PARAM_DESC_SIZE: usize = 256;

#[allow(non_upper_case_globals)]
pub static params: [test_case_params; 9] = [
    test_case_params { base: 64, exponent: 0, expected_result: 1, name: c"Power of zero".as_ptr() },
    test_case_params { base: 64, exponent: 1, expected_result: 64, name: c"Power of one".as_ptr() },
    test_case_params { base: 0, exponent: 5, expected_result: 0, name: c"Base zero".as_ptr() },
    test_case_params { base: 1, exponent: 64, expected_result: 1, name: c"Base one".as_ptr() },
    test_case_params { base: 2, exponent: 2, expected_result: 4, name: c"Two squared".as_ptr() },
    test_case_params { base: 2, exponent: 3, expected_result: 8, name: c"Two cubed".as_ptr() },
    test_case_params { base: 5, exponent: 5, expected_result: 3125, name: c"Five raised to the fifth power".as_ptr() },
    test_case_params { base: u64::MAX, exponent: 1, expected_result: u64::MAX, name: c"Max base".as_ptr() },
    test_case_params { base: 2, exponent: 63, expected_result: 9223372036854775808u64, name: c"Large result".as_ptr() },
];

pub unsafe fn get_desc(tc: *const test_case_params, desc: *mut c_char) {
    unsafe {
        strscpy(desc, (*tc).name, KUNIT_PARAM_DESC_SIZE);
    }
}

// KUNIT_ARRAY_PARAM(int_pow, params, get_desc);
unsafe extern "C" {
    fn int_pow_gen_params() -> *const c_void;
}

pub unsafe fn int_pow_test(test: *mut kunit) {
    let tc = unsafe { (*test).param_value as *const test_case_params };
    unsafe {
        kunit_expect_eq(test, (*tc).expected_result, int_pow((*tc).base, (*tc).exponent));
    }
}

// KUNIT_CASE_PARAM(int_pow_test, int_pow_gen_params), followed by the empty
// sentinel entry in the C array.
#[repr(C)]
pub struct kunit_case {
    pub test: Option<unsafe fn(*mut kunit)>,
    pub generate_params: Option<unsafe extern "C" fn() -> *const c_void>,
}

#[allow(non_upper_case_globals)]
pub static mut math_int_pow_test_cases: [kunit_case; 2] = [
    kunit_case { test: Some(int_pow_test), generate_params: Some(int_pow_gen_params) },
    kunit_case { test: None, generate_params: None },
];

#[repr(C)]
pub struct kunit_suite {
    pub name: *const c_char,
    pub test_cases: *mut kunit_case,
}

#[allow(non_upper_case_globals)]
pub static mut int_pow_test_suite: kunit_suite = kunit_suite {
    name: c"math-int_pow".as_ptr(),
    test_cases: core::ptr::addr_of_mut!(math_int_pow_test_cases) as *mut kunit_case,
};

// kunit_test_suites(&int_pow_test_suite);
// MODULE_DESCRIPTION("math.int_pow KUnit test suite");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
