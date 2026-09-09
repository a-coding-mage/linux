/* SPDX-License-Identifier: GPL-2.0 */
/*
 * rational fractions
 *
 * Copyright (C) 2009 emlix GmbH, Oskar Schirmer <oskar@scara.com>
 *
 * helper functions when coping with rational numbers,
 * e.g. when calculating optimum numerator/denominator pairs for
 * pll configuration taking into account restricted register size
 */

// C header guard: _LINUX_RATIONAL_H

use std::os::raw::c_ulong;

extern "C" {
    pub fn rational_best_approximation(
        given_numerator: c_ulong,
        given_denominator: c_ulong,
        max_numerator: c_ulong,
        max_denominator: c_ulong,
        best_numerator: *mut c_ulong,
        best_denominator: *mut c_ulong,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
