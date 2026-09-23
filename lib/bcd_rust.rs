// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for binary-coded decimal conversions.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `bcd.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "bcd.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(_bcd2bin, _bcd2bin, "", "");
ffi_export::export_symbol!(_bin2bcd, _bin2bcd, "", "");
