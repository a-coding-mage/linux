// SPDX-License-Identifier: GPL-2.0-only
//! Native kernel crate for kallsyms_show_value().
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `ksyms_common.c` translation unit.

#[path = "ksyms_common.rs"]
mod implementation;

pub use implementation::*;
