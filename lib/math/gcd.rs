// SPDX-License-Identifier: GPL-2.0-only
// Dependencies corresponding to <linux/kernel.h>, <linux/gcd.h>, and
// <linux/export.h> are supplied by the surrounding build.

use core::ffi::c_ulong;

extern "C" {
    fn __ffs(word: c_ulong) -> c_ulong;
    fn static_branch_likely(key: *const core::ffi::c_void) -> bool;
    static efficient_ffs_key: core::ffi::c_void;
}

// This implements the binary GCD algorithm. (Often attributed to Stein,
// but as Knuth has noted, appears in a first-century Chinese math text.)
//
// This is faster than the division-based algorithm even on x86, which
// has decent hardware division.

#[cfg(not(CONFIG_CPU_NO_EFFICIENT_FFS))]
unsafe fn binary_gcd(mut a: c_ulong, mut b: c_ulong) -> c_ulong {
    let r = a | b;

    b >>= __ffs(b);
    if b == 1 {
        return r & r.wrapping_neg();
    }

    loop {
        a >>= __ffs(a);
        if a == 1 {
            return r & r.wrapping_neg();
        }
        if a == b {
            return a << __ffs(r);
        }

        if a < b {
            core::mem::swap(&mut a, &mut b);
        }
        a = a.wrapping_sub(b);
    }
}

// If normalization is done by loops, the even/odd algorithm is a win.

/// gcd - calculate and return the greatest common divisor of 2 unsigned longs
/// @a: first value
/// @b: second value
pub unsafe fn gcd(mut a: c_ulong, mut b: c_ulong) -> c_ulong {
    let mut r = a | b;

    if a == 0 || b == 0 {
        return r;
    }

    #[cfg(not(CONFIG_CPU_NO_EFFICIENT_FFS))]
    if static_branch_likely(&efficient_ffs_key) {
        return binary_gcd(a, b);
    }

    // Isolate lsbit of r
    r &= r.wrapping_neg();

    while (b & r) == 0 {
        b >>= 1;
    }
    if b == r {
        return r;
    }

    loop {
        while (a & r) == 0 {
            a >>= 1;
        }
        if a == r {
            return r;
        }
        if a == b {
            return a;
        }

        if a < b {
            core::mem::swap(&mut a, &mut b);
        }
        a = a.wrapping_sub(b);
        a >>= 1;
        if (a & r) != 0 {
            a = a.wrapping_add(b);
        }
        a >>= 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
