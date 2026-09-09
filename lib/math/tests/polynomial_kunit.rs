// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel/KUnit environment.

#[repr(C)]
pub struct polynomial_term {
    pub degree: i64,
    pub coef: i64,
    pub divider: i64,
    pub divider_leftover: i64,
}

#[repr(C)]
pub struct polynomial {
    pub total_divider: i64,
    pub terms: [polynomial_term; 3],
}

#[repr(C)]
pub struct polynomial_test_param {
    pub poly: *const polynomial,
    pub data: i64,
    pub expected: i64,
    pub name: *const u8,
}

/* f(x) = 5 */
static POLY_CONSTANT: polynomial = polynomial {
    total_divider: 1,
    terms: [
        polynomial_term { degree: 0, coef: 5, divider: 1, divider_leftover: 1 },
        polynomial_term { degree: 0, coef: 0, divider: 1, divider_leftover: 1 },
        polynomial_term { degree: 0, coef: 0, divider: 1, divider_leftover: 1 },
    ],
};

/* f(x) = 2x^2 + 3x + 5 */
static POLY_SIMPLE: polynomial = polynomial {
    total_divider: 1,
    terms: [
        polynomial_term { degree: 2, coef: 2, divider: 1, divider_leftover: 1 },
        polynomial_term { degree: 1, coef: 3, divider: 1, divider_leftover: 1 },
        polynomial_term { degree: 0, coef: 5, divider: 1, divider_leftover: 1 },
    ],
};

/* f(x) = -5x + 100 */
static POLY_NEGATIVE_COEF: polynomial = polynomial {
    total_divider: 1,
    terms: [
        polynomial_term { degree: 1, coef: -5, divider: 1, divider_leftover: 1 },
        polynomial_term { degree: 0, coef: 100, divider: 1, divider_leftover: 1 },
        polynomial_term { degree: 0, coef: 0, divider: 1, divider_leftover: 1 },
    ],
};

/* f(x) = (150x + 50) / 10 */
static POLY_TOTAL_DIVIDER: polynomial = polynomial {
    total_divider: 10,
    terms: [
        polynomial_term { degree: 1, coef: 150, divider: 1, divider_leftover: 1 },
        polynomial_term { degree: 0, coef: 50, divider: 1, divider_leftover: 1 },
        polynomial_term { degree: 0, coef: 0, divider: 1, divider_leftover: 1 },
    ],
};

/* f(x) = x / 2
 * divider=2 applied once per multiply: mult_frac(coef, data, 2) = coef*data/2
 */
static POLY_STEP_DIVIDER: polynomial = polynomial {
    total_divider: 1,
    terms: [
        polynomial_term { degree: 1, coef: 1, divider: 2, divider_leftover: 1 },
        polynomial_term { degree: 0, coef: 0, divider: 1, divider_leftover: 1 },
        polynomial_term { degree: 0, coef: 0, divider: 1, divider_leftover: 1 },
    ],
};

/* f(x) = (100/500) * x^2 = 0.2 * x^2
 * Encoded as coef=100, divider=10, divider_leftover=5:
 *   denom = 10^2 * 5 = 500
 */
static POLY_LEFTOVER: polynomial = polynomial {
    total_divider: 1,
    terms: [
        polynomial_term { degree: 2, coef: 100, divider: 10, divider_leftover: 5 },
        polynomial_term { degree: 0, coef: 0, divider: 1, divider_leftover: 1 },
        polynomial_term { degree: 0, coef: 0, divider: 1, divider_leftover: 1 },
    ],
};

/* f(x) = 2x^3  (single high-degree term, no constant)
 * Used to exercise the power loop alone.
 */
static POLY_CUBIC: polynomial = polynomial {
    total_divider: 1,
    terms: [
        polynomial_term { degree: 3, coef: 2, divider: 1, divider_leftover: 1 },
        polynomial_term { degree: 0, coef: 0, divider: 1, divider_leftover: 1 },
        polynomial_term { degree: 0, coef: 0, divider: 1, divider_leftover: 1 },
    ],
};

