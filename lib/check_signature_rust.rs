// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for the BIOS signature comparison helper.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `check_signature.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "check_signature.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(check_signature, check_signature, "", "");
