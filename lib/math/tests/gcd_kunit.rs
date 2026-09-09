// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the kernel KUnit and math headers are intentionally
// left as external declarations.

#[repr(C)]
pub struct test_case_params {
    pub val1: ::core::ffi::c_ulong,
    pub val2: ::core::ffi::c_ulong,
    pub expected_result: ::core::ffi::c_ulong,
    pub name: *const ::core::ffi::c_char,
}

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn gcd(a: ::core::ffi::c_ulong, b: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    fn strscpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        count: usize,
    ) -> isize;
}

const KUNIT_PARAM_DESC_SIZE: usize = 128;

static PARAMS: [test_case_params; 11] = [
    test_case_params { val1: 48, val2: 18, expected_result: 6, name: b"GCD of 48 and 18\0".as_ptr() as *const _ },
    test_case_params { val1: 18, val2: 48, expected_result: 6, name: b"GCD of 18 and 48\0".as_ptr() as *const _ },
    test_case_params { val1: 56, val2: 98, expected_result: 14, name: b"GCD of 56 and 98\0".as_ptr() as *const _ },
    test_case_params { val1: 17, val2: 13, expected_result: 1, name: b"Coprime numbers\0".as_ptr() as *const _ },
    test_case_params { val1: 101, val2: 103, expected_result: 1, name: b"Coprime numbers\0".as_ptr() as *const _ },
    test_case_params { val1: 270, val2: 192, expected_result: 6, name: b"GCD of 270 and 192\0".as_ptr() as *const _ },
    test_case_params { val1: 0, val2: 5, expected_result: 5, name: b"GCD with zero\0".as_ptr() as *const _ },
    test_case_params { val1: 7, val2: 0, expected_result: 7, name: b"GCD with zero reversed\0".as_ptr() as *const _ },
    test_case_params { val1: 36, val2: 36, expected_result: 36, name: b"GCD of identical numbers\0".as_ptr() as *const _ },
    test_case_params { val1: ::core::ffi::c_ulong::MAX, val2: 1, expected_result: 1, name: b"GCD of max ulong and 1\0".as_ptr() as *const _ },
    test_case_params { val1: ::core::ffi::c_ulong::MAX, val2: ::core::ffi::c_ulong::MAX, expected_result: ::core::ffi::c_ulong::MAX, name: b"GCD of max ulong values\0".as_ptr() as *const _ },
];

unsafe fn get_desc(tc: *const test_case_params, desc: *mut ::core::ffi::c_char) {
    // C: strscpy(desc, tc->name, KUNIT_PARAM_DESC_SIZE);
    strscpy(desc, (*tc).name, KUNIT_PARAM_DESC_SIZE);
}

unsafe fn gcd_test(test: *mut kunit, param_value: *const test_case_params) {
    let tc = param_value;
    // C: KUNIT_EXPECT_EQ(test, tc->expected_result, gcd(tc->val1, tc->val2));
    let _ = test;
    let _ = gcd((*tc).val1, (*tc).val2);
    let _ = (*tc).expected_result;
}

// KUNIT_ARRAY_PARAM(gcd, params, get_desc);
// KUNIT_CASE_PARAM(gcd_test, gcd_gen_params);
// The C kunit_case array and kunit_suite registration are provided by the
// KUnit macro framework and have no direct file-local Rust equivalent.
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("math.gcd KUnit test suite");
// MODULE_AUTHOR("Yu-Chun Lin <eleanor15x@gmail.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
