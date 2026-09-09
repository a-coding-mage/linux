/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2008 Michal Simek
 * Copyright (C) 2007 John Williams
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

/* HZ is supplied by the surrounding kernel environment. */

/// Delay for the specified number of processor loops.
#[inline]
pub unsafe fn __delay(mut loops: libc::c_ulong) {
    // The original implementation uses MicroBlaze inline assembly.  This
    // preserves its decrement-and-test loop semantics.
    while {
        loops = loops.wrapping_sub(1);
        loops != 0
    } {}
}

/*
 * Note that 19 * 226 == 4294 ==~ 2^32 / 10^6, so
 * loops = (4294 * usecs * loops_per_jiffy * HZ) / 2^32.
 *
 * The mul instruction gives us loops = (a * b) / 2^32.
 * We choose a = usecs * 19 * HZ and b = loops_per_jiffy * 226
 * because this lets us support a wide range of HZ and
 * loops_per_jiffy values without either a or b overflowing 2^32.
 * Thus we need usecs * HZ <= (2^32 - 1) / 19 = 226050910 and
 * loops_per_jiffy <= (2^32 - 1) / 226 = 19004280
 * (which corresponds to ~3800 bogomips at HZ = 100).
 * -- paulus
 */
pub const __MAX_UDELAY: libc::c_ulong = 226050910u64 as libc::c_ulong / HZ;
pub const __MAX_NDELAY: libc::c_ulong = 4294967295u64 as libc::c_ulong / HZ;

extern "C" {
    pub static mut loops_per_jiffy: libc::c_ulong;
}

#[inline]
pub unsafe fn __udelay(x: libc::c_uint) {
    let tmp = (x as u64)
        .wrapping_mul(loops_per_jiffy as u64)
        .wrapping_mul(226u64);
    let loops = (tmp >> 32) as libc::c_ulong;
    __delay(loops);
}

extern "C" {
    pub fn __bad_udelay(); /* deliberately undefined */
    pub fn __bad_ndelay(); /* deliberately undefined */
}

/* __builtin_constant_p is a compiler extension; these macros retain the
 * source-level checks and arithmetic while using Rust macro expressions. */
#[macro_export]
macro_rules! udelay {
    ($n:expr) => {{
        let n = $n;
        if n / $crate::__MAX_UDELAY >= 1 {
            unsafe { $crate::__bad_udelay() }
        } else {
            unsafe { $crate::__udelay(n.wrapping_mul(19u32.wrapping_mul(HZ as u32))) }
        }
    }};
}

#[macro_export]
macro_rules! ndelay {
    ($n:expr) => {{
        let n = $n;
        if n / $crate::__MAX_NDELAY >= 1 {
            unsafe { $crate::__bad_ndelay() }
        } else {
            unsafe { $crate::__udelay(n.wrapping_mul(HZ as u32)) }
        }
    }};
}

#[inline]
pub const fn muldiv(a: u64, b: u64, c: u64) -> u64 {
    (a * b) / c
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
