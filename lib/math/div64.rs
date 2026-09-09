// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2003 Bernardo Innocenti <bernie@develer.com>
 *
 * Based on former do_div() implementation from asm-parisc/div64.h:
 * Copyright (C) 1999 Hewlett-Packard Co
 * Copyright (C) 1999 David Mosberger-Tang <davidm@hpl.hp.com>
 *
 * Generic C version of 64bit/32bit division and modulo, with
 * 64bit result and 32bit remainder.
 */

// Not needed on 64bit architectures: this section is intended for 32-bit builds.
#[cfg(target_pointer_width = "32")]
#[cfg(not(__div64_32))]
pub unsafe extern "C" fn __div64_32(n: *mut u64, base: u32) -> u32 {
    let mut rem = *n;
    let mut b = base as u64;
    let mut res: u64;
    let mut d: u64 = 1;
    let mut high = rem >> 32;

    /* Reduce the thing a bit first */
    res = 0;
    if high >= base as u64 {
        high /= base as u64;
        res = high << 32;
        rem -= (high * base as u64) << 32;
    }

    while (b as i64) > 0 && b < rem {
        b = b + b;
        d = d + d;
    }

    loop {
        if rem >= b {
            rem -= b;
            res += d;
        }
        b >>= 1;
        d >>= 1;
        if d == 0 {
            break;
        }
    }

    *n = res;
    rem as u32
}

#[cfg(target_pointer_width = "32")]
#[cfg(not(div_s64_rem))]
pub unsafe extern "C" fn div_s64_rem(dividend: i64, divisor: i32, remainder: *mut i32) -> i64 {
    let mut quotient: u64;
    if dividend < 0 {
        quotient = div_u64_rem(dividend.wrapping_neg() as u64, divisor.wrapping_abs() as u64, remainder as *mut u32);
        *remainder = (*remainder).wrapping_neg();
        if divisor > 0 { quotient = quotient.wrapping_neg(); }
    } else {
        quotient = div_u64_rem(dividend as u64, divisor.wrapping_abs() as u64, remainder as *mut u32);
        if divisor < 0 { quotient = quotient.wrapping_neg(); }
    }
    quotient as i64
}

#[cfg(target_pointer_width = "32")]
#[cfg(not(div64_u64_rem))]
pub unsafe extern "C" fn div64_u64_rem(dividend: u64, divisor: u64, remainder: *mut u64) -> u64 {
    let high = (divisor >> 32) as u32;
    let quot;
    if high == 0 {
        let mut rem32 = 0u32;
        quot = div_u64_rem(dividend, divisor, &mut rem32);
        *remainder = rem32 as u64;
    } else {
        let n = fls(high);
        quot = div_u64(dividend >> n, divisor >> n).wrapping_sub((div_u64(dividend >> n, divisor >> n) != 0) as u64);
        *remainder = dividend.wrapping_sub(quot.wrapping_mul(divisor));
        if *remainder >= divisor {
            *remainder = (*remainder).wrapping_sub(divisor);
            return quot + 1;
        }
    }
    quot
}

#[cfg(target_pointer_width = "32")]
#[cfg(not(div64_u64))]
pub unsafe extern "C" fn div64_u64(dividend: u64, divisor: u64) -> u64 {
    let high = (divisor >> 32) as u32;
    if high == 0 {
        div_u64(dividend, divisor)
    } else {
        let n = fls(high);
        let mut quot = div_u64(dividend >> n, divisor >> n);
        if quot != 0 { quot -= 1; }
        if dividend.wrapping_sub(quot.wrapping_mul(divisor)) >= divisor { quot += 1; }
        quot
    }
}

#[cfg(target_pointer_width = "32")]
#[cfg(not(div64_s64_rem))]
pub unsafe extern "C" fn div64_s64_rem(dividend: i64, divisor: i64, remainder: *mut i64) -> i64 {
    let mut rem = 0i64;
    let quot = div64_u64_rem(dividend.wrapping_abs() as u64, divisor.wrapping_abs() as u64, &mut rem as *mut i64 as *mut u64);
    let t = dividend >> 63;
    *remainder = (rem ^ t) - t;
    let t = (dividend ^ divisor) >> 63;
    (quot as i64 ^ t) - t
}

#[cfg(target_pointer_width = "32")]
#[cfg(not(div64_s64))]
pub unsafe extern "C" fn div64_s64(dividend: i64, divisor: i64) -> i64 {
    let quot = div64_u64(dividend.wrapping_abs() as u64, divisor.wrapping_abs() as u64);
    let t = (dividend ^ divisor) >> 63;
    (quot as i64 ^ t) - t
}

/* Iterative div/mod for use when dividend is not expected to be much bigger than divisor. */
#[cfg(not(iter_div_u64_rem))]
pub unsafe extern "C" fn iter_div_u64_rem(dividend: u64, divisor: u32, remainder: *mut u64) -> u32 {
    __iter_div_u64_rem(dividend, divisor, remainder)
}

#[cfg(not(mul_u64_add_u64_div_u64))]
#[cfg(not(test_mul_u64_add_u64_div_u64))]
pub unsafe extern "C" fn mul_u64_add_u64_div_u64(a: u64, b: u64, c: u64, d: u64) -> u64 {
    let (mut n_lo, mut n_hi) = ((a as u128).wrapping_mul(b as u128).wrapping_add(c as u128) as u64,
        ((a as u128).wrapping_mul(b as u128).wrapping_add(c as u128) >> 64) as u64);
    if n_hi == 0 { return div64_u64(n_lo, d); }
    if n_hi >= d { return if d == 0 { u64::MAX } else { u64::MAX }; }

    let d_z_hi = d.leading_zeros();
    if d_z_hi != 0 {
        d <<= d_z_hi;
        n_hi = n_hi << d_z_hi | n_lo >> (64 - d_z_hi);
        n_lo <<= d_z_hi;
    }

    let mut reps = 64 / 32;
    if (n_hi >> 32) as u32 == 0 {
        reps -= 1;
        n_hi = n_hi << 32 | n_lo >> 32;
        n_lo <<= 32;
    }

    n_lo = !n_lo;
    n_hi = !n_hi;
    let d_msig = (d >> (64 - 32)) + 1;
    let mut quotient = 0u64;
    while reps != 0 {
        reps -= 1;
        let mut q_digit = (!n_hi >> (64 - 64)) / d_msig;
        let overflow = n_hi >> 32;
        n_hi = n_hi << 32 | n_lo >> 32;
        n_lo <<= 32;
        let product = (d as u128) * q_digit as u128 + n_hi as u128;
        n_hi = product as u64;
        let mut overflow = overflow.wrapping_add((product >> 64) as u64);
        while overflow < (u32::MAX >> (32 - 32)) as u64 {
            q_digit += 1;
            let old = n_hi;
            n_hi = n_hi.wrapping_add(d);
            overflow = overflow.wrapping_add((n_hi < old) as u64);
        }
        quotient = quotient << 32 | q_digit;
    }
    if n_hi.wrapping_add(d) > n_hi { quotient += 1; }
    quotient
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
