// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for the generic BUG()/WARN() trap reporting.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `bug.c` translation unit.

#[path = "bug.rs"]
mod implementation;

pub use implementation::*;
