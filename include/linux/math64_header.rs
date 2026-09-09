/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/math64.h. External Linux types and helpers are supplied by dependencies. */

#[cfg(target_pointer_width = "64")]
pub const fn div64_long(x: i64, y: i64) -> i64 { div64_s64(x, y) }
#[cfg(target_pointer_width = "64")]
pub const fn div64_ul(x: u64, y: u64) -> u64 { div64_u64(x, y) }

#[cfg(target_pointer_width = "64")]
pub unsafe fn div_u64_rem(dividend: u64, divisor: u32, remainder: *mut u32) -> u64 {
    *remainder = dividend % divisor as u64 as u32;
    dividend / divisor as u64
}

#[cfg(target_pointer_width = "64")]
pub unsafe fn div_s64_rem(dividend: i64, divisor: i32, remainder: *mut i32) -> i64 {
    *remainder = (dividend % divisor as i64) as i32;
    dividend / divisor as i64
}

#[cfg(target_pointer_width = "64")]
pub unsafe fn div64_u64_rem(dividend: u64, divisor: u64, remainder: *mut u64) -> u64 {
    *remainder = dividend % divisor;
    dividend / divisor
}

#[cfg(target_pointer_width = "64")]
pub unsafe fn div64_s64_rem(dividend: i64, divisor: i64, remainder: *mut i64) -> i64 {
    *remainder = dividend % divisor;
    dividend / divisor
}

#[cfg(target_pointer_width = "64")]
pub const fn div64_u64(dividend: u64, divisor: u64) -> u64 { dividend / divisor }
#[cfg(target_pointer_width = "64")]
pub const fn div64_s64(dividend: i64, divisor: i64) -> i64 { dividend / divisor }

#[cfg(target_pointer_width = "32")]
pub const fn div64_long(x: i64, y: i32) -> i64 { div_s64(x, y) }
#[cfg(target_pointer_width = "32")]
pub const fn div64_ul(x: u64, y: u32) -> u64 { div_u64(x, y) }

#[cfg(target_pointer_width = "32")]
pub unsafe fn div_u64_rem(dividend: u64, divisor: u32, remainder: *mut u32) -> u64 {
    *remainder = (dividend % divisor as u64) as u32;
    dividend / divisor as u64
}
#[cfg(target_pointer_width = "32")]
extern "C" {
    pub fn div_s64_rem(dividend: i64, divisor: i32, remainder: *mut i32) -> i64;
    pub fn div64_u64_rem(dividend: u64, divisor: u64, remainder: *mut u64) -> u64;
    pub fn div64_s64_rem(dividend: i64, divisor: i64, remainder: *mut i64) -> i64;
    pub fn div64_u64(dividend: u64, divisor: u64) -> u64;
    pub fn div64_s64(dividend: i64, divisor: i64) -> i64;
}

pub unsafe fn div_u64(dividend: u64, divisor: u32) -> u64 {
    let mut remainder = 0u32;
    div_u64_rem(dividend, divisor, &mut remainder)
}
pub unsafe fn div_s64(dividend: i64, divisor: i32) -> i64 {
    let mut remainder = 0i32;
    div_s64_rem(dividend, divisor, &mut remainder)
}

extern "C" {
    pub fn iter_div_u64_rem(dividend: u64, divisor: u32, remainder: *mut u64) -> u32;
}

pub const fn mul_u32_u32(a: u32, b: u32) -> u64 { a as u64 * b as u64 }
pub const fn add_u64_u32(a: u64, b: u32) -> u64 { a + b as u64 }

#[cfg(all(target_pointer_width = "64", feature = "arch_supports_int128"))]
pub const fn mul_u64_u32_shr(a: u64, mul: u32, shift: u32) -> u64 {
    (((a as u128) * mul as u128) >> shift) as u64
}
#[cfg(all(target_pointer_width = "64", feature = "arch_supports_int128"))]
pub const fn mul_u64_u64_shr(a: u64, mul: u64, shift: u32) -> u64 {
    (((a as u128) * mul as u128) >> shift) as u64
}

#[cfg(not(all(target_pointer_width = "64", feature = "arch_supports_int128")))]
pub fn mul_u64_u32_shr(a: u64, mul: u32, shift: u32) -> u64 {
    let ah = a >> 32;
    let al = a as u32;
    let mut ret = mul_u32_u32(al, mul) >> shift;
    if ah != 0 { ret += mul_u32_u32(ah as u32, mul) << (32 - shift); }
    ret
}

#[cfg(not(all(target_pointer_width = "64", feature = "arch_supports_int128")))]
pub fn mul_u64_u64_shr(a: u64, b: u64, shift: u32) -> u64 {
    let result = (a as u128) * (b as u128);
    (result >> shift) as u64
}

pub fn mul_s64_u64_shr(a: i64, b: u64, shift: u32) -> u64 {
    let mut ret = mul_u64_u64_shr(a.unsigned_abs(), b, shift);
    if a < 0 { ret = (-(ret as i64)) as u64; }
    ret
}

pub fn mul_u64_u32_div(a: u64, mul: u32, divisor: u32) -> u64 {
    ((a as u128 * mul as u128) / divisor as u128) as u64
}

extern "C" {
    pub fn mul_u64_add_u64_div_u64(a: u64, b: u64, c: u64, d: u64) -> u64;
}

pub fn mul_u64_u64_div_u64(a: u64, b: u64, d: u64) -> u64 {
    unsafe { mul_u64_add_u64_div_u64(a, b, 0, d) }
}
pub fn mul_u64_u64_div_u64_roundup(a: u64, b: u64, d: u64) -> u64 {
    unsafe { mul_u64_add_u64_div_u64(a, b, d - 1, d) }
}
pub fn div64_u64_round_up(ll: u64, d: u64) -> u64 { div64_u64(ll + d - 1, d) }
pub fn div_u64_round_up(ll: u64, d: u32) -> u64 { unsafe { div_u64(ll + d as u64 - 1, d) } }
pub fn div64_u64_round_closest(dividend: u64, divisor: u64) -> u64 { div64_u64(dividend + divisor / 2, divisor) }
pub fn div_u64_round_closest(dividend: u64, divisor: u32) -> u64 { unsafe { div_u64(dividend + divisor as u64 / 2, divisor) } }
pub fn div_s64_round_closest(dividend: i64, divisor: i32) -> i64 {
    if (dividend > 0) == (divisor > 0) {
        unsafe { div_s64(dividend + divisor as i64 / 2, divisor) }
    } else {
        unsafe { div_s64(dividend - divisor as i64 / 2, divisor) }
    }
}
pub fn roundup_u64(x: u64, y: u32) -> u64 { div_u64_round_up(x, y) * y as u64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
