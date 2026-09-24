// SPDX-License-Identifier: GPL-2.0-only
//! Original polynomial KUnit vectors exercised through the native provider ABI.

use kernel::{bindings, ffi, str::CStr};

/// Original header followed by inline flexible-array storage. Every fixture
/// terminates within the array; entries after its constant term are not read.
#[repr(C)]
struct PolynomialStorage {
    header: bindings::polynomial,
    terms: [bindings::polynomial_term; 3],
}

const _: () = {
    assert!(core::mem::offset_of!(PolynomialStorage, header) == 0);
    assert!(
        core::mem::offset_of!(PolynomialStorage, terms)
            == core::mem::offset_of!(bindings::polynomial, terms)
    );
    assert!(
        core::mem::align_of::<PolynomialStorage>() >= core::mem::align_of::<bindings::polynomial>()
    );
};

const fn term(
    deg: u32,
    coef: ffi::c_long,
    divider: ffi::c_long,
    divider_leftover: ffi::c_long,
) -> bindings::polynomial_term {
    bindings::polynomial_term {
        deg,
        coef,
        divider,
        divider_leftover,
    }
}

const fn polynomial(
    total_divider: ffi::c_long,
    terms: [bindings::polynomial_term; 3],
) -> PolynomialStorage {
    PolynomialStorage {
        header: bindings::polynomial {
            total_divider,
            terms: bindings::__IncompleteArrayField::new(),
        },
        terms,
    }
}

// f(x) = 5.
static POLY_CONSTANT: PolynomialStorage =
    polynomial(1, [term(0, 5, 1, 1), term(0, 0, 1, 1), term(0, 0, 1, 1)]);

// f(x) = 2x^2 + 3x + 5.
static POLY_SIMPLE: PolynomialStorage =
    polynomial(1, [term(2, 2, 1, 1), term(1, 3, 1, 1), term(0, 5, 1, 1)]);

// f(x) = -5x + 100.
static POLY_NEGATIVE_COEF: PolynomialStorage =
    polynomial(1, [term(1, -5, 1, 1), term(0, 100, 1, 1), term(0, 0, 1, 1)]);

// f(x) = (150x + 50) / 10.
static POLY_TOTAL_DIVIDER: PolynomialStorage = polynomial(
    10,
    [term(1, 150, 1, 1), term(0, 50, 1, 1), term(0, 0, 1, 1)],
);

// f(x) = x / 2: divider=2 is applied once per multiplication.
static POLY_STEP_DIVIDER: PolynomialStorage =
    polynomial(1, [term(1, 1, 2, 1), term(0, 0, 1, 1), term(0, 0, 1, 1)]);

// f(x) = (100/500) * x^2: denom = 10^2 * 5.
static POLY_LEFTOVER: PolynomialStorage =
    polynomial(1, [term(2, 100, 10, 5), term(0, 0, 1, 1), term(0, 0, 1, 1)]);

// f(x) = 2x^3: a single high-degree term followed by the zero constant.
static POLY_CUBIC: PolynomialStorage =
    polynomial(1, [term(3, 2, 1, 1), term(0, 0, 1, 1), term(0, 0, 1, 1)]);

// f(x) = 4x + 1, with an inert zero-coefficient quadratic term.
static POLY_ZERO_COEF: PolynomialStorage =
    polynomial(1, [term(2, 0, 1, 1), term(1, 4, 1, 1), term(0, 1, 1, 1)]);

// f(x) = 9: the zero total divider defaults to one.
static POLY_ZERO_TOTAL_DIVIDER: PolynomialStorage =
    polynomial(0, [term(0, 9, 1, 1), term(0, 0, 1, 1), term(0, 0, 1, 1)]);

struct TestParam {
    poly: &'static PolynomialStorage,
    data: ffi::c_long,
    expected: ffi::c_long,
    name: &'static CStr,
}

