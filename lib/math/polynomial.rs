// SPDX-License-Identifier: GPL-2.0-only
/*
 * Generic polynomial calculation using integer coefficients.
 *
 * Copyright (C) 2020 BAIKAL ELECTRONICS, JSC
 *
 * Authors:
 *   Maxim Kaurkin <maxim.kaurkin@baikalelectronics.ru>
 *   Serge Semin <Sergey.Semin@baikalelectronics.ru>
 */

use std::os::raw::c_long;

/* Types and functions declared by the corresponding kernel headers. */
#[repr(C)]
pub struct polynomial_term {
    pub deg: i32,
    pub coef: c_long,
    pub divider: c_long,
    pub divider_leftover: c_long,
}

#[repr(C)]
pub struct polynomial {
    pub total_divider: c_long,
    pub terms: *const polynomial_term,
}

extern "C" {
    fn mult_frac(x: c_long, multiplier: c_long, divisor: c_long) -> c_long;
}

/**
 * polynomial_calc - calculate a polynomial using integer arithmetic
 *
 * @poly: pointer to the descriptor of the polynomial
 * @data: input value of the polynomial
 *
 * Calculate the result of a polynomial using only integer arithmetic. For
 * this to work without too much loss of precision the coefficients has to
 * be altered. This is called factor redistribution.
 *
 * Return: the result of the polynomial calculation.
 */
pub unsafe fn polynomial_calc(poly: *const polynomial, data: c_long) -> c_long {
    let mut term = (*poly).terms;
    let total_divider = if (*poly).total_divider != 0 {
        (*poly).total_divider
    } else {
        1
    };
    let mut ret: c_long = 0;

    /*
     * Here is the polynomial calculation function, which performs the
     * redistributed terms calculations. It's pretty straightforward.
     * We walk over each degree term up to the free one, and perform
     * the redistributed multiplication of the term coefficient, its
     * divider (as for the rationale fraction representation), data
     * power and the rational fraction divider leftover. Then all of
     * this is collected in a total sum variable, which value is
     * normalized by the total divider before being returned.
     */
    loop {
        let current = &*term;
        let mut tmp = current.coef;
        let mut deg: i32 = 0;
        while deg < current.deg {
            tmp = mult_frac(tmp, data, current.divider);
            deg += 1;
        }
        ret += tmp / current.divider_leftover;

        let current_deg = current.deg;
        term = term.add(1);
        if current_deg == 0 {
            break;
        }
    }

    ret / total_divider
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
