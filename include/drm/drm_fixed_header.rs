/*
 * Copyright 2009 Red Hat Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: Dave Airlie
 *          Christian König
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub union fixed20_12 {
    pub full: u32,
}

#[macro_export]
macro_rules! dfixed_const { ($a:expr) => { (($a) << 12) as u32 }; }
#[macro_export]
macro_rules! dfixed_const_half { ($a:expr) => { ((($a) << 12) + 2048) as u32 }; }
#[macro_export]
macro_rules! dfixed_const_666 { ($a:expr) => { ((($a) << 12) + 2731) as u32 }; }
#[macro_export]
macro_rules! dfixed_const_8 { ($a:expr) => { ((($a) << 12) + 3277) as u32 }; }
#[macro_export]
macro_rules! dfixed_mul { ($a:expr, $b:expr) => { (((($a).full as u64) * (($b).full as u64) + 2048) >> 12) }; }
#[macro_export]
macro_rules! dfixed_init { ($a:expr) => { fixed20_12 { full: dfixed_const!($a) } }; }
#[macro_export]
macro_rules! dfixed_init_half { ($a:expr) => { fixed20_12 { full: dfixed_const_half!($a) } }; }
#[macro_export]
macro_rules! dfixed_trunc { ($a:expr) => { ($a).full >> 12 }; }
#[macro_export]
macro_rules! dfixed_frac { ($a:expr) => { ($a).full & ((1u32 << 12) - 1) }; }

#[inline]
pub fn dfixed_floor(a: fixed20_12) -> u32 {
    let non_frac = unsafe { a.full >> 12 };
    dfixed_const!(non_frac)
}

#[inline]
pub fn dfixed_ceil(a: fixed20_12) -> u32 {
    let full = unsafe { a.full };
    let non_frac = full >> 12;
    if full > dfixed_const!(non_frac) { dfixed_const!(non_frac + 1) } else { dfixed_const!(non_frac) }
}

#[inline]
pub fn dfixed_div(a: fixed20_12, b: fixed20_12) -> u32 {
    let mut tmp = (unsafe { a.full } as u64) << 13;
    tmp /= unsafe { b.full } as u64;
    tmp += 1;
    tmp /= 2;
    tmp as u32
}

pub const DRM_FIXED_POINT: u32 = 32;
pub const DRM_FIXED_ONE: u64 = 1u64 << DRM_FIXED_POINT;
pub const DRM_FIXED_DECIMAL_MASK: u64 = DRM_FIXED_ONE - 1;
pub const DRM_FIXED_DIGITS_MASK: u64 = !DRM_FIXED_DECIMAL_MASK;
pub const DRM_FIXED_EPSILON: i64 = 1;
pub const DRM_FIXED_ALMOST_ONE: u64 = DRM_FIXED_ONE - DRM_FIXED_EPSILON as u64;

#[inline]
pub fn drm_sm2fixp(a: u64) -> i64 {
    if (a & (1u64 << 63)) != 0 { -(a & 0x7fffffffffffffff) as i64 } else { a as i64 }
}

#[inline]
pub fn drm_int2fixp(a: i32) -> i64 { (a as i64) << DRM_FIXED_POINT }
#[inline]
pub fn drm_fixp2int(a: i64) -> i32 { (a >> DRM_FIXED_POINT) as i32 }
#[inline]
pub fn drm_fixp2int_round(a: i64) -> i32 { drm_fixp2int(a + (DRM_FIXED_ONE / 2) as i64) }
#[inline]
pub fn drm_fixp2int_ceil(a: i64) -> i32 {
    if a >= 0 { drm_fixp2int(a + DRM_FIXED_ALMOST_ONE as i64) } else { drm_fixp2int(a - DRM_FIXED_ALMOST_ONE as i64) }
}

#[inline]
pub fn drm_fixp_msbset(a: i64) -> u32 {
    let sign = ((a >> 63) & 1) as i64;
    let mut shift = 62u32;
    while shift > 0 {
        if ((a >> shift) & 1) != sign { return shift; }
        shift -= 1;
    }
    0
}

#[inline]
pub fn drm_fixp_mul(mut a: i64, mut b: i64) -> i64 {
    let mut shift = drm_fixp_msbset(a) + drm_fixp_msbset(b);
    if shift > 61 {
        shift -= 61;
        a >>= (shift >> 1) + (shift & 1);
        b >>= shift >> 1;
    } else { shift = 0; }
    let result = a * b;
    if shift > DRM_FIXED_POINT { result << (shift - DRM_FIXED_POINT) }
    else if shift < DRM_FIXED_POINT { result >> (DRM_FIXED_POINT - shift) }
    else { result }
}

#[inline]
pub fn drm_fixp_div(mut a: i64, mut b: i64) -> i64 {
    let shift = 62 - drm_fixp_msbset(a);
    a <<= shift;
    if shift < DRM_FIXED_POINT { b >>= DRM_FIXED_POINT - shift; }
    let result = a / b;
    if shift > DRM_FIXED_POINT { result >> (shift - DRM_FIXED_POINT) } else { result }
}

#[inline]
pub fn drm_fixp_from_fraction(a: i64, b: i64) -> i64 {
    let a_neg = a < 0;
    let b_neg = b < 0;
    let a_abs = if a_neg { (-a) as u64 } else { a as u64 };
    let b_abs = if b_neg { (-b) as u64 } else { b as u64 };
    let mut rem;
    let mut res_abs = a_abs / b_abs;
    rem = a_abs % b_abs;
    let mut i = DRM_FIXED_POINT;
    loop {
        rem <<= 1;
        res_abs <<= 1;
        if rem >= b_abs { res_abs |= 1; rem -= b_abs; }
        i -= 1;
        if i == 0 { break; }
    }
    if (rem << 1) >= b_abs { res_abs += 1; }
    let mut res = res_abs as i64;
    if a_neg ^ b_neg { res = -res; }
    res
}

#[inline]
pub fn drm_fixp_exp(x: i64) -> i64 {
    let tolerance = DRM_FIXED_ONE as i64 / 1000000;
    let mut sum = DRM_FIXED_ONE as i64;
    let mut y = x;
    let mut count = 1i64;
    if x < 0 { y = -x; }
    let mut term = y;
    while term >= tolerance {
        sum += term;
        count += 1;
        term = drm_fixp_mul(term, drm_fixp_div(y, count));
    }
    if x < 0 { sum = drm_fixp_div(DRM_FIXED_ONE as i64, sum); }
    sum
}

#[inline]
pub fn fxp_q4_from_int(val_int: i32) -> i32 { val_int << 4 }
#[inline]
pub fn fxp_q4_to_int(val_q4: i32) -> i32 { val_q4 >> 4 }
#[inline]
pub fn fxp_q4_to_int_roundup(val_q4: i32) -> i32 { (val_q4 + 0xf) >> 4 }
#[inline]
pub fn fxp_q4_to_frac(val_q4: i32) -> i32 { val_q4 & 0xf }

pub const FXP_Q4_FMT: &str = "%d.%04d";
#[macro_export]
macro_rules! FXP_Q4_ARGS { ($val_q4:expr) => { (fxp_q4_to_int($val_q4), fxp_q4_to_frac($val_q4) * 625) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
