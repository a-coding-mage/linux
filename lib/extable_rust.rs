// SPDX-License-Identifier: GPL-2.0-or-later
//! Native kernel crate for exception table sorting and searching.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `extable.c` translation unit.

#[path = "extable.rs"]
mod implementation;

pub use implementation::*;
