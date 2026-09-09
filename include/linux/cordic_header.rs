/*
 * Copyright (c) 2011 Broadcom Corporation
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
 * OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
 * CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

pub const CORDIC_ANGLE_GEN: i32 = 39797;
pub const CORDIC_PRECISION_SHIFT: i32 = 16;
pub const CORDIC_NUM_ITER: i32 = CORDIC_PRECISION_SHIFT + 2;

#[inline]
pub const fn CORDIC_FIXED(x: i32) -> i32 {
    x.wrapping_shl(CORDIC_PRECISION_SHIFT as u32)
}

#[inline]
pub const fn CORDIC_FLOAT(x: i32) -> i32 {
    if x >= 0 {
        ((x >> (CORDIC_PRECISION_SHIFT - 1)) + 1) >> 1
    } else {
        -((((-x) >> (CORDIC_PRECISION_SHIFT - 1)) + 1) >> 1)
    }
}

/**
 * struct cordic_iq - i/q coordinate.
 *
 * @i: real part of coordinate (in phase).
 * @q: imaginary part of coordinate (quadrature).
 */
#[repr(C)]
pub struct cordic_iq {
    pub i: i32,
    pub q: i32,
}

/**
 * cordic_calc_iq() - calculates the i/q coordinate for given angle.
 *
 * @theta: angle in degrees for which i/q coordinate is to be calculated.
 * @coord: function output parameter holding the i/q coordinate.
 *
 * The function calculates the i/q coordinate for a given angle using the
 * CORDIC algorithm. The coordinate consists of a real (i) and an
 * imaginary (q) part. The real part is essentially the cosine of the
 * angle and the imaginary part is the sine of the angle. The returned
 * values are scaled by 2^16 for precision. The range for theta is
 * for -180 degrees to +180 degrees. Passed values outside this range are
 * converted before doing the actual calculation.
 */
unsafe extern "C" {
    pub fn cordic_calc_iq(theta: i32) -> cordic_iq;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
