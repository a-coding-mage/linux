// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for the generic bust_spinlocks().
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `bust_spinlocks.c` translation unit.

#[path = "bust_spinlocks.rs"]
mod implementation;

pub use implementation::*;
