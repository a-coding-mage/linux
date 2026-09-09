/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <asm/param.h>

/*
 * Copyright (C) 1994 Hamish Macdonald
 * Copyright (C) 2004 Greg Ungerer <gerg@uclinux.com>
 *
 * Delay routines, using a pre-computed "loops_per_jiffy" value.
 */

// CONFIG_COLDFIRE selects the aligned ColdFire delay-loop assembly.
#[cfg(CONFIG_COLDFIRE)]
const DELAY_ALIGN: &str = ".balignw 4, 0x4a8e\n\t";

// No instruction alignment required for other m68k types.
#[cfg(not(CONFIG_COLDFIRE))]
const DELAY_ALIGN: &str = "";

extern "C" {
    pub static mut loops_per_jiffy: ::core::ffi::c_ulong;
    pub fn __bad_udelay();
}

#[inline]
pub unsafe fn __delay(mut loops: ::core::ffi::c_ulong) {
    // The original uses m68k inline assembly: subql #1,%0; jcc 1b.
    core::arch::asm!(
        "{align}1: subql #1,%0\n\t",
        "jcc 1b",
        align = const DELAY_ALIGN,
        inout("d0") loops,
        options(nostack, preserves_flags)
    );
}

// CONFIG_CPU_HAS_NO_MULDIV64 selects the shift-and-32-bit-multiply path.
#[cfg(CONFIG_CPU_HAS_NO_MULDIV64)]
pub const HZSCALE: ::core::ffi::c_ulong =
    268435456 / (1000000 / HZ);

#[cfg(CONFIG_CPU_HAS_NO_MULDIV64)]
#[inline]
pub unsafe fn __const_udelay(u: ::core::ffi::c_ulong) {
    __delay((((u * HZSCALE) >> 11) * (loops_per_jiffy >> 11)) >> 6);
}

#[cfg(not(CONFIG_CPU_HAS_NO_MULDIV64))]
#[inline]
pub unsafe fn __xdelay(mut xloops: ::core::ffi::c_ulong) {
    let mut tmp: ::core::ffi::c_ulong;
    core::arch::asm!(
        "mulul {xloops},{xloops}:{tmp}",
        xloops = inout("d0") xloops,
        tmp = lateout("d1") tmp,
        in("d1") loops_per_jiffy,
    );
    __delay(xloops * HZ);
}

// The const factor (4295 = 2**32 / 1000000) is kept at the call site.
#[cfg(not(CONFIG_CPU_HAS_NO_MULDIV64))]
#[inline]
pub unsafe fn __const_udelay(n: ::core::ffi::c_ulong) {
    __xdelay(n * 4295);
}

#[inline]
pub unsafe fn __udelay(usecs: ::core::ffi::c_ulong) {
    __const_udelay(usecs);
}

/*
 * Use only for very small delays ( < 1 msec).  Should probably use a
 * lookup table, really, as the multiplications take much too long with
 * short delays.  This is a "reasonable" implementation, though (and the
 * first constant multiplications gets optimized away if the delay is
 * a constant)
 */
#[inline]
pub unsafe fn udelay(n: ::core::ffi::c_ulong) {
    // __builtin_constant_p cannot be represented directly; callers may use
    // the constant path explicitly through __const_udelay.
    if n > 20000 {
        __bad_udelay();
    } else {
        __udelay(n);
    }
}

/*
 * nanosecond delay:
 *
 * ((((HZSCALE) >> 11) * (loops_per_jiffy >> 11)) >> 6) is the number of loops
 * per microsecond
 *
 * 1000 / ((((HZSCALE) >> 11) * (loops_per_jiffy >> 11)) >> 6) is the number of
 * nanoseconds per loop
 *
 * So n / ( 1000 / ((((HZSCALE) >> 11) * (loops_per_jiffy >> 11)) >> 6) ) would
 * be the number of loops for n nanoseconds
 */

/*
 * The simpler m68k and ColdFire processors do not have a 32*32->64
 * multiply instruction. So we need to handle them a little differently.
 * We use a bit of shifting and a single 32*32->32 multiply to get close.
 * This is a macro so that the const version can factor out the first
 * multiply and shift.
 */
pub const HZSCALE_NSEC: ::core::ffi::c_ulong =
    268435456 / (1000000 / HZ);

#[inline]
pub unsafe fn ndelay(nsec: ::core::ffi::c_ulong) {
    __delay(DIV_ROUND_UP(
        nsec * (((HZSCALE_NSEC >> 11) * (loops_per_jiffy >> 11)) >> 6),
        1000,
    ));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
