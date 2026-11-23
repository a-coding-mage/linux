// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for range add and subtract.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `range.c` translation unit.

#[path = "range.rs"]
mod implementation;

pub use implementation::*;
