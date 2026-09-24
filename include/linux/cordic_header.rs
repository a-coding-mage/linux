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

//! Safe CORDIC API, backed by the single canonical fixed-point implementation.

#[path = "../../lib/math/cordic.rs"]
mod implementation;

pub use implementation::{
    cordic_calc_iq, cordic_fixed, cordic_float, CordicIq, CORDIC_ANGLE_GEN, CORDIC_NUM_ITER,
    CORDIC_PRECISION_SHIFT,
};

/// Compatibility spelling of the original integer-to-Q16 macro.
pub use implementation::cordic_fixed as CORDIC_FIXED;
/// Compatibility spelling of the original Q16-rounding macro.
pub use implementation::cordic_float as CORDIC_FLOAT;
/// Compatibility spelling of the original coordinate type.
pub use implementation::CordicIq as cordic_iq;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
