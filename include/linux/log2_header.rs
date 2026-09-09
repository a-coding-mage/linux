/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Integer base 2 logarithm calculation
 *
 * Copyright (C) 2006 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// C dependencies: linux/types.h and linux/bitops.h.
// Architecture configuration may provide optimized ilog2 implementations.

#[cfg(not(CONFIG_ARCH_HAS_ILOG2_U32))]
#[inline(always)]
pub const fn __ilog2_u32(n: u32) -> i32 {
    fls(n) - 1
}

#[cfg(not(CONFIG_ARCH_HAS_ILOG2_U64))]
#[inline(always)]
pub const fn __ilog2_u64(n: u64) -> i32 {
    fls64(n) - 1
}

// Supplied by linux/bitops.h.
extern "C" {
    pub fn fls(n: u32) -> i32;
    pub fn fls64(n: u64) -> i32;
    pub fn fls_long(n: c_ulong) -> i32;
}

// C unsigned long, whose width follows the target ABI.
pub type c_ulong = usize;

/**
 * is_power_of_2() - check if a value is a power of two
 * @n: the value to check
 *
 * Determine whether some value is a power of two, where zero is
 * *not* considered a power of two.
 * Return: true if @n is a power of 2, otherwise false.
 */
#[inline(always)]
pub const fn is_power_of_2(n: c_ulong) -> bool {
    n.wrapping_sub(1) < (n ^ n.wrapping_sub(1))
}

/**
 * __roundup_pow_of_two() - round up to nearest power of two
 * @n: value to round up
 */
#[inline]
pub fn __roundup_pow_of_two(n: c_ulong) -> c_ulong {
    1usize << fls_long(n.wrapping_sub(1))
}

/**
 * __rounddown_pow_of_two() - round down to nearest power of two
 * @n: value to round down
 */
#[inline]
pub fn __rounddown_pow_of_two(n: c_ulong) -> c_ulong {
    1usize << (fls_long(n) - 1)
}

// const_ilog2 is a C macro; Rust callers should use ilog2! below.
#[macro_export]
macro_rules! const_ilog2 {
    ($n:expr) => {{
        let n = $n as u64;
        if n < 2 { 0i32 }
        else if n & (1u64 << 63) != 0 { 63 }
        else { 63 - n.leading_zeros() as i32 }
    }};
}

/**
 * ilog2 - log base 2 of 32-bit or a 64-bit unsigned value
 * @n: parameter
 *
 * constant-capable log of base 2 calculation.
 */
#[macro_export]
macro_rules! ilog2 {
    ($n:expr) => {{
        let n = $n;
        if n < 2 { 0i32 }
        else { (64 - (n as u64).leading_zeros() as i32 - 1) }
    }};
}

/**
 * roundup_pow_of_two - round the given value up to nearest power of two
 * @n: parameter
 */
#[macro_export]
macro_rules! roundup_pow_of_two {
    ($n:expr) => {{
        let n = $n;
        if n == 1 { 1usize } else { 1usize << ($crate::ilog2!(n - 1) + 1) }
    }};
}

/**
 * rounddown_pow_of_two - round the given value down to nearest power of two
 * @n: parameter
 */
#[macro_export]
macro_rules! rounddown_pow_of_two {
    ($n:expr) => {{ 1usize << $crate::ilog2!($n) }};
}

#[inline]
pub fn __order_base_2(n: c_ulong) -> i32 {
    if n > 1 { ilog2!(n.wrapping_sub(1)) + 1 } else { 0 }
}

/**
 * order_base_2 - calculate the (rounded up) base 2 order of the argument
 * @n: parameter
 */
#[macro_export]
macro_rules! order_base_2 {
    ($n:expr) => {{
        let n = $n;
        if n == 0 || n == 1 { 0i32 } else { $crate::ilog2!(n - 1) + 1 }
    }};
}

#[inline]
pub fn __bits_per(n: c_ulong) -> i32 {
    if n < 2 { return 1; }
    if is_power_of_2(n) { return order_base_2!(n) + 1; }
    order_base_2!(n)
}

/**
 * bits_per - calculate the number of bits required for the argument
 * @n: parameter
 */
#[macro_export]
macro_rules! bits_per {
    ($n:expr) => {{
        let n = $n;
        if n == 0 || n == 1 { 1i32 } else { $crate::ilog2!(n) + 1 }
    }};
}

/**
 * max_pow_of_two_factor - return highest power-of-2 factor
 * @n: parameter
 *
 * find highest power-of-2 which is evenly divisible into n.
 * 0 is returned for n == 0 or 1.
 */
#[inline]
pub const fn max_pow_of_two_factor(n: u32) -> u32 {
    n & n.wrapping_neg()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
