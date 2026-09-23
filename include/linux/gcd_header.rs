// SPDX-License-Identifier: GPL-2.0
//! Safe GCD algorithms; native runtime static-key ownership is separate.

#[path = "../../lib/math/gcd.rs"]
mod greatest_common_divisor;

pub use greatest_common_divisor::{gcd, gcd_with_ffs};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
