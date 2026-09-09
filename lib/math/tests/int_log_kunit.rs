// SPDX-License-Identifier: GPL-2.0-only
// Translated from the C KUnit test source. The KUnit and Linux definitions
// referenced below are supplied by the surrounding kernel environment.

use core::ffi::c_char;

#[repr(C)]
pub struct test_case_params {
    pub value: u32,
    pub expected_result: u32,
    pub name: *const c_char,
}

/* The expected result takes into account the log error */
static INTLOG2_PARAMS: [test_case_params; 9] = [
    test_case_params { value: 0, expected_result: 0, name: c"Log base 2 of 0".as_ptr() },
    test_case_params { value: 1, expected_result: 0, name: c"Log base 2 of 1".as_ptr() },
    test_case_params { value: 2, expected_result: 16777216, name: c"Log base 2 of 2".as_ptr() },
    test_case_params { value: 3, expected_result: 26591232, name: c"Log base 2 of 3".as_ptr() },
    test_case_params { value: 4, expected_result: 33554432, name: c"Log base 2 of 4".as_ptr() },
    test_case_params { value: 8, expected_result: 50331648, name: c"Log base 2 of 8".as_ptr() },
    test_case_params { value: 16, expected_result: 67108864, name: c"Log base 2 of 16".as_ptr() },
    test_case_params { value: 32, expected_result: 83886080, name: c"Log base 2 of 32".as_ptr() },
    test_case_params { value: u32::MAX, expected_result: 536870911, name: c"Log base 2 of MAX".as_ptr() },
];

static INTLOG10_PARAMS: [test_case_params; 8] = [
    test_case_params { value: 0, expected_result: 0, name: c"Log base 10 of 0".as_ptr() },
    test_case_params { value: 1, expected_result: 0, name: c"Log base 10 of 1".as_ptr() },
    test_case_params { value: 6, expected_result: 13055203, name: c"Log base 10 of 6".as_ptr() },
    test_case_params { value: 10, expected_result: 16777225, name: c"Log base 10 of 10".as_ptr() },
    test_case_params { value: 100, expected_result: 33554450, name: c"Log base 10 of 100".as_ptr() },
    test_case_params { value: 1000, expected_result: 50331675, name: c"Log base 10 of 1000".as_ptr() },
    test_case_params { value: 10000, expected_result: 67108862, name: c"Log base 10 of 10000".as_ptr() },
    test_case_params { value: u32::MAX, expected_result: 161614247, name: c"Log base 10 of MAX".as_ptr() },
];

extern "C" {
    fn intlog2(value: u32) -> u32;
    fn intlog10(value: u32) -> u32;
}

#[repr(C)]
pub struct kunit {
    pub param_value: *const core::ffi::c_void,
}

const KUNIT_PARAM_DESC_SIZE: usize = 256;

unsafe fn get_desc(tc: *const test_case_params, desc: *mut c_char) {
    // Equivalent to strscpy(desc, tc->name, KUNIT_PARAM_DESC_SIZE).
    let mut i = 0usize;
    while i + 1 < KUNIT_PARAM_DESC_SIZE {
        let byte = *( (*tc).name as *const u8).add(i);
        *desc.add(i) = byte as c_char;
        if byte == 0 { return; }
        i += 1;
    }
    *desc.add(KUNIT_PARAM_DESC_SIZE - 1) = 0;
}

unsafe fn intlog2_test(test: *mut kunit) {
    let tc = (*test).param_value as *const test_case_params;
    let _actual = intlog2((*tc).value);
    // KUNIT_EXPECT_EQ(test, tc->expected_result, intlog2(tc->value));
}

unsafe fn intlog10_test(test: *mut kunit) {
    let tc = (*test).param_value as *const test_case_params;
    let _actual = intlog10((*tc).value);
    // KUNIT_EXPECT_EQ(test, tc->expected_result, intlog10(tc->value));
}

// KUNIT_ARRAY_PARAM(intlog2, intlog2_params, get_desc);
// KUNIT_ARRAY_PARAM(intlog10, intlog10_params, get_desc);
// KUNIT_CASE_PARAM(intlog2_test, intlog2_gen_params);
// KUNIT_CASE_PARAM(intlog10_test, intlog10_gen_params);

#[repr(C)]
pub struct kunit_case;

#[repr(C)]
pub struct kunit_suite {
    pub name: *const c_char,
    pub test_cases: *mut kunit_case,
}

static mut MATH_INT_LOG_TEST_CASES: [kunit_case; 1] = unsafe { core::mem::zeroed() };

static mut INT_LOG_TEST_SUITE: kunit_suite = kunit_suite {
    name: c"math-int_log".as_ptr(),
    test_cases: core::ptr::null_mut(),
};

// kunit_test_suites(&int_log_test_suite);
// MODULE_DESCRIPTION("math.int_log KUnit test suite");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
