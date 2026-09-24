// SPDX-License-Identifier: GPL-2.0
//! Original rational-approximation KUnit vectors exercised through the native ABI.

use kernel::{bindings, ffi, str::CStr};

struct TestParam {
    num: ffi::c_ulong,
    den: ffi::c_ulong,
    max_num: ffi::c_ulong,
    max_den: ffi::c_ulong,
    exp_num: ffi::c_ulong,
    exp_den: ffi::c_ulong,
    name: &'static CStr,
}

static TEST_PARAMS: [TestParam; 8] = [
    TestParam {
        num: 1230,
        den: 10,
        max_num: 100,
        max_den: 20,
        exp_num: 100,
        exp_den: 1,
        name: c"Exceeds bounds, semi-convergent term > 1/2 last term",
    },
    TestParam {
        num: 34567,
        den: 100,
        max_num: 120,
        max_den: 20,
        exp_num: 120,
        exp_den: 1,
        name: c"Exceeds bounds, semi-convergent term < 1/2 last term",
    },
    TestParam {
        num: 1,
        den: 30,
        max_num: 100,
        max_den: 10,
        exp_num: 0,
        exp_den: 1,
        name: c"Closest to zero",
    },
    TestParam {
        num: 1,
        den: 19,
        max_num: 100,
        max_den: 10,
        exp_num: 1,
        exp_den: 10,
        name: c"Closest to smallest non-zero",
    },
    TestParam {
        num: 27,
        den: 32,
        max_num: 16,
        max_den: 16,
        exp_num: 11,
        exp_den: 13,
        name: c"Use convergent",
    },
    TestParam {
        num: 1155,
        den: 7735,
        max_num: 255,
        max_den: 255,
        exp_num: 33,
        exp_den: 221,
        name: c"Exact answer",
    },
    TestParam {
        num: 87,
        den: 32,
        max_num: 70,
        max_den: 32,
        exp_num: 68,
        exp_den: 25,
        name: c"Semiconvergent, numerator limit",
    },
    TestParam {
        num: 14533,
        den: 4626,
        max_num: 15000,
        max_den: 2400,
        exp_num: 7433,
        exp_den: 2366,
        name: c"Semiconvergent, denominator limit",
    },
];

fn get_desc(param: &TestParam) -> &CStr {
    param.name
}

unsafe extern "C" fn rational_gen_params(
    test: *mut bindings::kunit,
    prev: *const ffi::c_void,
    desc: *mut ffi::c_char,
) -> *const ffi::c_void {
    // SAFETY: KUnit supplies its live context, a previous parameter from this
    // array (or NULL initially), and a writable KUNIT_PARAM_DESC_SIZE buffer.
    unsafe { kernel::kunit::array_params(test, prev, desc, &TEST_PARAMS, get_desc) }
}

unsafe extern "C" fn rational_test(test: *mut bindings::kunit) {
    // SAFETY: KUnit invokes this callback with its live context and a parameter
    // returned by rational_gen_params, hence a live immutable TestParam.
    let param = unsafe { &*(*test).param_value.cast::<TestParam>() };
    let mut n: ffi::c_ulong = 0;
    let mut d: ffi::c_ulong = 0;
    // SAFETY: Both output pointers refer to distinct live writable locals. The
    // native function accepts every unsigned-long input/bound combination.
    unsafe {
        bindings::rational_best_approximation(
            param.num,
            param.den,
            param.max_num,
            param.max_den,
            &mut n,
            &mut d,
        );
    }
    // SAFETY: This is the live callback context. Keep two original, nonfatal
    // expectations: a failed numerator must not suppress the denominator check.
    unsafe {
        kernel::kunit_expect_eq!(test, n, param.exp_num);
        kernel::kunit_expect_eq!(test, d, param.exp_den);
    }
}

static mut TEST_CASES: [bindings::kunit_case; 2] = [
    kernel::kunit::kunit_case_param(
        c"rational_test",
        c"rational_kunit",
        rational_test,
        rational_gen_params,
    ),
    // SAFETY: All-zero is the original KUnit array's terminating empty case.
    unsafe { core::mem::zeroed() },
];

// SAFETY: TEST_CASES is a static, NULL-terminated array of valid cases.
kernel::kunit_unsafe_test_suite!("rational", TEST_CASES);

#[cfg(MODULE)]
const MODINFO: &str = "description=Rational fractions unit test\0license=GPL v2\0";
#[cfg(not(MODULE))]
const MODINFO: &str = concat!(
    "rational_kunit.description=Rational fractions unit test\0",
    "rational_kunit.license=GPL v2\0",
    "rational_kunit.file=",
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
