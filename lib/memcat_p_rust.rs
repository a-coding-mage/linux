// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for the NULL-terminated pointer array merge helper.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `memcat_p.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "memcat_p.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(__memcat_p, __memcat_p, "GPL", "");
