// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for binary-coded decimal conversions.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `bcd.c` translation unit.

#[path = "bcd.rs"]
mod implementation;

pub use implementation::*;
