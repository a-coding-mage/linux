// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for the personality() system call.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `exec_domain.c` translation unit.

#[path = "exec_domain.rs"]
mod implementation;

pub use implementation::*;