/* f(x) = 4x + 1  with a zero-coefficient quadratic term.
 * The deg-2 term contributes nothing regardless of input.
 */
static POLY_ZERO_COEF: polynomial = polynomial {
    total_divider: 1,
    terms: [
        polynomial_term { degree: 2, coef: 0, divider: 1, divider_leftover: 1 },
        polynomial_term { degree: 1, coef: 4, divider: 1, divider_leftover: 1 },
        polynomial_term { degree: 0, coef: 1, divider: 1, divider_leftover: 1 },
    ],
};

/* f(x) = 9  with total_divider = 0.
 * The implementation treats 0 as 1 via `total_divider ?: 1`, so the
 * result must equal the constant term unchanged.
 */
static POLY_ZERO_TOTAL_DIVIDER: polynomial = polynomial {
    total_divider: 0,
    terms: [
        polynomial_term { degree: 0, coef: 9, divider: 1, divider_leftover: 1 },
        polynomial_term { degree: 0, coef: 0, divider: 1, divider_leftover: 1 },
        polynomial_term { degree: 0, coef: 0, divider: 1, divider_leftover: 1 },
    ],
};

static TEST_PARAMS: &[polynomial_test_param] = &[
    polynomial_test_param { poly: &POLY_CONSTANT, data: 0, expected: 5, name: b"Constant polynomial at x=0\0".as_ptr() },
    polynomial_test_param { poly: &POLY_CONSTANT, data: 42, expected: 5, name: b"Constant polynomial is independent of input\0".as_ptr() },
    polynomial_test_param { poly: &POLY_SIMPLE, data: 0, expected: 5, name: b"Zero input yields constant term only\0".as_ptr() },
    polynomial_test_param { poly: &POLY_SIMPLE, data: 10, expected: 235, name: b"Simple quadratic at x=10\0".as_ptr() },
    polynomial_test_param { poly: &POLY_NEGATIVE_COEF, data: 10, expected: 50, name: b"Negative coefficient at x=10\0".as_ptr() },
    polynomial_test_param { poly: &POLY_NEGATIVE_COEF, data: 20, expected: 0, name: b"Negative coefficient result is zero\0".as_ptr() },
    polynomial_test_param { poly: &POLY_TOTAL_DIVIDER, data: 3, expected: 50, name: b"total_divider scales the final sum\0".as_ptr() },
    polynomial_test_param { poly: &POLY_STEP_DIVIDER, data: 100, expected: 50, name: b"Per-step divider halves input\0".as_ptr() },
    polynomial_test_param { poly: &POLY_LEFTOVER, data: 30, expected: 180, name: b"divider_leftover with quadratic term\0".as_ptr() },
    polynomial_test_param { poly: &POLY_SIMPLE, data: 1, expected: 10, name: b"Boundary: data=1 (unit input)\0".as_ptr() },
    polynomial_test_param { poly: &POLY_SIMPLE, data: -1, expected: 4, name: b"Boundary: data=-1 (negative unit input)\0".as_ptr() },
    polynomial_test_param { poly: &POLY_SIMPLE, data: -3, expected: 14, name: b"Boundary: negative data with quadratic\0".as_ptr() },
    polynomial_test_param { poly: &POLY_ZERO_TOTAL_DIVIDER, data: 42, expected: 9, name: b"Boundary: total_divider=0 defaults to 1\0".as_ptr() },
    polynomial_test_param { poly: &POLY_ZERO_COEF, data: 10, expected: 41, name: b"Boundary: zero-coefficient term is inert\0".as_ptr() },
    polynomial_test_param { poly: &POLY_CUBIC, data: 5, expected: 250, name: b"Boundary: single cubic term\0".as_ptr() },
    polynomial_test_param { poly: &POLY_CUBIC, data: -2, expected: -16, name: b"Boundary: single cubic term, negative data\0".as_ptr() },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
