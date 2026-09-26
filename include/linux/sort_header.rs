/* SPDX-License-Identifier: GPL-2.0 */
//! Typed declarations for the original sort interfaces.

pub use bindings::{sort, sort_nonatomic, sort_r, sort_r_nonatomic};
pub use bindings::{SortCmp, SortPriv, SortRCmp, SortRSwap, SortSwap};

/// Three-way comparison without subtraction overflow or integer narrowing.
#[inline]
pub fn cmp_int<T: PartialOrd>(l: T, r: T) -> i32 {
    i32::from(l > r) - i32::from(l < r)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
