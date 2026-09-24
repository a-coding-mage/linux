// SPDX-License-Identifier: GPL-2.0-only
//! Original integer-power KUnit vectors exercised through the native provider ABI.

use kernel::{bindings, ffi, str::CStr};

struct TestParam {
    base: u64,
    exponent: u32,
    expected: u64,
    name: &'static CStr,
}

static TEST_PARAMS: [TestParam; 9] = [
    TestParam {
        base: 64,
        exponent: 0,
        expected: 1,
        name: c"Power of zero",
    },
    TestParam {
        base: 64,
        exponent: 1,
        expected: 64,
        name: c"Power of one",
    },
    TestParam {
        base: 0,
        exponent: 5,
        expected: 0,
        name: c"Base zero",
    },
    TestParam {
        base: 1,
        exponent: 64,
        expected: 1,
        name: c"Base one",
    },
    TestParam {
        base: 2,
        exponent: 2,
        expected: 4,
        name: c"Two squared",
    },
    TestParam {
        base: 2,
        exponent: 3,
        expected: 8,
        name: c"Two cubed",
    },
    TestParam {
        base: 5,
        exponent: 5,
        expected: 3125,
        name: c"Five raised to the fifth power",
    },
    TestParam {
        base: u64::MAX,
        exponent: 1,
        expected: u64::MAX,
        name: c"Max base",
    },
    TestParam {
        base: 2,
        exponent: 63,
        expected: 9223372036854775808,
        name: c"Large result",
    },
];

fn get_desc(param: &TestParam) -> &CStr {
    param.name
}

unsafe extern "C" fn int_pow_gen_params(
    test: *mut bindings::kunit,
    prev: *const ffi::c_void,
    desc: *mut ffi::c_char,
) -> *const ffi::c_void {
    // SAFETY: KUnit supplies its live context, a previous parameter from this
    // array (or NULL initially), and a writable KUNIT_PARAM_DESC_SIZE buffer.
    unsafe { kernel::kunit::array_params(test, prev, desc, &TEST_PARAMS, get_desc) }
}

unsafe extern "C" fn int_pow_test(test: *mut bindings::kunit) {
    // SAFETY: KUnit invokes this callback with its live context and a parameter
    // returned by int_pow_gen_params, hence a live immutable TestParam.
    let param = unsafe { &*(*test).param_value.cast::<TestParam>() };
    // SAFETY: The native integer-power function accepts every u64/u32 pair.
    // Calling the actual binding exercises whichever C/Rust provider is linked.
    let actual = unsafe { bindings::int_pow(param.base, param.exponent) };
    // SAFETY: This is the live callback context. Preserve the original expected
    // versus actual operand order and its nonfatal KUNIT_EXPECT_EQ semantics.
    unsafe { kernel::kunit_expect_eq!(test, param.expected, actual) };
}

static mut TEST_CASES: [bindings::kunit_case; 2] = [
    kernel::kunit::kunit_case_param(
        c"int_pow_test",
        c"int_pow_kunit",
        int_pow_test,
        int_pow_gen_params,
    ),
    // SAFETY: All-zero is the original KUnit array's terminating empty case.
    unsafe { core::mem::zeroed() },
];

// SAFETY: TEST_CASES is a static, NULL-terminated array of valid cases.
kernel::kunit_unsafe_test_suite!("math-int_pow", TEST_CASES);

#[cfg(MODULE)]
const MODINFO: &str = "description=math.int_pow KUnit test suite\0license=GPL\0";
#[cfg(not(MODULE))]
const MODINFO: &str = concat!(
    "int_pow_kunit.description=math.int_pow KUnit test suite\0",
    "int_pow_kunit.license=GPL\0",
    "int_pow_kunit.file=",
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
