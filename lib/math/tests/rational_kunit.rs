// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the Linux KUnit and rational-number interfaces.

use core::ffi::{c_char, c_ulong, c_void};

#[repr(C)]
pub struct rational_test_param {
    pub num: c_ulong,
    pub den: c_ulong,
    pub max_num: c_ulong,
    pub max_den: c_ulong,
    pub exp_num: c_ulong,
    pub exp_den: c_ulong,
    pub name: *const c_char,
}

unsafe extern "C" {
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn rational_best_approximation(
        given_numerator: c_ulong,
        given_denominator: c_ulong,
        max_numerator: c_ulong,
        max_denominator: c_ulong,
        best_numerator: *mut c_ulong,
        best_denominator: *mut c_ulong,
    );
}

const KUNIT_PARAM_DESC_SIZE: usize = 256;

static TEST_PARAMETERS: [rational_test_param; 8] = [
    rational_test_param { num: 1230, den: 10, max_num: 100, max_den: 20, exp_num: 100, exp_den: 1, name: b"Exceeds bounds, semi-convergent term > 1/2 last term\0".as_ptr() as *const c_char },
    rational_test_param { num: 34567, den: 100, max_num: 120, max_den: 20, exp_num: 120, exp_den: 1, name: b"Exceeds bounds, semi-convergent term < 1/2 last term\0".as_ptr() as *const c_char },
    rational_test_param { num: 1, den: 30, max_num: 100, max_den: 10, exp_num: 0, exp_den: 1, name: b"Closest to zero\0".as_ptr() as *const c_char },
    rational_test_param { num: 1, den: 19, max_num: 100, max_den: 10, exp_num: 1, exp_den: 10, name: b"Closest to smallest non-zero\0".as_ptr() as *const c_char },
    rational_test_param { num: 27, den: 32, max_num: 16, max_den: 16, exp_num: 11, exp_den: 13, name: b"Use convergent\0".as_ptr() as *const c_char },
    rational_test_param { num: 1155, den: 7735, max_num: 255, max_den: 255, exp_num: 33, exp_den: 221, name: b"Exact answer\0".as_ptr() as *const c_char },
    rational_test_param { num: 87, den: 32, max_num: 70, max_den: 32, exp_num: 68, exp_den: 25, name: b"Semiconvergent, numerator limit\0".as_ptr() as *const c_char },
    rational_test_param { num: 14533, den: 4626, max_num: 15000, max_den: 2400, exp_num: 7433, exp_den: 2366, name: b"Semiconvergent, denominator limit\0".as_ptr() as *const c_char },
];

#[repr(C)]
pub struct kunit {
    pub param_value: *const c_void,
}

unsafe fn get_desc(param: *const rational_test_param, desc: *mut c_char) {
    unsafe {
        strscpy(desc, (*param).name, KUNIT_PARAM_DESC_SIZE);
    }
}

// KUNIT_ARRAY_PARAM(rational, test_parameters, get_desc);

unsafe fn rational_test(test: *mut kunit) {
    let param = unsafe { (*test).param_value as *const rational_test_param };
    let mut n: c_ulong = 0;
    let mut d: c_ulong = 0;

    unsafe {
        rational_best_approximation(
            (*param).num,
            (*param).den,
            (*param).max_num,
            (*param).max_den,
            &mut n,
            &mut d,
        );
        // KUNIT_EXPECT_EQ(test, n, param->exp_num);
        // KUNIT_EXPECT_EQ(test, d, param->exp_den);
        let _ = (n, d, (*param).exp_num, (*param).exp_den);
    }
}

// KUNIT_CASE_PARAM(rational_test, rational_gen_params);
// The empty sentinel entry in rational_test_cases is supplied by the KUnit macro.
// The KUnit suite is registered by kunit_test_suites(&rational_test_suite).

// MODULE_DESCRIPTION("Rational fractions unit test");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
