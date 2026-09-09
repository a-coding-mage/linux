// SPDX-License-Identifier: GPL-2.0
/*
 * An integer based power function
 *
 * Derived from drivers/video/backlight/pwm_bl.c
 */

/**
 * int_pow - computes the exponentiation of the given base and exponent
 * @base: base which will be raised to the given power
 * @exp: power to be raised to
 *
 * Computes: pow(base, exp), i.e. @base raised to the @exp power
 */
pub fn int_pow(mut base: u64, mut exp: u32) -> u64 {
    let mut result: u64 = 1;

    while exp != 0 {
        if exp & 1 != 0 {
            result = result.wrapping_mul(base);
        }
        exp >>= 1;
        base = base.wrapping_mul(base);
    }

    result
}

// EXPORT_SYMBOL_GPL(int_pow);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
