/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Integer base 2 logarithm calculation
 *
 * Copyright (C) 2006 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Depends on linux/bitops.h and linux/types.h for fls(), fls64(), fls_long(),
 * u32, and u64 in the original C header.
 */

use core::ffi::c_ulong;

extern "C" {
    fn fls(n: u32) -> i32;
    fn fls64(n: u64) -> i32;
    fn fls_long(n: c_ulong) -> i32;
}

/*
 * non-constant log of base 2 calculators
 * - the arch may override these in asm/bitops.h if they can be implemented
 *   more efficiently than using fls() and fls64()
 * - the arch is not required to handle n==0 if implementing the fallback
 */
#[inline]
pub unsafe fn __ilog2_u32(n: u32) -> i32 {
    fls(n) - 1
}

#[inline]
pub unsafe fn __ilog2_u64(n: u64) -> i32 {
    fls64(n) - 1
}

/*
 *  Determine whether some value is a power of two, where zero is
 * *not* considered a power of two.
 */

#[inline]
pub fn is_power_of_2(n: c_ulong) -> bool {
    n != 0 && ((n & n.wrapping_sub(1)) == 0)
}

/*
 * round up to nearest power of two
 */
#[inline]
pub unsafe fn __roundup_pow_of_two(n: c_ulong) -> c_ulong {
    1 as c_ulong << fls_long(n.wrapping_sub(1))
}

/*
 * round down to nearest power of two
 */
#[inline]
pub unsafe fn __rounddown_pow_of_two(n: c_ulong) -> c_ulong {
    1 as c_ulong << (fls_long(n) - 1)
}

/**
 * ilog2 - log of base 2 of 32-bit or a 64-bit unsigned value
 * @n - parameter
 *
 * constant-capable log of base 2 calculation
 * - this can be used to initialise global variables from constant data, hence
 *   the massive ternary operator construction
 *
 * selects the appropriately-sized optimised version depending on sizeof(n)
 */
#[inline]
pub const fn ilog2_const(n: u64) -> i32 {
    if n < 2 {
        0
    } else if (n & (1u64 << 63)) != 0 {
        63
    } else if (n & (1u64 << 62)) != 0 {
        62
    } else if (n & (1u64 << 61)) != 0 {
        61
    } else if (n & (1u64 << 60)) != 0 {
        60
    } else if (n & (1u64 << 59)) != 0 {
        59
    } else if (n & (1u64 << 58)) != 0 {
        58
    } else if (n & (1u64 << 57)) != 0 {
        57
    } else if (n & (1u64 << 56)) != 0 {
        56
    } else if (n & (1u64 << 55)) != 0 {
        55
    } else if (n & (1u64 << 54)) != 0 {
        54
    } else if (n & (1u64 << 53)) != 0 {
        53
    } else if (n & (1u64 << 52)) != 0 {
        52
    } else if (n & (1u64 << 51)) != 0 {
        51
    } else if (n & (1u64 << 50)) != 0 {
        50
    } else if (n & (1u64 << 49)) != 0 {
        49
    } else if (n & (1u64 << 48)) != 0 {
        48
    } else if (n & (1u64 << 47)) != 0 {
        47
    } else if (n & (1u64 << 46)) != 0 {
        46
    } else if (n & (1u64 << 45)) != 0 {
        45
    } else if (n & (1u64 << 44)) != 0 {
        44
    } else if (n & (1u64 << 43)) != 0 {
        43
    } else if (n & (1u64 << 42)) != 0 {
        42
    } else if (n & (1u64 << 41)) != 0 {
        41
    } else if (n & (1u64 << 40)) != 0 {
        40
    } else if (n & (1u64 << 39)) != 0 {
        39
    } else if (n & (1u64 << 38)) != 0 {
        38
    } else if (n & (1u64 << 37)) != 0 {
        37
    } else if (n & (1u64 << 36)) != 0 {
        36
    } else if (n & (1u64 << 35)) != 0 {
        35
    } else if (n & (1u64 << 34)) != 0 {
        34
    } else if (n & (1u64 << 33)) != 0 {
        33
    } else if (n & (1u64 << 32)) != 0 {
        32
    } else if (n & (1u64 << 31)) != 0 {
        31
    } else if (n & (1u64 << 30)) != 0 {
        30
    } else if (n & (1u64 << 29)) != 0 {
        29
    } else if (n & (1u64 << 28)) != 0 {
        28
    } else if (n & (1u64 << 27)) != 0 {
        27
    } else if (n & (1u64 << 26)) != 0 {
        26
    } else if (n & (1u64 << 25)) != 0 {
        25
    } else if (n & (1u64 << 24)) != 0 {
        24
    } else if (n & (1u64 << 23)) != 0 {
        23
    } else if (n & (1u64 << 22)) != 0 {
        22
    } else if (n & (1u64 << 21)) != 0 {
        21
    } else if (n & (1u64 << 20)) != 0 {
        20
    } else if (n & (1u64 << 19)) != 0 {
        19
    } else if (n & (1u64 << 18)) != 0 {
        18
    } else if (n & (1u64 << 17)) != 0 {
        17
    } else if (n & (1u64 << 16)) != 0 {
        16
    } else if (n & (1u64 << 15)) != 0 {
        15
    } else if (n & (1u64 << 14)) != 0 {
        14
    } else if (n & (1u64 << 13)) != 0 {
        13
    } else if (n & (1u64 << 12)) != 0 {
        12
    } else if (n & (1u64 << 11)) != 0 {
        11
    } else if (n & (1u64 << 10)) != 0 {
        10
    } else if (n & (1u64 << 9)) != 0 {
        9
    } else if (n & (1u64 << 8)) != 0 {
        8
    } else if (n & (1u64 << 7)) != 0 {
        7
    } else if (n & (1u64 << 6)) != 0 {
        6
    } else if (n & (1u64 << 5)) != 0 {
        5
    } else if (n & (1u64 << 4)) != 0 {
        4
    } else if (n & (1u64 << 3)) != 0 {
        3
    } else if (n & (1u64 << 2)) != 0 {
        2
    } else {
        1
    }
}

#[inline]
pub unsafe fn ilog2(n: u64) -> i32 {
    if core::mem::size_of_val(&n) <= 4 {
        __ilog2_u32(n as u32)
    } else {
        __ilog2_u64(n)
    }
}

/**
 * roundup_pow_of_two - round the given value up to nearest power of two
 * @n - parameter
 *
 * round the given value up to the nearest power of two
 * - the result is undefined when n == 0
 * - this can be used to initialise global variables from constant data
 */
#[inline]
pub const fn roundup_pow_of_two_const(n: c_ulong) -> c_ulong {
    if n == 1 {
        1
    } else {
        1 as c_ulong << (ilog2_const(n.wrapping_sub(1) as u64) + 1)
    }
}

#[inline]
pub unsafe fn roundup_pow_of_two(n: c_ulong) -> c_ulong {
    __roundup_pow_of_two(n)
}

/**
 * rounddown_pow_of_two - round the given value down to nearest power of two
 * @n - parameter
 *
 * round the given value down to the nearest power of two
 * - the result is undefined when n == 0
 * - this can be used to initialise global variables from constant data
 */
#[inline]
pub const fn rounddown_pow_of_two_const(n: c_ulong) -> c_ulong {
    1 as c_ulong << ilog2_const(n as u64)
}

#[inline]
pub unsafe fn rounddown_pow_of_two(n: c_ulong) -> c_ulong {
    __rounddown_pow_of_two(n)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
