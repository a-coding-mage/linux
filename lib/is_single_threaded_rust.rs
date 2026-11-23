// SPDX-License-Identifier: GPL-2.0-or-later
//! Native kernel crate for current_is_single_threaded().
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `is_single_threaded.c` translation unit.

#[path = "is_single_threaded.rs"]
mod implementation;

pub use implementation::*;
