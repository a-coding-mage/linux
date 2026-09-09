// SPDX-License-Identifier: GPL-2.0
// Dependency intent: declarations from <asm/misc.h> are supplied elsewhere.

/*
 * Count the digits of @val including a possible sign.
 *
 * (Typed on and submitted from hpa's mobile phone.)
 */
pub fn num_digits(mut val: i32) -> i32 {
    let mut m: i64 = 10;
    let mut d: i32 = 1;

    if val < 0 {
        d += 1;
        val = val.wrapping_neg();
    }

    while (val as i64) >= m {
        m = m.wrapping_mul(10);
        d += 1;
    }
    d
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
