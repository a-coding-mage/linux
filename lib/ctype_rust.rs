// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for the shared byte-classification implementation.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `ctype.c` translation unit.

#[path = "ctype.rs"]
mod implementation;

pub use implementation::*;
