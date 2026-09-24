/* SPDX-License-Identifier: LGPL-2.1-or-later */
/*
 * Provides fixed-point logarithm operations.
 *
 * Copyright (C) 2006 Christoph Pfister (christophpfister@gmail.com)
 */

//! Pure Q24 logarithm approximations shared with the canonical implementation.
//!
//! Zero returns `None`, without the warning emitted by the native C interface.
//! No foreign declarations, native export owner, or allocation is imported.

#[path = "../../lib/math/int_log.rs"]
mod implementation;

pub use implementation::{intlog10, intlog2};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
