/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by asm/compiler.h in the original header.

/*
 * word-at-a-time interface for Alpha.
 */

/*
 * We do not use the word_at_a_time struct on Alpha, but it needs to be
 * implemented to humour the generic code.
 */
#[repr(C)]
pub struct word_at_a_time {
    pub unused: u64,
}

pub const WORD_AT_A_TIME_CONSTANTS: word_at_a_time = word_at_a_time { unused: 0 };

extern "C" {
    pub fn __kernel_cmpbge(a: u64, b: u64) -> u64;
    pub fn __kernel_cttz(bits: u64) -> u64;
}

/* Return nonzero if val has a zero */
#[inline]
pub unsafe fn has_zero(
    val: u64,
    bits: *mut u64,
    _c: *const word_at_a_time,
) -> u64 {
    let zero_locations = __kernel_cmpbge(0, val);
    *bits = zero_locations;
    zero_locations
}

#[inline]
pub unsafe fn prep_zero_mask(
    _val: u64,
    bits: u64,
    _c: *const word_at_a_time,
) -> u64 {
    bits
}

#[inline]
pub const fn create_zero_mask(bits: u64) -> u64 {
    bits
}

#[inline]
pub unsafe fn find_zero(mut bits: u64) -> u64 {
    // The original condition is CONFIG_ALPHA_EV6 && CONFIG_ALPHA_EV67.
    // Define both cfg features when the Alpha CIX instructions are available.
    #[cfg(all(feature = "CONFIG_ALPHA_EV6", feature = "CONFIG_ALPHA_EV67"))]
    {
        /* Simple if have CIX instructions */
        return __kernel_cttz(bits);
    }

    #[cfg(not(all(feature = "CONFIG_ALPHA_EV6", feature = "CONFIG_ALPHA_EV67")))]
    {
        let mut t1: u64;
        let mut t2: u64;
        let mut t3: u64;
        /* Retain lowest set bit only */
        bits &= bits.wrapping_neg();
        /* Binary search for lowest set bit */
        t1 = bits & 0xf0;
        t2 = bits & 0xcc;
        t3 = bits & 0xaa;
        if t1 != 0 {
            t1 = 4;
        }
        if t2 != 0 {
            t2 = 2;
        }
        if t3 != 0 {
            t3 = 1;
        }
        return t1 + t2 + t3;
    }
}

#[inline]
pub unsafe fn zero_bytemask(mask: u64) -> u64 {
    (2u64.wrapping_shl(find_zero(mask) as u32 * 8)).wrapping_sub(1)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
