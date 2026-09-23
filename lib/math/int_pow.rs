// SPDX-License-Identifier: GPL-2.0
/*
 * An integer based power function
 *
 * Derived from drivers/video/backlight/pwm_bl.c
 */
//! Allocation-free integer exponentiation with the kernel's wrapping rules.

/// Returns `base` raised to `exp`, reduced modulo 2⁶⁴.
///
/// Exponentiation by squaring takes at most 32 iterations. As in the original
/// C function, every multiplication wraps, and an exponent of zero returns
/// one, including when the base is zero.
#[inline]
pub const fn int_pow(mut base: u64, mut exp: u32) -> u64 {
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

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
