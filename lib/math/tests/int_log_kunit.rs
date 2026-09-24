// SPDX-License-Identifier: GPL-2.0-only
//! Original integer-log KUnit vectors exercised through the native provider ABI.

use kernel::{bindings, ffi, str::CStr};

struct TestParam {
    value: u32,
    expected_result: u32,
    name: &'static CStr,
}

/* The expected result takes into account the log error */
static INTLOG2_PARAMS: [TestParam; 9] = [
    TestParam {
        value: 0,
        expected_result: 0,
        name: c"Log base 2 of 0",
    },
    TestParam {
        value: 1,
        expected_result: 0,
        name: c"Log base 2 of 1",
    },
    TestParam {
        value: 2,
        expected_result: 16777216,
        name: c"Log base 2 of 2",
    },
    TestParam {
        value: 3,
        expected_result: 26591232,
        name: c"Log base 2 of 3",
    },
    TestParam {
        value: 4,
        expected_result: 33554432,
        name: c"Log base 2 of 4",
    },
    TestParam {
        value: 8,
        expected_result: 50331648,
        name: c"Log base 2 of 8",
    },
    TestParam {
        value: 16,
        expected_result: 67108864,
        name: c"Log base 2 of 16",
    },
    TestParam {
        value: 32,
        expected_result: 83886080,
        name: c"Log base 2 of 32",
    },
    TestParam {
        value: u32::MAX,
        expected_result: 536870911,
        name: c"Log base 2 of MAX",
    },
];

static INTLOG10_PARAMS: [TestParam; 8] = [
    TestParam {
        value: 0,
        expected_result: 0,
        name: c"Log base 10 of 0",
    },
    TestParam {
        value: 1,
        expected_result: 0,
        name: c"Log base 10 of 1",
    },
    TestParam {
        value: 6,
        expected_result: 13055203,
        name: c"Log base 10 of 6",
    },
    TestParam {
        value: 10,
        expected_result: 16777225,
        name: c"Log base 10 of 10",
    },
    TestParam {
        value: 100,
        expected_result: 33554450,
        name: c"Log base 10 of 100",
    },
    TestParam {
        value: 1000,
        expected_result: 50331675,
        name: c"Log base 10 of 1000",
    },
    TestParam {
        value: 10000,
        expected_result: 67108862,
        name: c"Log base 10 of 10000",
    },
    TestParam {
        value: u32::MAX,
        expected_result: 161614247,
        name: c"Log base 10 of MAX",
    },
];

fn get_desc(param: &TestParam) -> &CStr {
    param.name
}

unsafe extern "C" fn intlog2_gen_params(
    test: *mut bindings::kunit,
    prev: *const ffi::c_void,
    desc: *mut ffi::c_char,
) -> *const ffi::c_void {
    // SAFETY: KUnit supplies its live context, a previous parameter from this
    // array (or NULL initially), and a writable KUNIT_PARAM_DESC_SIZE buffer.
    unsafe { kernel::kunit::array_params(test, prev, desc, &INTLOG2_PARAMS, get_desc) }
}

unsafe extern "C" fn intlog10_gen_params(
    test: *mut bindings::kunit,
    prev: *const ffi::c_void,
    desc: *mut ffi::c_char,
) -> *const ffi::c_void {
    // SAFETY: KUnit supplies its live context, a previous parameter from this
    // array (or NULL initially), and a writable KUNIT_PARAM_DESC_SIZE buffer.
    unsafe { kernel::kunit::array_params(test, prev, desc, &INTLOG10_PARAMS, get_desc) }
}

unsafe extern "C" fn intlog2_test(test: *mut bindings::kunit) {
    // SAFETY: KUnit invokes this callback with its live context and a parameter
    // returned by intlog2_gen_params, hence a live immutable TestParam.
    let param = unsafe { &*(*test).param_value.cast::<TestParam>() };
    // SAFETY: The native function accepts u32 by value. The original zero case
    // deliberately calls the selected provider and retains its warning and
    // zero result; the pure checked API must not substitute for that behavior.
    let actual = unsafe { bindings::intlog2(param.value) };
    // SAFETY: This is the live callback context. Preserve the original expected
    // versus actual operand order and its nonfatal KUNIT_EXPECT_EQ semantics.
    unsafe { kernel::kunit_expect_eq!(test, param.expected_result, actual) };
}

unsafe extern "C" fn intlog10_test(test: *mut bindings::kunit) {
    // SAFETY: KUnit invokes this callback with its live context and a parameter
    // returned by intlog10_gen_params, hence a live immutable TestParam.
    let param = unsafe { &*(*test).param_value.cast::<TestParam>() };
    // SAFETY: As for intlog2, preserve the native zero call and its one warning.
    // In particular, do not evaluate intlog2(0) as a separate decimal-log step.
    let actual = unsafe { bindings::intlog10(param.value) };
    // SAFETY: This is the live callback context. Keep expected first and retain
    // the original nonfatal expectation, including for the zero parameter.
    unsafe { kernel::kunit_expect_eq!(test, param.expected_result, actual) };
}

static mut TEST_CASES: [bindings::kunit_case; 3] = [
    kernel::kunit::kunit_case_param(
        c"intlog2_test",
        c"int_log_kunit",
        intlog2_test,
        intlog2_gen_params,
    ),
    kernel::kunit::kunit_case_param(
        c"intlog10_test",
        c"int_log_kunit",
        intlog10_test,
        intlog10_gen_params,
    ),
    // SAFETY: All-zero is the original KUnit array's terminating empty case.
    unsafe { core::mem::zeroed() },
];

// SAFETY: TEST_CASES is a static, NULL-terminated array of valid cases.
kernel::kunit_unsafe_test_suite!("math-int_log", TEST_CASES);

#[cfg(MODULE)]
const MODINFO: &str = "description=math.int_log KUnit test suite\0license=GPL\0";
#[cfg(not(MODULE))]
const MODINFO: &str = concat!(
    "int_log_kunit.description=math.int_log KUnit test suite\0",
    "int_log_kunit.license=GPL\0",
    "int_log_kunit.file=",
    env!("RUST_MODFILE"),
    "\0",
);

#[used]
#[link_section = ".modinfo"]
static MODULE_INFO: [u8; MODINFO.len()] = {
    let mut bytes = [0; MODINFO.len()];
    let mut index = 0;
    while index < bytes.len() {
        bytes[index] = MODINFO.as_bytes()[index];
        index += 1;
    }
    bytes
};

#[cfg(MODULE)]
#[used]
static __IS_RUST_MODULE: () = ();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
