/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Integer base 2 logarithm calculation
 *
 * Copyright (C) 2006 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Depends on: linux/types.h, linux/bitops.h */

/*
 * Rust has no __builtin_constant_p(): every macro below takes the runtime
 * branch of the C definition.  The helpers are const fns, so the same
 * expressions still fold in const contexts.  The only input where the two C
 * branches disagree is 0 (constant: 0, runtime: fls(0) - 1 == -1); runtime
 * callers such as the generic radix tree rely on the -1.
 */

/*
 * non-constant log of base 2 calculators
 * - the arch may override these in asm/bitops.h if they can be implemented
 *   more efficiently than using fls() and fls64()
 * - the arch is not required to handle n==0 if implementing the fallback
 */
#[cfg(not(CONFIG_ARCH_HAS_ILOG2_U32))]
#[inline(always)]
pub const fn __ilog2_u32(n: u32) -> kernel::ffi::c_int {
    fls(n) - 1
}

#[cfg(not(CONFIG_ARCH_HAS_ILOG2_U64))]
#[inline(always)]
pub const fn __ilog2_u64(n: u64) -> kernel::ffi::c_int {
    fls64(n) - 1
}

/**
 * is_power_of_2() - check if a value is a power of two
 * @n: the value to check
 *
 * Determine whether some value is a power of two, where zero is
 * *not* considered a power of two.
 * Return: true if @n is a power of 2, otherwise false.
 */
#[inline(always)]
pub const fn is_power_of_2(n: kernel::ffi::c_ulong) -> bool {
    n.wrapping_sub(1) < (n ^ n.wrapping_sub(1))
}

/**
 * __roundup_pow_of_two() - round up to nearest power of two
 * @n: value to round up
 */
#[inline]
pub const fn __roundup_pow_of_two(n: kernel::ffi::c_ulong) -> kernel::ffi::c_ulong {
    1 << fls_long(n.wrapping_sub(1))
}

/**
 * __rounddown_pow_of_two() - round down to nearest power of two
 * @n: value to round down
 */
#[inline]
pub const fn __rounddown_pow_of_two(n: kernel::ffi::c_ulong) -> kernel::ffi::c_ulong {
    1 << (fls_long(n) - 1)
}

/**
 * const_ilog2 - log base 2 of 32-bit or a 64-bit constant unsigned value
 * @n: parameter
 *
 * Use this where sparse expects a true constant expression, e.g. for array
 * indices.
 */
#[macro_export]
macro_rules! const_ilog2 {
    ($n:expr) => {{
        let __n = ($n) as u64;
        if __n < 2 { 0 } else { 63 - __n.leading_zeros() as kernel::ffi::c_int }
    }};
}

/**
 * ilog2 - log base 2 of 32-bit or a 64-bit unsigned value
 * @n: parameter
 *
 * constant-capable log of base 2 calculation
 * - this can be used to initialise global variables from constant data, hence
 * the massive ternary operator construction
 *
 * selects the appropriately-sized optimised version depending on sizeof(n)
 */
#[macro_export]
macro_rules! ilog2 {
    ($n:expr) => {{
        let __n = $n;
        if core::mem::size_of_val(&__n) <= 4 {
            __ilog2_u32(__n as u32)
        } else {
            __ilog2_u64(__n as u64)
        }
    }};
}

/**
 * roundup_pow_of_two - round the given value up to nearest power of two
 * @n: parameter
 *
 * round the given value up to the nearest power of two
 * - the result is undefined when n == 0
 * - this can be used to initialise global variables from constant data
 */
#[macro_export]
macro_rules! roundup_pow_of_two {
    ($n:expr) => {
        __roundup_pow_of_two(($n) as kernel::ffi::c_ulong)
    };
}

/**
 * rounddown_pow_of_two - round the given value down to nearest power of two
 * @n: parameter
 *
 * round the given value down to the nearest power of two
 * - the result is undefined when n == 0
 * - this can be used to initialise global variables from constant data
 */
#[macro_export]
macro_rules! rounddown_pow_of_two {
    ($n:expr) => {
        __rounddown_pow_of_two(($n) as kernel::ffi::c_ulong)
    };
}

#[inline]
pub const fn __order_base_2(n: kernel::ffi::c_ulong) -> kernel::ffi::c_int {
    if n > 1 { ilog2!(n - 1) + 1 } else { 0 }
}

/**
 * order_base_2 - calculate the (rounded up) base 2 order of the argument
 * @n: parameter
 *
 * The first few values calculated by this routine:
 *  ob2(0) = 0
 *  ob2(1) = 0
 *  ob2(2) = 1
 *  ob2(3) = 2
 *  ob2(4) = 2
 *  ob2(5) = 3
 *  ... and so on.
 */
#[macro_export]
macro_rules! order_base_2 {
    ($n:expr) => {
        __order_base_2(($n) as kernel::ffi::c_ulong)
    };
}

#[inline]
pub const fn __bits_per(n: kernel::ffi::c_ulong) -> kernel::ffi::c_int {
    if n < 2 {
        return 1;
    }
    if is_power_of_2(n) {
        return order_base_2!(n) + 1;
    }
    order_base_2!(n)
}

/**
 * bits_per - calculate the number of bits required for the argument
 * @n: parameter
 *
 * This is constant-capable and can be used for compile time
 * initializations, e.g bitfields.
 *
 * The first few values calculated by this routine:
 * bf(0) = 1
 * bf(1) = 1
 * bf(2) = 2
 * bf(3) = 2
 * bf(4) = 3
 * ... and so on.
 */
#[macro_export]
macro_rules! bits_per {
    ($n:expr) => {
        __bits_per(($n) as kernel::ffi::c_ulong)
    };
}

/**
 * max_pow_of_two_factor - return highest power-of-2 factor
 * @n: parameter
 *
 * find highest power-of-2 which is evenly divisible into n.
 * 0 is returned for n == 0 or 1.
 */
#[inline]
pub const fn max_pow_of_two_factor(n: kernel::ffi::c_uint) -> kernel::ffi::c_uint {
    n & n.wrapping_neg()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
