// SPDX-License-Identifier: GPL-2.0-only
//! Original greatest-common-divisor KUnit vectors exercised through the native ABI.

use kernel::{bindings, ffi, str::CStr};

struct TestParam {
    val1: ffi::c_ulong,
    val2: ffi::c_ulong,
    expected: ffi::c_ulong,
    name: &'static CStr,
}

static TEST_PARAMS: [TestParam; 11] = [
    TestParam {
        val1: 48,
        val2: 18,
        expected: 6,
        name: c"GCD of 48 and 18",
    },
    TestParam {
        val1: 18,
        val2: 48,
        expected: 6,
        name: c"GCD of 18 and 48",
    },
    TestParam {
        val1: 56,
        val2: 98,
        expected: 14,
        name: c"GCD of 56 and 98",
    },
    TestParam {
        val1: 17,
        val2: 13,
        expected: 1,
        name: c"Coprime numbers",
    },
    TestParam {
        val1: 101,
        val2: 103,
        expected: 1,
        name: c"Coprime numbers",
    },
    TestParam {
        val1: 270,
        val2: 192,
        expected: 6,
        name: c"GCD of 270 and 192",
    },
    TestParam {
        val1: 0,
        val2: 5,
        expected: 5,
        name: c"GCD with zero",
    },
    TestParam {
        val1: 7,
        val2: 0,
        expected: 7,
        name: c"GCD with zero reversed",
    },
    TestParam {
        val1: 36,
        val2: 36,
        expected: 36,
        name: c"GCD of identical numbers",
    },
    TestParam {
        val1: ffi::c_ulong::MAX,
        val2: 1,
        expected: 1,
        name: c"GCD of max ulong and 1",
    },
    TestParam {
        val1: ffi::c_ulong::MAX,
        val2: ffi::c_ulong::MAX,
        expected: ffi::c_ulong::MAX,
        name: c"GCD of max ulong values",
    },
];

fn get_desc(param: &TestParam) -> &CStr {
    param.name
}

unsafe extern "C" fn gcd_gen_params(
    test: *mut bindings::kunit,
    prev: *const ffi::c_void,
    desc: *mut ffi::c_char,
) -> *const ffi::c_void {
    // SAFETY: KUnit supplies its live context, a previous parameter from this
    // array (or NULL initially), and a writable KUNIT_PARAM_DESC_SIZE buffer.
    unsafe { kernel::kunit::array_params(test, prev, desc, &TEST_PARAMS, get_desc) }
}

unsafe extern "C" fn gcd_test(test: *mut bindings::kunit) {
    // SAFETY: KUnit invokes this callback with its live context and a parameter
    // returned by gcd_gen_params, hence a live immutable TestParam.
    let param = unsafe { &*(*test).param_value.cast::<TestParam>() };
    // SAFETY: The native gcd function accepts every unsigned-long pair.
    // Use the selected provider, including its runtime efficient-FFS key.
    let actual = unsafe { bindings::gcd(param.val1, param.val2) };
    // SAFETY: This is the live callback context. Preserve the original expected
    // versus actual order and its nonfatal KUNIT_EXPECT_EQ semantics.
    unsafe { kernel::kunit_expect_eq!(test, param.expected, actual) };
}

static mut TEST_CASES: [bindings::kunit_case; 2] = [
    kernel::kunit::kunit_case_param(c"gcd_test", c"gcd_kunit", gcd_test, gcd_gen_params),
    // SAFETY: All-zero is the original KUnit array's terminating empty case.
    unsafe { core::mem::zeroed() },
];

// SAFETY: TEST_CASES is a static, NULL-terminated array of valid cases.
kernel::kunit_unsafe_test_suite!("math-gcd", TEST_CASES);

#[cfg(MODULE)]
const MODINFO: &str = "license=GPL\0description=math.gcd KUnit test suite\0author=Yu-Chun Lin <eleanor15x@gmail.com>\0";
#[cfg(not(MODULE))]
const MODINFO: &str = concat!(
    "gcd_kunit.license=GPL\0",
    "gcd_kunit.description=math.gcd KUnit test suite\0",
    "gcd_kunit.author=Yu-Chun Lin <eleanor15x@gmail.com>\0",
    "gcd_kunit.file=",
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
