/* SPDX-License-Identifier: GPL-2.0 */

/*
 * These are the constant used to fake the fixed-point load-average
 * counting. Some notes:
 *  - 11 bit fractions expand to 22 bits by the multiplies: this gives
 *    a load-average precision of 10 bits integer + 11 bits fractional
 *  - if you want to count load-averages more often, you need more
 *    precision, or rounding will get you. With 2-second counting freq,
 *    the EXP_n values would be 1981, 2034 and 2043 if still using only
 *    11 bit fractions.
 */

use core::ffi::c_ulong;

extern "C" {
    pub static mut avenrun: [c_ulong; 0]; /* Load averages */
    pub fn get_avenrun(loads: *mut c_ulong, offset: c_ulong, shift: core::ffi::c_int);
}

pub const FSHIFT: u32 = 11; /* nr of bits of precision */
pub const FIXED_1: c_ulong = 1u64.wrapping_shl(FSHIFT) as c_ulong; /* 1.0 as fixed-point */
/* LOAD_FREQ = 5 * HZ + 1; HZ is supplied by another dependency. */
pub const LOAD_FREQ: c_ulong = (5 * HZ + 1) as c_ulong; /* 5 sec intervals */
pub const EXP_1: c_ulong = 1884; /* 1/exp(5sec/1min) as fixed-point */
pub const EXP_5: c_ulong = 2014; /* 1/exp(5sec/5min) */
pub const EXP_15: c_ulong = 2037; /* 1/exp(5sec/15min) */

/*
 * a1 = a0 * e + a * (1 - e)
 */
#[inline]
pub fn calc_load(load: c_ulong, exp: c_ulong, active: c_ulong) -> c_ulong {
    let mut newload: c_ulong;

    newload = load
        .wrapping_mul(exp)
        .wrapping_add(active.wrapping_mul(FIXED_1.wrapping_sub(exp)));
    if active >= load {
        newload = newload.wrapping_add(FIXED_1.wrapping_sub(1));
    }

    newload / FIXED_1
}

extern "C" {
    pub fn calc_load_n(
        load: c_ulong,
        exp: c_ulong,
        active: c_ulong,
        n: core::ffi::c_uint,
    ) -> c_ulong;
    pub fn calc_global_load();
}

#[inline]
pub const fn load_int(x: c_ulong) -> c_ulong {
    x >> FSHIFT
}

#[inline]
pub const fn load_frac(x: c_ulong) -> c_ulong {
    load_int((x & FIXED_1.wrapping_sub(1)).wrapping_mul(100))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
