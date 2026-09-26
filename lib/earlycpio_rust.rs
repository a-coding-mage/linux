// SPDX-License-Identifier: GPL-2.0-only
//! Built-in early archive parser; the original has no module exports.

#[path = "earlycpio.rs"]
mod implementation;

pub use implementation::find_cpio_data;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
