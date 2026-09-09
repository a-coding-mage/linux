/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 BAIKAL ELECTRONICS, JSC
 */

/*
 * struct polynomial_term - one term descriptor of a polynomial
 * @deg: degree of the term.
 * @coef: multiplication factor of the term.
 * @divider: distributed divider per each degree.
 * @divider_leftover: divider leftover, which couldn't be redistributed.
 */
#[repr(C)]
pub struct polynomial_term {
    pub deg: core::ffi::c_uint,
    pub coef: core::ffi::c_long,
    pub divider: core::ffi::c_long,
    pub divider_leftover: core::ffi::c_long,
}

/*
 * struct polynomial - a polynomial descriptor
 * @total_divider: total data divider.
 * @terms: polynomial terms, last term must have degree of 0
 */
#[repr(C)]
pub struct polynomial {
    pub total_divider: core::ffi::c_long,
    pub terms: [polynomial_term; 0],
}

unsafe extern "C" {
    pub fn polynomial_calc(poly: *const polynomial, data: core::ffi::c_long) -> core::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