static TEST_PARAMS: [TestParam; 16] = [
    TestParam {
        poly: &POLY_CONSTANT,
        data: 0,
        expected: 5,
        name: c"Constant polynomial at x=0",
    },
    TestParam {
        poly: &POLY_CONSTANT,
        data: 42,
        expected: 5,
        name: c"Constant polynomial is independent of input",
    },
    TestParam {
        poly: &POLY_SIMPLE,
        data: 0,
        expected: 5,
        name: c"Zero input yields constant term only",
    },
    TestParam {
        poly: &POLY_SIMPLE,
        data: 10,
        expected: 235,
        name: c"Simple quadratic at x=10",
    },
    TestParam {
        poly: &POLY_NEGATIVE_COEF,
        data: 10,
        expected: 50,
        name: c"Negative coefficient at x=10",
    },
    TestParam {
        poly: &POLY_NEGATIVE_COEF,
        data: 20,
        expected: 0,
        name: c"Negative coefficient result is zero",
    },
    TestParam {
        poly: &POLY_TOTAL_DIVIDER,
        data: 3,
        expected: 50,
        name: c"total_divider scales the final sum",
    },
    TestParam {
        poly: &POLY_STEP_DIVIDER,
        data: 100,
        expected: 50,
        name: c"Per-step divider halves input",
    },
    TestParam {
        poly: &POLY_LEFTOVER,
        data: 30,
        expected: 180,
        name: c"divider_leftover with quadratic term",
    },
    TestParam {
        poly: &POLY_SIMPLE,
        data: 1,
        expected: 10,
        name: c"Boundary: data=1 (unit input)",
    },
    TestParam {
        poly: &POLY_SIMPLE,
        data: -1,
        expected: 4,
        name: c"Boundary: data=-1 (negative unit input)",
    },
    TestParam {
        poly: &POLY_SIMPLE,
        data: -3,
        expected: 14,
        name: c"Boundary: negative data with quadratic",
    },
    TestParam {
        poly: &POLY_ZERO_TOTAL_DIVIDER,
        data: 42,
        expected: 9,
        name: c"Boundary: total_divider=0 defaults to 1",
    },
    TestParam {
        poly: &POLY_ZERO_COEF,
        data: 10,
        expected: 41,
        name: c"Boundary: zero-coefficient term is inert",
    },
    TestParam {
        poly: &POLY_CUBIC,
        data: 5,
        expected: 250,
        name: c"Boundary: single cubic term",
    },
    TestParam {
        poly: &POLY_CUBIC,
        data: -2,
        expected: -16,
        name: c"Boundary: single cubic term, negative data",
    },
];

fn get_desc(param: &TestParam) -> &CStr {
    param.name
}

unsafe extern "C" fn polynomial_gen_params(
    test: *mut bindings::kunit,
    prev: *const ffi::c_void,
    desc: *mut ffi::c_char,
) -> *const ffi::c_void {
    // SAFETY: KUnit supplies its live test, previous parameter from this array
    // (or NULL initially), and a writable KUNIT_PARAM_DESC_SIZE-byte buffer.
    unsafe { kernel::kunit::array_params(test, prev, desc, &TEST_PARAMS, get_desc) }
}

unsafe extern "C" fn polynomial_calc_test(test: *mut bindings::kunit) {
    // SAFETY: KUnit invokes this callback with the live context and a parameter
    // returned by polynomial_gen_params, hence a live immutable TestParam.
    let param = unsafe { &*(*test).param_value.cast::<TestParam>() };
    // Derive the FAM pointer from the entire allocation, not a narrowed header
    // reference. The layout assertions place its real inline terms at the C
    // offset; every fixture terminates within this storage.
    let poly = core::ptr::from_ref(param.poly).cast::<bindings::polynomial>();
    // SAFETY: Each static fixture is aligned and immutable, has initialized
    // evaluated fields and a constant terminator, and all divisions are defined.
    let actual = unsafe { bindings::polynomial_calc(poly, param.data) };
    // SAFETY: This is the live KUnit callback context. An expectation failure
    // is nonfatal, preserving the original KUNIT_EXPECT_EQ behavior.
    unsafe { kernel::kunit_expect_eq!(test, actual, param.expected) };
}

static mut TEST_CASES: [bindings::kunit_case; 2] = [
    kernel::kunit::kunit_case_param(
        c"polynomial_calc_test",
        c"polynomial_kunit",
        polynomial_calc_test,
        polynomial_gen_params,
    ),
    // SAFETY: All-zero is the original KUnit array's terminating empty case.
    unsafe { core::mem::zeroed() },
];

// SAFETY: TEST_CASES is a static, NULL-terminated array of valid cases.
kernel::kunit_unsafe_test_suite!("math-polynomial", TEST_CASES);

#[cfg(MODULE)]
const MODINFO: &str = "description=math.polynomial_calc KUnit test suite\0license=GPL\0";
#[cfg(not(MODULE))]
const MODINFO: &str = concat!(
    "polynomial_kunit.description=math.polynomial_calc KUnit test suite\0",
    "polynomial_kunit.license=GPL\0",
    "polynomial_kunit.file=",
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
