/* SPDX-License-Identifier: GPL-2.0 */
/*
 * KUnit API to save and access test attributes
 *
 * Copyright (C) 2023, Google LLC.
 * Author: Rae Moar <rmoar@google.com>
 */

/*
 * struct kunit_attr_filter - representation of attributes filter with the
 * attribute object and string input
 */
#[repr(C)]
pub struct kunit_attr_filter {
    pub attr: *mut kunit_attr,
    pub input: *mut core::ffi::c_char,
}

/*
 * Returns the name of the filter's attribute.
 */
pub unsafe extern "C" fn kunit_attr_filter_name(
    filter: kunit_attr_filter,
) -> *const core::ffi::c_char;

/*
 * Print all test attributes for a test case or suite.
 * Output format for test cases: "# <test_name>.<attribute>: <value>"
 * Output format for test suites: "# <attribute>: <value>"
 */
pub unsafe extern "C" fn kunit_print_attr(
    test_or_suite: *mut core::ffi::c_void,
    is_test: bool,
    test_level: core::ffi::c_uint,
);

/*
 * Returns the number of fitlers in input.
 */
pub unsafe extern "C" fn kunit_get_filter_count(
    input: *mut core::ffi::c_char,
) -> core::ffi::c_int;

/*
 * Parse attributes filter input and return an objects containing the
 * attribute object and the string input of the next filter.
 */
pub unsafe extern "C" fn kunit_next_attr_filter(
    filters: *mut *mut core::ffi::c_char,
    err: *mut core::ffi::c_int,
) -> kunit_attr_filter;

/*
 * Returns a copy of the suite containing only tests that pass the filter.
 */
pub unsafe extern "C" fn kunit_filter_attr_tests(
    suite: *const kunit_suite,
    filter: kunit_attr_filter,
    action: *mut core::ffi::c_char,
    err: *mut core::ffi::c_int,
) -> *mut kunit_suite;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
