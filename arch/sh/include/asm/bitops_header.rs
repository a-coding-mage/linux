/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This header requires the Linux bitops interface.  The C include guard and
 * preprocessor-only inclusion check have no executable Rust equivalent.
 */

/* For __swab32: dependencies supplied by asm::byteorder and asm::barrier. */

/*
 * Configuration-dependent bitops implementations supplied by the original
 * headers:
 *
 * CONFIG_GUSA_RB     -> asm/bitops-grb.h
 * CONFIG_CPU_SH2A    -> asm-generic/bitops/atomic.h, asm/bitops-op32.h
 * CONFIG_CPU_SH4A    -> asm/bitops-llsc.h
 * CONFIG_CPU_J2+SMP  -> asm/bitops-cas.h
 * otherwise          -> asm-generic/bitops/atomic.h,
 *                       asm-generic/bitops/non-atomic.h
 */

/// Find the first zero bit in `word`.
///
/// The original implementation is undefined/non-terminating when no zero bit
/// exists; this loop preserves that behavior for an all-ones word.
#[inline]
pub fn ffz(mut word: usize) -> usize {
    let mut result = usize::MAX;
    loop {
        let bit = word & 1;
        word >>= 1;
        result = result.wrapping_add(1);
        if bit == 0 {
            return result;
        }
    }
}

/// __ffs - find first bit in word.
///
/// Undefined if no bit exists, so code should check against 0 first.
#[inline]
pub fn __ffs(mut word: usize) -> usize {
    let mut result = usize::MAX;
    loop {
        let bit = word & 1;
        word >>= 1;
        result = result.wrapping_add(1);
        if bit != 0 {
            return result;
        }
    }
}

/* Declarations supplied by asm-generic/bitops/{ffs,hweight,lock,sched,
 * ext2-atomic,fls,__fls,fls64,le}.h remain external dependencies. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
