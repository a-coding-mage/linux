// SPDX-License-Identifier: GPL-2.0-only
//! Native kernel crate for the rcuref slow paths.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `rcuref.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "rcuref.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(rcuref_get_slowpath, rcuref_get_slowpath, "GPL", "");
ffi_export::export_symbol!(rcuref_put_slowpath, rcuref_put_slowpath, "GPL", "");
