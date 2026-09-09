// SPDX-License-Identifier: GPL-2.0-only
// Dependencies: linux/compiler.h, linux/gcd.h, linux/export.h, linux/lcm.h

use core::ffi::c_ulong;

unsafe extern "C" {
    fn gcd(a: c_ulong, b: c_ulong) -> c_ulong;
}

/* Lowest common multiple */
pub unsafe fn lcm(a: c_ulong, b: c_ulong) -> c_ulong {
    if a != 0 && b != 0 {
        (a / gcd(a, b)) * b
    } else {
        0
    }
}

// EXPORT_SYMBOL_GPL(lcm);

pub unsafe fn lcm_not_zero(a: c_ulong, b: c_ulong) -> c_ulong {
    let l = lcm(a, b);

    if l != 0 {
        return l;
    }

    if b != 0 { b } else { a }
}

// EXPORT_SYMBOL_GPL(lcm_not_zero);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
