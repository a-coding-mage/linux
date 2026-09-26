// SPDX-License-Identifier: GPL-2.0
//! Safe, constant-evaluable bit reversal matching the C header interfaces.
//!
//! These helpers define no second table or exported symbol. They also work
//! on targets where the original C header selects architecture instructions.

#[path = "../../include/linux/bitrev_header.rs"]
mod implementation;

pub use implementation::*;
