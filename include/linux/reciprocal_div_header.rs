/* SPDX-License-Identifier: GPL-2.0 */
//! Pure reciprocal-division helpers shared with the canonical implementation.
//!
//! Constructors and division return `None` outside the original C's defined
//! input domain. No native owner, external symbol, or allocation is imported.

#[path = "../../lib/math/reciprocal_div.rs"]
mod implementation;

pub use implementation::{
    reciprocal_divide, reciprocal_value, reciprocal_value_adv, ReciprocalValue, ReciprocalValueAdv,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
