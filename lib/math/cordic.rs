/*
 * Copyright (c) 2011 Broadcom Corporation
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
 * SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
 * OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
 * CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

//! Safe, allocation-free fixed-point CORDIC arithmetic.
//!
//! The degree input is converted to signed Q16 before angle normalization,
//! preserving the original kernel C implementation's wrapping arithmetic.
//! This is significant outside the usual -180 through 180 degree range.

/// Initial in-phase coordinate compensating for the CORDIC iteration gain.
pub const CORDIC_ANGLE_GEN: i32 = 39797;
/// Number of fractional bits in the coordinate and angle representation.
pub const CORDIC_PRECISION_SHIFT: u32 = 16;
/// Number of rotation steps performed by the original implementation.
pub const CORDIC_NUM_ITER: usize = CORDIC_PRECISION_SHIFT as usize + 2;

static ARCTAN_TABLE: [i32; CORDIC_NUM_ITER] = [
    2949120, 1740967, 919879, 466945, 234379, 117304, 58666, 29335, 14668, 7334, 3667, 1833, 917,
    458, 229, 115, 57, 29,
];

/// In-phase and quadrature coordinates, scaled by 2^16.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CordicIq {
    /// Real (in-phase) coordinate.
    pub i: i32,
    /// Imaginary (quadrature) coordinate.
    pub q: i32,
}

/// Convert an integer to signed Q16, retaining the low 32 result bits.
pub const fn cordic_fixed(value: i32) -> i32 {
    value << CORDIC_PRECISION_SHIFT
}

/// Round signed Q16 to the nearest integer, preserving C's wrapping negation.
///
/// Ties normally round away from zero. For `i32::MIN`, the intermediate
/// negation wraps, so the original macro returns positive 32768.
pub const fn cordic_float(value: i32) -> i32 {
    if value >= 0 {
        ((value >> (CORDIC_PRECISION_SHIFT - 1)).wrapping_add(1)) >> 1
    } else {
        (((value.wrapping_neg() >> (CORDIC_PRECISION_SHIFT - 1)).wrapping_add(1)) >> 1)
            .wrapping_neg()
    }
}

/// Calculate the Q16 cosine-like and sine-like coordinates of a degree angle.
///
/// The original fixed conversion, signed remainder, quadrant choice and
/// eighteen rotations are retained, including full-width wrapping inputs.
pub const fn cordic_calc_iq(theta: i32) -> CordicIq {
    let mut coord = CordicIq {
        i: CORDIC_ANGLE_GEN,
        q: 0,
    };
    let mut angle: i32 = 0;
    let mut signx: i32 = 1;
    let mut theta = cordic_fixed(theta);
    let signtheta = if theta < 0 { -1 } else { 1 };
    let half_turn = cordic_fixed(180).wrapping_mul(signtheta);
    theta = (theta.wrapping_add(half_turn) % cordic_fixed(360)).wrapping_sub(half_turn);

    if cordic_float(theta) > 90 {
        theta = theta.wrapping_sub(cordic_fixed(180));
        signx = -1;
    } else if cordic_float(theta) < -90 {
        theta = theta.wrapping_add(cordic_fixed(180));
        signx = -1;
    }

    let mut iteration = 0;
    while iteration < CORDIC_NUM_ITER {
        let next_i;
        if theta > angle {
            next_i = coord.i.wrapping_sub(coord.q >> iteration);
            coord.q = coord.q.wrapping_add(coord.i >> iteration);
            angle = angle.wrapping_add(ARCTAN_TABLE[iteration]);
        } else {
            next_i = coord.i.wrapping_add(coord.q >> iteration);
            coord.q = coord.q.wrapping_sub(coord.i >> iteration);
            angle = angle.wrapping_sub(ARCTAN_TABLE[iteration]);
        }
        coord.i = next_i;
        iteration += 1;
    }

    coord.i = coord.i.wrapping_mul(signx);
    coord.q = coord.q.wrapping_mul(signx);
    coord
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
