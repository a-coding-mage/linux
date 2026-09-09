/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Count leading and trailing zeros functions
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Dependency supplied by asm/bitops.h. */
extern "C" {
    fn fls(x: usize) -> i32;
    fn fls64(x: u64) -> i32;
    fn __ffs(x: usize) -> i32;
}

/**
 * count_leading_zeros - Count the number of zeros from the MSB back
 * @x: The value
 *
 * Count the number of leading zeros from the MSB going towards the LSB in @x.
 *
 * If the MSB of @x is set, the result is 0.
 * If only the LSB of @x is set, then the result is BITS_PER_LONG-1.
 * If @x is 0 then the result is BITS_PER_LONG.
 */
#[inline]
pub unsafe fn count_leading_zeros(x: usize) -> i32 {
    if core::mem::size_of::<usize>() == 4 {
        BITS_PER_LONG - fls(x)
    } else {
        BITS_PER_LONG - fls64(x as u64)
    }
}

/**
 * count_trailing_zeros - Count the number of zeros from the LSB forwards
 * @x: The value
 *
 * Count the number of trailing zeros from the LSB going towards the MSB in @x.
 *
 * If the LSB of @x is set, the result is 0.
 * If only the MSB of @x is set, then the result is BITS_PER_LONG-1.
 * If @x is 0 then the result is BITS_PER_LONG.
 */
#[inline]
pub unsafe fn count_trailing_zeros(x: usize) -> i32 {
    if x != 0 {
        __ffs(x)
    } else {
        BITS_PER_LONG
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
