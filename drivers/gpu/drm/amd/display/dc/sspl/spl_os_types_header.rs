/* SPDX-License-Identifier: MIT */

/* Copyright 2024 Advanced Micro Devices, Inc. */
/* Copyright 2019 Raptor Engineering, LLC */

/*
 * Translated from spl_os_types.h.
 *
 * The Linux declarations referenced by the original includes are supplied by
 * other translation units.
 */

/*
 *
 * general debug capabilities
 *
 */

unsafe extern "C" {
    pub fn div_u64_rem(
        dividend: u64,
        divisor: u32,
        remainder: *mut u32,
    ) -> u64;

    pub fn div_u64(dividend: u64, divisor: u32) -> u64;

    pub fn div64_u64(dividend: u64, divisor: u64) -> u64;

    pub fn div64_u64_rem(
        dividend: u64,
        divisor: u64,
        remainder: *mut u64,
    ) -> u64;

    pub fn div64_s64(dividend: i64, divisor: i64) -> i64;
}

#[inline]
pub unsafe fn spl_div_u64_rem(
    dividend: u64,
    divisor: u32,
    remainder: *mut u32,
) -> u64 {
    unsafe { div_u64_rem(dividend, divisor, remainder) }
}

#[inline]
pub unsafe fn spl_div_u64(dividend: u64, divisor: u32) -> u64 {
    unsafe { div_u64(dividend, divisor) }
}

#[inline]
pub unsafe fn spl_div64_u64(dividend: u64, divisor: u64) -> u64 {
    unsafe { div64_u64(dividend, divisor) }
}

#[inline]
pub unsafe fn spl_div64_u64_rem(
    dividend: u64,
    divisor: u64,
    remainder: *mut u64,
) -> u64 {
    unsafe { div64_u64_rem(dividend, divisor, remainder) }
}

#[inline]
pub unsafe fn spl_div64_s64(dividend: i64, divisor: i64) -> i64 {
    unsafe { div64_s64(dividend, divisor) }
}

#[macro_export]
macro_rules! spl_swap {
    ($a:expr, $b:expr) => {{
        let __tmp = $a;
        $a = $b;
        $b = __tmp;
    }};
}

#[macro_export]
macro_rules! spl_min {
    ($a:expr, $b:expr) => {{
        if ($a) < ($b) { ($a) } else { ($b) }
    }};
}

/* SPL namespace macros. SPL_PFX_ is empty in the original header. */

/*
 * C token-pasting equivalents; Rust macro_rules! cannot concatenate arbitrary
 * identifiers without an external token-pasting facility.
 */
#[macro_export]
macro_rules! SPL_EXPAND2 {
    ($a:ident, $b:ident) => { $a $b };
}

#[macro_export]
macro_rules! SPL_EXPAND {
    ($a:ident, $b:ident) => { $crate::SPL_EXPAND2!($a, $b) };
}

#[macro_export]
macro_rules! SPL_NAMESPACE {
    ($symbol:ident) => { $crate::SPL_EXPAND!(SPL_PFX_, $symbol) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
