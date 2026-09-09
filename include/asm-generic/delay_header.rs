/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding translation unit:
 * linux/math.h and vdso/time64.h
 */

use core::ffi::c_ulong;

/* Undefined functions to get compile-time errors */
unsafe extern "C" {
    pub fn __bad_udelay();
    pub fn __bad_ndelay();

    pub fn __udelay(usecs: c_ulong);
    pub fn __ndelay(nsecs: c_ulong);
    pub fn __const_udelay(xloops: c_ulong);
    pub fn __delay(loops: c_ulong);
}

/*
 * The microseconds/nanosecond delay multiplicators are used to convert a
 * constant microseconds/nanoseconds value to a value which can be used by
 * the architectures specific implementation to transform it into loops.
 */
pub const UDELAY_CONST_MULT: c_ulong =
    (((1u64 << 32) + (USEC_PER_SEC as u64) - 1) / (USEC_PER_SEC as u64)) as c_ulong;
pub const NDELAY_CONST_MULT: c_ulong =
    (((1u64 << 32) + (NSEC_PER_SEC as u64) - 1) / (NSEC_PER_SEC as u64)) as c_ulong;

/*
 * The maximum constant udelay/ndelay value picked out of thin air to prevent
 * too long constant udelays/ndelays.
 */
pub const DELAY_CONST_MAX: c_ulong = 20000;

/**
 * udelay - Inserting a delay based on microseconds with busy waiting
 * @usec: requested delay in microseconds
 *
 * When delaying in an atomic context ndelay(), udelay() and mdelay() are the
 * only valid variants of delaying/sleeping to go with.
 *
 * When inserting delays in non atomic context which are shorter than the time
 * which is required to queue e.g. an hrtimer and to enter then the scheduler,
 * it is also valuable to use udelay(). But it is not simple to specify a
 * generic threshold for this which will fit for all systems. An approximation
 * is a threshold for all delays up to 10 microseconds.
 *
 * When having a delay which is larger than the architecture specific
 * %MAX_UDELAY_MS value, please make sure mdelay() is used. Otherwise a overflow
 * risk is given.
 *
 * Please note that ndelay(), udelay() and mdelay() may return early for several
 * reasons (https://lists.openwall.net/linux-kernel/2011/01/09/56):
 *
 * #. computed loops_per_jiffy too low (due to the time taken to execute the
 *    timer interrupt.)
 * #. cache behaviour affecting the time it takes to execute the loop function.
 * #. CPU clock rate changes.
 */
#[inline(always)]
pub unsafe fn udelay(usec: c_ulong) {
    unsafe { __udelay(usec) };
}

/* Literal-call form preserving the C __builtin_constant_p() path. */
#[macro_export]
macro_rules! udelay {
    ($usec:expr) => {{
        let __usec: c_ulong = $usec as c_ulong;
        if __usec >= DELAY_CONST_MAX {
            unsafe { __bad_udelay() }
        } else {
            unsafe { __const_udelay(__usec.wrapping_mul(UDELAY_CONST_MULT)) }
        }
    }};
}

/**
 * ndelay - Inserting a delay based on nanoseconds with busy waiting
 * @nsec: requested delay in nanoseconds
 *
 * See udelay() for basic information about ndelay() and it's variants.
 */
#[inline(always)]
pub unsafe fn ndelay(nsec: c_ulong) {
    unsafe { __ndelay(nsec) };
}

#[macro_export]
macro_rules! ndelay {
    ($nsec:expr) => {{
        let __nsec: c_ulong = $nsec as c_ulong;
        if __nsec >= DELAY_CONST_MAX {
            unsafe { __bad_ndelay() }
        } else {
            unsafe { __const_udelay(__nsec.wrapping_mul(NDELAY_CONST_MULT)) }
        }
    }};
}

/* The C self-referential function-like macro is unnecessary in Rust. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
