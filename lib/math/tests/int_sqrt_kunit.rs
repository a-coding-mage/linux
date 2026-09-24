// SPDX-License-Identifier: GPL-2.0-only
//! Original integer-root KUnit vectors exercised through the native provider ABI.

use kernel::{bindings, ffi, str::CStr};

struct TestParam {
    x: ffi::c_ulong,
    expected: ffi::c_ulong,
    name: &'static CStr,
}

static TEST_PARAMS: [TestParam; 21] = [
    TestParam {
        x: 0,
        expected: 0,
        name: c"edge case: square root of 0",
    },
    TestParam {
        x: 1,
        expected: 1,
        name: c"perfect square: square root of 1",
    },
    TestParam {
        x: 2,
        expected: 1,
        name: c"non-perfect square: square root of 2",
    },
    TestParam {
        x: 3,
        expected: 1,
        name: c"non-perfect square: square root of 3",
    },
    TestParam {
        x: 4,
        expected: 2,
        name: c"perfect square: square root of 4",
    },
    TestParam {
        x: 5,
        expected: 2,
        name: c"non-perfect square: square root of 5",
    },
    TestParam {
        x: 6,
        expected: 2,
        name: c"non-perfect square: square root of 6",
    },
    TestParam {
        x: 7,
        expected: 2,
        name: c"non-perfect square: square root of 7",
    },
    TestParam {
        x: 8,
        expected: 2,
        name: c"non-perfect square: square root of 8",
    },
    TestParam {
        x: 9,
        expected: 3,
        name: c"perfect square: square root of 9",
    },
    TestParam {
        x: 15,
        expected: 3,
        name: c"non-perfect square: square root of 15 (N-1 from 16)",
    },
    TestParam {
        x: 16,
        expected: 4,
        name: c"perfect square: square root of 16",
    },
    TestParam {
        x: 17,
        expected: 4,
        name: c"non-perfect square: square root of 17 (N+1 from 16)",
    },
    TestParam {
        x: 80,
        expected: 8,
        name: c"non-perfect square: square root of 80 (N-1 from 81)",
    },
    TestParam {
        x: 81,
        expected: 9,
        name: c"perfect square: square root of 81",
    },
    TestParam {
        x: 82,
        expected: 9,
        name: c"non-perfect square: square root of 82 (N+1 from 81)",
    },
    TestParam {
        x: 255,
        expected: 15,
        name: c"non-perfect square: square root of 255 (N-1 from 256)",
    },
    TestParam {
        x: 256,
        expected: 16,
        name: c"perfect square: square root of 256",
    },
    TestParam {
        x: 257,
        expected: 16,
        name: c"non-perfect square: square root of 257 (N+1 from 256)",
    },
    TestParam {
        x: 2147483648,
        expected: 46340,
        name: c"large input: square root of 2147483648",
    },
    TestParam {
        x: 4294967295,
        expected: 65535,
        name: c"edge case: ULONG_MAX for 32-bit",
    },
];

fn get_desc(param: &TestParam) -> &CStr {
    param.name
}

unsafe extern "C" fn int_sqrt_gen_params(
    test: *mut bindings::kunit,
    prev: *const ffi::c_void,
    desc: *mut ffi::c_char,
) -> *const ffi::c_void {
    // SAFETY: KUnit supplies its live context, a previous parameter from this
    // array (or NULL initially), and a writable KUNIT_PARAM_DESC_SIZE buffer.
    unsafe { kernel::kunit::array_params(test, prev, desc, &TEST_PARAMS, get_desc) }
}

unsafe extern "C" fn int_sqrt_test(test: *mut bindings::kunit) {
    // SAFETY: KUnit invokes this callback with its live context and a parameter
    // returned by int_sqrt_gen_params, hence a live immutable TestParam.
    let param = unsafe { &*(*test).param_value.cast::<TestParam>() };
    // SAFETY: The native integer-root function accepts every unsigned long.
    // This intentionally tests int_sqrt, not the distinct int_sqrt64 helper.
    let actual = unsafe { bindings::int_sqrt(param.x) };
    // SAFETY: This is the live callback context. Preserve the original expected
    // versus actual operand order and its nonfatal KUNIT_EXPECT_EQ semantics.
    unsafe { kernel::kunit_expect_eq!(test, param.expected, actual) };
}

static mut TEST_CASES: [bindings::kunit_case; 2] = [
    kernel::kunit::kunit_case_param(
        c"int_sqrt_test",
        c"int_sqrt_kunit",
        int_sqrt_test,
        int_sqrt_gen_params,
    ),
    // SAFETY: All-zero is the original KUnit array's terminating empty case.
    unsafe { core::mem::zeroed() },
];

// SAFETY: TEST_CASES is a static, NULL-terminated array of valid cases.
kernel::kunit_unsafe_test_suite!("math-int_sqrt", TEST_CASES);

#[cfg(MODULE)]
const MODINFO: &str = "description=math.int_sqrt KUnit test suite\0license=GPL\0";
#[cfg(not(MODULE))]
const MODINFO: &str = concat!(
    "int_sqrt_kunit.description=math.int_sqrt KUnit test suite\0",
    "int_sqrt_kunit.license=GPL\0",
    "int_sqrt_kunit.file=",
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
