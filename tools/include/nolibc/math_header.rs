/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * math definitions for NOLIBC
 * Copyright (C) 2025 Thomas Weissschuh <thomas.weissschuh@linutronix.de>
 */

/* C header dependency: include "nolibc.h" to make sure to include all global symbols. */

#[inline]
pub fn fabs(x: f64) -> f64 {
    if x >= 0.0 { x } else { -x }
}

#[inline]
pub fn fabsf(x: f32) -> f32 {
    if x >= 0.0 { x } else { -x }
}

/* C long double has no file-local Rust primitive equivalent; use f64 for this translation. */
#[inline]
pub fn fabsl(x: f64) -> f64 {
    if x >= 0.0 { x } else { -x }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
