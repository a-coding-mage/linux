// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2013 Davidlohr Bueso <davidlohr.bueso@hp.com>
 *
 *  Based on the shift-and-subtract algorithm for computing integer
 *  square root from Guy L. Steele.
 */
//! Integer square roots without allocation or foreign bit-scan functions.

// Keep one implementation of Steele's algorithm, specialized to each input
// width. This local macro creates neither public macros nor exported symbols.
macro_rules! square_root {
    ($value:expr, $word:ty) => {{
        let mut x: $word = $value;
        let mut y: $word = 0;
        if x <= 1 {
            x
        } else {
            let shift = (<$word>::BITS - 1).wrapping_sub(x.leading_zeros()) & !1;
            // x > 1 makes shift an in-range, even bit index. Rotating a lone
            // low bit therefore gives exactly 1 << shift, without a checked
            // shift or an unsafe intrinsic precondition in unoptimized code.
            let mut m = (1 as $word).rotate_left(shift);
            while m != 0 {
                let b = y.wrapping_add(m);
                y >>= 1;
                if x >= b {
                    x = x.wrapping_sub(b);
                    y = y.wrapping_add(m);
                }
                m >>= 2;
            }
            y
        }
    }};
}

/// Returns the floor of the square root of a native-width unsigned integer.
///
/// This matches C's `int_sqrt(unsigned long)` on both 32-bit and 64-bit kernels.
#[inline]
pub const fn int_sqrt(x: usize) -> usize {
    square_root!(x, usize)
}

/// Returns the floor of the square root of an explicitly 32-bit integer.
///
/// This specializes the same algorithm to 32-bit arithmetic regardless of the
/// caller's pointer width. It introduces no additional exported C symbol.
#[inline]
pub const fn int_sqrt32(x: u32) -> u32 {
    square_root!(x, u32)
}

/// Returns the floor of the square root of the complete 64-bit input.
///
/// The result always fits in `u32`. This safe helper exists on every pointer
/// width; unlike the original C inline/declaration split, callers never need
/// to select an implementation or truncate the input on 32-bit kernels.
#[inline]
pub const fn int_sqrt64(x: u64) -> u32 {
    if x <= usize::MAX as u64 {
        int_sqrt(x as usize) as u32
    } else {
        square_root!(x, u64) as u32
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
